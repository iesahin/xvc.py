# Python bindings for [Xvc](https://github.com/iesahin/xvc)

![PyPI - Version](https://img.shields.io/pypi/v/xvc)

![PyPI - Wheel](https://img.shields.io/pypi/wheel/xvc)

![PyPI - Python Version](https://img.shields.io/pypi/pyversions/xvc)



## Installation and Usage

```
$ pip install xvc
```

## Using xvc from CLI

It's recommended to use the [binary](https://github.com/iesahin/xvc/releases/)
directly for CLI. This package is created to be used as a Python library. 

## Import Xvc in your Python files

```python
import xvc
```


### Initialize an Xvc repository

Xvc is designed and intended to be run on top of a Git repository. 

```python
import os

os.system("git init")

xvc.init()
```

### File operations

Suppose we have a set of files to track with Xvc.


```python
xvc.test_helper.create_directory_tree(dirs=10, files=10)

xvc.file.track("dir-0001/file-0001.bin")
```
