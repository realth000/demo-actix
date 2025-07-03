#![allow(unused_variables)]
use actix::{dev::MessageResponse, prelude::*};

#[derive(Message)]
#[rtype(result = "Responses")]
enum Messages {
    Ping,
    Pong,
}

enum Responses {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for Responses
where
    A: Actor,
    M: Message<Result = Responses>,
{
    fn handle(
        self,
        ctx: &mut <A as Actor>::Context,
        tx: Option<dev::OneshotSender<<M as Message>::Result>>,
    ) {
        println!(">>> responses: handle");
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        println!(">>> started");
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!(">>> stopped");
    }
}

impl Handler<Messages> for MyActor {
    type Result = Responses;
    fn handle(&mut self, msg: Messages, ctx: &mut Self::Context) -> Self::Result {
        println!(">>> MyActor: handle");
        match msg {
            Messages::Ping => Responses::GotPing,
            Messages::Pong => Responses::GotPong,
        }
    }
}

#[actix::main]
async fn main() {
    let addr = MyActor.start();

    let ping_future = addr.send(Messages::Ping).await;
    let pong_future = addr.send(Messages::Pong).await;

    match pong_future {
        Ok(res) => match res {
            Responses::GotPing => println!(">>> pong future recv: ping"),
            Responses::GotPong => println!(">>> pong future recv: pong"),
        },
        Err(e) => println!(">>> pong future error: {e:?}"),
    }

    match ping_future {
        Ok(res) => match res {
            Responses::GotPing => println!(">>> ping future recv: ping"),
            Responses::GotPong => println!(">>> ping future recv: pong"),
        },
        Err(e) => println!(">>> ping future error: {e:?}"),
    }
}
