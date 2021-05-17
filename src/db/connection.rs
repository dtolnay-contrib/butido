//
// Copyright (c) 2020-2021 science+computing ag and other contributors
//
// This program and the accompanying materials are made
// available under the terms of the Eclipse Public License 2.0
// which is available at https://www.eclipse.org/legal/epl-2.0/
//
// SPDX-License-Identifier: EPL-2.0
//

use anyhow::Error;
use anyhow::Result;
use clap::ArgMatches;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use getset::Getters;
use log::debug;

use crate::config::Configuration;

#[derive(Getters)]
pub struct DbConnectionConfig {
    #[getset(get = "pub")]
    database_host: String,

    #[getset(get = "pub")]
    database_port: String,

    #[getset(get = "pub")]
    database_user: String,

    #[getset(get = "pub")]
    database_password: String,

    #[getset(get = "pub")]
    database_name: String,
}

impl Into<String> for DbConnectionConfig {
    fn into(self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{name}",
            host = self.database_host,
            port = self.database_port,
            user = self.database_user,
            password = self.database_password,
            name = self.database_name
        )
    }
}

impl std::fmt::Debug for DbConnectionConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "postgres://{user}:PASSWORD@{host}:{port}/{name}",
            host = self.database_host,
            port = self.database_port,
            user = self.database_user,
            name = self.database_name,
        )
    }
}

pub fn parse_db_connection_config(config: &Configuration, cli: &ArgMatches) -> DbConnectionConfig {
    fn find_value<F>(cli: &ArgMatches, key: &str, alternative: F) -> String
    where
        F: FnOnce() -> String,
    {
        cli.value_of(key)
            .map(String::from)
            .unwrap_or_else(alternative)
    }

    let database_host = find_value(cli, "database_host", || config.database_host().to_string());
    let database_port = find_value(cli, "database_port", || config.database_port().to_string());
    let database_user = find_value(cli, "database_user", || config.database_user().to_string());
    let database_password = find_value(cli, "database_password", || {
        config.database_password().to_string()
    });
    let database_name = find_value(cli, "database_name", || config.database_name().to_string());

    DbConnectionConfig {
        database_host,
        database_port,
        database_user,
        database_password,
        database_name,
    }
}

pub fn establish_connection(conn_config: DbConnectionConfig) -> Result<PgConnection> {
    debug!("Trying to connect to database: {:?}", conn_config);
    let database_uri: String = conn_config.into();
    PgConnection::establish(&database_uri).map_err(Error::from)
}
