#![allow(unused_variables)]
use actix::{dev::MessageResponse, prelude::*};

/// Message is the content sent to actor.
#[derive(Message)]
#[rtype(result = "Responses")]
enum Messages {
    Ping,
    Pong,
}

/// Response is what the actor will do when new message received.
enum Responses {
    GotPing,
    GotPong,
}

/// Make [`Responses`] works like response.
///
/// That is, for every Actor type, if it wants send a [`Responses`] back as result.
///
/// To limit actor type and message types, specify generic type parameter.
///
/// ```no_run
/// impl MessageResponse<MyActor, Messages> for Responses {
///     fn handle(
///         self,
///         ctx: &mut <MyActor as Actor>::Context,
///         tx: Option<dev::OneshotSender<<Messages as Message>::Result>>,
///     ) {
///         println!(">>> responses: handle");
///         if let Some(tx) = tx {
///             let _ = tx.send(self);
///         }
///     }
/// }
/// ```
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

/// Actor is the worker receive [`Messages`] and then do something [`Responses`].
struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    /// Actor has its own lifetime.
    ///
    /// Work process started.
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        println!(">>> started");
    }

    /// Actor has its own lifetime.
    ///
    /// Work process stopped.
    fn stopped(&mut self, ctx: &mut Self::Context) {
        println!(">>> stopped");
    }
}

/// Define what to do when Actor receive [`Messages`].
///
/// When [`MyActor`] receives [`Messages`], return a [`Responses`] back.
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

    // Messages sent to actor, use the returned future to check response.
    //
    // The returned type is defined in `Handler`.
    let ping_future = addr.send(Messages::Ping).await;
    let pong_future = addr.send(Messages::Pong).await;

    // Check response.
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
