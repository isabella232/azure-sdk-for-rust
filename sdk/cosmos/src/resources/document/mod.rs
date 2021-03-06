//! Utilities for interacting with [`Document`]s.

mod document_attributes;
mod indexing_directive;
mod query;

pub use document_attributes::DocumentAttributes;
pub use indexing_directive::IndexingDirective;
pub use query::Query;

use super::Resource;
use crate::{headers, CosmosError};

use azure_core::AddAsHeader;
use http::header::HeaderMap;
use http::request::Builder;
use serde::de::DeserializeOwned;

/// User-defined content in JSON format.
///
/// You can learn more about Documents [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/documents).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document<T> {
    #[serde(flatten)]
    pub document_attributes: DocumentAttributes,
    #[serde(flatten)]
    pub document: T, // raw, id not included
}

impl<T> Document<T> {
    pub fn new(document: T) -> Self {
        let document_attributes = DocumentAttributes::default();

        Self {
            document_attributes,
            document,
        }
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for Document<T>
where
    T: DeserializeOwned,
{
    type Error = CosmosError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let _headers = value.0;
        let body = value.1;

        Ok(serde_json::from_slice(body)?)
    }
}

impl<T> Resource for Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}

impl<T> Resource for &Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QueryCrossPartition {
    Yes,
    No,
}

impl QueryCrossPartition {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Yes => "true",
            Self::No => "false",
        }
    }
}

impl AddAsHeader for QueryCrossPartition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION,
            self.as_bool_str(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParallelizeCrossPartition {
    Yes,
    No,
}

impl ParallelizeCrossPartition {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Yes => "true",
            Self::No => "false",
        }
    }
}

impl AddAsHeader for ParallelizeCrossPartition {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(
            headers::HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY,
            self.as_bool_str(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsUpsert {
    Yes,
    No,
}

impl IsUpsert {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Yes => "true",
            Self::No => "false",
        }
    }
}

impl AddAsHeader for IsUpsert {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_DOCUMENTDB_IS_UPSERT, self.as_bool_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChangeFeed {
    Incremental,
    None,
}

impl AddAsHeader for ChangeFeed {
    fn add_as_header(&self, builder: Builder) -> Builder {
        match self {
            Self::Incremental => builder.header(headers::HEADER_A_IM, "Incremental feed"),
            Self::None => builder,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TenativeWritesAllowance {
    Allow,
    Deny,
}

impl TenativeWritesAllowance {
    fn as_bool_str(&self) -> &str {
        match self {
            Self::Allow => "true",
            Self::Deny => "false",
        }
    }
}

impl AddAsHeader for TenativeWritesAllowance {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_ALLOW_MULTIPLE_WRITES, self.as_bool_str())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PartitionRangeId<'a>(&'a str);

impl<'a> PartitionRangeId<'a> {
    pub fn new(id: &'a str) -> Self {
        Self(id)
    }
}

impl AddAsHeader for PartitionRangeId<'_> {
    fn add_as_header(&self, builder: Builder) -> Builder {
        builder.header(headers::HEADER_DOCUMENTDB_PARTITIONRANGEID, self.0)
    }
}
