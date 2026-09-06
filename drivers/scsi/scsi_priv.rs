//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/scsi_priv.h
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

//
// Error codes used by scsi-ml internally. These must not be used by drivers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_ml_status {
    SCSIML_STAT_OK			= 0x00,
    SCSIML_STAT_RESV_CONFLICT	= 0x01,	/* Reservation conflict */
    SCSIML_STAT_NOSPC		= 0x02,	/* Space allocation on the dev failed */
    SCSIML_STAT_MED_ERROR		= 0x03,	/* Medium error */
    SCSIML_STAT_TGT_FAILURE		= 0x04,	/* Permanent target failure */
    SCSIML_STAT_DL_TIMEOUT		= 0x05, /* Command Duration Limit timeout */
}

//
// Scsi Error Handler Flags
//
pub const SCSI_EH_ABORT_SCHEDULED: c_uint = 0x0002	/* Abort has been scheduled */;

// hosts.c
extern "C" {
    pub fn scsi_init_hosts() -> c_int;
}
extern "C" {
    pub fn scsi_exit_hosts();
}
// scsi.c
extern "C" {
    pub fn scsi_init_sense_cache(shost: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn scsi_init_command(dev: *mut scsi_device, cmd: *mut scsi_cmnd);
}

extern "C" {
    pub fn scsi_log_send(cmd: *mut scsi_cmnd);
}
extern "C" {
    pub fn scsi_log_completion(cmd: *mut scsi_cmnd, disposition: c_int);
}

// scsi_devinfo.c
// list of keys for the lists
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_devinfo_key {
    SCSI_DEVINFO_GLOBAL = 0,
    SCSI_DEVINFO_SPI,
}

extern "C" {
    pub fn scsi_dev_info_add_list(key: scsi_devinfo_key, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn scsi_dev_info_remove_list(key: scsi_devinfo_key) -> c_int;
}
extern "C" {
    pub fn scsi_init_devinfo() -> int __init;
}
extern "C" {
    pub fn scsi_exit_devinfo();
}
// scsi_error.c
extern "C" {
    pub fn scmd_eh_abort_handler(work: *mut work_struct);
}
extern "C" {
    pub fn scsi_timeout(req: *mut request) -> blk_eh_timer_return;
}
extern "C" {
    pub fn scsi_error_handler(host: *mut c_void) -> c_int;
}
extern "C" {
    pub fn scsi_decide_disposition(cmd: *mut scsi_cmnd) -> scsi_disposition;
}
extern "C" {
    pub fn scsi_eh_wakeup(shost: *mut Scsi_Host, busy: c_uint);
}
extern "C" {
    pub fn scsi_rcu_eh_wakeup(work: *mut work_struct);
}
extern "C" {
    pub fn scsi_eh_scmd_add(: *mut scsi_cmnd);
}
extern "C" {
    pub fn scsi_noretry_cmd(scmd: *mut scsi_cmnd) -> bool;
}
extern "C" {
    pub fn scsi_eh_done(scmd: *mut scsi_cmnd);
}
// scsi_lib.c
extern "C" {
    pub fn scsi_device_unbusy(sdev: *mut scsi_device, cmd: *mut scsi_cmnd);
}
extern "C" {
    pub fn scsi_io_completion(: *mut scsi_cmnd, int: unsigned);
}
extern "C" {
    pub fn scsi_run_host_queues(shost: *mut Scsi_Host);
}
extern "C" {
    pub fn scsi_requeue_run_queue(work: *mut work_struct);
}
extern "C" {
    pub fn scsi_start_queue(sdev: *mut scsi_device);
}
extern "C" {
    pub fn scsi_mq_setup_tags(shost: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn scsi_mq_free_tags(kref: *mut kref);
}
extern "C" {
    pub fn scsi_exit_queue();
}
extern "C" {
    pub fn scsi_evt_thread(work: *mut work_struct);
}
// scsi_proc.c

extern "C" {
    pub fn scsi_proc_hostdir_add(: *const scsi_host_template) -> c_int;
}
extern "C" {
    pub fn scsi_proc_hostdir_rm(: *const scsi_host_template);
}
extern "C" {
    pub fn scsi_proc_host_add(: *mut Scsi_Host);
}
extern "C" {
    pub fn scsi_proc_host_rm(: *mut Scsi_Host);
}
extern "C" {
    pub fn scsi_init_procfs() -> c_int;
}
extern "C" {
    pub fn scsi_exit_procfs();
}

// scsi_scan.c
extern "C" {
    pub fn scsi_enable_async_suspend(dev: *mut device);
}
extern "C" {
    pub fn scsi_complete_async_scans() -> c_int;
}
extern "C" {
    pub fn scsi_forget_host(: *mut Scsi_Host);
}
// scsi_sysctl.c

extern "C" {
    pub fn scsi_init_sysctl() -> c_int;
}
extern "C" {
    pub fn scsi_exit_sysctl();
}

// scsi_sysfs.c
extern "C" {
    pub fn scsi_sysfs_add_sdev(: *mut scsi_device) -> c_int;
}
extern "C" {
    pub fn scsi_sysfs_add_host(: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn scsi_sysfs_register() -> c_int;
}
extern "C" {
    pub fn scsi_sysfs_unregister();
}
extern "C" {
    pub fn scsi_sysfs_device_initialize(: *mut scsi_device);
}
extern "C" {
    pub fn __scsi_remove_device(: *mut scsi_device);
}
// scsi_netlink.c

extern "C" {
    pub fn scsi_netlink_init();
}
extern "C" {
    pub fn scsi_netlink_exit();
}

// scsi_pm.c

extern "C" {
    pub fn scsi_autopm_get_target(: *mut scsi_target);
}
extern "C" {
    pub fn scsi_autopm_put_target(: *mut scsi_target);
}
extern "C" {
    pub fn scsi_autopm_get_host(: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn scsi_autopm_put_host(: *mut Scsi_Host);
}

// scsi_dh.c

extern "C" {
    pub fn scsi_dh_add_device(sdev: *mut scsi_device);
}
extern "C" {
    pub fn scsi_dh_release_device(sdev: *mut scsi_device);
}

extern "C" {
    pub fn scsi_device_max_queue_depth(sdev: *mut scsi_device) -> c_int;
}
//
// internal scsi timeout functions: for use by mid-layer and transport
// classes.
//

