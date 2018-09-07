# PyValico

Small python wrapper around the [valico](https://github.com/rustless/valico) rust library to provide fast JSON schema validation.

## Usage

```python
>>> from valico import validate
>>> validate([2,3,4], {"maxItems", 2})
valico.ValidationError: MaxLength condition is not met
```

## TODO
* Add tests / travis build
* Benchmark against [jsonschema](https://github.com/Julian/jsonschema)
