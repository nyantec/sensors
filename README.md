# sensors

Example usage:

```rust
for chip in Sensors::new() {
    let name = chip.get_name().expect("name");
    println!("Found sensor: {}", &name);
    for feature in chip {
        let label = feature.get_label().expect("label");
        println!("Found feature: {}", &label);
        for subfeature in feature {
            let value = subfeature.get_value().expect("value");
            println!("Found subfeature {}: {}", subfeature.name(), value);
        }
    }
}
```
