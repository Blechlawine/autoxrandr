/*
    Copyright (C) 2024 Blechlawine
    GNU General Public License v3.0+ ( see LICENSE or https://www.gnu.org/licenses/gpl-3.0.txt )
*/
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, multispace0, none_of, not_line_ending, u32},
    combinator::{map, map_res, opt},
    multi::{many1, separated_list0},
    sequence::tuple,
    IResult,
};
use serde::{Deserialize, Serialize};

pub fn parse(xrandr_output: &str) -> XRandrOutput {
    let (_, xrandr_output) = XRandrOutput::parse(xrandr_output.as_bytes()).unwrap();
    xrandr_output
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XRandrOutput {
    pub displays: Vec<DisplayDevice>,
}

impl XRandrOutput {
    pub fn parse(input: &[u8]) -> IResult<&[u8], XRandrOutput> {
        map(
            tuple((
                not_line_ending,
                line_ending,
                separated_list0(line_ending, DisplayDevice::parse),
            )),
            |(_, _, displays)| XRandrOutput { displays },
        )(input)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayDevice {
    /// is the display connected or not
    pub state: ConnectionState,
    /// the resolution of the display
    pub resolution: Option<Resolution>,
    /// the position of the display
    pub offset: Option<Offset>,
    /// is the display primary or not
    pub primary: bool,
    /// Which connector is the display connected to
    pub connector: String,
    /// The capabilities of the display
    pub capabilities: Vec<Capability>,
}

type Offset = (u32, u32);
type Resolution = (u32, u32);

fn parse_resolution_and_offset(input: &[u8]) -> IResult<&[u8], (Resolution, Offset)> {
    map(
        tuple((u32, tag("x"), u32, tag("+"), u32, tag("+"), u32)),
        |(w, _, h, _, x, _, y)| ((w, h), (x, y)),
    )(input)
}

fn parse_connector(input: &[u8]) -> IResult<&[u8], String> {
    map(many1(none_of(" \t\n\r")), |connector| {
        connector.into_iter().collect::<String>()
    })(input)
}

impl DisplayDevice {
    fn parse(input: &[u8]) -> IResult<&[u8], DisplayDevice> {
        map(
            tuple((
                // TODO: the parser gets here, but still with the newline of the previous line,
                // which shouldn't be there
                parse_connector,
                multispace0,
                ConnectionState::parse,
                multispace0,
                opt(tag("primary")),
                opt(multispace0),
                opt(parse_resolution_and_offset),
                opt(multispace0),
                not_line_ending, // ignore the rest
                separated_list0(line_ending, Capability::parse),
            )),
            |(connector, _, state, _, primary, _, res_offset, _, _, capabilities)| DisplayDevice {
                state,
                resolution: res_offset.map(|((w, h), (_, _))| (w, h)),
                offset: res_offset.map(|((_, _), (x, y))| (x, y)),
                primary: primary.is_some(),
                connector,
                capabilities,
            },
        )(input)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capability {
    resolution: Resolution,
    pub refresh_rates: Vec<RefreshRate>,
}

impl Capability {
    fn parse(input: &[u8]) -> IResult<&[u8], Capability> {
        map(
            tuple((
                multispace0,
                u32,
                tag("x"),
                u32,
                multispace0,
                separated_list0(multispace0, RefreshRate::parse),
            )),
            |(_, w, _, h, _, refresh_rates)| Capability {
                resolution: (w, h),
                refresh_rates,
            },
        )(input)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRate {
    pub clock: f32,
    pub current: bool,
    preferred: bool,
}

fn parse_clock(input: &[u8]) -> IResult<&[u8], f32> {
    map_res(
        tuple((digit1, tag("."), digit1)),
        |(base_ten, _, decimal)| {
            format!(
                "{}.{}",
                std::str::from_utf8(base_ten).unwrap(),
                std::str::from_utf8(decimal).unwrap()
            )
            .parse()
        },
    )(input)
}

impl RefreshRate {
    fn parse(input: &[u8]) -> IResult<&[u8], RefreshRate> {
        map(
            tuple((
                parse_clock,
                alt((tag("*"), tag(" "))),
                alt((tag("+"), tag(" "))),
            )),
            |(clock, current, preferred)| RefreshRate {
                clock,
                current: match current {
                    b"*" => true,
                    b" " => false,
                    _ => unreachable!(),
                },
                preferred: match preferred {
                    b"+" => true,
                    b" " => false,
                    _ => unreachable!(),
                },
            },
        )(input)
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

impl ConnectionState {
    fn parse(input: &[u8]) -> IResult<&[u8], ConnectionState> {
        map(
            alt((tag("connected"), tag("disconnected"))),
            |s: &[u8]| match s {
                b"connected" => ConnectionState::Connected,
                b"disconnected" => ConnectionState::Disconnected,
                _ => unreachable!(),
            },
        )(input)
    }
}

pub fn parse_active_monitors(input: &[u8]) -> IResult<&[u8], Vec<String>> {
    map(
        tuple((
            not_line_ending,
            line_ending,
            separated_list0(
                line_ending,
                tuple((
                    multispace0,
                    u32,
                    tag(":"),
                    multispace0,
                    opt(tag("+")),
                    opt(tag("*")),
                    parse_connector,
                    not_line_ending,
                )),
            ),
        )),
        |(_, _, monitors)| {
            monitors
                .into_iter()
                .map(|(_, _, _, _, _, _, connector, _)| connector)
                .collect()
        },
    )(input)
}
