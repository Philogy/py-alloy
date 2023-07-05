# Py-Alloy

Python bindings for Alloy's dyn-abi library.

## Test

1. Initialize python env: `python -m venv .env`
2. Active env: `source .env/bin/activate`
3. Install `maturin`: `pip install maturin`
4. Compile python package: `maturin develop` (**Note:** For full performance check the _Running The Benchmark_ instructions)
5. Run simple python example: `python py-test/decode.py`

## Running The Benchmark

1. To get the full performance compile the optimized build: `maturin build -r`
2. This should generate a wheel under `target/wheels` of the repo, install the optimized build into your environment using (wheel file name may be different based on version and OS) `pip install --force-reinstall target/wheels/py_alloy-0.1.0-cp310-cp310-manylinux_2_34_x86_64.whl`
3. Run the benchmark `python py-test/bench.py`
