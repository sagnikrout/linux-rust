//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/qcom/hidma.h
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
// Qualcomm Technologies HIDMA data structures
//
// Copyright (c) 2014-2016, The Linux Foundation. All rights reserved.
//

pub const HIDMA_TRE_CFG_IDX: c_int = 0;
pub const HIDMA_TRE_LEN_IDX: c_int = 1;
pub const HIDMA_TRE_SRC_LOW_IDX: c_int = 2;
pub const HIDMA_TRE_SRC_HI_IDX: c_int = 3;
pub const HIDMA_TRE_DEST_LOW_IDX: c_int = 4;
pub const HIDMA_TRE_DEST_HI_IDX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tre_type {
    HIDMA_TRE_MEMCPY = 3,
    HIDMA_TRE_MEMSET = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidma_tre {
    pub /: *mut *mut atomic_t allocated; / if this channel is allocated,
    pub /: *mut *mut bool queued; / flag whether this is pending,
    pub /: *mut *mut u16 status; / status,
    pub /: *mut *mut u32 idx; / index of the tre,
    pub /: *mut *mut u32 dma_sig; / signature of the tre,
    pub /: *const *const *const char dev_name; / name of the device,
    pub /: *mut *mut *mut *mut void (callback)(void data); / requester callback,
    pub channel*/: *mut *mut *mut void data; / Data associated with this,
    pub /: *mut *mut *mut hidma_lldev lldev; / lldma device pointer,
    pub /: *mut *mut u32 tre_local[HIDMA_TRE_SIZE / sizeof(u32) + 1]; / TRE local copy,
    pub written*/: *mut *mut u32 tre_index; / the offset where this was,
    pub /: *mut *mut u32 int_flags; / interrupt flags,
    pub /: *mut *mut u8 err_info; / error record in this transfer,
    pub /: *mut *mut u8 err_code; / completion code,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidma_lldev {
    pub /: *mut *mut bool msi_support; / flag indicating MSI support,
    pub /: *mut *mut bool initialized; / initialized flag,
    pub /: *mut *mut u8 trch_state; / trch_state of the device,
    pub /: *mut *mut u8 evch_state; / evch_state of the device,
    pub /: *mut *mut u8 chidx; / channel index in the core,
    pub /: *mut *mut u32 nr_tres; / max number of configs,
    pub /: *mut *mut spinlock_t lock; / reentrancy,
    pub /: *mut *mut *mut hidma_tre trepool; / trepool of user configs,
    pub /: *mut *mut *mut device dev; / device,
    pub /: *mut *mut *mut void __iomem trca; / Transfer Channel address,
    pub /: *mut *mut *mut void __iomem evca; / Event Channel address,
// pending_tre_list;	/* Pointers to pending TREs
    pub /: *mut *mut atomic_t pending_tre_count; / Number of TREs pending,
    pub /: *mut *mut *mut void tre_ring; / TRE ring,
    pub /: *mut *mut dma_addr_t tre_dma; / TRE ring to be shared with HW,
    pub /: *mut *mut u32 tre_ring_size; / Byte size of the ring,
    pub /: *mut *mut u32 tre_processed_off; / last processed TRE,
    pub /: *mut *mut *mut void evre_ring; / EVRE ring,
    pub /: *mut *mut dma_addr_t evre_dma; / EVRE ring to be shared with HW,
    pub /: *mut *mut u32 evre_ring_size; / Byte size of the ring,
    pub /: *mut *mut u32 evre_processed_off; / last processed EVRE,
    pub /: *mut *mut u32 tre_write_offset; / TRE write location,
    pub /: *mut *mut tasklet_task; / task delivering notifications,
    pub /: *mut *mut *mut hidma_tre ); / pending TREs FIFO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidma_desc {
    pub desc: dma_async_tx_descriptor,
// link list node for this channel
    pub node: list_head,
    pub tre_ch: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidma_chan {
    pub paused: bool,
    pub allocated: bool,
    pub dbg_name: [c_char; 16],
    pub dma_sig: u32,
    pub last_success: dma_cookie_t,
//
// active descriptor on this channel
// It is used by the DMA complete notification to
// locate the descriptor that initiated the transfer.
//
    pub dmadev: *mut hidma_dev,
    pub running: *mut hidma_desc,
    pub chan: dma_chan,
    pub free: list_head,
    pub prepared: list_head,
    pub queued: list_head,
    pub active: list_head,
    pub completed: list_head,
// Lock for this structure
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidma_dev {
    pub irq: c_int,
    pub chidx: c_int,
    pub nr_descriptors: u32,
    pub msi_virqbase: c_int,
    pub lldev: *mut hidma_lldev,
    pub dev_trca: *mut void __iomem,
    pub trca_resource: *mut resource,
    pub dev_evca: *mut void __iomem,
    pub evca_resource: *mut resource,
// used to protect the pending channel list
    pub lock: spinlock_t,
    pub ddev: dma_device,
    pub debugfs: *mut dentry,
// sysfs entry for the channel id
    pub chid_attrs: *mut device_attribute,
// Task delivering issue_pending
    pub task: tasklet_struct,
}

extern "C" {
    pub fn hidma_ll_free(llhndl: *mut hidma_lldev, tre_ch: u32);
}
extern "C" {
    pub fn hidma_ll_status(llhndl: *mut hidma_lldev, tre_ch: u32) -> dma_status;
}
extern "C" {
    pub fn hidma_ll_isenabled(llhndl: *mut hidma_lldev) -> bool;
}
extern "C" {
    pub fn hidma_ll_queue_request(llhndl: *mut hidma_lldev, tre_ch: u32);
}
extern "C" {
    pub fn hidma_ll_start(llhndl: *mut hidma_lldev);
}
extern "C" {
    pub fn hidma_ll_disable(lldev: *mut hidma_lldev) -> c_int;
}
extern "C" {
    pub fn hidma_ll_enable(llhndl: *mut hidma_lldev) -> c_int;
}
extern "C" {
    pub fn hidma_ll_setup_irq(lldev: *mut hidma_lldev, msi: bool);
}
extern "C" {
    pub fn hidma_ll_setup(lldev: *mut hidma_lldev) -> c_int;
}
extern "C" {
    pub fn hidma_ll_uninit(llhndl: *mut hidma_lldev) -> c_int;
}
extern "C" {
    pub fn hidma_ll_inthandler(irq: c_int, arg: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn hidma_ll_inthandler_msi(irq: c_int, arg: *mut c_void, cause: c_int) -> irqreturn_t;
}
extern "C" {
    pub fn hidma_debug_init(dmadev: *mut hidma_dev);
}
extern "C" {
    pub fn hidma_debug_uninit(dmadev: *mut hidma_dev);
}
