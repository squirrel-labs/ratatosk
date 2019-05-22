from socket import (socket, AF_INET, SOCK_STREAM, IPPROTO_TCP,
                    SOL_SOCKET, SO_REUSEADDR)
from threading import Thread

WASM_MIME = 'application/wasm'
JS_MIME = 'application/javascript'
PLAIN_MIME = 'text/plain'
HTML_MIME = 'text/html'

REQUESTS = {
    '/': ('index.html', HTML_MIME),
#    '/load_ws.js': ('load_ws.js', JS_MIME),
    '/asm_paint_bg.wasm': ('pkg/asm_paint_bg.wasm', WASM_MIME)
}

PAGE_404 = '''<!doctype html><html><head></head>
<body>
    <marquee><h1>Request '404 Not Found'</h1></marquee>
    <span>resource <address>'<strong>{}</strong>'</address> not found</span>
</body>
</html>'''

def header_line_to_entry(line):
    key, value = line.decode('utf-8').split(': ')
    return key, value


class Server:
    def __init__(self):
        self.s = socket(AF_INET, SOCK_STREAM, IPPROTO_TCP)
        self.s.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
        self.threads = []

    def rec_http(self, client):
        headers = b''
        while not headers.endswith(b'\r\n' * 2):
            headers += client.recv(1)
        headers = headers.split(b'\r\n')
        head, headers = headers[0], headers[1:]
        method, url, _ = head.split(b' ')
        url = url.decode('utf-8')
        headers = dict(header_line_to_entry(v) for v in headers if v)
        if 'Content-Length' in headers:
            client.recv(int(headers['Content-Length']))
        return method, url, headers

    def sen_http(self, client, status='200 OK', payload=b'', mime=PLAIN_MIME):
        print('sende...')
        client.send((f'HTTP/1.1 {status}\r\n'
                    f'Content-Length: {len(payload)}\r\n'
                    f'Content-Type: {mime}\r\n\r\n').encode('utf-8')
                    + payload)
        print('gesendet')

    def run_client(self, client, addr):
        while True:
            print('wait for receive')
            method, url, headers = self.rec_http(client)
            print('got receive')
            if method == b'GET':
                if not url.startswith('/'):
                    url += '/'
                print(f'got request for "{url}"')
                if url in REQUESTS:
                    path, mime = REQUESTS[url]
                    f = open(path, 'rb')
                    payload = f.read()
                    f.close()
                    self.sen_http(client, '200 OK', payload, mime)
                elif url == '/close':
                    client.close()
                    self.kill()
                    exit()
                else:
                    self.sen_http(client, '404 Not Found',
                            PAGE_404.format(url).encode('utf-8'),
                            HTML_MIME)
            else:
                self.sen_http(client, '400 Bad Request', b'only supporting GET')

    def deploy(self, host='localhost', port=8080):
        self.s.bind((host, port))
        self.s.listen(1)
        while True:
            client, addr = self.s.accept()
            thread = Thread(target=self.run_client, args=(client,addr))
            self.threads.append(thread)
            thread.run()

    def kill(self):
        self.s.close()
        

if __name__ == '__main__':
    try:
        server = Server()
        server.deploy()
    finally:
        server.kill()
