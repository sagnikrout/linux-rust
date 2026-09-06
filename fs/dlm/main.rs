//! Automatically rewritten from C to Rust
//! Source: fs/dlm/main.c
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
// Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
// Copyright (C) 2004-2007 Red Hat, Inc.  All rights reserved.
//

// Macro flag: #define CREATE_TRACE_POINTS

    struct workqueue_struct *dlm_wq;
#[no_mangle]
unsafe extern "C" fn init_dlm() -> int __init {
    static int __init init_dlm(void)
    {
    int error;
    error = dlm_memory_init();
    if (error)
    goto out;
    dlm_midcomms_init();
    error = dlm_lockspace_init();
    if (error)
    goto out_mem;
    error = dlm_config_init();
    if (error)
    goto out_lockspace;
    dlm_register_debugfs();
    error = dlm_user_init();
    if (error)
    goto out_debug;
    error = dlm_plock_init();
    if (error)
    goto out_user;
    dlm_wq = alloc_workqueue("dlm_wq", WQ_PERCPU, 0);
    if (!dlm_wq) {
    error = -ENOMEM;
    goto out_plock;
    }
    printk("DLM installed\n");
    return 0;
    out_plock:
    dlm_plock_exit();
    out_user:
    dlm_user_exit();
    out_debug:
    dlm_unregister_debugfs();
    dlm_config_exit();
    out_lockspace:
    dlm_lockspace_exit();
    out_mem:
    dlm_midcomms_exit();
    dlm_memory_exit();
    out:
    return error;
    }
#[no_mangle]
unsafe extern "C" fn exit_dlm() -> void __exit {
    static void __exit exit_dlm(void)
    {
// be sure every pending work e.g. freeing is done
    destroy_workqueue(dlm_wq);
    dlm_plock_exit();
    dlm_user_exit();
    dlm_config_exit();
    dlm_lockspace_exit();
    dlm_midcomms_exit();
    dlm_unregister_debugfs();
    dlm_memory_exit();
    }
    module_init(init_dlm);
    module_exit(exit_dlm);
    MODULE_DESCRIPTION("Distributed Lock Manager");
    MODULE_AUTHOR("Red Hat, Inc.");
    MODULE_LICENSE("GPL");
    EXPORT_SYMBOL_GPL(dlm_new_lockspace);
    EXPORT_SYMBOL_GPL(dlm_release_lockspace);
    EXPORT_SYMBOL_GPL(dlm_lock);
    EXPORT_SYMBOL_GPL(dlm_unlock);
