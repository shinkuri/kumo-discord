#!/bin/bash
user = "shinkuri"
# $registry = "git.kurumi.at/${user}"
registry = "ghcr.io/$user"
image="kumo-discord"
tag="latest"
# Build docker image
docker build -t $registry/$image:$tag .
# Authenticate
# Authenticate - Password is taken from input
docker login $registry -u $user --password-stdin
# Push new image to Gitea
docker push $registry/$image:$tag
