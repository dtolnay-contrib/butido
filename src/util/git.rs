//
// Copyright (c) 2020-2021 science+computing ag and other contributors
//
// This program and the accompanying materials are made
// available under the terms of the Eclipse Public License 2.0
// which is available at https://www.eclipse.org/legal/epl-2.0/
//
// SPDX-License-Identifier: EPL-2.0
//

use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Error;
use anyhow::Result;
use git2::Repository;
use log::trace;

pub fn repo_is_clean(p: &Path) -> Result<bool> {
    Repository::open(p)
        .map_err(Error::from)
        .map(|r| r.state() == git2::RepositoryState::Clean)
}

pub fn get_repo_head_commit_hash(p: &Path) -> Result<String> {
    let r =
        Repository::open(p).with_context(|| anyhow!("Opening repository at {}", p.display()))?;

    let s = r
        .head()
        .with_context(|| anyhow!("Getting HEAD from repository at {}", p.display()))?
        .peel_to_commit()
        .with_context(|| anyhow!("Failed to get commit hash: Not valid UTF8"))?
        .id()
        .to_string();

    trace!("Found git commit hash = {}", s);
    Ok(s)
}
