.PHONY: linuxinstall linuxuninstall windowsinstall windowsuninstall

linuxinstall:
	cargo build --release
	sudo mv target/release/vls /bin

linuxuninstall:
	sudo rm /bin/vls

windowsinstall:
	cargo build --release
	move target\release\vls.exe C:\bin

windowsuninstall:
	del C:\vls.exe
