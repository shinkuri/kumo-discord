#!/bin/bash
image="git.kurumi.at/shinkuri/kumo-discord"
# Build docker image
docker build -t $image .
# Push new image to Gitea
docker push $image

