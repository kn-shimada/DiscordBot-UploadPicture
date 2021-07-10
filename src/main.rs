use std::env;
use dotenv::dotenv;

use tokio::{fs::File, io::AsyncWriteExt};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message (&self, ctx: Context, msg: Message) {
        if msg.content == "!upload" {
            for pictures in msg.attachments {
                let picture = match pictures.download().await {
                    Ok(picture) => picture,
                    Err(why) => {
                        println!("Error downloading attachment: {:?}", why);
                        let _ = msg.channel_id.say(&ctx, "Error downloading attachment").await;

                        return;
                    },
                };

                let mut file = match File::create(&pictures.filename).await {
                    Ok(file) => file,
                    Err(why) => {
                        println!("Error creating file: {:?}", why);
                        let _ = msg.channel_id.say(&ctx, "Error creating file").await;

                        return;
                    },
                };

                if let Err(why) = file.write(&picture).await {
                    println!("Error writing to file: {:?}", why);
    
                    return;
                }

                let _ = msg.channel_id.say(&ctx, &format!("Saved {:?}", pictures.filename)).await;
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client =
        Client::builder(&token).event_handler(Handler).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}