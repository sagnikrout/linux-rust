//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/HP__Elite-Presenter.bpf.c
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

pub const VID_HP: c_uint = 0x03F0;
pub const PID_ELITE_PRESENTER: c_uint = 0x464A;
    HID_BPF_CONFIG(
    HID_DEVICE(BUS_BLUETOOTH, HID_GROUP_GENERIC, VID_HP, PID_ELITE_PRESENTER)
    );
//
// Already fixed as of commit 0db117359e47 ("HID: add quirk for 03f0:464a
// HP Elite Presenter Mouse") in the kernel, but this is a slightly better
// fix.
//
// The HP Elite Presenter Mouse HID Record Descriptor shows
// two mice (Report ID 0x1 and 0x2), one keypad (Report ID 0x5),
// two Consumer Controls (Report IDs 0x6 and 0x3).
// Prior to these fixes it registers one mouse, one keypad
// and one Consumer Control, and it was usable only as a
// digital laser pointer (one of the two mouses).
// We replace the second mouse collection with a pointer collection,
// allowing to use the device both as a mouse and a digital laser
// pointer.
//
    SEC(HID_BPF_RDESC_FIXUP)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: hid_fix_rdesc, hctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(hid_fix_rdesc, struct hid_bpf_ctx *hctx)
    {
    __u8 *data = hid_bpf_get_data(hctx, 0 /* offset */, 4096 /* size */);
    if (!data)
    return 0; /* EPERM check */
// replace application mouse by application pointer on the second collection
    if (data[79] == 0x02)
    data[79] = 0x01;
    return 0;
    }
    HID_BPF_OPS(hp_elite_presenter) = {
    .hid_rdesc_fixup = (void *)hid_fix_rdesc,
    };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut hid_bpf_probe_args) -> c_int {
    int probe(struct hid_bpf_probe_args *ctx)
    {
    ctx.retval = ctx.rdesc_size != 264;
    if (ctx.retval)
    ctx.retval = -EINVAL;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
