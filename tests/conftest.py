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


@pytest.fixture
def xvc_repo_with_people_csv(empty_xvc_repo):
    filename = "people.csv"

    with open(filename, "w") as f:
        f.write("""
Name,Sex,Age,Height,Weight
Alex,M,41,74,170
Bert,M,42,68,166
Carl,M,32,70,155
Dave,M,39,72,167
Elly,F,30,66,124
Fran,F,33,66,115
Gwen,F,26,64,121
Hank,M,30,71,158
Ivan,M,53,72,175
Jake,M,32,69,143
Kate,F,47,69,139
Luke,M,34,72,163
Myra,F,23,62,98
Neil,M,36,75,160
Omar,M,38,70,145
Page,F,31,67,135
Quin,M,29,71,176
Ruth,F,28,65,131
""")

    return empty_xvc_repo
