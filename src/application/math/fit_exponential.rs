use varpro::prelude::*;
use varpro::solvers::levmar::{LevMarProblemBuilder, LevMarSolver};

use nalgebra::DVector;
use rand;


pub fn fit_exponential(x: DVector<f64>, y:DVector<f64>) -> f64 {
    let model = SeparableModelBuilder::<f64>::new(&["tau1"])
        .function(&["tau1"], exp_decay)
        .partial_deriv("tau1", exp_decay_dtau)
        .independent_variable(x)
        .initial_parameters(vec![1.])
        .build()
        .unwrap();

    let problem = LevMarProblemBuilder::new(model)
        .observations(y)
        .build()
        .unwrap();
    let fit_result = LevMarSolver::new()
        .fit(problem)
        .expect("fit must succeed");

    let alpha = fit_result.nonlinear_parameters();
    let decay_constant = 1./ alpha[(0, 0)];
    println!("Decay constant is {:.4}", decay_constant);
    decay_constant
}

pub fn fit_with_error(x: DVector<f64>, y:DVector<f64>) -> (f64,f64) {
    let mut results = Vec::new();
    for _ in 0..10 {
        let y_error = y.map(|val| val + val * (rand::random::<f64>() - 0.5) * 0.02);
        let tau = fit_exponential(x.clone(), y_error);
        results.push(tau);
    }
    let mean: f64 = results.iter().sum::<f64>() / results.len() as f64;
    let variance: f64 = results.iter().map(|val| (val - mean).powi(2)).sum::<f64>() / results.len() as f64;
    (mean, variance.sqrt())
}

fn exp_decay(x :&DVector<f64>, tau : f64) -> DVector<f64> {
  x.map(|x|(-x/tau).exp())
}

fn exp_decay_dtau(tvec: &DVector<f64>,tau: f64) -> DVector<f64> {
    tvec.map(|t| (-t / tau).exp() * t / tau.powi(2))
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use nalgebra::DVector;

    #[test]
    fn test_fit_exponential() {
        let y = DVector::from_vec(vec![450.0, 440.0, 430.0, 420.0, 410.0, 400.0, 390.0, 380.0, 370.0, 360.0, 350.0, 340.0, 330.0, 320.0, 310.0, 300.0, 290.0, 280.0, 270.0, 260.0, 250.0, 240.0, 230.0, 220.0, 210.0, 200.0, 190.0, 180.0, 170.0, 160.0]);
        let x = DVector::from_vec((1..=30).map(|i| i as f64).collect());
        let result = fit_exponential(x, y);
        // Was calculated via python scikit: .03224872339697765
        assert!((result - 0.03224872339697765).abs() < 0.001, "The fitting result should be within the range of plus minus 0.005");
    }


    #[test]
    fn test_multi_fit() {
        let y = DVector::from_vec(vec![450.0, 440.0, 430.0, 420.0, 410.0, 400.0, 390.0, 380.0, 370.0, 360.0, 350.0, 340.0, 330.0, 320.0, 310.0, 300.0, 290.0, 280.0, 270.0, 260.0, 250.0, 240.0, 230.0, 220.0, 210.0, 200.0, 190.0, 180.0, 170.0, 160.0]);
        let x = DVector::from_vec((1..=30).map(|i| i as f64).collect());
        let results = fit_with_error(x,y);
        println!("{:?}", results);
    }
}

