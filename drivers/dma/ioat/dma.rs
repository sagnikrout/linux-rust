//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dma/ioat/dma.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright(c) 2004 - 2009 Intel Corporation. All rights reserved.
//

extern "C" {
    pub fn system_has_dca_enabled(pdev: *mut pci_dev) -> c_int;
}

// ioat hardware assumes at least two sources for raid operations

//
// workaround for IOAT ver.3.0 null descriptor issue
// (channel returns error when size is 0)
//
pub const NULL_DESC_BUFFER_SIZE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ioat_irq_mode {
    IOAT_NOIRQ = 0,
    IOAT_MSIX,
    IOAT_MSI,
    IOAT_INTX
}

//
// struct ioatdma_device - internal representation of a IOAT device
// @pdev: PCI-Express device
// @reg_base: MMIO register space base address
// @completion_pool: DMA buffers for completion ops
// @sed_hw_pool: DMA super descriptor pools
// @dma_dev: embedded struct dma_device
// @version: version of ioatdma device
// @msix_entries: irq handlers
// @idx: per channel data
// @dca: direct cache access context
// @irq_mode: interrupt mode (INTX, MSI, MSIX)
// @cap: read DMA capabilities register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioatdma_device {
    pub pdev: *mut pci_dev,
    pub reg_base: *mut void __iomem,
    pub completion_pool: *mut dma_pool,
pub const MAX_SED_POOLS: c_int = 5;
    pub sed_hw_pool: [*mut dma_pool; MAX_SED_POOLS],
    pub dma_dev: dma_device,
    pub version: u8,
pub const IOAT_MAX_CHANS: c_int = 4;
    pub msix_entries: [msix_entry; IOAT_MAX_CHANS],
    pub idx: [*mut ioatdma_chan; IOAT_MAX_CHANS],
    pub dca: *mut dca_provider,
    pub irq_mode: ioat_irq_mode,
    pub cap: u32,
    pub chancnt: c_int,
// shadow version for CB3.3 chan reset errata workaround
    pub msixtba0: u64,
    pub msixdata0: u64,
    pub msixpba: u32,
}

pub const IOAT_MAX_ORDER: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_descs {
    pub virt: *mut c_void,
    pub hw: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioatdma_chan {
    pub dma_chan: dma_chan,
    pub reg_base: *mut void __iomem,
    pub last_completion: dma_addr_t,
    pub cleanup_lock: spinlock_t,
    pub state: c_ulong,
pub const IOAT_CHAN_DOWN: c_int = 0;
pub const IOAT_COMPLETION_ACK: c_int = 1;
pub const IOAT_RESET_PENDING: c_int = 2;
pub const IOAT_KOBJ_INIT_FAIL: c_int = 3;
pub const IOAT_RUN: c_int = 5;
pub const IOAT_CHAN_ACTIVE: c_int = 6;
    pub timer: timer_list,

    pub ioat_dma: *mut ioatdma_device,
    pub completion_dma: dma_addr_t,
    pub completion: *mut u64,
    pub cleanup_task: tasklet_struct,
    pub kobj: kobject,
// ioat v2 / v3 channel attributes
// @xfercap_log; log2 of channel max transfer length (for fast division)
// @head: allocated index
// @issued: hardware notification point
// @tail: cleanup index
// @dmacount: identical to 'head' except for occasionally resetting to zero
// @alloc_order: log2 of the number of allocated descriptors
// @produce: number of descriptors to produce at submit time
// @ring: software ring buffer implementation of hardware ring
// @prep_lock: serializes descriptor preparation (producers)
//
    pub xfercap_log: usize,
    pub head: u16,
    pub issued: u16,
    pub tail: u16,
    pub dmacount: u16,
    pub alloc_order: u16,
    pub produce: u16,
    pub ring: *mut ioat_ring_ent,
    pub prep_lock: spinlock_t,
    pub IOAT_DESCS_PER_CHUNK]: ioat_descs descs[IOAT_MAX_DESCS /,
    pub desc_chunks: c_int,
    pub intr_coalesce: c_int,
    pub prev_intr_coalesce: c_int,
}

//
// struct ioat_sed_ent - wrapper around super extended hardware descriptor
// @hw: hardware SED
// @dma: dma address for the SED
// @parent: point to the dma descriptor that's the parent
// @hw_pool: descriptor pool index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_sed_ent {
    pub hw: *mut ioat_sed_raw_descriptor,
    pub dma: dma_addr_t,
    pub parent: *mut ioat_ring_ent,
    pub hw_pool: c_uint,
}

