# import xvc

# import pytest
import os


# @pytest.fixture
# def empty_xvc_repo(monkeypatch, tmpdir):
#     monkeypatch.chdir(tmpdir)
#     os.system("git init")
#     xvc_repo = xvc.Xvc()
#     xvc_repo.init()
#     return xvc_repo


def test_xvc_root(empty_xvc_repo):
    print(empty_xvc_repo.root())
    assert ".xvc" in os.listdir(empty_xvc_repo.root())
