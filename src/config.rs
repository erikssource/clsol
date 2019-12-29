// Under development.

pub struct Config {
  refresh_stock: i8,
  draw: u8,
}

impl Config {
  pub fn get_refresh_stock(&self) -> Option<u8> {
    if self.refresh_stock >= 0 {
      Some(self.refresh_stock as u8)
    }
    else {
      None
    }
  }
}
