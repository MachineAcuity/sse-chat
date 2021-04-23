<script>
import {
    createEventDispatcher
} from "svelte";

const dispatch = createEventDispatcher();
let value = "";

const handleSubmit = () => {
    dispatch("message", {
        text: value
    });

    value = "";
};

function handleKeyPress(event) {
    var keyCode = event.code || event.key;
    if (keyCode == 'Enter') {
        event.preventDefault();
        handleSubmit();
    }
}
</script>

<div class="py-5">
    <form class="m-4 flex">
        <textarea
            bind:value
            on:keypress={handleKeyPress}
            class="w-full bg-gray-300 py-5 px-3 rounded-l-lg"
            type="text"
            placeholder="type your message here..."
            />
        <button on:click={handleSubmit} class="px-8 rounded-r-lg bg-yellow-400  text-gray-800 font-bold p-4 uppercase border-yellow-500 border-t border-b border-r">Send</button>
    </form>
</div>
