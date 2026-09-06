//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/device_handler/scsi_dh_hp_sw.c
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
// Basic HP/COMPAQ MSA 1000 support. This is only needed if your HW cannot be
// upgraded.
//
// Copyright (C) 2006 Red Hat, Inc.  All rights reserved.
// Copyright (C) 2006 Mike Christie
// Copyright (C) 2008 Hannes Reinecke <hare@suse.de>
//

pub const HP_SW_RETRIES: c_int = 3;

pub const HP_SW_PATH_ACTIVE: c_int = 0;
pub const HP_SW_PATH_PASSIVE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hp_sw_dh_data {
    pub path_state: c_int,
    pub retries: c_int,
    pub retry_cnt: c_int,
    pub sdev: *mut scsi_device,
}

    static int hp_sw_start_stop(struct hp_sw_dh_data *);
//
// tur_done - Handle TEST UNIT READY return status
// @sdev: sdev the command has been sent to
// @errors: blk error code
//
// Returns SCSI_DH_DEV_OFFLINED if the sdev is on the passive path
//
    static int tur_done(struct scsi_device *sdev, struct hp_sw_dh_data *h,
    struct scsi_sense_hdr *sshdr)
    {
    let mut ret: c_int = SCSI_DH_IO;
    switch (sshdr.sense_key) {
    case NOT_READY:
    if (sshdr.asc == 0x04 && sshdr.ascq == 2) {
//
// LUN not ready - Initialization command required
//
// This is the passive path
//
    h.path_state = HP_SW_PATH_PASSIVE;
    ret = SCSI_DH_OK;
    break;
    }
    fallthrough;
    default:
    sdev_printk(KERN_WARNING, sdev,
    "%s: sending tur failed, sense %x/%x/%x\n",
    HP_SW_NAME, sshdr.sense_key, sshdr.asc,
    sshdr.ascq);
    break;
    }
    return ret;
    }
