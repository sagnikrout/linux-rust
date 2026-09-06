//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/libsas/sas_internal.h
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
// Serial Attached SCSI (SAS) class internal header file
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sas_phy_data {
// let reset be performed in sas_queue_work() context
    pub phy: *mut sas_phy,
    pub event_lock: mutex,
    pub hard_reset: c_int,
    pub reset_result: c_int,
    pub reset_work: sas_work,
    pub enable: c_int,
    pub enable_result: c_int,
    pub enable_work: sas_work,
}

extern "C" {
    pub fn sas_hash_addr(hashed: *mut u8, sas_addr: *const u8);
}
extern "C" {
    pub fn sas_discover_root_expander(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn sas_ex_revalidate_domain(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn sas_unregister_domain_devices(port: *mut asd_sas_port, gone: bool);
}
extern "C" {
    pub fn sas_init_disc(disc: *mut sas_discovery, port: *mut asd_sas_port);
}
extern "C" {
    pub fn sas_discover_event(port: *mut asd_sas_port, ev: discover_event);
}
extern "C" {
    pub fn sas_init_dev(dev: *mut domain_device);
}
extern "C" {
    pub fn sas_unregister_dev(port: *mut asd_sas_port, dev: *mut domain_device);
}
extern "C" {
    pub fn sas_scsi_recover_host(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn sas_register_phys(sas_ha: *mut sas_ha_struct) -> c_int;
}
extern "C" {
    pub fn sas_unregister_phys(sas_ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn sas_free_event(event: *mut asd_sas_event);
}
extern "C" {
    pub fn sas_free_task(task: *mut sas_task);
}
extern "C" {
    pub fn sas_register_ports(sas_ha: *mut sas_ha_struct) -> c_int;
}
extern "C" {
    pub fn sas_unregister_ports(sas_ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn sas_disable_revalidation(ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn sas_enable_revalidation(ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn sas_queue_deferred_work(ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn __sas_drain_work(ha: *mut sas_ha_struct);
}
extern "C" {
    pub fn sas_deform_port(phy: *mut asd_sas_phy, gone: bool);
}
extern "C" {
    pub fn sas_porte_bytes_dmaed(work: *mut work_struct);
}
extern "C" {
    pub fn sas_porte_broadcast_rcvd(work: *mut work_struct);
}
extern "C" {
    pub fn sas_porte_link_reset_err(work: *mut work_struct);
}
extern "C" {
    pub fn sas_porte_timer_event(work: *mut work_struct);
}
extern "C" {
    pub fn sas_porte_hard_reset(work: *mut work_struct);
}
extern "C" {
    pub fn sas_queue_work(ha: *mut sas_ha_struct, sw: *mut sas_work) -> bool;
}
extern "C" {
    pub fn sas_notify_lldd_dev_found(: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn sas_notify_lldd_dev_gone(: *mut domain_device);
}
extern "C" {
    pub fn sas_smp_get_phy_events(phy: *mut sas_phy) -> c_int;
}
extern "C" {
    pub fn sas_device_set_phy(dev: *mut domain_device, port: *mut sas_port);
}
extern "C" {
    pub fn sas_ex_phy_discover(dev: *mut domain_device, single: c_int) -> c_int;
}
extern "C" {
    pub fn sas_try_ata_reset(phy: *mut asd_sas_phy) -> c_int;
}
extern "C" {
    pub fn sas_free_device(kref: *mut kref);
}
extern "C" {
    pub fn sas_destruct_devices(port: *mut asd_sas_port);
}
extern "C" {
    pub fn sas_task_internal_done(task: *mut sas_task);
}
extern "C" {
    pub fn sas_task_internal_timedout(t: *mut timer_list);
}

extern "C" {
    pub fn sas_smp_host_handler(job: *mut bsg_job, shost: *mut Scsi_Host);
}

extern "C" {
    pub fn SAS_ADDR(SAS_ADDR(phy->attached_sas_addr: dev->sas_addr) ==) -> return;
}
extern "C" {
    pub fn SAS_ADDR(SAS_ADDR(phy->attached_sas_addr: port->sas_addr) ==) -> return;
}
extern "C" {
    pub fn SAS_ADDR(SAS_ADDR(p2->attached_sas_addr: p1->attached_sas_addr) ==) -> return;
}
// FIXME: need sata device type

extern "C" {
    pub fn sas_ata_init(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn sas_ata_task_abort(task: *mut sas_task);
}
extern "C" {
    pub fn sas_discover_sata(dev: *mut domain_device) -> c_int;
}
extern "C" {
    pub fn sas_ata_strategy_handler(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn sas_ata_eh(shost: *mut Scsi_Host, work_q: *mut list_head);
}
extern "C" {
    pub fn sas_ata_end_eh(ap: *mut ata_port);
}
extern "C" {
    pub fn sas_ata_wait_eh(dev: *mut domain_device);
}
extern "C" {
    pub fn sas_probe_sata(port: *mut asd_sas_port);
}
extern "C" {
    pub fn sas_suspend_sata(port: *mut asd_sas_port);
}
extern "C" {
    pub fn sas_resume_sata(port: *mut asd_sas_port);
}

