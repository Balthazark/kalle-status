use crate::baritem::{
    audio::Audio, backlight::Backlight, charge::Bat, network::Network, time::Clock,
};
use baritem::BarItem;
use std::process::Command;
mod baritem;

fn main() {
    let mut bat = Bat::new(0);
    let mut clock = Clock::new();
    let mut backlight = Backlight::new("intel_backlight");
    let mut audio = Audio::new();
    let mut network = Network::new("wlp60s0");

    let mut baritems: Vec<&mut dyn BarItem> = vec![
        &mut clock,
        &mut network,
        &mut audio,
        &mut bat,
        &mut backlight,
    ];

    let mut bar_text = String::new();
    loop {
        bar_text.clear();
        bar_text.push_str("                    ^c#D8DEE9^");
        for item in baritems.iter_mut() {
            bar_text.push_str(&item.get_bar_text());
        }
        let _ = Command::new("xsetroot").args(["-name", &bar_text]).output();
    }
}
