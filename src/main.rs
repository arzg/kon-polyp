use polyp::protocol::Connection;
use polyp::{Key, ProcessletMsg, Ui, UserInput};

fn main() -> anyhow::Result<()> {
    let mut server_connection = Connection::new_from_current_process();

    let mut buffer_contents = String::new();
    let mut cursor_idx = 0;

    loop {
        let pressed_key = match server_connection.recv_message()? {
            ProcessletMsg::UserInput(UserInput::PressedKey(k)) => k,
            ProcessletMsg::Shutdown => {
                eprintln!("kon-polyp: shutting down...\r");
                return Ok(());
            }
        };

        eprintln!("kon-polyp: user pressed ‘{:?}’\r", pressed_key);

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

        eprintln!("kon-polyp: buffer now is ‘{}’\r", buffer_contents);

        let ui = Ui::TextField {
            current_text: buffer_contents.clone(),
            cursor_idx,
        };
        server_connection.send_message(&ui)?;
        eprintln!("kon-polyp: wrote UI to server\r");
    }
}
