use crate::app::App;
use crate::enums::Menu;
use crate::structs::{Key, Themes};

use ratatui::prelude::*;

use super::error_screen::render_error;
use super::fullscreen_player::render_player_in_fullscreen;
use super::help::{render_default_help, render_help};
use super::library::{render_default_library, render_library};
use super::main_area::render_main_area;
use super::new_release::{render_default_new_releases, render_new_releases};
use super::player::render_player;
use super::playlist_control::add_to_playlist::render_add_track_to_playlist_screen;
use super::search::search::{render_default_search, render_search};
use super::user_playlist::{render_default_user_playlist, render_user_playlist};

/// Renders the main frame of the application's user interface
pub fn render_frame(
    f: &mut Frame,
    selected_menu: Menu,
    app: &mut App,
    key: &mut Key,
    theme: &mut Themes,
) {
    // Calculate the layout constraints
    let size = f.size();

    // Whole display layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(7),
            Constraint::Percentage(83),
            Constraint::Percentage(10),
        ])
        .split(size);

    // Dividing the header into two horizontal layouts: Search section and Help section
    let header_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[0]);

    // Dividing the middle layout into three horizontal layouts: Library/New Release section, Main screen section and User Playlist section
    let content_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(chunks[1]);

    let front_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(content_chunk[1]);

    // Dividing the fist portion of middle layout into two vertical layouts: Library section and New Release section
    let content_sub_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(content_chunk[0]);

    // Dividing the middle screen into 2 vertical layout which will be firther divided into two horizontal layouts each to have the middle screen be divided into 4 sections
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_chunk[1]);

    // Dividing the middle screen upper layout into two section: Songs and Artist section
    let main_chunk_upper = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[0]);

    // Dividing the midlle screen lower layout into two section: Album and Playlist section
    let main_chunk_lower = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunk[1]);

    // Making the live player layout
    let player_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    let player_fullscreen_vertical_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(43),
            Constraint::Percentage(14),
            Constraint::Percentage(43),
        ])
        .split(size);

    let player_fullscreen_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(player_fullscreen_vertical_chunk[1]);

    // Render the default UI
    render_default_search(f, &header_chunk, theme);
    render_default_library(f, &content_sub_chunk, theme);
    render_default_user_playlist(f, &content_chunk, app, theme);
    render_player(f, &player_layout, app, theme);
    render_main_area(f, &content_chunk, &front_chunk, app, theme);
    render_default_help(f, &header_chunk, theme);
    render_default_new_releases(f, &content_sub_chunk, app, theme);

    // Render different sections based on the selected menu
    match selected_menu {
        Menu::Default => {
            render_main_area(f, &content_chunk, &front_chunk, app, theme);
        }
        Menu::Main => {
            render_main_area(f, &content_chunk, &front_chunk, app, theme);
        }
        Menu::Library => {
            render_library(f, &content_sub_chunk, &content_chunk, app, theme);
        }
        Menu::Playlists => {
            render_user_playlist(f, &content_chunk, app, theme);
        }
        Menu::Search => {
            render_search(
                f,
                &header_chunk,
                &main_chunk_upper,
                &main_chunk_lower,
                &content_chunk,
                app,
                theme,
            );
        }
        Menu::Help => {
            render_help(f, key, theme);
        }
        Menu::NewRelease => {
            render_new_releases(f, &content_sub_chunk, &content_chunk, app, theme);
        }
        Menu::Error => {
            render_error(f, app, key, theme);
        }
        Menu::Player => {
            render_player_in_fullscreen(
                f,
                &player_fullscreen_layout,
                &player_fullscreen_vertical_chunk,
                app,
                theme,
            );
        }
        Menu::AddTrackToPlaylist => {
            render_add_track_to_playlist_screen(f, app, key, theme);
        }
    }
}
