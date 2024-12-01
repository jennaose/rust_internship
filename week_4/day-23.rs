use tokio::time::{sleep,Duration};
#[tokio::main]
async fn main(){
    let task1 = tokio::spawn(async{ //spawan starts a new task
        sleep(Duration::from_secs(3)).await; //sleep stimulates asynchronous delay(work)
        println!("Task1 completed");
    });
    let task2 = tokio::spawn(async{
        sleep(Duration::from_secs(2)).await; //.await paused the task until the condition is done 
        println!("Task2 completed");
    });

    task1.await.unwrap();
    task2.await.unwrap();
}
