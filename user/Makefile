TARGET := riscv64gc-unknown-none-elf
TESTDIR := ./user_C_program/user
MODE := debug

TARGET := loongarch64-unknown-linux-gnu

ARCH=${shell echo ${TARGET}|cut -d- -f1 |grep -o "^[a-zA-Z]\+[[:digit:]]*"}
ifeq ($(ARCH), loongarch64)
	BOARD ?= laqemu
endif

build: env $(APPS)
ifeq ($(MODE),release)
	@cargo build --target=$(TARGET) --release
else
	@cargo build --target=$(TARGET)
endif

env:
	-(rustup target list | grep "$(TARGET) (installed)") || rustup target add loongarch64-unknown-linux-gnu
	-cargo install --list |grep cargo-binutils |grep "v0.2.0">/dev/null || cargo install cargo-binutils --vers ~0.2
	-rustup component add rust-src
	-rustup component add llvm-tools-preview

clean:
	@cargo clean
	@cd ${TESTDIR} && make clean

.PHONY: build c-user rust-user clean
