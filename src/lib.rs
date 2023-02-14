use crate::document::Document;
use crate::editor::{Editor, EditorState};
use editor::Position;
use notan::draw::*;
use notan::prelude::*;
use coolor::Hsl;

mod document;
mod editor;

#[notan_main]
pub fn run() -> Result<(), String> {
    notan::init_with(setup)
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

    let font2 = gfx
        .create_font(include_bytes!("../assets/Kenney Future.ttf"))
        .expect("Couldn't Load font!");

    let font3 = gfx
        .create_font(include_bytes!("../assets/Monocraft.ttf"))
        .expect("Couldn't Load font!");

    let font4 = gfx
        .create_font(include_bytes!("../assets/FiraCode-Regular.ttf"))
        .expect("Couldn't Load font!");

    Editor {
        font,
        font2,
        font3,
        font4,
        doc: Some(Document::empty()),
        state: EditorState::Splash,
        cursor_pos: Position { x: 0, y: 0 },
        quit: false,
        color_offset: 0.0,
    }
}

fn frame(app: &mut App, editor: &mut Editor) {
    match editor.state {
        EditorState::Splash => {
            if app.timer.time_since_init() > 1.0 {
                editor.state = EditorState::Edit;
            }
        }
        EditorState::Edit => {
            if editor.quit {
                app.exit();
            }
            if editor.color_offset >= 360.0 {
                editor.color_offset = 0.0;
            }
            editor.color_offset += 50.0 * app.timer.delta_f32();
        }
        _ => {}
    }
}

fn event(editor: &mut Editor, evt: Event) {
    match evt {
        Event::KeyDown { key } => {
            editor.process_key(key);
        }
        Event::ReceivedCharacter(c) if c != '\u{7f}' && c != '\u{8}' => {
            editor.process_char(c);
        }
        _ => {}
    }
}

fn draw(gfx: &mut Graphics, editor: &mut Editor) {
    let mut draw = gfx.create_draw();

    match editor.state {
        EditorState::Splash => {
            draw.text(&editor.font, "Crsa Editor ")
                .position(400.0, 30.0)
                .h_align_center()
                .v_align_middle()
                .color(Color::GREEN)
                .size(30.0);

            draw.text(&editor.font, "Version: 0.1.0")
                .position(500.0, 30.0)
                .size(20.0)
                .color(Color::ORANGE);
        }
        EditorState::Edit => {
            // Render Cursor
            draw.rect(
                (
                    30.0 + (25.0 * *&editor.cursor_pos.x as f32),
                    (editor.cursor_pos.y as f32 * 50.0),
                ),
                (25.0, 50.0),
            );

            // Render Document content
            /*
            draw.text(
                &editor.font4,
                &editor.doc.as_mut().expect("No document to draw!").as_str(),
            )
            .position(30.0, 0.0)
            .color(Color::AQUA)
            .size(50.0);
            */

            for (row, content) in editor.doc.as_mut().expect("No document to draw!").lines.iter().enumerate() {
                let row_color = ((row as f32 + 2.0) * &editor.color_offset).clamp(1.0, 359.0);
                let row_color = Hsl::new(row_color, 1.0, 0.5);
                let (r, g, b) = row_color.to_rgb().parts();
                let row_color = Color::from_rgb(r, g, b);
                //eprintln!("Row color: {}", row_color);
                draw.text(
                    &editor.font4,
                    &content.content,
                    )
                    .position(30.0, row as f32 * 50.0)
                    .color(row_color)
                    .size(50.0);

            }
        }
        _ => {}
    }
    draw.clear(Color::BLACK);

    gfx.render(&draw);
}

fn window_config() -> WindowConfig {
    WindowConfig::default()
        .size(800, 600)
        .min_size(400, 300)
        .resizable(true)
        .vsync(true)
        .title("crsa editor")
}
