//! standerd beavior for plugins
//! all plugins must implement this trait to some capacity.
//! data is transfered by sharing memory.
//! this is VERY work in progress
use serde::Serialize;

pub enum Updates {
    OnSubmit,
    OnChange,
    Time,
}

pub trait Plugin {
    /// displays the configuration options of the plugin.
    type Config: Serialize;

    /// when to update
    const UPDATE: Updates;

    /// initalise the plugin with a default config and its settings
    fn new() -> (Self::Config, Self);

    /// return if the config is within the allowd ranges
    fn verify(config: Self::Config) -> bool;

    /// set the allowed config
    fn set(config: Self::Config) -> Result<(), ()>;
}
