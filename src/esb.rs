use std::io::Write;
use std::option::Option::Some;

use bytes::{Buf, BufMut, Bytes, BytesMut};
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::{Error, Reader, Result, Writer};
use serde_json::{Map, Value};

#[derive(Eq, PartialEq)]
enum Level {
    Service,
    Body,
    Field,
}

pub fn esb_xml_to_json(b: Bytes) -> Result<Bytes> {
    let mut reader = Reader::from_reader(b.reader());
    let mut msg = BytesMut::new();
    let mut buf = Vec::new();
    let mut is_field = false;

    loop {
        match reader.read_event(&mut buf)? {
            Event::Start(s) => match &*s.name().to_ascii_lowercase() {
                b"service" | b"struct" => msg.put_u8(b'{'),
                b"body" => msg.put_slice(br#""BODY":{"#),
                b"array" => msg.put_u8(b'['),
                b"data" => {
                    let mut attrs = s.attributes();
                    while let Some(Ok(a)) = attrs.next() {
                        if b"name".eq(&*a.key.to_ascii_lowercase()) {
                            msg.put_u8(b'"');
                            msg.put_slice(&*a.value);
                            msg.put_slice(br#"":"#);
                            break;
                        }
                    }
                }
                b"field" => is_field = true,
                _ => {}
            },
            Event::End(e) => match &*e.name().to_ascii_lowercase() {
                b"service" => {
                    truncate(&mut msg, b",");
                    msg.put_u8(b'}');
                }
                b"struct" | b"body" => {
                    truncate(&mut msg, b",");
                    msg.put_slice(b"},");
                }
                b"array" => {
                    truncate(&mut msg, b",");
                    msg.put_slice(b"],");
                }
                _ => {}
            },
            Event::Empty(s) => match &*s.name().to_ascii_lowercase() {
                b"struct" => msg.put_slice(b"{}"),
                b"array" => msg.put_slice(b"[]"),
                _ => {}
            },
            Event::Text(t) => {
                if is_field {
                    msg.put_u8(b'"');
                    msg.put_slice(&*t);
                    msg.put_slice(br#"","#);
                    is_field = false;
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(msg.freeze())
}

fn truncate(msg: &mut BytesMut, needle: &[u8]) {
    if msg.ends_with(needle) {
        msg.truncate(msg.len() - needle.len())
    };
}

pub fn esb_json_to_xml(b: Bytes) -> Result<Bytes> {
    let mut res = Vec::new().writer();
    res.write(&br#"<?xml version="1.0" encoding="UTF-8"?><service>"#[..])
        .unwrap();
    let mut writer = Writer::new(res);

    let json: Value =
        serde_json::from_reader(&*b).map_err(|e| Error::UnexpectedToken(e.to_string()))?;
    write_value(&mut writer, &json, Level::Service)?;

    let elem = BytesEnd::borrowed("service".as_bytes());
    writer.write_event(Event::End(elem))?;

    Ok(writer.into_inner().into_inner().into())
}

fn write_value(
    writer: &mut Writer<bytes::buf::Writer<Vec<u8>>>,
    value: &Value,
    level: Level,
) -> Result<()> {
    match value {
        Value::String(s) => {
            let fields = s.split(",").collect::<Vec<&str>>();
            if fields.len() < 3 {
                return Ok(());
            }

            let mut elem = BytesStart::borrowed_name("field".as_bytes());
            elem.push_attribute(("type", match_field(fields[0])));
            elem.push_attribute(("length", fields[1]));
            elem.push_attribute(("scale", fields[2]));
            writer.write_event(Event::Start(elem.clone()))?;

            if let Some(field) = fields.get(3) {
                let text = BytesText::from_plain_str(field);
                writer.write_event(Event::Text(text))?;
            }

            writer.write_event(Event::End(elem.to_end()))?;
        }
        Value::Array(a) => {
            let elem = BytesStart::borrowed_name("array".as_bytes());
            writer.write_event(Event::Start(elem.clone()))?;
            for v in a {
                write_value(writer, v, Level::Field)?;
            }
            writer.write_event(Event::End(elem.to_end()))?;
        }
        Value::Object(o) => {
            write_map(writer, o, level)?;
        }
        _ => {}
    }

    Ok(())
}

fn write_map(
    writer: &mut Writer<bytes::buf::Writer<Vec<u8>>>,
    object: &Map<String, Value>,
    level: Level,
) -> Result<()> {
    if level == Level::Field {
        let elem = BytesStart::borrowed_name("struct".as_bytes());
        writer.write_event(Event::Start(elem))?;
    }

    for (k, v) in object.iter() {
        match k.as_str() {
            name if level == Level::Service && name.to_ascii_lowercase().eq("body") => {
                let elem = BytesStart::borrowed_name("body".as_bytes());
                writer.write_event(Event::Start(elem.clone()))?;
                if let Value::Object(o) = v {
                    write_map(writer, o, Level::Body)?;
                }
                writer.write_event(Event::End(elem.to_end()))?;
            }
            name if level == Level::Service => {
                let elem = BytesStart::borrowed_name(match_head(name).as_bytes());
                writer.write_event(Event::Start(elem.clone()))?;
                write_data_start(writer, name)?;
                write_value(writer, v, Level::Field)?;
                write_data_end(writer)?;
                writer.write_event(Event::End(elem.to_end()))?;
            }
            name => {
                write_data_start(writer, name)?;
                write_value(writer, v, Level::Field)?;
                write_data_end(writer)?;
            }
        }
    }

    if level == Level::Field {
        let elem = BytesEnd::borrowed("struct".as_bytes());
        writer.write_event(Event::End(elem))?;
    }

    Ok(())
}

fn write_data_start(writer: &mut Writer<bytes::buf::Writer<Vec<u8>>>, name: &str) -> Result<()> {
    let mut data = BytesStart::borrowed_name("data".as_bytes());
    data.push_attribute(("name", name));
    writer.write_event(Event::Start(data))
}

fn write_data_end(writer: &mut Writer<bytes::buf::Writer<Vec<u8>>>) -> Result<()> {
    let data = BytesEnd::borrowed("data".as_bytes());
    writer.write_event(Event::End(data))
}

fn match_head(data_name: &str) -> &str {
    match &*data_name.to_ascii_lowercase() {
        "sys_head" => "sys-header",
        "app_head" => "app-header",
        "local_head" => "local-header",
        _ => "",
    }
}

fn match_field(field_type: &str) -> &str {
    match &*field_type.to_ascii_lowercase() {
        "s" => "string",
        _ => "",
    }
}
