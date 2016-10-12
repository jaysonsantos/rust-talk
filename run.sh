#!/bin/sh
set -x
docker build -t rust-talk .
docker run --name rust-postgres -d postgres
if [ -n "$(which cygpath)" ]; then
    current_dir="$(cygpath -w $PWD)"
else
    current_dir=$PWD
fi
sleep 10s
docker run --name rust-talk --rm -v "$current_dir":/work -p3000:3000 --link rust-postgres:postgres rust-talk bash -c '/root/.cargo/bin/diesel migration run ; cargo run'
read
docker stop rust-talk
docker rm rust-talk
docker stop rust-postgres
docker rm rust-postgres
read
