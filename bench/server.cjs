const http = require('http');
const fs = require('fs');

const server = http.createServer((req, res) => {
  if (req.method === 'POST' && req.url === '/report') {
    let body = '';
    req.on('data', (chunk) => { body += chunk; });
    req.on('end', () => {
      console.log('REPORT ' + body);
      res.writeHead(204).end();
    });
    return;
  }
  const file = req.url === '/' || req.url === '/bench.html' ? '/tmp/perf/bench.html' : null;
  if (!file) { res.writeHead(404).end(); return; }
  res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
  res.end(fs.readFileSync(file));
});

server.listen(8799, '127.0.0.1', () => console.log('bench server on 127.0.0.1:8799'));
