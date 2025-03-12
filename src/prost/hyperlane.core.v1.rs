// @generated
/// Dispatch ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dispatch {
    /// origin_mailbox_id ...
    #[prost(string, tag="1")]
    pub origin_mailbox_id: ::prost::alloc::string::String,
    /// sender ...
    #[prost(string, tag="2")]
    pub sender: ::prost::alloc::string::String,
    /// destination ...
    #[prost(uint32, tag="3")]
    pub destination: u32,
    /// recipient ...
    #[prost(string, tag="4")]
    pub recipient: ::prost::alloc::string::String,
    /// message ...
    #[prost(string, tag="5")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for Dispatch {
const NAME: &'static str = "Dispatch";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// Process ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Process {
    /// origin_mailbox_id ...
    #[prost(string, tag="1")]
    pub origin_mailbox_id: ::prost::alloc::string::String,
    /// origin ...
    #[prost(uint32, tag="2")]
    pub origin: u32,
    /// sender ...
    #[prost(string, tag="3")]
    pub sender: ::prost::alloc::string::String,
    /// recipient ...
    #[prost(string, tag="4")]
    pub recipient: ::prost::alloc::string::String,
    /// message_id ...
    #[prost(string, tag="5")]
    pub message_id: ::prost::alloc::string::String,
    /// message ...
    #[prost(string, tag="6")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for Process {
