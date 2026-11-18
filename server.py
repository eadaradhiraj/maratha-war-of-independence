#!/usr/bin/env python3
import http.server
import socketserver
import os
import sys

PORT = 8080
if len(sys.argv) > 1:
    PORT = int(sys.argv[1])

DIRECTORY = os.path.join(os.path.dirname(os.path.abspath(__file__)), "web")

class NoCacheServer(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def end_headers(self):
        # Enforce zero-cache headers so browsers (Firefox, Chrome) always fetch latest files
        self.send_header("Cache-Control", "no-store, no-cache, must-revalidate, max-age=0")
        self.send_header("Pragma", "no-cache")
        self.send_header("Expires", "0")
        self.send_header("Access-Control-Allow-Origin", "*")
        super().end_headers()

    def guess_type(self, path):
        if path.endswith(".wasm"):
            return "application/wasm"
        if path.endswith(".js"):
            return "application/javascript"
        return super().guess_type(path)

socketserver.TCPServer.allow_reuse_address = True
with socketserver.TCPServer(("", PORT), NoCacheServer) as httpd:
    print(f"Empire Total War server running at http://localhost:{PORT} (directory: {DIRECTORY})")
    httpd.serve_forever()
