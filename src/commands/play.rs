use lavalink_rs::prelude::*;
// use lavalink_rs::model::track::TrackDiscordData;
// use poise::reply::CreateReply;
// use poise::serenity_prelude as serenity;
// use serenity::all::Message;
// use serenity::builder::{CreateActionRow, CreateButton, CreateEmbed};
// use serenity::futures::StreamExt;
// use serenity::prelude::Mutex;
// use songbird::input::{AuxMetadata, YoutubeDl as SongbirdDl};
// use songbird::Call;
// use std::borrow::Cow;
// use std::sync::Arc;
// use std::time::Duration;
// use youtube_dl::*;

use crate::utils::*;
use crate::*;
// use bot::HttpKey;

#[poise::command(slash_command, subcommands("url"))]
pub async fn play(ctx: Context<'_>) -> StdResult<()> {
    discord::send_message(&ctx, "Should require subcommand".to_string()).await;

    Ok(())
}

///Play via URL
#[poise::command(slash_command)]
pub async fn url(ctx: Context<'_>, #[description = "Enter a URL"] url: String) -> StdResult<()> {
    if !discord::has_perm(&ctx).await? {
        return Ok(());
    }

    if !url.starts_with("http") {
        discord::send_message(&ctx, "Ain't no url bruh").await;
        return Ok(());
    }

    match discord::get_player(&ctx) {
        Some(player_context) => add_to_queue(&ctx, &player_context, &url).await?,
        None => {
            ctx.defer().await?;
            let new_player_context = discord::join(&ctx).await?;
            add_to_queue(&ctx, &new_player_context, &url).await?;
        }
    };

    Ok(())
}

