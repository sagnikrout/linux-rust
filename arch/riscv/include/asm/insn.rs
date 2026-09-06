//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/insn.h
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
// Copyright (C) 2020 SiFive
//

pub const RV_INSN_FUNCT3_OPOFF: c_int = 12;

pub const RV_INSN_OPCODE_OPOFF: c_int = 0;
pub const RV_INSN_FUNCT12_OPOFF: c_int = 20;

// The bit field of immediate value in I-type instruction
pub const RV_I_IMM_SIGN_OPOFF: c_int = 31;
pub const RV_I_IMM_11_0_OPOFF: c_int = 20;
pub const RV_I_IMM_SIGN_OFF: c_int = 12;
pub const RV_I_IMM_11_0_OFF: c_int = 0;

// The bit field of immediate value in J-type instruction
pub const RV_J_IMM_SIGN_OPOFF: c_int = 31;
pub const RV_J_IMM_10_1_OPOFF: c_int = 21;
pub const RV_J_IMM_11_OPOFF: c_int = 20;
pub const RV_J_IMM_19_12_OPOFF: c_int = 12;
pub const RV_J_IMM_SIGN_OFF: c_int = 20;
pub const RV_J_IMM_10_1_OFF: c_int = 1;
pub const RV_J_IMM_11_OFF: c_int = 11;
pub const RV_J_IMM_19_12_OFF: c_int = 12;

//
// U-type IMMs contain the upper 20bits [31:20] of an immediate with
// the rest filled in by zeros, so no shifting required. Similarly,
// bit31 contains the signed state, so no sign extension necessary.
//
pub const RV_U_IMM_SIGN_OPOFF: c_int = 31;
pub const RV_U_IMM_31_12_OPOFF: c_int = 0;

// The bit field of immediate value in B-type instruction
pub const RV_B_IMM_SIGN_OPOFF: c_int = 31;
pub const RV_B_IMM_10_5_OPOFF: c_int = 25;
pub const RV_B_IMM_4_1_OPOFF: c_int = 8;
pub const RV_B_IMM_11_OPOFF: c_int = 7;
pub const RV_B_IMM_SIGN_OFF: c_int = 12;
pub const RV_B_IMM_10_5_OFF: c_int = 5;
pub const RV_B_IMM_4_1_OFF: c_int = 1;
pub const RV_B_IMM_11_OFF: c_int = 11;

// The register offset in RVG instruction
pub const RVG_RS1_OPOFF: c_int = 15;
pub const RVG_RS2_OPOFF: c_int = 20;
pub const RVG_RD_OPOFF: c_int = 7;

// The bit field of immediate value in RVC J instruction
pub const RVC_J_IMM_SIGN_OPOFF: c_int = 12;
pub const RVC_J_IMM_4_OPOFF: c_int = 11;
pub const RVC_J_IMM_9_8_OPOFF: c_int = 9;
pub const RVC_J_IMM_10_OPOFF: c_int = 8;
pub const RVC_J_IMM_6_OPOFF: c_int = 7;
pub const RVC_J_IMM_7_OPOFF: c_int = 6;
pub const RVC_J_IMM_3_1_OPOFF: c_int = 3;
pub const RVC_J_IMM_5_OPOFF: c_int = 2;
pub const RVC_J_IMM_SIGN_OFF: c_int = 11;
pub const RVC_J_IMM_4_OFF: c_int = 4;
pub const RVC_J_IMM_9_8_OFF: c_int = 8;
pub const RVC_J_IMM_10_OFF: c_int = 10;
pub const RVC_J_IMM_6_OFF: c_int = 6;
pub const RVC_J_IMM_7_OFF: c_int = 7;
pub const RVC_J_IMM_3_1_OFF: c_int = 1;
pub const RVC_J_IMM_5_OFF: c_int = 5;

// The bit field of immediate value in RVC B instruction
pub const RVC_B_IMM_SIGN_OPOFF: c_int = 12;
pub const RVC_B_IMM_4_3_OPOFF: c_int = 10;
pub const RVC_B_IMM_7_6_OPOFF: c_int = 5;
pub const RVC_B_IMM_2_1_OPOFF: c_int = 3;
pub const RVC_B_IMM_5_OPOFF: c_int = 2;
pub const RVC_B_IMM_SIGN_OFF: c_int = 8;
pub const RVC_B_IMM_4_3_OFF: c_int = 3;
pub const RVC_B_IMM_7_6_OFF: c_int = 6;
pub const RVC_B_IMM_2_1_OFF: c_int = 1;
pub const RVC_B_IMM_5_OFF: c_int = 5;

