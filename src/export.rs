use std::fs::{File, OpenOptions};
use std::io::Write;

use csv::{Error, ByteRecord};
use crate::RaceData;

pub fn sus_csv(data: RaceData) -> Result<(), Error> {
    let mut outfile = File::create("foo.txt")?;
    Ok(())
}
pub fn export_csv(data: RaceData) -> Result<(), Error> {
    let mut outfile = OpenOptions::new()
        .write(true)
        .append(true)
        .open("foo.txt")?;
    let mut wtr = csv::WriterBuilder::new()
        .flexible(true)
        .quote_style(csv::QuoteStyle::Always)
        //.from_writer(&outfile);
        .from_path("foo.txt")?;

    wtr.write_record(&["Format", "MoTec CSV File (Custom)", "", "", "Workbook", ""])?;
    wtr.write_record(&["Venue", &data.track, "", "", "Worksheet", ""])?;
    wtr.write_record(&["Vehicle", &data.vehicle, "", "", "Vehicle Desc", ""])?;
    wtr.write_record(&["Driver", &data.driver, "", "", "Engine ID", ""])?;
    wtr.write_record(&["Device", &data.device, "", "", "", ""])?;
    wtr.write_record(&["Comment", &data.comment.unwrap_or("".to_owned()), "", "", "Session", ""])?;
    wtr.write_record(&["Log Date", &data.date, "", "", "Origin Time", "TODO!", "s"])?;
    wtr.write_record(&["Log Time", &data.time, "", "", "Start Time", "TODO!", "s"])?;
    wtr.write_record(&["Sample Rate", "TODO!", "", "", "End Time", "TODO!", "s"])?;
    wtr.write_record(&["Duration", "TODO!", "", "", "Start Distance", "TODO!", "m"])?;
    wtr.write_record(&["Range", "TODO!", "", "", "End Distance", "TODO!", "m"])?;
    wtr.write_record(&["Beacon Markers", "TODO!", "", "", "", ""])?;
    wtr.write_record(&[""])?;
    wtr.write_record(&[""])?;

    let mut titleslice: Vec<String> = vec!("Time".to_string(), "Distance".to_string());
    for col in data.columns {
        titleslice.push(col.name);
    }
    wtr.serialize(titleslice)?;

    wtr.flush()?;
    Ok(())
}

