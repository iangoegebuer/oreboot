use core::fmt;
use drivers::model::Driver;

pub struct WriteTo<'a> {
    drv: &'a mut Driver,
}

impl<'a> WriteTo<'a> {
    pub fn new(drv: &'a mut Driver) -> Self {
        WriteTo { drv: drv }
    }
}

impl<'a> fmt::Write for WriteTo<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.drv.pwrite(s.as_bytes(), 0) {
            Err(_) => Err(fmt::Error),
            _ => Ok(()),
        }
    }
}
