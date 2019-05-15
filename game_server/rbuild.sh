#!/bin/bash
sh build.sh &> err && cat err | tac
