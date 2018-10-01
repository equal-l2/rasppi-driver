#!/usr/bin/env sh
TMPDIR=`mktemp -d`
OUTFILE="bundle.js"
CURDIR=`pwd`
cd $TMPDIR
curl -fsSLO "https://code.jquery.com/jquery-2.2.4.min.js"
curl -fsSLO "https://raw.githubusercontent.com/nohtcoltd/turnbox_js/master/turnBox.js"
curl -fsSLO "https://cdnjs.cloudflare.com/ajax/libs/howler/2.0.15/howler.min.js"
curl -fsSL "https://cdn.jsdelivr.net/npm/hls.js@latest" -o hls.js
uglifyjs *.js -o $OUTFILE -c -m
cd $CURDIR
cp $TMPDIR/$OUTFILE .
