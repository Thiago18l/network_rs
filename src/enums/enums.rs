use core::fmt;

use log::info;

enum Message {
    Join,
    Leave,
    Text(String),
    Ping,
    Pong,
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::Join => write!(f, "A user has joined the chat"),
            Message::Leave => write!(f, "A user has left the chat"),
            Message::Text(text) => write!(f, "Received message {}", text),
            Message::Ping => write!(f, "Received Ping"),
            Message::Pong => write!(f, "Received Pong"),
        }
    }
}

enum ProtocolMessage {
    Login { username: String, password: String },
    Logout,
    Chat { from: String, message: String },
    Error { code: u16, message: String },
}

impl fmt::Debug for ProtocolMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolMessage::Login { username, password } => write!(f, "{}, {}", username, password),
            ProtocolMessage::Chat { from, message } => write!(f, "{}, {}", from, message),
            ProtocolMessage::Error { code, message } => write!(f, "{}, {}", code, message),
            ProtocolMessage::Logout => write!(f, "Logout")
        }
    }
}

fn enums_overall() {
    let text = "Hello".to_string();
    let message = Message::Text(text);
    info!("Your message is {:#?}", message);

    let login_message = ProtocolMessage::Login { username: "thiago18l".to_string(), password: "my_pass".to_string() };
    let chat = ProtocolMessage::Chat { from: "thiago".to_string(), message: "hello there".to_string() };
    let err = ProtocolMessage::Error { code: 404, message: "user not found".to_string() };
    let logout = ProtocolMessage::Logout;
    info!("{:?}, {:?}, {:?}, {:?}", login_message, chat, err, logout);
}

pub fn process_message() {
    let join = Message::Join;
    let leave = Message::Leave;
    let text = Message::Text("Hello".to_string());
    let ping = Message::Ping;
    let pong = Message::Pong;
    info!("{:?}, {:?}, {:?}, {:?}, {:?}", join, leave, text, ping, pong);
    enums_overall();
}