pub const RVC_INSN_FUNCT4_OPOFF: c_int = 12;

pub const RVC_INSN_FUNCT3_OPOFF: c_int = 13;

// The register offset in RVC op=C0 instruction
pub const RVC_C0_RS1_OPOFF: c_int = 7;
pub const RVC_C0_RS2_OPOFF: c_int = 2;
pub const RVC_C0_RD_OPOFF: c_int = 2;
// The register offset in RVC op=C1 instruction
pub const RVC_C1_RS1_OPOFF: c_int = 7;
pub const RVC_C1_RS2_OPOFF: c_int = 2;
pub const RVC_C1_RD_OPOFF: c_int = 7;
// The register offset in RVC op=C2 instruction
pub const RVC_C2_RS1_OPOFF: c_int = 7;
pub const RVC_C2_RS2_OPOFF: c_int = 2;
pub const RVC_C2_RD_OPOFF: c_int = 7;

// parts of opcode for RVG
pub const RVG_OPCODE_FENCE: c_uint = 0x0f;
pub const RVG_OPCODE_AUIPC: c_uint = 0x17;
pub const RVG_OPCODE_BRANCH: c_uint = 0x63;
pub const RVG_OPCODE_JALR: c_uint = 0x67;
pub const RVG_OPCODE_JAL: c_uint = 0x6f;
pub const RVG_OPCODE_SYSTEM: c_uint = 0x73;
pub const RVG_SYSTEM_CSR_OFF: c_int = 20;

// parts of opcode for RVF, RVD and RVQ
pub const RVFDQ_FL_FS_WIDTH_OFF: c_int = 12;

pub const RVFDQ_FL_FS_WIDTH_W: c_int = 2;
pub const RVFDQ_FL_FS_WIDTH_D: c_int = 3;
pub const RVFDQ_LS_FS_WIDTH_Q: c_int = 4;
pub const RVFDQ_OPCODE_FL: c_uint = 0x07;
pub const RVFDQ_OPCODE_FS: c_uint = 0x27;
// parts of opcode for RVV
pub const RVV_OPCODE_VECTOR: c_uint = 0x57;
pub const RVV_VL_VS_WIDTH_8: c_int = 0;
pub const RVV_VL_VS_WIDTH_16: c_int = 5;
pub const RVV_VL_VS_WIDTH_32: c_int = 6;
pub const RVV_VL_VS_WIDTH_64: c_int = 7;

// parts of opcode for RVC
pub const RVC_OPCODE_C0: c_uint = 0x0;
pub const RVC_OPCODE_C1: c_uint = 0x1;
pub const RVC_OPCODE_C2: c_uint = 0x2;
// parts of funct3 code for I, M, A extension
pub const RVG_FUNCT3_JALR: c_uint = 0x0;
pub const RVG_FUNCT3_BEQ: c_uint = 0x0;
pub const RVG_FUNCT3_BNE: c_uint = 0x1;
pub const RVG_FUNCT3_BLT: c_uint = 0x4;
pub const RVG_FUNCT3_BGE: c_uint = 0x5;
pub const RVG_FUNCT3_BLTU: c_uint = 0x6;
pub const RVG_FUNCT3_BGEU: c_uint = 0x7;
// parts of funct3 code for C extension
pub const RVC_FUNCT3_C_BEQZ: c_uint = 0x6;
pub const RVC_FUNCT3_C_BNEZ: c_uint = 0x7;
pub const RVC_FUNCT3_C_J: c_uint = 0x5;
pub const RVC_FUNCT3_C_JAL: c_uint = 0x1;
pub const RVC_FUNCT4_C_JR: c_uint = 0x8;
pub const RVC_FUNCT4_C_JALR: c_uint = 0x9;
pub const RVC_FUNCT4_C_EBREAK: c_uint = 0x9;
pub const RVG_FUNCT12_EBREAK: c_uint = 0x1;
pub const RVG_FUNCT12_SRET: c_uint = 0x102;

pub const RVC_MASK_C_EBREAK: c_uint = 0xffff;
pub const RVG_MASK_EBREAK: c_uint = 0xffffffff;
pub const RVG_MASK_SRET: c_uint = 0xffffffff;

// C.JAL is an RV32C-only instruction

pub const riscv_insn_is_c_jal(opcode): c_int = 0;

