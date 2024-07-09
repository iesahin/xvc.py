import os


def test_xvc_init(empty_xvc_repo):
    assert os.path.exists(".xvc")
