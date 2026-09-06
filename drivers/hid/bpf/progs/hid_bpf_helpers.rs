//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/bpf/progs/hid_bpf_helpers.h
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
// Copyright (c) 2022 Benjamin Tissoires
//

// Compiler attributes

// bpf_wq implementation
pub const HID_MAX_DESCRIPTOR_SIZE: c_int = 4096;

//
// Use: _cleanup_(somefunction) struct foo *bar;
//

//
// Use: _release_(foo) *bar;
//
// This requires foo_releasep() to be present, use DEFINE_RELEASE_CLEANUP_FUNC.
//

//
// Define a cleanup function for the struct type foo with a matching
// foo_release(). Use:
// DEFINE_RELEASE_CLEANUP_FUNC(foo)
// _unref_(foo) struct foo *bar;
//

// for being able to have a cleanup function

//
// Kernel-style guard macros adapted for BPF
// Based on include/linux/cleanup.h from the Linux kernel
//
// These provide automatic lock/unlock using __attribute__((cleanup))
// similar to how _release_() works for contexts.
//
// DEFINE_GUARD(name, type, lock, unlock):
// Define a guard for automatic lock/unlock using the same pattern as _release_()
// @name: identifier for the guard (e.g., bpf_spin)
// @type: lock variable type (e.g., struct bpf_spin_lock)
// @lock: lock function name (e.g., bpf_spin_lock)
// @unlock: unlock function name (e.g., bpf_spin_unlock)
//
// guard(name):
// Declare and lock in one statement - lock held until end of scope
//
// Example:
// DEFINE_GUARD(bpf_spin, struct bpf_spin_lock, bpf_spin_lock, bpf_spin_unlock)
//
// void foo(struct bpf_spin_lock *lock) {
// guard(bpf_spin)(lock);
// // lock held until end of scope
// }
//
// Guard helper struct - stores lock pointer for cleanup

// Define BPF spinlock guard
// extracted from <linux/input.h>
pub const BUS_ANY: c_uint = 0x00;
pub const BUS_PCI: c_uint = 0x01;
pub const BUS_ISAPNP: c_uint = 0x02;
pub const BUS_USB: c_uint = 0x03;
pub const BUS_HIL: c_uint = 0x04;
pub const BUS_BLUETOOTH: c_uint = 0x05;
pub const BUS_VIRTUAL: c_uint = 0x06;
pub const BUS_ISA: c_uint = 0x10;
pub const BUS_I8042: c_uint = 0x11;
pub const BUS_XTKBD: c_uint = 0x12;
pub const BUS_RS232: c_uint = 0x13;
pub const BUS_GAMEPORT: c_uint = 0x14;
pub const BUS_PARPORT: c_uint = 0x15;
pub const BUS_AMIGA: c_uint = 0x16;
pub const BUS_ADB: c_uint = 0x17;
pub const BUS_I2C: c_uint = 0x18;
pub const BUS_HOST: c_uint = 0x19;
pub const BUS_GSC: c_uint = 0x1A;
pub const BUS_ATARI: c_uint = 0x1B;
pub const BUS_SPI: c_uint = 0x1C;
pub const BUS_RMI: c_uint = 0x1D;
pub const BUS_CEC: c_uint = 0x1E;
pub const BUS_INTEL_ISHTP: c_uint = 0x1F;
pub const BUS_AMD_SFH: c_uint = 0x20;
// extracted from <linux/hid.h>
pub const HID_GROUP_ANY: c_uint = 0x0000;
pub const HID_GROUP_GENERIC: c_uint = 0x0001;
pub const HID_GROUP_MULTITOUCH: c_uint = 0x0002;
pub const HID_GROUP_SENSOR_HUB: c_uint = 0x0003;
pub const HID_GROUP_MULTITOUCH_WIN_8: c_uint = 0x0004;
pub const HID_GROUP_RMI: c_uint = 0x0100;
pub const HID_GROUP_WACOM: c_uint = 0x0101;
pub const HID_GROUP_LOGITECH_DJ_DEVICE: c_uint = 0x0102;
pub const HID_GROUP_STEAM: c_uint = 0x0103;
pub const HID_GROUP_LOGITECH_27MHZ_DEVICE: c_uint = 0x0104;
pub const HID_GROUP_VIVALDI: c_uint = 0x0105;
// include/linux/mod_devicetable.h defines as (~0), but that gives us negative size arrays
pub const HID_VID_ANY: c_uint = 0x0000;
pub const HID_PID_ANY: c_uint = 0x0000;

