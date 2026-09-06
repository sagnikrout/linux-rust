//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/TUXEDO__Sirius-16-Gen1-and-Gen2.bpf.c
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
// Copyright (c) 2025 TUXEDO Computers GmbH
//

    HID_BPF_CONFIG(
    HID_DEVICE(BUS_USB, HID_GROUP_GENERIC, 0x048D, 0x8910)
    );
    SEC(HID_BPF_DEVICE_EVENT)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: ignore_key_fix_event, hid_ctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(ignore_key_fix_event, struct hid_bpf_ctx *hid_ctx)
    {
    let mut expected_length: c_int = 37;
    let mut expected_report_id: c_int = 1;
    __u8 *data;
    int i;
    if (hid_ctx.size < expected_length)
    return 0;
    data = hid_bpf_get_data(hid_ctx, 0, expected_length);
    if (!data || data[0] != expected_report_id)
    return 0;
// Zero out F13 (HID usage ID: 0x68) key press.
// The first 6 parallel key presses (excluding modifier keys) are
// encoded in an array containing usage IDs.
    for (i = 3; i < 9; ++i)
    if (data[i] == 0x68)
    data[i] = 0x00;
// Additional parallel key presses starting with the 7th (excluding
// modifier keys) are encoded as a bit flag with the offset being
// the usage ID.
    data[22] &= 0xfe;
    return 0;
    }
    HID_BPF_OPS(ignore_button) = {
    .hid_device_event = (void *)ignore_key_fix_event,
    };
    char _license[] SEC("license") = "GPL";
