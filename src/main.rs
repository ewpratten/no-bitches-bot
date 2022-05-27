use imgur2018::imgur_upload;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{channel::Message, gateway::Ready},
    prelude::GatewayIntents,
    Client,
};
use tokio::process::Command;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "?bitches" {
            // We can only process the message if it has an attached image
            for attachment in &msg.attachments {
                if let Some(content_type) = &attachment.content_type {
                    if content_type.contains("image") {
                        // Start typing to indicate to the user that we're processing their request
                        let typing = msg.channel_id.start_typing(&ctx.http);
                        if let Err(why) = typing {
                            println!("Error sending typing start: {:?}", why);
                        }

                        // Download the image
                        let image_url = attachment.url.clone();
                        Command::new("wget")
                            .arg("-O")
                            .arg("/tmp/bitches_image.jpg")
                            .arg(image_url)
                            .output().await
                            .expect("Failed to execute process");

                        // Spawn the Python script to mutate the image
                        Command::new("python3")
                            .arg("src/memeifyer.py")
                            .arg("-i")
                            .arg("/tmp/bitches_image.jpg")
                            .arg("-o")
                            .arg("/tmp/bitches_image_mutated.jpg")
                            .output().await
                            .expect("Failed to execute process");

                        // Upload the mutated image
                        let image_data = std::fs::read("/tmp/bitches_image_mutated.jpg")
                            .expect("Failed to read image");
                        let new_image_url = imgur_upload("725631460b74631", image_data)
                            .await
                            .expect("Failed to upload image");

                        if let Err(why) = msg.reply_ping(&ctx.http, new_image_url.to_string()).await
                        {
                            println!("Error sending message: {:?}", why);
                        }
                        return;
                    }
                }
            }

            // If we got here, we didn't find an image
            if let Err(why) = msg
                .reply_ping(
                    &ctx.http,
                    "This command requires one image to be attached to your message",
                )
                .await
            {
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
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start the bot
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
