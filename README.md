```python
# Xvc
```

This is the Python port of [Xvc](https://github.com/iesahin/xvc). Xvc's goal is let users perform all ML operations regarding data, files, models and pipelines from the command line, and version all of these on top of Git. Xvc.py extends this goal by allowing all commands to be run from Python shells, code and Jupyter notebooks. 

You can install `xvc` to your system or virtual environment with uncommenting the line below. 


```python
# !pip install xvc
```

Import Xvc like any other Python module. 


```python
import xvc
```

This notebook is created with the following version. Please test your issue with the latest version before creating any bug reports. 


```python
xvc.version()
```




    'v0.6.3-46-g71fc283-modified'



Python wrappings for `xvc` command uses `Xvc` class. 


```python
from xvc import Xvc
```

## Tracking files

Let's reset the directory we're in and create a directory tree. These are the data files we want to track. Note that `xvc-test-helper` is a cli application to make life easier for Xvc tests. If you want to reproduce this notebook, you can install it with `cargo install xvc-test-helper`


```python
!chmod -R +w .xvc .xvcignore .git .gitignore test-data/
!rm -rf .xvc .xvcignore .git .gitignore test-data/
!xvc-test-helper create-directory-tree --directories 3 --files 3 --root test-data --seed 20240101
```

    chmod: .xvc: No such file or directory
    chmod: .git: No such file or directory
    chmod: .gitignore: No such file or directory
    chmod: test-data/: No such file or directory


Above command created a directory structure as this. 


```python
!tree
```

    [1;36m.[0m
    ├── Debug.ipynb
    ├── Readme.ipynb
    ├── default-pipeline.json
    ├── dr-iq-scores.csv
    ├── generate_data.py
    ├── random_names_iq_scores.csv
    ├── requirements.txt
    ├── [1;36mtest-data[0m
    │   ├── [1;36mdir-0001[0m
    │   │   ├── file-0001.bin
    │   │   ├── file-0002.bin
    │   │   └── file-0003.bin
    │   ├── [1;36mdir-0002[0m
    │   │   ├── file-0001.bin
    │   │   ├── file-0002.bin
    │   │   └── file-0003.bin
    │   └── [1;36mdir-0003[0m
    │       ├── file-0001.bin
    │       ├── file-0002.bin
    │       └── file-0003.bin
    └── visualize.py
    
    5 directories, 17 files


Now we can initialize Xvc repository. We first init Git in the directory. Xvc tracks binary files with their metadata (size, timestamp and digest) and this metadata is kept in text files. You can use Xvc without Git, but adding versioning to your data is usually a good idea. Xvc manages Git operations on the text files it creates automatically by default, so it won't add any extra commands to the workflow.


```python
!git init
```

    Initialized empty Git repository in /Users/iex/github.com/iesahin/xvc-notebooks/Readme/.git/


Now we can init the Xvc repository

First we create an Xvc instance. You can specify the workdir and this instance
will work on that directory. For now, only one instance of Xvc can run in a
Python process. This limitation may be removed in future releases.

```python

xvc_test_data = Xvc()
```

Let's initialize Xvc in the directory. This will  create a `.xvc` directory, will put several initial files and also create a `.gitignore` entry for those elements that shouldn't be tracked by Git. 


```python
xvc_test_data.init()
```




    ''



We can see which Xvc repository a directory belongs with `root` command. By default, it returns the relative directory but you can set `absolute` flag to get see the absolute dir. 


```python
xvc_test_data.root(absolute=True)
```

    /Users/iex/github.com/iesahin/xvc-notebooks/Readme





    ''



Let's add a set of files to Xvc. 


```python
xvc_test_data.file().track("test-data/dir-0001/")
```




    ''



If there is no output, it (usually) means the command is successful. If you want more output, you can set verbosity level when creating Xvc object. 

Let's get the list of files in the workspace. 


```python
xvc_test_data.file().list()
```




    'FX         142 2024-06-15 16:11:50          2f4850fa visualize.py\nFX        2003 2024-06-16 09:49:47          41e16be7 test-data/dir-0003/file-0003.bin\nFX        2002 2024-06-16 09:49:47          27f0efd0 test-data/dir-0003/file-0002.bin\nFX        2001 2024-06-16 09:49:47          66de5084 test-data/dir-0003/file-0001.bin\nDX         160 2024-06-16 09:49:47                   test-data/dir-0003\nFX        2003 2024-06-16 09:49:47          41e16be7 test-data/dir-0002/file-0003.bin\nFX        2002 2024-06-16 09:49:47          27f0efd0 test-data/dir-0002/file-0002.bin\nFX        2001 2024-06-16 09:49:47          66de5084 test-data/dir-0002/file-0001.bin\nDX         160 2024-06-16 09:49:47                   test-data/dir-0002\nFC        2003 2024-06-16 09:49:47 41e16be7 41e16be7 test-data/dir-0001/file-0003.bin\nFC        2002 2024-06-16 09:49:47 27f0efd0 27f0efd0 test-data/dir-0001/file-0002.bin\nFC        2001 2024-06-16 09:49:47 66de5084 66de5084 test-data/dir-0001/file-0001.bin\nDX         160 2024-06-16 09:49:48                   test-data/dir-0001\nFX          85 2024-06-16 09:49:48          30842230 test-data/.gitignore\nDX         192 2024-06-16 09:49:48                   test-data\nFX          14 2024-06-15 16:11:50          2b56f46f requirements.txt\nFX       19021 2024-06-16 09:44:16          95dff305 random_names_iq_scores.csv\nFX         629 2024-06-15 16:11:50          1eecb98e generate_data.py\nFX           8 2024-06-16 09:46:37          af1349b9 dr-iq-scores.csv\nFX        2183 2024-06-16 09:46:46          35763b4e default-pipeline.json\nFX       85102 2024-06-16 09:48:12          396cd3fd Readme.ipynb\nFX       30380 2024-06-10 08:25:47          a4864bca Debug.ipynb\nTotal #: 22 Workspace Size:      156254 Cached Size:        6006\n'



Note that Xvc commands return Python strings as output. You can print and process the output as any other string. 


```python
xvc_test_data_list = xvc_test_data.file().list() 
print(xvc_test_data_list)
```

    FX         142 2024-06-15 16:11:50          2f4850fa visualize.py
    FX        2003 2024-06-16 09:49:47          41e16be7 test-data/dir-0003/file-0003.bin
    FX        2002 2024-06-16 09:49:47          27f0efd0 test-data/dir-0003/file-0002.bin
    FX        2001 2024-06-16 09:49:47          66de5084 test-data/dir-0003/file-0001.bin
    DX         160 2024-06-16 09:49:47                   test-data/dir-0003
    FX        2003 2024-06-16 09:49:47          41e16be7 test-data/dir-0002/file-0003.bin
    FX        2002 2024-06-16 09:49:47          27f0efd0 test-data/dir-0002/file-0002.bin
    FX        2001 2024-06-16 09:49:47          66de5084 test-data/dir-0002/file-0001.bin
    DX         160 2024-06-16 09:49:47                   test-data/dir-0002
    FC        2003 2024-06-16 09:49:47 41e16be7 41e16be7 test-data/dir-0001/file-0003.bin
    FC        2002 2024-06-16 09:49:47 27f0efd0 27f0efd0 test-data/dir-0001/file-0002.bin
    FC        2001 2024-06-16 09:49:47 66de5084 66de5084 test-data/dir-0001/file-0001.bin
    DX         160 2024-06-16 09:49:48                   test-data/dir-0001
    FX          85 2024-06-16 09:49:48          30842230 test-data/.gitignore
    DX         192 2024-06-16 09:49:48                   test-data
    FX          14 2024-06-15 16:11:50          2b56f46f requirements.txt
    FX       19021 2024-06-16 09:44:16          95dff305 random_names_iq_scores.csv
    FX         629 2024-06-15 16:11:50          1eecb98e generate_data.py
    FX           8 2024-06-16 09:46:37          af1349b9 dr-iq-scores.csv
    FX        2183 2024-06-16 09:46:46          35763b4e default-pipeline.json
    FX       85102 2024-06-16 09:48:12          396cd3fd Readme.ipynb
    FX       30380 2024-06-10 08:25:47          a4864bca Debug.ipynb
    Total #: 22 Workspace Size:      156254 Cached Size:        6006
    


`xvc.file().list()` returns the list of all files and directories recursively. Its default formatting shows whether a file `F` is tracked and commit (`C`) and its hash values in the workspace and the cache. You can, for example, see files not tracked by processing the output string. 


```python
[l for l in xvc_test_data_list.split("\n") if l.startswith("FX")]
```




    ['FX         142 2024-06-15 16:11:50          2f4850fa visualize.py',
     'FX        2003 2024-06-16 09:49:47          41e16be7 test-data/dir-0003/file-0003.bin',
     'FX        2002 2024-06-16 09:49:47          27f0efd0 test-data/dir-0003/file-0002.bin',
     'FX        2001 2024-06-16 09:49:47          66de5084 test-data/dir-0003/file-0001.bin',
     'FX        2003 2024-06-16 09:49:47          41e16be7 test-data/dir-0002/file-0003.bin',
     'FX        2002 2024-06-16 09:49:47          27f0efd0 test-data/dir-0002/file-0002.bin',
     'FX        2001 2024-06-16 09:49:47          66de5084 test-data/dir-0002/file-0001.bin',
     'FX          85 2024-06-16 09:49:48          30842230 test-data/.gitignore',
     'FX          14 2024-06-15 16:11:50          2b56f46f requirements.txt',
     'FX       19021 2024-06-16 09:44:16          95dff305 random_names_iq_scores.csv',
     'FX         629 2024-06-15 16:11:50          1eecb98e generate_data.py',
     'FX           8 2024-06-16 09:46:37          af1349b9 dr-iq-scores.csv',
     'FX        2183 2024-06-16 09:46:46          35763b4e default-pipeline.json',
     'FX       85102 2024-06-16 09:48:12          396cd3fd Readme.ipynb',
     'FX       30380 2024-06-10 08:25:47          a4864bca Debug.ipynb']



It's also possible to specify the format string to `file().list()` with `format` argument. For example the following shows the size of all workspace files. You can see all format elements in the [`xvc file list` reference](https://docs.xvc.dev/ref/xvc-file-list). 


```python
print(xvc_test_data.file().list(format="{{name}}: {{asz}}"))
```

    visualize.py:         142
    test-data/dir-0003/file-0003.bin:        2003
    test-data/dir-0003/file-0002.bin:        2002
    test-data/dir-0003/file-0001.bin:        2001
    test-data/dir-0003:         160
    test-data/dir-0002/file-0003.bin:        2003
    test-data/dir-0002/file-0002.bin:        2002
    test-data/dir-0002/file-0001.bin:        2001
    test-data/dir-0002:         160
    test-data/dir-0001/file-0003.bin:        2003
    test-data/dir-0001/file-0002.bin:        2002
    test-data/dir-0001/file-0001.bin:        2001
    test-data/dir-0001:         160
    test-data/.gitignore:          85
    test-data:         192
    requirements.txt:          14
    random_names_iq_scores.csv:       19021
    generate_data.py:         629
    dr-iq-scores.csv:           8
    default-pipeline.json:        2183
    Readme.ipynb:       85102
    Debug.ipynb:       30380
    Total #: 22 Workspace Size:      156254 Cached Size:        6006
    


By default Xvc adds files to the cache and _copies_ them to the workspace. For data files that you only need to read, you can specify to use symlinks to track the files. This way, there will only one copy of the file in the cache and a read-only symlink will be placed in the workspace. 


```python
xvc_test_data.file().track("test-data/dir-0002/", recheck_method="symlink")
```




    ''




```python
print(xvc_test_data.file().list("test-data/dir-0002"))
```

    SS         131 2024-06-16 09:49:51 41e16be7          test-data/dir-0002/file-0003.bin
    SS         131 2024-06-16 09:49:51 27f0efd0          test-data/dir-0002/file-0002.bin
    SS         131 2024-06-16 09:49:51 66de5084          test-data/dir-0002/file-0001.bin
    Total #: 3 Workspace Size:         393 Cached Size:        6006
    


Note that the workspace size and cached size are different. The listed workspace size is only the size of symbolic links. Also the initials (symlink as recheck type and symlink in the workspace) is different the files in `test-data/dir-0001/`


```python
!ls -l test-data/dir-000[12]/
```

    
    test-data/dir-0001/:
    .[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;6m[39m [38;5;230miex[39m [38;5;187mstaff[39m [38;5;229m2.0[39m [38;5;229mKB[39m [38;5;40mSun Jun 16 12:49:47 2024[39m  file-0001.bin
    .[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;6m[39m [38;5;230miex[39m [38;5;187mstaff[39m [38;5;229m2.0[39m [38;5;229mKB[39m [38;5;40mSun Jun 16 12:49:47 2024[39m  file-0002.bin
    .[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;245m-[39m[38;5;6m[39m [38;5;230miex[39m [38;5;187mstaff[39m [38;5;229m2.0[39m [38;5;229mKB[39m [38;5;40mSun Jun 16 12:49:47 2024[39m  file-0003.bin
    
    test-data/dir-0002/:
    [38;5;5ml[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;1mx[39m[38;5;2mr[39m[38;5;245m-[39m[38;5;1mx[39m[38;5;2mr[39m[38;5;245m-[39m[38;5;1mx[39m[38;5;6m[39m [38;5;230miex[39m [38;5;187mstaff[39m [38;5;229m131[39m [38;5;229mB[39m [38;5;40mSun Jun 16 12:49:51 2024[39m [38;5;5m file-0001.bin[39m ⇒ [38;5;5m/Users/iex/github.com/iesahin/xvc-notebooks/Readme/.xvc/b3/66d/e50/84c7bbab22aa25db35e7e28c16953764cc9036d1782a58444eb3b9394b/0.bin[39m
    [38;5;5ml[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;1mx[39m[38;5;2mr[39m[38;5;245m-[39m[38;5;1mx[39m[38;5;2mr[39m[38;5;245m-[39m[38;5;1mx[39m[38;5;6m[39m [38;5;230miex[39m [38;5;187mstaff[39m [38;5;229m131[39m [38;5;229mB[39m [38;5;40mSun Jun 16 12:49:51 2024[39m [38;5;5m file-0002.bin[39m ⇒ [38;5;5m/Users/iex/github.com/iesahin/xvc-notebooks/Readme/.xvc/b3/27f/0ef/d00cd3ab2c9d9cd753b0c3a3bd10b79bf0ffc7719fac44218e8130a071/0.bin[39m
    [38;5;5ml[39m[38;5;2mr[39m[38;5;3mw[39m[38;5;1mx[39m[38;5;2mr[39m[38;5;245m-[39m[38;5;1mx[39m[38;5;2mr[39m[38;5;245m-[39m[38;5;1mx[39m[38;5;6m[39m [38;5;230miex[39m [38;5;187mstaff[39m [38;5;229m131[39m [38;5;229mB[39m [38;5;40mSun Jun 16 12:49:51 2024[39m [38;5;5m file-0003.bin[39m ⇒ [38;5;5m/Users/iex/github.com/iesahin/xvc-notebooks/Readme/.xvc/b3/41e/16b/e7387698c97b703e7607c649498594840cd8bf37295ef430de22ac72c1/0.bin[39m


Xvc allows to track files with hardlinks and reflinks (where supported) as well. You can keep your model files as copies and data files as links. Files in the cache are read-only, so if you don't change their permissions manually, your code cannot write on them.

You can also change their [recheck (checkout) methods](https://docs.xvc.dev/ref/xvc-file-recheck/) after you add them to Xvc. So, if you change your mind and want to start to track `test-data/dir-0001` as hardlinks, you can do so. 


```python
xvc_test_data.file().recheck("test-data/dir-0001/", recheck_method="hardlink")
```




    ''




```python
print(xvc_test_data.file().list("test-data/dir-0001"))
```

    FH        2003 2024-06-16 09:49:47 41e16be7 41e16be7 test-data/dir-0001/file-0003.bin
    FH        2002 2024-06-16 09:49:47 27f0efd0 27f0efd0 test-data/dir-0001/file-0002.bin
    FH        2001 2024-06-16 09:49:47 66de5084 66de5084 test-data/dir-0001/file-0001.bin
    Total #: 3 Workspace Size:        6006 Cached Size:        6006
    


Note that the filesystem doesn't make a distinction between hardlinks and files, they all have the same attributes. So the file sizes are identical between in the workspace and the cache. In the second column you can see that these are tracked as `H`ardlinks by Xvc. 

If the files are tracked by Xvc, they are also normally in the cache. You can delete them and get them back from the cache when they are required. 


```python
!rm -rf test-data/dir-0002/
```


```python
!tree
```

    [1;36m.[0m
    ├── Debug.ipynb
    ├── Readme.ipynb
    ├── default-pipeline.json
    ├── dr-iq-scores.csv
    ├── generate_data.py
    ├── random_names_iq_scores.csv
    ├── requirements.txt
    ├── [1;36mtest-data[0m
    │   ├── [1;36mdir-0001[0m
    │   │   ├── file-0001.bin
    │   │   ├── file-0002.bin
    │   │   └── file-0003.bin
    │   └── [1;36mdir-0003[0m
    │       ├── file-0001.bin
    │       ├── file-0002.bin
    │       └── file-0003.bin
    └── visualize.py
    
    4 directories, 14 files



```python
xvc_test_data.file().recheck("test-data/dir-0002/")
```




    ''




```python
print(xvc_test_data.file().list("test-data/dir-0002"))
```

    SS         131 2024-06-16 09:49:53 41e16be7          test-data/dir-0002/file-0003.bin
    SS         131 2024-06-16 09:49:53 27f0efd0          test-data/dir-0002/file-0002.bin
    SS         131 2024-06-16 09:49:53 66de5084          test-data/dir-0002/file-0001.bin
    Total #: 3 Workspace Size:         393 Cached Size:        6006
    


Note that recheck (checkout) methods for files are also tracked and you don't have to specify them every time. 

## Sharing files

Xvc can copy cached files to [S3](https://docs.xvc.dev/ref/xvc-storage-new-s3) compatible servers like [MinIO](https://docs.xvc.dev/ref/xvc-storage-new-minio) or [Google Cloud Storage](https://docs.xvc.dev/ref/xvc-storage-new-gcs); [locally accessible](https://docs.xvc.dev/ref/xvc-storage-new-local) paths like NFS or external drives; [SSH servers with Rsync](https://docs.xvc.dev/ref/xvc-storage-new-rsync) and via [external commands](https://docs.xvc.dev/ref/xvc-storage-new-generic) like rclone or s5cmd. For the purposes of this introduction, we are going to use S3.  

Note that Xvc doesn't store any credentials and rely on environment variables or existing SSH connections. 

Let's start by configuring the S3 connection that we'll use. 


```python
xvc_test_data.storage().new_s3(name="backup", bucket_name="xvc-test", region="eu-central-1", storage_prefix="xvc-storage")
```




    ''



You can now send the files you track to this storage.


```python
xvc_test_data.file().send(storage="backup")

```




    ''



When you (or someone else) want to access these files later, you can clone the Git repository and get the files from the
storage.

```shell
$ git clone https://example.com/my-machine-learning-project
Cloning into 'my-machine-learning-project'...
```



```python
xvc_test_data.file().bring("test-data/", storage="backup")
```




    ''



You can also share the files with others for a period if your storage type supports signed URLs. The default period is 24 hours. 


```python
xvc_test_data.file().share("test-data/dir-0001/file-0001.bin", storage="backup", period="1h")
```




    'https://xvc-test.s3.eu-central-1.amazonaws.com/xvc-test/xvc-storage/3c7f8146-df25-4c22-b0e6-bb20d600acaf/b3/66d/e50/84c7bbab22aa25db35e7e28c16953764cc9036d1782a58444eb3b9394b/0.bin?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=AKIAJOXNDTB55QE3GOEQ%2F20240616%2Feu-central-1%2Fs3%2Faws4_request&X-Amz-Date=20240616T094957Z&X-Amz-Expires=86400&X-Amz-SignedHeaders=host&X-Amz-Signature=d82f59453c95abc9421bac2a12605373708b678c6a1e48d8b2734eb34bd22942'



This approach ensures convenient access to files from the shared storage when needed. You can share the URL with anyone and they will be able to download your file within this period. 



You don't have to reconfigure the storage after cloning, but you need to have valid credentials as environment variables
to access the storage.

Xvc never stores any credentials.


# Pipelines

If you have commands that depend on data or code elements, you can configure a pipeline.



For this example, we'll use [a Python script](https://github.com/iesahin/xvc/blob/main/workflow_tests/templates/README.in/generate_data.py) to generate a data set with random names with random IQ scores.

The script uses the Faker library and this library must be available where you run the pipeline. To make it repeatable, we start the pipeline by adding a step that creates the virtual environment and installs dependencies.


```python
pipeline = xvc_test_data.pipeline()
```


```python
pipeline.step().new(step_name="install-deps", command="python3 -m pip install -r requirements.txt")
```




    ''



We'll make `install-deps` step to depend on `requirements.txt` file, so when the requirements file changes it will make the step run. 


```python
pipeline.step().dependency(step_name="install-deps", file="requirements.txt")

```




    ''




Xvc allows to create dependencies between pipeline steps. Dependent steps wait for dependencies to finish successfully. 

Now we create a step to run the script and make `install-deps` step a dependency of it. 


```python

```


```python
pipeline.step().new(step_name="generate-data", command="python3 generate_data.py")
pipeline.step().dependency(step_name="generate-data", step="install-deps")
```




    ''



After you define the pipeline, you can run it by:


```python
print(pipeline.run())

```

    [OUT] [install-deps] Requirement already satisfied: faker in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from -r requirements.txt (line 1)) (25.8.0)
    Requirement already satisfied: seaborn in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from -r requirements.txt (line 2)) (0.13.2)
    Requirement already satisfied: python-dateutil>=2.4 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from faker->-r requirements.txt (line 1)) (2.9.0.post0)
    Requirement already satisfied: numpy!=1.24.0,>=1.20 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from seaborn->-r requirements.txt (line 2)) (1.26.4)
    Requirement already satisfied: pandas>=1.2 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from seaborn->-r requirements.txt (line 2)) (2.2.2)
    Requirement already satisfied: matplotlib!=3.6.1,>=3.4 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from seaborn->-r requirements.txt (line 2)) (3.9.0)
    Requirement already satisfied: contourpy>=1.0.1 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from matplotlib!=3.6.1,>=3.4->seaborn->-r requirements.txt (line 2)) (1.2.1)
    Requirement already satisfied: cycler>=0.10 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from matplotlib!=3.6.1,>=3.4->seaborn->-r requirements.txt (line 2)) (0.12.1)
    Requirement already satisfied: fonttools>=4.22.0 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from matplotlib!=3.6.1,>=3.4->seaborn->-r requirements.txt (line 2)) (4.53.0)
    Requirement already satisfied: kiwisolver>=1.3.1 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from matplotlib!=3.6.1,>=3.4->seaborn->-r requirements.txt (line 2)) (1.4.5)
    Requirement already satisfied: packaging>=20.0 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from matplotlib!=3.6.1,>=3.4->seaborn->-r requirements.txt (line 2)) (24.0)
    Requirement already satisfied: pillow>=8 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from matplotlib!=3.6.1,>=3.4->seaborn->-r requirements.txt (line 2)) (10.3.0)
    Requirement already satisfied: pyparsing>=2.3.1 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from matplotlib!=3.6.1,>=3.4->seaborn->-r requirements.txt (line 2)) (3.1.2)
    Requirement already satisfied: pytz>=2020.1 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from pandas>=1.2->seaborn->-r requirements.txt (line 2)) (2024.1)
    Requirement already satisfied: tzdata>=2022.7 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from pandas>=1.2->seaborn->-r requirements.txt (line 2)) (2024.1)
    Requirement already satisfied: six>=1.5 in /Users/iex/github.com/iesahin/xvc.py/.venv/lib/python3.12/site-packages (from python-dateutil>=2.4->faker->-r requirements.txt (line 1)) (1.16.0)
     [DONE] install-deps (python3 -m pip install -r requirements.txt)[OUT] [generate-data] CSV file generated successfully.
     [DONE] generate-data (python3 generate_data.py)




Xvc allows many kinds of dependencies, like [files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#file-dependencies), 
[groups of files and directories defined by globs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#glob-dependencies), 
[regular expression searches in files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#regex-dependencies), 
[line ranges in files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#line-dependencies), 
[hyper-parameters defined in YAML, JSON or TOML files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#hyper-parameter-dependencies)
[HTTP URLs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#url-dependencies),
[shell command outputs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#generic-command-dependencies), 
and [other steps](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#step-dependencies). 

> 🔅
> The rest of this document introduces some of these dependencies. **Please do not panic** if you're not into regexes or hyperparameters. 



Suppose you're only interested in the IQ scores of those with _Dr._ in front of their names and how they differ from the rest in the dataset we created. Let's create a regex search dependency to the data file that will show all _doctors_ IQ scores.



```python
pipeline.step().new(step_name="dr-iq", command='echo "${XVC_REGEX_ADDED_ITEMS}" >> dr-iq-scores.csv')
pipeline.step().dependency(step_name="dr-iq", regex_items=r'random_names_iq_scores.csv:/^Dr\..*')

```




    ''





The first line specifies a command, when run writes `${XVC_REGEX_ADDED_ITEMS}` environment variable to `dr-iq-scores.csv` file. 
The second line specifies the dependency which will also populate the `$[XVC_REGEX_ADDED_ITEMS]` environment variable in the command. 



Some dependency types like [regex items], 
[line items] and [glob items] inject environment variables in the commands they are a dependency.
For example, if you have two million files specified with a glob, but want to run a script only on the added files after the last run, you can use these environment variables. 

[regex items]: https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#regex-item-dependencies
[line items]: https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#line-item-dependencies
[glob items]: https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#glob-item-dependencies


When you run the pipeline again, a file named `dr-iq-scores.csv` will be created. Note that, as `requirements.txt` didn't change `install-deps` step and its dependent `generate-data` steps didn't run.



```python
print(pipeline.run())
```

    [DONE] dr-iq (echo "${XVC_REGEX_ADDED_ITEMS}" >> dr-iq-scores.csv)





We are using this feature to get lines starting with `Dr.` from the file and write them to another file. When the file changes, e.g. another record matching the dependency regex added to the `random_names_iq_scores.csv` file, it will also be added to `dr-iq-scores.csv` file.



```python

!zsh -cl 'echo "Dr. Albert Einstein,144" >> random_names_iq_scores.csv'

print(pipeline.run())

```

    [DONE] dr-iq (echo "${XVC_REGEX_ADDED_ITEMS}" >> dr-iq-scores.csv)




Now we want to add a another command that draws a fancy histogram from `dr-iq-scores.csv`. As this new step must wait `dr-iq-scores.csv` file to be ready, we'll define `dr-iq-scores.csv` as an _output_ of `dr-iq` step and set the file as a dependency to this new `visualize` step.



```python
pipeline.step().output(step_name="dr-iq", output_file="dr-iq-scores.csv")

pipeline.step().new(step_name="visualize", command='python3 visualize.py')
pipeline.step().dependency(step_name="visualize", file="dr-iq-scores.csv")

print(pipeline.run())


```

    [DONE] dr-iq (echo "${XVC_REGEX_ADDED_ITEMS}" >> dr-iq-scores.csv)[ERROR] Step visualize finished UNSUCCESSFULLY with command python3 visualize.py



You can get the pipeline in Graphviz DOT or mermaid format to convert to an image.



```python
print(pipeline.dag(format="mermaid"))
```

    flowchart TD
        n0["install-deps"]
        n1["requirements.txt"] --> n0
        n2["generate-data"]
        n0["install-deps"] --> n2
        n3["dr-iq"]
        n4["visualize"]
        n5["dr-iq-scores.csv"] --> n4
    


```mermaid
flowchart TD
    n0["install-deps"]
    n1["requirements.txt"] --> n0
    n2["generate-data"]
    n0["install-deps"] --> n2
    n3["dr-iq"]
    n4["visualize"]
    n5["dr-iq-scores.csv"] --> n4
```

You can also export and import the pipeline to JSON to edit in your editor.


```python
pipeline.export(file="default-pipeline.json")
```




    ''




```python
!cat default-pipeline.json
```

    [38;5;238m───────┬────────────────────────────────────────────────────────────────────────[0m
           [38;5;238m│ [0mFile: [1mdefault-pipeline.json[0m
    [38;5;238m───────┼────────────────────────────────────────────────────────────────────────[0m
    [38;5;238m   1[0m   [38;5;238m│[0m [38;2;73;72;62m{[0m
    [38;5;238m   2[0m   [38;5;238m│[0m [38;2;73;72;62m  [0m[38;2;153;143;47m"[0m[38;2;153;143;47mname[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mdefault[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m   3[0m   [38;5;238m│[0m [38;2;73;72;62m  [0m[38;2;153;143;47m"[0m[38;2;153;143;47msteps[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m
    [38;5;238m   4[0m   [38;5;238m│[0m [38;2;73;72;62m    [0m[38;2;73;72;62m{[0m
    [38;5;238m   5[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mcommand[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mpython3 -m pip install -r requirements.txt[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m   6[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mdependencies[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m
    [38;5;238m   7[0m   [38;5;238m│[0m [38;2;73;72;62m        [0m[38;2;73;72;62m{[0m
    [38;5;238m   8[0m   [38;5;238m│[0m [38;2;73;72;62m          [0m[38;2;153;143;47m"[0m[38;2;153;143;47mFile[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m{[0m
    [38;5;238m   9[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;153;143;47m"[0m[38;2;153;143;47mcontent_digest[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m{[0m
    [38;5;238m  10[0m   [38;5;238m│[0m [38;2;73;72;62m              [0m[38;2;153;143;47m"[0m[38;2;153;143;47malgorithm[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mBlake3[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  11[0m   [38;5;238m│[0m [38;2;73;72;62m              [0m[38;2;153;143;47m"[0m[38;2;153;143;47mdigest[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m
    [38;5;238m  12[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m43[0m[38;2;73;72;62m,[0m
    [38;5;238m  13[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m86[0m[38;2;73;72;62m,[0m
    [38;5;238m  14[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m244[0m[38;2;73;72;62m,[0m
    [38;5;238m  15[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m111[0m[38;2;73;72;62m,[0m
    [38;5;238m  16[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m13[0m[38;2;73;72;62m,[0m
    [38;5;238m  17[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m243[0m[38;2;73;72;62m,[0m
    [38;5;238m  18[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m28[0m[38;2;73;72;62m,[0m
    [38;5;238m  19[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m110[0m[38;2;73;72;62m,[0m
    [38;5;238m  20[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m140[0m[38;2;73;72;62m,[0m
    [38;5;238m  21[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m213[0m[38;2;73;72;62m,[0m
    [38;5;238m  22[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m105[0m[38;2;73;72;62m,[0m
    [38;5;238m  23[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m20[0m[38;2;73;72;62m,[0m
    [38;5;238m  24[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m239[0m[38;2;73;72;62m,[0m
    [38;5;238m  25[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m62[0m[38;2;73;72;62m,[0m
    [38;5;238m  26[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m73[0m[38;2;73;72;62m,[0m
    [38;5;238m  27[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m75[0m[38;2;73;72;62m,[0m
    [38;5;238m  28[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m13[0m[38;2;73;72;62m,[0m
    [38;5;238m  29[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m146[0m[38;2;73;72;62m,[0m
    [38;5;238m  30[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m82[0m[38;2;73;72;62m,[0m
    [38;5;238m  31[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m17[0m[38;2;73;72;62m,[0m
    [38;5;238m  32[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m148[0m[38;2;73;72;62m,[0m
    [38;5;238m  33[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m152[0m[38;2;73;72;62m,[0m
    [38;5;238m  34[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m66[0m[38;2;73;72;62m,[0m
    [38;5;238m  35[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m86[0m[38;2;73;72;62m,[0m
    [38;5;238m  36[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m154[0m[38;2;73;72;62m,[0m
    [38;5;238m  37[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m230[0m[38;2;73;72;62m,[0m
    [38;5;238m  38[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m154[0m[38;2;73;72;62m,[0m
    [38;5;238m  39[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m246[0m[38;2;73;72;62m,[0m
    [38;5;238m  40[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m213[0m[38;2;73;72;62m,[0m
    [38;5;238m  41[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m214[0m[38;2;73;72;62m,[0m
    [38;5;238m  42[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m40[0m[38;2;73;72;62m,[0m
    [38;5;238m  43[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;104;77;153m119[0m
    [38;5;238m  44[0m   [38;5;238m│[0m [38;2;73;72;62m              [0m[38;2;73;72;62m][0m
    [38;5;238m  45[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;73;72;62m}[0m[38;2;73;72;62m,[0m
    [38;5;238m  46[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;153;143;47m"[0m[38;2;153;143;47mpath[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mrequirements.txt[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  47[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;153;143;47m"[0m[38;2;153;143;47mxvc_metadata[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m{[0m
    [38;5;238m  48[0m   [38;5;238m│[0m [38;2;73;72;62m              [0m[38;2;153;143;47m"[0m[38;2;153;143;47mfile_type[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mFile[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  49[0m   [38;5;238m│[0m [38;2;73;72;62m              [0m[38;2;153;143;47m"[0m[38;2;153;143;47mmodified[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m{[0m
    [38;5;238m  50[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;153;143;47m"[0m[38;2;153;143;47mnanos_since_epoch[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;104;77;153m431372139[0m[38;2;73;72;62m,[0m
    [38;5;238m  51[0m   [38;5;238m│[0m [38;2;73;72;62m                [0m[38;2;153;143;47m"[0m[38;2;153;143;47msecs_since_epoch[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;104;77;153m1718467910[0m
    [38;5;238m  52[0m   [38;5;238m│[0m [38;2;73;72;62m              [0m[38;2;73;72;62m}[0m[38;2;73;72;62m,[0m
    [38;5;238m  53[0m   [38;5;238m│[0m [38;2;73;72;62m              [0m[38;2;153;143;47m"[0m[38;2;153;143;47msize[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;104;77;153m14[0m
    [38;5;238m  54[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;73;72;62m}[0m
    [38;5;238m  55[0m   [38;5;238m│[0m [38;2;73;72;62m          [0m[38;2;73;72;62m}[0m
    [38;5;238m  56[0m   [38;5;238m│[0m [38;2;73;72;62m        [0m[38;2;73;72;62m}[0m
    [38;5;238m  57[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;73;72;62m][0m[38;2;73;72;62m,[0m
    [38;5;238m  58[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47minvalidate[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mByDependencies[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  59[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mname[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47minstall-deps[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  60[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47moutputs[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m[38;2;73;72;62m][0m
    [38;5;238m  61[0m   [38;5;238m│[0m [38;2;73;72;62m    [0m[38;2;73;72;62m}[0m[38;2;73;72;62m,[0m
    [38;5;238m  62[0m   [38;5;238m│[0m [38;2;73;72;62m    [0m[38;2;73;72;62m{[0m
    [38;5;238m  63[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mcommand[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mpython3 generate_data.py[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  64[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mdependencies[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m
    [38;5;238m  65[0m   [38;5;238m│[0m [38;2;73;72;62m        [0m[38;2;73;72;62m{[0m
    [38;5;238m  66[0m   [38;5;238m│[0m [38;2;73;72;62m          [0m[38;2;153;143;47m"[0m[38;2;153;143;47mStep[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m{[0m
    [38;5;238m  67[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;153;143;47m"[0m[38;2;153;143;47mname[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47minstall-deps[0m[38;2;153;143;47m"[0m
    [38;5;238m  68[0m   [38;5;238m│[0m [38;2;73;72;62m          [0m[38;2;73;72;62m}[0m
    [38;5;238m  69[0m   [38;5;238m│[0m [38;2;73;72;62m        [0m[38;2;73;72;62m}[0m
    [38;5;238m  70[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;73;72;62m][0m[38;2;73;72;62m,[0m
    [38;5;238m  71[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47minvalidate[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mByDependencies[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  72[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mname[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mgenerate-data[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  73[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47moutputs[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m[38;2;73;72;62m][0m
    [38;5;238m  74[0m   [38;5;238m│[0m [38;2;73;72;62m    [0m[38;2;73;72;62m}[0m[38;2;73;72;62m,[0m
    [38;5;238m  75[0m   [38;5;238m│[0m [38;2;73;72;62m    [0m[38;2;73;72;62m{[0m
    [38;5;238m  76[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mcommand[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mecho [0m[38;2;104;77;153m\"[0m[38;2;153;143;47m${XVC_REGEX_ADDED_ITEMS}[0m[38;2;104;77;153m\"[0m[38;2;153;143;47m >> dr-iq-scores.csv[0m
    [38;5;238m    [0m   [38;5;238m│[0m [38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  77[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mdependencies[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m[38;2;73;72;62m][0m[38;2;73;72;62m,[0m
    [38;5;238m  78[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47minvalidate[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mByDependencies[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  79[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mname[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mdr-iq[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  80[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47moutputs[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m[38;2;73;72;62m][0m
    [38;5;238m  81[0m   [38;5;238m│[0m [38;2;73;72;62m    [0m[38;2;73;72;62m}[0m[38;2;73;72;62m,[0m
    [38;5;238m  82[0m   [38;5;238m│[0m [38;2;73;72;62m    [0m[38;2;73;72;62m{[0m
    [38;5;238m  83[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mcommand[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mpython3 visualize.py[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  84[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mdependencies[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m
    [38;5;238m  85[0m   [38;5;238m│[0m [38;2;73;72;62m        [0m[38;2;73;72;62m{[0m
    [38;5;238m  86[0m   [38;5;238m│[0m [38;2;73;72;62m          [0m[38;2;153;143;47m"[0m[38;2;153;143;47mFile[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m{[0m
    [38;5;238m  87[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;153;143;47m"[0m[38;2;153;143;47mcontent_digest[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;104;77;153mnull[0m[38;2;73;72;62m,[0m
    [38;5;238m  88[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;153;143;47m"[0m[38;2;153;143;47mpath[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mdr-iq-scores.csv[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  89[0m   [38;5;238m│[0m [38;2;73;72;62m            [0m[38;2;153;143;47m"[0m[38;2;153;143;47mxvc_metadata[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;104;77;153mnull[0m
    [38;5;238m  90[0m   [38;5;238m│[0m [38;2;73;72;62m          [0m[38;2;73;72;62m}[0m
    [38;5;238m  91[0m   [38;5;238m│[0m [38;2;73;72;62m        [0m[38;2;73;72;62m}[0m
    [38;5;238m  92[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;73;72;62m][0m[38;2;73;72;62m,[0m
    [38;5;238m  93[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47minvalidate[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mByDependencies[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  94[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47mname[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47mvisualize[0m[38;2;153;143;47m"[0m[38;2;73;72;62m,[0m
    [38;5;238m  95[0m   [38;5;238m│[0m [38;2;73;72;62m      [0m[38;2;153;143;47m"[0m[38;2;153;143;47moutputs[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;73;72;62m[[0m[38;2;73;72;62m][0m
    [38;5;238m  96[0m   [38;5;238m│[0m [38;2;73;72;62m    [0m[38;2;73;72;62m}[0m
    [38;5;238m  97[0m   [38;5;238m│[0m [38;2;73;72;62m  [0m[38;2;73;72;62m][0m[38;2;73;72;62m,[0m
    [38;5;238m  98[0m   [38;5;238m│[0m [38;2;73;72;62m  [0m[38;2;153;143;47m"[0m[38;2;153;143;47mversion[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;104;77;153m1[0m[38;2;73;72;62m,[0m
    [38;5;238m  99[0m   [38;5;238m│[0m [38;2;73;72;62m  [0m[38;2;153;143;47m"[0m[38;2;153;143;47mworkdir[0m[38;2;153;143;47m"[0m[38;2;73;72;62m:[0m[38;2;73;72;62m [0m[38;2;153;143;47m"[0m[38;2;153;143;47m"[0m
    [38;5;238m 100[0m   [38;5;238m│[0m [38;2;73;72;62m}[0m
    [38;5;238m───────┴────────────────────────────────────────────────────────────────────────[0m



You can edit the file (in Jupyter lab or in your editor) to change commands, add new dependencies, etc. and import it back to Xvc with `import_pipeline` method. You can create copies of pipelines by either editing the pipeline name in the file or supplying the name with `name` argument. 


> 🔅 `import_pipeline` is `xvc pipeline import` in command line. Renamed because `import` is a reserved word. 


```python
pipeline.import_pipeline(file="default-pipeline.json", name="another-pipeline")

```




    ''



Now you can list all pipelines and run `another-pipeline` if you want to.


```python
print(pipeline.list())
```

    +------------------+---------+
    | Name             | Run Dir |
    +============================+
    | default          |         |
    |------------------+---------|
    | another-pipeline |         |
    +------------------+---------+




Please create an issue or discussion for any other kinds of dependencies that you'd like to be included.

I'm planning to add [data label and annotations tracking](https://github.com/iesahin/xvc/discussions/208)), [experiments tracking](https://github.com/iesahin/xvc/discussions/207)), [model tracking](https://github.com/iesahin/xvc/discussions/211)), encrypted cache, server to control all commands from a web interface, and more as my time permits.

Please check [`docs.xvc.dev`](https://docs.xvc.dev) for documentation.





## 🤟 Big Thanks

xvc stands on the following (giant) crates:

- [trycmd] is used to run all example commands in this file, [reference, and how-to documentation](https://docs.xvc.dev) at
  every PR. It makes sure that the documentation is always up-to-date and shown commands work as described. We start
  development by writing documentation and implementing them thanks to [trycmd].

- [serde] allows all data structures to be stored in text files. Special thanks from [`xvc-ecs`] for serializing components in an ECS with a single line of code.

- Xvc processes files in parallel with pipelines and parallel iterators thanks to [crossbeam] and [rayon].

- Thanks to [strum], Xvc uses enums extensively and converts almost everything to typed values from strings.

- Xvc has a deep CLI that has subcommands of subcommands (e.g. `xvc storage new s3`), and all these work with minimum bugs thanks to [clap].

- Xvc uses [rust-s3] to connect to S3 and compatible storage services. It employs excellent [tokio] for fast async Rust. These cloud storage features can be turned off thanks to Rust conditional compilation.

- Without implementations of [BLAKE3], BLAKE2, SHA-2 and SHA-3 from Rust [crypto] crate, Xvc couldn't detect file changes so fast.

- Many thanks to small and well built crates, [reflink], [relative-path], [path-absolutize], [glob] for file system and glob handling.

- Thanks to [sad_machine] for providing a State Machine implementation that I used in `xvc pipeline run`. A DAG composed of State Machines made running pipeline steps in parallel with a clean separation of process states.

- Thanks to [thiserror] and [anyhow] for making error handling a breeze. These two crates make me feel I'm doing something good for the humanity when handling errors.

- Xvc is split into many crates and owes this organization to [cargo workspaces].

[crossbeam]: https://docs.rs/crossbeam/
[cargo workspaces]: https://crates.io/crates/cargo-workspaces
[rayon]: https://docs.rs/rayon/
[strum]: https://docs.rs/strum/
[clap]: https://docs.rs/clap/
[serde]: https://serde.rs
[blake3]: https://docs.rs/blake3/
[crypto]: https://docs.rs/rust-crypto/
[reflink]: https://docs.rs/reflink/
[relative-path]: https://docs.rs/relative-path/
[path-absolutize]: https://docs.rs/path-absolutize/
[glob]: https://docs.rs/glob/
[wax]: https://docs.rs/wax/
[trycmd]: https://docs.rs/trycmd/
[sad_machine]: https://docs.rs/sad_machine/
[thiserror]: https://docs.rs/thiserror/
[anyhow]: https://docs.rs/anyhow/
[rust-s3]: https://docs.rs/rust-s3/
[`xvc-ecs`]: https://docs.rs/xvc-ecs/
[tokio]: https://tokio.rs

And, biggest thanks to Rust designers, developers and contributors. Although I can't see myself expert to appreciate it all, it's a fabulous language and environment to work with.


## 🚁 Support

- You can use [Discussions](https://github.com/iesahin/xvc/discussions) to ask questions. I'll answer as much as possible. Thank you.
- I don't follow any other sites regularly. You can also reach me at [emre@xvc.dev](mailto:emre@xvc.dev)



## 👐 Contributing

- Star this repo. I feel very happy for every star and send my best wishes to you. That's a certain win to spend your two seconds for me. Thanks.
- Use xvc. Tell me how it works for you, read the [documentation](https://docs.xvc.dev), [report bugs](https://github.com/iesahin/xvc/issues), [discuss features](https://github.com/iesahin/xvc/discussions).
- Please note that, I don't accept large code PRs. Please open an issue to discuss your idea and write/modify a
  reference page before sending a PR. I'm happy to discuss and help you to implement your idea. Also, it may require a copyright transfer to me, as there may be cases which I provide the code in other licenses. 



## 📜 License

Xvc and these Python bindings are licensed under the [GNU GPL 3.0 License](https://github.com/iesahin/xvc/blob/main/LICENSE). If you want to use the code in your project with other licenses, please contact me.


## 🌦️ Future and Maintenance

I'm using Xvc daily and I'm happy with it. Tracking all my files with Git via arbitrary servers and cloud providers is
something I always need. I'm happy to improve and maintain it as long as I use it.

Given that I'm working on this for the last two years for pure technical bliss, you can expect me to work on it more. 



## ⚠️ Disclaimer

This software is fresh and ambitious. Although I use it and test it close to real-world conditions, it didn't go under
the test of time. **Xvc can eat your files and spit them into the eternal void!** Please take backups.

