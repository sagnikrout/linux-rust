//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/chrp/gg2.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


//
// include/asm-ppc/gg2.h -- VLSI VAS96011/12 `Golden Gate 2' register definitions
//
// Copyright (C) 1997 Geert Uytterhoeven
//
// This file is based on the following documentation:
//
// The VAS96011/12 Chipset, Data Book, Edition 1.0
// VLSI Technology, Inc.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//
// Memory Map (CHRP mode)
//
pub const GG2_PCI_MEM_BASE: c_uint = 0xc0000000	/* Peripheral memory space */;
pub const GG2_ISA_MEM_BASE: c_uint = 0xf7000000	/* Peripheral memory alias */;
pub const GG2_ISA_IO_BASE: c_uint = 0xf8000000	/* Peripheral I/O space */;
pub const GG2_PCI_CONFIG_BASE: c_uint = 0xfec00000	/* PCI configuration space */;
pub const GG2_INT_ACK_SPECIAL: c_uint = 0xfec80000	/* Interrupt acknowledge and */;
// special PCI cycles
pub const GG2_ROM_BASE0: c_uint = 0xff000000	/* ROM bank 0 */;
pub const GG2_ROM_BASE1: c_uint = 0xff800000	/* ROM bank 1 */;
//
// GG2 specific PCI Registers
//
pub const GG2_PCI_BUSNO: c_uint = 0x40	/* Bus number */;
pub const GG2_PCI_SUBBUSNO: c_uint = 0x41	/* Subordinate bus number */;
pub const GG2_PCI_DISCCTR: c_uint = 0x42	/* Disconnect counter */;
pub const GG2_PCI_PPC_CTRL: c_uint = 0x50	/* PowerPC interface control register */;
pub const GG2_PCI_ADDR_MAP: c_uint = 0x5c	/* Address map */;
pub const GG2_PCI_PCI_CTRL: c_uint = 0x60	/* PCI interface control register */;
pub const GG2_PCI_ROM_CTRL: c_uint = 0x70	/* ROM interface control register */;
pub const GG2_PCI_ROM_TIME: c_uint = 0x74	/* ROM timing */;
pub const GG2_PCI_CC_CTRL: c_uint = 0x80	/* Cache controller control register */;
pub const GG2_PCI_DRAM_BANK0: c_uint = 0x90	/* Control register for DRAM bank #0 */;
pub const GG2_PCI_DRAM_BANK1: c_uint = 0x94	/* Control register for DRAM bank #1 */;
pub const GG2_PCI_DRAM_BANK2: c_uint = 0x98	/* Control register for DRAM bank #2 */;
pub const GG2_PCI_DRAM_BANK3: c_uint = 0x9c	/* Control register for DRAM bank #3 */;
pub const GG2_PCI_DRAM_BANK4: c_uint = 0xa0	/* Control register for DRAM bank #4 */;
pub const GG2_PCI_DRAM_BANK5: c_uint = 0xa4	/* Control register for DRAM bank #5 */;
pub const GG2_PCI_DRAM_TIME0: c_uint = 0xb0	/* Timing parameters set #0 */;
pub const GG2_PCI_DRAM_TIME1: c_uint = 0xb4	/* Timing parameters set #1 */;
pub const GG2_PCI_DRAM_CTRL: c_uint = 0xc0	/* DRAM control */;
pub const GG2_PCI_ERR_CTRL: c_uint = 0xd0	/* Error control register */;
pub const GG2_PCI_ERR_STATUS: c_uint = 0xd4	/* Error status register */;
// Cleared when read
