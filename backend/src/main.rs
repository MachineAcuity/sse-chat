use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{sse::Event, Filter};

/// Port for serving
const PORT_SERVER: u16 = 5050;

/// If environment variable for CORS allowed origin is not provided, use default for frontend development (dev)
const CORS_ALLOW_ORIGIN_DEFAULT: &str = "http://localhost:5000";

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Message variants.
#[derive(Debug)]
enum Message {
    UserId(usize),
    Reply(String),
}

// Message sent from client
#[derive(Serialize, Deserialize, Debug)]
struct PostedMessage {
    pub message: String,
    pub user_id: usize,
    pub time: i64,
}

#[derive(Debug)]
struct NotUtf8;
impl warp::reject::Reject for NotUtf8 {}

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `Message`
type Users = Arc<Mutex<HashMap<String, HashMap<usize, mpsc::UnboundedSender<Message>>>>>;

fn user_connected(
    room_name: String,
    users: Users,
) -> impl Stream<Item = Result<Event, warp::Error>> + Send + 'static {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", my_id);

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the event source...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tx.send(Message::UserId(my_id))
        // rx is right above, so this cannot fail
        .unwrap();

    // Save the sender in our list of connected users.
    let mut rooms = users.lock().unwrap();
    let room = rooms.get_mut(&room_name);
    match room {
        Some(room) => {
            room.insert(my_id, tx);
        }
        None => {
            let mut new_room = HashMap::new();
            new_room.insert(my_id, tx);
            rooms.insert(room_name, new_room);
        }
    }

    // Convert messages into Server-Sent Events and return resulting stream.
    rx.map(|msg| match msg {
        Message::UserId(my_id) => Ok(Event::default().event("user").data(my_id.to_string())),
        Message::Reply(reply) => Ok(Event::default().data(reply)),
    })
}

fn user_message(room_name: String, posted_message: PostedMessage, users: &Users) {
    // New message from this user, send it to everyone else including the same user...
    //
    // We use `retain` instead of a for loop so that we can reap any user that
    // appears to have disconnected.
    let mut rooms = users.lock().unwrap();
    let room = rooms.get_mut(&room_name);
    match room {
        Some(room) => {
            room.retain(|uid, tx| {
                let is_marco = posted_message.message.eq_ignore_ascii_case("marco");
                if posted_message.user_id == *uid && is_marco {
                    // Send polo to the same user, only retain if not `is_ok`
                    let response = "{\"user_id\": 0, \"message\": \"Polo\", \"time\": 0 }"
                        .to_string()
                    ;

                    // If not `is_ok`, the SSE stream is gone, and so don't retain
                    tx.send(Message::Reply(response)).is_ok()
                } else  if posted_message.user_id != *uid && is_marco {
                    // don't send polo to other users, but do retain
                    true
                } else {
                    let response = match serde_json::to_string(&posted_message) {
                        Ok(str) => str,
                        Err(_) => {
                            "{\"user_id\": 0, \"message\": \"????????????????????????????????????\", \"time\": 0 }"
                                .to_string()
                        }
                    };

                    // If not `is_ok`, the SSE stream is gone, and so don't retain
                    tx.send(Message::Reply(response)).is_ok()
                }
            });
        }
        None => {
            println!("????????????????????????????????????")
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is an event stream sender.
    let users = Arc::new(Mutex::new(HashMap::new()));

    // Turn our "state" into a new Filter...
    let users = warp::any().map(move || users.clone());

    // Determine CORS allowed origin
    let allow_origin: String = match env::var("CORS_ALLOW_ORIGIN") {
        Ok(allowed_origin) => allowed_origin,
        Err(_) => CORS_ALLOW_ORIGIN_DEFAULT.to_string(),
    };
    println!("CORS allow origin: {}", allow_origin);

    // Set up CORS
    let cors = warp::cors()
        .allow_origin(&*allow_origin)
        .allow_methods(vec!["GET", "POST"])
        .allow_headers(vec!["content-type"]);

    // TODO Implement backend logging
    let log = warp::log("sse-chat");

    // POST /room/:name/send -> send message
    let chat_send = warp::path!("room" / String / "send")
        .and(warp::post())
        .and(warp::body::content_length_limit(5 * 1024))
        .and(warp::body::json())
        .and(users.clone())
        .map(|room_name, posted_message: PostedMessage, users| {
            user_message(room_name, posted_message, &users);
            warp::reply()
        })
        .with(&cors)
        .with(log);

    // GET /room/:name/listen -> messages stream
    let chat_listen = warp::path!("room" / String / "listen")
        .and(warp::get())
        .and(users)
        .map(|room_name, users| {
            // reply using server-sent events
            let stream = user_connected(room_name, users);
            warp::sse::reply(warp::sse::keep_alive().stream(stream))
        })
        .with(&cors)
        .with(log);

    // GET public static files and index
    const PUBLIC_DIR: &str = "./public";
    let public_files = warp::fs::dir(PUBLIC_DIR);
    // TODO: Enable index file but also provide a way to catch errors (this implementation does not)
    //let public_files_index = warp::fs::file(format!("{}/index.html", PUBLIC_DIR));

    // Combine all routes
    let routes = chat_listen.or(chat_send).or(public_files);
    // TODO: Enable index file but also provide a way to catch errors (this implementation does not)
    //.or(public_files_index)

    warp::serve(routes).run(([0, 0, 0, 0], PORT_SERVER)).await;
}
