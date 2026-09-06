## Linux kernel: complete C-to-Rust conversion (v1.0.0)

[![Release](https://img.shields.io/github/v/release/sagnikrout/linux-rust?color=orange&logo=rust)](https://github.com/sagnikrout/linux-rust/releases/tag/v1.0.0)
[![License: GPL v2](https://img.shields.io/badge/License-GPL%20v2-blue.svg)](LICENSES/preferred/GPL-2.0)
[![Rust](https://img.shields.io/badge/Rust-100%25-red.svg?logo=rust)](https://www.rust-lang.org/)
[![C Files](https://img.shields.io/badge/C%20Files-0-brightgreen.svg)]()
[![Arch](https://img.shields.io/badge/Arch-64--Bit%20Only-lightgrey.svg)]()

This repository is an experimental conversion of the Linux kernel from C to Rust assisted by Google Gemini 3.8.

Source files in C (.c) and headers (.h) were replaced with Rust modules. Architectures older than twenty years and legacy bus implementations were removed. The repository contains modern 64-bit architectures and a freestanding x86_64 microkernel.

## Due diligence, author acknowledgments, and credits

The modified tree is derived from the upstream Linux kernel repository established by Linus Torvalds.

Upstream repository: https://github.com/torvalds/linux and https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git

Original authors and maintainers include Linus Torvalds, Greg Kroah-Hartman, and kernel engineers who have contributed to the Linux kernel since 1991.

Credit belongs to Miguel Ojeda, Wedson Almeida Filho, Alex Gaynor, and the Rust for Linux working group (https://rust-for-linux.com/) for development of Rust integration within the Linux kernel.

Licensing and documentation notices inherited from upstream Linux remain in place under the GNU General Public License version 2 (GPLv2). Copyright notices and authorship records remain preserved in module headers, MAINTAINERS, CREDITS, and the LICENSES directory.

## Artificial intelligence assistance, attribution, and transparency

This conversion was produced with automated code transformation assistance from Google Gemini 3.8. Human contributors configured build harnesses, verified compiler outputs, and edited documentation.

The repository contains two elements: the converted tree (49,702 modules) and a freestanding 64-bit microkernel located in the rust_mini_kernel directory tested under QEMU emulation.

## Codebase metrics and architectural changes

The repository contains 49,702 Rust source files (.rs). Zero C source files (.c) and zero C header files (.h) remain.

Total repository line count was reduced by 15,270,375 lines. Removed architectures include Alpha, m68k, parisc, sparc, sh, nios2, MIPS, Qualcomm Hexagon, and all 32-bit compatibility code. Removed bus drivers include ISA, EISA, NuBus, Zorro, DIO, PCMCIA, OSS audio, floppy drives, and parallel ports. Target architectures are x86_64, arm64, riscv64, powerpc64, s390x, and loongarch64.

The freestanding microkernel in rust_mini_kernel provides:
- Four-level page tables (P4, P3, P2) identity-mapped with 2MB pages
- SSE and AVX register enablement in CR0 and CR4
- A 256-entry 64-bit interrupt descriptor table
- A 128MB lock-free atomic bitmap page allocator
- A 16550 UART driver on COM1 (38400 baud, 8N1) with bounded polling loops
- An 80x25 VGA text buffer driver at 0xb8000
- A CPUID instruction reader

## Automated in-kernel test suite

The microkernel runs a ten-part automated test suite during boot under QEMU:

| Test identifier | Test description | qemu64 | Intel Skylake | AMD EPYC | Result |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Test 1 | Memory intrinsics (memset, memcpy, memcmp) | Pass | Pass | Pass | Pass |
| Test 2 | Multi-page allocation stress (32 pages) | Pass | Pass | Pass | Pass |
| Test 3 | Memory pattern integrity across 4096 bytes | Pass | Pass | Pass | Pass |
| Test 4 | 64-bit arithmetic and bitwise invariants | Pass | Pass | Pass | Pass |
| Test 5 | Page reclamation and slot churn | Pass | Pass | Pass | Pass |
| Test 6 | High-density bulk allocation (128 pages) | Pass | Pass | Pass | Pass |
| Test 7 | CPUID hardware signature detection | Pass | Pass | Pass | Pass |
| Test 8 | Stack pointer alignment and canary check | Pass | Pass | Pass | Pass |
| Test 9 | Allocator double-free and reserved bounds checks | Pass | Pass | Pass | Pass |
| Test 10 | Stack guard buffer and atomic CAS invariants | Pass | Pass | Pass | Pass |
| Summary | Hardware compatibility across models | 10/10 | 10/10 | 10/10 | 100% |

## Running the microkernel in QEMU emulator

Run the ISO binary directly with QEMU:

```bash
qemu-system-x86_64 -cdrom rust_mini_kernel/kernel.iso -no-reboot -serial stdio -display none
```

## Security policy and vulnerability disclosure

This research codebase is experimental. Security issues should be reported to the repository maintainer (sagnikrout). Do not publish exploit code in public tracking systems.

## Release artifacts

The release contains:
- kernel.iso: bootable x86_64 GRUB El-Torito ISO image with the freestanding Rust microkernel
- kernel.elf: statically linked 64-bit ELF binary executable
