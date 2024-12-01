# TOKIO
This is an asynchronous runtime that providess the blocks for writing netwok applications.

It is applied for 
- Task scheduling
- Memory safety
- Ecosystem
- Asynchronous input and output

Asynchronous runtime is a system that manages the execution of tasks in a way that it doesnt block the program while waiting for something else to happen.
in synchronous programming if the program is waiting for a task (eg like downloading a file), the whole program might stop to wait for the task to finish. But it is not so sor asynchronous as the program can be executing while the file is downloading at the same time, thereby improving efficiency and responsiveness.
Tokio is used when we want to run programs asynchronously.

### To use TOKIO, 
        
        install Tokio library
        confirm rustc --version
        cargo install mini-redis
        confirm mini-redis-server
        mini-redis-clo get foo

#### Example

```rust
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main (){
prntln!("Start");
sleep(Duration::from_secs(2)).await;
println!("End");
}
```

- tokio::time: tokio crate for working with time based delays
- Duration: a struct from the standard library (std::time) used to represent a span of time; to specify time intervals 
- sleep: a function in (tokio::time) that initaites an asynchronous delay
- tokio::main: is the entry point for the Tokio asynchronous runtime(sets the enviroment to execute asynchronous tasks)
- async: marks the function for asynchronous runtime 
- sleep(Duration::from_secs(2)).await;
    - sleep: a function that initaites an asynchronous delay.
    - Duration::from_secs(2): creates a duration object representing 2 objects meaning the function will delay 2 seconds 
    - .await: allows other taskds to run during thr delay; waits asynchronously for the delay to complete 

#### Program flow 
- print Start
- wait asynchronously for 2 seconds
- print End
