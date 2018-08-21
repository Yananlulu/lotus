#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "mysql")]
pub use self::mysql::{schema, Config, Connection, DRIVER};

#[cfg(feature = "postgresql")]
pub mod postgresql;
#[cfg(feature = "postgresql")]
pub use self::postgresql::{schema, Config, Connection, DRIVER};

use std::ops::Deref;

use diesel::r2d2::ConnectionManager;
use r2d2;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;
pub struct PooledConnection(pub r2d2::PooledConnection<ConnectionManager<Connection>>);

impl Deref for PooledConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
