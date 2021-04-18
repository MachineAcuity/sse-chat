const express = require('express');
const app = express();
const bodyParser = require('body-parser');
const cors = require('cors');

const rooms = {};

function sendEventsToAll(event, roomId) {
	if (!rooms[roomId]) {
		rooms[roomId] = [];
	}

	rooms[roomId].forEach((c) => c.res.write(`data: ${JSON.stringify(event)}\n\n`));
}

app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: false }));

app.post('/room/:roomId/send', (req, res, next) => {
	const { roomId } = req.params;
	sendEventsToAll(req.body, roomId);
	return res.send('ok');
});

// GET /room/:name/listen -> messages stream
app.get('/room/:roomId/listen', function(req, res) {
	res.writeHead(200, {
		'Content-Type': 'text/event-stream',
		Connection: 'keep-alive',
		'Cache-Control': 'no-cache'
	});

	const { roomId } = req.params;
	const clientId = Date.now();

	if (!rooms[roomId]) {
		rooms[roomId] = [];
	}

	rooms[roomId].push({
		id: clientId,
		res
	});

	const userData = `event: user\ndata: ${clientId}\n\n`;
	res.write(userData);

	const firstMessageData = `data: ${JSON.stringify({
		user_id: 0,
		message: 'Welcome! Happy to see you ;)',
		time: Date.now()
	})}\n\n`;
	res.write(firstMessageData);

	req.on('close', () => {
		console.log(`${clientId} Connection closed`);
		rooms[roomId] = rooms[roomId].filter((c) => c.id !== clientId);
	});
});

app.listen(3000, function() {
	console.log('SSE Tchat listening on port 3000!');
});
