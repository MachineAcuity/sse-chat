## Introduction

SSE chat with Rust and Svelte.

## Installation

```
npm install
```

## Development

Start the backend

```
npm run dev-backend
```

Start the frontend

```
(cd frontend && npm run dev)
```

Access development server at `http://localhost:5000/`.

## Build

Build frontend, backend, and update backend with latest frontend files

```
npm run build-all
```

Start with

```
npm run start-all
```

Access built server at `http://localhost:5050/`.

## Attribution

* The original repository is for the article [Building A Chat Application Using SvelteJS and SSE](https://marmelab.com/blog/2020/10/02/build-a-chat-application-using-sveltejs-and-sse.html) by [jdemangeon](https://github.com/jdemangeon) Julien Demangeon.
* The arrow icon is used from (https://www.flaticon.com/free-icon/right-arrow_724843).
* The rust backend is based on [madmaxio's sse_chat](https://github.com/madmaxio/tokio/blob/203ab8bd5e91daea728e9bf1f907de211c222f27/warp/examples/sse_chat.rs), [seanmonstar's sse_chat](https://github.com/seanmonstar/warp/blob/b6d1fc0719604ef1010aec00544408e6af1289a5/examples/sse_chat.rs) and [kouki-dan's Ratchat](https://github.com/kouki-dan/Ratchat/blob/1f4f6fc3a7227076d32906121d2eaedb03c76115/src/main.rs).

## Improvements

* TODO: Place next tasks here.