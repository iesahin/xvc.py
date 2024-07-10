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


# TODO: def test_pipeline_export(xvc_repo_with_dir):
#     assert False
#
#
# TODO: def test_pipeline_import(xvc_repo_with_dir):
#     assert False
#
#
# TODO: def test_pipeline_step(xvc_repo_with_dir):
#     assert False
#
# TODO: def test_pipeline_step_new(xvc_repo_with_dir):
#     assert False
#
#
# TODO: def test_pipeline_step_update(xvc_repo_with_dir):
#     assert False
#
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
