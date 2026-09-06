//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/Trust__Philips-SPK6327.bpf.c
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
// Fix for Trust Philips SPK6327 (145f:024b)
// Modifier keys report as Array (0x00) instead of Variable (0x02)
// causing LCtrl, LAlt, Super etc. to all act as LShift
//

pub const VID_TRUST: c_uint = 0x145F;
pub const PID_SPK6327: c_uint = 0x024B;
    HID_BPF_CONFIG(
    HID_DEVICE(BUS_USB, HID_GROUP_GENERIC, VID_TRUST, PID_SPK6327)
    );
    SEC(HID_BPF_RDESC_FIXUP)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: hid_fix_rdesc, hctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(hid_fix_rdesc, struct hid_bpf_ctx *hctx)
    {
    __u8 *data = hid_bpf_get_data(hctx, 0, 4096);
    if (!data)
    return 0;
// Fix modifier keys: Input Array (0x00) -> Input Variable (0x02)
    if (data[101] == 0x00)
    data[101] = 0x02;
    return 0;
    }
    HID_BPF_OPS(trust_spk6327) = {
    .hid_rdesc_fixup = (void *)hid_fix_rdesc,
    };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut hid_bpf_probe_args) -> c_int {
    int probe(struct hid_bpf_probe_args *ctx)
    {
// Only apply to interface 1 (169 bytes) not interface 0 (62 bytes)
    if (ctx.rdesc_size == 169)
    ctx.retval = 0;
    else
    ctx.retval = -EINVAL;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
