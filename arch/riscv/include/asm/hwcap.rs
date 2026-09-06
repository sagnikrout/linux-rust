//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/hwcap.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copied from arch/arm64/include/asm/hwcap.h
//
// Copyright (C) 2012 ARM Ltd.
// Copyright (C) 2017 SiFive
//

//
// These macros represent the logical IDs of each multi-letter RISC-V ISA
// extension and are used in the ISA bitmap. The logical IDs start from
// RISCV_ISA_EXT_BASE, which allows the 0-25 range to be reserved for single
// letter extensions. The maximum, RISCV_ISA_EXT_MAX, is defined in order
// to allocate the bitmap and may be increased when necessary.
//
// New extensions should just be added to the bottom, rather than added
// alphabetically, in order to avoid unnecessary shuffling.
//
pub const RISCV_ISA_EXT_BASE: c_int = 26;
pub const RISCV_ISA_EXT_SSCOFPMF: c_int = 26;
pub const RISCV_ISA_EXT_SSTC: c_int = 27;
pub const RISCV_ISA_EXT_SVINVAL: c_int = 28;
pub const RISCV_ISA_EXT_SVPBMT: c_int = 29;
pub const RISCV_ISA_EXT_ZBB: c_int = 30;
pub const RISCV_ISA_EXT_ZICBOM: c_int = 31;
pub const RISCV_ISA_EXT_ZIHINTPAUSE: c_int = 32;
pub const RISCV_ISA_EXT_SVNAPOT: c_int = 33;
pub const RISCV_ISA_EXT_ZICBOZ: c_int = 34;
pub const RISCV_ISA_EXT_SMAIA: c_int = 35;
pub const RISCV_ISA_EXT_SSAIA: c_int = 36;
pub const RISCV_ISA_EXT_ZBA: c_int = 37;
pub const RISCV_ISA_EXT_ZBS: c_int = 38;
pub const RISCV_ISA_EXT_ZICNTR: c_int = 39;
pub const RISCV_ISA_EXT_ZICSR: c_int = 40;
pub const RISCV_ISA_EXT_ZIFENCEI: c_int = 41;
pub const RISCV_ISA_EXT_ZIHPM: c_int = 42;
pub const RISCV_ISA_EXT_SMSTATEEN: c_int = 43;
pub const RISCV_ISA_EXT_ZICOND: c_int = 44;
pub const RISCV_ISA_EXT_ZBC: c_int = 45;
pub const RISCV_ISA_EXT_ZBKB: c_int = 46;
pub const RISCV_ISA_EXT_ZBKC: c_int = 47;
pub const RISCV_ISA_EXT_ZBKX: c_int = 48;
pub const RISCV_ISA_EXT_ZKND: c_int = 49;
pub const RISCV_ISA_EXT_ZKNE: c_int = 50;
pub const RISCV_ISA_EXT_ZKNH: c_int = 51;
pub const RISCV_ISA_EXT_ZKR: c_int = 52;
pub const RISCV_ISA_EXT_ZKSED: c_int = 53;
pub const RISCV_ISA_EXT_ZKSH: c_int = 54;
pub const RISCV_ISA_EXT_ZKT: c_int = 55;
pub const RISCV_ISA_EXT_ZVBB: c_int = 56;
pub const RISCV_ISA_EXT_ZVBC: c_int = 57;
pub const RISCV_ISA_EXT_ZVKB: c_int = 58;
pub const RISCV_ISA_EXT_ZVKG: c_int = 59;
pub const RISCV_ISA_EXT_ZVKNED: c_int = 60;
pub const RISCV_ISA_EXT_ZVKNHA: c_int = 61;
pub const RISCV_ISA_EXT_ZVKNHB: c_int = 62;
pub const RISCV_ISA_EXT_ZVKSED: c_int = 63;
pub const RISCV_ISA_EXT_ZVKSH: c_int = 64;
pub const RISCV_ISA_EXT_ZVKT: c_int = 65;
pub const RISCV_ISA_EXT_ZFH: c_int = 66;
pub const RISCV_ISA_EXT_ZFHMIN: c_int = 67;
pub const RISCV_ISA_EXT_ZIHINTNTL: c_int = 68;
pub const RISCV_ISA_EXT_ZVFH: c_int = 69;
pub const RISCV_ISA_EXT_ZVFHMIN: c_int = 70;
pub const RISCV_ISA_EXT_ZFA: c_int = 71;
pub const RISCV_ISA_EXT_ZTSO: c_int = 72;
pub const RISCV_ISA_EXT_ZACAS: c_int = 73;
pub const RISCV_ISA_EXT_ZVE32X: c_int = 74;
pub const RISCV_ISA_EXT_ZVE32F: c_int = 75;
pub const RISCV_ISA_EXT_ZVE64X: c_int = 76;
pub const RISCV_ISA_EXT_ZVE64F: c_int = 77;
pub const RISCV_ISA_EXT_ZVE64D: c_int = 78;
pub const RISCV_ISA_EXT_ZIMOP: c_int = 79;
pub const RISCV_ISA_EXT_ZCA: c_int = 80;
pub const RISCV_ISA_EXT_ZCB: c_int = 81;
pub const RISCV_ISA_EXT_ZCD: c_int = 82;
pub const RISCV_ISA_EXT_ZCF: c_int = 83;
pub const RISCV_ISA_EXT_ZCMOP: c_int = 84;
pub const RISCV_ISA_EXT_ZAWRS: c_int = 85;
pub const RISCV_ISA_EXT_SVVPTC: c_int = 86;
pub const RISCV_ISA_EXT_SMMPM: c_int = 87;
pub const RISCV_ISA_EXT_SMNPM: c_int = 88;
pub const RISCV_ISA_EXT_SSNPM: c_int = 89;
pub const RISCV_ISA_EXT_ZABHA: c_int = 90;
pub const RISCV_ISA_EXT_ZICCRSE: c_int = 91;
pub const RISCV_ISA_EXT_SVADE: c_int = 92;
pub const RISCV_ISA_EXT_SVADU: c_int = 93;
pub const RISCV_ISA_EXT_ZFBFMIN: c_int = 94;
pub const RISCV_ISA_EXT_ZVFBFMIN: c_int = 95;
pub const RISCV_ISA_EXT_ZVFBFWMA: c_int = 96;
pub const RISCV_ISA_EXT_ZAAMO: c_int = 97;
pub const RISCV_ISA_EXT_ZALRSC: c_int = 98;
pub const RISCV_ISA_EXT_ZICBOP: c_int = 99;
pub const RISCV_ISA_EXT_SVRSW60T59B: c_int = 100;
pub const RISCV_ISA_EXT_ZALASR: c_int = 101;
pub const RISCV_ISA_EXT_ZILSD: c_int = 102;
pub const RISCV_ISA_EXT_ZCLSD: c_int = 103;
pub const RISCV_ISA_EXT_ZICFILP: c_int = 104;
pub const RISCV_ISA_EXT_ZICFISS: c_int = 105;
pub const RISCV_ISA_EXT_SSCSRIND: c_int = 106;
pub const RISCV_ISA_EXT_SMCSRIND: c_int = 107;
pub const RISCV_ISA_EXT_SMCNTRPMF: c_int = 108;
pub const RISCV_ISA_EXT_SSCCFG: c_int = 109;
pub const RISCV_ISA_EXT_SMCDELEG: c_int = 110;
pub const RISCV_ISA_EXT_SSQOSID: c_int = 111;
pub const RISCV_ISA_EXT_ZICCLSM: c_int = 112;
pub const RISCV_ISA_EXT_ZICCAMOA: c_int = 113;
pub const RISCV_ISA_EXT_ZICCIF: c_int = 114;
pub const RISCV_ISA_EXT_ZA64RS: c_int = 115;
pub const RISCV_ISA_EXT_XLINUXENVCFG: c_int = 127;
pub const RISCV_ISA_EXT_MAX: c_int = 128;

