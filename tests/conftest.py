from xvc import Xvc
import pytest
import os

import xvc


@pytest.fixture
def empty_xvc_repo(monkeypatch, tmpdir):
    monkeypatch.chdir(tmpdir)
    os.system("git init")
    xvc = Xvc(verbosity=1)
    xvc.init()
    return xvc


@pytest.fixture
def xvc_repo_with_dir(empty_xvc_repo):
    os.system(
        "xvc-test-helper create-directory-tree --directories 3 --files 3 --seed 42"
    )
    return empty_xvc_repo


@pytest.fixture
def xvc_pipeline_single_step(xvc_repo_with_dir):
    xvc_repo_with_dir.pipeline().step().new(
        step_name="hello", command="echo 'hello xvc'"
    )
    return xvc_repo_with_dir
