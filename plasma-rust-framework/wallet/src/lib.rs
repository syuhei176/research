// Wallet
#[derive(Clone)]
pub struct Wallet {
  pub number: u32
}
impl Wallet {
  pub fn hello(&self) -> u32 {
    return self.number
  }
}

