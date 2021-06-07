# Metis API For Python

Python api for metis token components.

## Usage

For Unittest, should install [europa](https://github.com/patractlabs/europa) at first.

```bash
# check europa version
europa --version
```

All of test pased by europa environment.

Install `pytest` and `executor` to run test:

```bash
pip3 install -U patract-interface
pip3 install pytest
pip3 install executor
```

To run test, like erc20:

```bash
pytest ./token/erc20/test --log-cli-level info 
```