// Helper macro to convert (foo, __LINE__)  into foo134 so we can use __LINE__ for
// field/variable names
//

// Macro magic:
// __uint(foo, 123) creates a int (*foo)[1234]
//
// We use that macro to declare an anonymous struct with several
// fields, each is the declaration of an pointer to an array of size
// bus/group/vid/pid. (Because it's a pointer to such an array, actual storage
// would be sizeof(pointer) rather than sizeof(array). Not that we ever
// instantiate it anyway).
//
// This is only used for BTF introspection, we can later check "what size
// is the bus array" in the introspection data and thus extract the bus ID
// again.
//
// And we use the __LINE__ to give each of our structs a unique name so the
// BPF program writer doesn't have to.
//
// $ bpftool btf dump file target/bpf/HP_Elite_Presenter.bpf.o
// shows the inspection data, start by searching for .hid_bpf_config
// and working backwards from that (each entry references the type_id of the
// content).
//

// Macro magic below is to make HID_BPF_CONFIG() look like a function call that
// we can pass multiple HID_DEVICE() invocations in.
//
// For up to 16 arguments, HID_BPF_CONFIG(one, two) resolves to
//
// union {
// HID_DEVICE(...);
// } _device_ids SEC(".hid_bpf_config")
//
// Returns the number of macro arguments, this expands
// NARGS(a, b, c) to NTH_ARG(a, b, c, 15, 14, 13, .... 4, 3, 2, 1).
// NTH_ARG always returns the 16th argument which in our case is 3.
//
// If we want more than 16 values _COUNTDOWN and _NTH_ARG both need to be
// updated.
//

// Add to this if we need more than 16 args

// Return the 16 argument passed in. See _NARGS above for usage. Note this is
// 1-indexed.
//

// Turns EXPAND(_ARG, a, b, c) into _ARG3(a, b, c)

// And now define all the ARG macros for each number of args we want to accept

// Equivalency macros for bpf_htons and friends which are
// Big Endian only - HID needs little endian so these are the
// corresponding macros for that. See bpf/bpf_endian.h
//

//
// The following macros are helpers for exporting udev properties:
//
// EXPORT_UDEV_PROP(name, len) generates:
// - a map with a single element UDEV_PROP_##name, of size len
// - a const global declaration of that len: SIZEOF_##name
//
// udev_prop_ptr(name) retrieves the data pointer behind the map.
//
// UDEV_PROP_SPRINTF(name, fmt, ...) writes data into the udev property.
//
// Can be used as such:
// EXPORT_UDEV_PROP(HID_FOO, 32);
//
// SEC("syscall")
// int probe(struct hid_bpf_probe_args *ctx)
// {
// const char *foo = "foo";
// UDEV_PROP_SPRINTF(HID_FOO, "%s", foo);
//
// return 0;
// }
//

// Fast path for byte-aligned standard-sized reads
// 8-bit aligned read
// 16-bit aligned read - use separate variables for verifier
// 32-bit aligned read - use separate variables for verifier
// General case: bit manipulation for unaligned or non-standard sizes

// Base macro for iterating over HID arrays with bounds checking.
// Follows the bpf_for pattern from libbpf.
//

// initialize and define destructor */                                 \
// ___p pointer is necessary to call bpf_iter_num_new() *once* */      \
// ___p __attribute__((unused)) = (                  \
// always initialize iterator; if bounds fail, iterate 0 times */ \
// workaround for Clang bug */                                 \
// iteration step */                                           \
// termination and bounds check, assign var */                 \
// Iterate over input reports in a descriptor

// Iterate over feature reports in a descriptor

// Iterate over output reports in a descriptor

// Iterate over fields in a report

// Iterate over collections in a field

