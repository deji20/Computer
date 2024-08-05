// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod speech;
mod brain;
mod abilities;
mod config;
mod actions;
mod senses;
use abilities::{movies::{self, MovieArgs, MovieResponse}, Ability};
use actions::Action;


use brain::Brain;
use senses::listen::Listener;
use speech::Speech;
use tauri::{async_runtime::channel, Manager, Window};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![speak])
    .setup(|app| {
      let speech = Speech::default();
      app.manage(speech);
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
async fn speak(command: String, window: Window) -> Result<String, String> {
  //parse audio file for command

  let (tx, mut rx) = channel::<String>(100);

  let brain = Brain::new(Some(tx.clone()));
  println!("{}", command);
    //receive thoughts from the brain while awaiting the decision
  let thoughts = async {
    loop {
      if let Some(thought) = rx.recv().await {
        window.emit("thought", thought).unwrap();
      }else {
        break;
      }
    };
  };

  window.emit("loading", true).unwrap();
  let decision = async {
    window.emit("speak", "Deciding...".to_string()).unwrap();
    let ability = brain.decision(&command).await?;
    window.emit("speak", format!("Decided on the ability: {}", ability)).unwrap();
    match ability {
      Ability::SearchMovies(_) => {
        ability.run(|args: Option<MovieArgs>| async {
          let Some(args) = args else { return Err("No Arguments".to_string()) }; 
          let config = crate::config::Config::get_config();
          window.emit("speak", format!("Searching for movies with the query: {}", args.movie)).unwrap();
          let url = format!("https://api.themoviedb.org/3/search/movie?api_key={}&query={}&year={}", config.media.tmdb_key, args.movie, args.year.unwrap_or(0));
          let client = reqwest::Client::new();
          let movies = &client.get(&url).send()
            .await.map_err(|e| format!("Error: {}", e))?
            .text().await.map_err(|e| format!("Error: {}", e))?;
          window.emit("movieList", movies).unwrap();
          Ok("Movies".to_string())
        }).await
      },
      _ => {
        Err("Ability not implemented".to_string())
      }
    }
  };
  let (_thought, decision) = tokio::join!(thoughts, decision);
  window.emit("loading", false).unwrap();
  match decision {
    Ok(result) => {
      Ok(result)
    },
    Err(e) => {
      window.emit("error", format!("Error: {}", e)).unwrap();
      Err(e)
    }
  }
  // if let Ok(mut stream) = speech.respond_stream(command.to_string()).await {
  //   while let Some(Ok(res)) = stream.next().await {
  //     for resp in res {
  //         if let Ok(_) = window.emit_all("speech", resp.response) {}
  //     }
  //   }
  //   Ok(format!("Hello, {}!", "world".to_string()))
  // }else {
  //   Err("Error".to_string())
  // }
}



#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}