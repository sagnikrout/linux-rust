//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/fsl-dpaa2-qdma/dpaa2-qdma.h
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
// Copyright 2019 NXP
pub const DPAA2_QDMA_STORE_SIZE: c_int = 16;
pub const NUM_CH: c_int = 8;
pub const DPAA2_QDMA_DEFAULT_PRIORITY: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_qdma_sd_d {
    pub rsv:32: u32,
    pub /: *mut *mut u32 ssd:12; / source stride distance,
    pub /: *mut *mut u32 sss:12; / source stride size,
    pub rsv1:8: u32,
    pub sdf: },
    pub /: *mut *mut u32 dsd:12; / Destination stride distance,
    pub /: *mut *mut u32 dss:12; / Destination stride size,
    pub rsv2:8: u32,
    pub ddf: },
    pub df: },
    pub /: *mut *mut u32 rbpcmd; / Route-by-port command,
    pub cmd: u32,
    pub __attribute__((__packed__)): },
// Source descriptor command read transaction type for RBP=0:
// coherent copy of cacheable memory

// Destination descriptor command write transaction type for RBP=0:
// coherent copy of cacheable memory

// Flow Context: 49bit physical address

// Description of Frame list table structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_qdma_chan {
    pub qdma: *mut dpaa2_qdma_engine,
    pub vchan: virt_dma_chan,
    pub vdesc: virt_dma_desc,
    pub status: dma_status,
    pub fqid: u32,
// spinlock used by dpaa2 qdma driver
    pub queue_lock: spinlock_t,
    pub fd_pool: *mut dma_pool,
    pub fl_pool: *mut dma_pool,
    pub sdd_pool: *mut dma_pool,
    pub comp_used: list_head,
    pub comp_free: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_qdma_comp {
    pub fd_bus_addr: dma_addr_t,
    pub fl_bus_addr: dma_addr_t,
    pub desc_bus_addr: dma_addr_t,
    pub fd_virt_addr: *mut dpaa2_fd,
    pub fl_virt_addr: *mut dpaa2_fl_entry,
    pub desc_virt_addr: *mut dpaa2_qdma_sd_d,
    pub qchan: *mut dpaa2_qdma_chan,
    pub vdesc: virt_dma_desc,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_qdma_engine {
    pub dma_dev: dma_device,
    pub n_chans: u32,
    pub chans: [dpaa2_qdma_chan; NUM_CH],
    pub qdma_wrtype_fixup: c_int,
    pub desc_allocated: c_int,
    pub priv: *mut dpaa2_qdma_priv,
}

//
// dpaa2_qdma_priv - driver private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_qdma_priv {
    pub dpqdma_id: c_int,
    pub iommu_domain: *mut iommu_domain,
    pub dpdmai_attr: dpdmai_attr,
    pub dev: *mut device,
    pub mc_io: *mut fsl_mc_io,
    pub dpdmai_dev: *mut fsl_mc_device,
    pub num_pairs: u8,
    pub dpaa2_qdma: *mut dpaa2_qdma_engine,
    pub ppriv: *mut dpaa2_qdma_priv_per_prio,
    pub rx_queue_attr: [dpdmai_rx_queue_attr; DPDMAI_MAX_QUEUE_NUM],
    pub tx_queue_attr: [dpdmai_tx_queue_attr; DPDMAI_MAX_QUEUE_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpaa2_qdma_priv_per_prio {
    pub req_fqid: c_int,
    pub rsp_fqid: c_int,
    pub prio: c_int,
    pub store: *mut dpaa2_io_store,
    pub nctx: dpaa2_io_notification_ctx,
    pub priv: *mut dpaa2_qdma_priv,
}

// FD pool size: one FD + 3 Frame list + 2 source/destination descriptor

extern "C" {
    pub fn dpaa2_dpdmai_free_channels(dpaa2_qdma: *mut dpaa2_qdma_engine) -> static void;
}
