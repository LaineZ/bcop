#!/bin/sh
FILE=sciter-js-sdk-4.4.9.3.zip
BASS_FILE=bass24-linux.zip
BASS_URL=https://www.un4seen.com/files/bass24-linux.zip

if [[ ! -f "$FILE" ]]; then
    curl https://gitlab.com/sciter-engine/sciter-js-sdk/-/archive/4.4.9.3/sciter-js-sdk-4.4.9.3.zip --output $FILE
fi

unamestr=$(uname)
if [[ "$unamestr" == 'Darwin' ]]; then
    BASS_URL=https://www.un4seen.com/files/bass24-osx.zip
fi

if [[ ! -f "$BASS_FILE" ]]; then
    curl $BASS_URL --output $BASS_FILE
fi

unzip $FILE -d .
mkdir -p bass24
unzip $BASS_FILE -d ./bass24