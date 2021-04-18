<script>
import Router from "svelte-spa-router";
import Home from "./routes/Home.svelte";
import Room from "./routes/Room.svelte";

const routes = {
    "/": Home,
    "/room/:room_id": Room
};
</script>

<Router {routes} />
