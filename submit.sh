#!/usr/bin/bash

login=xsleza26
mkdir $login

cd doc
make
cd ..
cp doc/$login.pdf $login

cp -r src doc README.md Cargo.toml $login

cd $login
tar cf $login.tar -- *

cd ..
mv $login/$login.tar .
rm -r $login
