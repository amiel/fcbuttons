.PHONY: default

default: deploy

deploy:
	rsync -avz Cargo.* src fcbuttons.service root@chip.lan:/root/fcbuttons
	ssh root@chip.lan "cd /root/fcbuttons && cp fcbuttons.service /etc/systemd/system && time cargo install --path . && systemctl enable fcbuttons && systemctl restart fcbuttons"

# TODO: install fcserver and fcserver.service
install_deps:
	ssh root@chip.lan "apt install -y mpd"
