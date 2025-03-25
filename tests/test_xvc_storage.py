import tempfile
import os


# How to set virtual env in lazyvim python
def test_storage_list(xvc_repo_with_dir):
    temp_dir = tempfile.mkdtemp()

    assert os.path.isdir(temp_dir)

    xvc_repo_with_dir.storage().new_local(name="local", path=temp_dir)

    assert "local" in str(xvc_repo_with_dir.storage().list())
    assert temp_dir in str(xvc_repo_with_dir.storage().list())


def test_storage_remove(xvc_repo_with_dir):
    # Create a temporary directory to be used with local storage
    temp_dir = tempfile.mkdtemp()

    assert os.path.isdir(temp_dir)

    xvc_repo_with_dir.storage().new_local(name="my-local-storage", path=temp_dir)

    assert "local" in str(xvc_repo_with_dir.storage().list())
    assert temp_dir in str(xvc_repo_with_dir.storage().list())

    xvc_repo_with_dir.storage().remove("my-local-storage")

    assert "local" not in str(xvc_repo_with_dir.storage().list())
    assert temp_dir not in str(xvc_repo_with_dir.storage().list())


#
#
#
# def test_storage_new(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_local(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_generic(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_rsync(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_s3(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_minio(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_digital_ocean(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_r2(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_gcs(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_wasabi(xvc_repo_with_dir):
#     assert False
#
#
# def test_storage_new_help(xvc_repo_with_dir):
#     assert False
#

# def test_file_send(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_bring(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_share(xvc_repo_with_dir):
#     assert False
#
# def test_file_remove_from_storage(xvc_repo_with_dir):
#     assert False
#
