//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ppc-opcode.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2009 Freescale Semiconductor, Inc.
//
// provides masks and opcode images for use by code generation, emulation
// and for instructions that older assemblers might not know about
//

pub const __REG_R0: c_int = 0;
pub const __REG_R1: c_int = 1;
pub const __REG_R2: c_int = 2;
pub const __REG_R3: c_int = 3;
pub const __REG_R4: c_int = 4;
pub const __REG_R5: c_int = 5;
pub const __REG_R6: c_int = 6;
pub const __REG_R7: c_int = 7;
pub const __REG_R8: c_int = 8;
pub const __REG_R9: c_int = 9;
pub const __REG_R10: c_int = 10;
pub const __REG_R11: c_int = 11;
pub const __REG_R12: c_int = 12;
pub const __REG_R13: c_int = 13;
pub const __REG_R14: c_int = 14;
pub const __REG_R15: c_int = 15;
pub const __REG_R16: c_int = 16;
pub const __REG_R17: c_int = 17;
pub const __REG_R18: c_int = 18;
pub const __REG_R19: c_int = 19;
pub const __REG_R20: c_int = 20;
pub const __REG_R21: c_int = 21;
pub const __REG_R22: c_int = 22;
pub const __REG_R23: c_int = 23;
pub const __REG_R24: c_int = 24;
pub const __REG_R25: c_int = 25;
pub const __REG_R26: c_int = 26;
pub const __REG_R27: c_int = 27;
pub const __REG_R28: c_int = 28;
pub const __REG_R29: c_int = 29;
pub const __REG_R30: c_int = 30;
pub const __REG_R31: c_int = 31;
pub const __REGA0_0: c_int = 0;
pub const __REGA0_R1: c_int = 1;
pub const __REGA0_R2: c_int = 2;
pub const __REGA0_R3: c_int = 3;
pub const __REGA0_R4: c_int = 4;
pub const __REGA0_R5: c_int = 5;
pub const __REGA0_R6: c_int = 6;
pub const __REGA0_R7: c_int = 7;
pub const __REGA0_R8: c_int = 8;
pub const __REGA0_R9: c_int = 9;
pub const __REGA0_R10: c_int = 10;
pub const __REGA0_R11: c_int = 11;
pub const __REGA0_R12: c_int = 12;
pub const __REGA0_R13: c_int = 13;
pub const __REGA0_R14: c_int = 14;
pub const __REGA0_R15: c_int = 15;
pub const __REGA0_R16: c_int = 16;
pub const __REGA0_R17: c_int = 17;
pub const __REGA0_R18: c_int = 18;
pub const __REGA0_R19: c_int = 19;
pub const __REGA0_R20: c_int = 20;
pub const __REGA0_R21: c_int = 21;
pub const __REGA0_R22: c_int = 22;
pub const __REGA0_R23: c_int = 23;
pub const __REGA0_R24: c_int = 24;
pub const __REGA0_R25: c_int = 25;
pub const __REGA0_R26: c_int = 26;
pub const __REGA0_R27: c_int = 27;
pub const __REGA0_R28: c_int = 28;
pub const __REGA0_R29: c_int = 29;
pub const __REGA0_R30: c_int = 30;
pub const __REGA0_R31: c_int = 31;
// For use with PPC_RAW_() macros
pub const _R0: c_int = 0;
pub const _R1: c_int = 1;
pub const _R2: c_int = 2;
pub const _R3: c_int = 3;
pub const _R4: c_int = 4;
pub const _R5: c_int = 5;
pub const _R6: c_int = 6;
pub const _R7: c_int = 7;
pub const _R8: c_int = 8;
pub const _R9: c_int = 9;
pub const _R10: c_int = 10;
pub const _R11: c_int = 11;
pub const _R12: c_int = 12;
pub const _R13: c_int = 13;
pub const _R14: c_int = 14;
pub const _R15: c_int = 15;
pub const _R16: c_int = 16;
pub const _R17: c_int = 17;
pub const _R18: c_int = 18;
pub const _R19: c_int = 19;
pub const _R20: c_int = 20;
pub const _R21: c_int = 21;
pub const _R22: c_int = 22;
pub const _R23: c_int = 23;
pub const _R24: c_int = 24;
pub const _R25: c_int = 25;
pub const _R26: c_int = 26;
pub const _R27: c_int = 27;
pub const _R28: c_int = 28;
pub const _R29: c_int = 29;
pub const _R30: c_int = 30;
pub const _R31: c_int = 31;

