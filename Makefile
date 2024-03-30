all:
	mv ./os/cargo ./os/.cargo
	mv ./user/cargo ./user/.cargo
	cd os && make build
	mv ./os/.cargo ./os/cargo
	mv ./user/.cargo ./user/cargo
	
rename:
	mv ./os/.cargo ./os/cargo
	mv ./user/.cargo ./user/cargo

clean:
	cd os && make clean
