#!/usr/bin/env bash

socat -v -T0.05 tcp-l:8080,reuseaddr,fork system:"echo 'HTTP/1.1 200 OK'; echo 'Connection: close'; echo; cat"
