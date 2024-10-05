# Expand RGB

A python module that expands RGB region of the RGBA image written in rust.

## Usage

Environment setup:

```sh
uv sync --frozen
source .venv/bin/activate
maturin develop --uv --release
```

Run image processing:

```sh
python src/main.py --input_image <path/to/rgba_image>
```
