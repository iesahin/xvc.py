# TODO: write pytests for xvc file
#
# xvc file --help
#   track     Add file and directories to Xvc
#   hash      Get digest hash of files with the supported algorithms
#   recheck   Get files from cache by copy or *link
#   carry-in  Carry (commit) changed files to cache
#   copy      Copy from source to another location in the workspace
#   move      Move files to another location in the workspace
#   list      List tracked and untracked elements in the workspace
#   send      Send (push, upload) files to external storages
#   bring     Bring (download, pull, fetch) files from external storages
#   remove    Remove files from Xvc and possibly storages
#   untrack   Untrack (delete) files from Xvc and possibly storages
#   share     Share a file from S3 compatible storage for a limited time
#   help      Print this message or the help of the given subcommand(s)
#
# Options:
#   -v, --verbose...         Verbosity level. Use multiple times to increase command output detail
#       --quiet              Suppress error messages
#   -C <WORKDIR>             Set the working directory to run the command as if it's in that directory [default: .]
#   -c, --config <CONFIG>    Configuration options set from the command line in the form section.key=value
#       --no-system-config   Ignore system config file
#       --no-user-config     Ignore user config file
#       --no-project-config  Ignore project config (.xvc/config)
#       --no-local-config    Ignore local config (.xvc/config.local)
#       --no-env-config      Ignore configuration options from the environment
#   -h, --help               Print help
#   -V, --version            Print version
