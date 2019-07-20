#!/usr/bin/env python3

import sys
from http.server import SimpleHTTPRequestHandler as Handler
import socketserver

def main():
    try:
        port = int(sys.argv[1])
    except Exception:
        port = 8000

    Handler.extensions_map.update({ '.wasm': 'application/wasm' })

    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(('', port), Handler) as httpd:
        httpd.allow_reuse_address = True
        print('serving at port', port)
        httpd.serve_forever()

if __name__ == '__main__':
    main()
