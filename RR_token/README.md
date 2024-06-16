# RR_TOKEN


This repository contains a smart contract implementation for a token on the Stellar network, built using the Soroban SDK. The project is organized into several Rust modules, each handling different aspects of the token's functionality.
Contract Deployment

This is the actual contract deployed on the Stellar blockchain with the ID: CDJDZ4D27GR4ZBZCTSVC5ZACDINKXHVVNNW3JUMS5H5HPMYLSEU77HLH. You can interact with the contract using this ID on the Stellar network.

The main components of this project are:

    admin.rs: Manages the administrator functionalities.
    allowance.rs: Handles allowance-related operations.
    balance.rs: Manages balance operations.
    contract.rs: Main contract implementation.
    metadata.rs: Manages token metadata.
    storage_types.rs: Defines various storage types used in the contract.
    test.rs: Contains test cases for the contract.

Detailed Module Descriptions
admin.rs

    has_administrator: Checks if an administrator is set.
    read_administrator: Retrieves the current administrator's address.
    write_administrator: Sets a new administrator's address.

allowance.rs

    read_allowance: Reads the allowance for a spender from a given address.
    write_allowance: Writes or updates the allowance for a spender.
    spend_allowance: Deducts a specified amount from the allowance.

balance.rs

    read_balance: Reads the balance of a given address.
    write_balance: Writes or updates the balance for a given address.
    receive_balance: Increases the balance of a given address.
    spend_balance: Decreases the balance of a given address.

contract.rs

    Implements the core token functionalities, such as initialization, minting, transferring, and burning tokens.
    Implements the token interface from the Soroban SDK.
    Defines the Token struct and its methods.

metadata.rs

    read_decimal: Reads the token's decimal places.
    read_name: Reads the token's name.
    read_symbol: Reads the token's symbol.
    write_metadata: Writes or updates the token's metadata.

storage_types.rs

    Defines constants and data structures for managing storage keys and values.
    Includes DataKey, AllowanceDataKey, AllowanceValue, and various constants for storage management.

test.rs

    Contains test cases to verify the correct functionality of the token contract.
    Includes tests for minting, transferring, burning tokens, and handling allowances.

