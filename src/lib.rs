pub mod cli;
pub mod collectors;
pub mod display;

pub use collectors::{
    BatteryInfo, DisksInfo, HostsInfo, NetworkInfo, ProcessInfo, SystemInfo, TemperatureInfo,
};
pub use display::{display_info, OutputFormat};
