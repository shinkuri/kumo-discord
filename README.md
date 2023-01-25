# Kumo-Discord

## Rust app
`/ping` on Discord

## Docker image
`./update-package.sh` will use cross-rs to compile the rust project for a x86_64 linux target, build an updated docker image and push it to Gitea

## Ansible playbook
Deploy the container using `ansible-playbook -i "localhost," -c local playbook.yml`