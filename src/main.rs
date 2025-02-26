use druid::{
    widget::prelude::*, widget::{TextBox, List, Align, Svg, SvgData, Label, Controller, Flex, 
    SizedBox, LineBreaking, Scroll}, commands::QUIT_APP, im::Vector, 
    FontWeight, FontDescriptor, Cursor, Screen, Point, AppLauncher, Color, 
    Data, Lens, Widget, WidgetExt, WindowDesc
};
use std::{str::FromStr, fs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Lens, Data, Deserialize, Serialize, Debug)]
struct UpdateDetails {
    update_key: String,
    update_value: String,
    additional_info: String,
}

#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
    updates: Vector<UpdateDetails>,
    drag_offset: Point,
    is_dragging: bool,
}

struct ClosureHandler;
impl<W: Widget<AppState>> Controller<AppState, W> for ClosureHandler {
	fn event( &mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env ) {
		match event {
			Event::WindowCloseRequested => {
				ctx.submit_command(QUIT_APP);
			}
			_ => {}
		}

		child.event(ctx, event, data, env);
	}
} 

struct DragController;
impl<W: Widget<AppState>> Controller<AppState, W> for DragController {
    fn event( &mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::MouseDown(mouse) => {
                data.is_dragging = true;
                data.drag_offset = mouse.pos;
                ctx.set_active(true);
            }
            Event::MouseMove(mouse) if data.is_dragging => {
                let delta = mouse.pos - data.drag_offset;
                let new_pos = ctx.window().get_position() + delta;
                ctx.window().set_position(new_pos);
            }
            Event::MouseUp(_) => {
                data.is_dragging = false;
                ctx.set_active(false);
            }
            _ => {}
        }

        child.event(ctx, event, data, env);
    }
}

struct Hover;
impl<W: Widget<AppState>> Controller<AppState, W> for Hover {
    fn event( &mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env ) {
        match event {
            Event::MouseMove(_) => { ctx.set_cursor(&Cursor::Pointer); }
            _ => {}
        }

        child.event(ctx, event, data, env);
    }
}

const GRAY_COLOR: &str = "3C3C3C";
const BLACK_COLOR: &str = "282828";
const RED_COLOR: &str = "B03B3B";

fn create_draggable_panel() -> impl Widget<AppState> {
	Flex::row()
		.fix_size(467., 20.)
		.background(Color::from_hex_str(BLACK_COLOR).unwrap())
		.controller(DragController)
}

fn build_news_panel() -> impl Widget<AppState> {
    let news = List::new(|| {
        Flex::column()
            .with_child(
                Label::new(|item: &UpdateDetails, env: &Env| item.update_key.clone())
                    .with_line_break_mode(LineBreaking::WordWrap)
                    .with_text_color(Color::from_hex_str(RED_COLOR).unwrap())
                    .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD).with_size(46.))
                    .padding((0., 0., 0., 5.))
            )
            .with_child(
                Flex::column().with_child(Label::new(|item: &UpdateDetails, env: &Env| item.update_value.clone()).with_line_break_mode(LineBreaking::WordWrap).with_text_size(14.).padding(10.).background(Color::from_hex_str(BLACK_COLOR).unwrap()).padding((0., 0., 0., 15.)))
                .fix_width(380.)
                
            )
            .with_child(
                Flex::column().with_child(Label::new(|item: &UpdateDetails, env: &Env| item.additional_info.clone()).with_line_break_mode(LineBreaking::WordWrap).with_text_size(14.).align_left().padding((0., 0., 0., 30.)))
                .fix_width(380.)
            )
            .fix_width(430.)
            .background(Color::from_hex_str(GRAY_COLOR).unwrap())
            .padding((0., 10., 0., 10.))
    })
    .lens(AppState::updates);

    Scroll::new(Align::centered(news))
        .vertical().fix_size(455., 430.)
        .background(Color::from_hex_str(BLACK_COLOR).unwrap())
}

