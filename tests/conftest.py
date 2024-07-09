from xvc import Xvc
import pytest
import os


@pytest.fixture
def empty_xvc_repo(monkeypatch, tmpdir):
    monkeypatch.chdir(tmpdir)
    os.system("git init")
    xvc = Xvc()
    xvc.init()
    return xvc
