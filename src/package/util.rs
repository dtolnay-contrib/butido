//! Utility types for the package definitions
//!
//! These types exist only for the purpose of strong typing
//! and cannot do anything special.

use std::ops::Deref;

use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PackageName(String);

impl Deref for PackageName {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct PackageVersion(String);

impl From<String> for PackageVersion {
    fn from(s: String) -> Self {
        PackageVersion(s)
    }
}

#[derive(Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SystemDependency(String);

#[derive(Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct BuildDependency(String);

#[derive(Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dependency(String);

#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
pub struct HashValue(String);


/// A type which can be used to express a package version constraint
#[derive(Debug, Eq, PartialEq)]
pub enum PackageVersionConstraint {
    Any,
    Latest,
    LowerAs(PackageVersion),
    HigherAs(PackageVersion),
    InRange(PackageVersion, PackageVersion),
    Exact(PackageVersion),
}

impl PackageVersionConstraint {
    pub fn matches(&self, v: &PackageVersion) -> Result<PackageVersionMatch> {
        match self {
            PackageVersionConstraint::Any                   => Ok(PackageVersionMatch::True),
            PackageVersionConstraint::Latest                => Ok(PackageVersionMatch::Undecided),
            PackageVersionConstraint::LowerAs(vers)         => unimplemented!(),
            PackageVersionConstraint::HigherAs(vers)        => unimplemented!(),
            PackageVersionConstraint::InRange(vers1, vers2) => unimplemented!(),
            PackageVersionConstraint::Exact(vers)           => Ok(PackageVersionMatch::from(*v == *vers)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PackageVersionMatch {
    True,
    False,
    Undecided,
}

impl PackageVersionMatch {
    pub fn is_true(&self) -> bool {
        *self == PackageVersionMatch::True
    }

    pub fn is_false(&self) -> bool {
        *self == PackageVersionMatch::False
    }

    pub fn is_undecided(&self) -> bool {
        *self == PackageVersionMatch::Undecided
    }
}

impl From<bool> for PackageVersionMatch {
    fn from(b: bool) -> Self {
        if b {
            PackageVersionMatch::True
        } else {
            PackageVersionMatch::False
        }
    }
}

