#!/usr/bin/env sh

echo "Making object files..."
arm-none-eabi-as -obootloader.o bootloader.s
arm-none-eabi-as -otest.o test.s

echo "Making binaries..."
arm-none-eabi-ld -Ttest.ld -otest.elf test.o
arm-none-eabi-ld -Tbootloader.ld -obootloader.elf bootloader.o

echo "Stripping binary..."
arm-none-eabi-strip --strip-debug --strip-unneeded bootloader.elf
arm-none-eabi-strip --strip-debug --strip-unneeded test.elf
arm-none-eabi-objcopy -Obinary bootloader.elf bootloader.bin
arm-none-eabi-objcopy -Obinary test.elf test.agb

echo "Patching header (test)..."
agbsum -psitest.agb
