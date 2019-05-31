#!/usr/bin/env python3

from socket import socket, SOL_SOCKET, SO_REUSEADDR

ws = socket()
ws.setsockopt(SOL_SOCKET, SO_REUSEADDR, 1)
ws.bind(('localhost', 8080))
ws.listen()

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
        return method, loc, attrs

    def sen(self, loc, ver):
        f = open(loc, 'rb')
        c = f.read()
        f.close()
        self.sock.send(f'HTTP/1.1 200'.encode('utf-8') + c)

    def run(self):
        method, loc, attrs = self.rec()
        self.sen(loc, ver)


clients = []

while True:
    c, a = ws.accept()
    print(f'{a[0]}:{a[1]} connected')
    client = Client(c, a)
    clients.append(clients)
    client.run()
