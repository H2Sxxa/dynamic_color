fn main() -> Result<(), Box<dyn std::error::Error>> {
    let color = dynamic_color::accent_color()?;
    println!("{}", color.to_hex());
    Ok(())
}
