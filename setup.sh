#!/bin/bash

dir=$(pwd)
cd "$(dirname "$0")"

mkdir bin
mkdir servers
mkdir modules
mkdir -p pelecan/src/{servers,modules}

cd $dir

