use std::sync::Arc;
use std::thread;

use tokio::runtime::Runtime as TokioRuntime;

use prost_types::Any;
use ibc::signer::Signer;
use ibc::events::IbcEvent;
use tendermint_rpc::HttpClient;
use tonic::codegen::http::Uri;
use ibc_proto::cosmos::auth::v1beta1::BaseAccount;
use crate::error::Error;
use ibc::Height as ICSHeight;
use crate::light_client::LightClient;
use tendermint_light_client::types::LightBlock as TMLightBlock;
use ibc::ics07_tendermint::header::Header as TmHeader;
use ibc::ics07_tendermint::consensus_state::ConsensusState as TMConsensusState;
use ibc::ics07_tendermint::client_state::{AllowUpdate, ClientState};
use crate::{config::ChainConfig, event::monitor::EventReceiver};
use ibc::ics24_host::identifier::{ChainId, ChannelId, ClientId, ConnectionId, PortId};
use ibc_proto::ibc::core::client::v1::{QueryClientStatesRequest, QueryConsensusStatesRequest};
use ibc::ics02_client::client_state::{AnyClientState, IdentifiedAnyClientState};
use ibc_proto::ibc::core::commitment::v1::MerkleProof;
use crate::keyring::{KeyEntry, KeyRing};
use super::Chain;
use ibc::ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes};
use ibc::ics03_connection::version::{get_compatible_versions, Version};
use ibc::ics04_channel::packet::{PacketMsgType, Sequence};
use ibc::ics02_client::client_consensus::{
    AnyConsensusState, AnyConsensusStateWithHeight, ConsensusState,
};
use ibc_proto::ibc::core::connection::v1::{
    QueryClientConnectionsRequest, QueryConnectionsRequest,
};
use ibc_proto::ibc::core::channel::v1::{
    PacketState, QueryChannelClientStateRequest, QueryChannelsRequest,
    QueryConnectionChannelsRequest, QueryNextSequenceReceiveRequest,
    QueryPacketAcknowledgementsRequest, QueryPacketCommitmentsRequest, QueryUnreceivedAcksRequest,
    QueryUnreceivedPacketsRequest,
};
use ibc::ics03_connection::connection::{ConnectionEnd, IdentifiedConnectionEnd, State};
use ibc::ics04_channel::channel::{ChannelEnd, IdentifiedChannelEnd};
use ibc::query::QueryTxRequest;

pub struct SubstrateChain {
    config: ChainConfig,
    rpc_client: HttpClient,
    grpc_addr: Uri,
    rt: Arc<TokioRuntime>,
    keybase: KeyRing,

    /// A cached copy of the account information
    account: Option<BaseAccount>,
}

impl SubstrateChain {
    fn new() -> Self {
        unimplemented!()
    }
}


impl Chain for SubstrateChain {
    type LightBlock = TMLightBlock;
    type Header = TmHeader;
    type ConsensusState = TMConsensusState;
    type ClientState = ClientState;

    fn bootstrap(config: ChainConfig, rt: Arc<TokioRuntime>) -> Result<Self, Error> {
        unimplemented!()
    }

    fn init_light_client(&self) -> Result<Box<dyn LightClient<Self>>, Error> {
        unimplemented!()
    }

    fn init_event_monitor(
        &self,
        rt: Arc<TokioRuntime>,
    ) -> Result<(EventReceiver, Option<thread::JoinHandle<()>>), Error> {
        unimplemented!()
    }

    fn id(&self) -> &ChainId {
        unimplemented!()
    }

    fn keybase(&self) -> &KeyRing {
        unimplemented!()
    }

    fn keybase_mut(&mut self) -> &mut KeyRing {
        unimplemented!()
    }

    fn send_msgs(&mut self, proto_msgs: Vec<Any>) -> Result<Vec<IbcEvent>, Error> {
        unimplemented!()
    }

    fn get_signer(&mut self) -> Result<Signer, Error> {
        unimplemented!()
    }

    fn get_key(&mut self) -> Result<KeyEntry, Error> {
        unimplemented!()
    }

    // Queries

    fn query_commitment_prefix(&self) -> Result<CommitmentPrefix, Error> {
        unimplemented!()
    }

    fn query_compatible_versions(&self) -> Result<Vec<Version>, Error> {
        unimplemented!()
    }

    fn query_latest_height(&self) -> Result<ICSHeight, Error> {
        unimplemented!()
    }

