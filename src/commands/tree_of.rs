//
// Copyright (c) 2020-2021 science+computing ag and other contributors
//
// This program and the accompanying materials are made
// available under the terms of the Eclipse Public License 2.0
// which is available at https://www.eclipse.org/legal/epl-2.0/
//
// SPDX-License-Identifier: EPL-2.0
//

//! Implementation of the 'tree-of' subcommand

use std::convert::TryFrom;

use anyhow::Error;
use anyhow::Result;
use clap::ArgMatches;
use resiter::AndThen;

use crate::package::Dag;
use crate::package::PackageName;
use crate::package::PackageVersionConstraint;
use crate::package::condition::ConditionData;
use crate::repository::Repository;
use crate::util::EnvironmentVariableName;
use crate::util::docker::ImageName;

/// Implementation of the "tree_of" subcommand
pub async fn tree_of(
    matches: &ArgMatches,
    repo: Repository,
) -> Result<()> {
    let pname = matches
        .value_of("package_name")
        .map(String::from)
        .map(PackageName::from);
    let pvers = matches
        .value_of("package_version")
        .map(PackageVersionConstraint::try_from)
        .transpose()?;

    let image_name = matches
        .value_of("image")
        .map(String::from)
        .map(ImageName::from);

    let additional_env = matches
        .values_of("env")
        .unwrap_or_default()
        .map(crate::util::env::parse_to_env)
        .collect::<Result<Vec<(EnvironmentVariableName, String)>>>()?;

    let condition_data = ConditionData {
        image_name: image_name.as_ref(),
        env: &additional_env,
    };

    repo.packages()
        .filter(|p| pname.as_ref().map(|n| p.name() == n).unwrap_or(true))
        .filter(|p| {
            pvers
                .as_ref()
                .map(|v| v.matches(p.version()))
                .unwrap_or(true)
        })
        .map(|package| Dag::for_root_package(package.clone(), &repo, None, &condition_data))
        .and_then_ok(|tree| {
            let stdout = std::io::stdout();
            let mut outlock = stdout.lock();

            ptree::write_tree(&tree.display(), &mut outlock).map_err(Error::from)
        })
        .collect::<Result<()>>()
}
