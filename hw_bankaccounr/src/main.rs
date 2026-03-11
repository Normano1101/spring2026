#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
            BankAccount {
                balance: initial_balance,
            }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance

    }

    pub fn apply_interest(&mut self, rate: f64) {
        
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let account = BankAccount::new(100.0);
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert_eq!(account.balance(), 150.0);

    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut account = BankAccount::new(100.0);
        account.withdraw(30.0);
        assert_eq!(account.balance(), 70.0);
    }

    // Add more tests here
    #[test]
    fn test_over_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(200.0);

        // balance should stay the same
        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_negative_deposit() {
        let mut account = BankAccount::new(100.0);
        account.deposit(-50.0);

        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_negative_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(-20.0);

        assert_eq!(account.balance(), 100.0);
    }

    #[test]
    fn test_apply_interest() {
        let mut account = BankAccount::new(100.0);
        account.apply_interest(0.10);

        assert_eq!(account.balance(), 110.0);
    }
}


fn main() {
    let mut account = BankAccount::new(100.0);

    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("After deposit: {}", account.balance());

    account.withdraw(30.0);
    println!("After withdraw: {}", account.balance());
}