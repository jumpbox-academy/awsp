TARGET_PATH = target/release

buildWindows:
	cargo build --release 

buildMacOs:
	cargo build --release
	upx --best --lzma $(TARGET_PATH)/awsp

buildLinux:
	cargo build --release
	upx --best --lzma $(TARGET_PATH)/awsp

# build: buildWindows buildMacOS buildLinux
build: 
	cargo build --release

clean:
	rm -rf target

# all: clean build