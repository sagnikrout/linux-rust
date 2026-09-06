//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/efa/efa_com.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright 2018-2026 Amazon.com, Inc. or its affiliates. All rights reserved.
//

pub const EFA_MAX_HANDLERS: c_int = 256;
pub const EFA_ADMIN_V1_PROTO_VER: c_int = 0;
pub const EFA_ADMIN_V2_PROTO_VER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_admin_cq {
    pub entries: *mut efa_admin_acq_entry,
    pub dma_addr: dma_addr_t,
    pub /: *mut *mut spinlock_t lock; / Protects ACQ,
    pub validate_checksum: bool,
    pub /: *mut *mut u16 cc; / consumer counter,
    pub phase: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_admin_sq {
    pub buffer: *mut u8,
    pub entry_size: u16,
    pub payload_offset: u16,
    pub max_payload_size: u16,
    pub dma_addr: dma_addr_t,
    pub /: *mut *mut spinlock_t lock; / Protects ASQ,
    pub proto_ver: u8,
    pub db_addr: *mut u32 __iomem,
    pub /: *mut *mut u16 cc; / consumer counter,
    pub /: *mut *mut u16 pc; / producer counter,
    pub phase: u8,
}

// Don't use anything other than atomic64
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_stats_admin {
    pub submitted_cmd: core::sync::atomic::AtomicI64,
    pub completed_cmd: core::sync::atomic::AtomicI64,
    pub cmd_err: core::sync::atomic::AtomicI64,
    pub no_completion: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_admin_queue {
    pub dmadev: *mut c_void,
    pub efa_dev: *mut c_void,
    pub comp_ctx: *mut efa_comp_ctx,
    pub /: *mut *mut u32 completion_timeout; / usecs,
    pub /: *mut *mut u16 poll_interval; / msecs,
    pub depth: u16,
    pub cq: efa_com_admin_cq,
    pub sq: efa_com_admin_sq,
    pub msix_vector_idx: u32,
    pub state: c_ulong,
// Count the number of available admin commands
    pub avail_cmds: semaphore,
    pub stats: efa_com_stats_admin,
    pub /: *mut *mut spinlock_t comp_ctx_lock; / Protects completion context pool,
    pub comp_ctx_pool: *mut u32,
    pub comp_ctx_pool_next: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_aenq {
    pub entries: *mut efa_admin_aenq_entry,
    pub aenq_handlers: *mut efa_aenq_handlers,
    pub dma_addr: dma_addr_t,
    pub /: *mut *mut u32 cc; / consumer counter,
    pub msix_vector_idx: u32,
    pub depth: u16,
    pub phase: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_mmio_read {
    pub read_resp: *mut efa_admin_mmio_req_read_less_resp,
    pub read_resp_dma_addr: dma_addr_t,
    pub seq_num: u16,
    pub /: *mut *mut u16 mmio_read_timeout; / usecs,
// serializes mmio reads
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_dev {
    pub aq: efa_com_admin_queue,
    pub aenq: efa_com_aenq,
    pub reg_bar: *mut u8 __iomem,
    pub dmadev: *mut c_void,
    pub efa_dev: *mut c_void,
    pub supported_features: u32,
    pub dma_addr_bits: u32,
    pub ah_cache: efa_ah_cache,
    pub dev_api_ver: u32,
    pub mmio_read: efa_com_mmio_read,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_eq {
    pub edev: *mut efa_com_dev,
    pub eqes: *mut efa_admin_eqe,
    pub dma_addr: dma_addr_t,
    pub /: *mut *mut u32 cc; / Consumer counter,
    pub eqn: u16,
    pub depth: u16,
    pub phase: u8,
    pub cb: efa_eqe_handler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_eq_params {
    pub dma_addr: dma_addr_t,
    pub event_bitmask: u32,
    pub depth: u16,
    pub entry_size_in_bytes: u8,
    pub msix_vec: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_create_eq_result {
    pub eqn: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_com_destroy_eq_params {
    pub eqn: u16,
}

// Holds aenq handlers. Indexed by AENQ event group
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efa_aenq_handlers {
    pub handlers: [efa_aenq_handler; EFA_MAX_HANDLERS],
    pub unimplemented_handler: efa_aenq_handler,
}

extern "C" {
    pub fn efa_com_set_dma_addr(addr: dma_addr_t, addr_high: *mut u32, addr_low: *mut u32);
}
extern "C" {
    pub fn efa_com_admin_destroy(edev: *mut efa_com_dev);
}
extern "C" {
    pub fn efa_com_eq_destroy(edev: *mut efa_com_dev, eeq: *mut efa_com_eq);
}
extern "C" {
    pub fn efa_com_set_admin_polling_mode(edev: *mut efa_com_dev, polling: bool);
}
extern "C" {
    pub fn efa_com_admin_q_comp_intr_handler(edev: *mut efa_com_dev);
}
extern "C" {
    pub fn efa_com_mmio_reg_read_init(edev: *mut efa_com_dev) -> c_int;
}
extern "C" {
    pub fn efa_com_mmio_reg_read_destroy(edev: *mut efa_com_dev);
}
extern "C" {
    pub fn efa_com_validate_version(edev: *mut efa_com_dev) -> c_int;
}
extern "C" {
    pub fn efa_com_get_dma_width(edev: *mut efa_com_dev) -> c_int;
}
extern "C" {
    pub fn efa_com_aenq_intr_handler(edev: *mut efa_com_dev, data: *mut c_void);
}
