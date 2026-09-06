//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic94xx/aic94xx_hwi.h
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
// Aic94xx SAS/SATA driver hardware interface header file.
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

// Define ASD_MAX_PHYS to the maximum phys ever. Currently 8.
pub const ASD_MAX_PHYS: c_int = 8;
pub const ASD_PCBA_SN_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ha_addrspace {
    pub addr: *mut void __iomem,
    pub /: *mut *mut unsigned long start; / pci resource start,
    pub /: *mut *mut unsigned long len; / pci resource len,
    pub /: *mut *mut unsigned long flags; / pci resource flags,
// addresses internal to the host adapter
    pub /: *mut *mut u32 swa_base; / mmspace 1 (MBAR1) uses this only,
    pub swb_base: u32,
    pub swc_base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_struct {
    pub present: c_int,
    pub maj: u8,
    pub min: u8,
    pub bld: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unit_element_struct {
    pub num: u16,
    pub size: u16,
    pub area: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_struct {
    pub bar: u32,
    pub present: c_int,
    pub wide: c_int,
    pub manuf: u8,
    pub dev_id: u8,
    pub sec_prot: u8,
    pub method: u8,
    pub dir_offs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_phy_desc {
// From CTRL-A settings, then set to what is appropriate
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub max_sas_lrate: u8,
    pub min_sas_lrate: u8,
    pub max_sata_lrate: u8,
    pub min_sata_lrate: u8,
    pub flags: u8,
pub const ASD_CRC_DIS: c_int = 1;
pub const ASD_SATA_SPINUP_HOLD: c_int = 2;
    pub /: *mut *mut u8 phy_control_0; / mode 5 reg 0x160,
    pub /: *mut *mut u8 phy_control_1; / mode 5 reg 0x161,
    pub /: *mut *mut u8 phy_control_2; / mode 5 reg 0x162,
    pub /: *mut *mut u8 phy_control_3; / mode 5 reg 0x163,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_dma_tok {
    pub vaddr: *mut c_void,
    pub dma_handle: dma_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_profile {
    pub bios: bios_struct,
    pub ue: unit_element_struct,
    pub flash: flash_struct,
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub pcba_sn: [c_char; ASD_PCBA_SN_SIZE+1],
    pub /: *mut *mut u8 enabled_phys; / mask of enabled phys,
    pub phy_desc: [asd_phy_desc; ASD_MAX_PHYS],
    pub /: *mut *mut u32 max_scbs; / absolute sequencer scb queue size,
    pub scb_ext: *mut asd_dma_tok,
    pub max_ddbs: u32,
    pub ddb_ext: *mut asd_dma_tok,
    pub ddb_lock: spinlock_t,
    pub ddb_bitmap: *mut c_void,
    pub /: *mut *mut int num_phys; / ENABLEABLE,
    pub /: *mut *mut int max_phys; / REPORTED + ENABLEABLE,
    pub /: *mut *mut unsigned addr_range; / max # of addrs; max # of possible ports,
    pub port_name_base: unsigned,
    pub dev_name_base: unsigned,
    pub sata_name_base: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ascb {
    pub list: list_head,
    pub ha: *mut asd_ha_struct,
    pub /: *mut *mut *mut scb scb; / equals dma_scb->vaddr,
    pub dma_scb: asd_dma_tok,
    pub sg_arr: *mut asd_dma_tok,
    pub ): *mut *mut *mut void (tasklet_complete)(struct asd_ascb , struct done_list_struct,
    pub uldd_timer:1: u8,
// internally generated command
    pub timer: timer_list,
    pub completion: *mut completion,
    pub tag_valid:1: u8,
    pub /: *mut *mut __be16 tag; / error recovery only,
// If this is an Empty SCB, index of first edb in seq->edb_arr.
    pub edb_index: c_int,
// Used by the timer timeout function.
    pub tc_index: c_int,
    pub uldd_task: *mut c_void,
}

pub const ASD_DL_SIZE_BITS: c_uint = 0x8;

pub const ASD_DEF_DL_TOGGLE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_seq_data {
    pub pend_q_lock: spinlock_t,
    pub scbpro: u16,
    pub pending: c_int,
    pub pend_q: list_head,
    pub /: *mut *mut int can_queue; / per adapter,
    pub /: *mut *mut asd_dma_tok next_scb; / next scb to be delivered to CSEQ,
    pub tc_index_lock: spinlock_t,
    pub tc_index_array: *mut c_void,
    pub tc_index_bitmap: *mut c_void,
    pub tc_index_bitmap_bits: c_int,
    pub dl_tasklet: tasklet_struct,
    pub /: *mut *mut *mut done_list_dl; / array of done list entries, equals,
    pub /: *mut *mut *mut asd_dma_tok actual_dl; / actual_dl->vaddr,
    pub dl_toggle: c_int,
    pub dl_next: c_int,
    pub num_edbs: c_int,
    pub edb_arr: *mut asd_dma_tok,
    pub num_escbs: c_int,
    pub /: *mut *mut *mut *mut asd_ascb escb_arr; / array of pointers to escbs,
}

// This is an internal port structure. These are used to get accurate
// phy_mask for updating DDB 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_port {
    pub sas_addr: [u8; SAS_ADDR_SIZE],
    pub attached_sas_addr: [u8; SAS_ADDR_SIZE],
    pub phy_mask: u32,
    pub num_phys: c_int,
}

// This is the Host Adapter structure.  It describes the hardware
// SAS adapter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asd_ha_struct {
    pub pcidev: *mut pci_dev,
    pub name: *const c_char,
    pub sas_ha: sas_ha_struct,
    pub revision_id: u8,
    pub iospace: c_int,
    pub iolock: spinlock_t,
    pub io_handle: [asd_ha_addrspace; 2],
    pub hw_prof: hw_profile,
    pub phys: [asd_phy; ASD_MAX_PHYS],
    pub asd_ports_lock: spinlock_t,
    pub asd_ports: [asd_port; ASD_MAX_PHYS],
    pub ports: [asd_sas_port; ASD_MAX_PHYS],
    pub scb_pool: *mut dma_pool,
    pub /: *mut *mut asd_seq_data seq; / sequencer related,
    pub bios_status: u32,
    pub bios_image: *const firmware,
}

// ---------- Common macros ----------

// For each bit set in __lseq_mask, set __lseq to equal the bit
// position of the set bit and execute the statement following.
// __mc is the temporary mask, used as a mask "counter".
//

// ---------- DMA allocs ----------
extern "C" {
    pub fn kmem_cache_alloc(_arg: asd_dma_token_cache, _arg: flags) -> return;
}
// Must be called with the tc_index_lock held!
//
// Must be called with the tc_index_lock held!
//
// Must be called with the tc_index_lock held!
//
// asd_ascb_free -- free a single aSCB after is has completed
// @ascb: pointer to the aSCB of interest
//
// This frees an aSCB after it has been executed/completed by
// the sequencer.
//
// asd_ascb_list_free -- free a list of ascbs
// @ascb_list: a list of ascbs
//
// This function will free a list of ascbs allocated by asd_ascb_alloc_list.
// It is used when say the scb queueing function returned QUEUE_FULL,
// and we do not need the ascbs any more.
//
// ---------- Function declarations ----------
extern "C" {
    pub fn asd_init_hw(asd_ha: *mut asd_ha_struct) -> c_int;
}
extern "C" {
    pub fn asd_hw_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
// asd_ha, int *num,
extern "C" {
    pub fn asd_init_post_escbs(asd_ha: *mut asd_ha_struct) -> c_int;
}
extern "C" {
    pub fn asd_build_control_phy(ascb: *mut asd_ascb, phy_id: c_int, subfunc: u8);
}
extern "C" {
    pub fn asd_control_led(asd_ha: *mut asd_ha_struct, phy_id: c_int, op: c_int);
}
extern "C" {
    pub fn asd_turn_led(asd_ha: *mut asd_ha_struct, phy_id: c_int, op: c_int);
}
extern "C" {
    pub fn asd_enable_phys(asd_ha: *mut asd_ha_struct, phy_mask: u8) -> c_int;
}
extern "C" {
    pub fn asd_ascb_timedout(t: *mut timer_list);
}
extern "C" {
    pub fn asd_chip_hardrst(asd_ha: *mut asd_ha_struct) -> c_int;
}
