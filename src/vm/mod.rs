pub struct Vm {
    /// Maintains the Vm-specific states.
    pub state: Arc<RwLock<VmState>>,
    pub app_sender: Option<Box<dyn subnet::rpc::common::appsender::AppSender + Send + Sync>>,

    /// A queue of data that have not been put into a block and proposed yet.
    /// Mempool is not persistent, so just keep in memory via Vm.
    pub mempool: Arc<RwLock<VecDeque<Vec<u8>>>>,
}

/// Represents VM-specific states.
/// Defined in a separate struct, for interior mutability in [`Vm`](Vm).
/// To be protected with `Arc` and `RwLock`.
pub struct VmState {
    pub ctx: Option<subnet::rpc::context::Context>,
    pub version: Version,
    pub genesis: Genesis,

    /// Represents persistent Vm state.
    pub state: Option<state::State>,
    /// Currently preferred block Id.
    pub preferred: ids::Id,
    /// Channel to send messages to the snowman consensus engine.
    pub to_engine: Option<Sender<subnet::rpc::common::message::Message>>,
    /// Set "true" to indicate that the Vm has finished bootstrapping
    /// for the chain.
    pub bootstrapped: bool,
}