async fn add_to_queue(ctx: &Context<'_>, player_context: &PlayerContext, song: &str) -> Result<()> {
    let lava_client = &ctx.data().lavalink;

    let loaded_tracks = lava_client
        .load_tracks(
            ctx.guild_id().expect("No guild_id found for 'play url'"),
            song,
        )
        .await?;

    let tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(track)) => vec![track.into()],
        Some(TrackLoadData::Playlist(playlist)) => {
            let mut playlist_tracks: Vec<TrackInQueue> = Vec::new();
            for i in 0..=9 {
                match playlist.tracks.get(i) {
                    Some(track) => playlist_tracks.push(track.clone().into()),
                    None => continue,
                }
            }

            playlist_tracks

            // .tracks
            // .iter()
            // .map(|track| track.clone().into())
            // .collect(),
        }
        None => {
            eprintln!("No data found in Track");
            return Ok(());
        }
        _ => {
            println!("Yet to be implemented");
            return Ok(());
        }
    };

    let message = tracks
        .iter()
        .map(|track| {
            let track_data = &track.track;
            match &track_data.info.uri {
                Some(uri) => {
                    format!(
                        "Added to queue: [{} - {}](<{}>)",
                        track_data.info.author, track_data.info.title, uri
                    )
                }
                None => {
                    format!(
                        "Added to queue: [{} - {}]",
                        track_data.info.author, track_data.info.title
                    )
                }
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    let queue = player_context.get_queue();
    queue.append(tracks.into())?;

    discord::send_message(ctx, message).await;

    if let Ok(player) = player_context.get_player().await {
        if player.track.is_none() && queue.get_count().await? > 0 as usize {
            player_context.skip()?;
        }
    }

    Ok(())
}

// TODO: No need for 'search' function due to 'songbird' update and new 'search' function
// Replace it with new function soon

// Search a title (WIP)
// #[poise::command(slash_command)]
// pub async fn search(
//     ctx: Context<'_>,
//     #[description = "Enter a title"] title: String,
// ) -> StdResult<()> {
//     // if let Some(handler) = commands::handler_exist(ctx).await {
//     //     // check_result(search_up(&ctx, title, handler).await);
//     //     if let Err(e) = search_up(&ctx, title, handler).await {
//     //         panic!("Error queuing search, existing handler: {:?}", e);
//     //     }
//     // } else {
//     //     let new_handler: Arc<Mutex<Call>> = commands::join_channel(ctx).await?;
//     //     // check_result(search_up(&ctx, title, new_handler).await);
//     //     if let Err(e) = search_up(&ctx, title, new_handler).await {
//     //         panic!("Error queuing search, new created handler: {:?}", e);
//     //     }
//     // }

//     Ok(())
// }

// async fn search_up(ctx: &Context<'_>, title: String, handler: Arc<Mutex<Call>>) -> StdResult<()> {
//     // check_result(ctx.defer().await);
//     if let Err(e) = ctx.defer().await {
//         panic!("Error deferring play search command: {:?}", e);
//     }

//     let http_client = {
//         let data = ctx.serenity_context().data.read().await;
//         data.get::<HttpKey>()
//             .cloned()
//             .expect("Guaranteed to exist in the typemap.")
//     };

//     // let search_result = YoutubeDl::search_for(&SearchOptions::youtube(title).with_count(5))
//     //    .socket_timeout("20")
//     //    .extract_audio(true)
//     //    .run_async()
//     //    .await?;

//     let search_result: Vec<AuxMetadata> = match SongbirdDl::new(http_client.clone(), title)
//         .search(Some(5))
//         .await
//     {
//         Ok(search) => search,
//         Err(e) => panic!("Error searching: {:?}", e),
//     };

//     if let Err(e) = search_init(ctx, &search_result, handler).await {
//         panic!("Error beginning serach: {:?}", e);
//     }

//     Ok(())
// }

// async fn search_init(
//     ctx: &Context<'_>,
//     search: &Vec<AuxMetadata>,
//     handler: Arc<Mutex<Call>>,
// ) -> StdResult<()> {
//     let mut index = 0;
//     // let reply = check_result(ctx.send(search_msg(search, index).unwrap()).await);
//     let message_reply = match search_msg(search, index) {
//         Ok(message) => message,
//         Err(e) => panic!("Error creating search message: {:?}", e),
//     };

//     let reply = match ctx.send(message_reply).await {
//         Ok(reply) => reply,
//         Err(e) => panic!("Error creating search message: {:?}", e),
//     };

//     let msg: Cow<Message> = reply.message().await?;
//     let mut interaction_stream = msg
//         .clone()
//         .await_component_interaction(&ctx.serenity_context().shard)
//         .timeout(Duration::from_secs(60))
//         .stream();
//     while let Some(interaction) = interaction_stream.next().await {
//         let custom_id = interaction.data.custom_id.as_str();
//         match custom_id {
//             "up" => {
//                 check_result(ctx.defer().await);

//                 if index > 0 {
//                     index -= 1;
//                 } else {
//                     index = 4;
//                 }

//                 check_result(
//                     interaction
//                         .create_response(
//                             &ctx,
//                             serenity::CreateInteractionResponse::UpdateMessage(
//                                 search_msg(search, index)
//                                     .unwrap()
//                                     .to_slash_initial_response(
//                                         serenity::CreateInteractionResponseMessage::new(),
//                                     ),
//                             ),
//                         )
//                         .await,
//                 );
//             }
//             "down" => {
//                 check_result(ctx.defer().await);

//                 if index < 4 {
//                     index += 1;
//                 } else {
//                     index = 0;
//                 }

//                 check_result(
//                     interaction
//                         .create_response(
//                             &ctx,
//                             serenity::CreateInteractionResponse::UpdateMessage(
//                                 search_msg(search, index)
//                                     .unwrap()
//                                     .to_slash_initial_response(
//                                         serenity::CreateInteractionResponseMessage::new(),
//                                     ),
//                             ),
//                         )
//                         .await,
//                 );
//             }
//             "select" => {
//                 check_result(ctx.defer().await);

//                 let video: AuxMetadata = search
//                     .get(index as usize)
//                     .expect("No video found in search")
//                     .to_owned();
//                 let http_client = {
//                     let data = ctx.serenity_context().data.read().await;
//                     data.get::<HttpKey>()
//                         .cloned()
//                         .expect("Guaranteed to exist in the typemap.")
//                 };
//                 let src =
//                     SongbirdDl::new(http_client.clone(), video.source_url.expect("No url found"));
//                 let mut handler_lock = handler.lock().await;
//                 handler_lock.enqueue_input(src.into()).await;

//                 let video_respone = format!(
//                     "**Successfully added track:** {}",
//                     video.title.expect("No title for video")
//                 );
//                 check_result(ctx.say(video_respone).await);

//                 return Ok(());
//             }
//             _ => println!("Unknown custom_id"),
//         }
//     }

//     Ok(())
// }

// pub fn search_msg(search: &Vec<AuxMetadata>, index: u8) -> StdResult<CreateReply> {
//     // let mut index_list = String::new();
//     let mut song_list = String::new();
//     for (k, v) in search.iter().enumerate() {
//         match k {
//             0 => {
//                 if 0 == index {
//                     song_list.push_str(
//                         format!("__**{}**__", v.title.clone().expect("No title found")).as_str(),
//                     );
//                 } else {
//                     song_list
//                         .push_str(format!("{}", v.title.clone().expect("No title found")).as_str());
//                 }
//             }
//             _ => {
//                 if k == index as usize {
//                     song_list.push_str(
//                         format!("\n\n__**{}**__", v.title.clone().expect("No title found"))
//                             .as_str(),
//                     );
//                 } else {
//                     song_list.push_str(
//                         format!("\n\n{}", v.title.clone().expect("No title found")).as_str(),
//                     );
//                 }
//             }
//         }
//     }

//     let thumbnail_string: &String = search
//         .get(index as usize)
//         .expect("No video found in search")
//         .thumbnail
//         .as_ref()
//         .expect("No thumbnail found");
//     let embed: CreateEmbed = serenity::CreateEmbed::new()
//         .title("Search result")
//         .color((255, 0, 0))
//         .field("Found tracks:", song_list, false)
//         .thumbnail(thumbnail_string);
//     let components: CreateActionRow = serenity::CreateActionRow::Buttons(vec![
//         CreateButton::new("up")
//             .emoji("⬆️".chars().next().unwrap())
//             .style(serenity::ButtonStyle::Primary),
//         CreateButton::new("down")
//             .emoji("⬇️".chars().next().unwrap())
//             .style(serenity::ButtonStyle::Primary),
//         CreateButton::new("select")
//             .emoji("🎵".chars().next().unwrap())
//             .style(serenity::ButtonStyle::Success),
//     ]);

//     Ok(CreateReply::default()
//         .embed(embed)
//         .components(vec![components]))
// }

// async fn queue_up(ctx: &Context<'_>, url: &String, handler: &Arc<Mutex<Call>>) -> StdResult<()> {
//     check_result(ctx.defer().await);

//     let http_client = {
//         let data = ctx.serenity_context().data.read().await;
//         data.get::<HttpKey>()
//             .cloned()
//             .expect("Guaranteed to exist in the typemap.")
//     };
//     let test = YoutubeDl::new(url)
//         .socket_timeout("10")
//         .extract_audio(true)
//         .run_async()
//         .await?;

//     let mut handler_lock = handler.lock().await;
//     match test {
//         YoutubeDlOutput::SingleVideo(video) => {
//             let src = SongbirdDl::new(http_client.clone(), video.url.expect("No url found"));
//             handler_lock.enqueue_input(src.into()).await;

//             let video_respone = format!(
//                 "**Successfully added track:** {}",
//                 video.title.expect("No title for video")
//             );
//             check_result(ctx.say(video_respone).await);
//         }
//         YoutubeDlOutput::Playlist(playlist) => {
//             let videos = playlist.entries.expect("Failed to get videos of playlist");
//             if videos.len() >= 10 {
//                 check_result(ctx.say("Sorry, don't take playlists with 10 videos or more\n(This is experimental)").await);

//                 return Ok(());
//             }

//             let mut video_list = String::new();
//             for video in videos {
//                 video_list
//                     .push_str(format!("\n{}", video.title.expect("No title for video")).as_str());
//                 let src = SongbirdDl::new(http_client.clone(), video.url.expect("No url found"));
//                 handler_lock.enqueue_input(src.into()).await;
//             }

//             let playlist_respone = format!(
//                 "**Successfully added playlist:** {}\n**__Added tracks :__** {}",
//                 playlist.title.expect("No title for playlist"),
//                 video_list
//             );
//             check_result(ctx.say(playlist_respone).await);
//         }
//     }

//     Ok(())
// }
