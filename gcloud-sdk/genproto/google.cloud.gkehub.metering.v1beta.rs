/// **Metering**: Per-Membership Feature State.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipState {
    /// The time stamp of the most recent measurement of the number of vCPUs
    /// in the cluster.
    #[prost(message, optional, tag = "1")]
    pub last_measurement_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The vCPUs capacity in the cluster according to the most recent
    /// measurement (1/1000 precision).
    #[prost(float, tag = "3")]
    pub precise_last_measured_cluster_vcpu_capacity: f32,
}
