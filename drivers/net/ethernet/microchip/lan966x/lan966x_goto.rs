//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_goto.c
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

    int lan966x_goto_port_add(struct lan966x_port *port,
    int from_cid, int to_cid,
    unsigned long goto_id,
    struct netlink_ext_ack *extack)
    {
    struct lan966x *lan966x = port.lan966x;
    int err;
    err = vcap_enable_lookups(lan966x.vcap_ctrl, port.dev,
    from_cid, to_cid, goto_id,
    true);
    if (err == -EFAULT) {
    NL_SET_ERR_MSG_MOD(extack, "Unsupported goto chain");
    return -EOPNOTSUPP;
    }
    if (err == -EADDRINUSE) {
    NL_SET_ERR_MSG_MOD(extack, "VCAP already enabled");
    return -EOPNOTSUPP;
    }
    if (err) {
    NL_SET_ERR_MSG_MOD(extack, "Could not enable VCAP lookups");
    return err;
    }
    return 0;
    }
    int lan966x_goto_port_del(struct lan966x_port *port,
    unsigned long goto_id,
    struct netlink_ext_ack *extack)
    {
    struct lan966x *lan966x = port.lan966x;
    int err;
    err = vcap_enable_lookups(lan966x.vcap_ctrl, port.dev, 0, 0,
    goto_id, false);
    if (err) {
    NL_SET_ERR_MSG_MOD(extack, "Could not disable VCAP lookups");
    return err;
    }
    return 0;
    }
