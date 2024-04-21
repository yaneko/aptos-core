module module_owner::dice {
    use std::signer::address_of;
    use aptos_framework::randomness;

    struct DiceRollHistory has drop, key {
        last_roll: u64,
    }

    #[randomness]
    entry fun roll(account: signer) acquires DiceRollHistory {
        let addr = address_of(&account);
        let roll_history = if (exists<DiceRollHistory>(addr)) {
            move_from<DiceRollHistory>(addr)
        } else {
            DiceRollHistory { last_roll: 999 }
        };
        let new_roll = randomness::u64_range(0, 6);
        move_to(&account, DiceRollHistory { last_roll: new_roll })
    }
}
