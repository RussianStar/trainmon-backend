# Run locally
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
cargo install cargo-watch
cargo watch -x run

```

# Use

POST an `http://localhost:3030/analyze/full` mit body :

```
{
    "user_id": "0edade91-3ffc-523c-be43-c649b6412a35",
       "path": "C:\\work\\privat\\fitfiles\\tilman",
  "modes": [
    "workout",
    "power",
    "heart_rate"
  ],
    "hr_zones": "2022-12-01|SGVsbG8gd29ybGQ=",
    "pwr_zones": "2022-12-01|SGVsbG8gd29ybGQ=" 
}

```
Dabei Pfad und ID anpassen.
