#[macro_use]
extern  crate serde_derive;

use std::{io, string};
use std::process;
use std::io::Write;



mod blockchain;

fn main() {
    //building our blockchain use
    let mut miner_addr = String::new();
    let mut difficulty  = String::new();
    let mut choice = String::new(); //letting user choose what they want to do

    println!("Input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    println!("Difficulty: ");

    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);

    let diff = difficulty.trim().parse::<u32>().expect("We need an integer");
    println!("Generating genesis block");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    //building menu inside a loop and letting user choose

    loop {
        println!("Menu:");
        println!("1. New transaction");
        println!("2. Mine block");
        println!("3. Change Difficulty");
        println!("4. Change reward");
        println!("0. Exit");

        io::stdout().flush() ;//reading our input from the console
        choice.clear();
        io::stdin().read_line(&mut choice);

        match choice.trim().parse().unwrap(){
            0 => {
                println!("Exiting");
                process::exit(0);

            },
            1=> {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                println!("Enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);

                println!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap());

                match res {
                    true  => println!("Transaction added"),
                    false => println!("Transaction faailed") // the above is for the first transaction

                }


            },
            2 => {
                // this second option is to mine a block
                println!(" Generating block");
                let res: bool = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }


            },

            3 => {
                //This option is to change the difficulty
                let mut new_diff = String::new();
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());

                match res {
                    true => println!("Updated difficulty"),
                    false => println!("Failed update"),

                }

            },

            4 => {
                // to change the reward
                let mut new_reward = String::new();
                println!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);

                let res: bool = chain.update_reward(new_reward.trim().parse().unwrap());

                match res{
                    true => println!("updated reward"),
                    false => println!("Failed to  update"),
                }

            }
            _ => println!("Wrong choice please try again")
        };
    }

}
