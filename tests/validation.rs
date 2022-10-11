use invoice_generator::*;

#[test]
fn total_is_correct() {
    assert_eq!(build_invoice(1000).price(), 245_000);
}

#[test]
fn statistics_are_correct() {
    let stats = build_invoice(1000).statistics();
    assert_eq!(stats.hammers_sold, 2000);
    assert_eq!(stats.steel_required, 4000);
    assert_eq!(stats.nails_dispensed, 250_000);
}

#[test]
fn formatting_is_correct() {
    let output = build_invoice(2).to_string();
    assert_eq!(
        output,
        r#"Bundle: steel framing hammer, steel framing nails (x200), brass roofing nails (x50), steel jigsaw, steel sledge hammer: 245,00
Bundle: steel framing hammer, steel framing nails (x200), brass roofing nails (x50), steel jigsaw, steel sledge hammer: 245,00
"#
    );
}
