# tetragon-grpc-client

## Demo

![demo](https://github.com/Aditeya/tetragon-grpc-client/assets/22963960/2e9b5429-248a-4d0b-94ce-88156aa4eb76)

## Running Example

> *NOTE:* Requires Root. The program will ask for sudo password.

```sh 
$ cargo run --bin getevents
```

## Troubleshooting

If you're requests aren't happening as you expect, try checking the [tetra cli code](https://github.com/cilium/tetragon/blob/main/cmd/tetra/main.go)
and how they create their request objects.
