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

## Used by

- The ipaws_on_44 relay, which publishes FEMA IPAWS alerts to licensed radio
  amateurs over 44net (MQTT at `44.27.128.55:1883`, open to 44net addresses
  only; its page at <http://44.27.128.55/> is public): its MQTT topic
  `ipaws/cap/json` carries each alert as produced by `to_json`.
- [ipaws44client](https://github.com/artbotterell/ipaws44client) (`ipawsClient`),
  a command-line client for an IPAWS alert relay on 44net: it uses
  `parse_alert` to read raw CAP XML alerts so it can match their SAME codes,
  polygons, circles and UGC codes against a Maidenhead grid square.

## License

MIT; see [LICENSE](LICENSE).
