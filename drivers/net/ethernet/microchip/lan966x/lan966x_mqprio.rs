//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_mqprio.c
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


// SPDX-License-Identifier: GPL-2.0+

#[no_mangle]
pub unsafe extern "C" fn lan966x_mqprio_add(port: *mut lan966x_port, num_tc: u8) -> c_int {
    int lan966x_mqprio_add(struct lan966x_port *port, u8 num_tc)
    {
    u8 i;
    if (num_tc != NUM_PRIO_QUEUES) {
    netdev_err(port.dev, "Only %d traffic classes supported\n",
    NUM_PRIO_QUEUES);
    return -EINVAL;
    }
    netdev_set_num_tc(port.dev, num_tc);
    for (i = 0; i < num_tc; ++i)
    netdev_set_tc_queue(port.dev, i, 1, i);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lan966x_mqprio_del(port: *mut lan966x_port) -> c_int {
    int lan966x_mqprio_del(struct lan966x_port *port)
    {
    netdev_reset_tc(port.dev);
    return 0;
    }
