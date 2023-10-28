use dbus::{blocking::Connection, Message};
use ddc_hi::{Ddc, Display};
use std::sync::mpsc::Sender;
use std::{
    error::Error,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

mod brightness;
mod kscreen;

use brightness::OrgKdeSolidPowerManagementActionsBrightnessControl;
use brightness::OrgKdeSolidPowerManagementActionsBrightnessControlBrightnessChanged as BrightnessChanged;
use kscreen::OrgKdeKscreenBackendConfigChanged as ScreenConfigChanged;

struct Monitor {
    display: Display,
    max_brightness: f32,
}

impl Monitor {
    fn new(mut display: Display) -> Result<Monitor, Box<dyn Error>> {
        display.update_capabilities()?;
        let feature = display
            .info
            .mccs_database
            .get(0x10)
            .ok_or("The monitor does not support the brightness feature")?;

        let max_brightness = display.handle.get_vcp_feature(feature.code)?.maximum() as f32;
        Ok(Monitor {
            display,
            max_brightness,
        })
    }

    fn set_brightness(&mut self, brightness: f32) -> Result<(), Box<dyn Error>> {
        let brigthness_to_set = (brightness * self.max_brightness) as u16;
        self.display
            .handle
            .set_vcp_feature(0x10, brigthness_to_set)?;
        Ok(())
    }

    fn name(&self) -> String {
        self.display
            .info
            .model_name
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or("unknown".to_string())
    }
}

#[derive(PartialEq, Eq)]
enum Notif {
    BrightnessChanged,
    Stop,
}

fn setup(brightness: &Arc<Mutex<f32>>) -> Result<Vec<Sender<Notif>>, Box<dyn Error>> {
    let mut senders = vec![];

    for display in Display::enumerate() {
        match Monitor::new(display) {
            Ok(mut monitor) => {
                let brightness = brightness.clone();
                let (sender, receiver) = mpsc::channel();
                senders.push(sender);

                thread::spawn(move || {
                    let mut current_brightness = None;
                    for notif in receiver {
                        if notif == Notif::Stop {
                            return;
                        }
                        let b = *brightness.lock().unwrap();
                        if current_brightness == Some(b) {
                            continue;
                        }
                        match monitor.set_brightness(b) {
                            Err(e) => {
                                println!(
                                    "Error setting brightness on monitor {}: {}",
                                    monitor.name(),
                                    e
                                );
                            }
                            _ => current_brightness = Some(b),
                        }
                    }
                });
            }
            Err(e) => {
                println!("Skipped a monitor: {}", e);
            }
        }
    }

    Ok(senders)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_session()?;
    let brightness_proxy = conn.with_proxy(
        "org.kde.Solid.PowerManagement",
        "/org/kde/Solid/PowerManagement/Actions/BrightnessControl",
        Duration::from_millis(5000),
    );
    let screen_proxy = conn.with_proxy("org.kde.KScreen", "/backend", Duration::from_millis(5000));

    let brightness_max: i32 = brightness_proxy.brightness_max()?;
    println!("Maximum brightness = {brightness_max}");

    let current_brightness: i32 = brightness_proxy.brightness()?;
    let current_brightness = current_brightness as f32 / brightness_max as f32;
    println!("Current brightness = {:.0}%", current_brightness * 100.);

    let brightness = Arc::new(Mutex::new(current_brightness));
    let brightness2 = brightness.clone();
    let senders = Arc::new(Mutex::new(setup(&brightness)?));
    let senders2 = senders.clone();

    brightness_proxy.match_signal(
        move |brightness_changed: BrightnessChanged, _: &Connection, _: &Message| {
            let b = brightness_changed.arg0 as f32 / brightness_max as f32;
            *brightness.lock().unwrap() = b;
            // println!("Brightness changed to {:3>.0}%", b * 100.);

            for sender in &*senders.lock().unwrap() {
                if let Err(e) = sender.send(Notif::BrightnessChanged) {
                    println!("Cannot send brightness change notif: {e}");
                }
            }
            true
        },
    )?;

    // Screen config change listener
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        for _ in &receiver {
            println!("screen configuration changed, stopping previous workers...");
            for sender in &*senders2.lock().unwrap() {
                if let Err(e) = sender.send(Notif::Stop) {
                    println!("failed to send notification: {e}");
                }
            }
            senders2.lock().unwrap().clear();
    
            // wait for the monitor to be up
            loop {
                match receiver.recv_timeout(Duration::from_secs(7)) {
                    Ok(_) => println!("additional change notif received, waiting a bit more..."),
                    Err(_) => break,
                }
            }

            println!("reloading configuration...");
            match setup(&brightness2) {
                Ok(s) => *senders2.lock().unwrap() = s,
                Err(e) => println!("Error while reloading screen config: {e}"),
            }
        }
    });

    screen_proxy.match_signal(move |_: ScreenConfigChanged, _: &Connection, _: &Message| {
        if let Err(e) = sender.send(()) {
            println!("Cannot send config change notif: {e}");
        }
        true
    })?;

    loop {
        conn.process(Duration::from_secs(1000))?;
    }
}
