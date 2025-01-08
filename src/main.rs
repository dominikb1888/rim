#![warn(clippy::all, clippy::pedantic)]

use rim::editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
}
