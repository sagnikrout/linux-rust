//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ata/libata.h
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
// libata.h - helper library for ATA
//
// Copyright 2003-2004 Red Hat, Inc.  All rights reserved.
// Copyright 2003-2004 Jeff Garzik
//
// libata documentation is available via 'make {ps|pdf}docs',
// as Documentation/driver-api/libata.rst
//

// libata-core.c
// flags for ata_dev_read_id()
// selector for ata_down_xfermask_limit()

// Host managed device or host aware device

extern "C" {
    pub fn ata_force_cbl(ap: *mut ata_port);
}

extern "C" {
    pub fn ata_tf_to_lba(tf: *const ata_taskfile) -> u64;
}
extern "C" {
    pub fn ata_tf_to_lba48(tf: *const ata_taskfile) -> u64;
}
extern "C" {
    pub fn ata_dev_reread_id(dev: *mut ata_device, readid_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn ata_dev_configure(dev: *mut ata_device) -> c_int;
}
extern "C" {
    pub fn ata_dev_power_set_standby(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_dev_power_set_active(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_dev_free_resources(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_down_xfermask_limit(dev: *mut ata_device, sel: c_uint) -> c_int;
}
extern "C" {
    pub fn ata_qc_free(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn __ata_qc_complete(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn atapi_check_dma(qc: *mut ata_queued_cmd) -> c_int;
}
extern "C" {
    pub fn swap_buf_le16(buf: *mut u16, buf_words: c_uint);
}
extern "C" {
    pub fn ata_phys_link_online(link: *mut ata_link) -> bool;
}
extern "C" {
    pub fn ata_phys_link_offline(link: *mut ata_link) -> bool;
}
extern "C" {
    pub fn ata_adapter_is_online(ap: *mut ata_port) -> bool;
}
extern "C" {
    pub fn ata_dev_init(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_link_init(ap: *mut ata_port, link: *mut ata_link, pmp: c_int);
}
extern "C" {
    pub fn sata_link_init_spd(link: *mut ata_link) -> c_int;
}
extern "C" {
    pub fn ata_task_ioctl(scsidev: *mut scsi_device, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn ata_cmd_ioctl(scsidev: *mut scsi_device, arg: *mut void __user) -> c_int;
}

// libata-sata.c

extern "C" {
    pub fn sata_down_spd_limit(link: *mut ata_link, spd_limit: u32) -> c_int;
}
extern "C" {
    pub fn ata_eh_get_ncq_success_sense(link: *mut ata_link) -> c_int;
}

// libata-acpi.c

extern "C" {
    pub fn ata_acpi_dissociate(host: *mut ata_host);
}
extern "C" {
    pub fn ata_acpi_on_resume(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_acpi_on_devcfg(dev: *mut ata_device) -> c_int;
}
extern "C" {
    pub fn ata_acpi_on_disable(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_acpi_set_state(ap: *mut ata_port, state: pm_message_t);
}
extern "C" {
    pub fn ata_acpi_bind_port(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_acpi_bind_dev(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_acpi_port_power_on(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_acpi_dev_manage_restart(dev: *mut ata_device) -> bool;
}
extern "C" {
    pub fn ata_dev_acpi_handle(dev: *mut ata_device) -> acpi_handle;
}

// libata-scsi.c
pub const ATA_SCSI_RBUF_SIZE: c_int = 2048;
//
// Maximum number of concurrent positioning ranges (CPR) supported. The ACS
// specifications allow up to 255, but we limit this to the number of CPR
// descriptors that fit in the rbuf buffer used to emit VPD page B9h.
//

extern "C" {
    pub fn ata_scsi_scan_host(ap: *mut ata_port, sync: c_int);
}
extern "C" {
    pub fn ata_scsi_offline_dev(dev: *mut ata_device) -> bool;
}
extern "C" {
    pub fn ata_scsi_sense_is_valid(sk: u8, asc: u8, ascq: u8) -> bool;
}
extern "C" {
    pub fn ata_scsi_media_change_notify(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_scsi_hotplug(work: *mut work_struct);
}
extern "C" {
    pub fn ata_scsi_dev_rescan(work: *mut work_struct);
}
extern "C" {
    pub fn ata_scsi_sdev_config(sdev: *mut scsi_device);
}
extern "C" {
    pub fn ata_scsi_deferred_qc_work(work: *mut work_struct);
}
// libata-eh.c
extern "C" {
    pub fn ata_internal_cmd_timeout(dev: *mut ata_device, cmd: u8) -> c_uint;
}
extern "C" {
    pub fn ata_internal_cmd_timed_out(dev: *mut ata_device, cmd: u8);
}
extern "C" {
    pub fn ata_scsi_error(host: *mut Scsi_Host);
}
extern "C" {
    pub fn ata_eh_fastdrain_timerfn(t: *mut timer_list);
}
extern "C" {
    pub fn ata_qc_schedule_eh(qc: *mut ata_queued_cmd);
}
extern "C" {
    pub fn ata_dev_disable(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_eh_detach_dev(dev: *mut ata_device);
}
extern "C" {
    pub fn ata_eh_autopsy(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_eh_report(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_eh_finish(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_eh_decide_disposition(qc: *mut ata_queued_cmd) -> scsi_disposition;
}
extern "C" {
    pub fn atapi_eh_tur(dev: *mut ata_device, r_sense_key: *mut u8) -> c_uint;
}
// libata-pmp.c

extern "C" {
    pub fn sata_pmp_scr_read(link: *mut ata_link, reg: c_int, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn sata_pmp_scr_write(link: *mut ata_link, reg: c_int, val: u32) -> c_int;
}
extern "C" {
    pub fn sata_pmp_attach(dev: *mut ata_device) -> c_int;
}

// libata-sff.c

extern "C" {
    pub fn ata_sff_flush_pio_task(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sff_port_init(ap: *mut ata_port);
}
extern "C" {
    pub fn ata_sff_init() -> c_int;
}
extern "C" {
    pub fn ata_sff_exit();
}

// libata-zpodd.c

extern "C" {
    pub fn zpodd_init(dev: *mut ata_device);
}
extern "C" {
    pub fn zpodd_exit(dev: *mut ata_device);
}
extern "C" {
    pub fn zpodd_on_suspend(dev: *mut ata_device);
}
extern "C" {
    pub fn zpodd_zpready(dev: *mut ata_device) -> bool;
}
extern "C" {
    pub fn zpodd_enable_run_wake(dev: *mut ata_device);
}
extern "C" {
    pub fn zpodd_disable_run_wake(dev: *mut ata_device);
}
extern "C" {
    pub fn zpodd_post_poweron(dev: *mut ata_device);
}

