//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx2/otx2_cpt_hw_types.h
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
// Copyright (C) 2020 Marvell.
//

// Device IDs
pub const OTX2_CPT_PCI_PF_DEVICE_ID: c_uint = 0xA0FD;
pub const OTX2_CPT_PCI_VF_DEVICE_ID: c_uint = 0xA0FE;
pub const CN10K_CPT_PCI_PF_DEVICE_ID: c_uint = 0xA0F2;
pub const CN10K_CPT_PCI_VF_DEVICE_ID: c_uint = 0xA0F3;
pub const CPT_PCI_SUBSYS_DEVID_CN10K_A: c_uint = 0xB900;
pub const CPT_PCI_SUBSYS_DEVID_CN10K_B: c_uint = 0xBD00;
// Mailbox interrupts offset
pub const OTX2_CPT_PF_MBOX_INT: c_int = 6;

// Maximum supported microcode groups
pub const OTX2_CPT_MAX_ENGINE_GROUPS: c_int = 8;
// CPT instruction size in bytes
pub const OTX2_CPT_INST_SIZE: c_int = 64;
//
// CPT VF MSIX vectors and their offsets
//
pub const OTX2_CPT_VF_MSIX_VECTORS: c_int = 1;

// CPT LF MSIX vectors
pub const OTX2_CPT_LF_MSIX_VECTORS: c_int = 2;
// OcteonTX2 CPT PF registers

// OcteonTX2 CPT LF registers

pub const OTX2_CPT_RVU_FUNC_BLKADDR_SHIFT: c_int = 20;
// LMT LF registers

// RVU VF registers

//
// Enumeration otx2_cpt_ucode_error_code_e
//
// Enumerates ucode errors
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_cpt_ucode_comp_code_e {
    OTX2_CPT_UCC_SUCCESS = 0x00,
    OTX2_CPT_UCC_INVALID_OPCODE = 0x01,

// Scatter gather
    OTX2_CPT_UCC_SG_WRITE_LENGTH = 0x02,
    OTX2_CPT_UCC_SG_LIST = 0x03,
    OTX2_CPT_UCC_SG_NOT_SUPPORTED = 0x04,

}

//
// Enumeration otx2_cpt_comp_e
//
// OcteonTX2 CPT Completion Enumeration
// Enumerates the values of CPT_RES_S[COMPCODE].
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_cpt_comp_e {
    OTX2_CPT_COMP_E_NOTDONE = 0x00,
    OTX2_CPT_COMP_E_GOOD = 0x01,
    OTX2_CPT_COMP_E_FAULT = 0x02,
    OTX2_CPT_COMP_E_HWERR = 0x04,
    OTX2_CPT_COMP_E_INSTERR = 0x05,
    OTX2_CPT_COMP_E_WARN = 0x06
}

//
// Enumeration otx2_cpt_vf_int_vec_e
//
// OcteonTX2 CPT VF MSI-X Vector Enumeration
// Enumerates the MSI-X interrupt vectors.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_cpt_vf_int_vec_e {
    OTX2_CPT_VF_INT_VEC_E_MBOX = 0x00
}

//
// Enumeration otx2_cpt_lf_int_vec_e
//
// OcteonTX2 CPT LF MSI-X Vector Enumeration
// Enumerates the MSI-X interrupt vectors.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_cpt_lf_int_vec_e {
    OTX2_CPT_LF_INT_VEC_E_MISC = 0x00,
    OTX2_CPT_LF_INT_VEC_E_DONE = 0x01
}

