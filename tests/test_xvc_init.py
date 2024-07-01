from xvc import Xvc
import pytest
import os


@pytest.fixture
def empty_xvc_repo(monkeypatch, tmpdir):
    monkeypatch.chdir(tmpdir)
    os.system("git init")
    xvc = Xvc(verbosity=4)
    xvc.init()
    return tmpdir


def test_xvc_init(empty_xvc_repo):
    assert os.path.exists(".xvc")


def test_xvc_root(monkeypatch, empty_xvc_repo):
    monkeypatch.chdir(empty_xvc_repo)
    print(os.getcwd())
    print(os.listdir())
    xvc = Xvc(verbosity=4)
    assert xvc.root() == os.getcwd()


# def test_xvc_root(monkeypatch, tmpdir):
#     monkeypatch.chdir(tmpdir)
#     os.system("git init")
#     xvc = Xvc()
#     xvc.init()
#     assert os.path.exists(".xvc")
#     assert str(xvc.root(absolute=True)) == str(tmpdir.realpath())
