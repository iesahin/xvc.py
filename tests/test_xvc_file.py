import os
import shutil


def test_file_hash(xvc_repo_with_dir):
    print(xvc_repo_with_dir.root(absolute=True))
    hash_string = xvc_repo_with_dir.file().hash("dir-0001/file-0002.bin")
    assert hash_string.startswith(
        "6432c99dec9e4a6c208ab78cfb58749ece5090fa8e279e6fd5a8cfd431e053f5"
    )


def test_file_track_symlink(xvc_repo_with_dir):
    assert ".git" in os.listdir()
    assert ".xvc" in os.listdir()

    xvc_repo_with_dir.file().track("dir-0001/file-0001.bin", recheck_method="symlink")
    assert os.path.islink("dir-0001/file-0001.bin")


def test_file_track_copy(xvc_repo_with_dir):
    print(os.listdir())
    xvc_repo_with_dir.file().track("dir-0001/file-0002.bin")
    assert os.path.isfile("dir-0001/file-0001.bin")
    assert os.path.isfile(
        ".xvc/b3/643/2c9/9dec9e4a6c208ab78cfb58749ece5090fa8e279e6fd5a8cfd431e053f5/0.bin"
    )


def test_file_recheck(xvc_repo_with_dir):
    xvc_repo_with_dir.file().track("dir-0001/file-0001.bin", recheck_method="symlink")
    os.remove("dir-0001/file-0001.bin")
    xvc_repo_with_dir.file().recheck("dir-0001/file-0001.bin")
    assert os.path.islink("dir-0001/file-0001.bin")


def test_file_carry_in(xvc_repo_with_dir):
    xvc_repo_with_dir.file().track("dir-0001/file-0001.bin")
    os.remove("dir-0001/file-0001.bin")
    shutil.copy("dir-0001/file-0002.bin", "dir-0001/file-0001.bin")
    xvc_repo_with_dir.file().carry_in("dir-0001/file-0001.bin")
    assert os.path.isfile(
        ".xvc/b3/643/2c9/9dec9e4a6c208ab78cfb58749ece5090fa8e279e6fd5a8cfd431e053f5/0.bin"
    )


def test_file_copy(xvc_repo_with_dir):
    xvc_repo_with_dir.file().track("dir-0001/file-0001.bin")
    os.remove("dir-0001/file-0001.bin")
    xvc_repo_with_dir.file().copy("dir-0001/file-0001.bin", "dir-0001/file-0005.bin")
    assert os.path.isfile("dir-0001/file-0005.bin")


def test_file_move(xvc_repo_with_dir):
    xvc_repo_with_dir.file().track("dir-0001/file-0002.bin", recheck_method="symlink")
    xvc_repo_with_dir.file().mv("dir-0001/file-0002.bin", "dir-0001/file-0005.bin")
    assert os.path.islink("dir-0001/file-0005.bin")
    assert not os.path.islink("dir-0001/file-0002.bin")


def test_file_list(xvc_repo_with_dir):
    file_list = xvc_repo_with_dir.file().list().split("\n")
    assert len([line for line in file_list if line.startswith("FX")]) == 9

    xvc_repo_with_dir.file().track("dir-0001/")
    file_list = xvc_repo_with_dir.file().list().split("\n")
    assert len([line for line in file_list if line.startswith("FC")]) == 3
    assert len([line for line in file_list if line.startswith("FX")]) == 6

    file_list = xvc_repo_with_dir.file().list(show_directories=True).split("\n")
    assert len([line for line in file_list if line.startswith("DX")]) == 3


def test_file_remove(xvc_repo_with_dir):
    xvc_repo_with_dir.file().track("dir-0001/")
    assert len(os.listdir(".xvc/b3/")) == 3

    xvc_repo_with_dir.file().remove("dir-0001/file-0001.bin", from_cache=True)
    assert len(os.listdir(".xvc/b3/")) == 2


def test_file_untrack(xvc_repo_with_dir):
    xvc_repo_with_dir.file().track("dir-0001/")
    assert len(os.listdir(".xvc/b3/")) == 3

    file_list = xvc_repo_with_dir.file().list().split("\n")
    assert len([line for line in file_list if line.startswith("FX")]) == 6
    assert len([line for line in file_list if line.startswith("FC")]) == 3

    xvc_repo_with_dir.file().untrack("dir-0001/file-0001.bin")

    file_list = xvc_repo_with_dir.file().list().split("\n")
    assert len([line for line in file_list if line.startswith("FX")]) == 7
    assert len([line for line in file_list if line.startswith("FC")]) == 2

    assert len(os.listdir(".xvc/b3/")) == 2
