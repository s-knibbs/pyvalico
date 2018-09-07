#[macro_use] extern crate cpython;
extern crate cpython_json;
extern crate valico as valico_rs;

use cpython::{PyResult, Python, PyObject, PyErr};
use cpython_json::to_json;
use valico_rs::{json_schema, common};

py_exception!(valico, ValidationError);

py_module_initializer!(valico, initvalico, PyInit_valico, |py, m| {
    try!(m.add(py, "validate", py_fn!(py, validate(object: PyObject, schema: PyObject))));
    try!(m.add(py, "ValidationError", py.get_type::<ValidationError>()));
    Ok(())
});

fn schema_err_to_pyerr(py: Python, err: json_schema::SchemaError) -> PyErr {
    PyErr::new::<ValidationError, _>(py, &format!("Invalid schema: {:?}", err))
}

fn valico_err_to_string(err: &Box<common::error::ValicoError>) -> String {
    let title = err.get_title();
    let path = err.get_path();
    if let Some(detail) = err.get_detail() {
        return format!("{}: {} : {}", title, path, detail);
    }
    format!("{}: {}", title, path)
}

fn validate(py: Python, object: PyObject, schema: PyObject) -> PyResult<bool> {
    let schema_value = try!(to_json(py, &schema).map_err(|e| e.to_pyerr(py)));
    let object_value = try!(to_json(py, &object).map_err(|e| e.to_pyerr(py)));
    let mut scope = json_schema::Scope::new();

    let compiled_schema = try!(scope.compile_and_return(schema_value, true).map_err(|e| schema_err_to_pyerr(py, e)));
    let validation = compiled_schema.validate(&object_value);
    if validation.is_strictly_valid() {
        return Ok(true);
    }
    // TODO: Include all other error information
    if validation.errors.len() > 0 {
        return Err(PyErr::new::<ValidationError, _>(py, &valico_err_to_string(&validation.errors[0])));
    }
    Err(PyErr::new::<ValidationError, _>(py, &format!("Missing property: {}", validation.missing[0].to_string())))
}