fn build_settings_panel() -> impl Widget<AppState> {
	Flex::row()
		.with_child(create_account_input().padding((6.0, 0.0, 0.0, 0.0)))
		.with_child(create_refresh_button().padding((87.0, 0.0, 0.0, 0.0)))
		.with_child(create_settings_button().padding((8.0, 0.0, 0.0, 0.0)))
		.fix_size(455., 40.)
		.background(Color::from_hex_str(BLACK_COLOR).unwrap())
}

fn create_account_input() -> impl Widget<AppState> {
	Flex::row()
        .fix_size(300.0, 26.0)
        .background(Color::from_hex_str(RED_COLOR).unwrap())
}

fn create_refresh_button() -> impl Widget<AppState> {
	Svg::new(SvgData::from_str(include_str!("../images/refresh.svg")).unwrap())
		.fix_width(24.)
		.on_click(move |ctx, data: &mut AppState, env| {
			data.updates.clear();
			data.updates = fetch_news::get_news_vector();
		})
		.controller(Hover)
}

fn create_settings_button() -> impl Widget<AppState> {
	Svg::new(SvgData::from_str(include_str!("../images/settings.svg")).unwrap())
		.fix_width(24.)
		.on_click(move | ctx, data: &mut AppState, env | {
			// Create a new window description for the settings window
            let new_window = WindowDesc::new(build_settings_ui())
                .title("Настройки")
                .resizable(false)
                .window_size(Size::new(455.0, 530.0))
                .set_position(ctx.window().get_position());

            // Open the new window
            let winid = ctx.new_window(new_window);
		})
		.controller(Hover)
}

fn build_run_online_panel() -> impl Widget<AppState> {
	Flex::row()
		.with_child(create_online_status())
		.with_child(create_run_button().padding((11.0, 0.0, 0.0, 0.0)))
}

fn create_online_status() -> impl Widget<AppState> {
	Flex::column()
        .fix_size(284.0, 50.0)
        .background(Color::from_hex_str(BLACK_COLOR).unwrap())
}

mod mc_command; // launch minecraft
fn create_run_button() -> impl Widget<AppState> {
	SizedBox::new(
		Align::centered(
			Label::new("ПОДКЛЮЧИТСЯ")
				.with_text_color(Color::from_hex_str(GRAY_COLOR).unwrap())
                .with_font(FontDescriptor::default().with_weight(FontWeight::BOLD).with_size(16.0))
		)
	)
	.fix_size(160., 50.)
	.background(Color::from_hex_str(RED_COLOR).unwrap())
    .on_click(move |ctx, data, env| {
        mc_command::launch_command();
    })
	.controller(Hover)
}

fn build_settings_ui() -> impl Widget<AppState> {
    TextBox::new()
        .with_placeholder("placeholder text")
        .lens(AppState::name)
}

fn build_ui() -> impl Widget<AppState> {
    Flex::column()
    	.with_child(create_draggable_panel())
    	.with_child(build_news_panel().padding((0., 12., 0., 12.)))
    	.with_child(build_settings_panel().padding((0., 0., 0., 4.)))
    	.with_child(build_run_online_panel())
    	.background(Color::from_hex_str(GRAY_COLOR).unwrap())
        .controller(ClosureHandler)
}

mod fetch_news;
fn main() {
    let monitors = Screen::get_display_rect();
    let window_size = Size::new(467., 578.);

    let window = WindowDesc::new(build_ui())
        .title("главное меню")
        .window_size(window_size)
        .resizable(false)
        .show_titlebar(false)
        .set_position(Point::new((monitors.x1 - window_size.width) / 2.0, (monitors.y1 - window_size.height - 100.0) / 2.0));

    let initial_state = AppState {
        name: "".to_string(),
        updates:fetch_news::get_news_vector(),
        drag_offset: Point::ZERO,
        is_dragging: false,
    };

    AppLauncher::with_window(window)
        .launch(initial_state).unwrap();
}

// connect news // connected [ need to handle no internet ]
// connect advenced runtime installation [ better to make it in updater ]
// connect launch
// make advanced settings
// fix updater and that it project done