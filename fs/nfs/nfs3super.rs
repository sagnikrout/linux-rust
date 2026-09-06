//! Automatically rewritten from C to Rust
//! Source: fs/nfs/nfs3super.c
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
// Copyright (c) 2012 Netapp, Inc. All rights reserved.
//

    struct nfs_subversion nfs_v3 = {
    .owner = THIS_MODULE,
    .nfs_fs   = &nfs_fs_type,
    .rpc_vers = &nfs_version3,
    .rpc_ops  = &nfs_v3_clientops,
    .sops     = &nfs_sops,
    };
#[no_mangle]
unsafe extern "C" fn init_nfs_v3() -> int __init {
    static int __init init_nfs_v3(void)
    {
    register_nfs_version(&nfs_v3);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn exit_nfs_v3() -> void __exit {
    static void __exit exit_nfs_v3(void)
    {
    unregister_nfs_version(&nfs_v3);
    }
    MODULE_DESCRIPTION("NFSv3 client support");
    MODULE_LICENSE("GPL");
    module_init(init_nfs_v3);
    module_exit(exit_nfs_v3);
