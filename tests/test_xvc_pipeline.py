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


# def test_pipeline_new(empty_xvc_repo):
#     assert False
#
# def test_pipeline_update(xvc_repo_with_dir):
#     assert False
#
#
# def test_pipeline_delete(xvc_repo_with_dir):
#     assert False
#
#
# def test_pipeline_run(xvc_repo_with_dir):
#     assert False
#
#
# def test_pipeline_list(xvc_repo_with_dir):
#     assert False
#
#
# def test_pipeline_dag(xvc_repo_with_dir):
#     assert False
#
#
# def test_pipeline_export(xvc_repo_with_dir):
#     assert False
#
#
# def test_pipeline_import(xvc_repo_with_dir):
#     assert False
#
#
# def test_pipeline_step(xvc_repo_with_dir):
#     assert False
