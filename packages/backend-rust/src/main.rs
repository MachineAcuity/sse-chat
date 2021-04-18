use futures::{Stream, StreamExt};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{sse::Event, Filter};
// hyper::body::Bytes

/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Message variants.
#[derive(Debug)]
enum Message {
    UserId(usize),
    Reply(String),
}

#[derive(Deserialize, Debug)]
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
    let new_msg = posted_message.message;
    let user_id = posted_message.user_id;

    // New message from this user, send it to everyone else (except same uid)...
    //
    // We use `retain` instead of a for loop so that we can reap any user that
    // appears to have disconnected.
    let mut rooms = users.lock().unwrap();
    let room = rooms.get_mut(&room_name);
    match room {
        Some(room) => {
            room.retain(|uid, tx| {
                if user_id == *uid {
                    // don't send to same user, but do retain
                    true
                } else {
                    // If not `is_ok`, the SSE stream is gone, and so don't retain
                    tx.send(Message::Reply(new_msg.clone())).is_ok()
                }
            });
        }
        None => {
            println!("なんか変だけど何もしない")
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

    // POST /room/:name/send -> send message
    let chat_send = warp::path!("room" / String / "send")
        .and(warp::post())
        .and(warp::body::content_length_limit(5 * 1024))
        .and(warp::body::json())
        .and(users.clone())
        .map(|room_name, posted_message: PostedMessage, users| {
            user_message(room_name, posted_message, &users);
            warp::reply()
        });

    // GET /room/:name/listen -> messages stream
    let chat_recv = warp::path!("room" / String / "listen")
        .and(warp::get())
        .and(users)
        .map(|room_name, users| {
            // reply using server-sent events
            let stream = user_connected(room_name, users);
            warp::sse::reply(warp::sse::keep_alive().stream(stream))
        });

    // GET /room/:name -> chat html
    let room = warp::path!("room" / String).map(|_| {
        warp::http::Response::builder()
            .header("content-type", "text/html; charset=utf-8")
            .body(CHAT_HTML)
    });

    // GET public static files and index
    const PUBLIC_DIR: &str = "./public";
    let public_files = warp::fs::dir(PUBLIC_DIR);
    let public_files_index = warp::fs::file(format!("{}/index.html", PUBLIC_DIR));

    // Combine all routes
    let routes = room
        .or(chat_recv)
        .or(chat_send)
        .or(public_files)
        .or(public_files_index);

    warp::serve(routes).run(([127, 0, 0, 1], 5050)).await;
}

static CHAT_HTML: &str = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        var uri = 'http://' + location.host + location.pathname;
        var sse = new EventSource(uri + '/listen');
        function message(data) {
            var line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }
        sse.onopen = function() {
            chat.innerHTML = "<p><em>Connected!</em></p>";
        }
        var user_id;
        sse.addEventListener("user", function(msg) {
            user_id = msg.data;
        });
        sse.onmessage = function(msg) {
            message(msg.data);
        };
        send.onclick = async e => {
            var msg = text.value;
            user_id = parseInt (user_id, 10);
            await fetch(uri + '/send', {
                body: JSON.stringify({
                    message: msg,
                    user_id,
                    time: Date.now()
                }),
                headers: {
                    "Content-Type": "application/json"
                },
                method: "POST"
            });
            text.value = '';
            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
"#;
