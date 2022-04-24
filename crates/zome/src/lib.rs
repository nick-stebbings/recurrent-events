//! ## hc_zome_recurrent_events
//!
//! RecurrentEvents zome for any Holochain app.
//!
//! If you need to manage recurrent_events
//! you can directly include this zome in your DNA.
//!
//! Read about how to include both this zome and its frontend module in your application [here](https://holochain-open-dev.github.io/recurrent_events).

use hdk::prelude::*;

mod handlers;
mod utils;

use hc_zome_recurrent_events_types::*;

entry_defs![PathEntry::entry_def(), RecurrentEvent::entry_def()];

/// Creates the recurrent_event for the agent executing this call.
#[hdk_extern]
pub fn create_recurrent_event(
    recurrent_event: RecurrentEvent,
) -> ExternResult<AgentRecurrentEvent> {
    handlers::create_recurrent_event(recurrent_event)
}

/// Updates the recurrent_event for the agent executing this call.
#[hdk_extern]
pub fn update_recurrent_event(
    recurrent_event: RecurrentEvent,
) -> ExternResult<AgentRecurrentEvent> {
    handlers::update_recurrent_event(recurrent_event)
}

/// Gets all the recurrent_events that have been created in the network.
///
/// Careful! This will not be very performant in large networks.
/// In the future a cursor type functionality will be added to make this function performant.
#[hdk_extern]
pub fn get_all_recurrent_events(_: ()) -> ExternResult<Vec<AgentRecurrentEvent>> {
    let agent_recurrent_events = handlers::get_all_recurrent_events()?;

    Ok(agent_recurrent_events)
}
