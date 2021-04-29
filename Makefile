.PHONY: default

default: deploy

deploy: 
	rsync -avz Cargo.* src fcbuttons.service root@chip.lan:/root/fcbuttons
	ssh root@chip.lan "mpc stop; cd /root/fcbuttons && cp fcbuttons.service /etc/systemd/system && time cargo install --path . && systemctl enable fcbuttons && systemctl restart fcbuttons"
