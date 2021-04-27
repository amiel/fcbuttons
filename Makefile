.PHONY: default

default: deploy

deploy: 
	rsync -avz Cargo.* src root@chip.lan:fcbuttons
	ssh root@chip.lan "tmux send-keys C-c Enter 'mpc stop; cd ~/fcbuttons && cargo run' Enter"
