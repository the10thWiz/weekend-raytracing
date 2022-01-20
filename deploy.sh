#! /bin/sh
#
# deploy.sh
# Copyright (C) 2022 matthew <matthew@WINDOWS-05HIC4F>
#
# Distributed under terms of the MIT license.
#


cargo +nightly run && wsl_run files result.png
