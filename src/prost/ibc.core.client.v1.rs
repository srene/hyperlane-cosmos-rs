// @generated
/// IdentifiedClientState defines a client state with an additional client
/// identifier field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedClientState {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// client state
    #[prost(message, optional, tag="2")]
    pub client_state: ::core::option::Option<::tendermint_proto::google::protobuf::Any>,
}
impl ::prost::Name for IdentifiedClientState {
const NAME: &'static str = "IdentifiedClientState";
const PACKAGE: &'static str = "ibc.core.client.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
            }}
/// ConsensusStateWithHeight defines a consensus state with an additional height
/// field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusStateWithHeight {
    /// consensus state height
    #[prost(message, optional, tag="1")]
    pub height: ::core::option::Option<Height>,
    /// consensus state
    #[prost(message, optional, tag="2")]
    pub consensus_state: ::core::option::Option<::tendermint_proto::google::protobuf::Any>,
}
impl ::prost::Name for ConsensusStateWithHeight {
const NAME: &'static str = "ConsensusStateWithHeight";
const PACKAGE: &'static str = "ibc.core.client.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
            }}
/// ClientConsensusStates defines all the stored consensus states for a given
/// client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientConsensusStates {
    /// client identifier
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    /// consensus states and their heights associated with the client
    #[prost(message, repeated, tag="2")]
    pub consensus_states: ::prost::alloc::vec::Vec<ConsensusStateWithHeight>,
}
impl ::prost::Name for ClientConsensusStates {
const NAME: &'static str = "ClientConsensusStates";
const PACKAGE: &'static str = "ibc.core.client.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
            }}
/// Height is a monotonically increasing data type
/// that can be compared against another Height for the purposes of updating and
/// freezing clients
///
/// Normally the RevisionHeight is incremented at each height while keeping
/// RevisionNumber the same. However some consensus algorithms may choose to
/// reset the height in certain conditions e.g. hard forks, state-machine
/// breaking changes In these cases, the RevisionNumber is incremented so that
/// height continues to be monitonically increasing even as the RevisionHeight
/// gets reset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Height {
    /// the revision that the client is currently on
    #[prost(uint64, tag="1")]
    pub revision_number: u64,
    /// the height within the given revision
    #[prost(uint64, tag="2")]
    pub revision_height: u64,
}
impl ::prost::Name for Height {
const NAME: &'static str = "Height";
const PACKAGE: &'static str = "ibc.core.client.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
            }}
/// Params defines the set of IBC light client parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// allowed_clients defines the list of allowed client state types which can be created
    /// and interacted with. If a client type is removed from the allowed clients list, usage
    /// of this client will be disabled until it is added again to the list.
    #[prost(string, repeated, tag="1")]
    pub allowed_clients: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for Params {
const NAME: &'static str = "Params";
const PACKAGE: &'static str = "ibc.core.client.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
            }}
/// ClientUpdateProposal is a legacy governance proposal. If it passes, the substitute
/// client's latest consensus state is copied over to the subject client. The proposal
/// handler may fail if the subject and the substitute do not match in client and
/// chain parameters (with exception to latest height, frozen height, and chain-id).
///
/// Deprecated: Please use MsgRecoverClient in favour of this message type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUpdateProposal {
    /// the title of the update proposal
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// the description of the proposal
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// the client identifier for the client to be updated if the proposal passes
    #[prost(string, tag="3")]
    pub subject_client_id: ::prost::alloc::string::String,
    /// the substitute client identifier for the client standing in for the subject
    /// client
    #[prost(string, tag="4")]
    pub substitute_client_id: ::prost::alloc::string::String,
}
impl ::prost::Name for ClientUpdateProposal {
const NAME: &'static str = "ClientUpdateProposal";
const PACKAGE: &'static str = "ibc.core.client.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
            }}
/// UpgradeProposal is a gov Content type for initiating an IBC breaking
/// upgrade.
///
/// Deprecated: Please use MsgIBCSoftwareUpgrade in favour of this message type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeProposal {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub plan: ::core::option::Option<super::super::super::super::cosmos::upgrade::v1beta1::Plan>,
    /// An UpgradedClientState must be provided to perform an IBC breaking upgrade.
    /// This will make the chain commit to the correct upgraded (self) client state
    /// before the upgrade occurs, so that connecting chains can verify that the
    /// new upgraded client is valid by verifying a proof on the previous version
    /// of the chain. This will allow IBC connections to persist smoothly across
    /// planned chain upgrades
    #[prost(message, optional, tag="4")]
    pub upgraded_client_state: ::core::option::Option<::tendermint_proto::google::protobuf::Any>,
}
impl ::prost::Name for UpgradeProposal {
const NAME: &'static str = "UpgradeProposal";
const PACKAGE: &'static str = "ibc.core.client.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("ibc.core.client.v1.{}", Self::NAME)
            }}
// @@protoc_insertion_point(module)
