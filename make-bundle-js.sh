#!/usr/bin/env zsh
TMPDIR=`mktemp -d`
OUTFILE="bundle.js"
pushd $TMPDIR
curl -fsSLO "https://code.jquery.com/jquery-2.2.4.min.js"
curl -fsSLO "https://raw.githubusercontent.com/nohtcoltd/turnbox_js/master/turnBox.js"
uglifyjs jquery-2.2.4.min.js turnBox.js -o $OUTFILE -c -m
popd
cp $TMPDIR/$OUTFILE .