    fn query_clients(
        &self,
        request: QueryClientStatesRequest,
    ) -> Result<Vec<IdentifiedAnyClientState>, Error> {
        unimplemented!()
    }

    fn query_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<Self::ClientState, Error> {
        unimplemented!()
    }

    fn query_consensus_states(
        &self,
        request: QueryConsensusStatesRequest,
    ) -> Result<Vec<AnyConsensusStateWithHeight>, Error> {
        unimplemented!()
    }

    fn query_consensus_state(
        &self,
        client_id: ClientId,
        consensus_height: ICSHeight,
        query_height: ICSHeight,
    ) -> Result<AnyConsensusState, Error> {
        unimplemented!()
    }

    fn query_upgraded_client_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        unimplemented!()
    }

    fn query_upgraded_consensus_state(
        &self,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        unimplemented!()
    }

    fn query_connections(
        &self,
        request: QueryConnectionsRequest,
    ) -> Result<Vec<IdentifiedConnectionEnd>, Error> {
        unimplemented!()
    }

    fn query_client_connections(
        &self,
        request: QueryClientConnectionsRequest,
    ) -> Result<Vec<ConnectionId>, Error> {
        unimplemented!()
    }

    fn query_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<ConnectionEnd, Error> {
        unimplemented!()
    }

    fn query_connection_channels(
        &self,
        request: QueryConnectionChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        unimplemented!()
    }

    fn query_channels(
        &self,
        request: QueryChannelsRequest,
    ) -> Result<Vec<IdentifiedChannelEnd>, Error> {
        unimplemented!()
    }

    fn query_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<ChannelEnd, Error> {
        unimplemented!()
    }

    fn query_channel_client_state(
        &self,
        request: QueryChannelClientStateRequest,
    ) -> Result<Option<IdentifiedAnyClientState>, Error> {
        unimplemented!()
    }

    fn query_packet_commitments(
        &self,
        request: QueryPacketCommitmentsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        unimplemented!()
    }

    fn query_unreceived_packets(
        &self,
        request: QueryUnreceivedPacketsRequest,
    ) -> Result<Vec<u64>, Error> {
        unimplemented!()
    }

    fn query_packet_acknowledgements(
        &self,
        request: QueryPacketAcknowledgementsRequest,
    ) -> Result<(Vec<PacketState>, ICSHeight), Error> {
        unimplemented!()
    }

    fn query_unreceived_acknowledgements(
        &self,
        request: QueryUnreceivedAcksRequest,
    ) -> Result<Vec<u64>, Error> {
        unimplemented!()
    }

    fn query_next_sequence_receive(
        &self,
        request: QueryNextSequenceReceiveRequest,
    ) -> Result<Sequence, Error> {
        unimplemented!()
    }

    fn query_txs(&self, request: QueryTxRequest) -> Result<Vec<IbcEvent>, Error> {
        unimplemented!()
    }

    // Provable queries
    fn proven_client_state(
        &self,
        client_id: &ClientId,
        height: ICSHeight,
    ) -> Result<(Self::ClientState, MerkleProof), Error> {
        unimplemented!()
    }

    fn proven_connection(
        &self,
        connection_id: &ConnectionId,
        height: ICSHeight,
    ) -> Result<(ConnectionEnd, MerkleProof), Error> {
        unimplemented!()
    }

    fn proven_client_consensus(
        &self,
        client_id: &ClientId,
        consensus_height: ICSHeight,
        height: ICSHeight,
    ) -> Result<(Self::ConsensusState, MerkleProof), Error> {
        unimplemented!()
    }

    fn proven_channel(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        height: ICSHeight,
    ) -> Result<(ChannelEnd, MerkleProof), Error> {
        unimplemented!()
    }

    fn proven_packet(
        &self,
        packet_type: PacketMsgType,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
        height: ICSHeight,
    ) -> Result<(Vec<u8>, MerkleProof), Error> {
        unimplemented!()
    }

    fn build_client_state(&self, height: ICSHeight) -> Result<Self::ClientState, Error> {
        unimplemented!()
    }

    fn build_consensus_state(
        &self,
        light_block: Self::LightBlock,
    ) -> Result<Self::ConsensusState, Error> {
        unimplemented!()
    }

    fn build_header(
        &self,
        trusted_height: ICSHeight,
        target_height: ICSHeight,
        client_state: &AnyClientState,
        light_client: &mut dyn LightClient<Self>,
    ) -> Result<(Self::Header, Vec<Self::Header>), Error> {
        unimplemented!()
    }
}
