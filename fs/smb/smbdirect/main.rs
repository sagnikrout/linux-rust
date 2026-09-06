//! Automatically rewritten from C to Rust
//! Source: fs/smb/smbdirect/main.c
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
// Copyright (c) 2025, Stefan Metzmacher
//

    struct smbdirect_module_state smbdirect_globals = {
    .mutex = __MUTEX_INITIALIZER(smbdirect_globals.mutex),
    };
#[no_mangle]
unsafe extern "C" fn smbdirect_module_init() -> __init int {
    static __init int smbdirect_module_init(void)
    {
    let mut ret: c_int = -ENOMEM;
    pr_notice("subsystem loading...\n");
    mutex_lock(&smbdirect_globals.mutex);
    smbdirect_globals.workqueues.accept = alloc_workqueue("smbdirect-accept",
    WQ_SYSFS |
    WQ_PERCPU |
    WQ_POWER_EFFICIENT,
    0);
    if (smbdirect_globals.workqueues.accept == core::ptr::null_mut())
    goto alloc_accept_wq_failed;
    smbdirect_globals.workqueues.connect = alloc_workqueue("smbdirect-connect",
    WQ_SYSFS |
    WQ_PERCPU |
    WQ_POWER_EFFICIENT,
    0);
    if (smbdirect_globals.workqueues.connect == core::ptr::null_mut())
    goto alloc_connect_wq_failed;
    smbdirect_globals.workqueues.idle = alloc_workqueue("smbdirect-idle",
    WQ_SYSFS |
    WQ_PERCPU |
    WQ_POWER_EFFICIENT,
    0);
    if (smbdirect_globals.workqueues.idle == core::ptr::null_mut())
    goto alloc_idle_wq_failed;
    smbdirect_globals.workqueues.refill = alloc_workqueue("smbdirect-refill",
    WQ_HIGHPRI |
    WQ_SYSFS |
    WQ_PERCPU |
    WQ_POWER_EFFICIENT,
    0);
    if (smbdirect_globals.workqueues.refill == core::ptr::null_mut())
    goto alloc_refill_wq_failed;
    smbdirect_globals.workqueues.immediate = alloc_workqueue("smbdirect-immediate",
    WQ_HIGHPRI |
    WQ_SYSFS |
    WQ_PERCPU |
    WQ_POWER_EFFICIENT,
    0);
    if (smbdirect_globals.workqueues.immediate == core::ptr::null_mut())
    goto alloc_immediate_wq_failed;
    smbdirect_globals.workqueues.cleanup = alloc_workqueue("smbdirect-cleanup",
    WQ_MEM_RECLAIM |
    WQ_HIGHPRI |
    WQ_SYSFS |
    WQ_PERCPU |
    WQ_POWER_EFFICIENT,
    0);
    if (smbdirect_globals.workqueues.cleanup == core::ptr::null_mut())
    goto alloc_cleanup_wq_failed;
    ret = smbdirect_devices_init();
    if (ret)
    goto devices_init_failed;
    mutex_unlock(&smbdirect_globals.mutex);
    pr_notice("subsystem loaded\n");
    return 0;
    devices_init_failed:
    destroy_workqueue(smbdirect_globals.workqueues.cleanup);
    alloc_cleanup_wq_failed:
    destroy_workqueue(smbdirect_globals.workqueues.immediate);
    alloc_immediate_wq_failed:
    destroy_workqueue(smbdirect_globals.workqueues.refill);
    alloc_refill_wq_failed:
    destroy_workqueue(smbdirect_globals.workqueues.idle);
    alloc_idle_wq_failed:
    destroy_workqueue(smbdirect_globals.workqueues.connect);
    alloc_connect_wq_failed:
    destroy_workqueue(smbdirect_globals.workqueues.accept);
    alloc_accept_wq_failed:
    mutex_unlock(&smbdirect_globals.mutex);
    pr_crit("failed to loaded: %d (%1pe)\n",
    ret, SMBDIRECT_DEBUG_ERR_PTR(ret));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn smbdirect_module_exit() -> __exit void {
    static __exit void smbdirect_module_exit(void)
    {
    pr_notice("subsystem unloading...\n");
    mutex_lock(&smbdirect_globals.mutex);
    smbdirect_devices_exit();
    destroy_workqueue(smbdirect_globals.workqueues.accept);
    destroy_workqueue(smbdirect_globals.workqueues.connect);
    destroy_workqueue(smbdirect_globals.workqueues.idle);
    destroy_workqueue(smbdirect_globals.workqueues.refill);
    destroy_workqueue(smbdirect_globals.workqueues.immediate);
    destroy_workqueue(smbdirect_globals.workqueues.cleanup);
    mutex_unlock(&smbdirect_globals.mutex);
    pr_notice("subsystem unloaded\n");
    }
    module_init(smbdirect_module_init);
    module_exit(smbdirect_module_exit);
    MODULE_DESCRIPTION("smbdirect subsystem");
    MODULE_LICENSE("GPL");
