#!/usr/bin/env bash

docker build --tag ci-runner:latest ..
docker run --rm --privileged -v "$(pwd)":/ci-runner/workspace ci-runner:latest
