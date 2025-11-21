use std::io;

use std::io::Write;

use prometheus_parse::Scrape;
use serde_json::to_writer;

mod json_models;
use json_models::PrometheusJson;
use json_models::SampleJson;

pub fn parse<I>(prom_lines: I) -> Result<Scrape, io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    Scrape::parse(prom_lines)
}

pub fn scrape2writer<W>(s: &Scrape, wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let samples = s.samples.iter().map(SampleJson::from).collect();
    let prom_json = PrometheusJson {
        docs: &s.docs,
        samples,
    };
    to_writer(wtr, &prom_json).map_err(io::Error::from)
}

pub fn docs2writer<W>(s: &Scrape, wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    to_writer(wtr, &s.docs).map_err(io::Error::from)
}

pub fn samples2writer<W>(s: &Scrape, wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let samples: Vec<SampleJson> = s.samples.iter().map(SampleJson::from).collect();
    to_writer(wtr, &samples).map_err(io::Error::from)
}
