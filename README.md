# Kumo-Discord

## Rust app
`/ping` on Discord

## Docker image
`./update-image.sh` will build an updated docker image and push it to Gitea

## Ansible playbook
Deploy the container using `ansible-playbook -i inventory.yml, -u root -k --extra-vars "discord_token=" kumo-discord.yml`

|segment|explanation|
|-|-|
|`ansible-playbook`|the command|
|`-i inventory.yml,`|use inventory.yml as inventory|
|`-u root`|use root as ssh user|
|`-k`|ask for password interactively|
|`--extra-vars "discord_token="`|insert token after the equal-sign before executing the command|
|`kumo-discord.yml`|the playbook to run|