#[derive(Debug, PartialEq, Eq)]
pub enum CheckResult {
    Sensor,
    Beacon,
    Cannot,
    Possible,
}
