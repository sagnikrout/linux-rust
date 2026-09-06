//! Automatically rewritten from C to Rust
//! Source: drivers/dma/idxd/defaults.c
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
// Copyright(c) 2023 Intel Corporation. All rights rsvd.

#[no_mangle]
pub unsafe extern "C" fn idxd_load_iaa_device_defaults(idxd: *mut idxd_device) -> c_int {
    int idxd_load_iaa_device_defaults(struct idxd_device *idxd)
    {
    struct idxd_engine *engine;
    struct idxd_group *group;
    struct idxd_wq *wq;
    int i;
    if (!test_bit(IDXD_FLAG_CONFIGURABLE, &idxd.flags))
    return 0;
    wq = idxd.wqs[0];
    if (wq.state != IDXD_WQ_DISABLED)
    return -EPERM;
// set mode to "dedicated"
    set_bit(WQ_FLAG_DEDICATED, &wq.flags);
    wq.threshold = 0;
// only setting up 1 wq, so give it all the wq space
    wq.size = idxd.max_wq_size;
// set priority to 10
    wq.priority = 10;
// set type to "kernel"
    wq.type = IDXD_WQT_KERNEL;
// set wq group to 0
    group = idxd.groups[0];
    wq.group = group;
    group.num_wqs++;
// set name to "iaa_crypto"
    strscpy_pad(wq.name, "iaa_crypto");
// set driver_name to "crypto"
    strscpy_pad(wq.driver_name, "crypto");
// assign all engines to group 0
    for (i = 0; i < idxd.max_engines; i++) {
    engine = idxd.engines[i];
    engine.group = group;
    group.num_engines++;
    }
    return 0;
    }
