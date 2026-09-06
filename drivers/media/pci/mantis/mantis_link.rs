//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/mantis/mantis_link.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mantis_sbuf_status {
    MANTIS_SBUF_DATA_AVAIL		= 1,
    MANTIS_SBUF_DATA_EMPTY		= 2,
    MANTIS_SBUF_DATA_OVFLW		= 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mantis_slot {
    pub timeout: u32,
    pub slave_cfg: u32,
    pub bar: u32,
}

// Physical layer
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mantis_slot_state {
    MODULE_INSERTED			= 3,
    MODULE_XTRACTED			= 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mantis_ca {
    pub slot: [mantis_slot; 4],
    pub hif_evm_work: work_struct,
    pub hif_event: u32,
    pub hif_opdone_wq: wait_queue_head_t,
    pub hif_brrdyw_wq: wait_queue_head_t,
    pub hif_data_wq: wait_queue_head_t,
    pub /: *mut *mut wait_queue_head_t hif_write_wq; / HIF Write op,
    pub sbuf_status: mantis_sbuf_status,
    pub slot_state: mantis_slot_state,
    pub ca_priv: *mut c_void,
    pub en50221: dvb_ca_en50221,
    pub ca_lock: mutex,
}

// CA
extern "C" {
    pub fn mantis_event_cam_plugin(ca: *mut mantis_ca);
}
extern "C" {
    pub fn mantis_event_cam_unplug(ca: *mut mantis_ca);
}
extern "C" {
    pub fn mantis_pcmcia_init(ca: *mut mantis_ca) -> c_int;
}
extern "C" {
    pub fn mantis_pcmcia_exit(ca: *mut mantis_ca);
}
extern "C" {
    pub fn mantis_evmgr_init(ca: *mut mantis_ca) -> c_int;
}
extern "C" {
    pub fn mantis_evmgr_exit(ca: *mut mantis_ca);
}
// HIF
extern "C" {
    pub fn mantis_hif_init(ca: *mut mantis_ca) -> c_int;
}
extern "C" {
    pub fn mantis_hif_exit(ca: *mut mantis_ca);
}
extern "C" {
    pub fn mantis_hif_read_mem(ca: *mut mantis_ca, addr: u32) -> c_int;
}
extern "C" {
    pub fn mantis_hif_write_mem(ca: *mut mantis_ca, addr: u32, data: u8) -> c_int;
}
extern "C" {
    pub fn mantis_hif_read_iom(ca: *mut mantis_ca, addr: u32) -> c_int;
}
extern "C" {
    pub fn mantis_hif_write_iom(ca: *mut mantis_ca, addr: u32, data: u8) -> c_int;
}
