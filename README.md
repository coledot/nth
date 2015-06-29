# nth

a Unix command-line utility to filter output by defined columns

## usage

`nth <columns>`, each column being an integer index

## building

`cargo build --release`

## why?

an opportunity to learn Rust, and to scratch a commandline itch.

### what about `cut`?

`cut` works great for situations where the whitespace is fixed, e.g. a single space or single tab. `nth` just goes by whitespace.

### what about `awk`?

filtering by columns is a basic use case for awk, and having a separate tool to do so reduces the semantic overhead of writing out commands. no more "{print $1, $2, ...}" verbosity, just `nth 1 2 3`

## License

WTFPL

