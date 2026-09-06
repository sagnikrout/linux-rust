//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/elf.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

// The ABI of a file.
pub const EF_LOONGARCH_ABI_LP64_SOFT_FLOAT: c_uint = 0x1;
pub const EF_LOONGARCH_ABI_LP64_SINGLE_FLOAT: c_uint = 0x2;
pub const EF_LOONGARCH_ABI_LP64_DOUBLE_FLOAT: c_uint = 0x3;
pub const EF_LOONGARCH_ABI_ILP32_SOFT_FLOAT: c_uint = 0x5;
pub const EF_LOONGARCH_ABI_ILP32_SINGLE_FLOAT: c_uint = 0x6;
pub const EF_LOONGARCH_ABI_ILP32_DOUBLE_FLOAT: c_uint = 0x7;
// LoongArch relocation types used by the dynamic linker
pub const R_LARCH_NONE: c_int = 0;
pub const R_LARCH_32: c_int = 1;
pub const R_LARCH_64: c_int = 2;
pub const R_LARCH_RELATIVE: c_int = 3;
pub const R_LARCH_COPY: c_int = 4;
pub const R_LARCH_JUMP_SLOT: c_int = 5;
pub const R_LARCH_TLS_DTPMOD32: c_int = 6;
pub const R_LARCH_TLS_DTPMOD64: c_int = 7;
pub const R_LARCH_TLS_DTPREL32: c_int = 8;
pub const R_LARCH_TLS_DTPREL64: c_int = 9;
pub const R_LARCH_TLS_TPREL32: c_int = 10;
pub const R_LARCH_TLS_TPREL64: c_int = 11;
pub const R_LARCH_IRELATIVE: c_int = 12;
pub const R_LARCH_MARK_LA: c_int = 20;
pub const R_LARCH_MARK_PCREL: c_int = 21;
pub const R_LARCH_SOP_PUSH_PCREL: c_int = 22;
pub const R_LARCH_SOP_PUSH_ABSOLUTE: c_int = 23;
pub const R_LARCH_SOP_PUSH_DUP: c_int = 24;
pub const R_LARCH_SOP_PUSH_GPREL: c_int = 25;
pub const R_LARCH_SOP_PUSH_TLS_TPREL: c_int = 26;
pub const R_LARCH_SOP_PUSH_TLS_GOT: c_int = 27;
pub const R_LARCH_SOP_PUSH_TLS_GD: c_int = 28;
pub const R_LARCH_SOP_PUSH_PLT_PCREL: c_int = 29;
pub const R_LARCH_SOP_ASSERT: c_int = 30;
pub const R_LARCH_SOP_NOT: c_int = 31;
pub const R_LARCH_SOP_SUB: c_int = 32;
pub const R_LARCH_SOP_SL: c_int = 33;
pub const R_LARCH_SOP_SR: c_int = 34;
pub const R_LARCH_SOP_ADD: c_int = 35;
pub const R_LARCH_SOP_AND: c_int = 36;
pub const R_LARCH_SOP_IF_ELSE: c_int = 37;
pub const R_LARCH_SOP_POP_32_S_10_5: c_int = 38;
pub const R_LARCH_SOP_POP_32_U_10_12: c_int = 39;
pub const R_LARCH_SOP_POP_32_S_10_12: c_int = 40;
pub const R_LARCH_SOP_POP_32_S_10_16: c_int = 41;
pub const R_LARCH_SOP_POP_32_S_10_16_S2: c_int = 42;
pub const R_LARCH_SOP_POP_32_S_5_20: c_int = 43;
pub const R_LARCH_SOP_POP_32_S_0_5_10_16_S2: c_int = 44;
pub const R_LARCH_SOP_POP_32_S_0_10_10_16_S2: c_int = 45;
pub const R_LARCH_SOP_POP_32_U: c_int = 46;
pub const R_LARCH_ADD8: c_int = 47;
pub const R_LARCH_ADD16: c_int = 48;
pub const R_LARCH_ADD24: c_int = 49;
pub const R_LARCH_ADD32: c_int = 50;
pub const R_LARCH_ADD64: c_int = 51;
pub const R_LARCH_SUB8: c_int = 52;
pub const R_LARCH_SUB16: c_int = 53;
pub const R_LARCH_SUB24: c_int = 54;
pub const R_LARCH_SUB32: c_int = 55;
pub const R_LARCH_SUB64: c_int = 56;
pub const R_LARCH_GNU_VTINHERIT: c_int = 57;
pub const R_LARCH_GNU_VTENTRY: c_int = 58;
pub const R_LARCH_B16: c_int = 64;
pub const R_LARCH_B21: c_int = 65;
pub const R_LARCH_B26: c_int = 66;
pub const R_LARCH_ABS_HI20: c_int = 67;
pub const R_LARCH_ABS_LO12: c_int = 68;
pub const R_LARCH_ABS64_LO20: c_int = 69;
pub const R_LARCH_ABS64_HI12: c_int = 70;
pub const R_LARCH_PCALA_HI20: c_int = 71;
pub const R_LARCH_PCALA_LO12: c_int = 72;
pub const R_LARCH_PCALA64_LO20: c_int = 73;
pub const R_LARCH_PCALA64_HI12: c_int = 74;
pub const R_LARCH_GOT_PC_HI20: c_int = 75;
pub const R_LARCH_GOT_PC_LO12: c_int = 76;
pub const R_LARCH_GOT64_PC_LO20: c_int = 77;
pub const R_LARCH_GOT64_PC_HI12: c_int = 78;
pub const R_LARCH_GOT_HI20: c_int = 79;
pub const R_LARCH_GOT_LO12: c_int = 80;
pub const R_LARCH_GOT64_LO20: c_int = 81;
pub const R_LARCH_GOT64_HI12: c_int = 82;
pub const R_LARCH_TLS_LE_HI20: c_int = 83;
pub const R_LARCH_TLS_LE_LO12: c_int = 84;
pub const R_LARCH_TLS_LE64_LO20: c_int = 85;
pub const R_LARCH_TLS_LE64_HI12: c_int = 86;
pub const R_LARCH_TLS_IE_PC_HI20: c_int = 87;
pub const R_LARCH_TLS_IE_PC_LO12: c_int = 88;
pub const R_LARCH_TLS_IE64_PC_LO20: c_int = 89;
pub const R_LARCH_TLS_IE64_PC_HI12: c_int = 90;
pub const R_LARCH_TLS_IE_HI20: c_int = 91;
pub const R_LARCH_TLS_IE_LO12: c_int = 92;
pub const R_LARCH_TLS_IE64_LO20: c_int = 93;
pub const R_LARCH_TLS_IE64_HI12: c_int = 94;
pub const R_LARCH_TLS_LD_PC_HI20: c_int = 95;
pub const R_LARCH_TLS_LD_HI20: c_int = 96;
pub const R_LARCH_TLS_GD_PC_HI20: c_int = 97;
pub const R_LARCH_TLS_GD_HI20: c_int = 98;
pub const R_LARCH_32_PCREL: c_int = 99;
pub const R_LARCH_RELAX: c_int = 100;
pub const R_LARCH_DELETE: c_int = 101;
pub const R_LARCH_ALIGN: c_int = 102;
pub const R_LARCH_PCREL20_S2: c_int = 103;
pub const R_LARCH_CFA: c_int = 104;
pub const R_LARCH_ADD6: c_int = 105;
pub const R_LARCH_SUB6: c_int = 106;
pub const R_LARCH_ADD_ULEB128: c_int = 107;
pub const R_LARCH_SUB_ULEB128: c_int = 108;
pub const R_LARCH_64_PCREL: c_int = 109;
pub const R_LARCH_CALL36: c_int = 110;
pub const R_LARCH_TLS_DESC_PC_HI20: c_int = 111;
pub const R_LARCH_TLS_DESC_PC_LO12: c_int = 112;
pub const R_LARCH_TLS_DESC64_PC_LO20: c_int = 113;
pub const R_LARCH_TLS_DESC64_PC_HI12: c_int = 114;
pub const R_LARCH_TLS_DESC_HI20: c_int = 115;
pub const R_LARCH_TLS_DESC_LO12: c_int = 116;
pub const R_LARCH_TLS_DESC64_LO20: c_int = 117;
pub const R_LARCH_TLS_DESC64_HI12: c_int = 118;
pub const R_LARCH_TLS_DESC_LD: c_int = 119;
pub const R_LARCH_TLS_DESC_CALL: c_int = 120;
pub const R_LARCH_TLS_LE_HI20_R: c_int = 121;
pub const R_LARCH_TLS_LE_ADD_R: c_int = 122;
pub const R_LARCH_TLS_LE_LO12_R: c_int = 123;
pub const R_LARCH_TLS_LD_PCREL20_S2: c_int = 124;
pub const R_LARCH_TLS_GD_PCREL20_S2: c_int = 125;
pub const R_LARCH_TLS_DESC_PCREL20_S2: c_int = 126;
pub const R_LARCH_CALL30: c_int = 127;
pub const R_LARCH_PCADD_HI20: c_int = 128;
pub const R_LARCH_PCADD_LO12: c_int = 129;
pub const R_LARCH_GOT_PCADD_HI20: c_int = 130;
pub const R_LARCH_GOT_PCADD_LO12: c_int = 131;
pub const R_LARCH_TLS_IE_PCADD_HI20: c_int = 132;
pub const R_LARCH_TLS_IE_PCADD_LO12: c_int = 133;
pub const R_LARCH_TLS_LD_PCADD_HI20: c_int = 134;
pub const R_LARCH_TLS_LD_PCADD_LO12: c_int = 135;
pub const R_LARCH_TLS_GD_PCADD_HI20: c_int = 136;
pub const R_LARCH_TLS_GD_PCADD_LO12: c_int = 137;
pub const R_LARCH_TLS_DESC_PCADD_HI20: c_int = 138;
pub const R_LARCH_TLS_DESC_PCADD_LO12: c_int = 139;

