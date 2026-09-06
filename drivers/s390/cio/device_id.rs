//! Automatically rewritten from C to Rust
//! Source: drivers/s390/cio/device_id.c
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
// CCW device SENSE ID I/O handling.
//
// Copyright IBM Corp. 2002, 2009
// Author(s): Cornelia Huck <cornelia.huck@de.ibm.com>
// Martin Schwidefsky <schwidefsky@de.ibm.com>
// Peter Oberparleiter <peter.oberparleiter@de.ibm.com>
//

pub const SENSE_ID_RETRIES: c_int = 256;

pub const SENSE_ID_MIN_LEN: c_int = 4;
pub const SENSE_ID_BASIC_LEN: c_int = 7;
//
// diag210_to_senseid - convert diag 0x210 data to sense id information
// @senseid: sense id
// @diag: diag 0x210 data
//
// Return 0 on success, non-zero otherwise.
//
#[no_mangle]
unsafe extern "C" fn diag210_to_senseid(senseid: *mut senseid, diag: *mut diag210) -> c_int {
    static int diag210_to_senseid(struct senseid *senseid, struct diag210 *diag)
    {
    static struct {
    int class, type, cu_type;
    } vm_devices[] = {
    { 0x08, 0x01, 0x3480 },
    { 0x08, 0x02, 0x3430 },
    { 0x08, 0x10, 0x3420 },
    { 0x08, 0x42, 0x3424 },
    { 0x08, 0x44, 0x9348 },
    { 0x08, 0x81, 0x3490 },
    { 0x08, 0x82, 0x3422 },
    { 0x10, 0x41, 0x1403 },
    { 0x10, 0x42, 0x3211 },
    { 0x10, 0x43, 0x3203 },
    { 0x10, 0x45, 0x3800 },
    { 0x10, 0x47, 0x3262 },
    { 0x10, 0x48, 0x3820 },
    { 0x10, 0x49, 0x3800 },
    { 0x10, 0x4a, 0x4245 },
    { 0x10, 0x4b, 0x4248 },
    { 0x10, 0x4d, 0x3800 },
    { 0x10, 0x4e, 0x3820 },
    { 0x10, 0x4f, 0x3820 },
    { 0x10, 0x82, 0x2540 },
    { 0x10, 0x84, 0x3525 },
    { 0x20, 0x81, 0x2501 },
    { 0x20, 0x82, 0x2540 },
    { 0x20, 0x84, 0x3505 },
    { 0x40, 0x01, 0x3278 },
    { 0x40, 0x04, 0x3277 },
    { 0x40, 0x80, 0x2250 },
    { 0x40, 0xc0, 0x5080 },
    { 0x80, 0x00, 0x3215 },
    };
    int i;
// Special case for osa devices.
    if (diag.vrdcvcla == 0x02 && diag.vrdcvtyp == 0x20) {
    senseid.cu_type = 0x3088;
    senseid.cu_model = 0x60;
    senseid.reserved = 0xff;
    return 0;
    }
    for (i = 0; i < ARRAY_SIZE(vm_devices); i++) {
    if (diag.vrdcvcla == vm_devices[i].class &&
    diag.vrdcvtyp == vm_devices[i].type) {
    senseid.cu_type = vm_devices[i].cu_type;
    senseid.reserved = 0xff;
    return 0;
    }
    }
    return -ENODEV;
    }
//
// diag210_get_dev_info - retrieve device information via diag 0x210
// @cdev: ccw device
//
// Returns zero on success, non-zero otherwise.
//
#[no_mangle]
unsafe extern "C" fn diag210_get_dev_info(cdev: *mut ccw_device) -> c_int {
    static int diag210_get_dev_info(struct ccw_device *cdev)
    {
    struct ccw_dev_id *dev_id = &cdev.private.dev_id;
    struct senseid *senseid = &cdev.private.dma_area.senseid;
    struct diag210 diag_data;
    int rc;
    if (dev_id.ssid != 0)
    return -ENODEV;
    memset(&diag_data, 0, sizeof(diag_data));
    diag_data.vrdcdvno	= dev_id.devno;
    diag_data.vrdclen	= sizeof(diag_data);
    rc = diag210(&diag_data);
    CIO_TRACE_EVENT(4, "diag210");
    CIO_HEX_EVENT(4, &rc, sizeof(rc));
    CIO_HEX_EVENT(4, &diag_data, sizeof(diag_data));
    if (rc != 0 && rc != 2)
    goto err_failed;
    if (diag210_to_senseid(senseid, &diag_data))
    goto err_unknown;
    return 0;
    err_unknown:
    CIO_MSG_EVENT(0, "snsid: device 0.%x.%04x: unknown diag210 data\n",
    dev_id.ssid, dev_id.devno);
    return -ENODEV;
    err_failed:
    CIO_MSG_EVENT(0, "snsid: device 0.%x.%04x: diag210 failed (rc=%d)\n",
    dev_id.ssid, dev_id.devno, rc);
    return -ENODEV;
    }
//
// Initialize SENSE ID data.
//
#[no_mangle]
unsafe extern "C" fn snsid_init(cdev: *mut ccw_device) {
    static void snsid_init(struct ccw_device *cdev)
    {
    cdev.private.flags.esid = 0;
    memset(&cdev.private.dma_area.senseid, 0,
    sizeof(cdev.private.dma_area.senseid));
    cdev.private.dma_area.senseid.cu_type = 0xffff;
    }
//
// Check for complete SENSE ID data.
//
#[no_mangle]
unsafe extern "C" fn snsid_check(cdev: *mut ccw_device, data: *mut c_void) -> c_int {
    static int snsid_check(struct ccw_device *cdev, void *data)
    {
    struct cmd_scsw *scsw = &cdev.private.dma_area.irb.scsw.cmd;
    let mut len: c_int = sizeof(struct senseid) - scsw.count;
// Check for incomplete SENSE ID data.
    if (len < SENSE_ID_MIN_LEN)
    goto out_restart;
    if (cdev.private.dma_area.senseid.cu_type == 0xffff)
    goto out_restart;
// Check for incompatible SENSE ID data.
    if (cdev.private.dma_area.senseid.reserved != 0xff)
    return -EOPNOTSUPP;
// Check for extended-identification information.
    if (len > SENSE_ID_BASIC_LEN)
    cdev.private.flags.esid = 1;
    return 0;
    out_restart:
    snsid_init(cdev);
    return -EAGAIN;
    }
//
// Process SENSE ID request result.
//
#[no_mangle]
unsafe extern "C" fn snsid_callback(cdev: *mut ccw_device, data: *mut c_void, rc: c_int) {
    static void snsid_callback(struct ccw_device *cdev, void *data, int rc)
    {
    struct ccw_dev_id *id = &cdev.private.dev_id;
    struct senseid *senseid = &cdev.private.dma_area.senseid;
    let mut vm: c_int = 0;
    if (rc && machine_is_vm()) {
// Try diag 0x210 fallback on z/VM.
    snsid_init(cdev);
    if (diag210_get_dev_info(cdev) == 0) {
    rc = 0;
    vm = 1;
    }
    }
    CIO_MSG_EVENT(2, "snsid: device 0.%x.%04x: rc=%d %04x/%02x "
    "%04x/%02x%s\n", id.ssid, id.devno, rc,
    senseid.cu_type, senseid.cu_model, senseid.dev_type,
    senseid.dev_model, vm ? " (diag210)" : "");
    ccw_device_sense_id_done(cdev, rc);
    }
//
// ccw_device_sense_id_start - perform SENSE ID
// @cdev: ccw device
//
// Execute a SENSE ID channel program on @cdev to update its sense id
// information. When finished, call ccw_device_sense_id_done with a
// return code specifying the result.
//
#[no_mangle]
pub unsafe extern "C" fn ccw_device_sense_id_start(cdev: *mut ccw_device) {
    void ccw_device_sense_id_start(struct ccw_device *cdev)
    {
    struct subchannel *sch = to_subchannel(cdev.dev.parent);
    struct ccw_request *req = &cdev.private.req;
    struct ccw1 *cp = cdev.private.dma_area.iccws;
    CIO_TRACE_EVENT(4, "snsid");
    CIO_HEX_EVENT(4, &cdev.private.dev_id, sizeof(cdev.private.dev_id));
// Data setup.
    snsid_init(cdev);
// Channel program setup.
    cp.cmd_code	= CCW_CMD_SENSE_ID;
    cp.cda		= virt_to_dma32(&cdev.private.dma_area.senseid);
    cp.count	= sizeof(struct senseid);
    cp.flags	= CCW_FLAG_SLI;
// Request setup.
    memset(req, 0, sizeof(*req));
    req.cp		= cp;
    req.timeout	= SENSE_ID_TIMEOUT;
    req.maxretries	= SENSE_ID_RETRIES;
    req.lpm	= sch.schib.pmcw.pam & sch.opm;
    req.check	= snsid_check;
    req.callback	= snsid_callback;
    ccw_request_start(cdev);
    }
