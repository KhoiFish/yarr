import http.server
import socketserver
import posixpath
import mimetypes
import ssl

PORT = 8080
BIND_ADDR = ""
DIRECTORY = "www"

class HttpRequestHandler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        '': 'application/octet-stream',
        '.manifest': 'text/cache-manifest',
        '.html': 'text/html',
        '.png':  'image/png',
        '.jpg':  'image/jpg',
        '.svg':	 'image/svg+xml',
        '.css':	 'text/css',
        '.js':   'application/x-javascript',
        '.wasm': 'application/wasm',
        '.json': 'application/json',
        '.xml':  'application/xml',
        '.wasm': 'application/wasm',
        '.mjs': 'text/javascript',
    }

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    # Keep this arround when we to try shared memory in web assembly
    # def end_headers(self):
    #     self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
    #     self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
    #     http.server.SimpleHTTPRequestHandler.end_headers(self)

httpd = socketserver.TCPServer((BIND_ADDR, PORT), HttpRequestHandler)

# Keep this arround when we do local https serving (required for cross-origin)
# httpd.socket = ssl.wrap_socket(httpd.socket,
#                                server_side=True,
#                                certfile='localhost.pem',
#                                ssl_version=ssl.PROTOCOL_TLS)

try:
    print(f"serving at http://{BIND_ADDR}:{PORT}")
    httpd.serve_forever()
except KeyboardInterrupt:
    pass