// special case to catch _any_ system instruction
// special case to catch _any_ branch instruction
pub const INSN_MATCH_LB: c_uint = 0x3;
pub const INSN_MASK_LB: c_uint = 0x707f;
pub const INSN_MATCH_LH: c_uint = 0x1003;
pub const INSN_MASK_LH: c_uint = 0x707f;
pub const INSN_MATCH_LW: c_uint = 0x2003;
pub const INSN_MASK_LW: c_uint = 0x707f;
pub const INSN_MATCH_LD: c_uint = 0x3003;
pub const INSN_MASK_LD: c_uint = 0x707f;
pub const INSN_MATCH_LBU: c_uint = 0x4003;
pub const INSN_MASK_LBU: c_uint = 0x707f;
pub const INSN_MATCH_LHU: c_uint = 0x5003;
pub const INSN_MASK_LHU: c_uint = 0x707f;
pub const INSN_MATCH_LWU: c_uint = 0x6003;
pub const INSN_MASK_LWU: c_uint = 0x707f;
pub const INSN_MATCH_SB: c_uint = 0x23;
pub const INSN_MASK_SB: c_uint = 0x707f;
pub const INSN_MATCH_SH: c_uint = 0x1023;
pub const INSN_MASK_SH: c_uint = 0x707f;
pub const INSN_MATCH_SW: c_uint = 0x2023;
pub const INSN_MASK_SW: c_uint = 0x707f;
pub const INSN_MATCH_SD: c_uint = 0x3023;
pub const INSN_MASK_SD: c_uint = 0x707f;
pub const INSN_MATCH_C_LD: c_uint = 0x6000;
pub const INSN_MASK_C_LD: c_uint = 0xe003;
pub const INSN_MATCH_C_SD: c_uint = 0xe000;
pub const INSN_MASK_C_SD: c_uint = 0xe003;
pub const INSN_MATCH_C_LW: c_uint = 0x4000;
pub const INSN_MASK_C_LW: c_uint = 0xe003;
pub const INSN_MATCH_C_SW: c_uint = 0xc000;
pub const INSN_MASK_C_SW: c_uint = 0xe003;
pub const INSN_MATCH_C_LDSP: c_uint = 0x6002;
pub const INSN_MASK_C_LDSP: c_uint = 0xe003;
pub const INSN_MATCH_C_SDSP: c_uint = 0xe002;
pub const INSN_MASK_C_SDSP: c_uint = 0xe003;
pub const INSN_MATCH_C_LWSP: c_uint = 0x4002;
pub const INSN_MASK_C_LWSP: c_uint = 0xe003;
pub const INSN_MATCH_C_SWSP: c_uint = 0xc002;
pub const INSN_MASK_C_SWSP: c_uint = 0xe003;
pub const INSN_OPCODE_MASK: c_uint = 0x007c;
pub const INSN_OPCODE_SHIFT: c_int = 2;
pub const INSN_OPCODE_SYSTEM: c_int = 28;
pub const INSN_MASK_WFI: c_uint = 0xffffffff;
pub const INSN_MATCH_WFI: c_uint = 0x10500073;
pub const INSN_MASK_WRS: c_uint = 0xffffffff;
pub const INSN_MATCH_WRS: c_uint = 0x00d00073;
pub const INSN_MATCH_CSRRW: c_uint = 0x1073;
pub const INSN_MASK_CSRRW: c_uint = 0x707f;
pub const INSN_MATCH_CSRRS: c_uint = 0x2073;
pub const INSN_MASK_CSRRS: c_uint = 0x707f;
pub const INSN_MATCH_CSRRC: c_uint = 0x3073;
pub const INSN_MASK_CSRRC: c_uint = 0x707f;
pub const INSN_MATCH_CSRRWI: c_uint = 0x5073;
pub const INSN_MASK_CSRRWI: c_uint = 0x707f;
pub const INSN_MATCH_CSRRSI: c_uint = 0x6073;
pub const INSN_MASK_CSRRSI: c_uint = 0x707f;
pub const INSN_MATCH_CSRRCI: c_uint = 0x7073;
pub const INSN_MASK_CSRRCI: c_uint = 0x707f;
pub const INSN_MATCH_FLW: c_uint = 0x2007;
pub const INSN_MASK_FLW: c_uint = 0x707f;
pub const INSN_MATCH_FLD: c_uint = 0x3007;
pub const INSN_MASK_FLD: c_uint = 0x707f;
pub const INSN_MATCH_FLQ: c_uint = 0x4007;
pub const INSN_MASK_FLQ: c_uint = 0x707f;
pub const INSN_MATCH_FSW: c_uint = 0x2027;
pub const INSN_MASK_FSW: c_uint = 0x707f;
pub const INSN_MATCH_FSD: c_uint = 0x3027;
pub const INSN_MASK_FSD: c_uint = 0x707f;
pub const INSN_MATCH_FSQ: c_uint = 0x4027;
pub const INSN_MASK_FSQ: c_uint = 0x707f;
pub const INSN_MATCH_C_FLD: c_uint = 0x2000;
pub const INSN_MASK_C_FLD: c_uint = 0xe003;
pub const INSN_MATCH_C_FLW: c_uint = 0x6000;
pub const INSN_MASK_C_FLW: c_uint = 0xe003;
pub const INSN_MATCH_C_FSD: c_uint = 0xa000;
pub const INSN_MASK_C_FSD: c_uint = 0xe003;
pub const INSN_MATCH_C_FSW: c_uint = 0xe000;
pub const INSN_MASK_C_FSW: c_uint = 0xe003;
pub const INSN_MATCH_C_FLDSP: c_uint = 0x2002;
pub const INSN_MASK_C_FLDSP: c_uint = 0xe003;
pub const INSN_MATCH_C_FSDSP: c_uint = 0xa002;
pub const INSN_MASK_C_FSDSP: c_uint = 0xe003;
pub const INSN_MATCH_C_FLWSP: c_uint = 0x6002;
pub const INSN_MASK_C_FLWSP: c_uint = 0xe003;
pub const INSN_MATCH_C_FSWSP: c_uint = 0xe002;
pub const INSN_MASK_C_FSWSP: c_uint = 0xe003;
pub const INSN_MATCH_C_LHU: c_uint = 0x8400;
pub const INSN_MASK_C_LHU: c_uint = 0xfc43;
pub const INSN_MATCH_C_LH: c_uint = 0x8440;
pub const INSN_MASK_C_LH: c_uint = 0xfc43;
pub const INSN_MATCH_C_SH: c_uint = 0x8c00;
pub const INSN_MASK_C_SH: c_uint = 0xfc43;
pub const INSN_16BIT_MASK: c_uint = 0x3;

