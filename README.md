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
* The chat window is implemented following the examples in [Chat Messages](https://tailwindcomponents.com/component/chat-messages).

## Improvements

* TODO:  Implement script for increment on build.
* TODO:  Investigate switching to type script.
* TODO:  Switch to SvelteKit (when mature enough).
* TODO:  Switch Dockerfile from alpine to scratch (maybe).
* TODO:  Implement container health service check.
* TODO:  Consider using env variable in yaml instead of modifying it - https://v1-18.docs.kubernetes.io/docs/tasks/inject-data-application/define-interdependent-environment-variables/