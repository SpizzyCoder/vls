BUILD = cargo build --release
LINUX_DESTINATION = /bin/vls
WINDOWS_DESTINATION = C:\bin\vls.exe

.PHONY: linuxinstall linuxuninstall windowsinstall windowsuninstall

linuxinstall:
	$(BUILD)
	sudo mv target/release/vls $(LINUX_DESTINATION)

linuxuninstall:
	sudo rm $(LINUX_DESTINATION)

windowsinstall:
	$(BUILD)
	move target\release\vls.exe $(WINDOWS_DESTINATION)

windowsuninstall:
	del $(WINDOWS_DESTINATION)
