use anyhow::anyhow;
use polyp::{Ui, UserInput};
use std::env;
use tungstenite::Message;
use url::Url;

fn main() -> anyhow::Result<()> {
    let (mut server_websocket, _) = {
        let server_socket_addr = env::args()
            .nth(1)
            .ok_or_else(|| anyhow!("expected argument of server socket address"))?;

        let server_websocket_url = Url::parse(&format!("ws://{}", server_socket_addr))?;

        tungstenite::client::connect(server_websocket_url)?
    };

    println!("kon-polyp: connected to server\r");

    let mut buffer_contents = String::new();

    loop {
        let pressed_character = match server_websocket.read_message()? {
            Message::Binary(json) => {
                let UserInput::PressedKey(pressed_character) = serde_json::from_slice(&json)?;
                pressed_character
            }
            Message::Close(_) => return Ok(()),
            _ => unreachable!(),
        };
        println!("kon-polyp: user pressed ‘{}’\r", pressed_character);

        buffer_contents.push(pressed_character);
        println!("kon-polyp: buffer now is ‘{}’\r", buffer_contents);

        let ui = Ui::TextField {
            current_text: buffer_contents.clone(),
        };
        let serialized = serde_json::to_vec(&ui)?;
        server_websocket.write_message(Message::Binary(serialized))?;
        println!("kon-polyp: wrote UI to server\r");
    }
}
