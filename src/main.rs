use druid::widget::{Button, Flex, Label};
use druid::{
    AppLauncher, Data, Lens, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc,
};
winrt::import!(
    dependencies
        os
    modules
        "windows.foundation"
        "windows.system.power"
);
use windows::system::power::*;

fn battery_status() -> std::string::String {
    let battery_status = PowerManager::battery_status().unwrap();
    let result = match battery_status {
        BatteryStatus::Charging => "Charging",
        BatteryStatus::Discharging => "Discharging",
        BatteryStatus::Idle => "Idle",
        BatteryStatus::NotPresent => "NotPresent",
        _ => "Unknown",
    };
    String::from(result)
}

fn energy_saver_status() -> std::string::String {
    let energy_saver_status = PowerManager::energy_saver_status().unwrap();
    let result = match energy_saver_status {
        EnergySaverStatus::On => "On",
        EnergySaverStatus::Off => "Off",
        EnergySaverStatus::Disabled => "Disabled",
        _ => "Unknown",
    };
    String::from(result)
}

fn power_supply_status() -> std::string::String {
    let power_supply_status = PowerManager::power_supply_status().unwrap();
    let result = match power_supply_status {
        PowerSupplyStatus::Adequate => "Adequate",
        PowerSupplyStatus::Inadequate => "Inadequate",
        PowerSupplyStatus::NotPresent => "NotPresent",
        _ => "Unknown",
    };
    String::from(result)
}

#[derive(Clone, Data, Lens)]
struct PowerData {
    battery_status: String,
    energy_saver_status: String,
    power_supply_status: String,
}

fn build_ui() -> impl Widget<PowerData> {
    let battery_label =
        Label::dynamic(|text: &String, _| text.to_string()).lens(PowerData::battery_status);
    let energy_saver_label =
        Label::dynamic(|text: &String, _| text.to_string()).lens(PowerData::energy_saver_status);
    let power_supply_label =
        Label::dynamic(|text: &String, _| text.to_string()).lens(PowerData::power_supply_status);
    Flex::row().with_flex_child(
        Flex::column()
            .with_flex_child(battery_label, 1.0)
            .with_flex_child(energy_saver_label, 1.0)
            .with_flex_child(power_supply_label, 1.0)
            .with_flex_child(
                Button::new("Update").on_click(|_ctx, data: &mut PowerData, _env| {
                    data.battery_status = battery_status();
                    data.energy_saver_status = energy_saver_status();
                    data.power_supply_status = power_supply_status();
                }),
                1.0,
            ),
        1.0,
    )
}

fn main() {
    let data = PowerData {
        battery_status: battery_status(),
        energy_saver_status: energy_saver_status(),
        power_supply_status: power_supply_status(),
    };
    AppLauncher::with_window(
        WindowDesc::new(build_ui)
            .title(LocalizedString::new("app_title").with_placeholder("Druid + WinRT")),
    )
    .use_simple_logger()
    .launch(data)
    .expect("launch failed");
}
