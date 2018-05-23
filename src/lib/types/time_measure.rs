use time;

#[allow(dead_code)]
pub struct TimeMeasure {
    name: String,
    start: f64,
}
impl TimeMeasure {
    #[allow(dead_code)]
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            start: TimeMeasure::milliseconds(time::get_time()),
        }
    }
    #[allow(dead_code)]
    fn milliseconds(timespec: time::Timespec) -> f64{
        1000.0 * timespec.sec as f64 + (timespec.nsec as f64 / 1000.0 / 1000.0)
    }
}
impl Drop for TimeMeasure {
    fn drop(&mut self) {
        println!("{} done in {:>6.3} milliseconds.",
                 self.name,
                 TimeMeasure::milliseconds(time::get_time()) - self.start);
    }
}