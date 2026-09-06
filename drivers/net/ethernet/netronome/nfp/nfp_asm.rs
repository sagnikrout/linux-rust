//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_asm.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2016-2018 Netronome Systems, Inc.
pub const __NFP_ASM_H__: c_int = 1;

pub const REG_NONE: c_int = 0;
pub const REG_WIDTH: c_int = 4;
pub const RE_REG_NO_DST: c_uint = 0x020;
pub const RE_REG_IMM: c_uint = 0x020;

pub const RE_REG_IMM_MAX: c_uint = 0x07fULL;
pub const RE_REG_LM: c_uint = 0x050;
pub const RE_REG_LM_IDX: c_uint = 0x008;
pub const RE_REG_LM_IDX_MAX: c_uint = 0x7;
pub const RE_REG_XFR: c_uint = 0x080;
pub const UR_REG_XFR: c_uint = 0x180;
pub const UR_REG_LM: c_uint = 0x200;
pub const UR_REG_LM_IDX: c_uint = 0x020;
pub const UR_REG_LM_POST_MOD: c_uint = 0x010;
pub const UR_REG_LM_POST_MOD_DEC: c_uint = 0x001;
pub const UR_REG_LM_IDX_MAX: c_uint = 0xf;
pub const UR_REG_NN: c_uint = 0x280;
pub const UR_REG_NO_DST: c_uint = 0x300;

pub const UR_REG_IMM_MAX: c_uint = 0x0ffULL;
pub const OP_BR_BASE: c_uint = 0x0d800000020ULL;
pub const OP_BR_BASE_MASK: c_uint = 0x0f8000c3ce0ULL;
pub const OP_BR_MASK: c_uint = 0x0000000001fULL;
pub const OP_BR_EV_PIP: c_uint = 0x00000000300ULL;
pub const OP_BR_CSS: c_uint = 0x0000003c000ULL;
pub const OP_BR_DEFBR: c_uint = 0x00000300000ULL;
pub const OP_BR_ADDR_LO: c_uint = 0x007ffc00000ULL;
pub const OP_BR_ADDR_HI: c_uint = 0x10000000000ULL;
pub const OP_BR_BIT_BASE: c_uint = 0x0d000000000ULL;
pub const OP_BR_BIT_BASE_MASK: c_uint = 0x0f800080300ULL;
pub const OP_BR_BIT_A_SRC: c_uint = 0x000000000ffULL;
pub const OP_BR_BIT_B_SRC: c_uint = 0x0000003fc00ULL;
pub const OP_BR_BIT_BV: c_uint = 0x00000040000ULL;
pub const OP_BR_BIT_SRC_LMEXTN: c_uint = 0x40000000000ULL;

