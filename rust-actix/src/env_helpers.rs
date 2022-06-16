use std::{env, str::FromStr};

pub fn set_default_env_var(key: &str, value: &str) {
    if env::var(key).is_err() {
        env::set_var(key, value);
    }
}

pub fn cast_required_env_var<F: FromStr>(key: &str) -> F
where
    <F as FromStr>::Err: std::fmt::Debug,
{
    env::var(key)
        .expect(&format!("{} env var does not exist!", key))
        .parse::<F>()
        .expect(&format!("Error casting {} env var!", key))
}
