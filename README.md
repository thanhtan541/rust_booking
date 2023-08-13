# Linter
clippy
# Hot reload
cargo add cargo-watch
cargo watch -q -c -w src/ -x 'run -q'

# Database
## Install diesel
cargo install diesel_cli --no-default-features --features postgres
## Init diesel
diesel setup
## Generate migration
diesel migration generate posts
## Run or redo the migration
diesel migration run (or redo) 

# Notes
- diesel requires mysqlclient

