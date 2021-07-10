use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!upload" {
            
            if let Err(why) = msg.channel_id.say(&ctx.http, "Uploaded!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = "ODYzMDM2MDYxNDE1MTEyNzI1.YOhC9w.KaxHTAC1P0qbXB9ZnTtXlRxJMXU";
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}