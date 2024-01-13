use pyo3::prelude::*;

use aws_lambda_events::dynamodb::Event;
use lambda_runtime::{LambdaEvent,service_fn, Error};
use aws_config::{BehaviorVersion,load_defaults};
use tracing::{info,error};
use std::{sync::Arc, env};
use tokio::{runtime::Runtime,task::JoinSet};

#[pyclass]
struct DDBEventRust{
 event:Event,
}

#[pymethods]
impl DDBEventRust{
    #[new]
    
}
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn ecommerce_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
