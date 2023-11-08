use std::sync::{
    Arc,
};
use futures::future::join_all;
use tokio::sync::Semaphore;
use num_cpus;


#[tokio::main]
async fn main() {
    println!("Starting main function");
    let array: Vec<_> = (0..20).collect();
    let semaphore = Arc::new(Semaphore::new(num_cpus::get()));
    println!("Array and semaphore initialized");
    
    let results = process_entries(semaphore, &array).await;
    println!("Processing completed");
}

async fn process_entries(semaphore: Arc<Semaphore>, entries: &[i32]) -> Vec<i32> {
    println!("Starting process_entries function");
    let futures: Vec<_> = entries.iter().map(|&entry|  {
        let semaphore_clone = semaphore.clone();
        tokio::spawn(async move {
            let permit = semaphore_clone.acquire_owned().await.unwrap();

            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await; // delay for 1000ms
            let result = entry * 2; // example processing

            println!("Processing completed for entry: {}", entry);
            drop(permit);
            result
        })
    }).collect();

    let results = join_all(futures).await;
    let results = results.into_iter().map(|res| res.unwrap()).collect::<Vec<_>>();
    println!("All futures joined. Results: {:?}", results);
    results
}
