// use aper_stateroom::{AperStateroomService, StateMachineContainerProgram};
// use counter_common::Counter;
// use stateroom_wasm::prelude::stateroom_wasm;

// #[stateroom_wasm]
// type DropFourService = AperStateroomService<StateMachineContainerProgram<Counter>>;
use stateroom::{
    MessageRecipient, SimpleStateroomService, StateroomContext, StateroomServiceFactory,
    WrappedStateroomService, ClientId
};
use stateroom_wasm::stateroom_wasm;
use serde_json::{Result, Value};
use serde_json::json;

#[stateroom_wasm]
struct NodeEditorServer;

impl SimpleStateroomService for NodeEditorServer {
    fn new(_: &str,
           _: &impl StateroomContext) -> Self {
            NodeEditorServer
    }

    fn message(&mut self, _: ClientId,
               message: &str,
               ctx: &impl StateroomContext) {
        ctx.send_message( MessageRecipient::Broadcast, &format!("{}", message));
    }
}