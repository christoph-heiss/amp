extern crate bloodhound;
extern crate rustbox;
extern crate scribe;

use models::application::modes::open::OpenMode;
use rustbox::Color;
use pad::PadStr;
use view::{Data, StatusLine, View};
use scribe::Buffer;

pub fn display(buffer: Option<&mut Buffer>, data: &Data, mode: &OpenMode, view: &View) {
    // Wipe the slate clean.
    view.clear();

    // Draw the visible set of tokens to the terminal.
    view.draw_tokens(&data);

    // Draw the status line.
    let content = match buffer {
        Some(buf) => {
            match buf.path {
                Some(ref path) => path.to_string_lossy().into_owned(),
                None => String::new(),
            }
        },
        None => String::new(),
    };
    view.draw_status_line(&StatusLine{
        content: content,
        color: None,
    });

    // Draw the list of search results.
    for (line, result) in mode.results.iter().enumerate() {
        let color = if line == mode.selected_index() { view.alt_background_color() } else { Color::Default };
        let padded_content = result.path.as_path().to_str().unwrap().pad_to_width(view.width());
        view.print(0, line, rustbox::RB_NORMAL, Color::Default, color, &padded_content);
    }

    // Draw the divider.
    let line = 5;
    let padded_content = mode.input.pad_to_width(view.width());
    view.print(0, line, rustbox::RB_BOLD, Color::Black, Color::White, &padded_content);

    // Place the cursor on the search input line, right after its contents.
    view.set_cursor(mode.input.len() as isize, 5);

    // Render the changes to the screen.
    view.present();
}