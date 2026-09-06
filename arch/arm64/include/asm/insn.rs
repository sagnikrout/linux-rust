//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/insn.h
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
// Copyright (C) 2013 Huawei Ltd.
// Author: Jiang Liu <liuj97@gmail.com>
//
// Copyright (C) 2014 Zi Shen Lim <zlim.lnx@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_hint_cr_op {
    AARCH64_INSN_HINT_NOP	= 0x0 << 5,
    AARCH64_INSN_HINT_YIELD	= 0x1 << 5,
    AARCH64_INSN_HINT_WFE	= 0x2 << 5,
    AARCH64_INSN_HINT_WFI	= 0x3 << 5,
    AARCH64_INSN_HINT_SEV	= 0x4 << 5,
    AARCH64_INSN_HINT_SEVL	= 0x5 << 5,

    AARCH64_INSN_HINT_XPACLRI    = 0x07 << 5,
    AARCH64_INSN_HINT_PACIA_1716 = 0x08 << 5,
    AARCH64_INSN_HINT_PACIB_1716 = 0x0A << 5,
    AARCH64_INSN_HINT_AUTIA_1716 = 0x0C << 5,
    AARCH64_INSN_HINT_AUTIB_1716 = 0x0E << 5,
    AARCH64_INSN_HINT_PACIAZ     = 0x18 << 5,
    AARCH64_INSN_HINT_PACIASP    = 0x19 << 5,
    AARCH64_INSN_HINT_PACIBZ     = 0x1A << 5,
    AARCH64_INSN_HINT_PACIBSP    = 0x1B << 5,
    AARCH64_INSN_HINT_AUTIAZ     = 0x1C << 5,
    AARCH64_INSN_HINT_AUTIASP    = 0x1D << 5,
    AARCH64_INSN_HINT_AUTIBZ     = 0x1E << 5,
    AARCH64_INSN_HINT_AUTIBSP    = 0x1F << 5,

    AARCH64_INSN_HINT_ESB  = 0x10 << 5,
    AARCH64_INSN_HINT_PSB  = 0x11 << 5,
    AARCH64_INSN_HINT_TSB  = 0x12 << 5,
    AARCH64_INSN_HINT_CSDB = 0x14 << 5,
    AARCH64_INSN_HINT_CLEARBHB = 0x16 << 5,

    AARCH64_INSN_HINT_BTI   = 0x20 << 5,
    AARCH64_INSN_HINT_BTIC  = 0x22 << 5,
    AARCH64_INSN_HINT_BTIJ  = 0x24 << 5,
    AARCH64_INSN_HINT_BTIJC = 0x26 << 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_imm_type {
    AARCH64_INSN_IMM_ADR,
    AARCH64_INSN_IMM_26,
    AARCH64_INSN_IMM_19,
    AARCH64_INSN_IMM_16,
    AARCH64_INSN_IMM_14,
    AARCH64_INSN_IMM_12,
    AARCH64_INSN_IMM_9,
    AARCH64_INSN_IMM_7,
    AARCH64_INSN_IMM_6,
    AARCH64_INSN_IMM_S,
    AARCH64_INSN_IMM_R,
    AARCH64_INSN_IMM_N,
    AARCH64_INSN_IMM_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_register_type {
    AARCH64_INSN_REGTYPE_RT,
    AARCH64_INSN_REGTYPE_RN,
    AARCH64_INSN_REGTYPE_RT2,
    AARCH64_INSN_REGTYPE_RM,
    AARCH64_INSN_REGTYPE_RD,
    AARCH64_INSN_REGTYPE_RA,
    AARCH64_INSN_REGTYPE_RS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_register {
    AARCH64_INSN_REG_0  = 0,
    AARCH64_INSN_REG_1  = 1,
    AARCH64_INSN_REG_2  = 2,
    AARCH64_INSN_REG_3  = 3,
    AARCH64_INSN_REG_4  = 4,
    AARCH64_INSN_REG_5  = 5,
    AARCH64_INSN_REG_6  = 6,
    AARCH64_INSN_REG_7  = 7,
    AARCH64_INSN_REG_8  = 8,
    AARCH64_INSN_REG_9  = 9,
    AARCH64_INSN_REG_10 = 10,
    AARCH64_INSN_REG_11 = 11,
    AARCH64_INSN_REG_12 = 12,
    AARCH64_INSN_REG_13 = 13,
    AARCH64_INSN_REG_14 = 14,
    AARCH64_INSN_REG_15 = 15,
    AARCH64_INSN_REG_16 = 16,
    AARCH64_INSN_REG_17 = 17,
    AARCH64_INSN_REG_18 = 18,
    AARCH64_INSN_REG_19 = 19,
    AARCH64_INSN_REG_20 = 20,
    AARCH64_INSN_REG_21 = 21,
    AARCH64_INSN_REG_22 = 22,
    AARCH64_INSN_REG_23 = 23,
    AARCH64_INSN_REG_24 = 24,
    AARCH64_INSN_REG_25 = 25,
    AARCH64_INSN_REG_26 = 26,
    AARCH64_INSN_REG_27 = 27,
    AARCH64_INSN_REG_28 = 28,
    AARCH64_INSN_REG_29 = 29,
    AARCH64_INSN_REG_FP = 29, /* Frame pointer */
    AARCH64_INSN_REG_30 = 30,
    AARCH64_INSN_REG_LR = 30, /* Link register */
    AARCH64_INSN_REG_ZR = 31, /* Zero: as source register */
    AARCH64_INSN_REG_SP = 31  /* Stack pointer: as load/store base reg */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_special_register {
    AARCH64_INSN_SPCLREG_SPSR_EL1	= 0xC200,
    AARCH64_INSN_SPCLREG_ELR_EL1	= 0xC201,
    AARCH64_INSN_SPCLREG_SP_EL0	= 0xC208,
    AARCH64_INSN_SPCLREG_SPSEL	= 0xC210,
    AARCH64_INSN_SPCLREG_CURRENTEL	= 0xC212,
    AARCH64_INSN_SPCLREG_DAIF	= 0xDA11,
    AARCH64_INSN_SPCLREG_NZCV	= 0xDA10,
    AARCH64_INSN_SPCLREG_FPCR	= 0xDA20,
    AARCH64_INSN_SPCLREG_DSPSR_EL0	= 0xDA28,
    AARCH64_INSN_SPCLREG_DLR_EL0	= 0xDA29,
    AARCH64_INSN_SPCLREG_SPSR_EL2	= 0xE200,
    AARCH64_INSN_SPCLREG_ELR_EL2	= 0xE201,
    AARCH64_INSN_SPCLREG_SP_EL1	= 0xE208,
    AARCH64_INSN_SPCLREG_SPSR_INQ	= 0xE218,
    AARCH64_INSN_SPCLREG_SPSR_ABT	= 0xE219,
    AARCH64_INSN_SPCLREG_SPSR_UND	= 0xE21A,
    AARCH64_INSN_SPCLREG_SPSR_FIQ	= 0xE21B,
    AARCH64_INSN_SPCLREG_SPSR_EL3	= 0xF200,
    AARCH64_INSN_SPCLREG_ELR_EL3	= 0xF201,
    AARCH64_INSN_SPCLREG_SP_EL2	= 0xF210
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_system_register {
    AARCH64_INSN_SYSREG_TPIDR_EL1	= 0x4684,
    AARCH64_INSN_SYSREG_TPIDR_EL2	= 0x6682,
    AARCH64_INSN_SYSREG_SP_EL0	= 0x4208,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_variant {
    AARCH64_INSN_VARIANT_32BIT,
    AARCH64_INSN_VARIANT_64BIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_condition {
    AARCH64_INSN_COND_EQ = 0x0, /* == */
    AARCH64_INSN_COND_NE = 0x1, /* != */
    AARCH64_INSN_COND_CS = 0x2, /* unsigned >= */
    AARCH64_INSN_COND_CC = 0x3, /* unsigned < */
    AARCH64_INSN_COND_MI = 0x4, /* < 0 */
    AARCH64_INSN_COND_PL = 0x5, /* >= 0 */
    AARCH64_INSN_COND_VS = 0x6, /* overflow */
    AARCH64_INSN_COND_VC = 0x7, /* no overflow */
    AARCH64_INSN_COND_HI = 0x8, /* unsigned > */
    AARCH64_INSN_COND_LS = 0x9, /* unsigned <= */
    AARCH64_INSN_COND_GE = 0xa, /* signed >= */
    AARCH64_INSN_COND_LT = 0xb, /* signed < */
    AARCH64_INSN_COND_GT = 0xc, /* signed > */
    AARCH64_INSN_COND_LE = 0xd, /* signed <= */
    AARCH64_INSN_COND_AL = 0xe, /* always */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_branch_type {
    AARCH64_INSN_BRANCH_NOLINK,
    AARCH64_INSN_BRANCH_LINK,
    AARCH64_INSN_BRANCH_RETURN,
    AARCH64_INSN_BRANCH_COMP_ZERO,
    AARCH64_INSN_BRANCH_COMP_NONZERO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_size_type {
    AARCH64_INSN_SIZE_8,
    AARCH64_INSN_SIZE_16,
    AARCH64_INSN_SIZE_32,
    AARCH64_INSN_SIZE_64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_ldst_type {
    AARCH64_INSN_LDST_LOAD_REG_OFFSET,
    AARCH64_INSN_LDST_STORE_REG_OFFSET,
    AARCH64_INSN_LDST_LOAD_IMM_OFFSET,
    AARCH64_INSN_LDST_STORE_IMM_OFFSET,
    AARCH64_INSN_LDST_LOAD_PAIR_PRE_INDEX,
    AARCH64_INSN_LDST_STORE_PAIR_PRE_INDEX,
    AARCH64_INSN_LDST_LOAD_PAIR_POST_INDEX,
    AARCH64_INSN_LDST_STORE_PAIR_POST_INDEX,
    AARCH64_INSN_LDST_LOAD_ACQ,
    AARCH64_INSN_LDST_LOAD_EX,
    AARCH64_INSN_LDST_LOAD_ACQ_EX,
    AARCH64_INSN_LDST_STORE_REL,
    AARCH64_INSN_LDST_STORE_EX,
    AARCH64_INSN_LDST_STORE_REL_EX,
    AARCH64_INSN_LDST_SIGNED_LOAD_IMM_OFFSET,
    AARCH64_INSN_LDST_SIGNED_LOAD_REG_OFFSET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_adsb_type {
    AARCH64_INSN_ADSB_ADD,
    AARCH64_INSN_ADSB_SUB,
    AARCH64_INSN_ADSB_ADD_SETFLAGS,
    AARCH64_INSN_ADSB_SUB_SETFLAGS
}

// option field of add/sub (extended register)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_extend_type {
    AARCH64_INSN_EXTEND_UXTB,
    AARCH64_INSN_EXTEND_UXTH,
    AARCH64_INSN_EXTEND_UXTW,
    AARCH64_INSN_EXTEND_UXTX,
    AARCH64_INSN_EXTEND_SXTB,
    AARCH64_INSN_EXTEND_SXTH,
    AARCH64_INSN_EXTEND_SXTW,
    AARCH64_INSN_EXTEND_SXTX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_movewide_type {
    AARCH64_INSN_MOVEWIDE_ZERO,
    AARCH64_INSN_MOVEWIDE_KEEP,
    AARCH64_INSN_MOVEWIDE_INVERSE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_bitfield_type {
    AARCH64_INSN_BITFIELD_MOVE,
    AARCH64_INSN_BITFIELD_MOVE_UNSIGNED,
    AARCH64_INSN_BITFIELD_MOVE_SIGNED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_data1_type {
    AARCH64_INSN_DATA1_REVERSE_16,
    AARCH64_INSN_DATA1_REVERSE_32,
    AARCH64_INSN_DATA1_REVERSE_64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_data2_type {
    AARCH64_INSN_DATA2_UDIV,
    AARCH64_INSN_DATA2_SDIV,
    AARCH64_INSN_DATA2_LSLV,
    AARCH64_INSN_DATA2_LSRV,
    AARCH64_INSN_DATA2_ASRV,
    AARCH64_INSN_DATA2_RORV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_data3_type {
    AARCH64_INSN_DATA3_MADD,
    AARCH64_INSN_DATA3_MSUB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_logic_type {
    AARCH64_INSN_LOGIC_AND,
    AARCH64_INSN_LOGIC_BIC,
    AARCH64_INSN_LOGIC_ORR,
    AARCH64_INSN_LOGIC_ORN,
    AARCH64_INSN_LOGIC_EOR,
    AARCH64_INSN_LOGIC_EON,
    AARCH64_INSN_LOGIC_AND_SETFLAGS,
    AARCH64_INSN_LOGIC_BIC_SETFLAGS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_prfm_type {
    AARCH64_INSN_PRFM_TYPE_PLD,
    AARCH64_INSN_PRFM_TYPE_PLI,
    AARCH64_INSN_PRFM_TYPE_PST,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_prfm_target {
    AARCH64_INSN_PRFM_TARGET_L1,
    AARCH64_INSN_PRFM_TARGET_L2,
    AARCH64_INSN_PRFM_TARGET_L3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_prfm_policy {
    AARCH64_INSN_PRFM_POLICY_KEEP,
    AARCH64_INSN_PRFM_POLICY_STRM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_adr_type {
    AARCH64_INSN_ADR_TYPE_ADRP,
    AARCH64_INSN_ADR_TYPE_ADR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_mem_atomic_op {
    AARCH64_INSN_MEM_ATOMIC_ADD,
    AARCH64_INSN_MEM_ATOMIC_CLR,
    AARCH64_INSN_MEM_ATOMIC_EOR,
    AARCH64_INSN_MEM_ATOMIC_SET,
    AARCH64_INSN_MEM_ATOMIC_SWP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_mem_order_type {
    AARCH64_INSN_MEM_ORDER_NONE,
    AARCH64_INSN_MEM_ORDER_ACQ,
    AARCH64_INSN_MEM_ORDER_REL,
    AARCH64_INSN_MEM_ORDER_ACQREL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aarch64_insn_mb_type {
    AARCH64_INSN_MB_SY,
    AARCH64_INSN_MB_ST,
    AARCH64_INSN_MB_LD,
    AARCH64_INSN_MB_ISH,
    AARCH64_INSN_MB_ISHST,
    AARCH64_INSN_MB_ISHLD,
    AARCH64_INSN_MB_NSH,
    AARCH64_INSN_MB_NSHST,
    AARCH64_INSN_MB_NSHLD,
    AARCH64_INSN_MB_OSH,
    AARCH64_INSN_MB_OSHST,
    AARCH64_INSN_MB_OSHLD,
}

//
// ARM Architecture Reference Manual for ARMv8 Profile-A, Issue A.a
// Section C3.1 "A64 instruction index by encoding":
// AArch64 main encoding table
// Bit position
// 28 27 26 25	Encoding Group
// 0  0  -  -		Unallocated
// 1  0  0  -		Data processing, immediate
// 1  0  1  -		Branch, exception generation and system instructions
// -  1  -  0		Loads and stores
// -  1  0  1		Data processing - register
// 0  1  1  1		Data processing - SIMD and floating point
// 1  1  1  1		Data processing - SIMD and floating point
// "-" means "don't care"
//

// b, bl, cb*, tb*, ret*, b.cond, br*, blr*
// ldr/ldrsw (literal), prfm
extern "C" {
    pub fn aarch64_get_insn_class(insn: u32) -> aarch64_insn_encoding_class;
}
extern "C" {
    pub fn aarch64_insn_decode_immediate(type: aarch64_insn_imm_type, insn: u32) -> u64;
}
extern "C" {
    pub fn aarch64_insn_gen_hint(_arg: AARCH64_INSN_HINT_NOP) -> return;
}
extern "C" {
    pub fn aarch64_insn_gen_dmb(type: aarch64_insn_mb_type) -> u32;
}
extern "C" {
    pub fn aarch64_insn_gen_dsb(type: aarch64_insn_mb_type) -> u32;
}
extern "C" {
    pub fn aarch64_get_branch_offset(insn: u32) -> i32;
}
extern "C" {
    pub fn aarch64_set_branch_offset(insn: u32, offset: i32) -> u32;
}
extern "C" {
    pub fn aarch64_insn_adrp_get_offset(insn: u32) -> i32;
}
extern "C" {
    pub fn aarch64_insn_adrp_set_offset(insn: u32, offset: i32) -> u32;
}
extern "C" {
    pub fn aarch32_insn_is_wide(insn: u32) -> bool;
}
pub const A32_RN_OFFSET: c_int = 16;
pub const A32_RT_OFFSET: c_int = 12;
pub const A32_RT2_OFFSET: c_int = 0;
extern "C" {
    pub fn aarch64_insn_extract_system_reg(insn: u32) -> u32;
}
extern "C" {
    pub fn aarch32_insn_extract_reg_num(insn: u32, offset: c_int) -> u32;
}
extern "C" {
    pub fn aarch32_insn_mcr_extract_opc2(insn: u32) -> u32;
}
extern "C" {
    pub fn aarch32_insn_mcr_extract_crm(insn: u32) -> u32;
}
extern "C" {
    pub fn bool(long: pstate_check_t)(unsigned) -> typedef;
}

