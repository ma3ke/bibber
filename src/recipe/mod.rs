use std::{cmp::Ordering, num::ParseFloatError};

use crate::{time::Time, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Recipe {
    pub title: String,

    pub start: Time,
    pub end: Time,
    pub timestep: Time,
    pub snapshot: Time,

    /// Constant temperature (Kelvin).
    pub temperature: f64,

    pub particles: usize,

    /// Vector specifying boundary (meter).
    pub boundary: Vec3,
}

impl Recipe {
    /// Returns the time from start to end as specified by the recipe.
    pub(crate) fn time(&self) -> Time {
        self.end - self.start
    }

    /// Returns the number of snapshots that will be taken over the course of the simulation.
    ///
    /// The value is calculated as follows: (end - start) / snapshot
    pub(crate) fn snapshots(&self) -> usize {
        (self.time().seconds() / self.snapshot.seconds()) as usize
    }

    /// Returns the number of timesteps that will be taken over the course of the simulation.
    ///
    /// The value is calculated as follows: (end - start) / timestep
    pub(crate) fn timesteps(&self) -> usize {
        (self.time().seconds() / self.timestep.seconds()) as usize
    }
}

impl Recipe {
    /// Create a new recipe from an ASCII string in bibber format.
    pub fn from_string(src: String) -> Result<Self, BibberParseError> {
        let mut title = None;
        let mut start = None;
        let mut end = None;
        let mut timestep = None;
        let mut snapshot = None;
        let mut temperature = None;
        let mut particles = None;
        let mut boundary = None;
        for line in src.lines() {
            let mut words = line.split_ascii_whitespace();
            match words.next() {
                Some("title") => title = Some(words.collect()),
                Some("start") => start = Some(parse_single_time(words.collect())?),
                Some("end") => end = Some(parse_single_time(words.collect())?),
                Some("snapshot") => snapshot = Some(parse_single_time(words.collect())?),
                Some("timestep") => timestep = Some(parse_single_time(words.collect())?),
                Some("temperature") => temperature = Some(parse_temperature(words.collect())?),
                Some("particles") => particles = Some(parse_particles(words.collect())?),
                Some("boundary") => boundary = Some(parse_boundary(words.collect())?),
                None => {}
                _ => todo!(),
            }
        }

        Ok(Self {
            title: title.expect("recipe should specify title"),
            start: start.expect("recipe should specify start"),
            end: end.expect("recipe should specify end"),
            snapshot: snapshot.expect("recipe should specify snapshot"),
            timestep: timestep.expect("recipe should specify timestep"),
            temperature: temperature.expect("recipe should specify temperature"),
            particles: particles.expect("recipe should specify particles"),
            boundary: boundary.expect("recipe should specify boundary"),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BibberParseError {
    TooFewArguments,
    TooManyArguments,
    NoUnit,
    UnknownUnit,
    InvalidUnit,
    ParseFloatError(ParseFloatError),
}

impl std::fmt::Display for BibberParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for BibberParseError {}

impl From<ParseFloatError> for BibberParseError {
    fn from(value: ParseFloatError) -> Self {
        Self::ParseFloatError(value)
    }
}

fn check_arguments_count(arguments: &[&str], expected: usize) -> Result<(), BibberParseError> {
    match arguments.len().cmp(&expected) {
        Ordering::Less => Err(BibberParseError::TooFewArguments),
        Ordering::Greater => Err(BibberParseError::TooManyArguments),
        Ordering::Equal => Ok(()),
    }
}

fn parse_arguments<const EXPECTED: usize>(
    arguments: Vec<&str>,
) -> Result<[String; EXPECTED], BibberParseError> {
    check_arguments_count(&arguments, EXPECTED)?;
    let args: Vec<_> = arguments[..EXPECTED]
        .iter()
        .map(|s| s.to_string())
        .collect();
    // Unwrap should be safe here, because we have just verified that the length of arguments
    // is correct.
    Ok(args.try_into().unwrap())
}

// TODO: Fix horrible code duplication across parse_length, parse_time, parse_temperature.
fn parse_length(s: &str) -> Result<f64, BibberParseError> {
    match s.split_once(':') {
        None | Some((_, "")) => Err(BibberParseError::NoUnit),
        Some((number, unit)) => {
            let value: f64 = number.parse()?;
            let factor = match unit {
                "km" => 1e3,
                "m" => 1.0,
                "dm" => 1e-1,
                "cm" => 1e-2,
                "mm" => 1e-3,
                "um" => 1e-6,
                "nm" => 1e-9,
                "pm" => 1e-12,
                "fm" => 1e-15,
                "s" | "ms" | "us" | "ns" | "ps" | "fs" | "K" | "C" => {
                    return Err(BibberParseError::InvalidUnit)
                }
                _ => return Err(BibberParseError::UnknownUnit),
            };
            let meters = value * factor;
            Ok(meters)
        }
    }
}

fn parse_time(s: &str) -> Result<Time, BibberParseError> {
    match s.split_once(':') {
        None | Some((_, "")) => Err(BibberParseError::NoUnit),
        Some((number, unit)) => {
            let value: f64 = number.parse()?;
            let time = match unit {
                "s" => Time::from_seconds(value),
                "ms" => Time::from_milliseconds(value),
                "us" => Time::from_microseconds(value),
                "ns" => Time::from_nanoseconds(value),
                "ps" => Time::from_picoseconds(value),
                "fs" => Time::from_femtoseconds(value),
                "km" | "m" | "dm" | "cm" | "mm" | "um" | "nm" | "pm" | "fm" | "K" | "C" => {
                    return Err(BibberParseError::InvalidUnit)
                }
                _ => return Err(BibberParseError::UnknownUnit),
            };
            Ok(time)
        }
    }
}

fn parse_temperature_value(s: &str) -> Result<f64, BibberParseError> {
    match s.split_once(':') {
        None | Some((_, "")) => Err(BibberParseError::NoUnit),
        Some((number, unit)) => {
            let value: f64 = number.parse()?;
            let offset = match unit {
                "K" => 0.0,
                "C" => 273.15, // 0 C == -273.15 K
                "km" | "m" | "dm" | "cm" | "mm" | "um" | "nm" | "pm" | "fm" | "s" | "ms" | "us"
                | "ns" | "ps" | "fs" => return Err(BibberParseError::InvalidUnit),
                _ => return Err(BibberParseError::UnknownUnit),
            };
            let kelvin = value - offset;
            Ok(kelvin)
        }
    }
}

/// Parse one time value.
fn parse_single_time(arguments: Vec<&str>) -> Result<Time, BibberParseError> {
    let [time] = parse_arguments(arguments)?;
    parse_time(&time)
}

/// Parse temperature.
///
/// # Example
///
/// ```
/// // Line from which args are derived: temperature 300:K
/// let args = vec!["300:K"];
/// assert_eq!(parse_temperature(args), 300.0)
/// ```
fn parse_temperature(arguments: Vec<&str>) -> Result<f64, BibberParseError> {
    let [temperature] = parse_arguments(arguments)?;
    parse_temperature_value(&temperature)
}

/// Parse number of particles.
///
/// # Example
///
/// ```
/// // Line from which args are derived: boundary cubic 100:nm 100:nm 100:nm
/// let args = vec!["100"];
/// assert_eq!(parse_particles(args), 100)
/// ```
fn parse_particles(arguments: Vec<&str>) -> Result<usize, BibberParseError> {
    let [particles] = parse_arguments(arguments)?;
    Ok(particles.parse::<f64>()? as usize)
}

/// Parse specification of periodic boundary conditions.
///
/// # Example
///
/// ```
/// // Line from which args are derived: boundary cubic 100:nm 100:nm 100:nm
/// let args = vec!["cubic", "100:nm", "100:nm", "100:nm"];
/// assert_eq!(parse_boundary(args), Vec3::new(100e-9, 100e-9, 100e-9))
/// ```
fn parse_boundary(arguments: Vec<&str>) -> Result<Vec3, BibberParseError> {
    // TODO: Update parser to accept different periodic boundary shapes once they have been
    // implemented.
    let [_kind, x, y, z] = parse_arguments(arguments)?;
    Ok(Vec3::new(
        parse_length(&x)?,
        parse_length(&y)?,
        parse_length(&z)?,
    ))
}
