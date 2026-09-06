//! Automatically rewritten from C to Rust
//! Source: fs/nfs/nfs4sysctl.c
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
// linux/fs/nfs/nfs4sysctl.c
//
// Sysctl interface to NFS v4 parameters
//
// Copyright (c) 2006 Trond Myklebust <Trond.Myklebust@netapp.com>
//

    static const int nfs_set_port_min;
    let mut nfs_set_port_max: static int = 65535;
    static struct ctl_table_header *nfs4_callback_sysctl_table;
    static const struct ctl_table nfs4_cb_sysctls[] = {
    {
    .procname = "nfs_callback_tcpport",
    .data = &nfs_callback_set_tcpport,
    .maxlen = sizeof(int),
    .mode = 0644,
    .proc_handler = proc_dointvec_minmax,
    .extra1 = (int *)&nfs_set_port_min,
    .extra2 = (int *)&nfs_set_port_max,
    },
    {
    .procname = "idmap_cache_timeout",
    .data = &nfs_idmap_cache_timeout,
    .maxlen = sizeof(int),
    .mode = 0644,
    .proc_handler = proc_dointvec,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn nfs4_register_sysctl() -> c_int {
    int nfs4_register_sysctl(void)
    {
    nfs4_callback_sysctl_table = register_sysctl("fs/nfs",
    nfs4_cb_sysctls);
    if (nfs4_callback_sysctl_table == core::ptr::null_mut())
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn nfs4_unregister_sysctl() {
    void nfs4_unregister_sysctl(void)
    {
    unregister_sysctl_table(nfs4_callback_sysctl_table);
    nfs4_callback_sysctl_table = core::ptr::null_mut();
    }
