use crate::xrandr_output::XRandrOutput;

pub fn parse(xrandr_output: &str) -> XRandrOutput {
    let (_, xrandr_output) = XRandrOutput::parse(xrandr_output.as_bytes()).unwrap();
    xrandr_output
}
