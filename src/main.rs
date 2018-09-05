extern crate actix;
extern crate futures;
extern crate tokio;

use actix::prelude::*;
use futures::Future;

use std::thread;
use std::time::Duration;

/// Define `Prime` message
struct Prime(usize);

impl Message for Prime {
    type Result = usize;
}

/// Actor
struct MyActor {
    count: usize,
    printed: bool,
    next: Option<Addr<MyActor>>,
}

/// Declare actor and its context
impl Actor for MyActor {
    type Context = Context<Self>;
}

/// Handler for `Prime` message
impl Handler<Prime> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Prime, _: &mut Context<Self>) -> Self::Result {

        if self.printed == false {
            println!("{}", self.count);
            self.printed = true;
            return self.count;
        }

        let test = self.next.clone();

        if let Some(addr) = test {
            if msg.0 % self.count != 0 {
                addr.do_send(Prime(msg.0));
            }
        } else {
            println!("Starting actor {}", msg.0);
            let addr = MyActor { count: msg.0, printed: false, next: None }.start();

            // Initial 'handle' trigger
            addr.do_send(Prime(100));
            self.next = Some(addr)
        }

        self.count
    }
}

fn main() {
    // start system, this is required step
    System::run(|| {
        // start new actor
        let addr = MyActor { count: 2, printed: false, next: None }.start();

        let daddr = addr.clone();

    thread::spawn(move|| {
        for i in 2..40 {
            daddr.do_send(Prime(i));
        }
    });


    });
}