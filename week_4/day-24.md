In Rust and with Tokio, **concurrency** and **parallelism** are closely related but distinct concepts. 
-
### **1. Concurrency**: Handling Multiple Tasks at Once

Concurrency refers to the ability to **manage multiple tasks** (or operations) at the same time. This doesn’t necessarily mean that they are running **simultaneously**—it just means that the system can handle multiple tasks, switching between them to make progress.

- **Concurrency** is about structuring a program to perform multiple tasks in a way that they can make progress without blocking each other.

#### **Concurrency in Rust with Tokio**
- Rust, by default, is **single-threaded**. However, with **Tokio**, you can **run concurrent tasks**. These tasks are not necessarily executed at the same exact time but are **interleaved**—Tokio switches between them efficiently so that each can make progress.
  
- **Async Functions**: When you use `async` functions in Rust, you’re defining **concurrent tasks**. These tasks use `await` to yield control and allow other tasks to run while they wait for something (e.g., I/O operations).
  
  - For example, you can start multiple network requests at once, and while each is waiting for data to arrive, Tokio can run other tasks in the meantime.

#### Example of Concurrency:
```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 completed!");
    });

    let task2 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 2 completed!");
    });

    task1.await.unwrap();
    task2.await.unwrap();

    println!("All tasks are done!");
}
```
- In this example, **Task 1** and **Task 2** are running concurrently. While Task 2 sleeps for 1 second, Task 1 is sleeping for 2 seconds, but they don’t block each other. Tokio handles this by switching between them.

### **2. Parallelism**: Running Tasks Simultaneously

Parallelism is when multiple tasks are executed at the same time using multiple CPU cores. This is different from concurrency, where tasks are executed independently but not necessarily at the same time.

#### **Parallelism in Rust with Tokio**
- **Tokio** can provide parallelism, but it depends on how you configure it:
  - Tokio is an asynchronous runtime for Rust that can handle both concurrency and parallelism. By default, Tokio tasks are run concurrently on a single thread. To achieve parallelism, you need to configure Tokio to use multiple threads.
  
- To enable parallelism, you can use Tokio's multi-threaded scheduler. This means that tasks can run on different CPU cores simultaneously. The example below demonstrates how to configure and use the multi-threaded scheduler in Tokio:

#### Example of Parallelism (Multi-threaded Scheduler):
You can enable parallelism by specifying the number of threads for the Tokio runtime:
```rust
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let task1 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 completed!");
    });

    let task2 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 2 completed!");
    });

    task1.await.unwrap();
    task2.await.unwrap();

    println!("All tasks are done!");
}
```
#### Explanation
- #[tokio::main(flavor = "multi_thread", worker_threads = 4)]: This line configures the Tokio runtime to use the multi-threaded scheduler with 4 worker threads. Each worker thread can run tasks in parallel on different CPU cores.

- tokio::spawn(async { ... }): This macro spawns asynchronous tasks. The tasks are placed into a work-stealing task queue, which allows them to be distributed across the available worker threads.

- sleep(Duration::from_secs(2)).await;: This introduces a delay of 2 seconds asynchronously without blocking the thread.

- task1.await.unwrap();: This line waits for task1 to complete. If the task fails, it will panic.

- Parallel Execution
With the multi-threaded scheduler, task1 and task2 can run on different threads.

In the example:

Task 1 sleeps for 2 seconds, and Task 2 sleeps for 1 second.

Since they are running in parallel, the total time to complete both tasks will be approximately 2 seconds, not 3.

### **Key Differences Between Concurrency and Parallelism**:

| **Concept**      | **Concurrency**                             | **Parallelism**                                     |
|------------------|---------------------------------------------|-----------------------------------------------------|
| **Definition**   | Handling multiple tasks, often by switching between them. | Executing multiple tasks at the exact same time (on different threads/cores). |
| **Execution**    | Tasks are interleaved; only one task is running at a time. | Tasks are running simultaneously on multiple threads. |
| **Focus**        | Efficiently managing tasks that can be paused and resumed (e.g., async I/O). | Splitting tasks into multiple threads to take advantage of multiple CPU cores. |
| **Use Case**     | I/O-bound operations (e.g., network requests, reading files). | CPU-bound operations that can benefit from multi-core processing. |

---

### **Concurrency and Parallelism in Tokio:**
- **Concurrency**: You define async tasks, and **Tokio** schedules them on the available threads. It allows multiple tasks to run **concurrently** without blocking.
- **Parallelism**: If you use the **multi-threaded scheduler** (`flavor = "multi_thread"`), Tokio can run tasks in **parallel** across multiple threads, utilizing the system’s available CPU cores.

### **When to Use Each?**
- **Concurrency**: When you have multiple I/O tasks that need to be handled, such as web servers, handling user requests, or managing multiple file downloads.
- **Parallelism**: When you need to do heavy computation (like processing large datasets), you can use parallelism to take full advantage of a multi-core CPU.

---

### **Summary**
- **Concurrency** in Tokio allows multiple tasks to make progress **by switching between them**, without blocking each other.
- **Parallelism** in Tokio involves **running tasks simultaneously** on multiple threads, utilizing multiple CPU cores for tasks that are CPU-bound.
