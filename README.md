# PyValico

Small python wrapper around the [valico](https://github.com/rustless/valico) rust library to provide fast JSON schema validation.

## Usage

```python
>>> from valico import validate
>>> validate([2,3,4], {"maxItems": 2})
valico.ValidationError: MaxLength condition is not met
```

## Building
To build manylinux wheels for python 3.5, 3.6 and 3.7, follow the instructions at [setuptools-rust](https://pypi.org/project/setuptools-rust/), and build using `build-wheels.sh`
in the project root.


## TODO
* Add tests / travis build
* Benchmark against [jsonschema](https://github.com/Julian/jsonschema)