//
// struct ioat_ring_ent - wrapper around hardware descriptor
// @hw: hardware DMA descriptor (for memcpy)
// @xor: hardware xor descriptor
// @xor_ex: hardware xor extension descriptor
// @pq: hardware pq descriptor
// @pq_ex: hardware pq extension descriptor
// @pqu: hardware pq update descriptor
// @raw: hardware raw (un-typed) descriptor
// @txd: the generic software descriptor for all engines
// @len: total transaction length for unmap
// @result: asynchronous result of validate operations
// @id: identifier for debug
// @sed: pointer to super extended descriptor sw desc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioat_ring_ent {
    pub hw: *mut ioat_dma_descriptor,
    pub xor: *mut ioat_xor_descriptor,
    pub xor_ex: *mut ioat_xor_ext_descriptor,
    pub pq: *mut ioat_pq_descriptor,
    pub pq_ex: *mut ioat_pq_ext_descriptor,
    pub pqu: *mut ioat_pq_update_descriptor,
    pub raw: *mut ioat_raw_descriptor,
}

extern "C" {
    pub fn container_of(_arg: c, ioatdma_chan: struct, _arg: dma_chan) -> return;
}
// wrapper around hardware descriptor format + additional software fields

extern "C" {
    pub fn readq(IOAT_CHANSTS_OFFSET: ioat_chan->reg_base +) -> return;
}
extern "C" {
    pub fn readl(IOAT_CHANERR_OFFSET: ioat_chan->reg_base +) -> return;
}
// channel was fatally programmed
// count of descriptors in flight with the engine
// count of descriptors pending submission to hardware
extern "C" {
    pub fn ioat_ring_size(ioat_ring_active(ioat_chan: ioat_chan) -) -> return;
}
// IOAT Prep functions
// IOAT Operation functions
extern "C" {
    pub fn ioat_dma_do_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ioat_dma_do_interrupt_msix(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn ioat_start_null_desc(ioat_chan: *mut ioatdma_chan);
}
extern "C" {
    pub fn ioat_free_ring_ent(desc: *mut ioat_ring_ent, chan: *mut dma_chan);
}
extern "C" {
    pub fn ioat_reset_hw(ioat_chan: *mut ioatdma_chan) -> c_int;
}
extern "C" {
    pub fn ioat_cleanup_event(t: *mut tasklet_struct);
}
extern "C" {
    pub fn ioat_timer_event(t: *mut timer_list);
}
extern "C" {
    pub fn ioat_check_space_lock(ioat_chan: *mut ioatdma_chan, num_descs: c_int) -> c_int;
}
extern "C" {
    pub fn ioat_issue_pending(chan: *mut dma_chan);
}
// IOAT Init functions
extern "C" {
    pub fn is_bwd_ioat(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn ioat_kobject_add(ioat_dma: *mut ioatdma_device, type: *const kobj_type);
}
extern "C" {
    pub fn ioat_kobject_del(ioat_dma: *mut ioatdma_device);
}
extern "C" {
    pub fn ioat_dma_setup_interrupts(ioat_dma: *mut ioatdma_device) -> c_int;
}
extern "C" {
    pub fn ioat_stop(ioat_chan: *mut ioatdma_chan);
}
