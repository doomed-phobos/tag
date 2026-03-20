use xxhash_rust::xxh3;
use std::{collections::HashMap, fs::OpenOptions, io::{self, Write}, path::{Path, PathBuf}};
use serde::{Deserialize, Serialize, de::{DeserializeOwned, Error}};

pub struct NonEmptyString(String);

impl NonEmptyString {
  pub fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}

impl TryFrom<String> for NonEmptyString {
  type Error = &'static str;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    if value.trim().is_empty() {
      Err("Cannot create NonEmptyString from emptry string")
    } else {
      Ok(Self(value))
    }
  }
}

impl Serialize for NonEmptyString {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: serde::Serializer {
    serializer.serialize_str(&self.0)
  }
}

impl<'de> Deserialize<'de> for NonEmptyString {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
          D: serde::Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    NonEmptyString::try_from(s).map_err(D::Error::custom)
  }
}

#[derive(Serialize, Deserialize)]
pub struct Tag {
  id: u64,
  name: NonEmptyString,
}

impl Tag {
  pub fn new(name: NonEmptyString) -> Self {
    Self {
      id: xxh3::xxh3_64(name.as_bytes()),
      name
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
  id: u64,
  name: NonEmptyString,
  tags: Vec<(u64 /* TagId */, Option<NonEmptyString> /* Image */)>,
}

impl Artist {
  pub fn id(&self) -> u64 {
    self.id
  }
}

impl Artist {
  pub fn new(name: NonEmptyString) -> Self {
    Self {
      id: xxh3::xxh3_64(name.as_bytes()),
      name,
      tags: Vec::new()
    }
  }
}

pub struct Database {
  artists: HashMap<u64, Artist>,
  tags: HashMap<u64, Tag>,
  path: PathBuf,
}

impl Database {
  const ARTIST_TABLE_NAME: &str = "artists";
  const TAG_TABLE_NAME: &str = "tags";

  fn load_table<T: DeserializeOwned>(path: PathBuf) -> Result<Vec<T>, io::Error> {
    if !path.exists() {
      return Ok(Vec::new());
    }

    let file = OpenOptions::new()
      .read(true)
      .open(path)?;

    let data: Vec<T> = rmp_serde::from_read(file)
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?; 

    Ok(data)
  }

  fn save_table<T: Serialize>(values: Vec<&T>, path: PathBuf) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(path)?;

    let bytes = rmp_serde::to_vec(&values)
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?; 

    file.write_all(&bytes)?;

    Ok(())
  }

  pub fn try_load(path: PathBuf) -> Result<Self, io::Error> {
    let artists = Self::load_table::<Artist>(path.join(Self::ARTIST_TABLE_NAME))?;
    let tags = Self::load_table::<Tag>(path.join(Self::TAG_TABLE_NAME))?;

    Ok(Self {
      path,
      artists: artists.into_iter().map(|x| (x.id, x)).collect(),
      tags: tags.into_iter().map(|x| (x.id, x)).collect(),
    })
  }

  pub fn save(&self) -> Result<(), io::Error> {
    Self::save_table(self.artists.values().collect(), self.path.join(Self::ARTIST_TABLE_NAME))?;
    Self::save_table(self.tags.values().collect(), self.path.join(Self::TAG_TABLE_NAME))?;

    Ok(())
  }
}