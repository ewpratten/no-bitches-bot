use imgur2018::imgur_upload;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        gateway::{Activity, Ready},
    },
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
                        log::debug!("Detected an image");

                        // Start typing to indicate to the user that we're processing their request
                        let typing = msg.channel_id.start_typing(&ctx.http);
                        if let Err(why) = typing {
                            println!("Error sending typing start: {:?}", why);
                        }

                        // Get the current timestamp
                        let timestamp = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        log::debug!("Current timestamp is: {}", timestamp);

                        // Build our filepaths
                        let download_file_path = format!("/tmp/bitches_image_{}.jpg", timestamp);
                        let output_file_path =
                            format!("/tmp/bitches_image_mutated_{}.jpg", timestamp);
                        log::info!(
                            "Using files: {} -> {}",
                            download_file_path,
                            output_file_path
                        );

                        // Download the image
                        let image_url = attachment.url.clone();
                        let result = Command::new("wget")
                            .arg("-O")
                            .arg(&download_file_path)
                            .arg(image_url)
                            .output()
                            .await
                            .expect("Failed to execute process");

                        // If we failed to download the image, we can't do anything else
                        if !result.status.success() {
                            msg.reply_ping(&ctx.http, "Your image could not be processed")
                                .await
                                .unwrap();
                        }

                        // Spawn the Python script to mutate the image
                        let result = Command::new("python3")
                            .arg("src/memeifyer.py")
                            .arg("-i")
                            .arg(download_file_path)
                            .arg("-o")
                            .arg(&output_file_path)
                            .output()
                            .await
                            .expect("Failed to execute process");

                        // Print the output of the script
                        println!("{}", String::from_utf8_lossy(&result.stdout));

                        // If we failed to process the image, we can't do anything else
                        if !result.status.success() {
                            msg.reply_ping(
                                &ctx.http,
                                "Your image could not be edited. There probably aren't any recognizable faces in it.",
                            )
                            .await
                            .unwrap();
                        }

                        // Upload the mutated image
                        let image_data =
                            std::fs::read(output_file_path).expect("Failed to read image");
                        let new_image_url = imgur_upload("725631460b74631", image_data)
                            .await
                            .expect("Failed to upload image");

                        msg.reply_ping(&ctx.http, new_image_url.to_string())
                            .await
                            .unwrap();
                        return;
                    }
                }
            }

            // If we got here, we didn't find an image
            log::debug!("No image found");
            msg.reply_ping(
                &ctx.http,
                "This command requires one image to be attached to your message",
            )
            .await
            .unwrap();
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);
        ctx.set_activity(Activity::watching("Your girl sleep at night"))
            .await;
    }
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Set up Fern
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        }).filter(|meta| {
            meta.target().starts_with("no_bitches_bot") 
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    // Create a new instance of the Client
    log::debug!("Creating new client");
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start the bot
    log::debug!("Starting bot");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
