CARGO ?= cargo
INSTALL ?= install

BIN_NAME := t2-zonepower
BUILD_DIR := $(or $(CARGO_TARGET_DIR),target)/release
BIN_SRC := $(BUILD_DIR)/$(BIN_NAME)
UNIT_SRC := systemd/t2-zonepowerd.service

PREFIX ?= /usr/local
BIN_DIR := $(PREFIX)/bin
UNIT_DIR := $(PREFIX)/lib/systemd/system

.PHONY: all build check-build install uninstall clean rebuild

all: build

build:
	$(CARGO) build --release --locked

check-build:
	@test -x "$(BIN_SRC)" || { echo "error: $(BIN_SRC) is missing; run 'make build' as your normal user first" >&2; exit 1; }

install: check-build
	$(INSTALL) -d -o root -g root -m 0755 $(BIN_DIR) $(UNIT_DIR)
	$(INSTALL) -o root -g root -m 0755 $(BIN_SRC) $(BIN_DIR)/$(BIN_NAME)
	$(INSTALL) -o root -g root -m 0644 $(UNIT_SRC) $(UNIT_DIR)/t2-zonepowerd.service
	systemctl daemon-reload
	systemctl enable t2-zonepowerd.service
	systemctl restart t2-zonepowerd.service

uninstall:
	systemctl disable --now t2-zonepowerd.service || true
	rm -f $(BIN_DIR)/$(BIN_NAME)
	rm -f $(UNIT_DIR)/t2-zonepowerd.service
	systemctl daemon-reload

clean:
	$(CARGO) clean

rebuild: clean build
