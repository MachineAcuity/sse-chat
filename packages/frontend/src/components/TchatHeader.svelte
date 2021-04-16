<script>
export let title = "Chat";
export let messageCount = 0;
</script>

<style>
.root {
    padding: 20px;
    border-bottom: 2px solid white;
}

.root .about {
    float: left;
    padding-left: 10px;
    margin-top: 6px;
}

.root .with {
    font-weight: bold;
    font-size: 16px;
}

.root .num-messages {
    color: #92959e;
}

.clearfix:after {
    visibility: hidden;
    display: block;
    font-size: 0;
    content: " ";
    clear: both;
    height: 0;
}
</style>

<div class="root clearfix">
    <div class="about">
        <div class="with">{title}</div>
        <div class="num-messages">
            already {messageCount > 1 ? `${messageCount} messages` : `${messageCount} message`}
        </div>
    </div>
</div>
