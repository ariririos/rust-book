mod page_title {
    use trpl::{Either, Html};
    use std::env;

    async fn page_title(url: &str) -> (&str, Option<String>) {
        let response_text = trpl::get(url).await.text().await;
        let title = Html::parse(&response_text)
            .select_first("title")
            .map(|title| title.inner_html());
        (url, title)
    }

    pub fn run_page_title() {
        let args: Vec<String> = env::args().collect();
        trpl::run(async {
            let title_fut_1 = page_title(&args[1]);
            let title_fut_2 = page_title(&args[2]);
            let (url, maybe_title) = 
                match trpl::race(title_fut_1, title_fut_2).await {
                    Either::Left(left) => left,
                    Either::Right(right) => right
                };
            println!("{url} returned first");
            match maybe_title {
                Some(title) => println!("Its page title was: '{title}'"),
                None => println!("It had no title")
            }
        });
    }
}

mod handles {
    use std::time::Duration;

    pub fn run_handles() {
        trpl::run(async {
            let handle = trpl::spawn_task(async {
                for i in 1..10 {
                    println!("hi number {i} from first task");
                    trpl::sleep(Duration::from_millis(500)).await
                }
            });

            for i in 1..5 {
                println!("hi number {i} from second task");
                trpl::sleep(Duration::from_millis(500)).await;
            }

            handle.await.unwrap()
        });
    }
}

mod timeout {
    use std::time::Duration;
    use trpl::Either;

    pub async fn timeout<'a, F: Future>(
        func: F,
        deadline: Duration
    ) -> Result<F::Output, Duration> {
        match trpl::race(func, trpl::sleep(deadline)).await {
            Either::Left(msg) => Ok(msg),
            Either::Right(_) => Err(deadline)
        }
    }

    pub fn run_timeout() {
        trpl::run(async {
            let slow = async {
                trpl::sleep(Duration::from_secs(5)).await;
                "Finally finished"
            };

            match timeout(slow, Duration::from_secs(20)).await {
                Ok(msg) => println!("Succeeded with '{msg}'"),
                Err(deadline) => println!("Failed after {} seconds", deadline.as_secs())
            }
        });
    }        
}

mod join {
    use std::time::Duration;

    pub fn run_join() {
        trpl::run(async {
            let fut1 = async {
                for i in 1..10 {
                    println!("hi number {i} from first task");
                    trpl::sleep(Duration::from_millis(500)).await
                }
            };
            let fut2 = async {
                for i in 1..5 {
                    println!("hi number {i} from second task");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };
            trpl::join(fut1,fut2).await;
        });
    }
}

mod channels {
    pub fn run_channels() {
        use std::pin::{Pin, pin};
        use std::time::Duration;

        trpl::run(async {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx1_fut = pin!(async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];
                for val in vals {
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });
            
            let rx_fut = pin!(async {
                while let Some(value) = rx.recv().await {
                    println!("received: '{value}'");
                }
            });

            let tx_fut = pin!(async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];
                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            });

            let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];

            trpl::join_all(futures).await;
        });
    }
}

mod async_results {
    pub fn run_results() {
        trpl::run(async {
            let a = async { 1u32 };
            let b = async { "Hello!" };
            let c = async { true };

            let (a_result, b_result, c_result) = trpl::join!(a, b, c);
            println!("a: {a_result}, b: {b_result}, c: {c_result}");
        });
    }
}

mod race {
    pub fn run_race() {
        use std::time::Duration;

        trpl::run(async {
            let slow = async {
                println!("'slow' started");
                trpl::sleep(Duration::from_millis(100)).await;
                println!("'slow' finished");
            };

            let fast = async {
                println!("'fast' started");
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'fast' finished");
            };

            trpl::race(slow, fast).await;
        });
    }
}

mod yielding {
    use std::thread;
    use std::time::Duration;

    pub fn slow(name: &str, ms: u64) {
        thread::sleep(Duration::from_millis(ms));
        println!("'{name}' ran for {ms}ms");
    }

    pub fn run_yielding() {
        trpl::run(async {
            let a = async {
                println!("'a' started");
                slow("a", 30);
                trpl::yield_now().await;
                slow("a", 10);
                trpl::yield_now().await;
                slow("a", 20);
                trpl::yield_now().await;
                println!("'a' finished");
            };

            let b = async {
                println!("'b' started");
                slow("b", 75);
                trpl::yield_now().await;
                slow("b", 10);
                trpl::yield_now().await;
                slow("b", 15);
                trpl::yield_now().await;
                slow("b", 350);
                trpl::yield_now().await;
                println!("'b' finished");
            };

            trpl::join(a, b).await;
        });
    }
}

mod streams {
    use trpl::StreamExt;

    pub fn run_streams() {
        trpl::run(async {
            let values = 1..101;
            let iter = values.map(|n| n * 2);
            let stream = trpl::stream_from_iter(iter);

            let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

            while let Some(value) = filtered.next().await {
                println!("The value was: {value}");
            }
        })
    }
}

mod compose {
    use std::{pin::pin, time::Duration};
    use trpl::{ReceiverStream, Stream, StreamExt};

    fn get_messages() -> impl Stream<Item = String> {
        let (tx, rx) = trpl::channel();
        
        trpl::spawn_task(async move {
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
            for (index, message) in messages.into_iter().enumerate() {
                let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                trpl::sleep(Duration::from_millis(time_to_sleep)).await;

                if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                    eprintln!("Cannot send message '{message}': {send_error}");
                    break;
                }
            }
        });

        ReceiverStream::new(rx)
    }

    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            let mut count = 0;
            loop {
                trpl::sleep(Duration::from_millis(1)).await;
                count += 1;
                if let Err(send_error) = tx.send(count) {
                    eprintln!("Cannot send interval '{count}': {send_error}");
                    break;
                }
            }
        });

        ReceiverStream::new(rx)
    }

    pub fn run_compose() {
        trpl::run(async {
            let messages = pin!(get_messages().timeout(Duration::from_millis(200)));
            let intervals = get_intervals()
                .map(|count| format!("Interval: {count}"))
                .throttle(Duration::from_millis(100))
                .timeout(Duration::from_secs(10));
            let merged = messages.merge(intervals).take(20);
            let mut stream = pin!(merged);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(message) => println!("{message}"),
                    Err(reason) => eprintln!("Problem: {reason:?}")
                }
            }
        });
    }
}

fn main() {

    use crate::{async_results, channels, compose, handles, join, page_title, race, streams, timeout, yielding};

    if false {
        page_title::run_page_title();
        handles::run_handles();
        timeout::run_timeout();
        join::run_join();
        channels::run_channels();
        async_results::run_results();
        race::run_race();
        yielding::run_yielding();
        streams::run_streams();
    }
    compose::run_compose();
}