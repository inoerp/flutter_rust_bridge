use crate::app::{cache::global_cache::GlobalCache, system::error::no_value::NoValueFoundError};

pub struct AppUrl;

impl AppUrl {
    pub fn get_base_url() -> Result<String, NoValueFoundError> {
        let settings = GlobalCache::get_settings().map_err(|err| {
            let msg = format!("Error in get_full_url : {:?}", err);
            NoValueFoundError::new(&msg)
        })?;
        let ret = settings.protocol.to_string()
            + "://"
            + settings.host.as_str()
            + ":"
            + settings.port.as_str();
        Ok(ret)
    }

    pub fn get_full_url(path: &str) -> Result<String, NoValueFoundError> {
        Ok(Self::get_base_url()? + "/" + path)
    }
}
