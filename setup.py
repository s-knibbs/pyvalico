from setuptools import setup
from setuptools_rust import RustExtension, Binding


def read(f):
    return open(f, encoding='utf-8').read()


setup(
    name="PyValico",
    version="0.0.2",
    author='Simon Knibbs',
    license='MIT',
    url='https://github.com/s-knibbs/pyvalico',
    author_email='simon.knibbs@gmail.com',
    description='Wrapper around the valico rust library for JSON schema validation',
    long_description=read('README.md'),
    long_description_content_type='text/markdown',
    packages=["valico"],
    rust_extensions=[RustExtension("valico.valico", binding=Binding.RustCPython)],
    setup_requires=["setuptools_rust"],
    zip_safe=False,
    classifiers=[
        'Development Status :: 4 - Beta',
        'Intended Audience :: Developers',
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python',
        'Programming Language :: Python :: 3.5'
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Topic :: Software Development :: Libraries'
    ]
)
