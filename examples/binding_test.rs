use cstr_core::{cstr, CStr, CString};
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use lvgl::font::Font;
use lvgl::input_device::{
    pointer::{Pointer, PointerInputData},
    InputDriver,
};
use lvgl::misc::anim::{AnimRepeatCount, Animation};
use lvgl::misc::area::LV_SIZE_CONTENT;
use lvgl::style::{Opacity, Style};
use lvgl::widgets::{Btn, Btnmatrix, Canvas, Chart, Dropdown, Label};
use lvgl::{self, NativeObject, Obj};
use lvgl::{Align, Color, Display, DrawBuffer, LvError, Part, Widget};
use lvgl_sys::{
    lv_anim_path_ease_out, lv_chart_add_series, lv_chart_type_t, lv_coord_t, lv_flex_flow_t_LV_FLEX_FLOW_COLUMN, lv_grid_align_t_LV_GRID_ALIGN_CENTER, lv_grid_align_t_LV_GRID_ALIGN_START, lv_grid_align_t_LV_GRID_ALIGN_STRETCH, lv_label_set_text, lv_obj_set_grid_cell, lv_obj_set_style_opa, lv_obj_set_width, lv_opa_t, lv_palette_t_LV_PALETTE_AMBER, lv_palette_t_LV_PALETTE_BLUE, lv_palette_t_LV_PALETTE_BLUE_GREY, lv_palette_t_LV_PALETTE_BROWN, lv_palette_t_LV_PALETTE_DEEP_ORANGE, lv_palette_t_LV_PALETTE_DEEP_PURPLE, lv_palette_t_LV_PALETTE_GREY, lv_palette_t_LV_PALETTE_PURPLE, lv_palette_t_LV_PALETTE_RED, lv_palette_t_LV_PALETTE_TEAL, LV_CHART_AXIS_PRIMARY_X, LV_CHART_TYPE_BAR, LV_GRID_CONTENT, LV_GRID_TEMPLATE_LAST, LV_OBJ_FLAG_HIDDEN, LV_OPA_50, LV_OPA_70, LV_OPA_COVER, LV_PART_MAIN
};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

// #define LV_GRID_FR(x)          (LV_COORD_MAX - 100 + x)
macro_rules! lv_grid_fr {
    ($x:literal) => {
        lvgl_sys::LV_COORD_MAX - 100 + $x
    };
}

// #define _LV_COORD_TYPE_SHIFT    (13U)
// #define _LV_COORD_TYPE_SPEC     (1 << _LV_COORD_TYPE_SHIFT)
// #define LV_COORD_SET_SPEC(x)   ((x) | _LV_COORD_TYPE_SPEC)
macro_rules! lv_coord_set_spec {
    ($x: expr) => {
        ($x) | (1 << 13u32)
    };
}

// #define LV_PCT(x)              (x < 0 ? LV_COORD_SET_SPEC(1000 - (x)) : LV_COORD_SET_SPEC(x))
macro_rules! lv_pct {
    ($x: literal) => {
        if $x < 0 {
            lv_coord_set_spec!(1000 - ($x))
        } else {
            lv_coord_set_spec!($x)
        }
    };
}

macro_rules! lv_canvas_buf_size_indexed_2bit {
    ($w: literal, $h:literal) => {
        ((($w / 4) + 1) * $h)
    };
}

