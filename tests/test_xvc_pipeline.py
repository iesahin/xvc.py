import os
import pytest
import yaml
import sqlite3
import time


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


def test_pipeline_step_dependency_file(xvc_pipeline_single_step):
    pipeline = xvc_pipeline_single_step.pipeline()
    dependency_file = "dir-0001/file-0001.bin"
    pipeline.step().dependency(step_name="hello", file=dependency_file)
    first_run = pipeline.run()
    second_run = pipeline.run()
    os.system(f"xvc-test-helper generate-random-file {dependency_file}")
    third_run = pipeline.run()

    assert first_run == third_run
    assert second_run.strip() == ""


def test_pipeline_step_dependency_url(xvc_pipeline_single_step):
    pipeline = xvc_pipeline_single_step.pipeline()
    expected = """
[OUT] [hello] hello xvc
[DONE] [hello] (echo 'hello xvc')
    """.strip()

    pipeline.step().dependency(step_name="hello", url="https://xvc.dev")
    first_run = pipeline.run()
    second_run = pipeline.run()
    print(first_run.strip())
    assert first_run.strip() == expected
    assert second_run.strip() == ""


def test_pipeline_step_dependency_glob(xvc_pipeline_single_step):
    pipeline = xvc_pipeline_single_step.pipeline()
    dependency_file = "dir-0001/new-file.bin"
    pipeline.step().dependency(step_name="hello", glob="dir-0001/*.bin")

    first_run = pipeline.run()
    second_run = pipeline.run()
    os.system(f"xvc-test-helper generate-random-file {dependency_file}")
    third_run = pipeline.run()

    assert first_run == third_run
    assert second_run.strip() == ""


def test_pipeline_step_dependency_glob_items(xvc_repo_with_dir):
    pipeline = xvc_repo_with_dir.pipeline()
    dependency_dir = "dir-0001"
    dependency_file = f"{dependency_dir}/new-file.bin"
    xvc_repo_with_dir.pipeline().step().new(
        step_name="files", command='echo "ADDED_FILES: ${XVC_ADDED_GLOB_ITEMS}"'
    )
    pipeline.step().dependency(step_name="files", glob_items="dir-0001/*.bin")

    first_run = pipeline.run()
    second_run = pipeline.run()
    os.system(f"xvc-test-helper generate-random-file {dependency_file}")
    third_run = pipeline.run()

    print("FIRST RUN")
    print(first_run)
    print("SECOND RUN")
    print(second_run)
    print("THIRD RUN")
    print(third_run)

    assert (
        first_run.strip()
        == """
[OUT] [files] ADDED_FILES: dir-0001/file-0001.bin
dir-0001/file-0002.bin
dir-0001/file-0003.bin
[DONE] [files] (echo "ADDED_FILES: ${XVC_ADDED_GLOB_ITEMS}")
""".strip()
    )

    assert second_run.strip() == ""

    assert (
        third_run.strip()
        == """
[OUT] [files] ADDED_FILES: dir-0001/new-file.bin
[DONE] [files] (echo "ADDED_FILES: ${XVC_ADDED_GLOB_ITEMS}")
""".strip()
    )


def test_pipeline_step_dependency_step(xvc_pipeline_single_step):
    xvc_pipeline_single_step.pipeline().step().new(
        step_name="world", command="echo 'and the world'"
    )
    pipeline = xvc_pipeline_single_step.pipeline()
    pipeline.step().dependency(step_name="world", step="hello")
    first_run = pipeline.run()

    print(first_run)

    assert (
        first_run.strip()
        == """
[OUT] [hello] hello xvc
[DONE] [hello] (echo 'hello xvc')
[OUT] [world] and the world
[DONE] [world] (echo 'and the world')
        """.strip()
    )


def test_pipeline_step_dependency_regex(xvc_repo_with_people_csv):
    pipeline = xvc_repo_with_people_csv.pipeline()
    pipeline.step().new(step_name="a", command='echo "New names starting with A added"')
    pipeline.step().dependency(step_name="a", regex="people.csv:/^A.*$")

    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)
    with open("people.csv", "a") as f:
        f.write("Ali,M,13,74,170\n")
    third_run = pipeline.run()
    print(third_run)

    assert first_run.strip() == third_run.strip()
    assert second_run.strip() == ""


def test_pipeline_step_dependency_regex_items(xvc_repo_with_people_csv):
    pipeline = xvc_repo_with_people_csv.pipeline()
    pipeline.step().new(
        step_name="a", command='echo "Lines with A: ${XVC_ADDED_REGEX_ITEMS}"'
    )
    pipeline.step().dependency(step_name="a", regex_items="people.csv:/^A.*$")
    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)
    with open("people.csv", "a") as f:
        f.write("Ali,M,13,74,170\n")

    third_run = pipeline.run()
    print(third_run)

    assert second_run.strip() == ""

    assert (
        third_run.strip()
        == """
[OUT] [a] Lines with A: Ali,M,13,74,170
[DONE] [a] (echo "Lines with A: ${XVC_ADDED_REGEX_ITEMS}")
""".strip()
    )


