use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use transport::{MediaKind, MediaPacket, TrackId};

pub type ClusterTrackUuid = u64;
pub type ClusterPeerId = String;
pub type ClusterTrackName = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ClusterTrackStatus {
    #[serde(rename = "connecting")]
    Connecting,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "reconnecting")]
    Reconnecting,
    #[serde(rename = "disconnected")]
    Disconnected,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct ClusterTrackMeta {
    pub kind: MediaKind,
    pub scaling: String,
    pub layers: Vec<u32>,
    pub status: ClusterTrackStatus,
    pub active: bool,
    pub label: Option<String>,
}

pub enum ClusterEndpointError {
    InternalError,
    NotImplement,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ClusterRemoteTrackIncomingEvent {
    RequestKeyFrame,
    RequestLimitBitrate(u32),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ClusterLocalTrackIncomingEvent {
    MediaPacket(MediaPacket),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ClusterEndpointIncomingEvent {
    PeerTrackAdded(ClusterPeerId, ClusterTrackName, ClusterTrackMeta),
    PeerTrackUpdated(ClusterPeerId, ClusterTrackName, ClusterTrackMeta),
    PeerTrackRemoved(ClusterPeerId, ClusterTrackName),
    LocalTrackEvent(TrackId, ClusterLocalTrackIncomingEvent),
    RemoteTrackEvent(TrackId, ClusterRemoteTrackIncomingEvent),
}

#[derive(PartialEq, Eq, Debug)]
pub enum ClusterRemoteTrackOutgoingEvent {
    MediaPacket(MediaPacket),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClusterLocalTrackOutgoingEvent {
    RequestKeyFrame,
    Subscribe(ClusterPeerId, ClusterTrackName),
    Unsubscribe(ClusterPeerId, ClusterTrackName),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClusterEndpointOutgoingEvent {
    TrackAdded(TrackId, ClusterTrackName, ClusterTrackMeta),
    TrackRemoved(TrackId, ClusterTrackName),
    SubscribeRoom,
    UnsubscribeRoom,
    SubscribePeer(ClusterPeerId),
    UnsubscribePeer(ClusterPeerId),
    LocalTrackEvent(TrackId, ClusterLocalTrackOutgoingEvent),
    RemoteTrackEvent(TrackId, ClusterTrackUuid, ClusterRemoteTrackOutgoingEvent),
}

/// generate for other peer
pub fn generate_cluster_track_uuid(room_id: &str, peer_id: &str, track_name: &str) -> ClusterTrackUuid {
    let based = format!("{}-{}-{}", room_id, peer_id, track_name);
    let mut s = DefaultHasher::new();
    based.hash(&mut s);
    s.finish()
}

#[async_trait::async_trait]
pub trait ClusterEndpoint: Send + Sync {
    fn on_event(&mut self, event: ClusterEndpointOutgoingEvent) -> Result<(), ClusterEndpointError>;
    async fn recv(&mut self) -> Result<ClusterEndpointIncomingEvent, ClusterEndpointError>;
}

#[async_trait::async_trait]
pub trait Cluster<C>: Send + Sync
where
    C: ClusterEndpoint,
{
    fn build(&mut self, room_id: &str, peer_id: &str) -> C;
}