use std::{collections::HashMap, path::PathBuf, str::FromStr};

use serde::Deserialize;
use tauri::path::BaseDirectory;
use tauri::Manager;

use std::fs;

use syntect::{
    highlighting::{
        Color, FontStyle, ParseThemeError, ScopeSelectors, StyleModifier, Theme, ThemeItem,
        ThemeSet, ThemeSettings, UnderlineOption,
    },
    parsing::{ParseScopeError, SyntaxSet, SyntaxSetBuilder},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse JSON")]
    Json(#[from] serde_json::Error),
    #[error("Failed to parse scope")]
    ParseScope(#[from] ParseScopeError),
    #[error("Failed to parse theme")]
    ParseTheme(#[from] ParseThemeError),
}

#[derive(Deserialize)]
pub struct Rule {
    pub name: Option<String>,
    pub scope: String,
    pub font_style: Option<String>,
    pub foreground: Option<String>,
    pub background: Option<String>,
}

#[derive(Deserialize)]
pub struct ColorScheme {
    pub name: Option<String>,
    pub author: Option<String>,
    pub globals: HashMap<String, String>,
    pub rules: Vec<Rule>,
}
impl FromStr for ColorScheme {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(ParseError::Json)
    }
}
impl TryFrom<ColorScheme> for Theme {
    type Error = ParseError;
    fn try_from(value: ColorScheme) -> Result<Self, Self::Error> {
        let mut settings = ThemeSettings::default();
        for (key, value) in &value.globals {
            match &key[..] {
                "foreground" => settings.foreground = Color::from_str(value).ok(),
                "background" => settings.background = Color::from_str(value).ok(),
                "caret" => settings.caret = Color::from_str(value).ok(),
                "line_highlight" => settings.line_highlight = Color::from_str(value).ok(),
                "misspelling" => settings.misspelling = Color::from_str(value).ok(),
                "minimap_border" => settings.minimap_border = Color::from_str(value).ok(),
                "accent" => settings.accent = Color::from_str(value).ok(),

                "popup_css" => settings.popup_css = Some(value.clone()),
                "phantom_css" => settings.phantom_css = Some(value.clone()),

                "bracket_contents_foreground" => {
                    settings.bracket_contents_foreground = Color::from_str(value).ok()
                }
                "bracket_contents_options" => {
                    settings.bracket_contents_options = UnderlineOption::from_str(value).ok()
                }
                "brackets_foreground" => settings.brackets_foreground = Color::from_str(value).ok(),
                "brackets_background" => settings.brackets_background = Color::from_str(value).ok(),
                "brackets_options" => {
                    settings.brackets_options = UnderlineOption::from_str(value).ok()
                }
                "tags_foreground" => settings.tags_foreground = Color::from_str(value).ok(),
                "tags_options" => settings.tags_options = UnderlineOption::from_str(value).ok(),
                "highlight" => settings.highlight = Color::from_str(value).ok(),
                "find_highlight" => settings.find_highlight = Color::from_str(value).ok(),
                "find_highlight_foreground" => {
                    settings.find_highlight_foreground = Color::from_str(value).ok()
                }
                "gutter" => settings.gutter = Color::from_str(value).ok(),
                "gutter_foreground" => settings.gutter_foreground = Color::from_str(value).ok(),
                "selection" => settings.selection = Color::from_str(value).ok(),
                "selection_foreground" => {
                    settings.selection_foreground = Color::from_str(value).ok()
                }
                "selection_border" => settings.selection_border = Color::from_str(value).ok(),
                "inactive_selection" => settings.inactive_selection = Color::from_str(value).ok(),
                "inactive_selection_foreground" => {
                    settings.inactive_selection_foreground = Color::from_str(value).ok()
                }
                "guide" => settings.guide = Color::from_str(value).ok(),
                "active_guide" => settings.active_guide = Color::from_str(value).ok(),
                "stack_guide" => settings.stack_guide = Color::from_str(value).ok(),
                "shadow" => settings.shadow = Color::from_str(value).ok(),
                _ => (), // E.g. "shadowWidth" and "invisibles" are ignored
            }
        }

        Ok(Self {
            name: value.name,
            author: value.author,
            settings,
            scopes: value
                .rules
                .into_iter()
                .map(ThemeItem::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl TryFrom<Rule> for ThemeItem {
    type Error = ParseError;
    fn try_from(value: Rule) -> Result<Self, Self::Error> {
        Ok(Self {
            scope: ScopeSelectors::from_str(&value.scope)?,
            style: StyleModifier {
                foreground: value.foreground.map(|s| Color::from_str(&s)).transpose()?,
                background: value.background.map(|s| Color::from_str(&s)).transpose()?,
                font_style: value
                    .font_style
                    .map(|s| FontStyle::from_str(&s))
                    .transpose()?,
            },
        })
    }
}

fn get_theme_set(color_path: PathBuf) -> Result<ThemeSet, ParseError> {
    let mut set = ThemeSet::load_defaults();
    let path_literal: String = color_path.to_str().expect("Color path error 2").to_string();
    let contents = fs::read_to_string(path_literal).expect("Error with parsing color file");
    let color_scheme = ColorScheme::from_str(&contents)?;
    set.themes.insert("aleo".to_string(), color_scheme.try_into()?);
    Ok(set)
}

// Function parameter `code` is provided because Rust doesn't allow function calls with no arguments, it isn't meant to be used
// Tauri: When using a Rust frontend to call invoke() without arguments, you will need to adapt your frontend code as below. The reason is that Rust doesn’t support optional arguments.
// See the Basic Example - WASM section of https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn load_leo_syntax(handle: tauri::AppHandle, _code: String) -> (SyntaxSet, Theme) {
    let color_path = handle
        .path()
        .resolve(
            "resources/leo.sublime-color-scheme",
            BaseDirectory::Resource,
        )
        .expect("Color path error");
    let theme_set = get_theme_set(color_path).unwrap();
    let theme = theme_set.themes.get("aleo").expect("Error with Theme");

    let mut builder = SyntaxSetBuilder::new();

    let syntax_path = handle
        .path()
        .resolve("resources/leo.sublime-syntax", BaseDirectory::Resource)
        .expect("Error with syntax path");
    builder.add_from_folder(syntax_path, true).unwrap();
    let ss = builder.build();

    return (ss.clone(), theme.clone());
}

// Function parameter `code` is provided because Rust doesn't allow function calls with no arguments, it isn't meant to be used
// Tauri: When using a Rust frontend to call invoke() without arguments, you will need to adapt your frontend code as below. The reason is that Rust doesn’t support optional arguments.
// See the Basic Example - WASM section of https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn load_aleo_syntax(handle: tauri::AppHandle, _code: String) -> (SyntaxSet, Theme) {
    let color_path = handle
        .path()
        .resolve(
            "resources/aleo.sublime-color-scheme",
            BaseDirectory::Resource,
        )
        .expect("Color path error");
    let theme_set = get_theme_set(color_path).unwrap();
    let theme = theme_set.themes.get("aleo").expect("Error with Theme");

    let mut builder = SyntaxSetBuilder::new();

    let syntax_path = handle
        .path()
        .resolve("resources/aleo.sublime-syntax", BaseDirectory::Resource)
        .expect("Error with syntax path");
    builder.add_from_folder(syntax_path, true).unwrap();
    let ss = builder.build();

    return (ss.clone(), theme.clone());
}
