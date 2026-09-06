//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/aic94xx/aic94xx.h
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
// Aic94xx SAS/SATA driver header file.
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//
// $Id: //depot/aic94xx/aic94xx.h#31 $
//

// Macro flag: #define ENTER
// Macro flag: #define EXIT

// 2*ITNL timeout + 1 second

extern "C" {
    pub fn asd_read_ocm(asd_ha: *mut asd_ha_struct) -> c_int;
}
extern "C" {
    pub fn asd_read_flash(asd_ha: *mut asd_ha_struct) -> c_int;
}
extern "C" {
    pub fn asd_dev_found(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn asd_dev_gone(dev: *mut domain_device);
}
extern "C" {
    pub fn asd_invalidate_edb(ascb: *mut asd_ascb, edb_id: c_int);
}
extern "C" {
    pub fn asd_execute_task(task: *mut sas_task, gfp_flags: gfp_t) -> c_int;
}
extern "C" {
    pub fn asd_set_dmamode(dev: *mut domain_device);
}
// ---------- TMFs ----------
extern "C" {
    pub fn asd_abort_task(: *mut sas_task) -> c_int;
}
extern "C" {
    pub fn asd_abort_task_set(: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn asd_clear_task_set(: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn asd_lu_reset(: *mut domain_device, lun: *mut u8) -> c_int;
}
extern "C" {
    pub fn asd_I_T_nexus_reset(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn asd_query_task(: *mut sas_task) -> c_int;
}
// ---------- Adapter and Port management ----------
extern "C" {
    pub fn asd_clear_nexus_port(port: *mut asd_sas_port) -> c_int;
}
extern "C" {
    pub fn asd_clear_nexus_ha(sas_ha: *mut sas_ha_struct) -> c_int;
}
// ---------- Phy Management ----------
extern "C" {
    pub fn asd_control_phy(phy: *mut asd_sas_phy, func: phy_func, arg: *mut c_void) -> c_int;
}
