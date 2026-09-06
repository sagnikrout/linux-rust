//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/amdkfd/kfd_module.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2014-2022 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

#[no_mangle]
unsafe extern "C" fn kfd_init() -> c_int {
    static int kfd_init(void)
    {
    int err;
// Verify module parameters
    if ((sched_policy < KFD_SCHED_POLICY_HWS) ||
    (sched_policy > KFD_SCHED_POLICY_NO_HWS)) {
    pr_err("sched_policy has invalid value\n");
    return -EINVAL;
    }
// Verify module parameters
    if ((max_num_of_queues_per_device < 1) ||
    (max_num_of_queues_per_device >
    KFD_MAX_NUM_OF_QUEUES_PER_DEVICE)) {
    pr_err("max_num_of_queues_per_device must be between 1 to KFD_MAX_NUM_OF_QUEUES_PER_DEVICE\n");
    return -EINVAL;
    }
    err = kfd_chardev_init();
    if (err < 0)
    goto err_ioctl;
    err = kfd_topology_init();
    if (err < 0)
    goto err_topology;
    err = kfd_process_create_wq();
    if (err < 0)
    goto err_create_wq;
// Ignore the return value, so that we can continue
// to init the KFD, even if procfs isn't craated
//
    kfd_procfs_init();
    kfd_debugfs_init();
    return 0;
    err_create_wq:
    kfd_topology_shutdown();
    err_topology:
    kfd_chardev_exit();
    err_ioctl:
    pr_err("KFD is disabled due to module initialization failure\n");
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kfd_exit() {
    static void kfd_exit(void)
    {
    kfd_cleanup_processes();
    kfd_process_destroy_wq();
    kfd_debugfs_fini();
    kfd_procfs_shutdown();
    kfd_topology_shutdown();
    kfd_chardev_exit();
    }
#[no_mangle]
pub unsafe extern "C" fn kgd2kfd_init() -> c_int {
    int kgd2kfd_init(void)
    {
    return kfd_init();
    }
#[no_mangle]
pub unsafe extern "C" fn kgd2kfd_exit() {
    void kgd2kfd_exit(void)
    {
    kfd_exit();
    }
