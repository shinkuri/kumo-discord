#!/bin/bash
registry="git.kurumi.at/shinkuri"
image="kumo_discord"
tag="latest"
# Build docker image
docker build -t $registry/$image:$tag .
# Authenticate
docker login $registry
# Push new image to Gitea
docker push $registry/$image:$tag

