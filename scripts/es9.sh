#!/bin/sh

alsa_out -d hw:ES9 -r 48000 -c 16 -j ES-9_out -q 1 </dev/null >/dev/null 2>/dev/null &
#disown
alsa_in -d hw:ES9 -r 48000 -c 16 -j ES-9_in -q 1 </dev/null >/dev/null 2>/dev/null &
#disown
pkill es4jack
es5jack >/dev/null 2>/dev/null &
sleep 1
jack_connect es-5:out ES-9_out:playback_7

