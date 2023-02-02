use crate::document::Document;
use crate::editor::{Editor, EditorState};
use editor::Position;
use notan::draw::*;
use notan::prelude::*;
use notan::text::*;

mod document;
mod editor;

#[notan_main]
pub fn run() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(TextConfig)
        .add_config(DrawConfig)
        .add_config(window_config())
        .event(event)
        .update(frame)
        .draw(draw)
        .build()
}

fn setup(gfx: &mut Graphics) -> Editor {
    let font = gfx
        .create_font(include_bytes!("../assets/Kenney Blocks.ttf"))
        .expect("Couldn't Load font!");

    Editor {
        font,
        doc: Some(Document::empty()),
        state: EditorState::Splash,
        cursor_pos: Position { x: 0, y: 0 },
    }
}

fn frame(app: &mut App, editor: &mut Editor) {
    match editor.state {
        EditorState::Splash => {
            if app.timer.time_since_init() > 5.0 {
                editor.state = EditorState::Edit;
            }
        }
        _ => {}
    }
}

fn event(editor: &mut Editor, evt: Event) {
    match evt {
        Event::ReceivedCharacter(c) if c != '\u{7f}' => {
            editor.process_char(c);
        }
        _ => {}
    }
}

fn draw(gfx: &mut Graphics, editor: &mut Editor) {
    let mut text = gfx.create_text();
    text.clear_color(Color::BLACK);

    match editor.state {
        EditorState::Splash => {
            text.add("Crsa Editor ")
                .font(&editor.font)
                .position(400.0, 30.0)
                .h_align_center()
                .color(Color::GREEN)
                .size(30.0);

            text.chain("Version: 0.1.0").size(20.0).color(Color::ORANGE);
        }
        EditorState::Edit => {
            text.add("")
                .font(&editor.font)
                .position(gfx.size().0 as f32 / 2.0, gfx.size().1 as f32 / 2.0)
                .h_align_center()
                .v_align_middle()
                .color(Color::AQUA)
                .size(60.0);

            for line in &editor.doc.as_mut().unwrap().lines {
                text.chain(&line.content).color(Color::AQUA).size(60.0);
                text.chain("\n");
            }
        }
        _ => {}
    }

    gfx.render(&text);
}

fn window_config() -> WindowConfig {
    WindowConfig::default()
        .size(800, 600)
        .min_size(400, 300)
        .resizable(true)
        .vsync(true)
        .title("crsa editor")
}
