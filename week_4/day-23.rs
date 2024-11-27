use tokio::time::{sleep,Duration};
#[tokio::main]
async fn main(){
    let task1 = tokio::spawn(async{
        sleep(Duration::from_secs(3)).await;
        println!("Task1 completed");
    });
    let task2 = tokio::spawn(async{
        sleep(Duration::from_secs(2)).await;
        println!("Task2 completed");
    });

    task1.await.unwrap();
    task2.await.unwrap();
}
