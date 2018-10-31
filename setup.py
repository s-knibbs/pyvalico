from setuptools import setup
from setuptools_rust import RustExtension, Binding


setup(
    name="PyValico",
    version="0.0.2",
    packages=["valico"],
    rust_extensions=[RustExtension("valico.valico", binding=Binding.RustCPython)],
    setup_requires=["setuptools_rust"],
    zip_safe=False
)
