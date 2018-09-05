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
    next: Option<Addr<MyActor>>,
}

/// Declare actor and its context
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("{}", self.count);
    }
}

/// Handler for `Prime` message
impl Handler<Prime> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Prime, _: &mut Context<Self>) -> Self::Result {

        let test = self.next.clone();

        if msg.0 % self.count != 0 {

            if let Some(addr) = test {
                    addr.do_send(Prime(msg.0));
            } else {
                println!("Starting actor {}", msg.0);
                let addr = MyActor { count: msg.0, next: None }.start();
                self.next = Some(addr)
            }
        }

        self.count
    }


}

fn main() {
    // start system, this is required step
    System::run(|| {
        // start new actor
        let addr = MyActor { count: 2, next: None }.start();

        let daddr = addr.clone();

    thread::spawn(move|| {
        for i in 2..100000 {
            daddr.do_send(Prime(i));
            thread::sleep(Duration::from_millis(1));
        }
    });


    });
}