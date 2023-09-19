use inputbot::KeybdKey;
use std::{thread::sleep, time::Duration};
use arboard::Clipboard;

const MODIFIERS: [KeybdKey; 3] = [
    KeybdKey::LControlKey,
    KeybdKey::RControlKey,
    KeybdKey::RAltKey
];

pub fn listen() {
    KeybdKey::bind_all(|key| {
        if KeybdKey::RAltKey.is_pressed() {
            match key {
                KeybdKey::EKey => {
                    release_modidiers();

                    let mut clipboard = Clipboard::new().unwrap();
                    let original_text = clipboard.get_text().unwrap();

                    sleep(Duration::from_millis(1));

                    press_with_modifier(
                        KeybdKey::LControlKey,
                        KeybdKey::CKey
                    );

                    let clipped_text = clipboard.get_text().unwrap();
                    let use_empty = clipped_text == original_text;

                    let mut paste = "[".to_owned();
                    if !use_empty {
                        paste.push_str(&clipped_text);
                    }
                    paste.push_str("]");

                    clipboard.set_text(paste).unwrap();

                    press_with_modifier(
                        KeybdKey::LControlKey,
                        KeybdKey::VKey
                    );

                    clipboard.set_text(original_text).unwrap();
                },
                KeybdKey::RKey => {
                    release_modidiers();

                    let mut clipboard = Clipboard::new().unwrap();
                    let original_text = clipboard.get_text().unwrap();

                    sleep(Duration::from_millis(1));

                    press_with_modifier(
                        KeybdKey::LControlKey,
                        KeybdKey::CKey
                    );

                    let clipped_text = clipboard.get_text().unwrap();
                    let use_empty = clipped_text == original_text;

                    let mut paste = "{".to_owned();
                    if !use_empty {
                        paste.push_str(&clipped_text);
                    }
                    paste.push_str("}");

                    clipboard.set_text(paste).unwrap();

                    press_with_modifier(
                        KeybdKey::LControlKey,
                        KeybdKey::VKey
                    );

                    clipboard.set_text(original_text).unwrap();
                },
                KeybdKey::MinusKey => {
                    println!("Closing..");
                    release_modidiers();
                    std::process::exit(1);
                },
                _ => (),
            };
        }
    });
    inputbot::handle_input_events();
}

pub fn press(key: KeybdKey, release_modifiers: bool) {
    if release_modifiers {
        release_modidiers()
    }

    key.press();
    sleep(Duration::from_millis(1));
    key.release();
}

pub fn press_with_modifier(modifier: KeybdKey, key: KeybdKey) {
    modifier.press();
    sleep(Duration::from_millis(30));
    press(key, false);
    release_modidiers();
}

pub fn release_modidiers() {
    for key in &MODIFIERS {
        key.release();
    }
}
