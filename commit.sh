#!/bin/bash
rustfmt src/*
git add * */*
git commit -am "`curl -s http://whatthecommit.com/index.txt`"
git push
