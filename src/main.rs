use std::env;

use libretranslate::{Language, Translate};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.len() > 0 && !msg.author.bot {
            // let source = Language::French;
            // let target = Language::English;
            // let input = msg.content;

            // let data = translate(Language::Detect, target, input, None).await.unwrap();

            let data = msg
                .content
                .to_lang(Language::English)
                .from_lang(Language::Japanese)
                .url("https://libretranslate.de/")
                .translate()
                .await
                .unwrap();

            // let source = format!("Input {}: {}", data.source.as_pretty(), data.input);
            // let translation = format!("Output {}: {}", data.target.as_pretty(), data.output);

            if let Err(why) = msg.channel_id.say(&ctx.http, data).await {
                println!("Error sending message: {:?}?", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
