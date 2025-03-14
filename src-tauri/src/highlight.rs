use syntect::{
    easy::HighlightLines,
    highlighting::{Color, Style, Theme},
    parsing::SyntaxSet,
};

use std::cmp;
use std::fmt::Write;

fn write_css_color(s: &mut String, c: Color) {
    if c.a != 0xFF {
        write!(s, "#{:02x}{:02x}{:02x}{:02x}", c.r, c.g, c.b, c.a).unwrap();
    } else {
        write!(s, "#{:02x}{:02x}{:02x}", c.r, c.g, c.b).unwrap();
    }
}

fn unicode_replace(text: &str) -> String {
    let new_text = text
        .replace(" ", "\u{00A0}")
        .replace("'", "\u{0027}")
        .replace("\"", "\u{0022}")
        .replace(":", "\u{003A}")
        .replace(";", "\u{003B}")
        .replace(">", "\u{003E}")
        .replace("<", "\u{003C}")
        .replace("-", "\u{002D}")
        .replace("(", "\u{0028}")
        .replace(")", "\u{0029}");

    return new_text;
}

fn generate_html(hl_vec: Vec<(Style, &str)>) -> String {
    let mut html: String = "".to_string();
    for item in hl_vec {
        let formatted_text = unicode_replace(item.1);
        let mut css_color = "".to_string();
        write_css_color(&mut css_color, item.0.foreground);
        html = html + "<span style=\"color:" + &css_color + ";\">" + &formatted_text + "</span>";
        // println!("{:?}", item);
    }
    return html;
}


fn generate_html_default(code : String) -> String {
    let formatted_text = unicode_replace(&code);
    return "<span style=\"color: #EEEEEE;\">".to_owned() + &formatted_text + "</span>";
}

#[tauri::command]
pub fn highlight(code: String, ss: SyntaxSet, theme: Theme, filetype : String) -> String {
    if &filetype == "default"{
        let html = generate_html_default(code);
        return html.into(); 
    } else {
        let syntax = ss.find_syntax_by_name(&filetype).expect("error with syntax");
        let mut h = HighlightLines::new(syntax, &theme);
    
        let mut code2 = code;
        let last_index = cmp::max(0, (code2.len() as isize) - 1) as usize;
        let last_char = &code2[last_index..code2.len()];
        if last_char == "\n" {
            code2 = format!("{}{}", code2, "\u{00A0}");
        }
    
        let highlighted = h.highlight_line(&code2, &ss).unwrap();
        let html = generate_html(highlighted);
        return html.into(); 
    }
}
