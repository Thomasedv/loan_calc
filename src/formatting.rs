use num_format::{Buffer, CustomFormat, Error, Grouping};

pub fn get_formatter() -> Result<CustomFormat, Error> {
    CustomFormat::builder()
        .grouping(Grouping::Standard)
        .minus_sign("-")
        .separator(" ")
        .build()
}

pub fn present_int(number: i64, formatter: &CustomFormat) -> Result<String, Error> {
    let mut buf = Buffer::default();
    buf.write_formatted(&number, formatter);
    // Get a view into the buffer as a &str
    return Ok(buf.as_str().to_string());
}