#[allow(unused_assignments)]
fn main() -> Result<(), LvError> {
    const HOR_RES: u32 = 800;
    const VER_RES: u32 = 480;

    let mut sim_display: SimulatorDisplay<Rgb565> =
        SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Test UI", &output_settings);

    let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

    let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
        sim_display.draw_iter(refresh.as_pixels()).unwrap();
    })?;

    // Define the initial state of your input
    let mut latest_touch_status = PointerInputData::Touch(Point::new(0, 0)).released().once();

    // Register a new input device that's capable of reading the current state of the input
    let _touch_screen = Pointer::register(|| latest_touch_status, &display)?;

    let c1;
    let c2;
    let c3;
    unsafe {
        c1 = lvgl_sys::lv_palette_main(lv_palette_t_LV_PALETTE_DEEP_ORANGE);
        c2 = lvgl_sys::lv_palette_darken(lv_palette_t_LV_PALETTE_BLUE, 2);
        c3 = lvgl_sys::lv_palette_main(lv_palette_t_LV_PALETTE_RED);
    }

    let mut style_big_font = Style::default();
    //style_big_font.set_text_font(Font::new_raw(lvgl_sys::lv_font_montserrat_24));

    let mut screen = display.get_scr_act()?;

    let grid_cols = [
        300,
        lv_grid_fr!(3) as i16,
        lv_grid_fr!(2) as i16,
        LV_GRID_TEMPLATE_LAST as i16,
    ];
    let grid_rows = [
        100,
        lv_grid_fr!(1) as i16,
        LV_GRID_CONTENT as i16,
        LV_GRID_TEMPLATE_LAST as i16,
    ];
    unsafe {
        lvgl_sys::lv_obj_set_grid_dsc_array(
            screen.raw().as_ptr(),
            grid_cols.as_ptr(),
            grid_rows.as_ptr(),
        );
    }

    //let chart_type_subject = Subject::new()?;
    //lv_subject_init_int(&chart_type_subject, 0);

    let mut dropdown = Dropdown::create(&mut screen)?;
    dropdown.set_options(cstr_core::cstr!("Lines\nBars"));
    unsafe {
        lvgl_sys::lv_obj_set_grid_cell(
            dropdown.raw().as_ptr(),
            lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            0,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            0,
            1,
        );
        //dropdown.bind_value(&mut chart_type_subject);
        dropdown.set_selected(1);
    }

    /*Create a chart with an external array of points*/
    unsafe {
        let mut chart = Chart::create(&mut screen)?;
        lvgl_sys::lv_obj_set_grid_cell(
            chart.raw().as_ptr(),
            lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            0,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            1,
            1,
        );

        let series =
            lvgl_sys::lv_chart_add_series(chart.raw().as_ptr(), c3, LV_CHART_AXIS_PRIMARY_X as u8);

        let mut chart_y_array = [10, 25, 50, 40, 30, 35, 60, 65, 70, 75];
        chart.set_ext_y_array(series.as_mut().unwrap(), &mut chart_y_array[0]);
        chart.set_type(LV_CHART_TYPE_BAR as lv_chart_type_t);
    }

    /*Add custom observer callback*/
    //lv_subject_add_observer_obj(&chart_type_subject, chart_type_observer_cb, chart, NULL);

    /*Manually set the subject's value*/
    //lv_subject_set_int(&chart_type_subject, 1);


    let mut label = Label::create(&mut screen)?;
    unsafe {
        lvgl_sys::lv_obj_set_grid_cell(
            label.raw().as_ptr(),
            lv_grid_align_t_LV_GRID_ALIGN_START,
            1,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_CENTER,
            0,
            1,
        );
    }

    let mut label_style = Style::default();
    label_style.set_bg_opa(Opacity::OPA_70);
    label_style.set_bg_color(Color::from_raw(c1));
    label_style.set_text_color(Color::from_raw(c2));
    label.add_style(Part::Main, &mut label_style);
    label.add_style(Part::Main, &mut style_big_font);

    let mut btnmatrix_options = [
        cstr!("First").as_ptr(),
        cstr!("Second").as_ptr(),
        cstr!("\n").as_ptr(),
        cstr!("Third").as_ptr(),
        cstr!("").as_ptr(),
    ];

    let btnmatrix_ctrl = [
        lvgl_sys::LV_BTNMATRIX_CTRL_DISABLED as u16,
        2 | lvgl_sys::LV_BTNMATRIX_CTRL_CHECKED as u16,
        1,
    ];

    let mut btnmatrix = Btnmatrix::create(&mut screen)?;
    unsafe {
        lvgl_sys::lv_obj_set_grid_cell(
            btnmatrix.raw().as_ptr(),
            lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            1,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            1,
            1,
        );
        lvgl_sys::lv_btnmatrix_set_map(btnmatrix.raw().as_ptr(), btnmatrix_options.as_mut_ptr());
        lvgl_sys::lv_btnmatrix_set_ctrl_map(btnmatrix.raw().as_ptr(), btnmatrix_ctrl.as_ptr());
    }

    let mut cont = Obj::create(&mut screen)?;
    unsafe {
        lvgl_sys::lv_obj_set_grid_cell(
            cont.raw().as_ptr(),
            lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            2,
            1,
            lv_grid_align_t_LV_GRID_ALIGN_STRETCH,
            0,
            2,
        );
        lvgl_sys::lv_obj_set_flex_flow(cont.raw().as_ptr(), lv_flex_flow_t_LV_FLEX_FLOW_COLUMN);
    }

    let mut btns: Vec<Btn> = Vec::with_capacity(10);
    let mut labels: Vec<Label> = Vec::new();
    let mut animations: Vec<Animation> = Vec::new();
    for i in 0..10u32 {
        let (mut btn, mut label) = list_button_create(&mut cont)?;

        if i == 0 {
            /*let mut a = Animation::new(
                &mut btn,
                Duration::from_secs(1),
                LV_OPA_COVER as i32,
                LV_OPA_50 as i32,
                |obj, val| unsafe {
                    lv_obj_set_style_opa(obj.raw().as_ptr(), val as lv_opa_t, LV_PART_MAIN);
                },
            )?;
            a.start();
            animations.push(a);*/
        }

        if i == 1 {
            unsafe {
                lvgl_sys::lv_obj_add_flag(btn.raw().as_ptr(), LV_OBJ_FLAG_HIDDEN);
            }
        }

        if i == 2 {
            // unsafe {
            //     let mut label = lvgl_sys::lv_obj_get_child(btn.raw().as_ptr(), 0);
            //     lv_label_set_text(label, cstr!("A multi-line text with a ° symbol").as_ptr());
            //     lv_obj_set_width(label, lv_pct!(100));
            // }
            label.set_text(cstr!("A multi-line text with a ° symbol"));
            label.set_width(lv_pct!(100));
        }

        if i == 4 {
            /*let mut a = Animation::new(
                &mut btn,
                Duration::from_millis(300),
                LV_OPA_COVER as i32,
                LV_OPA_50 as i32,
                |obj, val| unsafe {
                    //lv_obj_set_style_opa(obj.raw().as_ptr(), val as u8, 0);
                },
            )?;
            a.set_repeat_count(AnimRepeatCount::Infinite);
            a.start();*/
        }

        btns.push(btn);
        labels.push(label);
    }

    //let canvas_buf = [0u8; lv_canvas_buf_size_indexed_2bit!(400, 100)];

    let mut canvas = Canvas::create(&mut screen)?;
    canvas.set_size(400, 100);
    unsafe {
        let canvas = canvas.raw().as_ptr();
        lvgl_sys::lv_obj_set_grid_cell(
            canvas,
            lv_grid_align_t_LV_GRID_ALIGN_START,
            0,
            2,
            lv_grid_align_t_LV_GRID_ALIGN_START,
            2,
            1,
        );
        /*lvgl_sys::lv_canvas_set_buffer(
            canvas,
            lvgl_sys::lv_draw_buf_align(canvas_buf, LV_COLOR_FORMAT_RGB565),
            400,
            100,
            LV_COLOR_FORMAT_RGB565,
        );
        lvgl_sys::lv_canvas_fill_bg(canvas, c2, LV_OPA_COVER);
        lvgl_sys::draw_to_canvas(canvas);*/
    }

    let mut is_mouse_down = false;
    'running: loop {
        let start = Instant::now();
        lvgl::task_handler();
        window.update(&sim_display);

        let events = window.events().peekable();

        for event in events {
            match event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => {
                    is_mouse_down = true;
                    latest_touch_status = PointerInputData::Touch(point).pressed().once();
                }
                SimulatorEvent::MouseMove { point } => {
                    if is_mouse_down {
                        latest_touch_status = PointerInputData::Touch(point).pressed().once();
                    }
                }
                SimulatorEvent::MouseButtonUp {
                    mouse_btn: _,
                    point,
                } => {
                    is_mouse_down = false;
                    latest_touch_status = PointerInputData::Touch(point).released().once();
                }
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        sleep(Duration::from_millis(5));
        lvgl::tick_inc(Instant::now().duration_since(start));
    }

    Ok(())
}

fn list_button_create<'a>(parent: &mut impl NativeObject) -> Result<(Btn<'a>, Label<'a>), LvError> {
    //fn list_button_create<'a>(parent: &mut impl NativeObject) -> Result<Btn<'a>, LvError> {
    let mut btn = Btn::create(parent)?;
    btn.set_size(lv_pct!(100), LV_SIZE_CONTENT as i16);
    let idx;
    unsafe {
        idx = lvgl_sys::lv_obj_get_index(btn.raw().as_ptr());
    }
    let mut label = Label::create(&mut btn)?;
    label.set_text(CString::new(format!("Item {idx}")).unwrap().as_c_str());

    Ok((btn, label))
    //Ok(btn)
}
