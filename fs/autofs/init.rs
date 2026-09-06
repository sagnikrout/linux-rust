//! Automatically rewritten from C to Rust
//! Source: fs/autofs/init.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 1997-1998 Transmeta Corporation -- All Rights Reserved
//

    struct file_system_type autofs_fs_type = {
    .owner		= THIS_MODULE,
    .name		= "autofs",
    .init_fs_context = autofs_init_fs_context,
    .parameters	= autofs_param_specs,
    .kill_sb	= autofs_kill_sb,
    };
    MODULE_ALIAS_FS("autofs");
    MODULE_ALIAS("autofs");
#[no_mangle]
unsafe extern "C" fn init_autofs_fs() -> int __init {
    static int __init init_autofs_fs(void)
    {
    int err;
    autofs_dev_ioctl_init();
    err = register_filesystem(&autofs_fs_type);
    if (err)
    autofs_dev_ioctl_exit();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn exit_autofs_fs() -> void __exit {
    static void __exit exit_autofs_fs(void)
    {
    autofs_dev_ioctl_exit();
    unregister_filesystem(&autofs_fs_type);
    }
    module_init(init_autofs_fs)
    module_exit(exit_autofs_fs)
    MODULE_DESCRIPTION("Kernel automounter support");
    MODULE_LICENSE("GPL");
