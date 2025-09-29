#[derive(Debug)]
pub struct Account {
  pub id: u32,
  pub balance: i32,
  pub holder: String,

}

impl Account {
  pub fn new(id: u32, holder: String) -> Self {
    return Account { id, holder, balance: 0 };
  }

  pub fn summary(&self) -> String {
    return format!("{} has a balance of ${}", self.holder, self.balance)

  }

  pub fn deposit(&mut self, amount: i32) -> i32 {
    self.balance += amount;
    return self.balance;
  }

  pub fn withdraw(&mut self, amount: i32) -> i32 {
    self.balance -= amount;
    return self.balance;
  }
}
