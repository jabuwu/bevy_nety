# bevy nety

My attempt at adding networking to Bevy 0.6.1.

## Goals

- Server/client model
- Protocol agnostic (can be tcp/udp, webrtc, etc behind the scenes)
- Support multiple hosts (listen on websocket and native tcp at same time)
- API first design
- WASM/browser compatible

## Features

- Host locally, as server/client, or server (no client)
- Networked bevy events (send from server to client(s), or client to server)
- Entity relevancy (currently there is no API to interact with this though)
- Entity ownership
- Entity based events (send events from owner to server, or from any client to the entity's owner)

## Status

The code is messy, but since Bevy is easy to test, I'm doing a TDD approach to this library. There is about as much test code as there is library code (found in `src/tests`). Once all the main features are working with relevant tests I will spend some time refactoring the code. Currently, all the heavy lifting occurs in `src/network.rs`.
