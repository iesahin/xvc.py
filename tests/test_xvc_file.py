import os
# TODO: write pytests for xvc file


def test_file_hash(xvc_repo_with_dir):
    hash_string = xvc_repo_with_dir.file().hash("dir-0001/file-0002.bin")
    assert hash_string.starts_with(
        "6432c99dec9e4a6c208ab78cfb58749ece5090fa8e279e6fd5a8cfd431e053f5"
    )


def test_file_track_symlink(xvc_repo_with_dir):
    print(os.listdir())
    xvc_repo_with_dir.file().track("dir-0001/file-0001.bin", recheck_method="symlink")
    assert os.path.islink("dir-0001/file-0001.bin")


# def test_file_recheck(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_carry_in(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_copy(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_move(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_list(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_send(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_bring(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_remove(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_untrack(xvc_repo_with_dir):
#     assert False
#
#
# def test_file_share(xvc_repo_with_dir):
#     assert False
#
#
# #
