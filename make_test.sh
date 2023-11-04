#!/usr/bin/env sh

echo Making object file...
arm-none-eabi-as -otest.o test.s

echo Making binary...
arm-none-eabi-ld -Ttest.ld -otest.elf test.o

echo Stripping binary...
arm-none-eabi-strip --strip-debug --strip-unneeded test.elf
arm-none-eabi-objcopy -Obinary test.elf test.agb

echo Patching header...
agbsum -psitest.agb
