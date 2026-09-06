//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/IOGEAR__Kaliber-MMOmentum.bpf.c
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
// Copyright (c) 2023 Benjamin Tissoires
//

pub const VID_IOGEAR: c_uint = 0x258A /* VID is shared with SinoWealth and Glorious and prob others */;
pub const PID_MOMENTUM: c_uint = 0x0027;
    HID_BPF_CONFIG(
    HID_DEVICE(BUS_USB, HID_GROUP_GENERIC, VID_IOGEAR, PID_MOMENTUM)
    );
//
// The IOGear Kaliber Gaming MMOmentum Pro mouse has multiple buttons (12)
// but only 5 are accessible out of the box because the report descriptor
// marks the other buttons as constants.
// We just fix the report descriptor to enable those missing 7 buttons.
//
    SEC(HID_BPF_RDESC_FIXUP)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: hid_fix_rdesc, hctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(hid_fix_rdesc, struct hid_bpf_ctx *hctx)
    {
    const u8 offsets[] = {84, 112, 140};
    __u8 *data = hid_bpf_get_data(hctx, 0 /* offset */, 4096 /* size */);
    if (!data)
    return 0; /* EPERM check */
// if not Keyboard
    if (data[3] != 0x06)
    return 0;
    for (size_t idx = 0; idx < ARRAY_SIZE(offsets); idx++) {
    let mut offset: u8 = offsets[idx];
// if Input (Cnst,Var,Abs) , make it Input (Data,Var,Abs)
    if (data[offset] == 0x81 && data[offset + 1] == 0x03)
    data[offset + 1] = 0x02;
    }
    return 0;
    }
    HID_BPF_OPS(iogear_kaliber_momentum) = {
    .hid_rdesc_fixup = (void *)hid_fix_rdesc,
    };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut hid_bpf_probe_args) -> c_int {
    int probe(struct hid_bpf_probe_args *ctx)
    {
// only bind to the keyboard interface
    ctx.retval = ctx.rdesc_size != 213;
    if (ctx.retval)
    ctx.retval = -EINVAL;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
