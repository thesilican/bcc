SRC_DIR := src
OBJ_DIR := obj
BIN_DIR := bin
BIN_NAME := bcc

SRCS := $(shell find $(SRC_DIR) -name '*.c')
OBJS := $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)
ASMS := $(OBJS:.o=.s)
DEPS := $(OBJS:.o=.d)

CC := gcc
CFLAGS := -std=c99 -MMD -MP -Wall -Wpedantic -g3

.PHONY: build asm clean run test

build: $(BIN_DIR)/$(BIN_NAME)

run: $(BIN_DIR)/$(BIN_NAME)
	$(BIN_DIR)/$(BIN_NAME)

test: $(BIN_DIR)/$(BIN_NAME)
	test/test.py

asm: $(ASMS)

clean:
	rm -rf $(OBJ_DIR)/* $(BIN_DIR)/*

$(BIN_DIR):
	mkdir -p $(BIN_DIR)

$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJ_DIR)/%.s: $(SRC_DIR)/%.c $(OBJ_DIR)
	$(CC) -S -c $< -o $@

$(BIN_DIR)/$(BIN_NAME): $(OBJS) $(BIN_DIR)
	$(CC) $(OBJS) -o $@ $(LDFLAGS)

-include $(DEPS)
