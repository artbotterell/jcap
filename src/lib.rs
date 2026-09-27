//! CAP 1.2 alert XML -> JSON. Fields are matched by local name, so the
//! enveloped XML-DSig <Signature> child is never carried over. Absent in XML
//! means absent in JSON. <resource> is not modeled yet.

use roxmltree::{Document, Node};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    #[serde(skip_serializing_if = "Option::is_none")] pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub sender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub sent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub msg_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub restriction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub addresses: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub code: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub references: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub incidents: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub info: Vec<Info>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    #[serde(skip_serializing_if = "Option::is_none")] pub language: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub category: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub event: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub response_type: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub urgency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub severity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub certainty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub audience: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub event_code: Vec<NameValue>,
    #[serde(skip_serializing_if = "Option::is_none")] pub effective: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub onset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub expires: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub sender_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub headline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub instruction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub web: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub contact: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub parameter: Vec<NameValue>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub area: Vec<Area>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    #[serde(skip_serializing_if = "Option::is_none")] pub area_desc: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub polygon: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub circle: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")] pub geocode: Vec<NameValue>,
    #[serde(skip_serializing_if = "Option::is_none")] pub altitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub ceiling: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NameValue {
    pub value_name: String,
    pub value: String,
}

fn children<'a, 'i>(n: Node<'a, 'i>, name: &'static str) -> impl Iterator<Item = Node<'a, 'i>> {
    n.children().filter(move |c| c.is_element() && c.tag_name().name() == name)
}

fn text(n: Node, name: &'static str) -> Option<String> {
    children(n, name).next().map(|c| c.text().unwrap_or("").to_string())
}

fn texts(n: Node, name: &'static str) -> Vec<String> {
    children(n, name).map(|c| c.text().unwrap_or("").to_string()).collect()
}

fn pairs(n: Node, name: &'static str) -> Vec<NameValue> {
    children(n, name)
        .map(|c| NameValue {
            value_name: text(c, "valueName").unwrap_or_default(),
            value: text(c, "value").unwrap_or_default(),
        })
        .collect()
}

pub fn parse_alert(xml: &str) -> Result<Alert, String> {
    let doc = Document::parse(xml).map_err(|e| e.to_string())?;
    let a = doc.root_element();
    if a.tag_name().name() != "alert" {
        return Err(format!("root is <{}>, not <alert>", a.tag_name().name()));
    }
    Ok(Alert {
        identifier: text(a, "identifier"),
        sender: text(a, "sender"),
        sent: text(a, "sent"),
        status: text(a, "status"),
        msg_type: text(a, "msgType"),
        source: text(a, "source"),
        scope: text(a, "scope"),
        restriction: text(a, "restriction"),
        addresses: text(a, "addresses"),
        code: texts(a, "code"),
        note: text(a, "note"),
        references: text(a, "references"),
        incidents: text(a, "incidents"),
        info: children(a, "info")
            .map(|i| Info {
                language: text(i, "language"),
                category: texts(i, "category"),
                event: text(i, "event"),
                response_type: texts(i, "responseType"),
                urgency: text(i, "urgency"),
                severity: text(i, "severity"),
                certainty: text(i, "certainty"),
                audience: text(i, "audience"),
                event_code: pairs(i, "eventCode"),
                effective: text(i, "effective"),
                onset: text(i, "onset"),
                expires: text(i, "expires"),
                sender_name: text(i, "senderName"),
                headline: text(i, "headline"),
                description: text(i, "description"),
                instruction: text(i, "instruction"),
                web: text(i, "web"),
                contact: text(i, "contact"),
                parameter: pairs(i, "parameter"),
                area: children(i, "area")
                    .map(|r| Area {
                        area_desc: text(r, "areaDesc"),
                        polygon: texts(r, "polygon"),
                        circle: texts(r, "circle"),
                        geocode: pairs(r, "geocode"),
                        altitude: text(r, "altitude"),
                        ceiling: text(r, "ceiling"),
                    })
                    .collect(),
            })
            .collect(),
    })
}

/// One call: CAP XML in, compact signature-free JSON out.
pub fn to_json(xml: &str) -> Result<String, String> {
    parse_alert(xml).map(|a| serde_json::to_string(&a).expect("Alert serializes"))
}

#[cfg(test)]
mod tests {
    use super::{parse_alert, to_json};
    use serde_json::{json, Value};

    fn value(xml: &str) -> Value {
        serde_json::to_value(parse_alert(xml).unwrap()).unwrap()
    }

    #[test]
    fn sample_fixture() {
        let v = value(include_str!("../testdata/sample-cap.xml"));
        assert_eq!(v["identifier"], "NWS-IDP-PROD-SAMPLE-0001");
        assert_eq!(v["msgType"], "Alert");
        assert_eq!(v["info"][0]["category"], json!(["Met"]));
        assert_eq!(v["info"][0]["event"], "Sample Test Event");
        assert_eq!(v["info"][0]["headline"], "Sample alert for feedhook tests");
        assert!(v["info"][0].get("area").is_none());
    }

    #[test]
    fn signature_is_dropped() {
        let plain = value(include_str!("../testdata/sample-cap.xml"));
        let signed_xml = include_str!("../testdata/sample-cap-signed.xml");
        assert!(signed_xml.contains("SignatureValue"));
        let signed = value(signed_xml);
        assert_eq!(plain, signed);
        assert!(!signed.to_string().contains("FAKESIGNATURE"));
    }

    #[test]
    fn area_and_pairs() {
        let v = value(
            r#"<cap:alert xmlns:cap="urn:oasis:names:tc:emergency:cap:1.2">
              <cap:identifier>X</cap:identifier>
              <cap:info>
                <cap:parameter><cap:valueName>BLOCKCHANNEL</cap:valueName><cap:value>CMAS</cap:value></cap:parameter>
                <cap:area>
                  <cap:areaDesc>Santa Clara</cap:areaDesc>
                  <cap:polygon>37.1,-122.0 37.2,-121.9 37.1,-121.8 37.1,-122.0</cap:polygon>
                  <cap:geocode><cap:valueName>SAME</cap:valueName><cap:value>006085</cap:value></cap:geocode>
                  <cap:geocode><cap:valueName>UGC</cap:valueName><cap:value>CAZ513</cap:value></cap:geocode>
                </cap:area>
              </cap:info>
            </cap:alert>"#,
        );
        let area = &v["info"][0]["area"][0];
        assert_eq!(area["areaDesc"], "Santa Clara");
        assert_eq!(area["polygon"][0], "37.1,-122.0 37.2,-121.9 37.1,-121.8 37.1,-122.0");
        assert_eq!(area["geocode"][1], json!({"valueName": "UGC", "value": "CAZ513"}));
        assert_eq!(v["info"][0]["parameter"][0]["value"], "CMAS");
    }

    #[test]
    fn to_json_is_compact_and_unsigned() {
        let s = to_json(include_str!("../testdata/sample-cap-signed.xml")).unwrap();
        assert!(s.starts_with(r#"{"identifier":"NWS-IDP-PROD-SAMPLE-0001""#));
        assert!(!s.contains('\n') && !s.contains("FAKESIGNATURE"));
    }

    #[test]
    fn rejects_non_alert() {
        assert!(parse_alert("<alerts/>").is_err());
        assert!(parse_alert("not xml").is_err());
    }
}