//
// Structure otx2_cpt_inst_s
//
// CPT Instruction Structure
// This structure specifies the instruction layout. Instructions are
// stored in memory as little-endian unless CPT()_PF_Q()_CTL[INST_BE] is set.
// cpt_inst_s_s
// Word 0
// doneint:1 Done interrupt.
// 0 = No interrupts related to this instruction.
// 1 = When the instruction completes, CPT()_VQ()_DONE[DONE] will be
// incremented,and based on the rules described there an interrupt may
// occur.
// Word 1
// res_addr [127: 64] Result IOVA.
// If nonzero, specifies where to write CPT_RES_S.
// If zero, no result structure will be written.
// Address must be 16-byte aligned.
// Bits <63:49> are ignored by hardware; software should use a
// sign-extended bit <48> for forward compatibility.
// Word 2
// grp:10 [171:162] If [WQ_PTR] is nonzero, the SSO guest-group to use when
// CPT submits work SSO.
// For the SSO to not discard the add-work request, FPA_PF_MAP() must map
// [GRP] and CPT()_PF_Q()_GMCTL[GMID] as valid.
// tt:2 [161:160] If [WQ_PTR] is nonzero, the SSO tag type to use when CPT
// submits work to SSO
// tag:32 [159:128] If [WQ_PTR] is nonzero, the SSO tag to use when CPT
// submits work to SSO.
// Word 3
// wq_ptr [255:192] If [WQ_PTR] is nonzero, it is a pointer to a
// work-queue entry that CPT submits work to SSO after all context,
// output data, and result write operations are visible to other
// CNXXXX units and the cores. Bits <2:0> must be zero.
// Bits <63:49> are ignored by hardware; software should
// use a sign-extended bit <48> for forward compatibility.
// Internal:
// Bits <63:49>, <2:0> are ignored by hardware, treated as always 0x0.
// Word 4
// ei0; [319:256] Engine instruction word 0. Passed to the AE/SE.
// Word 5
// ei1; [383:320] Engine instruction word 1. Passed to the AE/SE.
// Word 6
// ei2; [447:384] Engine instruction word 1. Passed to the AE/SE.
// Word 7
// ei3; [511:448] Engine instruction word 1. Passed to the AE/SE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cpt_inst_s {
    pub u: [u64; 8],
// Word 0
    pub nixtxl:3: u64,
    pub doneint:1: u64,
    pub nixtx_addr:60: u64,
// Word 1
    pub res_addr: u64,
// Word 2
    pub tag:32: u64,
    pub tt:2: u64,
    pub grp:10: u64,
    pub reserved_172_175:4: u64,
    pub rvu_pf_func:16: u64,
// Word 3
    pub qord:1: u64,
    pub reserved_194_193:2: u64,
    pub wq_ptr:61: u64,
// Word 4
    pub ei0: u64,
// Word 5
    pub ei1: u64,
// Word 6
    pub ei2: u64,
// Word 7
    pub ei3: u64,
    pub s: },
}

//
// Structure otx2_cpt_res_s
//
// CPT Result Structure
// The CPT coprocessor writes the result structure after it completes a
// CPT_INST_S instruction. The result structure is exactly 16 bytes, and
// each instruction completion produces exactly one result structure.
//
// This structure is stored in memory as little-endian unless
// CPT()_PF_Q()_CTL[INST_BE] is set.
// cpt_res_s_s
// Word 0
// doneint:1 [16:16] Done interrupt. This bit is copied from the
// corresponding instruction's CPT_INST_S[DONEINT].
// compcode:8 [7:0] Indicates completion/error status of the CPT coprocessor
// for the	associated instruction, as enumerated by CPT_COMP_E.
// Core software may write the memory location containing [COMPCODE] to
// 0x0 before ringing the doorbell, and then poll for completion by
// checking for a nonzero value.
// Once the core observes a nonzero [COMPCODE] value in this case,the CPT
// coprocessor will have also completed L2/DRAM write operations.
// Word 1
// reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cpt_res_s {
    pub u: [u64; 2],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn9k_cpt_res_s {
    pub compcode:8: u64,
    pub uc_compcode:8: u64,
    pub doneint:1: u64,
    pub reserved_17_63:47: u64,
    pub reserved_64_127: u64,
    pub s: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cn10k_cpt_res_s {
    pub compcode:7: u64,
    pub doneint:1: u64,
    pub uc_compcode:8: u64,
    pub rlen:16: u64,
    pub spi:32: u64,
    pub esn: u64,
    pub cn10k: },
}

//
// Register (RVU_PF_BAR0) cpt#_af_constants1
//
// CPT AF Constants Register
// This register contains implementation-related parameters of CPT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_af_constants1 {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_af_constants1_s {
    pub se:16: u64,
    pub ie:16: u64,
    pub ae:16: u64,
    pub reserved_48_63:16: u64,
    pub s: },
}

//
// RVU_PFVF_BAR2 - cpt_lf_misc_int
//
// This register contain the per-queue miscellaneous interrupts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_lf_misc_int {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_lf_misc_int_s {
    pub reserved_0:1: u64,
    pub nqerr:1: u64,
    pub irde:1: u64,
    pub nwrp:1: u64,
    pub reserved_4:1: u64,
    pub hwerr:1: u64,
    pub fault:1: u64,
    pub reserved_7_63:57: u64,
    pub s: },
}

