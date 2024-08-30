# poem-openapi with passage_flex

This is an example of how to use the `passage_flex` crate with an HTTP server implemented with [poem-openapi](https://crates.io/crates/poem-openapi).


## How to run

```bash
```

Start the server, setting `PASSAGE_APP_ID` and `PASSAGE_API_KEY` environment variables with valid values:
```bash
PASSAGE_APP_ID=your-app-id PASSAGE_API_KEY=your-api-key cargo run
```

Test the server operations:
```bash
curl -X POST http://localhost:3000/auth/register \
    -H 'content-type: application/json' \
    -d '{"email":"demo@passage.id"}'
```
