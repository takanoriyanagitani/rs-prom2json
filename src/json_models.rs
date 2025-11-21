use serde::Serialize;

#[derive(Serialize)]
pub struct PrometheusJson<'a> {
    pub docs: &'a std::collections::HashMap<String, String>,
    pub samples: Vec<SampleJson<'a>>,
}

#[derive(Serialize)]
pub struct SampleJson<'a> {
    pub metric: &'a str,
    pub value: MyValue,
    pub labels: &'a std::collections::HashMap<String, String>,
    pub timestamp: i64,
}

impl<'a> From<&'a prometheus_parse::Sample> for SampleJson<'a> {
    fn from(sample: &'a prometheus_parse::Sample) -> Self {
        Self {
            metric: &sample.metric,
            value: MyValue::from(&sample.value),
            labels: &sample.labels,
            timestamp: sample.timestamp.timestamp_micros(),
        }
    }
}

#[derive(Serialize)]
pub struct MyHistogramCount {
    pub less_than: f64,
    pub count: f64,
}

impl From<&prometheus_parse::HistogramCount> for MyHistogramCount {
    fn from(hc: &prometheus_parse::HistogramCount) -> Self {
        Self {
            less_than: hc.less_than,
            count: hc.count,
        }
    }
}

#[derive(Serialize)]
pub struct MySummaryCount {
    pub quantile: f64,
    pub count: f64,
}

impl From<&prometheus_parse::SummaryCount> for MySummaryCount {
    fn from(sc: &prometheus_parse::SummaryCount) -> Self {
        Self {
            quantile: sc.quantile,
            count: sc.count,
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)] // This allows deserialization to try variants in order
pub enum MyValue {
    Counter(f64),
    Gauge(f64),
    Histogram(Vec<MyHistogramCount>),
    Summary(Vec<MySummaryCount>),
    Untyped(f64),
}

impl From<&prometheus_parse::Value> for MyValue {
    fn from(value: &prometheus_parse::Value) -> Self {
        match value {
            prometheus_parse::Value::Counter(f) => MyValue::Counter(*f),
            prometheus_parse::Value::Gauge(f) => MyValue::Gauge(*f),
            prometheus_parse::Value::Histogram(hcs) => {
                let my_hcs: Vec<MyHistogramCount> =
                    hcs.iter().map(MyHistogramCount::from).collect();
                MyValue::Histogram(my_hcs)
            }
            prometheus_parse::Value::Summary(scs) => {
                let my_scs: Vec<MySummaryCount> = scs.iter().map(MySummaryCount::from).collect();
                MyValue::Summary(my_scs)
            }
            prometheus_parse::Value::Untyped(f) => MyValue::Untyped(*f),
        }
    }
}
