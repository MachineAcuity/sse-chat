## Introduction

SSE chat using:

* Rust.
* Svelte.

Prerequisites:

* Tested on `Mac OS` only.
* `Node.js` 14 or higher.
* `Rust` 1.51 or higher.

## Installation

```
npm install
```

## Local Development

Start the backend

```
npm run dev-backend
```

Start the frontend

```
(cd frontend && npm run dev)
```

Access development server at `http://localhost:5000/`.

## Local Build

Build frontend, backend, and update backend with latest frontend files

```
npm run build-all
```

Start with

```
npm run start-all
```

Access build server at `http://localhost:5050/`.

## Docker Build

Set up [rust-musl-builder](https://github.com/emk/rust-musl-builder):

```
alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
```

Build static executable:

```
(cd backend && rust-musl-builder cargo build --release)
```

Create docker image:

```
docker build -t sse-chat:v1.0.1-0 .
```

## Local Docker Run

```
docker run --rm --name sse-chat-backend -p 5050:5050 --env CORS_ALLOW_ORIGIN=http://localhost:5050 sse-chat:v1.0.1-0
```

## Digital Ocean Kubernetes

Push local image to DO registry:

```
docker tag sse-chat:v1.0.1-0 registry.digitalocean.com/samwise-gamgee/sse-chat:v1.0.1-0
docker push registry.digitalocean.com/samwise-gamgee/sse-chat:v1.0.1-0
```

Apply deplyment and service to kubernetes:

```
kubectl apply -f dok8s.yaml
```

Verify deployment:

```
kubectl get pods --watch
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

* [DONE] Use ENV variables to set up CORS.
* TODO:  Investigate switching to type script.
* TODO:  Switch to Tailwind CSS.
* TODO:  Switch to SvelteKit (when mature enough).
* TODO:  Switch Dockerfile from alpine to scratch (maybe).
* TODO:  Implement container health service check.