//
// 16-bit immediate helper macros: HA() is for use with sign-extending instrs
// (e.g. LD, ADDI).  If the bottom 16 bits is "-ve", add another bit into the
// top half to negate the effect (i.e. 0xffff + 1 = 0x(1)0000).
//
// XXX: should these mask out possible sign bits?
//

//
// 18-bit immediate helper for prefix 18-bit upper immediate si0 field.
//

// opcode and xopcode for instructions
pub const OP_PREFIX: c_int = 1;
pub const OP_TRAP_64: c_int = 2;
pub const OP_TRAP: c_int = 3;
pub const OP_SC: c_int = 17;
pub const OP_19: c_int = 19;
pub const OP_31: c_int = 31;
pub const OP_LWZ: c_int = 32;
pub const OP_LWZU: c_int = 33;
pub const OP_LBZ: c_int = 34;
pub const OP_LBZU: c_int = 35;
pub const OP_STW: c_int = 36;
pub const OP_STWU: c_int = 37;
pub const OP_STB: c_int = 38;
pub const OP_STBU: c_int = 39;
pub const OP_LHZ: c_int = 40;
pub const OP_LHZU: c_int = 41;
pub const OP_LHA: c_int = 42;
pub const OP_LHAU: c_int = 43;
pub const OP_STH: c_int = 44;
pub const OP_STHU: c_int = 45;
pub const OP_LMW: c_int = 46;
pub const OP_STMW: c_int = 47;
pub const OP_LFS: c_int = 48;
pub const OP_LFSU: c_int = 49;
pub const OP_LFD: c_int = 50;
pub const OP_LFDU: c_int = 51;
pub const OP_STFS: c_int = 52;
pub const OP_STFSU: c_int = 53;
pub const OP_STFD: c_int = 54;
pub const OP_STFDU: c_int = 55;
pub const OP_LQ: c_int = 56;
pub const OP_LD: c_int = 58;
pub const OP_STD: c_int = 62;
pub const OP_19_XOP_RFID: c_int = 18;
pub const OP_19_XOP_RFMCI: c_int = 38;
pub const OP_19_XOP_RFDI: c_int = 39;
pub const OP_19_XOP_RFI: c_int = 50;
pub const OP_19_XOP_RFCI: c_int = 51;
pub const OP_19_XOP_RFSCV: c_int = 82;
pub const OP_19_XOP_HRFID: c_int = 274;
pub const OP_19_XOP_URFID: c_int = 306;
pub const OP_19_XOP_STOP: c_int = 370;
pub const OP_19_XOP_DOZE: c_int = 402;
pub const OP_19_XOP_NAP: c_int = 434;
pub const OP_19_XOP_SLEEP: c_int = 466;
pub const OP_19_XOP_RVWINKLE: c_int = 498;
pub const OP_31_XOP_TRAP: c_int = 4;
pub const OP_31_XOP_LDX: c_int = 21;
pub const OP_31_XOP_LWZX: c_int = 23;
pub const OP_31_XOP_LDUX: c_int = 53;
pub const OP_31_XOP_DCBST: c_int = 54;
pub const OP_31_XOP_LWZUX: c_int = 55;
pub const OP_31_XOP_TRAP_64: c_int = 68;
pub const OP_31_XOP_DCBF: c_int = 86;
pub const OP_31_XOP_LBZX: c_int = 87;
pub const OP_31_XOP_STDX: c_int = 149;
pub const OP_31_XOP_STWX: c_int = 151;
pub const OP_31_XOP_STDUX: c_int = 181;
pub const OP_31_XOP_STWUX: c_int = 183;
pub const OP_31_XOP_STBX: c_int = 215;
pub const OP_31_XOP_LBZUX: c_int = 119;
pub const OP_31_XOP_STBUX: c_int = 247;
pub const OP_31_XOP_LHZX: c_int = 279;
pub const OP_31_XOP_LHZUX: c_int = 311;
pub const OP_31_XOP_MSGSNDP: c_int = 142;
pub const OP_31_XOP_MSGCLRP: c_int = 174;
pub const OP_31_XOP_MTMSR: c_int = 146;
pub const OP_31_XOP_MTMSRD: c_int = 178;
pub const OP_31_XOP_TLBIE: c_int = 306;
pub const OP_31_XOP_MFSPR: c_int = 339;
pub const OP_31_XOP_LWAX: c_int = 341;
pub const OP_31_XOP_LHAX: c_int = 343;
pub const OP_31_XOP_LWAUX: c_int = 373;
pub const OP_31_XOP_LHAUX: c_int = 375;
pub const OP_31_XOP_STHX: c_int = 407;
pub const OP_31_XOP_STHUX: c_int = 439;
pub const OP_31_XOP_MTSPR: c_int = 467;
pub const OP_31_XOP_DCBI: c_int = 470;
pub const OP_31_XOP_LDBRX: c_int = 532;
pub const OP_31_XOP_LWBRX: c_int = 534;
pub const OP_31_XOP_TLBSYNC: c_int = 566;
pub const OP_31_XOP_STDBRX: c_int = 660;
pub const OP_31_XOP_STWBRX: c_int = 662;
pub const OP_31_XOP_STFSX: c_int = 663;
pub const OP_31_XOP_STFSUX: c_int = 695;
pub const OP_31_XOP_STFDX: c_int = 727;
pub const OP_31_XOP_HASHCHK: c_int = 754;
pub const OP_31_XOP_STFDUX: c_int = 759;
pub const OP_31_XOP_LHBRX: c_int = 790;
pub const OP_31_XOP_LFIWAX: c_int = 855;
pub const OP_31_XOP_LFIWZX: c_int = 887;
pub const OP_31_XOP_STHBRX: c_int = 918;
pub const OP_31_XOP_STFIWX: c_int = 983;
// VSX Scalar Load Instructions
pub const OP_31_XOP_LXSDX: c_int = 588;
pub const OP_31_XOP_LXSSPX: c_int = 524;
pub const OP_31_XOP_LXSIWAX: c_int = 76;
pub const OP_31_XOP_LXSIWZX: c_int = 12;
// VSX Scalar Store Instructions
pub const OP_31_XOP_STXSDX: c_int = 716;
pub const OP_31_XOP_STXSSPX: c_int = 652;
pub const OP_31_XOP_STXSIWX: c_int = 140;
// VSX Vector Load Instructions
pub const OP_31_XOP_LXVD2X: c_int = 844;
pub const OP_31_XOP_LXVW4X: c_int = 780;
// VSX Vector Load and Splat Instruction
pub const OP_31_XOP_LXVDSX: c_int = 332;
// VSX Vector Store Instructions
pub const OP_31_XOP_STXVD2X: c_int = 972;
pub const OP_31_XOP_STXVW4X: c_int = 908;
pub const OP_31_XOP_LFSX: c_int = 535;
pub const OP_31_XOP_LFSUX: c_int = 567;
pub const OP_31_XOP_LFDX: c_int = 599;
pub const OP_31_XOP_LFDUX: c_int = 631;
// VMX Vector Load Instructions
pub const OP_31_XOP_LVX: c_int = 103;
// VMX Vector Store Instructions
pub const OP_31_XOP_STVX: c_int = 231;
// sorted alphabetically
pub const PPC_INST_BCCTR_FLUSH: c_uint = 0x4c400420;
pub const PPC_INST_COPY: c_uint = 0x7c20060c;
pub const PPC_INST_DCBA: c_uint = 0x7c0005ec;
pub const PPC_INST_DCBA_MASK: c_uint = 0xfc0007fe;
pub const PPC_INST_DSSALL: c_uint = 0x7e00066c;
pub const PPC_INST_ISEL: c_uint = 0x7c00001e;
pub const PPC_INST_ISEL_MASK: c_uint = 0xfc00003e;
pub const PPC_INST_LSWI: c_uint = 0x7c0004aa;
pub const PPC_INST_LSWX: c_uint = 0x7c00042a;
pub const PPC_INST_LWSYNC: c_uint = 0x7c2004ac;
pub const PPC_INST_SYNC: c_uint = 0x7c0004ac;
pub const PPC_INST_SYNC_MASK: c_uint = 0xfc0007fe;
pub const PPC_INST_MCRXR: c_uint = 0x7c000400;
pub const PPC_INST_MCRXR_MASK: c_uint = 0xfc0007fe;
pub const PPC_INST_MFSPR_PVR: c_uint = 0x7c1f42a6;
pub const PPC_INST_MFSPR_PVR_MASK: c_uint = 0xfc1ffffe;
pub const PPC_INST_MTMSRD: c_uint = 0x7c000164;
pub const PPC_INST_PASTE: c_uint = 0x7c20070d;
pub const PPC_INST_PASTE_MASK: c_uint = 0xfc2007ff;
pub const PPC_INST_POPCNTB: c_uint = 0x7c0000f4;
pub const PPC_INST_POPCNTB_MASK: c_uint = 0xfc0007fe;
pub const PPC_INST_RFEBB: c_uint = 0x4c000124;
pub const PPC_INST_RFID: c_uint = 0x4c000024;
pub const PPC_INST_MFSPR_DSCR: c_uint = 0x7c1102a6;
pub const PPC_INST_MFSPR_DSCR_MASK: c_uint = 0xfc1ffffe;
pub const PPC_INST_MTSPR_DSCR: c_uint = 0x7c1103a6;
pub const PPC_INST_MTSPR_DSCR_MASK: c_uint = 0xfc1ffffe;
pub const PPC_INST_MFSPR_DSCR_USER: c_uint = 0x7c0302a6;
pub const PPC_INST_MFSPR_DSCR_USER_MASK: c_uint = 0xfc1ffffe;
pub const PPC_INST_MTSPR_DSCR_USER: c_uint = 0x7c0303a6;
pub const PPC_INST_MTSPR_DSCR_USER_MASK: c_uint = 0xfc1ffffe;
pub const PPC_INST_STRING: c_uint = 0x7c00042a;
pub const PPC_INST_STRING_MASK: c_uint = 0xfc0007fe;
pub const PPC_INST_STRING_GEN_MASK: c_uint = 0xfc00067e;
pub const PPC_INST_STSWI: c_uint = 0x7c0005aa;
pub const PPC_INST_STSWX: c_uint = 0x7c00052a;
pub const PPC_INST_TRECHKPT: c_uint = 0x7c0007dd;
pub const PPC_INST_TRECLAIM: c_uint = 0x7c00075d;
pub const PPC_INST_TSR: c_uint = 0x7c0005dd;
pub const PPC_INST_BRANCH_COND: c_uint = 0x40800000;
// Prefixes
pub const PPC_INST_LFS: c_uint = 0xc0000000;
pub const PPC_INST_STFS: c_uint = 0xd0000000;
pub const PPC_INST_LFD: c_uint = 0xc8000000;
pub const PPC_INST_STFD: c_uint = 0xd8000000;
pub const PPC_PREFIX_MLS: c_uint = 0x06000000;
pub const PPC_PREFIX_8LS: c_uint = 0x04000000;
// Prefixed instructions
pub const PPC_INST_PADDI: c_uint = 0x38000000;
pub const PPC_INST_PLD: c_uint = 0xe4000000;
pub const PPC_INST_PSTD: c_uint = 0xf4000000;
// macros to insert fields into opcodes

//
// Both low and high 16 bits are added as SIGNED additions, so if low 16 bits
// has high bit set, high 16 bits must be adjusted. These macros do that (stolen
// from binutils).
//

// LI Field
pub const PPC_LI_MASK: c_uint = 0x03fffffc;

// Base instruction encoding

//
// Define what the VSX XX1 form instructions will look like, then add
// the 128 bit load store instructions based on that.
//

// slwi = rlwinm Rx, Ry, n, 0, 31-n

// srwi = rlwinm Rx, Ry, 32-n, n, 31

// sldi = rldicr Rx, Ry, n, 63-n

// sldi = rldicl Rx, Ry, 64-n, n

// bcl 20,31,$+4

// Deal with instructions that older assemblers aren't aware of

// PASemi instructions

// BHRB instructions

// Transactional memory instructions

// book3e thread control instructions

// Coprocessor instructions

//
// These may only be used on ISA v3.0 or later (aka. CPU_FTR_ARCH_300, radix
// implies CPU_FTR_ARCH_300). USER/GUEST invalidates may only be used by radix
// mode (on HPT these would also invalidate various SLBEs which may not be
// desired).
//

