#!/usr/bin/bash

dir=submit
login=xsleza26
sources=$dir/$login

rm -rf $sources
mkdir -p $sources

cd doc
make clean
cd ..

cargo build -r
cp target/release/futoshiki $sources/futoshiki

cp -r src doc README.md Cargo.toml $sources

cd doc
make
cd ..
cp doc/$login.pdf $dir

cd $dir
zip -9 -r $login.zip $login
# rm -rf $login
