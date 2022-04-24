// use hdk::prelude::*;

// mod recurrent_event;
// use recurrent_event::{create_recurrent_event, OccurenceRestriction, Period, RecurrentEvent};

// entry_defs![RecurrentEvent::entry_def()];

// #[hdk_extern]
// fn init(_: ()) -> ExternResult<InitCallbackResult> {
//     let test_event = create_recurrent_event(RecurrentEvent::new(
//         "title".to_string(),
//         agent_info(),
//         1650369600,
//         1650974400,
//         1650369600,
//         Period::Daily,
//         11,
//         1,
//         OccurenceRestriction::OnlyWeekdays,
//     ));

//     println!("test_event = {:?}", test_event);
//     Ok(InitCallbackResult::Pass)
// }
