use polyp::{Key, Ui, UserInput};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let mut buffer_contents = String::new();
    let mut cursor_idx = 0;

    loop {
        let UserInput::PressedKey(pressed_key) = {
            let mut buf = String::new();
            io::stdin().read_line(&mut buf)?;
            serde_json::from_slice(buf.as_bytes())?
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
        let serialized = serde_json::to_vec(&ui)?;
        io::stdout().write_all(&serialized)?;
        io::stdout().write_all(b"\n")?;
        eprintln!("kon-polyp: wrote UI to server\r");
    }
}
