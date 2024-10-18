mod fs030w;

pub use fs030w::FS030W;

#[derive(Debug)]
pub struct BatteryInfo {
    pub is_charging: bool,
    pub percentage: i8,
}
