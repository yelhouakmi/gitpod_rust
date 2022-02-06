# Rust demo project

This project is a simple demonstration of a WS

This project use the «Rocket» framework and need an recent Rust version.

## Running

To run the project, simply launch the command:

```
cargo run
```

The server will be launched on port 8000

## Services

### get /Hello

Return the «Hello, word!» String.

### get /hello/\<name>

Catch the "name" url variable and return the «Hello, {name}!» String.

## get /complex

Return the following Json:

```Json
{"id":35,"name":"test","description":["human","animal"],"map":{"Test":"one","other":"two"}}
```

## post /complex

Take a Json in the following format:

```Json
{"id": number, "name": string, "description": Array<String>, "map": Map<String, String>}
```

And return the String «Received object with id {id} and name {name}»