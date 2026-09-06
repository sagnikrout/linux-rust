//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/pensando/ionic/ionic.h
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
// Copyright(c) 2017 - 2019 Pensando Systems, Inc

pub const PCI_VENDOR_ID_PENSANDO: c_uint = 0x1dd8;
pub const PCI_DEVICE_ID_PENSANDO_IONIC_ETH_PF: c_uint = 0x1002;
pub const PCI_DEVICE_ID_PENSANDO_IONIC_ETH_VF: c_uint = 0x1003;
pub const DEVCMD_TIMEOUT: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic_vf {
    pub index: u16,
    pub macaddr: [u8; 6],
    pub maxrate: __le32,
    pub vlanid: __le16,
    pub spoofchk: u8,
    pub trusted: u8,
    pub linkstate: u8,
    pub stats_pa: dma_addr_t,
    pub stats: ionic_lif_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ionic {
    pub pdev: *mut pci_dev,
    pub dev: *mut device,
    pub dl_port: devlink_port,
    pub idev: ionic_dev,
    pub /: *mut *mut mutex dev_cmd_lock; / lock for dev_cmd operations,
    pub dentry: *mut dentry,
    pub bars: [ionic_dev_bar; IONIC_BARS_MAX],
    pub num_bars: c_uint,
    pub ident: ionic_identity,
    pub wq: *mut workqueue_struct,
    pub lif: *mut ionic_lif,
    pub nnqs_per_lif: c_uint,
    pub neqs_per_lif: c_uint,
    pub ntxqs_per_lif: c_uint,
    pub nrxqs_per_lif: c_uint,
    pub nintrs: c_uint,
    pub IONIC_INTR_CTRL_REGS_MAX): DECLARE_BITMAP(intrs,,
    pub affinity_masks: *mut cpumask_var_t,
    pub doorbell_check_dwork: delayed_work,
    pub nb: notifier_block,
    pub /: *mut *mut rw_semaphore vf_op_lock; / lock for VF operations,
    pub vfs: *mut ionic_vf,
    pub num_vfs: c_int,
    pub watchdog_timer: timer_list,
    pub watchdog_period: c_int,
}

extern "C" {
    pub fn ionic_adminq_post(lif: *mut ionic_lif, ctx: *mut ionic_admin_ctx) -> c_int;
}
extern "C" {
    pub fn ionic_adminq_post_wait_nomsg(lif: *mut ionic_lif, ctx: *mut ionic_admin_ctx) -> c_int;
}
extern "C" {
    pub fn ionic_notifyq_service(cq: *mut ionic_cq) -> bool;
}
extern "C" {
    pub fn ionic_adminq_service(cq: *mut ionic_cq) -> bool;
}
extern "C" {
    pub fn ionic_dev_cmd_wait(ionic: *mut ionic, max_wait: c_ulong) -> c_int;
}
extern "C" {
    pub fn ionic_dev_cmd_wait_nomsg(ionic: *mut ionic, max_wait: c_ulong) -> c_int;
}
extern "C" {
    pub fn ionic_setup(ionic: *mut ionic) -> c_int;
}
extern "C" {
    pub fn ionic_identify(ionic: *mut ionic) -> c_int;
}
extern "C" {
    pub fn ionic_init(ionic: *mut ionic) -> c_int;
}
extern "C" {
    pub fn ionic_reset(ionic: *mut ionic) -> c_int;
}
extern "C" {
    pub fn ionic_port_identify(ionic: *mut ionic) -> c_int;
}
extern "C" {
    pub fn ionic_port_init(ionic: *mut ionic) -> c_int;
}
extern "C" {
    pub fn ionic_port_reset(ionic: *mut ionic) -> c_int;
}
extern "C" {
    pub fn ionic_doorbell_wa(ionic: *mut ionic) -> bool;
}
