RPI_BOX=<SET_YOUR_RPI_HOST_ADDRESS>
CONTAINER_NAME=rust-arm-builder

# check if rust builder is running
RUNNING = $(shell docker ps --format '{{.Names}}' | grep -w $(CONTAINER_NAME) -m1)

# build the rust image with cross compile dependencies and runs a container in background
start-env:
ifneq ($(RUNNING), $(CONTAINER_NAME))
	docker build -t rust-arm-builder .
	docker run --name $(CONTAINER_NAME) --volume $(PWD):/usr/src/app --detach $(CONTAINER_NAME)
endif

# stops the docker container
stop-env:
ifeq ($(RUNNING), $(CONTAINER_NAME))
	docker stop $(CONTAINER_NAME) && docker rm $(CONTAINER_NAME)
endif

# setting linker in .cargo/config doesn't seem to work
build: start-env
	docker exec -it $(CONTAINER_NAME) /bin/bash -c \
		"RUSTFLAGS='-C linker=arm-linux-gnueabihf-gcc' cargo build --target armv7-unknown-linux-gnueabihf --release"

clean:
	rm -rf target

# copy the built binary to the target machine
copy-bin: build
	scp ./target/armv7-unknown-linux-gnueabihf/release/blink $(RPI_BOX):

.PHONY: start-env stop-env build clean