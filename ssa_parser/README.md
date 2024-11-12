# ssa and ass subtitle parser

# example

```rust
fn main() -> anyhow::Result<()> {
    let path = "your .ass file path here";
    let mut file = ssa_parser::file::File::from_file(path)?;
    let offset_duration = Duration::from_secs(1);
    for event in file.events.iter_mut() {
        if let Some(start) = event
            .get_mut(EventFormat::Start)
            .and_then(Value::as_duration_mut)
        {
            *start += offset_duration;
        }
        if let Some(end) = event
            .get_mut(EventFormat::End)
            .and_then(Value::as_duration_mut)
        {
            *end += offset_duration;
        }
    }
    Ok(())
}
```