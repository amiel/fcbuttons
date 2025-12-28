.PHONY: default install-network-recovery install-recover install-watchdog install-service enable-service configure-autoconnect

CHIP_HOST=root@chip.local

REMOTE_BIN_DIR = /usr/local/bin
REMOTE_SERVICE_DIR = /etc/systemd/system

default: deploy

deploy:
	rsync -avz Cargo.* src fcbuttons.service fadecandy.service $(CHIP_HOST):/root/fcbuttons
	ssh $(CHIP_HOST) "cd /root/fcbuttons && cp *.service $(REMOTE_SERVICE_DIR) && time cargo install --path ."
	ssh $(CHIP_HOST) "systemctl enable fadecandy && systemctl enable fcbuttons && systemctl start fadecandy && systemctl restart fcbuttons"

# TODO: install fcserver and fcserver.service
install_deps:
	ssh $(CHIP_HOST) "apt install -y mpd mpc rsync build-essential git ca-certificates"
	ssh $(CHIP_HOST) "rustup show || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
	ssh $(CHIP_HOST) "git clone https://git.approximate.life/fadecandy.git"

install_playlists:
	# TODO

# Copies playlists from chip to here
backup_playlists:
	rsync -avz $(CHIP_HOST):/var/lib/mpd/playlists/ playlists

shell:
	ssh $(CHIP_HOST)

install-network-recovery: configure-autoconnect install-recover install-watchdog 

install-recover:
	scp config/wifi-recover.sh $(CHIP_HOST):$(REMOTE_BIN_DIR)/
	ssh $(CHIP_HOST) "chmod +x $(REMOTE_BIN_DIR)/wifi-recover.sh"

install-watchdog:
	scp config/wifi-watchdog.sh $(CHIP_HOST):$(REMOTE_BIN_DIR)/
	ssh $(CHIP_HOST) "chmod +x $(REMOTE_BIN_DIR)/wifi-watchdog.sh"
	scp config/wifi-watchdog.service $(CHIP_HOST):$(REMOTE_SERVICE_DIR)/wifi-watchdog.service
	ssh $(CHIP_HOST) "systemctl daemon-reload && systemctl enable wifi-watchdog.service"

REMOTE_AUTOCONNECT_DIR=/etc/NetworkManager/conf.d
LOCAL_AUTOCONNECT=config/autoconnect.conf

configure-autoconnect:
	ssh $(CHIP_HOST) "mkdir -p $(REMOTE_AUTOCONNECT_DIR)"
	scp $(LOCAL_AUTOCONNECT) $(CHIP_HOST):$(REMOTE_AUTOCONNECT_DIR)/autoconnect.conf
	ssh $(CHIP_HOST) "chmod 644 $(REMOTE_AUTOCONNECT_DIR)/autoconnect.conf && chown root:root $(REMOTE_AUTOCONNECT_DIR)/autoconnect.conf"
	ssh $(CHIP_HOST) "systemctl restart NetworkManager"
