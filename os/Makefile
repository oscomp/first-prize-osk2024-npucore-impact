# Building
TARGET := loongarch64-unknown-linux-gnu
MODE := debug
CORE_NUM := 1

KERNEL_ELF = target/$(TARGET)/$(MODE)/os
KERNEL_BIN = $(KERNEL_ELF).bin
KERNEL_UIMG = $(KERNEL_ELF).ui

ARCH=${shell echo ${TARGET}|cut -d- -f1 |grep -o "^[a-zA-Z]\+[[:digit:]]*"}
# BOARD
BOARD ?= laqemu
LDBOARD = la2k1000

SDDEV := $(shell lsblk -do NAME | grep sd | tail -1)
SDCARD := /dev/$(SDDEV)
LOG:=trace
# Disassembly
DISASM ?= -x


# img, choose EXT4 or FAT32
IMG_FS ?= EXT4


ifeq ($(BOARD), 2K1000)
	BOARD = 2K1000
endif

# Binutils
OBJCOPY := loongarch64-linux-gnu-objcopy
OBJDUMP := loongarch64-linux-gnu-objdump
READELF := loongarch64-linux-gnu-readelf

ifndef LOG
	LOG_OPTION := "log_off"
endif

IMG_DIR := ../easy-fs-fuse
FAT32_IMG_LA := rootfs-ubifs-ze.img
U_FAT32_DIR := $(IMG_DIR)
FAT32_IMG_NAME := $(FAT32_IMG_LA)
ifeq ($(ARCH), loongarch64)
	FAT32_IMG_NAME = $(FAT32_IMG_LA)
endif

ifeq ($(MODE), debug)
	LA_2K1000_DISABLE_EH_FRAME := -D EH_ENABLED
endif

U_FAT32 := ${U_FAT32_DIR}/$(FAT32_IMG_NAME)	
FAT32_2K1000_LN = $(shell readlink -f $(U_FAT32_DIR))/$(FAT32_IMG_LA)

QEMU_2K1000_DIR=../util/qemu/2k1000
QEMU_2K1000=../util/qemu/runqemu
IMG_DIR=../easy-fs-fuse
U_IMG=$(IMG_DIR)/uImage

LA_DEBUGGER_SERIAL_PORT = $$(python3 -m serial.tools.list_ports 1A86:7523 -q | head -n 1)
LA_DEBUGGER_PORT_FREQ = $(LA_DEBUGGER_SERIAL_PORT) 115200
LA_2K1000_SERIAL_PORT = $$(python3 -m serial.tools.list_ports 067B:2303 -q | head -n 1)
LA_2K1000_PORT_FREQ = $(LA_2K1000_SERIAL_PORT) 115200
MINITERM_START_CMD=python3 -m serial.tools.miniterm --dtr 0 --rts 0 --filter direct 

LA_ENTRY_POINT = 0x9000000090000000
LA_LOAD_ADDR = 0x9000000090000000

RUST_LA_TOOLCHAIN = $(shell rustup show |grep "\(la\|loongarch\)\(64\)\?" |cut -d' ' -f1|head -n 1)


all: user

build: env $(KERNEL_BIN)
env: # switch-toolchain
	-(rustup target list | grep "$(TARGET) (installed)") || rustup target add $(TARGET)
	if [ "$(dpkg --list|grep "ii[[:space:]]*expect")"="" ];then true;else sudo apt install expect;fi


# build all user programs
user: env
	@cd ../user && make build BOARD=$(BOARD) MODE=$(MODE)

sdcard:
    ifeq ($(findstring WSL, $(shell uname -a)), )
		@echo "Are you sure write to $(SDCARD) ? \nYou may also specify another drive with the option \`SDCARD=/dev/<drive>\` [y/N] " && read ans && [ $${ans:-N} = y ]
		./buildfs.sh "$(SDCARD)" "qemu" $(MODE)
    else
		@echo "Are you sure write to E:/ ? [y/N] " && read ans && [ $${ans:-N} = y ]
		@sudo mount $(U_FAT32_DIR)/$(FAT32_IMG_NAME) $(U_FAT32_DIR)/fs
		@powershell.exe 'cd $(U_FAT32_DIR); copy -Recurse -path "$$(pwd)/fs/*" -destination "E:/" -force'
		@sudo umount $(U_FAT32_DIR)/fs
    endif
