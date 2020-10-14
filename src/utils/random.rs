use rand::seq::SliceRandom;

pub fn gen_sequence(len: u8) -> String {
    let mut alph = String::from("abcdefghijklmnopqrstuvwxyz");
    alph += &alph.to_uppercase();
    let chars: Vec<char> = alph.chars().collect();
    let mut cache = String::new();
    for _ in 0..len {
        cache.push(*chars.choose(&mut rand::thread_rng()).unwrap());
    };
    cache
}

/*
pub fn choice<T>(opts: Vec<T>) -> T {
    let mut rng = rand::thread_rng();
    let choice = opts.choose(&mut rng).unwrap();
    *choice
}
*/