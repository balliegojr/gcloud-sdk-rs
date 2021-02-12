/// A SourceContext is a reference to a tree of files. A SourceContext together
/// with a path point to a unique revision of a single file or directory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceContext {
    /// Labels with user defined metadata.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// A SourceContext can refer any one of the following types of repositories.
    #[prost(oneof = "source_context::Context", tags = "1, 2, 3")]
    pub context: ::core::option::Option<source_context::Context>,
}
/// Nested message and enum types in `SourceContext`.
pub mod source_context {
    /// A SourceContext can refer any one of the following types of repositories.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Context {
        /// A SourceContext referring to a revision in a Google Cloud Source Repo.
        #[prost(message, tag = "1")]
        CloudRepo(super::CloudRepoSourceContext),
        /// A SourceContext referring to a Gerrit project.
        #[prost(message, tag = "2")]
        Gerrit(super::GerritSourceContext),
        /// A SourceContext referring to any third party Git repo (e.g., GitHub).
        #[prost(message, tag = "3")]
        Git(super::GitSourceContext),
    }
}
/// An alias to a repo revision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliasContext {
    /// The alias kind.
    #[prost(enumeration = "alias_context::Kind", tag = "1")]
    pub kind: i32,
    /// The alias name.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AliasContext`.
pub mod alias_context {
    /// The type of an alias.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Unknown.
        Unspecified = 0,
        /// Git tag.
        Fixed = 1,
        /// Git branch.
        Movable = 2,
        /// Used to specify non-standard aliases. For example, if a Git repo has a
        /// ref named "refs/foo/bar".
        Other = 4,
    }
}
/// A CloudRepoSourceContext denotes a particular revision in a Google Cloud
/// Source Repo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRepoSourceContext {
    /// The ID of the repo.
    #[prost(message, optional, tag = "1")]
    pub repo_id: ::core::option::Option<RepoId>,
    /// A revision in a Cloud Repo can be identified by either its revision ID or
    /// its alias.
    #[prost(oneof = "cloud_repo_source_context::Revision", tags = "2, 3")]
    pub revision: ::core::option::Option<cloud_repo_source_context::Revision>,
}
/// Nested message and enum types in `CloudRepoSourceContext`.
pub mod cloud_repo_source_context {
    /// A revision in a Cloud Repo can be identified by either its revision ID or
    /// its alias.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Revision {
        /// A revision ID.
        #[prost(string, tag = "2")]
        RevisionId(::prost::alloc::string::String),
        /// An alias, which may be a branch or tag.
        #[prost(message, tag = "3")]
        AliasContext(super::AliasContext),
    }
}
/// A SourceContext referring to a Gerrit project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GerritSourceContext {
    /// The URI of a running Gerrit instance.
    #[prost(string, tag = "1")]
    pub host_uri: ::prost::alloc::string::String,
    /// The full project name within the host. Projects may be nested, so
    /// "project/subproject" is a valid project name. The "repo name" is the
    /// hostURI/project.
    #[prost(string, tag = "2")]
    pub gerrit_project: ::prost::alloc::string::String,
    /// A revision in a Gerrit project can be identified by either its revision ID
    /// or its alias.
    #[prost(oneof = "gerrit_source_context::Revision", tags = "3, 4")]
    pub revision: ::core::option::Option<gerrit_source_context::Revision>,
}
/// Nested message and enum types in `GerritSourceContext`.
pub mod gerrit_source_context {
    /// A revision in a Gerrit project can be identified by either its revision ID
    /// or its alias.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Revision {
        /// A revision (commit) ID.
        #[prost(string, tag = "3")]
        RevisionId(::prost::alloc::string::String),
        /// An alias, which may be a branch or tag.
        #[prost(message, tag = "4")]
        AliasContext(super::AliasContext),
    }
}
/// A GitSourceContext denotes a particular revision in a third party Git
/// repository (e.g., GitHub).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitSourceContext {
    /// Git repository URL.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// Git commit hash.
    #[prost(string, tag = "2")]
    pub revision_id: ::prost::alloc::string::String,
}
/// A unique identifier for a Cloud Repo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepoId {
    /// A cloud repo can be identified by either its project ID and repository name
    /// combination, or its globally unique identifier.
    #[prost(oneof = "repo_id::Id", tags = "1, 2")]
    pub id: ::core::option::Option<repo_id::Id>,
}
/// Nested message and enum types in `RepoId`.
pub mod repo_id {
    /// A cloud repo can be identified by either its project ID and repository name
    /// combination, or its globally unique identifier.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        /// A combination of a project ID and a repo name.
        #[prost(message, tag = "1")]
        ProjectRepoId(super::ProjectRepoId),
        /// A server-assigned, globally unique identifier.
        #[prost(string, tag = "2")]
        Uid(::prost::alloc::string::String),
    }
}
/// Selects a repo using a Google Cloud Platform project ID (e.g.,
/// winged-cargo-31) and a repo name within that project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectRepoId {
    /// The ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// The name of the repo. Leave empty for the default repo.
    #[prost(string, tag = "2")]
    pub repo_name: ::prost::alloc::string::String,
}
