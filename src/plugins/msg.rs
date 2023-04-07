//!
//! Copyright (C) 2023 cyberknight777
//!
//! SPDX-License-Identifier: MIT
//!

use grammers_client::{
    Client,
    types::{InputMessage, Message}
};

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub async fn knightcmd_msg(client: Client, message: Message, text: String) -> Result {
    if let Some(id) = message.reply_to_message_id() {
	client.send_message(message.chat(), InputMessage::markdown(text.trim().replace(r#"\n"#, "  \n")).reply_to(Some(id))).await?;
    } else {
	message.reply(InputMessage::markdown(text.trim().replace(r#"\n"#, "  \n"))).await?;
    }
    return Ok(());
}
