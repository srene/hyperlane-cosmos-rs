// @generated
/// Module is the app config object of the module.
/// Learn more: <https://docs.cosmos.network/main/building-modules/depinject>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    #[prost(int32, repeated, tag="1")]
    pub enabled_tokens: ::prost::alloc::vec::Vec<i32>,
    /// authority defines the custom module authority.
    /// if not set, defaults to the governance module.
    #[prost(string, tag="2")]
    pub authority: ::prost::alloc::string::String,
}
impl ::prost::Name for Module {
const NAME: &'static str = "Module";
const PACKAGE: &'static str = "hyperlane.warp.module.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.warp.module.v1.{}", Self::NAME)
            }}
// @@protoc_insertion_point(module)
