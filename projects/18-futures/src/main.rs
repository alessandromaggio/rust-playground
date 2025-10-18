use trpl::{Html, Either, StreamExt, Stream, ReceiverStream};
use std::time::Duration;
use std::pin::{Pin, pin};

async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|el| el.inner_html())
}

fn race_two_sites(url_1: &str, url_2: &str) {
    trpl::run(async {
    let title_fut_1 = async { (url_1, page_title(url_1).await) };
    let title_fut_2 = async { (url_2, page_title(url_2).await) };

    let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
      Either::Left(result) => result,
      Either::Right(result) => result,
    };

    println!("{} returned first", url);
    match maybe_title {
      Some(title) => println!("Title: {}", title),
      None => println!("No title found"),
    }
  });
}

fn use_spawn_task() {
    trpl::run(async {
        let fut_1 = pin!(async {
            for i in 1..10 {
                println!("Task 1: {}", i);
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let fut_2 = pin!(async {
            for i in 1..5 {
                println!("Main: {}", i);
                trpl::sleep(Duration::from_millis(150)).await;
            }
        });

        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![Box::pin(fut_1), Box::pin(fut_2)];
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![fut_1, fut_2];
        trpl::join_all(futures).await;
        // If you know in advance how many futures you have, you can use trpl::join! macro
        // trpl::join(fut_1, fut_2).await;
    });
}

fn channels() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        // Move is not necessary, but we do so as to move tx, so that it goes out of scope at the end of the block
        // This will shut down the channel and thus let the rx loop end
        let tx_fut = async move {
            let vals = vec!["hi", "from", "the", "other", "side"];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let rx_fut = async {
            while let Some(received) = rx.recv().await {
                println!("Got: {}", received);
            }
        };
        
        trpl::join!(tx_fut, rx_fut);
    });
}

fn slow_function(name: &str, ms: u64) {
    // We use std::thread::sleep here to simulate a blocking operation
    // Unlike trpl::sleep, this will block the entire thread
    std::thread::sleep(Duration::from_millis(ms));
    println!("'{}' ran for {}ms", name, ms);
}

fn racing_futures() {
    trpl::run(async {
        let slow = async {
            println!("'slow' started");
            trpl::sleep(Duration::from_millis(1000)).await;
            println!("'slow' finished");
        };

        let fast = async {
            println!("'fast' started");
            trpl::sleep(Duration::from_millis(500)).await;
            println!("'fast' finished");
        };

        trpl::race(slow, fast).await;
    });
}

 fn handing_back_control() {
    trpl::run(async {
        let a = async {
            let name = "a";
            println!("a: started");
            slow_function(name, 30);
            trpl::yield_now().await; // Yield control to runtime to allow other tasks to run
            slow_function(name, 10);
            trpl::yield_now().await;
            slow_function(name, 20);
            trpl::yield_now().await;
            trpl::sleep(Duration::from_millis(50)).await;
            println!("a: ended");
        };

        let b = async {
            let name = "b";
            println!("b: started");
            slow_function(name, 75);
            trpl::yield_now().await;
            slow_function(name, 10);
            trpl::yield_now().await;
            slow_function(name, 15);
            trpl::yield_now().await;
            slow_function(name, 350);
            trpl::yield_now().await;
            trpl::sleep(Duration::from_millis(50)).await;
            println!("b: ended");
        };

        trpl::race(a, b).await;
    });
 }

 async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(result) => Ok(result),
        Either::Right(_) => Err(max_time),
    }
 }

 fn custom_timeout() {
    trpl::run(async {
        let long_fut = async {
            println!("Long future started");
            trpl::sleep(Duration::from_millis(2000)).await;
            println!("Long future finished");
            42
        };

        match timeout(long_fut, Duration::from_millis(1000)).await {
            Ok(value) => println!("Future completed with value: {}", value),
            Err(duration) => println!("Future timed out after {:?}", duration),
        }
    });
 }

 fn stream_iterators() {
    trpl::run(async {
        let values = vec![1, 2, 3, 4, 5];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("Got value: {}", value);
        }
    });
 }


 fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for messages in messages {
        if let Err(send_err) = tx.send(format!("Message {}", messages)) {
            eprintln!("Failed to send message: {}", send_err);
            break;
        }
    }

    ReceiverStream::new(rx)
 }

 fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            if let Err(send_err) = tx.send(count) {
                eprintln!("Could not send interval {}: {}", count, send_err);
                break;
            };
        }
    });

    ReceiverStream::new(rx)
 }

fn streams() {
    trpl::run(async {
        let mut messages = get_messages();

        while let Some(message) = messages.next().await {
            println!("Got message: {:?}", message);
        }
    });
}

fn merged_streams() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals()
            .map(|count| format!("Interval {}", count))
            .throttle(Duration::from_millis(100))
            .timeout(Duration::from_secs(10));
        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(item) = stream.next().await {
            println!("Got item: {:?}", item);
        }
    });
}

fn main() {
    merged_streams();
    streams();
    stream_iterators();
    custom_timeout();
    handing_back_control();
    racing_futures();
    use_spawn_task();
    channels();
}
