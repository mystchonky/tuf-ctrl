extern crate gtk;
extern crate libappindicator;

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};

use cmd_lib::run_cmd;

enum State {
    Normal,
    Turbo,
    Silent,
}

static FAN_CTL_PATH: &str = "/sys/devices/platform/faustus/throttle_thermal_policy";

fn main() {
    gtk::init().unwrap();
    let mut indicator = AppIndicator::new("libappindicator test application", "");
    indicator.set_status(AppIndicatorStatus::Active);
    let mut menu = gtk::Menu::new();

    let item_normal_mode = gtk::MenuItem::with_label("Normal");
    item_normal_mode.connect_activate(|_| {
        change_state(State::Normal);
    });
    menu.append(&item_normal_mode);

    let item_turbo_mode = gtk::MenuItem::with_label("Turbo");
    item_turbo_mode.connect_activate(|_| {
        change_state(State::Turbo);
    });
    menu.append(&item_turbo_mode);

    let item_silent_mode = gtk::MenuItem::with_label("Silent");
    item_silent_mode.connect_activate(|_| {
        change_state(State::Silent);
    });
    menu.append(&item_silent_mode);

    menu.append(&gtk::SeparatorMenuItem::new());

    let quit_item = gtk::MenuItem::with_label("Quit");
    quit_item.connect_activate(|_| {
        gtk::main_quit();
    });
    menu.append(&quit_item);

    indicator.set_menu(&mut menu);
    menu.show_all();
    gtk::main();
}

fn change_state(st: State) {
    match st {
        State::Normal => {
            run_cmd!(echo 0 > $FAN_CTL_PATH);
        }
        State::Turbo => {
            run_cmd!(echo 1 > $FAN_CTL_PATH);
        }

        State::Silent => {
            run_cmd!(echo 2 > $FAN_CTL_PATH);
        }
    }
}
