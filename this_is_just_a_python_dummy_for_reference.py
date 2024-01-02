
import numpy as np
import pandas as pd
from scipy import optimize
from sklearn.linear_model import LinearRegression
from sklearn.tree import DecisionTreeRegressor
from Training.data_preprocessing import get_data, preprocess_data, calculate_k1s, handle_metrics
from Training.model_training import train_ctl_model, train_k1_model, train_k1_tree

def get_processed_data_and_models(data, cols_of_interest =["Average Resting Heart Rate", "HRV Balance Score", 'Average HRV'], verbose = False):
    data['3d Sleep Score'] = data['Sleep Score'].rolling(window=3).mean()

    print('---- kappa ----')
    print(data.columns)
    cols_of_interest = cols_of_interest + ["3d Sleep Score"]
    for col in cols_of_interest:  
        data[col] = data[col].replace('None', 0)
        data[col] = data[col].fillna(0)
        data[col] = data[col].astype(float)

    data = data.dropna(subset=['CTL'])
    data = data.dropna(subset=['TSB'])

    total_vo2x = data.index
    data['vo2 estimate'] = data['vo2 estimate'].bfill().ffill()
    data['Weight Kilograms'] = data['Weight Kilograms'].bfill().ffill()
    total_vo2y = data['vo2 estimate'] * data['Weight Kilograms']
    total_vo2y = [np.mean(i) for i in total_vo2y]

    print(f'DEBUG :: {total_vo2y}')
    ctls_model =[]
    tsbs_model = []
    for index, row in data.iterrows():
        try:
            ctls_model.append(row['CTL'])
            tsbs_model.append(row['TSB'])
        except KeyError:
            ctls_model.append(ctls_model[-1] if ctls_model else 0)
            tsbs_model.append(tsbs_model[-1] if tsbs_model else 0)

    df = pd.DataFrame({'CTL':ctls_model, 'TSB':tsbs_model, 'VO2':total_vo2y})
    if verbose:
        print(f"DF: {df}")
        df.to_csv('vo2 model.csv')
    x=df[["CTL"]]
    y=df["VO2"]
    ctl_model = LinearRegression().fit(x, y)
    ctl_coefs = ctl_model.coef_
    ctl_intercept = ctl_model.intercept_
    if verbose:
        print(f"CTL Intercept: {ctl_intercept}")


    # Calculate the k1s
    ctls = {}
    for i, row in data.iterrows():
        ctls[i] = row['CTL']

    df = pd.DataFrame({'Dates': data.index, 'VO2': total_vo2y})
    df.set_index('Dates', inplace=True)
    df[cols_of_interest] = data[cols_of_interest]
    df['V02_30'] = df['VO2'].rolling(window=30).mean().bfill()
    df2 = pd.DataFrame({
        'Dates': data.index,
        'CTL': data['CTL'],
        'TSBS': data['TSB']
    })

    print(f'DEBUG PRE MERGE :: {df.head()}')
    df = pd.merge(df, df2, on="Dates", how="left")
    df=df.dropna()
    df['k1'] = (df["V02_30"] - ctl_intercept)/df['CTL']
    print(f'DEBUG : PRE 0.5 :: {df.head()}')
    #df=df[df['k1'] < 0.5]
    k1_x=df.index.tolist()
    k1_y=df['k1'].tolist()


    print(f'DEBUG :: PRE PLIN REG{df.head()}')
    if verbose:
        print(df.head())
        print(f"K1 df: {df}")
    k1_corr = {}
    for col in df.columns:
        if col != "Dates":
            k1_corr[col] = df['k1'].corr(df[col])
    if verbose: 
        print(f"k1 Correlations: {k1_corr}")
    # Passe die Werte um die gewünschten Faktoren zu erhalten :
    # Original : "HRV", "RHR", 'SORES','Stress', 'Fatigue', "Mood", "Freshness"
    x=df[cols_of_interest] 
    y=df['k1']
    print(y)
    print(x)
    k1_model =LinearRegression().fit(x, y)
    k1_coefs=k1_model.coef_
    k1_intercept= k1_model.intercept_
    if verbose:
        print(dict(zip(cols_of_interest, k1_coefs)))
        print(f"K1 Coefs: {k1_coefs}")
        print(f"K1 Intercept: {k1_intercept}")
    X=df[['CTL','TSBS'] + cols_of_interest] 
    y=df['k1']
    k1_tree = DecisionTreeRegressor(random_state=0).fit(X,y)
    
    # Return processed data and models
    return df, ctl_model, k1_model, k1_tree

