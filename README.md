# Kumo-Discord

## Rust app
`/ping` on Discord

## Docker image
`./update-image.sh` will build an updated docker image and push it to Gitea

## Ansible playbook
Deploy the container using `ansible-playbook -i inventory.yml, -u root -k --extra-vars "discord_token=" playbook.yml`