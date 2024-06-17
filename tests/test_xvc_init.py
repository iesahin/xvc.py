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


def test_xvc_init(empty_xvc_repo):
    assert os.path.exists(".xvc")


def test_xvc_root(empty_xvc_repo):
    assert False
