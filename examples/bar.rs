use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl;
use lvgl::style::Style;
use lvgl::widgets::{Bar, Label, Widget};
use lvgl::{Align, AnimationState, Color, Display, DrawBuffer, Event, LvError, Part};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 240;
    const VER_RES: u32 = 240;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Bar Example", &output_settings);

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        sim_display.draw_iter(refresh.as_pixels()).unwrap();
    })?;

    let mut screen = display.get_scr_act()?;

    let mut screen_style = Style::default();
    screen_style.set_bg_color(Color::from_rgb((255, 255, 255)));
    screen_style.set_radius(0);
    screen.add_style(screen_style.into_raw(), Part::Main.into());

    // Create the bar object
    let mut bar = Bar::create(&mut screen)?;
    bar.set_size(175, 20);
    bar.align(Align::Center.into(), 0, 10);
    bar.set_range(0, 100);
    bar.on_event(|_b, _e| {
        println!("Completed!");
    })?;

    // Set the indicator style for the bar object
    let mut ind_style = Style::default();
    ind_style.set_bg_color(Color::from_rgb((100, 245, 100)));
    bar.add_style(ind_style.into_raw(), Part::Any.into());

    let mut loading_lbl = Label::create(&mut screen)?;
    loading_lbl.set_text(CString::new("Loading...").unwrap().as_c_str());
    loading_lbl.align(Align::OutTopMid.into(), 0, 0);

    let mut loading_style = Style::default();
    loading_style.set_text_color(Color::from_rgb((0, 0, 0)));
    loading_lbl.add_style(loading_style.into_raw(), Part::Main.into());

    let mut i = 0;
    'running: loop {
        let start = Instant::now();
        if i > 100 {
            i = 0;
            // Enabling this line gives the followong errors:
            // - error[E0597]: `ind_style` does not live long enough when adding an style to the bar
            // - implementation of `Widget` is not general enough
            // lvgl::event_send(&mut bar, Event::Clicked);
        }
        bar.set_value(i, AnimationState::ON.into());
        i += 1;

        lvgl::task_handler();
        window.update(&sim_display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        sleep(Duration::from_millis(15));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }

    Ok(())
}
