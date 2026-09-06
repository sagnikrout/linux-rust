//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/marvell/octeontx/otx_cptvf.h
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
// Marvell OcteonTX CPT driver
//
// Copyright (C) 2019 Marvell International Ltd.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

// Flags to indicate the features supported

// Default command queue length

pub const OTX_CPT_CMD_QCHUNK_SIZE: c_int = 1023;
pub const OTX_CPT_NUM_QS_PER_VF: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_cmd_chunk {
    pub head: *mut u8,
    pub dma_addr: dma_addr_t,
    pub /: *mut *mut u32 size; / Chunk size, max OTX_CPT_INST_CHUNK_MAX_SIZE,
    pub nextchunk: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_cmd_queue {
    pub /: *mut *mut u32 idx; / Command queue host write idx,
    pub /: *mut *mut u32 num_chunks; / Number of command chunks,
    pub qhead;/*: *mut otx_cpt_cmd_chunk,
// Command queue head, instructions
// are inserted here
//
    pub base: *mut otx_cpt_cmd_chunk,
    pub chead: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_cmd_qinfo {
    pub /: *mut *mut u32 qchunksize; / Command queue chunk size,
    pub queue: [otx_cpt_cmd_queue; OTX_CPT_NUM_QS_PER_VF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cpt_pending_qinfo {
    pub /: *mut *mut u32 num_queues; / Number of queues supported,
    pub queue: [otx_cpt_pending_queue; OTX_CPT_NUM_QS_PER_VF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cptvf_wqe {
    pub twork: tasklet_struct,
    pub cptvf: *mut otx_cptvf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cptvf_wqe_info {
    pub vq_wqe: [otx_cptvf_wqe; OTX_CPT_NUM_QS_PER_VF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otx_cptvf {
    pub /: *mut *mut u16 flags; / Flags to hold device status bits,
    pub /: *mut *mut u8 vfid; / Device Index 0...OTX_CPT_MAX_VF_NUM,
    pub /: *mut *mut u8 num_vfs; / Number of enabled VFs,
    pub /: *mut *mut u8 vftype; / VF type of SE_TYPE(2) or AE_TYPE(1),
    pub /: *mut *mut u8 vfgrp; / VF group (0 - 8),
    pub /: *mut *mut u8 node; / Operating node: Bits (46:44) in BAR0 address,
    pub /*: *mut u8 priority;,
// VF priority ring: 1-High proirity round
// robin ring;0-Low priority round robin ring;
//
    pub /: *mut *mut *mut pci_dev pdev; / Pci device handle,
    pub /: *mut *mut *mut void __iomem reg_base; / Register start address,
    pub /: *mut *mut *mut void wqe_info; / BH worker info,
// MSI-X
    pub affinity_mask: [cpumask_var_t; OTX_CPT_VF_MSIX_VECTORS],
// Command and Pending queues
    pub qsize: u32,
    pub num_queues: u32,
    pub /: *mut *mut otx_cpt_cmd_qinfo cqinfo; / Command queue information,
    pub /: *mut *mut otx_cpt_pending_qinfo pqinfo; / Pending queue information,
// VF-PF mailbox communication
    pub pf_acked: bool,
    pub pf_nacked: bool,
}

extern "C" {
    pub fn otx_cptvf_send_vf_up(cptvf: *mut otx_cptvf) -> c_int;
}
extern "C" {
    pub fn otx_cptvf_send_vf_down(cptvf: *mut otx_cptvf) -> c_int;
}
extern "C" {
    pub fn otx_cptvf_send_vf_to_grp_msg(cptvf: *mut otx_cptvf, group: c_int) -> c_int;
}
extern "C" {
    pub fn otx_cptvf_send_vf_priority_msg(cptvf: *mut otx_cptvf) -> c_int;
}
extern "C" {
    pub fn otx_cptvf_send_vq_size_msg(cptvf: *mut otx_cptvf) -> c_int;
}
extern "C" {
    pub fn otx_cptvf_check_pf_ready(cptvf: *mut otx_cptvf) -> c_int;
}
extern "C" {
    pub fn otx_cptvf_handle_mbox_intr(cptvf: *mut otx_cptvf);
}
extern "C" {
    pub fn otx_cptvf_write_vq_doorbell(cptvf: *mut otx_cptvf, val: u32);
}
