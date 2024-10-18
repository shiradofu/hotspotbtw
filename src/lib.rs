mod device;
mod notify;

use crate::device::FS030W;

const PERCENTAGE_THRESHOLD: i8 = 90;

pub async fn run() -> Result<(), ()> {
    notify::init();

    let fs030w = FS030W::new();

    match fs030w.get_battery_info().await {
        Ok(b) if b.is_charging && b.percentage > PERCENTAGE_THRESHOLD => {
            if let Err(err) = notify::battery_percentage(fs030w.name, b.percentage) {
                eprintln!("{err}");
            }
        }
        Err(err) => eprintln!("{err}"),
        _ => (),
    };

    Ok(())
}
