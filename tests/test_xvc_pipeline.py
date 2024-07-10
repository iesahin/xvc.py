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
