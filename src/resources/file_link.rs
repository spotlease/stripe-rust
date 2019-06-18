// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{FileId, FileLinkId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{File, Scheduled};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "FileLink".
///
/// For more details see [https://stripe.com/docs/api/file_links/object](https://stripe.com/docs/api/file_links/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileLink {
    /// Unique identifier for the object.
    pub id: FileLinkId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Whether this link is already expired.
    pub expired: bool,

    /// Time at which the link expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,

    /// The file object this link points to.
    pub file: Expandable<File>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The publicly accessible URL to download the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Object for FileLink {
    type Id = FileLinkId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "file_link"
    }
}

/// The parameters for `FileLink::create`.
#[derive(Clone, Debug, Serialize)]
pub struct FileLinkCreateParams<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A future timestamp after which the link will no longer be usable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,

    /// The ID of the file.
    ///
    /// The file's `purpose` must be one of the following: `business_icon`, `business_logo`, `customer_signature`, `dispute_evidence`, `finance_report_run`, `pci_document`, `sigma_scheduled_query`, or `tax_document_user_upload`.
    pub file: FileId,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// The parameters for `FileLink::list`.
#[derive(Clone, Debug, Serialize)]
pub struct FileLinkListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<FileLinkId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Filter links by their expiration status.
    ///
    /// By default, all links are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,

    /// Only return links for the given file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<FileId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<FileLinkId>,
}

/// The parameters for `FileLink::update`.
#[derive(Clone, Debug, Serialize)]
pub struct FileLinkUpdateParams<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Scheduled>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
