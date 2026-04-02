// use std::{f32::consts::E, io};
use std::io;

#[derive(Debug)]
//--data structures--//
// States of a transaction
enum TransactionStatus {
    Pending,
    Confirmed,
    Failed(String) // reason for failure
}

#[derive(Debug)]
// Representation of a single transaction
struct Transaction {
    id: u32,
    amount: f64,
    transaction_type: String, // "deposit" OR "withdrawal" OR "transfer" 
    status: TransactionStatus,
}

#[derive(Debug)]
// Representation of a wallet with balance and transaction history
struct Wallet {
    owner: String,
    balance: f64,
    transactions: Vec<Transaction>,
    next_id: u32, // auto-incerement transaction ID
}

//--wallet implementation--//
impl Wallet {
    // Creates a wallet with zero balance
    fn new(owner: String) -> Self {
        Self {
            owner,
            balance: 0.0,
            transactions: Vec::new(),
            next_id: 1,
        }
    }

    // Deposits an amount into the wallet
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("Amount must be greater than zero"));
        }

        self.balance += amount;
        self.transactions.push(Transaction { 
            id: self.next_id, 
            amount, 
            transaction_type: String::from("deposit"), 
            status: TransactionStatus::Confirmed,
        });
        self.next_id += 1;

        Ok(())
    }

    // Withdraws an amount from the wallet
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err(String::from("Amount must be greater than zero"));
        }

        if amount > self.balance {
            self.transactions.push(Transaction { 
                id: self.next_id, 
                amount, 
                transaction_type: String::from("withdrawal"), 
                status: TransactionStatus::Failed(
                    String::from("Insufficient balance")
                ),
            });
            self.next_id += 1;
            return Err(String::from("Insufficient balance"));
        }

        self.balance -= amount;
        self.transactions.push(Transaction { 
            id: self.next_id, 
            amount, 
            transaction_type: String::from("withdrawal"), 
            status: TransactionStatus::Confirmed,
        });
        self.next_id += 1;

        Ok(())
    } 

    // Prints the transaction history
    fn print_history(&self) {
        if self.transactions.is_empty() {
            println!("No transactions yet");
            return;
        }

        println!("\n--- Transaction History for {} ---", self.owner);
        for tx in &self.transactions {
            let status = match &tx.status {
                TransactionStatus::Pending => String::from("PENDING"),
                TransactionStatus::Confirmed => String::from("CONFIRMED"),
                TransactionStatus::Failed(reason) => {
                    format!("FAILED: {}", reason)
                }
            };
            println!(
                "[{}] {} | Amount: {:.2} | Status: {}",
                tx.id, tx.transaction_type, tx.amount, status
            );
        }
        println!("Current balance: {:.2}", self.balance);
    }
}

fn get_amount() -> f64 {
    loop {
        println!("Enter amount:");
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read input");

        match amount.trim().parse::<f64>() {
            Ok(num) => return num, 
            Err(_) => {
                println!("Please input value in number(s).");
            }
        }
    }
}

fn main() {
    let mut wallet = Wallet::new(String::from("fbars"));
    
    println!("=== CLI Wallet ===\n");

    // // Test deposit
    // match wallet.deposit(1000.0) {
    //     Ok(()) => println!("Deposit successful"),
    //     Err(e) => println!("Deposit failed"),
    // }

    // // Test withdrawal 
    // match wallet.withdraw(250.0) {
    //     Ok(()) => println!("Withdrawal successful"),
    //     Err(e) => println!("Withdrawal failed: {}", e),
    // }

    // // Test failed withdrawal
    // match wallet.withdraw(5000.0) {
    //     Ok(()) => println!("Withdrawal successful"),
    //     Err(e) => println!("Withdrawal failed: {}", e),
    // }

    // // Test invalid deposit
    // match wallet.deposit(-50.0) {
    //     Ok(()) => println!("Deposit successful"),
    //     Err(e) => println!("Deposit failed: {}", e),
    // }

    // // Print full history
    // wallet.print_history();

    loop {
        // Prompts user and captures input
        println!("1. Deposit amount");
        println!("2. Withdraw amount");
        println!("3. Print transaction history");
        println!("4. Quit");
        println!("Enter your choice: ");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Input error");

        match user_input.trim() {
            "1" => {
                if let Err(e) = wallet.deposit(get_amount()) {
                    println!("Error: {}", e);
                }
            }
            "2" => {
                if let Err(e) = wallet.withdraw(get_amount()) {
                    println!("Error: {}", e);
                }
            }
            "3" => wallet.print_history(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Not valid"),
        }
    }
}
