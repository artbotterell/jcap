# jcap

Parses a CAP 1.2 (Common Alerting Protocol) alert from XML and transcodes it
to JSON, dropping the enveloped XML digital signature.

```rust
let json: String = jcap::to_json(xml)?;          // compact JSON, one call
let alert: jcap::Alert = jcap::parse_alert(xml)?; // or the parsed structs
```

Both return `Err(String)` if the input is not XML or its root is not `<alert>`.

## JSON conventions

- Keys are CAP element names: `identifier`, `msgType`, `info`, `areaDesc`, …
- Elements CAP allows to repeat are always arrays, even with one entry:
  `code`, `info`, `category`, `responseType`, `eventCode`, `parameter`,
  `area`, `polygon`, `circle`, `geocode`.
- `eventCode`, `parameter` and `geocode` entries are
  `{"valueName": ..., "value": ...}` objects.
- Elements absent from the XML are absent from the JSON. Text is kept as
  written, including line breaks.
- Elements are matched by local name, so a namespace prefix (`cap:alert`) is
  accepted and the XML-DSig `<Signature>` is never carried over.
- `<resource>` is not yet modeled.

## License

MIT; see [LICENSE](LICENSE).
