//! Automatically rewritten from C to Rust
//! Source: drivers/hid/bpf/progs/Microsoft__Xbox-Elite-2.bpf.c
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
// Copyright (c) 2024 Benjamin Tissoires
//

pub const VID_MICROSOFT: c_uint = 0x045e;
pub const PID_XBOX_ELITE_2: c_uint = 0x0b22;
    HID_BPF_CONFIG(
    HID_DEVICE(BUS_BLUETOOTH, HID_GROUP_GENERIC, VID_MICROSOFT, PID_XBOX_ELITE_2)
    );
//
// When using the Xbox Wireless Controller Elite 2 over Bluetooth,
// the device exports the paddles on the back of the device as a single
// bitfield value of usage "Assign Selection".
//
// The kernel doesn't process the paddles usage properly and reports KEY_UNKNOWN.
//
// SDL doesn't know how to interpret KEY_UNKNOWN and thus ignores the paddles.
//
// Given that over USB the kernel uses BTN_TRIGGER_HAPPY[5-8], we
// can tweak the report descriptor to make the kernel interpret it properly:
// - We need an application collection of gamepad (so we have to close the current
// Consumer Control one)
// - We need to change the usage to be buttons from 0x15 to 0x18
//
pub const OFFSET_ASSIGN_SELECTION: c_int = 211;
pub const ORIGINAL_RDESC_SIZE: c_int = 464;
    const __u8 rdesc_assign_selection[] = {
    0x0a, 0x99, 0x00,              //   Usage (Media Select Security)     211
    0x15, 0x00,                    //   Logical Minimum (0)               214
    0x26, 0xff, 0x00,              //   Logical Maximum (255)             216
    0x95, 0x01,                    //   Report Count (1)                  219
    0x75, 0x04,                    //   Report Size (4)                   221
    0x81, 0x02,                    //   Input (Data,Var,Abs)              223
    0x15, 0x00,                    //   Logical Minimum (0)               225
    0x25, 0x00,                    //   Logical Maximum (0)               227
    0x95, 0x01,                    //   Report Count (1)                  229
    0x75, 0x04,                    //   Report Size (4)                   231
    0x81, 0x03,                    //   Input (Cnst,Var,Abs)              233
    0x0a, 0x81, 0x00,              //   Usage (Assign Selection)          235
    0x15, 0x00,                    //   Logical Minimum (0)               238
    0x26, 0xff, 0x00,              //   Logical Maximum (255)             240
    0x95, 0x01,                    //   Report Count (1)                  243
    0x75, 0x04,                    //   Report Size (4)                   245
    0x81, 0x02,                    //   Input (Data,Var,Abs)              247
    };
//
// we replace the above report descriptor extract
// with the one below.
// To make things equal in size, we take out a larger
// portion than just the "Assign Selection" range, because
// we need to insert a new application collection to force
// the kernel to use BTN_TRIGGER_HAPPY[4-7].
//
    const __u8 fixed_rdesc_assign_selection[] = {
    0x0a, 0x99, 0x00,              //   Usage (Media Select Security)     211
    0x15, 0x00,                    //   Logical Minimum (0)               214
    0x26, 0xff, 0x00,              //   Logical Maximum (255)             216
    0x95, 0x01,                    //   Report Count (1)                  219
    0x75, 0x04,                    //   Report Size (4)                   221
    0x81, 0x02,                    //   Input (Data,Var,Abs)              223
// 0x15, 0x00, */              //   Logical Minimum (0)               ignored
    0x25, 0x01,                    //   Logical Maximum (1)               225
    0x95, 0x04,                    //   Report Count (4)                  227
    0x75, 0x01,                    //   Report Size (1)                   229
    0x81, 0x03,                    //   Input (Cnst,Var,Abs)              231
    0xc0,                          //  End Collection                     233
    0x05, 0x01,                    //  Usage Page (Generic Desktop)       234
    0x0a, 0x05, 0x00,              //  Usage (Game Pad)                   236
    0xa1, 0x01,                    //  Collection (Application)           239
    0x05, 0x09,                    //   Usage Page (Button)               241
    0x19, 0x15,                    //   Usage Minimum (21)                243
    0x29, 0x18,                    //   Usage Maximum (24)                245
// 0x15, 0x00, */              //  Logical Minimum (0)                ignored
// 0x25, 0x01, */              //  Logical Maximum (1)                ignored
// 0x95, 0x01, */              //  Report Size (1)                    ignored
// 0x75, 0x04, */              //  Report Count (4)                   ignored
    0x81, 0x02,                    //   Input (Data,Var,Abs)              247
    };
    _Static_assert(sizeof(rdesc_assign_selection) == sizeof(fixed_rdesc_assign_selection),
    "Rdesc and fixed rdesc of different size");
    _Static_assert(sizeof(rdesc_assign_selection) + OFFSET_ASSIGN_SELECTION < ORIGINAL_RDESC_SIZE,
    "Rdesc at given offset is too big");
    SEC(HID_BPF_RDESC_FIXUP)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: hid_fix_rdesc, hctx: *mut hid_bpf_ctx) -> c_int {
    int BPF_PROG(hid_fix_rdesc, struct hid_bpf_ctx *hctx)
    {
    __u8 *data = hid_bpf_get_data(hctx, 0 /* offset */, 4096 /* size */);
    if (!data)
    return 0; /* EPERM check */
// Check that the device is compatible
    if (__builtin_memcmp(data + OFFSET_ASSIGN_SELECTION,
    rdesc_assign_selection,
    sizeof(rdesc_assign_selection)))
    return 0;
    __builtin_memcpy(data + OFFSET_ASSIGN_SELECTION,
    fixed_rdesc_assign_selection,
    sizeof(fixed_rdesc_assign_selection));
    return 0;
    }
    HID_BPF_OPS(xbox_elite_2) = {
    .hid_rdesc_fixup = (void *)hid_fix_rdesc,
    };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn probe(ctx: *mut hid_bpf_probe_args) -> c_int {
    int probe(struct hid_bpf_probe_args *ctx)
    {
// only bind to the keyboard interface
    ctx.retval = ctx.rdesc_size != ORIGINAL_RDESC_SIZE;
    if (ctx.retval)
    ctx.retval = -EINVAL;
    if (__builtin_memcmp(ctx.rdesc + OFFSET_ASSIGN_SELECTION,
    rdesc_assign_selection,
    sizeof(rdesc_assign_selection)))
    ctx.retval = -EINVAL;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
