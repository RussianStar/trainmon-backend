use std::sync::Arc;
use tokio::sync::Semaphore;
use futures::future::join_all;
use std::future::Future;

use core::fmt::Debug;


pub async fn process_entries<T, F, Fut>(semaphore: Arc<Semaphore>, entries: &[T], process: F) -> Vec<Fut::Output>
where
T: Send + Sync + 'static + Clone + Debug,
F: Fn(T) -> Fut + Send + Sync + 'static + Clone,
Fut: Future + Send + 'static, 
<Fut as Future>::Output: Send + Debug + 'static, 
{
    let futures: Vec<_> = entries.iter().cloned().map(|entry|  {
        let semaphore_clone = semaphore.clone();
        let process = process.clone();
        tokio::spawn(async move {
            let permit = semaphore_clone.acquire_owned().await.unwrap();

            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await; // delay for 1000ms
            let result = process(entry).await; // processing

            drop(permit);
            result
        })
    }).collect();

    let results = join_all(futures).await;
    let results = results.into_iter().map(|res| res.unwrap()).collect::<Vec<_>>();
    results
}


