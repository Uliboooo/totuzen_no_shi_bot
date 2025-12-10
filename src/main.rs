use dotenv::dotenv;
use std::env;

use serenity::async_trait;
use serenity::{
    model::{
        application::{Command, Interaction},
        gateway::Ready,
        id::GuildId,
    },
    prelude::*,
};
mod cmds;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, _ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            let name = cmd.data.name.as_str();
            match name {
                cmds::shi::NAME => {
                    if let Err(e) = cmds::shi::slash_execute(&_ctx, &cmd).await {
                        eprintln!("Error: {}", e);
                    }
                }
                _ => {
                    if let Err(why) = cmd
                        .create_response(
                            &_ctx.http,
                            serenity::builder::CreateInteractionResponse::Message(
                                serenity::builder::CreateInteractionResponseMessage::new()
                                    .content("未対応のコマンドです"),
                            ),
                        )
                        .await
                    {
                        println!("スラッシュコマンドの応答に失敗: {why:?}");
                    }
                }
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} として接続しました", ready.user.name);
        // グローバルコマンドとして登録（反映に最大1時間）
        let cmds = cmds::slash_cmds();
        match Command::set_global_commands(&ctx.http, cmds).await {
            Ok(commands) => println!("登録されたグローバルスラッシュコマンド: {commands:#?}"),
            Err(why) => println!("スラッシュコマンド登録に失敗: {why:?}"),
        }

        // 開発用: GUILD_ID が設定されていればギルドコマンドとして即時反映
        if let Ok(guild_id_str) = std::env::var("GUILD_ID")
            && let Ok(id) = guild_id_str.parse::<u64>()
        {
            let guild_id = GuildId::new(id);
            match guild_id.set_commands(&ctx.http, cmds::slash_cmds()).await {
                Ok(commands) => println!("ギルド({id})のスラッシュコマンド: {commands:#?}"),
                Err(why) => println!("ギルドへのスラッシュコマンド登録に失敗: {why:?}"),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // .env をロード

    let token = env::var("DISCORD_TOKEN").expect("環境変数にトークンが必要です (DISCORD_TOKEN)");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("クライアントの作成に失敗しました");

    if let Err(why) = client.start().await {
        println!("クライアントエラー: {:?}", why);
    }
}
