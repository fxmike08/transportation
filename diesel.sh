#!/bin/bash
# For debug only
#set -o xtrace

echo DATABASE_URL="/tmp/Tansportation.db" > .env
diesel setup
diesel migration run