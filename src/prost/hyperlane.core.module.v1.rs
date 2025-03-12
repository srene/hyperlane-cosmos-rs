// @generated
/// Module is the app config object of the module.
/// Learn more: <https://docs.cosmos.network/main/building-modules/depinject>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// authority defines the custom module authority.
    /// if not set, defaults to the governance module.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
}
impl ::prost::Name for Module {
const NAME: &'static str = "Module";
const PACKAGE: &'static str = "hyperlane.core.module.v1";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("hyperlane.core.module.v1.{}", Self::NAME)
            }}
// @@protoc_insertion_point(module)
