#!/usr/bin/env bash
set -e

mkdir -p /home/sagnik/linux/rust_mini_kernel/isodir/boot/grub
cp /home/sagnik/linux/rust_mini_kernel/kernel.elf /home/sagnik/linux/rust_mini_kernel/isodir/boot/kernel.elf

cat > /home/sagnik/linux/rust_mini_kernel/isodir/boot/grub/grub.cfg << "CFG"
set timeout=0
set default=0
menuentry "Linux Modernized Rust Kernel" {
    multiboot /boot/kernel.elf
    boot
}
CFG

grub-mkrescue -o /home/sagnik/linux/rust_mini_kernel/kernel.iso /home/sagnik/linux/rust_mini_kernel/isodir
ls -lh /home/sagnik/linux/rust_mini_kernel/kernel.iso
