
CC = gcc
OBJCOPY = objcopy
OBJDUMP = objdump




.DEFAULT_GOAL := all
all:
	@cargo build --release

.PHONY : run
run: all
	@echo "------------------------------------"
	@cargo run --release

.PHONY : debug
debug: all
	@echo "-------------------------------------------------------"
	@./test.sh


.PHONY : clean
clean:
	rm -rf tmp*
