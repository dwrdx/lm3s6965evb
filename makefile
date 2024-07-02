
all:
	# default build configured in .cargo/config.toml file
	cargo build


build:
	cargo build --target thumbv7m-none-eabi

run:
	@qemu-system-arm \
	-cpu cortex-m3 \
	-machine lm3s6965evb \
	-nographic \
	-semihosting-config enable=on,target=native \
	-kernel target/thumbv7m-none-eabi/debug/lm3s6965evb