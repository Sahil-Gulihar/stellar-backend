use soroban_sdk::{vec, Env, Symbol};

pub(crate) fn amount_changed(e: &Env, total_amount: i128) {
    let topics = (Symbol::new(e, "amount_changed"),);
    e.events().publish(topics, total_amount);
}