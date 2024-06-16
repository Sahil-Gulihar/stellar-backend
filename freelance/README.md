freelance

This directory includes a Rust project with Cargo.lock and Cargo.toml.
It is the Initial boilerplate with all necessary functions and logic..
This code was initialised when project started

Overview

The Rust project in the freelance directory is a smart contract designed for managing freelancer gigs on the Stellar network. The main components of this contract are defined in lib.rs and events.rs.
lib.rs

The lib.rs file is the main entry point for the smart contract logic. It uses the soroban_sdk to interact with the Stellar blockchain.
Key Components

    Attributes and Imports:
        #![no_std]: Declares that the contract does not use the Rust standard library.
        Imports various modules and traits from the soroban_sdk and defines a module for events.

    Data Structures:
        DataPoint: Enum representing different types of data points stored in the contract.
        FreeLancer: Enum representing various attributes of a freelancer.
        State: Enum representing the different states of a gig.

    Function Implementations:
        get_ledger_timestamp: Retrieves the current timestamp from the ledger.
        get_deadline: Retrieves the deadline for the gig from the contract storage.
        get_freelancer: Retrieves the freelancer's address from the contract storage.
        get_claimed: Checks if the freelancer has claimed the reward.
        get_token: Retrieves the address of the token used in the contract.
        get_rating: Retrieves the rating of the freelancer from the contract storage.
        get_skills: Retrieves the freelancer's skills from the contract storage.
        get_user_deposited: Retrieves the amount deposited by a user.
        get_balance: Retrieves the balance of the contract in the specified token.
        set_rating: Sets the rating for the freelancer, ensuring it is between 0 and 5.
        set_user_deposited: Sets the deposited amount for a user.
        set_claimed: Marks the freelancer's reward as claimed.
        transfer: Transfers the specified amount of tokens to a given address.
        get_state: Determines the current state of the gig based on the ledger timestamp and deadline.

    Contract Implementation:
        transaction: Initializes a new transaction with the freelancer's details, deadline, token, and rating.
        freelancer: Returns the freelancer's address.
        deadline: Returns the deadline of the gig.
        state: Returns the current state of the gig as an integer.
        token: Returns the token address used in the contract.
        balance: Returns the balance of a user or the contract.
        deposit: Handles deposits made by users into the contract.
        withdraw: Allows users or the freelancer to withdraw funds based on the gig's state.

events.rs

The events.rs file contains the event handling logic for the contract.
Key Component

    Event Publishing:
        amount_changed: Publishes an event whenever the total amount changes in the contract.

Usage

To use this contract, ensure you have Rust and the necessary dependencies installed. Then, you can compile and deploy the contract on the Stellar network using the Soroban SDK.

bash

cargo build --target wasm32-unknown-unknown --release

Deploy the compiled WASM file to the Stellar network and interact with it using the provided functions in the contract.
