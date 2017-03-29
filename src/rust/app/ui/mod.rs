mod login_screen;
mod room_view;

use std::default::Default;

use conrod::{self, color, Colorable, Labelable, Positionable, Sizeable, Widget, Borderable};
use conrod::widget::*;

use super::AppCell;
pub use self::login_screen::LoginScreenState;
pub use self::room_view::RoomViewState;

const HEADER_HEIGHT: conrod::Scalar = 30.0;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum MenuState {
    Open,
    Closed,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct PanelStates {
    left: MenuState,
}

impl Default for MenuState {
    fn default() -> Self { MenuState::Closed }
}

#[derive(Debug)]
pub enum GraphicsState {
    LoginScreen(LoginScreenState),
    RoomView(RoomViewState),
    Exit,
}

impl GraphicsState {
    pub fn login_screen() -> Self { GraphicsState::LoginScreen(LoginScreenState::default()) }
}

pub fn create_ui(app: &mut AppCell, state: &mut GraphicsState) {
    let mut update = None;

    match *state {
        GraphicsState::LoginScreen(ref mut inner) => login_screen::create_ui(app, inner, &mut update),
        GraphicsState::RoomView(ref mut inner) => room_view::create_ui(app, inner, &mut update),
        GraphicsState::Exit => panic!("Should have exited."),
    }

    if let Some(inner) = update {
        *state = inner;
    }
}

fn left_panel_available(ui: &mut conrod::UiCell,
                        ids: &Ids,
                        state: &mut PanelStates,
                        update: &mut Option<GraphicsState>) {
    let left_toggle_clicks = Button::new()
        // style
        .color(color::DARK_CHARCOAL)
        .border(0.0)
        .w_h(100.0, HEADER_HEIGHT)
        // label
        .label("Screeps")
        .small_font(&ui)
        .left_justify_label()
        .label_color(color::WHITE)
        // place
        .parent(ids.header)
        .top_left_of(ids.header)
        .set(ids.left_panel_toggle, ui)
        // now TimesClicked(u16)
        .0;

    // left panel
    match state.left {
        MenuState::Open => {
            left_panel_panel_open(ui, ids, update);

            if left_toggle_clicks % 2 == 1 ||
               left_toggle_clicks == 0 && ui.global_input().current.mouse.buttons.pressed().next().is_some() &&
               ui.global_input()
                .current
                .widget_capturing_mouse
                .or_else(|| ui.global_input().current.widget_under_mouse)
                .map(|capturing| {
                    capturing != ids.left_panel_toggle &&
                    !ui.widget_graph().does_recursive_edge_exist(ids.left_panel_canvas, capturing, |_| true) &&
                    !ui.widget_graph().does_recursive_edge_exist(ids.left_panel_toggle, capturing, |_| true)
                })
                .unwrap_or(true) {

                state.left = MenuState::Closed;
            }
        }
        MenuState::Closed => {
            if left_toggle_clicks % 2 == 1 {
                state.left = MenuState::Open;
            }
        }
    }
}

fn left_panel_panel_open(ui: &mut conrod::UiCell, ids: &Ids, _update: &mut Option<GraphicsState>) {
    Canvas::new()
        // style
        .color(color::DARK_CHARCOAL)
        .border(0.0)
        .w_h(300.0, ui.window_dim()[1] - HEADER_HEIGHT)
        // behavior
        .scroll_kids_vertically()
        // place
        .floating(true)
        .mid_left_of(ids.root)
        .down_from(ids.left_panel_toggle, 0.0)
        .set(ids.left_panel_canvas, ui);
}

fn frame(ui: &mut conrod::UiCell, ids: &Ids, body: Canvas) {
    let header = Canvas::new()
        .color(color::DARK_CHARCOAL)
        .border(0.0)
        .length(HEADER_HEIGHT);

    Canvas::new()
        .border(0.0)
        .flow_down(&[(ids.header, header), (ids.body, body)])
        .set(ids.root, ui);
}

widget_ids! {
    pub struct Ids {
        // Root IDs
        root,
        header,
        body,

        // Main screen
        left_panel_toggle,
        left_panel_canvas,

        username_gcl_header,

        // Login screen
        login_canvas,
        login_header_canvas,

        login_username_canvas,
        login_username_textbox,
        login_username_label,

        login_password_canvas,
        login_password_textbox,
        login_password_label,

        login_submit_canvas,
        login_exit_button,
        login_submit_button,
    }
}
