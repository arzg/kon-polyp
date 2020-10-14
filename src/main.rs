use anyhow::anyhow;
use polyp::{Key, Ui, UserInput};
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
    let mut cursor_idx = 0;

    loop {
        let pressed_key = match server_websocket.read_message()? {
            Message::Binary(json) => {
                let UserInput::PressedKey(pressed_character) = serde_json::from_slice(&json)?;
                pressed_character
            }
            Message::Close(_) => return Ok(()),
            _ => unreachable!(),
        };
        println!("kon-polyp: user pressed ‘{:?}’\r", pressed_key);

        match pressed_key {
            Key::Char(c) => {
                buffer_contents.insert(cursor_idx, c);
                cursor_idx += 1;
            }
            Key::Backspace => {
                buffer_contents.remove(cursor_idx - 1);
                cursor_idx -= 1;
            }
            Key::Left => cursor_idx -= 1,
            Key::Right => cursor_idx += 1,
            _ => {}
        }

        println!("kon-polyp: buffer now is ‘{}’\r", buffer_contents);

        let ui = Ui::TextField {
            current_text: buffer_contents.clone(),
            cursor_idx,
        };
        let serialized = serde_json::to_vec(&ui)?;
        server_websocket.write_message(Message::Binary(serialized))?;
        println!("kon-polyp: wrote UI to server\r");
    }
}
