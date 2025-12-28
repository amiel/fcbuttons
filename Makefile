.PHONY: default

default: deploy

deploy:
	rsync -avz Cargo.* src fcbuttons.service fadecandy.service root@chip.local:/root/fcbuttons
	ssh root@chip.local "cd /root/fcbuttons && cp *.service /etc/systemd/system && time cargo install --path ."
	ssh root@chip.local "systemctl enable fadecandy && systemctl enable fcbuttons && systemctl start fadecandy && systemctl restart fcbuttons"

# TODO: install fcserver and fcserver.service
install_deps:
	ssh root@chip.local "apt install -y mpd mpc rsync build-essential git ca-certificates"
	ssh root@chip.local "rustup show || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
	ssh root@chip.local "git clone https://git.approximate.life/fadecandy.git"

install_playlists:
	# TODO

# Copies playlists from chip to here
backup_playlists:
	rsync -avz root@chip.local:/var/lib/mpd/playlists/ playlists

shell:
	ssh root@chip.local