//
// RVU_PFVF_BAR2 - cpt_lf_misc_int_ena_w1s
//
// This register sets interrupt enable bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_lf_misc_int_ena_w1s {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_lf_misc_int_ena_w1s_s {
    pub reserved_0:1: u64,
    pub nqerr:1: u64,
    pub irde:1: u64,
    pub nwrp:1: u64,
    pub reserved_4:1: u64,
    pub hwerr:1: u64,
    pub fault:1: u64,
    pub reserved_7_63:57: u64,
    pub s: },
}

//
// RVU_PFVF_BAR2 - cpt_lf_ctl
//
// This register configures the queue.
//
// When the queue is not execution-quiescent (see CPT_LF_INPROG[EENA,INFLIGHT]),
// software must only write this register with [ENA]=0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_lf_ctl {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_lf_ctl_s {
    pub ena:1: u64,
    pub fc_ena:1: u64,
    pub fc_up_crossing:1: u64,
    pub reserved_3:1: u64,
    pub fc_hyst_bits:4: u64,
    pub reserved_8_63:56: u64,
    pub s: },
}

//
// RVU_PFVF_BAR2 - cpt_lf_done_wait
//
// This register specifies the per-queue interrupt coalescing settings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_lf_done_wait {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_lf_done_wait_s {
    pub num_wait:20: u64,
    pub reserved_20_31:12: u64,
    pub time_wait:16: u64,
    pub reserved_48_63:16: u64,
    pub s: },
}

//
// RVU_PFVF_BAR2 - cpt_lf_done
//
// This register contain the per-queue instruction done count.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_lf_done {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_lf_done_s {
    pub done:20: u64,
    pub reserved_20_63:44: u64,
    pub s: },
}

//
// RVU_PFVF_BAR2 - cpt_lf_inprog
//
// These registers contain the per-queue instruction in flight registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_lf_inprog {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_lf_inprog_s {
    pub inflight:9: u64,
    pub reserved_9_15:7: u64,
    pub eena:1: u64,
    pub grp_drp:1: u64,
    pub reserved_18_30:13: u64,
    pub grb_partial:1: u64,
    pub grb_cnt:8: u64,
    pub gwb_cnt:8: u64,
    pub reserved_48_63:16: u64,
    pub s: },
}

//
// RVU_PFVF_BAR2 - cpt_lf_q_base
//
// CPT initializes these CSR fields to these values on any CPT_LF_Q_BASE write:
// _ CPT_LF_Q_INST_PTR[XQ_XOR]=0.
// _ CPT_LF_Q_INST_PTR[NQ_PTR]=2.
// _ CPT_LF_Q_INST_PTR[DQ_PTR]=2.
// _ CPT_LF_Q_GRP_PTR[XQ_XOR]=0.
// _ CPT_LF_Q_GRP_PTR[NQ_PTR]=1.
// _ CPT_LF_Q_GRP_PTR[DQ_PTR]=1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_lf_q_base {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_lf_q_base_s {
    pub fault:1: u64,
    pub reserved_1_6:6: u64,
    pub addr:46: u64,
    pub reserved_53_63:11: u64,
    pub s: },
}

//
// RVU_PFVF_BAR2 - cpt_lf_q_size
//
// CPT initializes these CSR fields to these values on any CPT_LF_Q_SIZE write:
// _ CPT_LF_Q_INST_PTR[XQ_XOR]=0.
// _ CPT_LF_Q_INST_PTR[NQ_PTR]=2.
// _ CPT_LF_Q_INST_PTR[DQ_PTR]=2.
// _ CPT_LF_Q_GRP_PTR[XQ_XOR]=0.
// _ CPT_LF_Q_GRP_PTR[NQ_PTR]=1.
// _ CPT_LF_Q_GRP_PTR[DQ_PTR]=1.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_lf_q_size {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_lf_q_size_s {
    pub size_div40:15: u64,
    pub reserved_15_63:49: u64,
    pub s: },
}

//
// RVU_PF_BAR0 - cpt_af_lf_ctl
//
// This register configures queues. This register should be written only
// when the queue is execution-quiescent (see CPT_LF_INPROG[INFLIGHT]).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union otx2_cptx_af_lf_ctrl {
    pub u: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptx_af_lf_ctrl_s {
    pub pri:1: u64,
    pub reserved_1_8:8: u64,
    pub pf_func_inst:1: u64,
    pub cont_err:1: u64,
    pub reserved_11_15:5: u64,
    pub nixtx_en:1: u64,
    pub ctx_ilen:3: u64,
    pub reserved_17_47:28: u64,
    pub grp:8: u64,
    pub reserved_56_63:8: u64,
    pub s: },
}