//
// hp_sw_tur - Send TEST UNIT READY
// @sdev: sdev command should be sent to
//
// Use the TEST UNIT READY command to determine
// the path state.
//
#[no_mangle]
unsafe extern "C" fn hp_sw_tur(sdev: *mut scsi_device, h: *mut hp_sw_dh_data) -> c_int {
    static int hp_sw_tur(struct scsi_device *sdev, struct hp_sw_dh_data *h)
    {
    unsigned char cmd[6] = { TEST_UNIT_READY };
    struct scsi_sense_hdr sshdr;
    int ret, res;
    blk_opf_t opf = REQ_OP_DRV_IN | REQ_FAILFAST_DEV |
    REQ_FAILFAST_TRANSPORT | REQ_FAILFAST_DRIVER;
    struct scsi_failure failure_defs[] = {
    {
    .sense = UNIT_ATTENTION,
    .asc = SCMD_FAILURE_ASC_ANY,
    .ascq = SCMD_FAILURE_ASCQ_ANY,
    .allowed = SCMD_FAILURE_NO_LIMIT,
    .result = SAM_STAT_CHECK_CONDITION,
    },
    {}
    };
    struct scsi_failures failures = {
    .failure_definitions = failure_defs,
    };
    const struct scsi_exec_args exec_args = {
    .sshdr = &sshdr,
    .failures = &failures,
    };
    res = scsi_execute_cmd(sdev, cmd, opf, core::ptr::null_mut(), 0, HP_SW_TIMEOUT,
    HP_SW_RETRIES, &exec_args);
    if (res > 0 && scsi_sense_valid(&sshdr)) {
    ret = tur_done(sdev, h, &sshdr);
    } else if (res == 0) {
    h.path_state = HP_SW_PATH_ACTIVE;
    ret = SCSI_DH_OK;
    } else {
    sdev_printk(KERN_WARNING, sdev,
    "%s: sending tur failed with %x\n",
    HP_SW_NAME, res);
    ret = SCSI_DH_IO;
    }
    return ret;
    }
//
// hp_sw_start_stop - Send START STOP UNIT command
// @sdev: sdev command should be sent to
//
// Sending START STOP UNIT activates the SP.
//
#[no_mangle]
unsafe extern "C" fn hp_sw_start_stop(h: *mut hp_sw_dh_data) -> c_int {
    static int hp_sw_start_stop(struct hp_sw_dh_data *h)
    {
    unsigned char cmd[6] = { START_STOP, 0, 0, 0, 1, 0 };
    struct scsi_sense_hdr sshdr;
    struct scsi_device *sdev = h.sdev;
    int res, rc;
    blk_opf_t opf = REQ_OP_DRV_IN | REQ_FAILFAST_DEV |
    REQ_FAILFAST_TRANSPORT | REQ_FAILFAST_DRIVER;
    struct scsi_failure failure_defs[] = {
    {
//
// LUN not ready - manual intervention required
//
// Switch-over in progress, retry.
//
    .sense = NOT_READY,
    .asc = 0x04,
    .ascq = 0x03,
    .allowed = HP_SW_RETRIES,
    .result = SAM_STAT_CHECK_CONDITION,
    },
    {}
    };
    struct scsi_failures failures = {
    .failure_definitions = failure_defs,
    };
    const struct scsi_exec_args exec_args = {
    .sshdr = &sshdr,
    .failures = &failures,
    };
    res = scsi_execute_cmd(sdev, cmd, opf, core::ptr::null_mut(), 0, HP_SW_TIMEOUT,
    HP_SW_RETRIES, &exec_args);
    if (!res) {
    return SCSI_DH_OK;
    } else if (res < 0 || !scsi_sense_valid(&sshdr)) {
    sdev_printk(KERN_WARNING, sdev,
    "%s: sending start_stop_unit failed, "
    "no sense available\n", HP_SW_NAME);
    return SCSI_DH_IO;
    }
    switch (sshdr.sense_key) {
    case NOT_READY:
    if (sshdr.asc == 0x04 && sshdr.ascq == 3) {
    rc = SCSI_DH_RETRY;
    break;
    }
    fallthrough;
    default:
    sdev_printk(KERN_WARNING, sdev,
    "%s: sending start_stop_unit failed, "
    "sense %x/%x/%x\n", HP_SW_NAME,
    sshdr.sense_key, sshdr.asc, sshdr.ascq);
    rc = SCSI_DH_IO;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn hp_sw_prep_fn(sdev: *mut scsi_device, req: *mut request) -> blk_status_t {
    static blk_status_t hp_sw_prep_fn(struct scsi_device *sdev, struct request *req)
    {
    struct hp_sw_dh_data *h = sdev.handler_data;
    if (h.path_state != HP_SW_PATH_ACTIVE) {
    req.rq_flags |= RQF_QUIET;
    return BLK_STS_IOERR;
    }
    return BLK_STS_OK;
    }
//
// hp_sw_activate - Activate a path
// @sdev: sdev on the path to be activated
//
// The HP Active/Passive firmware is pretty simple;
// the passive path reports NOT READY with sense codes
// 0x04/0x02; a START STOP UNIT command will then
// activate the passive path (and deactivate the
// previously active one).
//
    static int hp_sw_activate(struct scsi_device *sdev,
    activate_complete fn, void *data)
    {
    let mut ret: c_int = SCSI_DH_OK;
    struct hp_sw_dh_data *h = sdev.handler_data;
    ret = hp_sw_tur(sdev, h);
    if (ret == SCSI_DH_OK && h.path_state == HP_SW_PATH_PASSIVE)
    ret = hp_sw_start_stop(h);
    if (fn)
    fn(data, ret);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hp_sw_bus_attach(sdev: *mut scsi_device) -> c_int {
    static int hp_sw_bus_attach(struct scsi_device *sdev)
    {
    struct hp_sw_dh_data *h;
    int ret;
    h = kzalloc_obj(*h);
    if (!h)
    return SCSI_DH_NOMEM;
    h.path_state = HP_SW_PATH_UNINITIALIZED;
    h.retries = HP_SW_RETRIES;
    h.sdev = sdev;
    ret = hp_sw_tur(sdev, h);
    if (ret != SCSI_DH_OK)
    goto failed;
    if (h.path_state == HP_SW_PATH_UNINITIALIZED) {
    ret = SCSI_DH_NOSYS;
    goto failed;
    }
    sdev_printk(KERN_INFO, sdev, "%s: attached to %s path\n",
    HP_SW_NAME, h.path_state == HP_SW_PATH_ACTIVE?
    "active":"passive");
    sdev.handler_data = h;
    return SCSI_DH_OK;
    failed:
    kfree(h);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hp_sw_bus_detach(sdev: *mut scsi_device) {
    static void hp_sw_bus_detach( struct scsi_device *sdev )
    {
    kfree(sdev.handler_data);
    sdev.handler_data = core::ptr::null_mut();
    }
    static struct scsi_device_handler hp_sw_dh = {
    .name		= HP_SW_NAME,
    .module		= THIS_MODULE,
    .attach		= hp_sw_bus_attach,
    .detach		= hp_sw_bus_detach,
    .activate	= hp_sw_activate,
    .prep_fn	= hp_sw_prep_fn,
    };
#[no_mangle]
unsafe extern "C" fn hp_sw_init() -> int __init {
    static int __init hp_sw_init(void)
    {
    return scsi_register_device_handler(&hp_sw_dh);
    }
#[no_mangle]
unsafe extern "C" fn hp_sw_exit() -> void __exit {
    static void __exit hp_sw_exit(void)
    {
    scsi_unregister_device_handler(&hp_sw_dh);
    }
    module_init(hp_sw_init);
    module_exit(hp_sw_exit);
    MODULE_DESCRIPTION("HP Active/Passive driver");
    MODULE_AUTHOR("Mike Christie <michaelc@cs.wisc.edu");
    MODULE_LICENSE("GPL");
