use rand::Rng;

const ENV_SEED: Option<&str> = option_env!("QUINT_SEED");

pub fn gen_random_seed() -> String {
    ENV_SEED.map(str::to_string).unwrap_or_else(|| {
        let seed = rand::rng().random::<u32>();
        format!("0x{:x}", seed)
    })
}