pub const SH_RD: c_int = 7;
pub const SH_RS1: c_int = 15;
pub const SH_RS2: c_int = 20;
pub const SH_RS2C: c_int = 2;
pub const MASK_RX: c_uint = 0x1f;

pub const LOG_REGBYTES: c_int = 3;

pub const LOG_REGBYTES: c_int = 2;

pub const MASK_FUNCT3: c_uint = 0x7000;

//
// Get the immediate from a J-type instruction.
//
// @insn: instruction to process
// Return: immediate
//
extern "C" {
    pub fn RV_EXTRACT_JTYPE_IMM(_arg: insn) -> return;
}
//
// Update a J-type instruction with an immediate value.
//
// @insn: pointer to the jtype instruction
// @imm: the immediate to insert into the instruction
//
// drop the old IMMs, all jal IMM bits sit at 31:12
// insn &= ~GENMASK(31, 12);
// insn |= (RV_X_MASK(imm, RV_J_IMM_10_1_OFF, RV_J_IMM_10_1_MASK) << RV_J_IMM_10_1_OPOFF) |
//
// Put together one immediate from a U-type and I-type instruction pair.
//
// The U-type contains an upper immediate, meaning bits[31:12] with [11:0]
// being zero, while the I-type contains a 12bit immediate.
// Combined these can encode larger 32bit values and are used for example
// in auipc + jalr pairs to allow larger jumps.
//
// @utype_insn: instruction containing the upper immediate
// @itype_insn: instruction
// Return: combined immediate
//
// Update a set of two instructions (U-type + I-type) with an immediate value.
//
// Used for example in auipc+jalrs pairs the U-type instructions contains
// a 20bit upper immediate representing bits[31:12], while the I-type
// instruction contains a 12bit immediate representing bits[11:0].
//
// This also takes into account that both separate immediates are
// considered as signed values, so if the I-type immediate becomes
// negative (BIT(11) set) the U-type part gets adjusted.
//
// @utype_insn: pointer to the utype instruction of the pair
// @itype_insn: pointer to the itype instruction of the pair
// @imm: the immediate to insert into the two instructions
//
// drop possible old IMM values
// utype_insn &= ~(RV_U_IMM_31_12_MASK);
// itype_insn &= ~(RV_I_IMM_11_0_MASK << RV_I_IMM_11_0_OPOFF);
// add the adapted IMMs
// utype_insn |= (imm & RV_U_IMM_31_12_MASK) + ((imm & BIT(11)) << 1);
// itype_insn |= ((imm & RV_I_IMM_11_0_MASK) << RV_I_IMM_11_0_OPOFF);
