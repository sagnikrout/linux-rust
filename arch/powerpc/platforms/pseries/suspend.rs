//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/suspend.c
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
// Copyright (C) 2010 Brian King IBM Corporation
//

    static struct device suspend_dev;
//
// pseries_suspend_begin - First phase of hibernation
//
// Check to ensure we are in a valid state to hibernate
//
// Return value:
// 0 on success / other on failure
//
#[no_mangle]
unsafe extern "C" fn pseries_suspend_begin(stream_id: u64) -> c_int {
    static int pseries_suspend_begin(u64 stream_id)
    {
    long vasi_state, rc;
    unsigned long retbuf[PLPAR_HCALL_BUFSIZE];
// Make sure the state is valid
    rc = plpar_hcall(H_VASI_STATE, retbuf, stream_id);
    vasi_state = retbuf[0];
    if (rc) {
    pr_err("pseries_suspend_begin: vasi_state returned %ld\n",rc);
    return rc;
    } else if (vasi_state == H_VASI_ENABLED) {
    return -EAGAIN;
    } else if (vasi_state != H_VASI_SUSPENDING) {
    pr_err("pseries_suspend_begin: vasi_state returned state %ld\n",
    vasi_state);
    return -EIO;
    }
    return 0;
    }
//
// pseries_suspend_enter - Final phase of hibernation
//
// Return value:
// 0 on success / other on failure
//
#[no_mangle]
unsafe extern "C" fn pseries_suspend_enter(state: suspend_state_t) -> c_int {
    static int pseries_suspend_enter(suspend_state_t state)
    {
    return rtas_ibm_suspend_me(core::ptr::null_mut());
    }
//
// store_hibernate - Initiate partition hibernation
// @dev:		subsys root device
// @attr:		device attribute struct
// @buf:		buffer
// @count:		buffer size
//
// Write the stream ID received from the HMC to this file
// to trigger hibernating the partition
//
// Return value:
// number of bytes printed to buffer / other on failure
//
    static ssize_t store_hibernate(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    u64 stream_id;
    int rc;
    if (!capable(CAP_SYS_ADMIN))
    return -EPERM;
    stream_id = simple_strtoul(buf, core::ptr::null_mut(), 16);
    do {
    rc = pseries_suspend_begin(stream_id);
    if (rc == -EAGAIN)
    ssleep(1);
    } while (rc == -EAGAIN);
    if (!rc)
    rc = pm_suspend(PM_SUSPEND_MEM);
    if (!rc) {
    rc = count;
    post_mobility_fixup();
    }
    return rc;
    }
pub const USER_DT_UPDATE: c_int = 0;
pub const KERN_DT_UPDATE: c_int = 1;
//
// show_hibernate - Report device tree update responsibilty
// @dev:		subsys root device
// @attr:		device attribute struct
// @buf:		buffer
//
// Report whether a device tree update is performed by the kernel after a
// resume, or if drmgr must coordinate the update from user space.
//
// Return value:
// 0 if drmgr is to initiate update, and 1 otherwise
//
    static ssize_t show_hibernate(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", KERN_DT_UPDATE);
    }
    static DEVICE_ATTR(hibernate, 0644, show_hibernate, store_hibernate);
    static const struct bus_type suspend_subsys = {
    .name = "power",
    .dev_name = "power",
    };
    static const struct platform_suspend_ops pseries_suspend_ops = {
    .valid		= suspend_valid_only_mem,
    .enter		= pseries_suspend_enter,
    };
//
// pseries_suspend_sysfs_register - Register with sysfs
//
// Return value:
// 0 on success / other on failure
//
#[no_mangle]
unsafe extern "C" fn pseries_suspend_sysfs_register(dev: *mut device) -> c_int {
    static int pseries_suspend_sysfs_register(struct device *dev)
    {
    struct device *dev_root;
    int rc;
    if ((rc = subsys_system_register(&suspend_subsys, core::ptr::null_mut())))
    return rc;
    dev.id = 0;
    dev.bus = &suspend_subsys;
    dev_root = bus_get_dev_root(&suspend_subsys);
    if (dev_root) {
    rc = device_create_file(dev_root, &dev_attr_hibernate);
    put_device(dev_root);
    if (rc)
    goto subsys_unregister;
    }
    return 0;
    subsys_unregister:
    bus_unregister(&suspend_subsys);
    return rc;
    }
//
// pseries_suspend_init - initcall for pSeries suspend
//
// Return value:
// 0 on success / other on failure
//
#[no_mangle]
unsafe extern "C" fn pseries_suspend_init() -> int __init {
    static int __init pseries_suspend_init(void)
    {
    int rc;
    if (!firmware_has_feature(FW_FEATURE_LPAR))
    return 0;
    if ((rc = pseries_suspend_sysfs_register(&suspend_dev)))
    return rc;
    suspend_set_ops(&pseries_suspend_ops);
    return 0;
    }
    machine_device_initcall(pseries, pseries_suspend_init);
