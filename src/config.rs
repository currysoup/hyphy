use num_cpus;

pub struct Config {
    threads: usize,
}

impl Config {
    pub fn defaults() -> Config {
        Config {
            threads: num_cpus::get() * 5 / 4, // default thread count in hyper
        }
    }
}
