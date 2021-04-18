<script>
import {
    createEventDispatcher
} from "svelte";

import Svg from './right-arrow.svg'

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
        handleSubmit();
    }
}
</script>

<style>
.root {
    padding: 30px;
}

.root img {
    width: 66px;
    height: 66px;
    padding: 20px;
}

textarea {
    width: calc(100% - 120px);
    border: none;
    padding: 10px 20px;
    font: 14px/22px "Lato", Arial, sans-serif;
    margin-bottom: 10px;
    border-radius: 5px;
    resize: none;
}
</style>

<div class="root">
    <textarea bind:value placeholder="Type your message" rows="3"  on:keypress={handleKeyPress}/>
        <img src="data:image/svg+xml;utf8,{Svg}" alt="Send" on:click={handleSubmit}/>
        </div>
