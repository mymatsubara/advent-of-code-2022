#!/bin/sh

DAY=$1
DAY_PROJECT="day-$DAY"

if [ -z $DAY ]; then
    echo "usage: $0 <day>"
    return 1
fi

if [ -e $DAY_PROJECT ]; then
    echo "error: '$DAY_PROJECT' already exists"
    return 2
fi

if [ $(cargo new $DAY_PROJECT) ]; then
    echo "Failed to create project: '$DAY_PROJECT'"
else
    cp template/src/* $DAY_PROJECT/src/.
    touch $DAY_PROJECT/README.md
    echo "  ✅ Project '$DAY_PROJECT' successfully created"
fi