$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) $@ --strip-all -O binary &
	@$(OBJDUMP) $(KERNEL_ELF) -SC > target/$(TARGET)/$(MODE)/asm_all.txt 
	@$(READELF) -ash $(KERNEL_ELF) > target/$(TARGET)/$(MODE)/sec.txt &



fat32:
ifneq ($(BOARD),2k1000)
	cd ../util/qemu/2k1000 && sh create_qemu_img.sh && cd ../../../os
	@./buildfs.sh "$(U_FAT32_DIR)/$(FAT32_IMG_NAME)" "laqemu" $(MODE)
else
	if [ ! -d "$(U_FAT32_DIR)" ]; then @sudo mkdir "$(U_FAT32_DIR)"; fi
	@./buildfs.sh "$(U_FAT32_DIR)/$(FAT32_IMG_NAME)" 2k1000 $(MODE)
endif

remake-qemu-flash-fat-img: fat32 build qemu-flash-fat-img

qemu-flash-fat-img:
ifeq ($(BOARD), 2k1000)
	@echo $(CURDIR)
	@./la_fat $(MINITERM_START_CMD) $(LA_2K1000_PORT_FREQ) 
else ifeq ($(BOARD), laqemu)
	@cp ./target/loongarch64-unknown-linux-gnu/debug/os.bin ../os.bin
	@mv ../os.bin ../kernel.bin
	@./la_fat $(QEMU_2K1000)
endif


ext4:
ifneq ($(BOARD),2k1000)
	cd ../util/qemu/2k1000 && sh create_qemu_img.sh && cd ../../../os
	@./buildext4.sh "$(U_FAT32_DIR)/$(FAT32_IMG_NAME)" "laqemu" $(MODE)
else
	if [ ! -d "$(U_FAT32_DIR)" ]; then @sudo mkdir "$(U_FAT32_DIR)"; fi
	@./buildext4.sh "$(U_FAT32_DIR)/$(FAT32_IMG_NAME)" 2k1000 $(MODE)
endif

remake-qemu-flash-ext4-img: build
	@cp ./target/loongarch64-unknown-linux-gnu/debug/os.bin ../os.bin
	@mv ../os.bin ../kernel.bin

qemu-flash-ext4-img:
ifeq ($(BOARD), 2k1000)
	@echo $(CURDIR)
	@./la_fat $(MINITERM_START_CMD) $(LA_2K1000_PORT_FREQ) 
else ifeq ($(BOARD), laqemu)
	@cp ./target/loongarch64-unknown-linux-gnu/debug/os.bin ../os.bin
	@mv ../os.bin ../kernel.bin
endif

src/linker.ld: .FORCE
	cpp -P $(LA_2K1000_DISABLE_EH_FRAME) -D $(LDBOARD) src/linker.in.ld $@


kernel: src/linker.ld
	@echo Platform: $(BOARD)
    ifeq ($(MODE), debug)
		@cargo build --no-default-features --features "board_$(BOARD) $(LOG_OPTION)" --target $(TARGET)
    else
		@cargo build --no-default-features --release --features "board_$(BOARD) $(LOG_OPTION)"  --target $(TARGET)
    endif

clean:
	@cargo clean
	-@rm ../easy-fs-fuse/uImage
	-@rm ../easy-fs-fuse/rootfs-ubifs-ze.img
	-@cd ../user && make clean



do-run:
#First, link the image into the directory.
ifeq ($(BOARD), laqemu)
	-ln -s $(FAT32_2K1000_LN) $(QEMU_2K1000_DIR)/$(FAT32_IMG_LA)
	@echo "========WARNING!========"
	@echo "The next command is expecting a modified runqemu2k1000 script where any potential and implicit \"current working directory\" has been replaced by a generated script storage path."
	@./run_script $(QEMU_2K1000)
else ifeq ($(BOARD), 2k1000)
	@./run_script $(MINITERM_START_CMD) $(LA_2K1000_PORT_FREQ)
endif


new-gdb: build gdb
gdb:
ifeq ($(BOARD),laqemu)
	./la_fat $(QEMU_2K1000) "-S"
else ifeq ($(BOARD), 2k1000)
	@./la_gdbserver minicom -D $(LA_DEBUGGER_PORT_FREQ)
endif




.PHONY: user update gdb new-gdb monitor .FORCE

