//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/Logitech__SpaceNavigator.bpf.c
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
// Copyright (c) 2025 Curran Muhlberger
//

pub const VID_LOGITECH: c_uint = 0x046D;
pub const PID_SPACENAVIGATOR: c_uint = 0xC626;
    HID_BPF_CONFIG(
    HID_DEVICE(BUS_USB, HID_GROUP_ANY, VID_LOGITECH, PID_SPACENAVIGATOR)
    );
//
// The 3Dconnexion SpaceNavigator 3D Mouse is a multi-axis controller with 6
// axes (grouped as X,Y,Z and Rx,Ry,Rz).  Axis data is absolute, but the report
// descriptor erroneously declares it to be relative.  We fix the report
// descriptor to mark both axis collections as absolute.
//
// The kernel attempted to fix this in commit 24985cf68612 (HID: support
// Logitech/3DConnexion SpaceTraveler and SpaceNavigator), but the descriptor
// data offsets are incorrect for at least some SpaceNavigator units.
//
    SEC(HID_BPF_RDESC_FIXUP)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: hid_fix_rdesc, hctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(hid_fix_rdesc, struct hid_bpf_ctx *hctx)
    {
    __u8 *data = hid_bpf_get_data(hctx, 0 /* offset */, 4096 /* size */);
    if (!data)
    return 0; /* EPERM check */
// Offset of Input item in X,Y,Z and Rx,Ry,Rz collections for all known
// firmware variants.
// - 2009 model: X,Y,Z @ 32-33, Rx,Ry,Rz @ 49-50 (fixup originally
// applied in kernel)
// - 2016 model (size==228): X,Y,Z @ 36-37, Rx,Ry,Rz @ 53-54
//
// The descriptor size of the 2009 model is not known, and there is evidence
// for at least two other variants (with sizes 202 & 217) besides the 2016
// model, so we try all known offsets regardless of descriptor size.
//
    const u8 offsets[] = {32, 36, 49, 53};
    for (size_t idx = 0; idx < ARRAY_SIZE(offsets); idx++) {
    let mut offset: u8 = offsets[idx];
// if Input (Data,Var,Rel) , make it Input (Data,Var,Abs)
    if (data[offset] == 0x81 && data[offset + 1] == 0x06)
    data[offset + 1] = 0x02;
    }
    return 0;
    }
    HID_BPF_OPS(logitech_spacenavigator) = {
    .hid_rdesc_fixup = (void *)hid_fix_rdesc,
    };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut hid_bpf_probe_args) -> c_int {
    int probe(struct hid_bpf_probe_args *ctx)
    {
// Ensure report descriptor size matches one of the known variants.
    if (ctx.rdesc_size != 202 &&
    ctx.rdesc_size != 217 &&
    ctx.rdesc_size != 228) {
    ctx.retval = -EINVAL;
    return 0;
    }
// Check whether the kernel has already applied the fix.
    if ((ctx.rdesc[32] == 0x81 && ctx.rdesc[33] == 0x02 &&
    ctx.rdesc[49] == 0x81 && ctx.rdesc[50] == 0x02) ||
    (ctx.rdesc[36] == 0x81 && ctx.rdesc[37] == 0x02 &&
    ctx.rdesc[53] == 0x81 && ctx.rdesc[54] == 0x02))
    ctx.retval = -EINVAL;
    else
    ctx.retval = 0;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
