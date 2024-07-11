from tests.conftest import empty_xvc_repo


def test_pipeline_list(empty_xvc_repo):
    pipeline_table = empty_xvc_repo.pipeline().list()
    expected = """
+---------+---------+
| Name    | Run Dir |
+===================+
| default |         |
+---------+---------+
""".strip()
    assert pipeline_table == expected


def test_pipeline_new(empty_xvc_repo):
    empty_xvc_repo.pipeline().new(pipeline_name="test", workdir="dir-0001")
    pipeline_table = empty_xvc_repo.pipeline().list()
    print(pipeline_table)
    expected = """
+---------+----------+
| Name    | Run Dir  |
+====================+
| default |          |
|---------+----------|
| test    | dir-0001 |
+---------+----------+
""".strip()
    assert pipeline_table == expected


def test_pipeline_update(empty_xvc_repo):
    empty_xvc_repo.pipeline().new(pipeline_name="test")
    empty_xvc_repo.pipeline().update(pipeline_name="test", workdir="dir-0001")
    pipeline_table = empty_xvc_repo.pipeline().list()
    print(pipeline_table)
    expected = """
+---------+----------+
| Name    | Run Dir  |
+====================+
| default |          |
|---------+----------|
| test    | dir-0001 |
+---------+----------+
""".strip()
    assert pipeline_table == expected


def test_pipeline_delete(empty_xvc_repo):
    empty_xvc_repo.pipeline().new(pipeline_name="test")
    empty_xvc_repo.pipeline().delete(pipeline_name="test")
    pipeline_table = empty_xvc_repo.pipeline().list()
    print(pipeline_table)
    expected = """
+---------+---------+
| Name    | Run Dir |
+===================+
| default |         |
+---------+---------+
""".strip()
    assert pipeline_table == expected


def test_pipeline_step_list(empty_xvc_repo):
    empty_xvc_repo.pipeline().step().new(step_name="hello", command="echo 'hello xvc'")
    pipeline_steps = empty_xvc_repo.pipeline().step().list(names_only=True)
    assert pipeline_steps.strip() == "hello"


def test_pipeline_step_new(empty_xvc_repo):
    empty_xvc_repo.pipeline().step().new(step_name="hello", command="echo 'hello xvc'")
    pipeline_steps = empty_xvc_repo.pipeline().step().list()
    assert pipeline_steps.strip() == "hello: echo 'hello xvc' (by_dependencies)"


def test_pipeline_step_update(empty_xvc_repo):
    empty_xvc_repo.pipeline().step().new(step_name="hello", command="echo 'hello xvc'")
    empty_xvc_repo.pipeline().step().update(
        step_name="hello", command="echo 'hello world'", when="always"
    )
    pipeline_steps = empty_xvc_repo.pipeline().step().list()
    assert pipeline_steps.strip() == "hello: echo 'hello world' (always)"


# TODO: def test_pipeline_step_dependency(xvc_repo_with_dir):
#     assert False
#
# TODO: def test_pipeline_step_dependency_file(xvc_repo_with_dir):
# assert False
# TODO: def test_pipeline_step_dependency_url(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_glob(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_glob-items(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_step(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_param(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_regex(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_regex-items(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_line(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_line-items(xvc_repo_with_dir):
#   assert False
# TODO: def test_pipeline_step_dependency_generic(xvc_repo_with_dir):
#   assert False
#
# TODO: def test_pipeline_dag(xvc_repo_with_dir):
#     assert False
#
#
# TODO: def test_pipeline_export(xvc_repo_with_dir):
#     assert False
#
#
# TODO: def test_pipeline_import(xvc_repo_with_dir):
#     assert False
#
