//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/mpc5xxx_clocks.c
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


// SPDX-License-Identifier: GPL-2.0

//
// mpc5xxx_fwnode_get_bus_frequency - Find the bus frequency for a firmware node
// @fwnode:	firmware node
//
// Returns bus frequency (IPS on MPC512x, IPB on MPC52xx),
// or 0 if the bus frequency cannot be found.
//
#[no_mangle]
pub unsafe extern "C" fn mpc5xxx_fwnode_get_bus_frequency(fwnode: *mut fwnode_handle) -> c_ulong {
    unsigned long mpc5xxx_fwnode_get_bus_frequency(struct fwnode_handle *fwnode)
    {
    struct fwnode_handle *parent;
    u32 bus_freq;
    int ret;
    ret = fwnode_property_read_u32(fwnode, "bus-frequency", &bus_freq);
    if (!ret)
    return bus_freq;
    fwnode_for_each_parent_node(fwnode, parent) {
    ret = fwnode_property_read_u32(parent, "bus-frequency", &bus_freq);
    if (!ret) {
    fwnode_handle_put(parent);
    return bus_freq;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL(mpc5xxx_fwnode_get_bus_frequency);
