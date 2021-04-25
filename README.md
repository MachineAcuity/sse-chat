## Introduction

SSE chat using:

* Rust.
* Svelte.

Prerequisites:

* Tested on `Mac OS` only.
* `Node.js` 14 or higher.
* `Rust` 1.51 or higher.
* Set up [rust-musl-builder](https://github.com/emk/rust-musl-builder).

## Installation

```
npm install
```

## Local Development

```
npm run ::dev-all
```
Access development server at `http://localhost:5000/`.

## Local Build

Build frontend, backend, and update backend with latest frontend files

```
npm run ::build-local
```

Start with

```
npm run ::start-local
```

Access local server at `http://localhost:5050/`.

## Docker Build

```
npm run ::build-docker-all
```

## Docker Run

```
npm run ::start-docker
```

## Digital Ocean Kubernetes

Push local image to DO registry:

```
npm run ::k8s-tag-push-apply
```

## Attribution

* The Svelte frontend has been inspired by [Building A Chat Application Using SvelteJS and SSE](https://marmelab.com/blog/2020/10/02/build-a-chat-application-using-sveltejs-and-sse.html) by [jdemangeon](https://github.com/jdemangeon) Julien Demangeon.
* The arrow icon asset is obtained from (https://www.flaticon.com/free-icon/right-arrow_724843).
* The Rust backend has been inspired by [madmaxio's sse_chat](https://github.com/madmaxio/tokio/blob/203ab8bd5e91daea728e9bf1f907de211c222f27/warp/examples/sse_chat.rs), [seanmonstar's sse_chat](https://github.com/seanmonstar/warp/blob/b6d1fc0719604ef1010aec00544408e6af1289a5/examples/sse_chat.rs) and [kouki-dan's Ratchat](https://github.com/kouki-dan/Ratchat/blob/1f4f6fc3a7227076d32906121d2eaedb03c76115/src/main.rs).
* The backend CORS code based on [How to compose warp log](https://stackoverflow.com/questions/62107101/how-to-compose-warp-log).
* Docker container is built following the steps in [Building Minimal Docker Containers for Rust Applications](https://blog.semicolonsoftware.de/building-minimal-docker-containers-for-rust-applications/).
* Tailwind CSS configuration is done following [How To Use Svelte JS with Tailwind CSS](https://levelup.gitconnected.com/how-to-use-svelte-js-with-tailwind-css-f0554187eca1).
* The chat window is implemented following the examples in [Chat Messages](https://tailwindcomponents.com/component/chat-messages) and [Comment section](https://tailwindcomponents.com/component/comment-section).
* Photos used from https://unsplash.com/photos/PPfo56sLIB0 , https://unsplash.com/photos/40XgDxBfYXM , https://unsplash.com/photos/UwiAOiJ4OHI .

## Improvements

* [done] Reorganize names of chat components.
* [done] User 0 is message from server directly, not another user. Everybody else is a user. Create three separate icons for user, other users, and server.
* [done] Reorganize chat so that messages from server (and other chatters) are on the left (even IDs), and messages sent from client are on the right (odd IDs).
* [done] Modify server to send message to sender too (returned) this way it will display in their list of messages (and it has their user id so it will be displayed on the right).
* [done] Switch to format with separate field for date, probably something like this (above message, not isnide): https://tailwindcomponents.com/component/comment-section

* Update so that if a non-server user is sending a message, the name is displayed as on https://tailwindcomponents.com/component/comment-section , probably display name inside bubble on a separate line, like inside the message in the example.
* Reorganize chat so that date appears only if it is a different date, or the date changes from previous message. Make so that time is shown only when date is shown, or when more than two minutes passed since previous message.
* Attempt upgrading rollup-plugin-svelte from 6.1.1 to 7.1.0.
* Implement functionality so that when user says marco, the message is not transmitted but rather server replies only to said user with polo.
* Test container health service check (liveness probe). The test is successful is the container is not killed since at this point we expect to be always alive at least when without traffic.
* Implement script for increment on build.
* On iOS when viewed in both Chrome and Safari, the available screen height is smaller than 100vh, and it causes scrolling. Would be nice to fix somehow.
* Consider using env variable in yaml instead of modifying it - https://v1-18.docs.kubernetes.io/docs/tasks/inject-data-application/define-interdependent-environment-variables/
* Consider switching container from alpine to scratch. While it is definitely a plus having a smaller contaioner, having some basic bash for troubleshooting with alpine could be an advantage. If implemented, explain in readme.

* Include a list of messages not sent at bottom, and ability to re-send (or clear).

* Switch to SvelteKit (when mature enough).
    * Investigate switching to type script - https://svelte.dev/blog/svelte-and-typescript . According to https://codechips.me/how-to-use-typescript-with-svelte/ the dev experience is poor, and SvelteKit (by feature of using Svite) would prove to be a better time to make the transition
