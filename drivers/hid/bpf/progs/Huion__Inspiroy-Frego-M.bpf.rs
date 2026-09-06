//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/Huion__Inspiroy-Frego-M.bpf.c
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

//
// Huion Inspiroy Frego M Pen Tablet
// Model L610
// 256c:8251 (Bluetooth)
// 256c:2012 (USB)
//
pub const VID_HUION: c_uint = 0x256C;
pub const PID_INSPIROY_FREGO_M: c_uint = 0x8251;
pub const PID_L610: c_uint = 0x2012;
pub const PEN_RDESC_SIZE: c_int = 125;
pub const SECONDARY_SWITCH_OFFSET: c_int = 17;
    HID_BPF_CONFIG(
    HID_DEVICE(BUS_BLUETOOTH, HID_GROUP_GENERIC, VID_HUION, PID_INSPIROY_FREGO_M),
    HID_DEVICE(BUS_USB, HID_GROUP_GENERIC, VID_HUION, PID_L610)
    );
//
// The pen descriptor reports the second side button as Secondary Tip Switch
// instead of Secondary Barrel Switch.
//
// Relevant part of the original pen report descriptor:
//
// 0x09, 0x42,       // Usage (Tip Switch)                  12
// 0x09, 0x44,       // Usage (Barrel Switch)               14
// 0x09, 0x43,       // Usage (Secondary Tip Switch)        16 <- change to 0x5a
// 0x09, 0x3c,       // Usage (Invert)                      18
// 0x09, 0x45,       // Usage (Eraser)                      20
// 0x15, 0x00,       // Logical Minimum (0)                 22
// 0x25, 0x01,       // Logical Maximum (1)                 24
//
    SEC(HID_BPF_RDESC_FIXUP)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: fix_secondary_barrel_rdesc, hctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(fix_secondary_barrel_rdesc, struct hid_bpf_ctx *hctx)
    {
    __u8 *data = hid_bpf_get_data(hctx, 0 /* offset */, HID_MAX_DESCRIPTOR_SIZE /* size */);
    if (!data)
    return 0; /* EPERM check */
    if (hctx.size != PEN_RDESC_SIZE)
    return 0;
    if (data[0] != 0x05 || data[1] != 0x0d || /* Usage Page (Digitizers) */
    data[2] != 0x09 || data[3] != 0x02 || /* Usage (Pen) */
    data[16] != 0x09 ||
    data[SECONDARY_SWITCH_OFFSET] != 0x43) /* Secondary Tip Switch */
    return 0;
    data[SECONDARY_SWITCH_OFFSET] = 0x5a;
    return 0;
    }
    HID_BPF_OPS(fix_secondary_barrel) = {
    .hid_rdesc_fixup = (void *)fix_secondary_barrel_rdesc,
    };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut hid_bpf_probe_args) -> c_int {
    int probe(struct hid_bpf_probe_args *ctx)
    {
    ctx.retval = ctx.rdesc_size != PEN_RDESC_SIZE;
    if (ctx.retval) {
    ctx.retval = -EINVAL;
    return 0;
    }
    if (ctx.rdesc[0] != 0x05 || ctx.rdesc[1] != 0x0d || /* Usage Page (Digitizers) */
    ctx.rdesc[2] != 0x09 || ctx.rdesc[3] != 0x02 || /* Usage (Pen) */
    ctx.rdesc[16] != 0x09 ||
    ctx.rdesc[SECONDARY_SWITCH_OFFSET] != 0x43) { /* Secondary Tip Switch */
    ctx.retval = -EINVAL;
    return 0;
    }
    ctx.retval = 0;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
