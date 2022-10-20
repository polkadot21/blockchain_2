use hex_literal::hex;
use sha256::digest;
use ripemd::{Ripemd160};
use whirlpool::{Whirlpool};
use sha2::{Digest, Sha256};

//task_3
fn compare_hash_functions() {
    let test_string = "Hashfunktionen sind wichtig!";

    //sha256
    let hashed_sha256 = digest(test_string);
    println!("SHA256: {}", hashed_sha256);

    //ripemd160
    let mut hasher_ripemd160 = Ripemd160::new();
    hasher_ripemd160.update(test_string);
    let hashed_ripemd160 = hasher_ripemd160.finalize();
    println!("RIPEMD160: {:X}", hashed_ripemd160);

    //whirlpool
    let mut hasher_whirlpool = Whirlpool::new();
    hasher_whirlpool.update(test_string);
    let result_whirlpool = hasher_whirlpool.finalize();
    println!("WHIRLPOOL: {:X}", result_whirlpool);
}

//task 4
fn find_number_of_people_with_probability(probability: f64) -> f64 {
    let base = 0.99726;
    let number_of_people = (1.0-probability).log(base as f64).round();
    return number_of_people;

}


//task 8
fn brute_force_sha256(max_amount: u32) -> u32{

    let mut i = 0;
    while i < max_amount {

        let i_str: String = i.to_string();

        let mut hasher = Sha256::new();
        hasher.update(i_str);
        let result = hasher.finalize();

        if result[..] == hex!("6a22abc984f8bed473a6fe7841cb520f879ebfa97f7b1ad0c24bcb2e7bc79428")[..] {
            return i;

        } else {
            i = i + 1;
        }
    }

    return 0;
}

fn main() {

    const MAX_AMOUNT: u32 = 10000000;

    // Hashen Sie folgenden String „Hashfunktionen sind wichtig!“ mit
    // • SHA256,
    // • RIPEMD160 und
    // • Whirlpool
    println!("task 3");
    compare_hash_functions();


    // Wie groß muss die Gruppe sein, damit die Wahrscheinlichkeit mindestens 50 %, 75 %, 99 % beträgt,
    // dass eine Person an einem bestimmten Tag Geburtstag hat?
    // a) Berechnen Sie die Lösung in einem Tabellenkalkulationsprogramm.
    // b) Nutzen Sie den Logarithmus zur Berechnung.
    println!();
    println!("task 4");
    let probabilities = [0.5, 0.75, 0.99];
    for _probability in probabilities {
        let n_people: f64 = find_number_of_people_with_probability(_probability);
        println!("for probability: {} we need {} people", _probability, n_people);
    }

    // Richard Rich hat in dem folgenden SHA-256-Hash
    // 6a22abc984f8bed473a6fe7841cb520f879ebfa97f7b1ad0c24bcb2e7bc79428 die Höhe seines
    // Vermögens „versteckt“.
    // Finden Sie heraus, wie viele Euro Richard Rich besitzt.
    println!();
    println!("task 8");
    let amount  = brute_force_sha256(MAX_AMOUNT);
    println!("Richard Rich has {} $", amount);

}
