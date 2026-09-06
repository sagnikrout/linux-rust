//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/cavium/cpt/cptvf.h
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
// Copyright (C) 2016 Cavium, Inc.
//

// Default command queue length
pub const CPT_CMD_QLEN: c_int = 2046;
pub const CPT_CMD_QCHUNK_SIZE: c_int = 1023;
// Default command timeout in seconds
pub const CPT_COMMAND_TIMEOUT: c_int = 4;
pub const CPT_TIMER_THOLD: c_uint = 0xFFFF;
pub const CPT_NUM_QS_PER_VF: c_int = 1;
pub const CPT_INST_SIZE: c_int = 64;
pub const CPT_NEXT_CHUNK_PTR_SIZE: c_int = 8;
pub const CPT_VF_MSIX_VECTORS: c_int = 2;

pub const DMA_GATHER_SCATTER: c_int = 1;
pub const FROM_DPTR: c_int = 1;
//
// Enumeration cpt_vf_int_vec_e
//
// CPT VF MSI-X Vector Enumeration
// Enumerates the MSI-X interrupt vectors.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpt_vf_int_vec_e {
    CPT_VF_INT_VEC_E_MISC = 0x00,
    CPT_VF_INT_VEC_E_DONE = 0x01
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct command_chunk {
    pub head: *mut u8,
    pub dma_addr: dma_addr_t,
    pub /: *mut *mut u32 size; / Chunk size, max CPT_INST_CHUNK_MAX_SIZE,
    pub nextchunk: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct command_queue {
    pub /: *mut *mut spinlock_t lock; / command queue lock,
    pub /: *mut *mut u32 idx; / Command queue host write idx,
    pub /: *mut *mut u32 nchunks; / Number of command chunks,
    pub instructions: *mut *mut *mut command_chunk qhead; / Command queue head,,
// are inserted here
//
    pub chead: hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct command_qinfo {
    pub cmd_size: u32,
    pub /: *mut *mut u32 qchunksize; / Command queue chunk size,
    pub queue: [command_queue; CPT_NUM_QS_PER_VF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_entry {
    pub /: *mut *mut u8 busy; / Entry status (free/busy),
    pub /: *mut *mut *mut volatile u64 completion_addr; / Completion address,
    pub post_arg: *mut c_void,
    pub /: *mut *mut *mut *mut void (callback)(int, void ); / Kernel ASYNC request callabck,
    pub /: *mut *mut *mut void callback_arg; / Kernel ASYNC request callabck arg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_queue {
    pub /: *mut *mut *mut pending_entry head; / head of the queue,
    pub /: *mut *mut u32 front; / Process work from here,
    pub /: *mut *mut u32 rear; / Append new work here,
    pub pending_count: core::sync::atomic::AtomicI64,
    pub /: *mut *mut spinlock_t lock; / Queue lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_qinfo {
    pub /: *mut *mut u32 nr_queues; / Number of queues supported,
    pub /: *mut *mut u32 qlen; / Queue length,
    pub queue: [pending_queue; CPT_NUM_QS_PER_VF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_vf {
    pub /: *mut *mut u16 flags; / Flags to hold device status bits,
    pub /: *mut *mut u8 vfid; / Device Index 0...CPT_MAX_VF_NUM,
    pub /: *mut *mut u8 vftype; / VF type of SE_TYPE(1) or AE_TYPE(1),
    pub /: *mut *mut u8 vfgrp; / VF group (0 - 8),
    pub /: *mut *mut u8 node; / Operating node: Bits (46:44) in BAR0 address,
    pub round: *mut *mut u8 priority; / VF priority ring: 1-High proirity,
// robin ring;0-Low priority round robin ring;
//
    pub /: *mut *mut *mut pci_dev pdev; / pci device handle,
    pub /: *mut *mut *mut void __iomem reg_base; / Register start address,
    pub /: *mut *mut *mut void wqe_info; / BH worker info,
// MSI-X
    pub affinity_mask: [cpumask_var_t; CPT_VF_MSIX_VECTORS],
// Command and Pending queues
    pub qsize: u32,
    pub nr_queues: u32,
    pub /: *mut *mut command_qinfo cqinfo; / Command queue information,
    pub /: *mut *mut pending_qinfo pqinfo; / Pending queue information,
// VF-PF mailbox communication
    pub pf_acked: bool,
    pub pf_nacked: bool,
}

extern "C" {
    pub fn cptvf_send_vf_up(cptvf: *mut cpt_vf) -> c_int;
}
extern "C" {
    pub fn cptvf_send_vf_down(cptvf: *mut cpt_vf) -> c_int;
}
extern "C" {
    pub fn cptvf_send_vf_to_grp_msg(cptvf: *mut cpt_vf) -> c_int;
}
extern "C" {
    pub fn cptvf_send_vf_priority_msg(cptvf: *mut cpt_vf) -> c_int;
}
extern "C" {
    pub fn cptvf_send_vq_size_msg(cptvf: *mut cpt_vf) -> c_int;
}
extern "C" {
    pub fn cptvf_check_pf_ready(cptvf: *mut cpt_vf) -> c_int;
}
extern "C" {
    pub fn cptvf_handle_mbox_intr(cptvf: *mut cpt_vf);
}
extern "C" {
    pub fn cvm_crypto_exit();
}
extern "C" {
    pub fn cvm_crypto_init(cptvf: *mut cpt_vf) -> c_int;
}
extern "C" {
    pub fn vq_post_process(cptvf: *mut cpt_vf, qno: u32);
}
extern "C" {
    pub fn cptvf_write_vq_doorbell(cptvf: *mut cpt_vf, val: u32);
}
