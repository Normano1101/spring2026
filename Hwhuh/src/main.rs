use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use rand::Rng;

//assignment 3
// Message to be sent to the workers
enum Message {
    NewJob(Job),
    Terminate,
}

// Job type is a boxed closure that can be sent across threads
type Job = Box<dyn FnOnce() + Send + 'static>;

// ThreadPool struct
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    // Create a new ThreadPool with the specified size
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        
        // TODO: Create a channel for sending jobs
        let (sender, receiver) = mpsc::channel();
        
        
        // TODO: Create and store workers
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::new();
        
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
        
        // TODO: Return the ThreadPool
        ThreadPool { workers, sender }
        
    }
    
    // Execute a job in the thread pool
    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: Create a job from the closure and send it to a worker
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
    
}

// Clean up resources when ThreadPool is dropped
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // TODO: Send terminate message to all workers
            for _ in &self.workers {
                self.sender.send(Message::Terminate).unwrap();
            }
        
        
        // TODO: Wait for all workers to finish
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        
    }
}

// Worker struct represents a thread that can process jobs
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Create a new worker with the specified ID
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // TODO: Create a thread that loops and receives jobs from the channel
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                
                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        
        
        
        // TODO: Return the Worker
        Worker { id, thread: Some(thread) }
        
    }
}
//end of assignment 3

//assignment 4
// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal

    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let item = rng.gen_range(1..=100);
        tx.send(item).unwrap();
    }


}

// TODO: Implement consumer function
const TERMINATION_SIGNAL: i32 = -1;

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let item = rx.lock().unwrap().recv().unwrap();

        if item == TERMINATION_SIGNAL {
            println!("Consumer {} terminating.", id);
            break;
        }

        println!("Consumer {} processed item: {}", id, item);
    }
}

fn main() {
    //assesstment 3
    // Create a new thread pool with 4 workers
    let pool = ThreadPool::new(4);
    
    // Submit 10 tasks to the pool
    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(std::time::Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }
    
    println!("Main thread waiting for tasks to complete...");
    // ThreadPool will be dropped when it goes out of scope, triggering the cleanup

    //end of assignment 3

    //assignment 4
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    
    
    // TODO: Create 2 producer threads

    let producer_tx1 = tx.clone();
    let producer_handle1 = thread::spawn(move || {
        producer(1, producer_tx1, ITEM_COUNT);
    });


    let producer_tx2 = tx.clone();
    let producer_handle2 = thread::spawn(move || {
        producer(2, producer_tx2, ITEM_COUNT);
    });
    
    
    // TODO: Create 3 consumer threads
    let consumer_rx = Arc::new(Mutex::new(rx));
    let consumer_handle1 = thread::spawn({
        let consumer_rx = Arc::clone(&consumer_rx);
        move || {
            consumer(1, consumer_rx);
        }

    });
    
    // TODO: Wait for all threads to finish
    let consumer_handle2 = thread::spawn({
        let consumer_rx = Arc::clone(&consumer_rx);
        move || {
            consumer(2, consumer_rx);
        }
    });
    let consumer_handle3 = thread::spawn({
        let consumer_rx = Arc::clone(&consumer_rx);
        move || {
            consumer(3, consumer_rx);
        }
    }); 

    producer_handle1.join().unwrap();
    producer_handle2.join().unwrap();

    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }
    consumer_handle1.join().unwrap();
    consumer_handle2.join().unwrap();
    consumer_handle3.join().unwrap();

    
    
    println!("All items have been produced and consumed!");
    //end of assignment 4
}