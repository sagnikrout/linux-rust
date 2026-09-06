//! Automatically rewritten from C to Rust
//! Source: security/selinux/initcalls.c
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
// SELinux initcalls
//

//
// selinux_initcall - Perform the SELinux initcalls
//
// Used as a device initcall in the SELinux LSM definition.
//
#[no_mangle]
pub unsafe extern "C" fn selinux_initcall() -> int __init {
    int __init selinux_initcall(void)
    {
    let mut rc: c_int = 0, rc_tmp = 0;
    rc_tmp = init_sel_fs();
    if (!rc && rc_tmp)
    rc = rc_tmp;
    rc_tmp = sel_netport_init();
    if (!rc && rc_tmp)
    rc = rc_tmp;
    rc_tmp = sel_netnode_init();
    if (!rc && rc_tmp)
    rc = rc_tmp;
    rc_tmp = sel_netif_init();
    if (!rc && rc_tmp)
    rc = rc_tmp;
    rc_tmp = sel_netlink_init();
    if (!rc && rc_tmp)
    rc = rc_tmp;

    rc_tmp = sel_ib_pkey_init();
    if (!rc && rc_tmp)
    rc = rc_tmp;

    rc_tmp = selinux_nf_ip_init();
    if (!rc && rc_tmp)
    rc = rc_tmp;

    return rc;
    }
