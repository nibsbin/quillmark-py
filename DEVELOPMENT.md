# Development Guide

## Package Structure

This is a PyO3/maturin-based Python package with Rust bindings.

```
quillmark-py/
├── Cargo.toml          # Rust package configuration
├── pyproject.toml      # Python package configuration
├── README.md           # Package documentation
├── LICENSE             # Apache 2.0 license
├── src/
│   └── lib.rs         # Rust PyO3 bindings
└── python/
    └── pyquillmark/
        ├── __init__.py    # Python module entry point
        ├── __init__.pyi   # Type stubs
        └── py.typed       # PEP 561 marker file
```

## Building

### Prerequisites

- Rust toolchain (rustc, cargo)
- Python 3.9 or later
- maturin (`pip install maturin`)

### Build Commands

```bash
# Build wheel (binary distribution)
maturin build --release

# Build source distribution
maturin build --sdist

# Build and install in development mode
maturin develop

# Build for specific Python version
maturin build --release -i python3.10
```

## Installation

```bash
# Install from wheel
pip install target/wheels/pyquillmark-*.whl

# Or install in development mode
maturin develop
```

## Usage

```python
import pyquillmark

# Check version
print(pyquillmark.__version__)
```

## Publishing to PyPI

```bash
# Build distributions
maturin build --release --sdist

# Upload to PyPI (requires PyPI credentials)
maturin publish
```

## Type Checking

The package includes type stubs (`__init__.pyi`) and a `py.typed` marker for type checkers like mypy:

```bash
mypy your_script.py
```
