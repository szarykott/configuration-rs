use crate::{
    configuration::Configuration,
    error::ConfigurationError,
    format::Transformer,
    provider::{AsyncProvider, Provider, ProviderStruct},
    source::{AsyncSource, Source},
};
use std::default::Default;

pub struct ConfigurationBuilder<'builder> {
    sources: Vec<Box<dyn Provider + 'builder>>,
}

impl<'builder> Default for ConfigurationBuilder<'builder> {
    fn default() -> Self {
        ConfigurationBuilder::new()
    }
}

/// Holds intermediate configuration sources in order of adding them.
impl<'builder> ConfigurationBuilder<'builder> {
    fn new() -> Self {
        ConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    /// Core function to add new configurations to builder.
    pub fn add<S, D>(&mut self, source: S, de: D) -> &mut ConfigurationBuilder<'builder>
    where
        S: Source + 'builder,
        D: Transformer + 'builder,
    {
        self.sources
            .push(Box::new(ProviderStruct::synchronous(source, de)));
        self
    }

    pub fn add_provider<P>(&mut self, provider: P) -> &mut ConfigurationBuilder<'builder>
    where
        P: Provider + 'builder,
    {
        self.sources.push(Box::new(provider));
        self
    }

    pub fn add_async<S, D>(self, source: S, de: D) -> AsyncConfigurationBuilder<'builder>
    where
        S: AsyncSource + Send + Sync + 'builder,
        D: Transformer + Send + Sync + 'builder,
    {
        let mut async_builder = AsyncConfigurationBuilder::from_synchronous_builder(self);
        async_builder.add_async(source, de);
        async_builder
    }

    pub fn build(&mut self) -> Result<Configuration, ConfigurationError> {
        let mut result = Configuration::default();

        for provider in self.sources.iter_mut() {
            let roots = provider.collect()?;
            for configuration in roots.roots {
                result.roots.push(configuration);
            }
        }

        Ok(result)
    }
}

pub struct AsyncConfigurationBuilder<'builder> {
    sources: Vec<SourceType<'builder>>,
}

impl<'builder> AsyncConfigurationBuilder<'builder> {
    fn new() -> Self {
        AsyncConfigurationBuilder {
            sources: Vec::new(),
        }
    }

    pub fn from_synchronous_builder(
        mut builder: ConfigurationBuilder<'builder>,
    ) -> AsyncConfigurationBuilder<'builder> {
        AsyncConfigurationBuilder {
            sources: builder
                .sources
                .drain(..)
                .map(|s| (SourceType::Synchronous(s)))
                .collect(),
        }
    }

    pub fn add<S, D>(&mut self, source: S, de: D)
    where
        S: Source + 'builder,
        D: Transformer + 'builder,
    {
        self.sources.push(SourceType::Synchronous(Box::new(
            ProviderStruct::synchronous(source, de),
        )));
    }

    pub fn add_async<S, D>(&mut self, source: S, de: D)
    where
        S: AsyncSource + Send + Sync + 'builder,
        D: Transformer + Send + Sync + 'builder,
    {
        self.sources.push(SourceType::Asynchronous(Box::new(
            ProviderStruct::asynchronous(source, de),
        )));
    }
}

enum SourceType<'builder> {
    Synchronous(Box<dyn Provider + 'builder>),
    Asynchronous(Box<dyn AsyncProvider + Send + Sync + 'builder>),
}