// ELF register definitions
//
// General purpose have the following registers:
// Register	Number
// GPRs		32
// ORIG_A0		1
// ERA		1
// BADVADDR	1
// CRMD		1
// PRMD		1
// EUEN		1
// ECFG		1
// ESTAT		1
// Reserved	5
//
pub const ELF_NGREG: c_int = 45;
//
// Floating point have the following registers:
// Register	Number
// FPR		32
// FCC		1
// FCSR		1
//
pub const ELF_NFPREG: c_int = 34;
pub type elf_greg_t = c_ulong;
pub type elf_fpreg_t = double;
extern "C" {
    pub fn loongarch_dump_regs32(uregs: *mut u32, regs: *const pt_regs);
}
extern "C" {
    pub fn loongarch_dump_regs64(uregs: *mut u64, regs: *const pt_regs);
}

//
// This is used to ensure we don't load something for the wrong architecture.
//

//
// These are used to set parameters in the core dumps.
//

//
// This is used to ensure we don't load something for the wrong architecture.
//

//
// These are used to set parameters in the core dumps.
//

//
// These are used to set parameters in the core dumps.
//

//
// Return non-zero if HDR identifies an 32bit ELF binary.
//

//
// Return non-zero if HDR identifies an 64bit ELF binary.
//

// Macro flag: #define CORE_DUMP_USE_REGSET

//
// This yields a mask that user programs can use to figure out what
// instruction set this cpu supports. This could be done in userspace,
// but it's not easy, and we've already done it here.
//

//
// This yields a string that ld.so will use to load implementation
// specific libraries for optimization.	 This is more specific in
// intent than poking at uname or /proc/cpuinfo.
//

//
// This is the location that an ET_DYN program is loaded if exec'ed. Typical
// use of this is to invoke "./ld.so someprog" to test out a new version of
// the loader. We need to make sure that it is out of the way of the program
// that it will "exec", and that there is sufficient room for the brk.
//

// update AT_VECTOR_SIZE_ARCH if the number of NEW_AUX_ENT entries changes

pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_elf_state {
    pub fp_abi: c_int,
    pub interp_fp_abi: c_int,
}

