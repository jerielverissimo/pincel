use crate::domain::entities::color::*;

use gio::prelude::*;
use gtk::prelude::*;

use cairo::Context;

type String = &'static str;

pub const STYLE: &[u8] = include_bytes!("gtk.css");

use std::collections::HashMap;

pub fn init_gtk_window() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(move |app| {
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        provider.load_from_data(STYLE).expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        build_ui(app);
    });
    application.run(&[]);
}

pub fn new_button(
    mut buttons: HashMap<String, gtk::Button>,
    name: String,
) -> std::collections::HashMap<String, gtk::Button> {
    let button = gtk::Button::with_label("");
    button.set_size_request(48, 48);
    let btn_context = button.get_style_context();
    btn_context.add_class("circular");
    btn_context.add_class(name);

    let image = gtk::Image::new();
    image.set_from_gicon(
        &gio::Icon::new_for_string("object-select-symbolic").unwrap(),
        gtk::IconSize::Button,
    );

    button.set_image(Some(&image));
    button.show_all();

    button.connect_clicked(move |_| {
        let mut brush_color = CurrentColorSingleton::new();
        brush_color.set(name);
    });

    buttons.insert(name, button);

    buttons
}

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    set_visual(&window, None);

    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_default_size(350, 70);
    window.set_type_hint(gdk::WindowTypeHint::Toolbar);
    window.move_(1350 * 2, 1768 * 2);

    let window_context = window.get_style_context();
    window_context.add_class("window");

    window.set_app_paintable(true); // crucial for transparency

    let grid = gtk::Grid::new();
    grid.set_orientation(gtk::Orientation::Horizontal);
    grid.set_column_spacing(12);
    grid.set_margin_top(12);
    grid.set_margin_bottom(12);
    //grid.set_margin_end(6);
    grid.set_margin_start(6);

    let mut buttons = HashMap::new();
    buttons = new_button(buttons, "red");
    buttons = new_button(buttons, "blue");
    buttons = new_button(buttons, "yellow");
    buttons = new_button(buttons, "green");
    buttons = new_button(buttons, "orange");

    grid.add::<gtk::Button>(buttons.get("red").unwrap().as_ref());
    grid.add::<gtk::Button>(buttons.get("blue").unwrap().as_ref());
    grid.add::<gtk::Button>(buttons.get("yellow").unwrap().as_ref());
    grid.add::<gtk::Button>(buttons.get("green").unwrap().as_ref());
    grid.add::<gtk::Button>(buttons.get("orange").unwrap().as_ref());

    window.add(&grid);

    window.show_all();
    window.set_keep_above(true);
}

fn set_visual(window: &gtk::ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(ref visual) = screen.get_rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

fn draw(_window: &gtk::ApplicationWindow, ctx: &Context) -> Inhibit {
    // crucial for transparency
    ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint();
    Inhibit(false)
}
