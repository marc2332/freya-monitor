#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_std::utils::rw::use_rw;
use freya::prelude::*;
use serde::Deserialize;
use std::{thread::sleep, time::Duration};
use wmi::{COMLibrary, WMIConnection};

fn main() {
    launch_with_props(app, "Monitor", (400.0, 300.0));
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct SensorResult {
    Value: f32,
}

fn app(cx: Scope) -> Element {
    let temp = use_rw(cx, || None);

    cx.use_hook(|| {
       to_owned![temp];
        tokio::task::spawn_blocking( move || {
            let com_con = COMLibrary::new().unwrap();
            let wmi_con = WMIConnection::with_namespace_path("root/OpenHardwareMonitor", com_con).unwrap();
            loop {
                let mut results: Vec<SensorResult> = wmi_con.raw_query("SELECT Name, Value FROM Sensor where Name = 'CPU Package' and SensorType = 'Temperature'").unwrap();

                temp.write(results.pop()).unwrap();
                sleep(Duration::from_millis(800));
            }
        });
    });

    let temp = match temp.read().as_deref() {
        Ok(Some(temp)) => format!("{} °C", temp.Value.round()),
        Ok(None) => "...".to_owned(),
        _ => "err".to_owned(),
    };

    render!(
        rect {
            width: "100%",
            height: "100%",
            background: "black",
            direction: "vertical",
            main_align: "center",
            cross_align: "center",
            label {
                font_family: "Jetbrains Mono",
                width: "100%",
                font_size: "50",
                text_align: "center",
                color: "rgb(244, 206, 20)",
                "{temp}"
            }
        }
    )
}
