#!/usr/bin/bash

login=xsleza26
mkdir $login

cd doc
make
cp $login.pdf ../$login/$login.pdf
make clean
cd ..

cp -r src doc README.md Cargo.toml $login

cd $login
zip -r $login.zip .

cd ..
mv $login/$login.zip .
rm -r $login
