use lavalink_rs::model::events;
use lavalink_rs::prelude::*;

use crate::prelude::*;

pub async fn create_lavalink_client(user_id: UserId) -> Result<LavalinkClient> {
    let lavalink_password = match std::env::var("LAVALINK_PASSWORD") {
        Ok(token) => token,
        Err(e) => panic!("Failed to obtain LAVALINK_PASSWORD: {:?}", e),
    };

    let node_local = NodeBuilder {
        hostname: "lavalink:2333".to_string(),
        is_ssl: false,
        events: events::Events::default(),
        password: lavalink_password,
        user_id,
        session_id: None,
    };

    let custom_events = events::Events {
        track_end: Some(track_end),
        ..Default::default()
    };

    let lava_client = LavalinkClient::new(
        custom_events,
        vec![node_local],
        NodeDistributionStrategy::new(),
    )
    .await;

    Ok(lava_client)
}

#[lavalink_rs::hook]
async fn track_end(client: LavalinkClient, _session_id: String, event: &events::TrackEnd) {
    let player_context = client
        .get_player_context(event.guild_id)
        .expect("No PlayerContext found");

    let Ok(player_data) = player_context.data::<PlayerData>() else {
        eprintln!("Failed getting PlayerData");
        return;
    };

    let loop_state = *player_data.loop_state.lock().await;
    match loop_state {
        LoopState::Song => {
            let queue = player_context.get_queue();
            if let Err(e) = queue.push_to_front(event.track.clone()) {
                eprintln!("Error looping song track: {:?}", e);
            }
        }
        LoopState::Queue => {
            let queue = player_context.get_queue();
            if let Err(e) = queue.push_to_back(event.track.clone()) {
                eprintln!("Error looping queue track: {:?}", e);
            }
        }
        _ => {}
    }
}
