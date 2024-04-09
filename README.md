# Kumo-Discord

## Rust app
`/ping` on Discord
`/stampy` will output the local time in 8.4h

## Docker image
Run 
- Linux: `sh update-image.sh`
- Windows: `./update-image.ps1`

to build an updated docker image and push it to the configured container repository.  
Make sure to update the script with the desired repository. The script will ask for the container registry password through stdin.

### GitHub container registry
[docs.github.com page about ghcr.io authentication](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#authenticating-with-a-personal-access-token-classic)

Tl;dr example:
```sh
export CR_PAT=YOUR_TOKEN
$ echo $CR_PAT | docker login ghcr.io -u USERNAME --password-stdin
> Login Succeeded
```

## Ansible playbook
Deploy the container using `ansible-playbook -i inventory.yml, -u root -k --extra-vars "discord_token=" kumo-discord.yml`

The parameters `-u` and `-k` are not necessary if key based ssh authentication is used
|segment|explanation|
|-|-|
|`ansible-playbook`|the command|
|`-i inventory.yml,`|use inventory.yml as inventory|
|`-u root`|use root as ssh user|
|`-k`|ask for password interactively|
|`--extra-vars "discord_token="`|insert token after the equal-sign before executing the command|
|`kumo-discord.yml`|the playbook to run|