use crate::{utils, AgentRecurrentEvent, RecurrentEvent};
use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;
use std::convert::TryInto;

pub fn create_recurrent_event(
    recurrent_event: RecurrentEvent,
) -> ExternResult<AgentRecurrentEvent> {
    let agent_info = agent_info()?;

    create_entry(&recurrent_event.clone())?;

    let recurrent_event_hash = hash_entry(&recurrent_event.clone())?;

    let path = prefix_path(recurrent_event.title.clone());

    path.ensure()?;

    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.clone().into();

    create_link(
        path.path_entry_hash()?,
        recurrent_event_hash.clone(),
        LinkType(0),
        link_tag(recurrent_event.title.as_str().clone())?,
    )?;
    create_link(
        agent_address.into(),
        recurrent_event_hash.clone(),
        LinkType(0),
        link_tag("recurrent_event")?,
    )?;

    let agent_recurrent_event = AgentRecurrentEvent {
        agent_pub_key: AgentPubKeyB64::from(agent_info.agent_initial_pubkey),
        recurrent_event,
    };

    Ok(agent_recurrent_event)
}

pub fn update_recurrent_event(
    recurrent_event: RecurrentEvent,
) -> ExternResult<AgentRecurrentEvent> {
    let agent_info = agent_info()?;

    create_entry(&recurrent_event.clone())?;

    let recurrent_event_hash = hash_entry(&recurrent_event.clone())?;

    let path = prefix_path(recurrent_event.title.clone());

    path.ensure()?;

    let agent_address = agent_info.agent_initial_pubkey.clone();

    let link_details = get_link_details(path.path_entry_hash()?, None)?.into_inner();

    if link_details.len() > 0 {
        // check whether the agent has committed a recurrent_event before
        // needs to be checked because duplicate RecurrentEvent is possible
        let recurrent_event_exist = link_details
            .clone()
            .into_iter()
            .find(|detail| detail.0.header().author().to_owned() == agent_address)
            .is_some();
        if recurrent_event_exist {
            link_details
                .clone()
                .into_iter()
                .filter_map(|detail| {
                    let is_my_recurrent_event =
                        detail.0.header().author().to_owned() == agent_address;
                    let is_not_deleted = detail.1.is_empty();
                    if is_my_recurrent_event && is_not_deleted {
                        return Some(detail.0.as_hash().to_owned());
                    } else {
                        return None;
                    }
                })
                .for_each(|header| {
                    // ignore error
                    match delete_link(header) {
                        Ok(_) => (),
                        // TODO: probably should return error once one of the delete fails
                        Err(_) => (),
                    }
                });
        }
    }

    let links = get_links(
        agent_address.clone().into(),
        Some(link_tag("recurrent_event")?),
    )?;
    if links.len() > 0 {
        let link = links[0].clone();
        delete_link(link.create_link_hash)?;
    }

    create_link(
        path.path_entry_hash()?,
        recurrent_event_hash.clone(),
        LinkType(0),
        link_tag(recurrent_event.title.as_str().clone())?,
    )?;
    create_link(
        agent_address.into(),
        recurrent_event_hash.clone(),
        LinkType(0),
        link_tag("recurrent_event")?,
    )?;

    let agent_recurrent_event = AgentRecurrentEvent {
        agent_pub_key: AgentPubKeyB64::from(agent_info.agent_initial_pubkey),
        recurrent_event,
    };

    Ok(agent_recurrent_event)
}

pub fn get_all_recurrent_events() -> ExternResult<Vec<AgentRecurrentEvent>> {
    let path = Path::from("all_recurrent_events");

    let children = path.children()?;

    let agent_recurrent_events: Vec<AgentRecurrentEvent> = children
        .into_iter()
        .map(|link| get_agent_recurrent_events_for_path(link.target))
        .collect::<ExternResult<Vec<Vec<AgentRecurrentEvent>>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(agent_recurrent_events)
}

/** Private helpers */

fn prefix_path(title: String) -> Path {
    // conver to lowercase for path for ease of search
    let lower_title = title.to_lowercase();
    let (prefix, _) = lower_title.as_str().split_at(3);

    Path::from(format!("all_recurrent_events.{}", prefix))
}

fn get_agent_recurrent_events_for_path(
    path_hash: EntryHash,
) -> ExternResult<Vec<AgentRecurrentEvent>> {
    let links = get_links(path_hash, None)?;

    let get_input = links
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), GetOptions::default()))
        .collect();

    let get_output = HDK.with(|h| h.borrow().get(get_input))?;

    get_output
        .into_iter()
        .filter_map(|maybe_option| maybe_option)
        .map(get_agent_recurrent_event_from_element)
        .collect()
}

fn get_agent_recurrent_event_from_element(element: Element) -> ExternResult<AgentRecurrentEvent> {
    let author = element.header().author().clone();

    let recurrent_event: RecurrentEvent = utils::try_from_element(element)?;

    let agent_recurrent_event = AgentRecurrentEvent {
        agent_pub_key: AgentPubKeyB64::from(author),
        recurrent_event,
    };

    Ok(agent_recurrent_event)
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);
pub fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let sb: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(sb.bytes().clone()))
}
