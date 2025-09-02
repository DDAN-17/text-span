fn main() {
    let features = [
        "span-value-usize",
        "span-value-u128",
        "span-value-u64",
        "span-value-u32",
        "span-value-u16",
        "span-value-u8",
    ];

    let mut selected_features = 0;
    for feature in features {
        println!("{:?}", std::env::vars().collect::<Vec<_>>());
        if std::env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase().replace("-", "_"))).is_ok() {
            selected_features += 1;
        }
    }

    if selected_features == 0 {
        panic!("Error: You must choose a span value type; enable a feature.");
    }

    if selected_features > 1 {
        panic!(
            "Error: You can only pick one span value type; please disable {} of your {selected_features} features.",
            selected_features - 1
        );
    }
}
