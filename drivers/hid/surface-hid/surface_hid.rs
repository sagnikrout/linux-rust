//! Automatically rewritten from C to Rust
//! Source: drivers/hid/surface-hid/surface_hid.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Surface System Aggregator Module (SSAM) HID transport driver for the
// generic HID interface (HID/TC=0x15 subsystem). Provides support for
// integrated HID devices on Surface Laptop 3, Book 3, and later.
//
// Copyright (C) 2019-2021 Blaž Hrastnik <blaz@mxxn.io>,
// Maximilian Luz <luzmaximilian@gmail.com>
//

// -- SAM interface. --------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct surface_hid_buffer_slice {
    pub entry: __u8,
    pub offset: __le32,
    pub length: __le32,
    pub end: __u8,
    pub data: [__u8; ],
    pub __packed: },
    pub 10): static_assert(sizeof(struct surface_hid_buffer_slice) ==,
    enum surface_hid_cid {
    SURFACE_HID_CID_OUTPUT_REPORT      = 0x01,
    SURFACE_HID_CID_GET_FEATURE_REPORT = 0x02,
    SURFACE_HID_CID_SET_FEATURE_REPORT = 0x03,
    SURFACE_HID_CID_GET_DESCRIPTOR     = 0x04,
}

#[no_mangle]
unsafe extern "C" fn ssam_hid_get_descriptor(shid: *mut surface_hid_device, entry: u8, buf: *mut u8, len: usize) -> c_int {
    static int ssam_hid_get_descriptor(struct surface_hid_device *shid, u8 entry, u8 *buf, size_t len)
    {
    u8 buffer[sizeof(struct surface_hid_buffer_slice) + 0x76];
    struct surface_hid_buffer_slice *slice;
    struct ssam_request rqst;
    struct ssam_response rsp;
    u32 buffer_len, offset, length;
    int status;
//
// Note: The 0x76 above has been chosen because that's what's used by
// the Windows driver. Together with the header, this leads to a 128
// byte payload in total.
//
    buffer_len = ARRAY_SIZE(buffer) - sizeof(struct surface_hid_buffer_slice);
    rqst.target_category = shid.uid.category;
    rqst.target_id = shid.uid.target;
    rqst.command_id = SURFACE_HID_CID_GET_DESCRIPTOR;
    rqst.instance_id = shid.uid.instance;
    rqst.flags = SSAM_REQUEST_HAS_RESPONSE;
    rqst.length = sizeof(struct surface_hid_buffer_slice);
    rqst.payload = buffer;
    rsp.capacity = ARRAY_SIZE(buffer);
    rsp.pointer = buffer;
    slice = (struct surface_hid_buffer_slice *)buffer;
    slice.entry = entry;
    slice.end = 0;
    offset = 0;
    length = buffer_len;
    while (!slice.end && offset < len) {
    put_unaligned_le32(offset, &slice.offset);
    put_unaligned_le32(length, &slice.length);
    rsp.length = 0;
    status = ssam_retry(ssam_request_do_sync_onstack, shid.ctrl, &rqst, &rsp,
    sizeof(*slice));
    if (status)
    return status;
    offset = get_unaligned_le32(&slice.offset);
    length = get_unaligned_le32(&slice.length);
// Don't mess stuff up in case we receive garbage.
    if (length > buffer_len || offset > len)
    return -EPROTO;
    if (offset + length > len)
    length = len - offset;
    memcpy(buf + offset, &slice.data[0], length);
    offset += length;
    length = buffer_len;
    }
    if (offset != len) {
    dev_err(shid.dev, "unexpected descriptor length: got %u, expected %zu\n",
    offset, len);
    return -EPROTO;
    }
    return 0;
    }
    static int ssam_hid_set_raw_report(struct surface_hid_device *shid, u8 rprt_id, bool feature,
    u8 *buf, size_t len)
    {
    struct ssam_request rqst;
    u8 cid;
    if (feature)
    cid = SURFACE_HID_CID_SET_FEATURE_REPORT;
    else
    cid = SURFACE_HID_CID_OUTPUT_REPORT;
    rqst.target_category = shid.uid.category;
    rqst.target_id = shid.uid.target;
    rqst.instance_id = shid.uid.instance;
    rqst.command_id = cid;
    rqst.flags = 0;
    rqst.length = len;
    rqst.payload = buf;
    buf[0] = rprt_id;
    return ssam_retry(ssam_request_do_sync, shid.ctrl, &rqst, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ssam_hid_get_raw_report(shid: *mut surface_hid_device, rprt_id: u8, buf: *mut u8, len: usize) -> c_int {
    static int ssam_hid_get_raw_report(struct surface_hid_device *shid, u8 rprt_id, u8 *buf, size_t len)
    {
    struct ssam_request rqst;
    struct ssam_response rsp;
    rqst.target_category = shid.uid.category;
    rqst.target_id = shid.uid.target;
    rqst.instance_id = shid.uid.instance;
    rqst.command_id = SURFACE_HID_CID_GET_FEATURE_REPORT;
    rqst.flags = SSAM_REQUEST_HAS_RESPONSE;
    rqst.length = sizeof(rprt_id);
    rqst.payload = &rprt_id;
    rsp.capacity = len;
    rsp.length = 0;
    rsp.pointer = buf;
    return ssam_retry(ssam_request_do_sync_onstack, shid.ctrl, &rqst, &rsp, sizeof(rprt_id));
    }
#[no_mangle]
unsafe extern "C" fn ssam_hid_event_fn(nf: *mut ssam_event_notifier, event: *const ssam_event) -> u32 {
    static u32 ssam_hid_event_fn(struct ssam_event_notifier *nf, const struct ssam_event *event)
    {
    struct surface_hid_device *shid = container_of(nf, struct surface_hid_device, notif);
    if (event.command_id != 0x00)
    return 0;
    hid_input_report(shid.hid, HID_INPUT_REPORT, (u8 *)&event.data[0], event.length, 0);
    return SSAM_NOTIF_HANDLED;
    }
// -- Transport driver. -----------------------------------------------------
#[no_mangle]
unsafe extern "C" fn shid_output_report(shid: *mut surface_hid_device, rprt_id: u8, buf: *mut u8, len: usize) -> c_int {
    static int shid_output_report(struct surface_hid_device *shid, u8 rprt_id, u8 *buf, size_t len)
    {
    int status;
    status = ssam_hid_set_raw_report(shid, rprt_id, false, buf, len);
    return status >= 0 ? len : status;
    }
#[no_mangle]
unsafe extern "C" fn shid_get_feature_report(shid: *mut surface_hid_device, rprt_id: u8, buf: *mut u8, len: usize) -> c_int {
    static int shid_get_feature_report(struct surface_hid_device *shid, u8 rprt_id, u8 *buf, size_t len)
    {
    int status;
    status = ssam_hid_get_raw_report(shid, rprt_id, buf, len);
    return status >= 0 ? len : status;
    }
#[no_mangle]
unsafe extern "C" fn shid_set_feature_report(shid: *mut surface_hid_device, rprt_id: u8, buf: *mut u8, len: usize) -> c_int {
    static int shid_set_feature_report(struct surface_hid_device *shid, u8 rprt_id, u8 *buf, size_t len)
    {
    int status;
    status = ssam_hid_set_raw_report(shid, rprt_id, true, buf, len);
    return status >= 0 ? len : status;
    }
// -- Driver setup. ---------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn surface_hid_probe(sdev: *mut ssam_device) -> c_int {
    static int surface_hid_probe(struct ssam_device *sdev)
    {
    struct surface_hid_device *shid;
    shid = devm_kzalloc(&sdev.dev, sizeof(*shid), GFP_KERNEL);
    if (!shid)
    return -ENOMEM;
    shid.dev = &sdev.dev;
    shid.ctrl = sdev.ctrl;
    shid.uid = sdev.uid;
    shid.notif.base.priority = 1;
    shid.notif.base.fn = ssam_hid_event_fn;
    shid.notif.event.reg = SSAM_EVENT_REGISTRY_REG(sdev.uid.target);
    shid.notif.event.id.target_category = sdev.uid.category;
    shid.notif.event.id.instance = sdev.uid.instance;
    shid.notif.event.mask = SSAM_EVENT_MASK_STRICT;
    shid.notif.event.flags = 0;
    shid.ops.get_descriptor = ssam_hid_get_descriptor;
    shid.ops.output_report = shid_output_report;
    shid.ops.get_feature_report = shid_get_feature_report;
    shid.ops.set_feature_report = shid_set_feature_report;
    ssam_device_set_drvdata(sdev, shid);
    return surface_hid_device_add(shid);
    }
#[no_mangle]
unsafe extern "C" fn surface_hid_remove(sdev: *mut ssam_device) {
    static void surface_hid_remove(struct ssam_device *sdev)
    {
    surface_hid_device_destroy(ssam_device_get_drvdata(sdev));
    }
    static const struct ssam_device_id surface_hid_match[] = {
    { SSAM_SDEV(HID, ANY, SSAM_SSH_IID_ANY, 0x00) },
    { },
    };
    MODULE_DEVICE_TABLE(ssam, surface_hid_match);
    static struct ssam_device_driver surface_hid_driver = {
    .probe = surface_hid_probe,
    .remove = surface_hid_remove,
    .match_table = surface_hid_match,
    .driver = {
    .name = "surface_hid",
    .pm = &surface_hid_pm_ops,
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    };
    module_ssam_device_driver(surface_hid_driver);
    MODULE_AUTHOR("Blaž Hrastnik <blaz@mxxn.io>");
    MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
    MODULE_DESCRIPTION("HID transport driver for Surface System Aggregator Module");
    MODULE_LICENSE("GPL");