const NAME: &'static str = "Process";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// Mailbox ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mailbox {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner ...
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// message_sent ...
    #[prost(uint32, tag="3")]
    pub message_sent: u32,
    /// message_received ...
    #[prost(uint32, tag="4")]
    pub message_received: u32,
    /// default_ism ...
    #[prost(string, tag="5")]
    pub default_ism: ::prost::alloc::string::String,
    /// default_hook
    #[prost(string, tag="8")]
    pub default_hook: ::prost::alloc::string::String,
    /// required_hook
    #[prost(string, tag="9")]
    pub required_hook: ::prost::alloc::string::String,
    /// domain
    #[prost(uint32, tag="10")]
    pub local_domain: u32,
}
impl ::prost::Name for Mailbox {
const NAME: &'static str = "Mailbox";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// GenesisState is the state that must be provided at genesis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// ism_genesis
    #[prost(message, optional, tag="1")]
    pub ism_genesis: ::core::option::Option<super::interchain_security::v1::GenesisState>,
    /// post_dispatch_genesis
    #[prost(message, optional, tag="2")]
    pub post_dispatch_genesis: ::core::option::Option<super::post_dispatch::v1::GenesisState>,
    #[prost(message, repeated, tag="3")]
    pub mailboxes: ::prost::alloc::vec::Vec<Mailbox>,
    #[prost(message, repeated, tag="4")]
    pub messages: ::prost::alloc::vec::Vec<MailboxMessage>,
    #[prost(uint64, tag="5")]
    pub ism_sequence: u64,
    #[prost(uint64, tag="6")]
    pub post_dispatch_sequence: u64,
    #[prost(uint64, tag="7")]
    pub app_sequence: u64,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// Mailbox message for genesis state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MailboxMessage {
    #[prost(uint64, tag="1")]
    pub mailbox_id: u64,
    #[prost(bytes="vec", tag="2")]
    pub message_id: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for MailboxMessage {
const NAME: &'static str = "MailboxMessage";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMailboxesRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMailboxesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryMailboxesRequest {
const NAME: &'static str = "QueryMailboxesRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMailboxesResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMailboxesResponse {
    #[prost(message, repeated, tag="1")]
    pub mailboxes: ::prost::alloc::vec::Vec<Mailbox>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryMailboxesResponse {
const NAME: &'static str = "QueryMailboxesResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMailboxRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMailboxRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryMailboxRequest {
const NAME: &'static str = "QueryMailboxRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryMailboxResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMailboxResponse {
    #[prost(message, optional, tag="1")]
    pub mailbox: ::core::option::Option<Mailbox>,
}
impl ::prost::Name for QueryMailboxResponse {
const NAME: &'static str = "QueryMailboxResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryDeliveredRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDeliveredRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryDeliveredRequest {
const NAME: &'static str = "QueryDeliveredRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryDeliveredResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDeliveredResponse {
    #[prost(bool, tag="1")]
    pub delivered: bool,
}
impl ::prost::Name for QueryDeliveredResponse {
const NAME: &'static str = "QueryDeliveredResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// RecipientIsmRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecipientIsmRequest {
    #[prost(string, tag="1")]
    pub recipient: ::prost::alloc::string::String,
}
impl ::prost::Name for RecipientIsmRequest {
const NAME: &'static str = "RecipientIsmRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// RecipientIsmResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecipientIsmResponse {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
}
impl ::prost::Name for RecipientIsmResponse {
const NAME: &'static str = "RecipientIsmResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryVerifyDryRunRequest ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyDryRunRequest {
    #[prost(string, tag="1")]
    pub ism_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryVerifyDryRunRequest {
const NAME: &'static str = "QueryVerifyDryRunRequest";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// QueryVerifyDryRunResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVerifyDryRunResponse {
    #[prost(bool, tag="1")]
    pub verified: bool,
}
impl ::prost::Name for QueryVerifyDryRunResponse {
const NAME: &'static str = "QueryVerifyDryRunResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgCreateMailbox ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMailbox {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// local domain
    #[prost(uint32, tag="2")]
    pub local_domain: u32,
    #[prost(string, tag="3")]
    pub default_ism: ::prost::alloc::string::String,
    /// default_hook ...
    #[prost(string, tag="4")]
    pub default_hook: ::prost::alloc::string::String,
    /// required_hook ...
    #[prost(string, tag="5")]
    pub required_hook: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMailbox {
const NAME: &'static str = "MsgCreateMailbox";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgCreateMailboxResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateMailboxResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCreateMailboxResponse {
const NAME: &'static str = "MsgCreateMailboxResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgSetMailbox ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMailbox {
    /// owner is the message sender.
    #[prost(string, tag="1")]
    pub owner: ::prost::alloc::string::String,
    /// mailbox_id
    #[prost(string, tag="2")]
    pub mailbox_id: ::prost::alloc::string::String,
    /// default_ism ...
    #[prost(string, tag="3")]
    pub default_ism: ::prost::alloc::string::String,
    /// default_hook ...
    #[prost(string, tag="4")]
    pub default_hook: ::prost::alloc::string::String,
    /// required_hook ...
    #[prost(string, tag="5")]
    pub required_hook: ::prost::alloc::string::String,
    /// new_owner ...
    #[prost(string, tag="6")]
    pub new_owner: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSetMailbox {
const NAME: &'static str = "MsgSetMailbox";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgSetMailboxResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetMailboxResponse {
}
impl ::prost::Name for MsgSetMailboxResponse {
const NAME: &'static str = "MsgSetMailboxResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgProcessMessage ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgProcessMessage {
    /// mailbox_id ...
    #[prost(string, tag="1")]
    pub mailbox_id: ::prost::alloc::string::String,
    /// relayer ...
    #[prost(string, tag="2")]
    pub relayer: ::prost::alloc::string::String,
    /// metadata ...
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    /// message ...
    #[prost(string, tag="4")]
    pub message: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgProcessMessage {
const NAME: &'static str = "MsgProcessMessage";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
/// MsgProcessMessageResponse ...
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgProcessMessageResponse {
}
impl ::prost::Name for MsgProcessMessageResponse {
const NAME: &'static str = "MsgProcessMessageResponse";
const PACKAGE: &'static str = "hyperlane.core.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.v1.{}", Self::NAME)
            }}
include!("hyperlane.core.v1.tonic.rs");
// @@protoc_insertion_point(module)