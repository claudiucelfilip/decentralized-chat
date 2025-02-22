//! A simple decentralized chat example where all messages get pruned after they remain
//! within the contract over a period of 50 consensus rounds.

use std::error::Error;

use smart_contract_macros::smart_contract;

use smart_contract::log;
use smart_contract::payload::Parameters;
use std::collections::VecDeque;

struct Entry {
    sender: [u8; 32],
    message: String,
}

struct Chat {
    logs: VecDeque<Entry>
}

const MAX_LOG_CAPACITY: usize = 50;
const MAX_MESSAGE_SIZE: usize = 240;

fn prune_old_messages(chat: &mut Chat) {
    if chat.logs.len() > MAX_LOG_CAPACITY {
        chat.logs.pop_front();
    }
}

fn to_hex_string(bytes: [u8; 32]) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    strs.join("")
}

#[smart_contract]
impl Chat {
    fn init(_params: &mut Parameters) -> Self {
        Self { logs: VecDeque::new() }
    }

    fn send_message(&mut self, params: &mut Parameters) -> Result<(), Box<dyn Error>> {
        let entry = Entry { sender: params.sender, message: params.read() };

        // Ensure that messages are not empty.
        if entry.message.len() == 0 {
            return Err("Message must not be empty.".into());
        }

        // Ensure that message are at most 240 characters.
        if entry.message.len() > MAX_MESSAGE_SIZE {
            return Err(format!("Message must not be more than {} characters.", MAX_MESSAGE_SIZE).into());
        }

        // Push chat message into logs.
        self.logs.push_back(entry);

        // Prune old messages if necessary.
        prune_old_messages(self);

        Ok(())
    }

    fn get_messages(&mut self, _params: &mut Parameters) -> Result<(), Box<dyn Error>> {
        let mut messages = Vec::new();

        for entry in &self.logs {
            messages.insert(0, format!("<{}> {}", &to_hex_string(entry.sender)[..16], entry.message));
        }

        log(&messages.join("\n"));

        Ok(())
    }
}