import pytest
from valico import validate, ValidationError

def test_validate_max_items():
    validate([2,3,4], {"maxItems": 3})
    with pytest.raises(ValidationError):
        validate([2,3,4], {"maxItems": 2})

