# File Server

A simple rust webserver that gets your resources online.

## Main Ideas
- Config: Specifies how the file server should act.
- Resource: A file/data served by the file server.

## Architecture
A config is created to produce a server.

## Config
By default, the file server will serve files from the directory it's executed in at 127.0.0.1:3000



## Roadmap
- Allow Config to be created from environment args
  - Parse env args in main
  - Create from env args method that yields a result type
  
- Add "resources" as endpoints that require configurations that must exist for boot up