def test_pipeline_step_dependency_line(xvc_repo_with_people_csv):
    pipeline = xvc_repo_with_people_csv.pipeline()
    pipeline.step().new(step_name="a", command='echo "New lines added to the file"')
    pipeline.step().dependency(step_name="a", lines="people.csv::10-")

    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)
    with open("people.csv", "a") as f:
        f.write("Ali,M,13,74,170\n")
    third_run = pipeline.run()
    print(third_run)

    assert first_run.strip() == third_run.strip()
    assert second_run.strip() == ""


def test_pipeline_step_dependency_line_items(xvc_repo_with_people_csv):
    pipeline = xvc_repo_with_people_csv.pipeline()
    pipeline.step().new(
        step_name="a", command='echo "Added lines: ${XVC_ADDED_LINE_ITEMS}"'
    )
    pipeline.step().dependency(step_name="a", line_items="people.csv::10-")
    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)
    with open("people.csv", "a") as f:
        f.write("Ali,M,13,74,170\n")

    third_run = pipeline.run()
    print(third_run)

    assert second_run.strip() == ""

    assert (
        third_run.strip()
        == """
[OUT] [a] Added lines: Ali,M,13,74,170
[DONE] [a] (echo "Added lines: ${XVC_ADDED_LINE_ITEMS}")
""".strip()
    )


def test_pipeline_step_dependency_param(empty_xvc_repo):
    filename = "params.yaml"

    with open(filename, "w") as f:
        f.write("""
param: value
database:
  server: example.com
  port: 5432
  connection:
    timeout: 5000
numeric_param: 13
""")
    pipeline = empty_xvc_repo.pipeline()
    pipeline.step().new(
        step_name="read-database-config", command=f"rg timeout {filename}"
    )
    pipeline.step().dependency(
        step_name="read-database-config",
        param=f"{filename}::database.connection.timeout",
    )
    first_run = pipeline.run()
    print(first_run)
    second_run = pipeline.run()
    print(second_run)

    update_yaml(filename, "database.connection.timeout", 10000)
    third_run = pipeline.run()
    assert (
        first_run.strip()
        == """
[OUT] [read-database-config]     timeout: 5000
[DONE] [read-database-config] (rg timeout params.yaml)
""".strip()
    )

    assert second_run.strip() == ""
    assert (
        third_run.strip()
        == """
[OUT] [read-database-config]     timeout: 10000
[DONE] [read-database-config] (rg timeout params.yaml)
""".strip()
    )


def update_yaml(file_path, key, new_value):
    # Read the existing YAML file
    with open(file_path, "r") as file:
        data = yaml.safe_load(file)

    # Update the value
    keys = key.split(".")
    d = data
    for k in keys[:-1]:
        d = d.setdefault(k, {})
    d[keys[-1]] = new_value

    # Write the updated data back to the YAML file
    with open(file_path, "w") as file:
        yaml.safe_dump(data, file)


def test_pipeline_step_dependency_sqlite_query(empty_xvc_repo):
    filename = "people.db"
    db = sqlite3.connect(filename)

    db.execute("""
CREATE TABLE people (name, age, sex);
""")

    db.execute("""
INSERT INTO people VALUES ('Alice', 25, 'F'),
    ('Bob', 30, 'M'),
    ('Charlie', 35, 'M');
""")
    db.commit()

    pipeline = empty_xvc_repo.pipeline()
    pipeline.step().new(
        step_name="query", command=f'sqlite3 {filename} "SELECT AVG(age) FROM people;"'
    )
    pipeline.step().dependency(
        step_name="query",
        sqlite_file=filename,
        sqlite_query="SELECT COUNT(*) FROM people;",
    )
    first_run = pipeline.run()
    print(first_run)
    assert (
        first_run.strip()
        == """
[OUT] [query] 30.0
[DONE] [query] (sqlite3 people.db "SELECT AVG(age) FROM people;")
        """.strip()
    )
    second_run = pipeline.run()
    print(second_run)
    assert second_run.strip() == ""
    # NOTE: We are not changing the average age
    db.execute("INSERT INTO people VALUES ('David', 30, 'M');")
    db.commit()
    third_run = pipeline.run()
    print(third_run)
    assert first_run.strip() == third_run.strip()


# TODO: def test_pipeline_step_dependency_generic(xvc_repo_with_dir):
#   assert False
#
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
