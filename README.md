# rs-webserver
Lightweight multi-threading web-server written in Rust

As of now, supports only static files (html and markdown) and GET method. Templates are stored in the templates folder. Endpoint config is in metadata.json. 

To run it
`cargo run 127.0.0.1 8081 3` where the last para is num of threads for the execution.
