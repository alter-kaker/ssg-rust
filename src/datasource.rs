
use ureq::Agent;

use crate::{collection::Collection, error::GeneratorError};

pub trait DataSource {
    fn fetch(&self) -> Result<Collection, GeneratorError>;
}

pub struct ApiSource {
    agent: Agent,
    uri: String,
}

impl DataSource for ApiSource {
    fn fetch(&self) -> Result<Collection, GeneratorError> {
        Ok(self.agent.get(&self.uri).call()?.into_json::<Collection>()?)
    }
}


