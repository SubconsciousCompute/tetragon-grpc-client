# tetragon-grpc-client

## Demo

[![asciicast](https://asciinema.org/a/646726.svg)](https://asciinema.org/a/646726)

## Cloning

```sh
git clone --recurse-submodules https://github.com/Aditeya/tetragon-grpc-client.git
```

The proto directory is a symlink to the `tetragon/api/v1` folder. The tetragon directory is a submodule to the tetragon repository.

## Running Example

> *NOTE:* Requires Root. The program will ask for sudo password.

```sh 
$ cargo run --bin getevents
```
