/*
 * Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#![warn(missing_docs)]
//! # `mCaptcha` database operations
//!
//! Traits and datastructures used in mCaptcha to interact with database.
//!
//! To use an unsupported database with mCaptcha, traits present within this crate should be
//! implemented.
//!
//!
//! ## Organisation
//!
//! Database functionallity is divided accross various modules:
//!
//! - [errors](crate::auth): error data structures used in this crate
//! - [ops](crate::ops): meta operations like connection pool creation, migrations and getting
//! connection from pool
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

pub mod errors;
pub mod ops;
#[cfg(feature = "test")]
pub mod tests;

use dev::*;
pub use ops::GetConnection;

pub mod prelude {
    //! useful imports for users working with a supported database

    pub use super::errors::*;
    pub use super::ops::*;
    pub use super::*;
}

pub mod dev {
    //! useful imports for supporting a new database
    pub use super::prelude::*;
    pub use async_trait::async_trait;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// Data required to register a new user
pub struct Register<'a> {
    /// username of new user
    pub username: &'a str,
    /// secret of new user
    pub secret: &'a str,
    /// hashed password of new use
    pub hash: &'a str,
    /// Optionally, email of new use
    pub email: Option<&'a str>,
}

#[async_trait]
/// mCaptcha's database requirements. To implement support for $Database, kindly implement this
/// trait.
pub trait MCDatabase: std::marker::Send + std::marker::Sync + CloneSPDatabase {
    /// ping DB
    async fn ping(&self) -> bool;

    /// register a new user
    async fn register(&self, p: &Register) -> DBResult<()>;
}

/// Trait to clone MCDatabase
pub trait CloneSPDatabase {
    /// clone DB
    fn clone_db(&self) -> Box<dyn MCDatabase>;
}

impl<T> CloneSPDatabase for T
where
    T: MCDatabase + Clone + 'static,
{
    fn clone_db(&self) -> Box<dyn MCDatabase> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MCDatabase> {
    fn clone(&self) -> Self {
        (**self).clone_db()
    }
}
