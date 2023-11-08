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
    
    let process = |entry: i32| async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await; // delay for 1000ms
        entry * 2 // example processing
    };

    let results = process_entries(semaphore, &array, process).await;
    println!("Processing completed");
}


async fn process_entries<T, F, Fut>(semaphore: Arc<Semaphore>, entries: &[T], process: F) -> Vec<T>
where
    T: Send + Sync + 'static,
    F: Fn(T) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = T> + Send,
{
    let futures: Vec<_> = entries.iter().cloned().map(|entry|  {
        let semaphore_clone = semaphore.clone();
        let process = process.clone();
        tokio::spawn(async move {
            let permit = semaphore_clone.acquire_owned().await.unwrap();

            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await; // delay for 1000ms
            let result = process(entry).await; // processing

            println!("Processing completed for entry: {:?}", entry);
            drop(permit);
            result
        })
    }).collect();

    let results = join_all(futures).await;
    let results = results.into_iter().map(|res| res.unwrap()).collect::<Vec<_>>();
    println!("All futures joined. Results: {:?}", results);
    results
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
