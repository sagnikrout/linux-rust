//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/Generic__touchpad.bpf.c
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
// Copyright (c) 2025 Benjamin Tissoires
//

    HID_BPF_CONFIG(
    HID_DEVICE(BUS_ANY, HID_GROUP_MULTITOUCH_WIN_8, HID_VID_ANY, HID_PID_ANY),
    );
    EXPORT_UDEV_PROP(HID_DIGITIZER_PAD_TYPE, 32);
    __u8 hw_req_buf[1024];
// to be filled by udev-hid-bpf
    struct hid_rdesc_descriptor HID_REPORT_DESCRIPTOR;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut hid_bpf_probe_args) -> c_int {
    int probe(struct hid_bpf_probe_args *ctx)
    {
    struct hid_rdesc_report *pad_type_feature = core::ptr::null_mut();
    struct hid_rdesc_field *pad_type = core::ptr::null_mut();
    struct hid_rdesc_report *feature;
    struct hid_bpf_ctx *hid_ctx;
    char *pad_type_str = "";
    int ret;
    hid_bpf_for_each_feature_report(&HID_REPORT_DESCRIPTOR, feature) {
    struct hid_rdesc_field *field;
    hid_bpf_for_each_field(feature, field) {
    if (field.usage_page == HidUsagePage_Digitizers &&
    field.usage_id == HidUsage_Dig_PadType) {
    pad_type = field;
    pad_type_feature = feature;
    break;
    }
    }
    if (pad_type)
    break;
    }
    if (!pad_type || !pad_type_feature) {
    ctx.retval = -EINVAL;
    return 0;
    }
    hid_ctx = hid_bpf_allocate_context(ctx.hid);
    if (!hid_ctx)
    return -1; /* EPERM check */
    hw_req_buf[0] = pad_type_feature.report_id;
    ret = hid_bpf_hw_request(hid_ctx, hw_req_buf, sizeof(hw_req_buf),
    HID_FEATURE_REPORT, HID_REQ_GET_REPORT);
    hid_bpf_release_context(hid_ctx);
    if (ret < 0) {
    ctx.retval = ret;
    return 0;
    }
    ctx.retval = 0;
    switch (EXTRACT_BITS(hw_req_buf, pad_type)) {
    case 0:
    pad_type_str = "Clickpad";
    break;
    case 1:
    pad_type_str = "Pressurepad";
    break;
    case 2:
    pad_type_str = "Discrete";
    break;
    default:
    pad_type_str = "Unknown";
    }
    UDEV_PROP_SPRINTF(HID_DIGITIZER_PAD_TYPE, "%s", pad_type_str);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
