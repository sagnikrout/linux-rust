//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/inst.h
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

pub const INSN_NOP: c_uint = 0x03400000;
pub const INSN_BREAK: c_uint = 0x002a0000;
pub const INSN_HVCL: c_uint = 0x002b8000;
pub const ADDR_IMMMASK_LU52ID: c_uint = 0xFFF0000000000000;
pub const ADDR_IMMMASK_LU32ID: c_uint = 0x000FFFFF00000000;
pub const ADDR_IMMMASK_LU12IW: c_uint = 0x00000000FFFFF000;
pub const ADDR_IMMMASK_ORI: c_uint = 0x0000000000000FFF;
pub const ADDR_IMMMASK_ADDU16ID: c_uint = 0x00000000FFFF0000;
pub const ADDR_IMMSHIFT_LU52ID: c_int = 52;
pub const ADDR_IMMSBIDX_LU52ID: c_int = 11;
pub const ADDR_IMMSHIFT_LU32ID: c_int = 32;
pub const ADDR_IMMSBIDX_LU32ID: c_int = 19;
pub const ADDR_IMMSHIFT_LU12IW: c_int = 12;
pub const ADDR_IMMSBIDX_LU12IW: c_int = 19;
pub const ADDR_IMMSHIFT_ORI: c_int = 0;
pub const ADDR_IMMSBIDX_ORI: c_int = 63;
pub const ADDR_IMMSHIFT_ADDU16ID: c_int = 16;
pub const ADDR_IMMSBIDX_ADDU16ID: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg0i15_op {
    break_op	= 0x54,
    dbar_op		= 0x70e4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg0i26_op {
    b_op		= 0x14,
    bl_op		= 0x15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg1i20_op {
    lu12iw_op	= 0x0a,
    lu32id_op	= 0x0b,
    pcaddi_op	= 0x0c,
    pcalau12i_op	= 0x0d,
    pcaddu12i_op	= 0x0e,
    pcaddu18i_op	= 0x0f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg1i21_op {
    beqz_op		= 0x10,
    bnez_op		= 0x11,
    bceqz_op	= 0x12, /* bits[9:8] = 0x00 */
    bcnez_op	= 0x12, /* bits[9:8] = 0x01 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2_op {
    revb2h_op	= 0x0c,
    revb4h_op	= 0x0d,
    revb2w_op	= 0x0e,
    revbd_op	= 0x0f,
    revh2w_op	= 0x10,
    revhd_op	= 0x11,
    extwh_op	= 0x16,
    extwb_op	= 0x17,
    cpucfg_op	= 0x1b,
    iocsrrdb_op     = 0x19200,
    iocsrrdh_op     = 0x19201,
    iocsrrdw_op     = 0x19202,
    iocsrrdd_op     = 0x19203,
    iocsrwrb_op     = 0x19204,
    iocsrwrh_op     = 0x19205,
    iocsrwrw_op     = 0x19206,
    iocsrwrd_op     = 0x19207,
    llacqw_op	= 0xe15e0,
    screlw_op	= 0xe15e1,
    llacqd_op	= 0xe15e2,
    screld_op	= 0xe15e3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2i5_op {
    slliw_op	= 0x81,
    srliw_op	= 0x89,
    sraiw_op	= 0x91,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2i6_op {
    sllid_op	= 0x41,
    srlid_op	= 0x45,
    sraid_op	= 0x49,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2i12_op {
    sltui_op	= 0x09,
    addiw_op	= 0x0a,
    addid_op	= 0x0b,
    lu52id_op	= 0x0c,
    andi_op		= 0x0d,
    ori_op		= 0x0e,
    xori_op		= 0x0f,
    ldb_op		= 0xa0,
    ldh_op		= 0xa1,
    ldw_op		= 0xa2,
    ldd_op		= 0xa3,
    stb_op		= 0xa4,
    sth_op		= 0xa5,
    stw_op		= 0xa6,
    std_op		= 0xa7,
    ldbu_op		= 0xa8,
    ldhu_op		= 0xa9,
    ldwu_op		= 0xaa,
    flds_op		= 0xac,
    fsts_op		= 0xad,
    fldd_op		= 0xae,
    fstd_op		= 0xaf,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2i14_op {
    llw_op		= 0x20,
    scw_op		= 0x21,
    lld_op		= 0x22,
    scd_op		= 0x23,
    ldptrw_op	= 0x24,
    stptrw_op	= 0x25,
    ldptrd_op	= 0x26,
    stptrd_op	= 0x27,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2i16_op {
    jirl_op		= 0x13,
    beq_op		= 0x16,
    bne_op		= 0x17,
    blt_op		= 0x18,
    bge_op		= 0x19,
    bltu_op		= 0x1a,
    bgeu_op		= 0x1b,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg2bstrd_op {
    bstrinsd_op	= 0x2,
    bstrpickd_op	= 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg3_op {
    asrtle_op	= 0x02,
    asrtgt_op	= 0x03,
    addw_op		= 0x20,
    addd_op		= 0x21,
    subw_op		= 0x22,
    subd_op		= 0x23,
    maskeqz_op	= 0x26,
    masknez_op	= 0x27,
    nor_op		= 0x28,
    and_op		= 0x29,
    or_op		= 0x2a,
    xor_op		= 0x2b,
    orn_op		= 0x2c,
    andn_op		= 0x2d,
    sllw_op		= 0x2e,
    srlw_op		= 0x2f,
    sraw_op		= 0x30,
    slld_op		= 0x31,
    srld_op		= 0x32,
    srad_op		= 0x33,
    mulw_op		= 0x38,
    mulhw_op	= 0x39,
    mulhwu_op	= 0x3a,
    muld_op		= 0x3b,
    mulhd_op	= 0x3c,
    mulhdu_op	= 0x3d,
    divw_op		= 0x40,
    modw_op		= 0x41,
    divwu_op	= 0x42,
    modwu_op	= 0x43,
    divd_op		= 0x44,
    modd_op		= 0x45,
    divdu_op	= 0x46,
    moddu_op	= 0x47,
    ldxb_op		= 0x7000,
    ldxh_op		= 0x7008,
    ldxw_op		= 0x7010,
    ldxd_op		= 0x7018,
    stxb_op		= 0x7020,
    stxh_op		= 0x7028,
    stxw_op		= 0x7030,
    stxd_op		= 0x7038,
    ldxbu_op	= 0x7040,
    ldxhu_op	= 0x7048,
    ldxwu_op	= 0x7050,
    fldxs_op	= 0x7060,
    fldxd_op	= 0x7068,
    fstxs_op	= 0x7070,
    fstxd_op	= 0x7078,
    scq_op		= 0x70ae,
    amswapb_op	= 0x70b8,
    amswaph_op	= 0x70b9,
    amaddb_op	= 0x70ba,
    amaddh_op	= 0x70bb,
    amswapdbb_op	= 0x70bc,
    amswapdbh_op	= 0x70bd,
    amadddbb_op	= 0x70be,
    amadddbh_op	= 0x70bf,
    amswapw_op	= 0x70c0,
    amswapd_op	= 0x70c1,
    amaddw_op	= 0x70c2,
    amaddd_op	= 0x70c3,
    amandw_op	= 0x70c4,
    amandd_op	= 0x70c5,
    amorw_op	= 0x70c6,
    amord_op	= 0x70c7,
    amxorw_op	= 0x70c8,
    amxord_op	= 0x70c9,
    ammaxw_op	= 0x70ca,
    ammaxd_op	= 0x70cb,
    amminw_op	= 0x70cc,
    ammind_op	= 0x70cd,
    ammaxwu_op	= 0x70ce,
    ammaxdu_op	= 0x70cf,
    amminwu_op	= 0x70d0,
    ammindu_op	= 0x70d1,
    amswapdbw_op	= 0x70d2,
    amswapdbd_op	= 0x70d3,
    amadddbw_op	= 0x70d4,
    amadddbd_op	= 0x70d5,
    amanddbw_op	= 0x70d6,
    amanddbd_op	= 0x70d7,
    amordbw_op	= 0x70d8,
    amordbd_op	= 0x70d9,
    amxordbw_op	= 0x70da,
    amxordbd_op	= 0x70db,
    ammaxdbw_op	= 0x70dc,
    ammaxdbd_op	= 0x70dd,
    ammindbw_op	= 0x70de,
    ammindbd_op	= 0x70df,
    ammaxdbwu_op	= 0x70e0,
    ammaxdbdu_op	= 0x70e1,
    ammindbwu_op	= 0x70e2,
    ammindbdu_op	= 0x70e3,
    fldgts_op	= 0x70e8,
    fldgtd_op	= 0x70e9,
    fldles_op	= 0x70ea,
    fldled_op	= 0x70eb,
    fstgts_op	= 0x70ec,
    fstgtd_op	= 0x70ed,
    fstles_op	= 0x70ee,
    fstled_op	= 0x70ef,
    ldgtb_op	= 0x70f0,
    ldgth_op	= 0x70f1,
    ldgtw_op	= 0x70f2,
    ldgtd_op	= 0x70f3,
    ldleb_op	= 0x70f4,
    ldleh_op	= 0x70f5,
    ldlew_op	= 0x70f6,
    ldled_op	= 0x70f7,
    stgtb_op	= 0x70f8,
    stgth_op	= 0x70f9,
    stgtw_op	= 0x70fa,
    stgtd_op	= 0x70fb,
    stleb_op	= 0x70fc,
    stleh_op	= 0x70fd,
    stlew_op	= 0x70fe,
    stled_op	= 0x70ff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum reg3sa2_op {
    alslw_op	= 0x02,
    alslwu_op	= 0x03,
    alsld_op	= 0x16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg0i15_format {
    pub 15: unsigned int immediate :,
    pub 17: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg0i26_format {
    pub 10: unsigned int immediate_h :,
    pub 16: unsigned int immediate_l :,
    pub 6: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg1i20_format {
    pub 5: unsigned int rd :,
    pub 20: unsigned int immediate :,
    pub 7: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg1i21_format {
    pub 5: unsigned int immediate_h :,
    pub 5: unsigned int rj :,
    pub 16: unsigned int immediate_l :,
    pub 6: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 22: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i5_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 5: unsigned int immediate :,
    pub 17: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i6_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 6: unsigned int immediate :,
    pub 16: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i12_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 12: unsigned int immediate :,
    pub 10: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i14_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 14: unsigned int immediate :,
    pub 8: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2i16_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 16: unsigned int immediate :,
    pub 6: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2bstrd_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 6: unsigned int lsbd :,
    pub 6: unsigned int msbd :,
    pub 10: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg2csr_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 14: unsigned int csr :,
    pub 8: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg3_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 5: unsigned int rk :,
    pub 17: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg3sa2_format {
    pub 5: unsigned int rd :,
    pub 5: unsigned int rj :,
    pub 5: unsigned int rk :,
    pub 2: unsigned int immediate :,
    pub 15: unsigned int opcode :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union loongarch_instruction {
    pub word: c_uint,
    pub reg0i15_format: reg0i15_format,
    pub reg0i26_format: reg0i26_format,
    pub reg1i20_format: reg1i20_format,
    pub reg1i21_format: reg1i21_format,
    pub reg2_format: reg2_format,
    pub reg2i5_format: reg2i5_format,
    pub reg2i6_format: reg2i6_format,
    pub reg2i12_format: reg2i12_format,
    pub reg2i14_format: reg2i14_format,
    pub reg2i16_format: reg2i16_format,
    pub reg2bstrd_format: reg2bstrd_format,
    pub reg2csr_format: reg2csr_format,
    pub reg3_format: reg3_format,
    pub reg3sa2_format: reg3sa2_format,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum loongarch_gpr {
    LOONGARCH_GPR_ZERO = 0,
    LOONGARCH_GPR_RA = 1,
    LOONGARCH_GPR_TP = 2,
    LOONGARCH_GPR_SP = 3,
    LOONGARCH_GPR_A0 = 4,	/* Reused as V0 for return value */
    LOONGARCH_GPR_A1,	/* Reused as V1 for return value */
    LOONGARCH_GPR_A2,
    LOONGARCH_GPR_A3,
    LOONGARCH_GPR_A4,
    LOONGARCH_GPR_A5,
    LOONGARCH_GPR_A6,
    LOONGARCH_GPR_A7,
    LOONGARCH_GPR_T0 = 12,
    LOONGARCH_GPR_T1,
    LOONGARCH_GPR_T2,
    LOONGARCH_GPR_T3,
    LOONGARCH_GPR_T4,
    LOONGARCH_GPR_T5,
    LOONGARCH_GPR_T6,
    LOONGARCH_GPR_T7,
    LOONGARCH_GPR_T8,
    LOONGARCH_GPR_U0 = 21,	/* Kernel per-CPU base register ($r21) */
    LOONGARCH_GPR_FP = 22,
    LOONGARCH_GPR_S0 = 23,
    LOONGARCH_GPR_S1,
    LOONGARCH_GPR_S2,
    LOONGARCH_GPR_S3,
    LOONGARCH_GPR_S4,
    LOONGARCH_GPR_S5,
    LOONGARCH_GPR_S6,
    LOONGARCH_GPR_S7,
    LOONGARCH_GPR_S8,
    LOONGARCH_GPR_MAX
}

// st.w / st.d $ra, $sp, offset
// addi.w / addi.d $sp, $sp, -imm
extern "C" {
    pub fn simu_pc(regs: *mut pt_regs, insn: loongarch_instruction);
}
extern "C" {
    pub fn simu_branch(regs: *mut pt_regs, insn: loongarch_instruction);
}
extern "C" {
    pub fn insns_not_supported(insn: loongarch_instruction) -> bool;
}
extern "C" {
    pub fn insns_need_simulation(insn: loongarch_instruction) -> bool;
}
extern "C" {
    pub fn arch_simulate_insn(insn: loongarch_instruction, regs: *mut pt_regs);
}
extern "C" {
    pub fn larch_insn_read(addr: *mut c_void, insnp: *mut u32) -> c_int;
}
extern "C" {
    pub fn larch_insn_write(addr: *mut c_void, insn: u32) -> c_int;
}
extern "C" {
    pub fn larch_insn_patch_text(addr: *mut c_void, insn: u32) -> c_int;
}
extern "C" {
    pub fn larch_insn_text_copy(dst: *mut c_void, src: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn larch_insn_gen_nop() -> u32;
}
extern "C" {
    pub fn larch_insn_gen_b(pc: c_ulong, dest: c_ulong) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_bl(pc: c_ulong, dest: c_ulong) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_break(imm: c_int) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_or(rd: loongarch_gpr, rj: loongarch_gpr, rk: loongarch_gpr) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_move(rd: loongarch_gpr, rj: loongarch_gpr) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_lu12iw(rd: loongarch_gpr, imm: c_int) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_lu32id(rd: loongarch_gpr, imm: c_int) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_lu52id(rd: loongarch_gpr, rj: loongarch_gpr, imm: c_int) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_beq(rd: loongarch_gpr, rj: loongarch_gpr, imm: c_int) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_bne(rd: loongarch_gpr, rj: loongarch_gpr, imm: c_int) -> u32;
}
extern "C" {
    pub fn larch_insn_gen_jirl(rd: loongarch_gpr, rj: loongarch_gpr, imm: c_int) -> u32;
}

// like emit_break(imm) but returns a constant expression

extern "C" {
    pub fn emulate_load_store_insn(regs: *mut pt_regs, addr: *mut void __user, pc: *mut c_uint);
}
extern "C" {
    pub fn unaligned_read(addr: *mut void __user, value: *mut c_void, n: c_ulong, sign: bool) -> c_ulong;
}
extern "C" {
    pub fn unaligned_write(addr: *mut void __user, value: c_ulong, n: c_ulong) -> c_ulong;
}
