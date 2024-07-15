import os


def test_xvc_root(empty_xvc_repo):
    print(empty_xvc_repo.root())
    assert ".xvc" in os.listdir(empty_xvc_repo.root())
