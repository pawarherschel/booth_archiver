use ron::ser::PrettyConfig;
use serde::de::DeserializeOwned;
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use tracing::log::trace;
use tracing::{info, instrument};
use tracing_unwrap::ResultExt;

#[derive(Debug, Clone)]
pub struct Cache<'cache, Inner>
where
    Inner: KVInner,
{
    inner: Inner,
    path: Cow<'cache, Path>,
}

impl<'cache, Inner: KVInner + serde::Serialize> Serialize for Cache<'cache, Inner> {
    #[tracing::instrument(level = "trace", skip(self, serializer))]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<Inner> Default for Cache<'_, Inner>
where
    Inner: KVInner + Default + Debug,
{
    #[tracing::instrument(level = "trace", skip(), ret)]
    fn default() -> Self {
        Self {
            path: Cow::from(Path::new("cache/cache")),
            ..Default::default()
        }
    }
}

// TODO: make K, and V
impl<'cache> KVInner for BTreeMap<Cow<'cache, str>, Cow<'cache, str>> {
    type Key = Cow<'cache, str>;
    type Value = Cow<'cache, str>;

    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        trace!("getting key {key}");
        self.get(key)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn set(&mut self, key: Self::Key, value: Self::Value) {
        trace!("setting key: {key} to value: {value}");
        self.insert(key, value);
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn clear(&mut self) {
        trace!("clearing");
        self.clear();
    }
}

impl<'cache, Inner> Deref for Cache<'cache, Inner>
where
    Inner: KVInner,
{
    type Target = Inner;

    #[tracing::instrument(level = "trace", skip(self))]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<Inner> DerefMut for Cache<'_, Inner>
where
    Inner: KVInner,
{
    #[tracing::instrument(level = "trace", skip(self))]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'cache, Inner> KV for Cache<'cache, Inner>
where
    Inner: KVInner,
{
    type Inner = Inner;

    #[tracing::instrument(level = "trace", skip(self), ret)]
    fn get_path(&self) -> impl AsRef<Path> {
        &self.path
    }
}

impl<'cache, Inner> Cache<'cache, Inner>
where
    Inner: KVInner,
{
    #[tracing::instrument(level = "trace")]
    pub fn new<P>(path: &'cache P) -> Self
    where
        P: Debug + AsRef<std::ffi::OsStr>,
        Inner: Default + DeserializeOwned,
    {
        let path = Path::new(path);

        let path: Cow<'cache, Path> = path.into();

        let ron_path = format!("{}.ron", path.display());
        let cache = fs::read_to_string(&ron_path).unwrap_or_default();

        let s_cache = cache.as_str();

        let inner = if fs::metadata(&ron_path).is_ok() {
            trace!("loading cache from path: {}", ron_path);
            ron::from_str(s_cache).unwrap_or_log()
        } else {
            trace!("created new cache at path: {}", ron_path);
            Default::default()
        };

        Cache { inner, path }
    }
}

pub trait KVInner
where
    <Self as KVInner>::Key: Sized,
    <Self as KVInner>::Value: Sized,
{
    type Key;
    type Value;
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;
    fn set(&mut self, key: Self::Key, value: Self::Value);
    fn clear(&mut self);
}

pub trait KV: DerefMut<Target = <Self as KV>::Inner> + Deref<Target = <Self as KV>::Inner>
where
    <Self as KV>::Inner: Sized + KVInner,
{
    type Inner;

    fn get_path(&self) -> impl AsRef<Path>;

    #[tracing::instrument(level = "trace", skip(self, key, value))]
    fn insert<IK, IV>(&mut self, key: IK, value: IV)
    where
        <<Self as KV>::Inner as KVInner>::Key: From<IK>,
        <<Self as KV>::Inner as KVInner>::Value: From<IV>,
    {
        let key = key.into();
        let value = value.into();

        self.deref_mut().set(key, value);
    }

    #[tracing::instrument(level = "trace", skip(self, key))]
    fn get<IK>(&self, key: IK) -> Option<&<<Self as KV>::Inner as KVInner>::Value>
    where
        <<Self as KV>::Inner as KVInner>::Key: From<IK>,
    {
        let key = key.into();

        self.deref().get(&key)
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn persist(&self)
    where
        <Self as KV>::Inner: serde::Serialize,
    {
        let ron = self.get_path();
        let path_root = ron.as_ref().display();
        let ron_path = format!("{path_root}.ron");
        let f = fs::File::create(&ron_path).unwrap_or_log();
        ron::ser::to_writer_pretty(f, &**self, PrettyConfig::default()).unwrap_or_log();
        info!("wrote ron file to {ron_path}");

        let json = self.get_path();
        let path_root = json.as_ref().display();
        let json_path = format!("{path_root}.json");
        let f = fs::File::create(&json_path).unwrap_or_log();
        serde_json::to_writer_pretty(f, &**self).unwrap_or_log();
        info!("wrote json file to {json_path}");
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn clear(&mut self) {
        KVInner::clear(&mut **self);
    }
}
