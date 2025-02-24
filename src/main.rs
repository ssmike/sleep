use teloxide::{dispatching::dialogue::GetChatId, prelude::*, repls::CommandReplExt, utils::command::BotCommands};


#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
enum Command {
    Sleep,
    CheckLoad,
    Ping,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    if msg.chat.username() != Some("ssmike") {
        return Ok(())
    }
    match cmd {
        Command::Sleep => {
            if let Err(e) = std::process::Command::new("systemctl")
                .args(["suspend"])
                .output()
            {
                log::error!("error spawning systemctl {e}")
            }
        }
        Command::CheckLoad => {
            let response = match std::process::Command::new("free").args(["-m"]).output() {
                Ok(output) => String::from_utf8(output.stdout).expect("failed to decode output"),
                Err(err) => {
                    let mut result = String::from("error: ");
                    result.push_str(&err.to_string());
                    result
                }
            };
            if let Some(chat) = msg.chat_id() {
                bot.send_message(chat, response).await?;
            }
        },
        Command::Ping => {
            if let Some(chat) = msg.chat_id() {
                bot.send_message(chat, "pong").await?;
            }
        }
    };
    Ok(())
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting sleep bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await
}
