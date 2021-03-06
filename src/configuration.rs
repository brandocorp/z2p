#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
  pub port: u16,
  pub host: String,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
  pub username: String,
  pub password: String,
  pub port: u16,
  pub host: String,
  pub database: String,
}

impl DatabaseSettings {
  pub fn connection_string(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}/{}",
      self.username,
      self.password,
      self.host,
      self.port,
      self.database
    )
  }

  pub fn connection_string_without_db(&self) -> String {
    format!(
      "postgres://{}:{}@{}:{}",
      self.username,
      self.password,
      self.host,
      self.port
    )
  }
}

#[derive(serde::Deserialize)]
pub struct Settings {
  pub db: DatabaseSettings,
  pub api: ApplicationSettings,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  let mut settings = config::Config::default();

  settings.merge(config::File::with_name("config"))?;
  settings.try_into()
}