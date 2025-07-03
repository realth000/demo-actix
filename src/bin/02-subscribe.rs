#![allow(unused_variables)]
use actix::prelude::*;

/// The event goes back to all subscribers after [`Ship`] happened.
#[derive(Message)]
#[rtype(result = "()")]
struct ShipCallbackEvent(usize);

/// The source event, triggering all following changes, till subscribers
/// receive [`ShipCallbackEvent`].
#[derive(Message)]
#[rtype(result = "()")]
struct Ship(usize);

/// Subscribe let the notifier hold new subscribers.
#[derive(Message)]
#[rtype(result = "()")]
struct Subscribe {
    pub source: Recipient<ShipCallbackEvent>,
    name: &'static str,
}

/// The main notifier as the main work of all process.
///
/// 1. Add subscribers to it.
/// 2. Add [`Ship`] event.
/// 3. Notifier notify all subscribers for each [`Ship`] event.
struct OrderNotifier {
    recorded_subscribers: Vec<Recipient<ShipCallbackEvent>>,
}

impl OrderNotifier {
    fn new() -> Self {
        OrderNotifier {
            recorded_subscribers: vec![],
        }
    }
}

impl Actor for OrderNotifier {
    type Context = Context<Self>;
}

impl OrderNotifier {
    fn notify(&mut self, order_id: usize) {
        println!(">>> Notifier: notify all subscribers, new order id = {order_id} incoming");
        for sub in &self.recorded_subscribers {
            sub.do_send(ShipCallbackEvent(order_id));
        }
    }
}

/// Remember new subscribers.
impl Handler<Subscribe> for OrderNotifier {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, ctx: &mut Self::Context) -> Self::Result {
        println!(">>> Notifier: remember new subscriber: {}", msg.name);
        self.recorded_subscribers.push(msg.source);
    }
}

/// What to do when [`Ship`] triggered.
impl Handler<Ship> for OrderNotifier {
    type Result = ();

    fn handle(&mut self, msg: Ship, ctx: &mut Self::Context) -> Self::Result {
        // Notify all subscribers that a new `Ship` event came in.
        self.notify(msg.0);
        System::current().stop();
    }
}

/// One type of subscriber.
struct EmailSubscriber;

impl Actor for EmailSubscriber {
    type Context = Context<Self>;
}

impl Handler<ShipCallbackEvent> for EmailSubscriber {
    type Result = ();

    fn handle(&mut self, msg: ShipCallbackEvent, ctx: &mut Self::Context) -> Self::Result {
        println!(">>> email sent for order {}", msg.0);
    }
}

/// Another type of subscriber.
struct SmsSubscriber;

impl Actor for SmsSubscriber {
    type Context = Context<Self>;
}

impl Handler<ShipCallbackEvent> for SmsSubscriber {
    type Result = ();

    fn handle(&mut self, msg: ShipCallbackEvent, ctx: &mut Self::Context) -> Self::Result {
        println!(">>> sms sent for order {}", msg.0);
    }
}

#[actix::main]
async fn main() -> Result<(), actix::MailboxError> {
    let email_sub = EmailSubscriber {}.start();
    let sms_sub = SmsSubscriber {}.start();

    // Notifier is the status here.
    let notifier = OrderNotifier::new().start();

    notifier
        .send(Subscribe {
            source: email_sub.recipient(),
            name: "I am email subscriber. I want to subscribe",
        })
        .await?;
    notifier
        .send(Subscribe {
            source: sms_sub.recipient(),
            name: "I am sms subscriber, I want to subscribe",
        })
        .await?;
    notifier.send(Ship(1)).await?;
    notifier.send(Ship(2)).await?;

    Ok(())
}