pub const OP_BR_ALU_BASE: c_uint = 0x0e800000000ULL;
pub const OP_BR_ALU_BASE_MASK: c_uint = 0x0ff80000000ULL;
pub const OP_BR_ALU_A_SRC: c_uint = 0x000000003ffULL;
pub const OP_BR_ALU_B_SRC: c_uint = 0x000000ffc00ULL;
pub const OP_BR_ALU_DEFBR: c_uint = 0x00000300000ULL;
pub const OP_BR_ALU_IMM_HI: c_uint = 0x0007fc00000ULL;
pub const OP_BR_ALU_SRC_LMEXTN: c_uint = 0x40000000000ULL;
pub const OP_BR_ALU_DST_LMEXTN: c_uint = 0x80000000000ULL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mask {
    BR_BEQ = 0x00,
    BR_BNE = 0x01,
    BR_BMI = 0x02,
    BR_BHS = 0x04,
    BR_BCC = 0x05,
    BR_BLO = 0x05,
    BR_BGE = 0x08,
    BR_BLT = 0x09,
    BR_UNC = 0x18,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_ev_pip {
    BR_EV_PIP_UNCOND = 0,
    BR_EV_PIP_COND = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_ctx_signal_state {
    BR_CSS_NONE = 2,
}

extern "C" {
    pub fn br_get_offset(instr: u64) -> u16;
}
extern "C" {
    pub fn br_set_offset(instr: *mut u64, offset: u16);
}
extern "C" {
    pub fn br_add_offset(instr: *mut u64, offset: u16);
}
pub const OP_BBYTE_BASE: c_uint = 0x0c800000000ULL;
pub const OP_BB_A_SRC: c_uint = 0x000000000ffULL;
pub const OP_BB_BYTE: c_uint = 0x00000000300ULL;
pub const OP_BB_B_SRC: c_uint = 0x0000003fc00ULL;
pub const OP_BB_I8: c_uint = 0x00000040000ULL;
pub const OP_BB_EQ: c_uint = 0x00000080000ULL;
pub const OP_BB_DEFBR: c_uint = 0x00000300000ULL;
pub const OP_BB_ADDR_LO: c_uint = 0x007ffc00000ULL;
pub const OP_BB_ADDR_HI: c_uint = 0x10000000000ULL;
pub const OP_BB_SRC_LMEXTN: c_uint = 0x40000000000ULL;
pub const OP_BALU_BASE: c_uint = 0x0e800000000ULL;
pub const OP_BA_A_SRC: c_uint = 0x000000003ffULL;
pub const OP_BA_B_SRC: c_uint = 0x000000ffc00ULL;
pub const OP_BA_DEFBR: c_uint = 0x00000300000ULL;
pub const OP_BA_ADDR_HI: c_uint = 0x0007fc00000ULL;
pub const OP_IMMED_A_SRC: c_uint = 0x000000003ffULL;
pub const OP_IMMED_B_SRC: c_uint = 0x000000ffc00ULL;
pub const OP_IMMED_IMM: c_uint = 0x0000ff00000ULL;
pub const OP_IMMED_WIDTH: c_uint = 0x00060000000ULL;
pub const OP_IMMED_INV: c_uint = 0x00080000000ULL;
pub const OP_IMMED_SHIFT: c_uint = 0x00600000000ULL;
pub const OP_IMMED_BASE: c_uint = 0x0f000000000ULL;
pub const OP_IMMED_WR_AB: c_uint = 0x20000000000ULL;
pub const OP_IMMED_SRC_LMEXTN: c_uint = 0x40000000000ULL;
pub const OP_IMMED_DST_LMEXTN: c_uint = 0x80000000000ULL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum immed_width {
    IMMED_WIDTH_ALL = 0,
    IMMED_WIDTH_BYTE = 1,
    IMMED_WIDTH_WORD = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum immed_shift {
    IMMED_SHIFT_0B = 0,
    IMMED_SHIFT_1B = 1,
    IMMED_SHIFT_2B = 2,
}

extern "C" {
    pub fn immed_get_value(instr: u64) -> u16;
}
extern "C" {
    pub fn immed_set_value(instr: *mut u64, immed: u16);
}
extern "C" {
    pub fn immed_add_value(instr: *mut u64, offset: u16);
}
pub const OP_SHF_BASE: c_uint = 0x08000000000ULL;
pub const OP_SHF_A_SRC: c_uint = 0x000000000ffULL;
pub const OP_SHF_SC: c_uint = 0x00000000300ULL;
pub const OP_SHF_B_SRC: c_uint = 0x0000003fc00ULL;
pub const OP_SHF_I8: c_uint = 0x00000040000ULL;
pub const OP_SHF_SW: c_uint = 0x00000080000ULL;
pub const OP_SHF_DST: c_uint = 0x0000ff00000ULL;
pub const OP_SHF_SHIFT: c_uint = 0x001f0000000ULL;
pub const OP_SHF_OP: c_uint = 0x00e00000000ULL;
pub const OP_SHF_DST_AB: c_uint = 0x01000000000ULL;
pub const OP_SHF_WR_AB: c_uint = 0x20000000000ULL;
pub const OP_SHF_SRC_LMEXTN: c_uint = 0x40000000000ULL;
pub const OP_SHF_DST_LMEXTN: c_uint = 0x80000000000ULL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum shf_op {
    SHF_OP_NONE = 0,
    SHF_OP_AND = 2,
    SHF_OP_OR = 5,
    SHF_OP_ASHR = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum shf_sc {
    SHF_SC_R_ROT = 0,
    SHF_SC_NONE = SHF_SC_R_ROT,
    SHF_SC_R_SHF = 1,
    SHF_SC_L_SHF = 2,
    SHF_SC_R_DSHF = 3,
}

pub const OP_ALU_A_SRC: c_uint = 0x000000003ffULL;
pub const OP_ALU_B_SRC: c_uint = 0x000000ffc00ULL;
pub const OP_ALU_DST: c_uint = 0x0003ff00000ULL;
pub const OP_ALU_SW: c_uint = 0x00040000000ULL;
pub const OP_ALU_OP: c_uint = 0x00f80000000ULL;
pub const OP_ALU_DST_AB: c_uint = 0x01000000000ULL;
pub const OP_ALU_BASE: c_uint = 0x0a000000000ULL;
pub const OP_ALU_WR_AB: c_uint = 0x20000000000ULL;
pub const OP_ALU_SRC_LMEXTN: c_uint = 0x40000000000ULL;
pub const OP_ALU_DST_LMEXTN: c_uint = 0x80000000000ULL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alu_op {
    ALU_OP_NONE		= 0x00,
    ALU_OP_ADD		= 0x01,
    ALU_OP_NOT		= 0x04,
    ALU_OP_ADD_2B		= 0x05,
    ALU_OP_AND		= 0x08,
    ALU_OP_AND_NOT_A	= 0x0c,
    ALU_OP_SUB_C		= 0x0d,
    ALU_OP_AND_NOT_B	= 0x10,
    ALU_OP_ADD_C		= 0x11,
    ALU_OP_OR		= 0x14,
    ALU_OP_SUB		= 0x15,
    ALU_OP_XOR		= 0x18,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alu_dst_ab {
    ALU_DST_A = 0,
    ALU_DST_B = 1,
}

pub const OP_LDF_BASE: c_uint = 0x0c000000000ULL;
pub const OP_LDF_A_SRC: c_uint = 0x000000000ffULL;
pub const OP_LDF_SC: c_uint = 0x00000000300ULL;
pub const OP_LDF_B_SRC: c_uint = 0x0000003fc00ULL;
pub const OP_LDF_I8: c_uint = 0x00000040000ULL;
pub const OP_LDF_SW: c_uint = 0x00000080000ULL;
pub const OP_LDF_ZF: c_uint = 0x00000100000ULL;
pub const OP_LDF_BMASK: c_uint = 0x0000f000000ULL;
pub const OP_LDF_SHF: c_uint = 0x001f0000000ULL;
pub const OP_LDF_WR_AB: c_uint = 0x20000000000ULL;
pub const OP_LDF_SRC_LMEXTN: c_uint = 0x40000000000ULL;
pub const OP_LDF_DST_LMEXTN: c_uint = 0x80000000000ULL;
pub const OP_CMD_A_SRC: c_uint = 0x000000000ffULL;
pub const OP_CMD_CTX: c_uint = 0x00000000300ULL;
pub const OP_CMD_B_SRC: c_uint = 0x0000003fc00ULL;
pub const OP_CMD_TOKEN: c_uint = 0x000000c0000ULL;
pub const OP_CMD_XFER: c_uint = 0x00001f00000ULL;
pub const OP_CMD_CNT: c_uint = 0x0000e000000ULL;
pub const OP_CMD_SIG: c_uint = 0x000f0000000ULL;
pub const OP_CMD_TGT_CMD: c_uint = 0x07f00000000ULL;
pub const OP_CMD_INDIR: c_uint = 0x20000000000ULL;
pub const OP_CMD_MODE: c_uint = 0x1c0000000000ULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd_tgt_act {
    pub token: u8,
    pub tgt_cmd: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_tgt_map {
    CMD_TGT_READ8,
    CMD_TGT_WRITE8_SWAP,
    CMD_TGT_WRITE32_SWAP,
    CMD_TGT_READ32,
    CMD_TGT_READ32_LE,
    CMD_TGT_READ32_SWAP,
    CMD_TGT_READ_LE,
    CMD_TGT_READ_SWAP_LE,
    CMD_TGT_ADD,
    CMD_TGT_ADD_IMM,
    __CMD_TGT_MAP_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_mode {
    CMD_MODE_40b_AB	= 0,
    CMD_MODE_40b_BA	= 1,
    CMD_MODE_32b	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmd_ctx_swap {
    CMD_CTX_SWAP = 0,
    CMD_CTX_SWAP_DEFER1 = 1,
    CMD_CTX_SWAP_DEFER2 = 2,
    CMD_CTX_NO_SWAP = 3,
}

pub const OP_LCSR_BASE: c_uint = 0x0fc00000000ULL;
pub const OP_LCSR_A_SRC: c_uint = 0x000000003ffULL;
pub const OP_LCSR_B_SRC: c_uint = 0x000000ffc00ULL;
pub const OP_LCSR_WRITE: c_uint = 0x00000200000ULL;
pub const OP_LCSR_ADDR: c_uint = 0x001ffc00000ULL;
pub const OP_LCSR_SRC_LMEXTN: c_uint = 0x40000000000ULL;
pub const OP_LCSR_DST_LMEXTN: c_uint = 0x80000000000ULL;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lcsr_wr_src {
    LCSR_WR_AREG,
    LCSR_WR_BREG,
    LCSR_WR_IMM,
}

pub const OP_CARB_BASE: c_uint = 0x0e000000000ULL;
pub const OP_CARB_OR: c_uint = 0x00000010000ULL;
pub const NFP_CSR_CTX_PTR: c_uint = 0x20;
pub const NFP_CSR_ACT_LM_ADDR0: c_uint = 0x64;
pub const NFP_CSR_ACT_LM_ADDR1: c_uint = 0x6c;
pub const NFP_CSR_ACT_LM_ADDR2: c_uint = 0x94;
pub const NFP_CSR_ACT_LM_ADDR3: c_uint = 0x9c;
pub const NFP_CSR_PSEUDO_RND_NUM: c_uint = 0x148;
// Software register representation, independent of operand type

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_bpf_reg_type {
    NN_REG_GPR_A =	BIT(0),
    NN_REG_GPR_B =	BIT(1),
    NN_REG_GPR_BOTH = NN_REG_GPR_A | NN_REG_GPR_B,
    NN_REG_NNR =	BIT(2),
    NN_REG_XFER =	BIT(3),
    NN_REG_IMM =	BIT(4),
    NN_REG_NONE =	BIT(5),
    NN_REG_LMEM =	BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_bpf_lm_mode {
    NN_LM_MOD_NONE = 0,
    NN_LM_MOD_INC,
    NN_LM_MOD_DEC,
}

pub type swreg = __u32 ;
extern "C" {
    pub fn FIELD_GET(_arg: NN_REG_TYPE, _arg: swreg_raw(reg)) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: NN_REG_VAL, _arg: swreg_raw(reg)) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: NN_REG_LM_IDX_LO, _arg: swreg_raw(reg)) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: NN_REG_LM_IDX_HI, _arg: swreg_raw(reg)) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: NN_REG_LM_MOD, _arg: swreg_raw(reg)) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_insn_ur_regs {
    pub dst_ab: alu_dst_ab,
    pub dst: u16,
    pub breg: u16 areg,,
    pub swap: bool,
    pub wr_both: bool,
    pub dst_lmextn: bool,
    pub src_lmextn: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_insn_re_regs {
    pub dst_ab: alu_dst_ab,
    pub dst: u8,
    pub breg: u8 areg,,
    pub swap: bool,
    pub wr_both: bool,
    pub i8: bool,
    pub dst_lmextn: bool,
    pub src_lmextn: bool,
}

pub const NFP_USTORE_PREFETCH_WINDOW: c_int = 8;
extern "C" {
    pub fn nfp_ustore_check_valid_no_ecc(insn: u64) -> c_int;
}
extern "C" {
    pub fn nfp_ustore_calc_ecc_insn(insn: u64) -> u64;
}
pub const NFP_IND_ME_REFL_WR_SIG_INIT: c_int = 3;

pub const NFP_IND_NUM_CONTEXTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mul_type {
    MUL_TYPE_START		= 0x00,
    MUL_TYPE_STEP_24x8	= 0x01,
    MUL_TYPE_STEP_16x16	= 0x02,
    MUL_TYPE_STEP_32x32	= 0x03,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mul_step {
    MUL_STEP_1		= 0x00,
    MUL_STEP_NONE		= MUL_STEP_1,
    MUL_STEP_2		= 0x01,
    MUL_STEP_3		= 0x02,
    MUL_STEP_4		= 0x03,
    MUL_LAST		= 0x04,
    MUL_LAST_2		= 0x05,
}

pub const OP_MUL_BASE: c_uint = 0x0f800000000ULL;
pub const OP_MUL_A_SRC: c_uint = 0x000000003ffULL;
pub const OP_MUL_B_SRC: c_uint = 0x000000ffc00ULL;
pub const OP_MUL_STEP: c_uint = 0x00000700000ULL;
pub const OP_MUL_DST_AB: c_uint = 0x00000800000ULL;
pub const OP_MUL_SW: c_uint = 0x00040000000ULL;
pub const OP_MUL_TYPE: c_uint = 0x00180000000ULL;
pub const OP_MUL_WR_AB: c_uint = 0x20000000000ULL;
pub const OP_MUL_SRC_LMEXTN: c_uint = 0x40000000000ULL;
pub const OP_MUL_DST_LMEXTN: c_uint = 0x80000000000ULL;
