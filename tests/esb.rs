#[test]
fn esb_xml_to_json() {
    let xml = std::fs::read_to_string("tests/xml.xml").unwrap();

    let json_bytes = util::esb_xml_to_json(xml.into()).unwrap();
    let json_str = std::str::from_utf8(&*json_bytes).unwrap();
    println!("{}", json_str);

    let expect = std::fs::read_to_string("tests/json.json").unwrap();
    assert_eq!(json_str, expect);
}

#[test]
fn esb_json_to_xml() {
    let json = std::fs::read_to_string("tests/esb_json.json").unwrap();

    let xml_bytes = util::esb_json_to_xml(json.into()).unwrap();
    let xml_str = std::str::from_utf8(&*xml_bytes).unwrap();
    println!("{}", xml_str);

    let expect = std::fs::read_to_string("tests/xml.xml").unwrap();
    assert_eq!(xml_str, expect);
}
