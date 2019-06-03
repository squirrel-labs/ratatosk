#!/usr/bin/env python3

from threading import Thread
from socket import socket, SOL_SOCKET, SO_REUSEADDR

if False:
    CSP = {
        'script-src': ["*", "'unsafe-inline'"],
        'worker-src': ["*", "'unsafe-inline'"],
        'style-src': ["*", "'unsafe-inline'"],
        'default-src': ["*", "'unsafe-inline'"],
    }

    ADD_HEADERS = 'Content-Security-Policy: ' + '; '.join(
            k + ' ' + ' '.join(v) for k, v in CSP.items())
    # ADD_HEADERS += '\r\n' + ADD_HEADERS.replace('cy: ', 'cy-Report-Only: ')
    print(ADD_HEADERS)
    ADD_HEADERS = '\r\n' + ADD_HEADERS
ADD_HEADERS = ''

class Client:
    def __init__(self, sock, addr):
        self.sock, self.addr = sock, addr

    def rec(self):
        b = b''
        while not b.endswith(b'\r\n\r\n'):
            b += self.sock.recv(1)
        lines = b.replace(b'\r\n', b'\n').decode('utf-8').strip('\n').split('\n')
        method, loc, ver = lines[0].split(' ')
        print(f'request from \'{self.addr}\': "{loc}"')
        attrs = {key: value for key, value in (i.split(': ') for i in lines[1:])}
        return method, loc, attrs, ver

    def sen(self, loc, ver):
        print(f'request {loc}')
        if loc.startswith('/'):
            loc = loc[1:]
        if not loc:
            loc = 'index.html'
        try:
            if loc == 'favicon.ico':
                raise FileNotFoundError
            f = open(loc, 'rb')
            c = f.read()
            f.close()
            print(f'successfully requested {loc}')
            if loc.endswith('.js'):
                mime = 'application/javascript'
            elif loc.endswith('.html'):
                mime = 'text/html'
            elif loc.endswith('.wasm'):
                mime = 'application/wasm'
            else:
                mime = 'text/plain'
            packet = f'HTTP/1.1 200 Success\r\nContent-Length: {len(c)}\r\nContent-Type: {mime}{ADD_HEADERS}\r\n\r\n'.encode('utf-8') + c
            self.sock.send(packet)
        except FileNotFoundError:
            print(f'error request {loc}')
            self.sock.send(f'HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nContent-Type: text/plain\r\n\r\n'.encode('utf-8'))
        finally:
            try:
                f.close()
            except:
                ...

    def run(self):
        while True:
            method, loc, attrs, ver = self.rec()
            self.sen(loc, ver)

    def run_threaded(self):
        thread = Thread(target=self.run, daemon=True)
        thread.run()


def run_server(addr, port):
    ws = socket()
    ws.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
    ws.bind((addr, port))
    ws.listen()

    clients = []

    while True:
        c, a = ws.accept()
        print(f'{a[0]}:{a[1]} connected')
        client = Client(c, a)
        clients.append(clients)
        client.run_threaded()


if __name__ == '__main__':
    from sys import argv
    daemon = False
    addr = 'localhost'
    flag = None
    for arg in argv[1:]:
        if flag is not None:
            if flag == 'addr':
                addr = arg
            elif flag == 'port':
                port = int(arg)
            flag = None
        elif arg in ('-d', '--daemon'):
            daemon = True
        elif arg in ('-a', '--addr', '--address'):
            flag = 'addr'
        elif arg in ('-p', '--port'):
            flag = 'port'
    daemon = len(argv) > 1 and argv[1] in ('-d', '--daemon')
    if ':' in addr:
        addr, port = addr.split(':')
        port = int(port)
    if daemon:
        import sys
        from os import getcwd
        from daemon import DaemonContext
        with DaemonContext(working_directory=getcwd(), stderr=sys.stderr):
            run_server(addr, port)
    else:
        run_server(addr, port)
