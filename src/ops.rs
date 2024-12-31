use deno_core::{op2, OpState, error::AnyError, Op};
use std::sync::{Arc, Mutex};
use crate::EguiApp;

#[op2]
#[serde]
pub fn op_set_label(
    state: &mut OpState,
    #[serde] args: crate::SetLabelArgs,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    app.label_text = args.text.clone();
    Ok(())
}

#[op2]
#[serde]
pub fn op_set_slider(
    state: &mut OpState,
    #[serde] args: crate::SetSliderArgs,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    app.slider_value = args.value;
    Ok(())
}

#[op2]
#[serde]
pub fn op_set_input(
    state: &mut OpState,
    #[serde] args: crate::SetInputArgs,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    app.input_text = args.text.clone();
    Ok(())
}

#[op2]
#[serde]
pub fn op_set_checkbox(
    state: &mut OpState,
    #[serde] args: crate::SetCheckboxArgs,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    app.checkboxes.insert(args.id.clone(), args.checked);
    Ok(())
}

#[op2]
#[serde]
pub fn op_set_combo_box(
    state: &mut OpState,
    #[serde] args: crate::SetComboBoxArgs,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    app.combo_boxes.insert(args.id.clone(), (args.selected.clone(), args.options.clone()));
    Ok(())
}

#[op2]
#[serde]
pub fn op_set_radio(
    state: &mut OpState,
    #[serde] args: crate::SetRadioArgs,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    app.radio_groups.insert(args.id.clone(), args.selected.clone());
    Ok(())
}

#[op2]
#[serde]
pub fn op_set_progress(
    state: &mut OpState,
    #[serde] args: crate::SetProgressArgs,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    app.progress_bars.insert(args.id.clone(), args.value);
    Ok(())
}

#[op2]
#[serde]
pub fn op_rotate_3d(
    state: &mut OpState,
    #[serde] args: crate::Rotate3DArgs,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    app.rotation += args.angle;
    Ok(())
}

#[op2]
#[serde]
pub fn op_add_3d_object(
    _state: &mut OpState,
    #[serde] args: serde_json::Value,
) -> Result<(), AnyError> {
    println!("Add 3D object request: {:?}", args);
    Ok(())
}

#[op2]
#[serde]
pub fn op_start_recording(
    state: &mut OpState,
    #[serde] _args: serde_json::Value,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    if !app.is_recording {
        app.is_recording = true;
        app.recorded_events.clear();
        app.recording_start = Some(std::time::Instant::now());
        println!("Recording started.");
    }
    Ok(())
}

#[op2]
#[serde]
pub fn op_stop_recording(
    state: &mut OpState,
    #[serde] _args: serde_json::Value,
) -> Result<Vec<crate::events::RecordedEvent>, AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    if app.is_recording {
        app.is_recording = false;
        app.recording_start = None;
        println!("Recording stopped.");
        Ok(app.recorded_events.clone())
    } else {
        Ok(vec![])
    }
}

#[op2]
#[serde]
pub fn op_start_playback(
    state: &mut OpState,
    #[serde] _args: serde_json::Value,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    if !app.is_playing && !app.recorded_events.is_empty() {
        app.is_playing = true;
        app.playback_index = 0;
        app.playback_start = Some(std::time::Instant::now());
        println!("Playback started.");
    }
    Ok(())
}

#[op2]
#[serde]
pub fn op_stop_playback(
    state: &mut OpState,
    #[serde] _args: serde_json::Value,
) -> Result<(), AnyError> {
    let app = state.borrow::<Arc<Mutex<EguiApp>>>().clone();
    let mut app = app.lock().unwrap();
    if app.is_playing {
        app.is_playing = false;
        app.playback_index = 0;
        app.playback_start = None;
        println!("Playback stopped.");
    }
    Ok(())
}

pub fn init_ops() -> Vec<deno_core::OpDecl> {
    vec![
        op_set_label::DECL,
        op_set_slider::DECL,
        op_set_input::DECL,
        op_set_checkbox::DECL,
        op_set_combo_box::DECL,
        op_set_radio::DECL,
        op_set_progress::DECL,
        op_rotate_3d::DECL,
        op_add_3d_object::DECL,
        op_start_recording::DECL,
        op_stop_recording::DECL,
        op_start_playback::DECL,
        op_stop_playback::DECL,
    ]
}
