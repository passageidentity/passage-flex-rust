# poem-openapi with passage_flex

This is an example of how to use the `passage_flex` crate with an HTTP server implemented with [poem-openapi](https://crates.io/crates/poem-openapi) in Rust.


## How to run

Set `PASSAGE_APP_ID` and `PASSAGE_API_KEY` environment variables with valid values:
```bash
export PASSAGE_APP_ID=your-app-id
export PASSAGE_API_KEY=your-api-key
```

Start the server:
```bash
cargo run
```

Test the server operations:
```bash
curl -X POST http://localhost:3000/auth/register \
    -H 'content-type: application/json' \
    -d '{"email":"demo@passage.id"}'
```
