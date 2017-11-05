use std;
use futures::{Stream, Future};
use tokio_core;
use tokio_timer;
use multiqueue;

#[derive(Clone, Debug)]
enum Message {
    Tick(usize),
    Text(String),
}

//TODO: This will probably require a macro or callback to inject user code
#[test]
fn my_test() {
    let (tx, rx) = multiqueue::broadcast_fut_queue(60000);

    // Here's the magic
    spawn_understudy(tx.clone(), rx.add_stream(), printer);
    spawn_understudy(tx.clone(), rx.add_stream(), messenger);
    spawn_understudy(tx.clone(), rx.add_stream(), hey_thing);
    
    let delay = std::time::Duration::from_millis(1500);
    let mut count = 0usize;
    loop {
        tx.try_send(Message::Tick(count)).expect("Failed to send");
        std::thread::sleep(delay);
        count += 1;
    }
}

fn spawn_understudy<T: Clone + 'static>(tx: multiqueue::BroadcastFutSender<T>, 
                       rx: multiqueue::BroadcastFutReceiver<T>, 
                       function: (fn(T, multiqueue::BroadcastFutSender<T>) -> Result<(), ()>)) {

    std::thread::spawn(move || {
        let mut core = tokio_core::reactor::Core::new().expect("Could not instantiate reactor");
        let handle = core.handle();

        let incoming = rx.for_each(|message| {
            function(message, tx.clone())
        });

        core.run(incoming).expect("Could not run reactor event loop")
    });
}

fn printer(message: Message, tx: multiqueue::BroadcastFutSender<Message>) -> Result<(), ()> {
    println!("{:?}", message);
    Ok(())
}

fn messenger(message: Message, tx: multiqueue::BroadcastFutSender<Message>) -> Result<(), ()> {
    match message {
        Message::Tick(count) => { tx.try_send(Message::Text(format!("Message #{}", count))); },
        _ => {},
    }
    Ok(())
}

fn hey_thing(message: Message, tx: multiqueue::BroadcastFutSender<Message>) -> Result<(), ()> {
    match message {
        Message::Tick(_) => { tx.try_send(Message::Text("Hey Hey Hey".to_string())); },
        _ => {},
    }
    Ok(())
}

/*fn messenger(message: (), tx: multiqueue::BroadcastFutSender<String>) -> Future<Item=(), Error=tokio_timer::TimerError> {
    static mut count: usize = 0;
    tx.try_send(format!("{} #{}", "Message", count)).expect("Failed to send");
    unsafe {
        count += 1;
    }
    Ok(())

fn my_test() {
    let (tx, rx) = multiqueue::broadcast_fut_queue(60000);

    spawn_understudy(tx.clone(), rx.add_stream(), printer);

    let timer = tokio_timer::Timer::default();
    let delay = std::time::Duration::from_millis(100);
    let interval = timer.interval(delay);

    spawn_understudy(tx.clone(), interval, messenger);
}

fn spawn_understudy<T: Clone + 'static, R: 'static, RE, S: Stream<Item=R, Error=RE> + 'static + Send, F: Future<Item=R, Error=RE>>(tx: multiqueue::BroadcastFutSender<T>, 
                       rx: S, 
                       function: (fn(R, multiqueue::BroadcastFutSender<T>) -> F)) {

    std::thread::spawn(move || {
        let mut core = tokio_core::reactor::Core::new().expect("Could not instantiate reactor");
        let handle = core.handle();

        let incoming = rx.for_each(|message| {
            function(message, tx)
        });

        core.run(incoming);
    });
}
}*/