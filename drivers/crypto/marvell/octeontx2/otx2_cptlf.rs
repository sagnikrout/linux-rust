//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx2/otx2_cptlf.h
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

//
// CPT instruction and pending queues user requested length in CPT_INST_S msgs
//
pub const OTX2_CPT_USER_REQUESTED_QLEN_MSGS: c_int = 8200;
//
// CPT instruction queue size passed to HW is in units of 40*CPT_INST_S
// messages.
//

//
// CPT instruction and pending queues length in CPT_INST_S messages
//

//
// LDWB is getting incorrectly used when IQB_LDWB = 1 and CPT instruction
// queue has less than 320 free entries. So, increase HW instruction queue
// size by 320 and give 320 entries less for SW/NIX RX as a workaround.
//

// CPT instruction queue length in bytes

// CPT instruction group queue length in bytes

// CPT FC length in bytes
pub const OTX2_CPT_Q_FC_LEN: c_int = 128;
// CPT instruction queue alignment
pub const OTX2_CPT_INST_Q_ALIGNMENT: c_int = 128;
// Mask which selects all engine groups
pub const OTX2_CPT_ALL_ENG_GRPS_MASK: c_uint = 0xFF;
// Maximum LFs supported in OcteonTX2 for CPT
pub const OTX2_CPT_MAX_LFS_NUM: c_int = 64;
// Queue priority
pub const OTX2_CPT_QUEUE_HI_PRIO: c_uint = 0x1;
pub const OTX2_CPT_QUEUE_LOW_PRIO: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum otx2_cptlf_state {
    OTX2_CPTLF_IN_RESET,
    OTX2_CPTLF_STARTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cpt_inst_queue {
    pub vaddr: *mut u8,
    pub real_vaddr: *mut u8,
    pub dma_addr: dma_addr_t,
    pub real_dma_addr: dma_addr_t,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptlf_wqe {
    pub work: tasklet_struct,
    pub lfs: *mut otx2_cptlfs_info,
    pub lf_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptlf_info {
    pub /: *mut *mut *mut otx2_cptlfs_info lfs; / Ptr to cptlfs_info struct,
    pub /: *mut *mut *mut void __iomem lmtline; / Address of LMTLINE,
    pub /: *mut *mut *mut void __iomem ioreg; / LMTLINE send register,
    pub /: *mut *mut int msix_offset; / MSI-X interrupts offset,
    pub /: *mut *mut cpumask_var_t affinity_mask; / IRQs affinity mask,
    pub /: *mut *mut u8 irq_name[OTX2_CPT_LF_MSIX_VECTORS][32];/ Interrupts name,
    pub /: *mut *mut u8 is_irq_reg[OTX2_CPT_LF_MSIX_VECTORS]; / Is interrupt registered,
    pub /: *mut *mut u8 slot; / Slot number of this LF,
    pub /: *mut *mut otx2_cpt_inst_queue iqueue;/ Instruction queue,
    pub /: *mut *mut otx2_cpt_pending_queue pqueue; / Pending queue,
    pub /: *mut *mut *mut otx2_cptlf_wqe wqe; / Tasklet work info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_hw_ops {
    pub lf): *mut otx2_cptlf_info,
    pub result): *mut *mut u8 (cpt_get_compcode)(union otx2_cpt_res_s,
    pub result): *mut *mut u8 (cpt_get_uc_compcode)(union otx2_cpt_res_s,
    pub gfp): gfp_t,
}

pub const LMTLINE_SIZE: c_int = 128;
pub const LMTLINE_ALIGN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_lmt_info {
    pub base: *mut c_void,
    pub iova: dma_addr_t,
    pub size: u32,
    pub align: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx2_cptlfs_info {
// Registers start address of VF/PF LFs are attached to
    pub reg_base: *mut void __iomem,
    pub lmt_info: otx2_lmt_info,
    pub /: *mut *mut *mut pci_dev pdev; / Device LFs are attached to,
    pub lf: [otx2_cptlf_info; OTX2_CPT_MAX_LFS_NUM],
    pub mbox: *mut otx2_mbox,
    pub ops: *mut cpt_hw_ops,
    pub /: *mut *mut u8 are_lfs_attached; / Whether CPT LFs are attached,
    pub /: *mut *mut u8 lfs_num; / Number of CPT LFs,
    pub /: *mut *mut u8 kcrypto_se_eng_grp_num; / Crypto symmetric engine group number,
    pub /: *mut *mut u8 kcrypto_ae_eng_grp_num; / Crypto asymmetric engine group number,
    pub /: *mut *mut u8 kvf_limits; / Kernel crypto limits,
    pub /: *mut *mut atomic_t state; / LF's state. started/reset,
    pub /: *mut *mut int blkaddr; / CPT blkaddr: BLKADDR_CPT0/BLKADDR_CPT1,
    pub /: *mut *mut int global_slot; / Global slot across the blocks,
    pub ctx_ilen: u8,
    pub ctx_ilen_ovrd: u8,
}

// Align pointers

// Disable instructions enqueuing
// Wait for CPT queue to become execution-quiescent
// Wait for 2 us to flush all queue writes to memory
// Set iqueue's enqueuing
// Set iqueue's execution
// Enable flush on FLR for Errata
//
// On OcteonTX2 platform the parameter insts_num is used as a count of
// instructions to be enqueued. The valid values for insts_num are:
// 1 - 1 CPT instruction will be enqueued during LMTST operation
// 2 - 2 CPT instructions will be enqueued during LMTST operation
//
// Make sure memory areas pointed in CPT_INST_S
// are flushed before the instruction is sent to CPT
//
// Copy CPT command to LMTLINE
//
// LDEOR initiates atomic transfer to I/O device
// The following will cause the LMTST to fail (the LDEOR
// returns zero):
// - No stores have been performed to the LMTLINE since it was
// last invalidated.
// - The bytes which have been stored to LMTLINE since it was
// last invalidated form a pattern that is non-contiguous, does
// not start at byte 0, or does not end on a 8-byte boundary.
// (i.e.comprises a formation of other than 1–16 8-byte
// words.)
//
// These rules are designed such that an operating system
// context switch or hypervisor guest switch need have no
// knowledge of the LMTST operations; the switch code does not
// need to store to LMTCANCEL. Also note as LMTLINE data cannot
// be read, there is no information leakage between processes.
//
extern "C" {
    pub fn otx2_cptlf_shutdown(lfs: *mut otx2_cptlfs_info);
}
extern "C" {
    pub fn otx2_cptlf_register_misc_interrupts(lfs: *mut otx2_cptlfs_info) -> c_int;
}
extern "C" {
    pub fn otx2_cptlf_register_done_interrupts(lfs: *mut otx2_cptlfs_info) -> c_int;
}
extern "C" {
    pub fn otx2_cptlf_unregister_misc_interrupts(lfs: *mut otx2_cptlfs_info);
}
extern "C" {
    pub fn otx2_cptlf_unregister_done_interrupts(lfs: *mut otx2_cptlfs_info);
}
extern "C" {
    pub fn otx2_cptlf_free_irqs_affinity(lfs: *mut otx2_cptlfs_info);
}
extern "C" {
    pub fn otx2_cptlf_set_irqs_affinity(lfs: *mut otx2_cptlfs_info) -> c_int;
}
