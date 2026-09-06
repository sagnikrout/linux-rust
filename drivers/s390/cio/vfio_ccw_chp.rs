//! Automatically rewritten from C to Rust
//! Source: drivers/s390/cio/vfio_ccw_chp.c
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
// Channel path related status regions for vfio_ccw
//
// Copyright IBM Corp. 2020
//
// Author(s): Farhan Ali <alifm@linux.ibm.com>
// Eric Farman <farman@linux.ibm.com>
//

    static ssize_t vfio_ccw_schib_region_read(struct vfio_ccw_private *private,
    char __user *buf, size_t count,
    loff_t *ppos)
    {
    struct subchannel *sch = to_subchannel(private.vdev.dev.parent);
    let mut i: c_uint = VFIO_CCW_OFFSET_TO_INDEX(*ppos) - VFIO_CCW_NUM_REGIONS;
    let mut pos: loff_t = *ppos & VFIO_CCW_OFFSET_MASK;
    struct ccw_schib_region *region;
    int ret;
    if (pos + count > sizeof(*region))
    return -EINVAL;
    mutex_lock(&private.io_mutex);
    if (i >= private.num_regions) {
    ret = -EINVAL;
    goto out;
    }
    i = array_index_nospec(i, private.num_regions);
    region = private.region[i].data;
    if (cio_update_schib(sch)) {
    ret = -ENODEV;
    goto out;
    }
    memcpy(region, &sch.schib, sizeof(*region));
    if (copy_to_user(buf, (void *)region + pos, count)) {
    ret = -EFAULT;
    goto out;
    }
    ret = count;
    out:
    mutex_unlock(&private.io_mutex);
    return ret;
    }
    static ssize_t vfio_ccw_schib_region_write(struct vfio_ccw_private *private,
    const char __user *buf, size_t count,
    loff_t *ppos)
    {
    return -EINVAL;
    }
    static void vfio_ccw_schib_region_release(struct vfio_ccw_private *private,
    struct vfio_ccw_region *region)
    {
    }
    static const struct vfio_ccw_regops vfio_ccw_schib_region_ops = {
    .read = vfio_ccw_schib_region_read,
    .write = vfio_ccw_schib_region_write,
    .release = vfio_ccw_schib_region_release,
    };
#[no_mangle]
pub unsafe extern "C" fn vfio_ccw_register_schib_dev_regions(private: *mut vfio_ccw_private) -> c_int {
    int vfio_ccw_register_schib_dev_regions(struct vfio_ccw_private *private)
    {
    return vfio_ccw_register_dev_region(private,
    VFIO_REGION_SUBTYPE_CCW_SCHIB,
    &vfio_ccw_schib_region_ops,
    sizeof(struct ccw_schib_region),
    VFIO_REGION_INFO_FLAG_READ,
    private.schib_region);
    }
    static ssize_t vfio_ccw_crw_region_read(struct vfio_ccw_private *private,
    char __user *buf, size_t count,
    loff_t *ppos)
    {
    let mut i: c_uint = VFIO_CCW_OFFSET_TO_INDEX(*ppos) - VFIO_CCW_NUM_REGIONS;
    let mut pos: loff_t = *ppos & VFIO_CCW_OFFSET_MASK;
    struct ccw_crw_region *region;
    struct vfio_ccw_crw *crw;
    unsigned long flags;
    int ret;
    if (pos + count > sizeof(*region))
    return -EINVAL;
    mutex_lock(&private.io_mutex);
    if (i >= private.num_regions) {
    ret = -EINVAL;
    goto out;
    }
    i = array_index_nospec(i, private.num_regions);
    region = private.region[i].data;
    spin_lock_irqsave(&private.crw_lock, flags);
    crw = list_first_entry_or_null(&private.crw,
    struct vfio_ccw_crw, next);
    if (crw)
    list_del(&crw.next);
// Drop CRW lock while copying to userspace
    spin_unlock_irqrestore(&private.crw_lock, flags);
    if (crw)
    memcpy(&region.crw, &crw.crw, sizeof(region.crw));
    if (copy_to_user(buf, (void *)region + pos, count))
    ret = -EFAULT;
    else
    ret = count;
    region.crw = 0;
    kfree(crw);
// Notify the guest if more CRWs are on our queue
    spin_lock_irqsave(&private.crw_lock, flags);
    if (!list_empty(&private.crw) && private.crw_trigger)
    eventfd_signal(private.crw_trigger);
    spin_unlock_irqrestore(&private.crw_lock, flags);
    out:
    mutex_unlock(&private.io_mutex);
    return ret;
    }
    static ssize_t vfio_ccw_crw_region_write(struct vfio_ccw_private *private,
    const char __user *buf, size_t count,
    loff_t *ppos)
    {
    return -EINVAL;
    }
    static void vfio_ccw_crw_region_release(struct vfio_ccw_private *private,
    struct vfio_ccw_region *region)
    {
    }
    static const struct vfio_ccw_regops vfio_ccw_crw_region_ops = {
    .read = vfio_ccw_crw_region_read,
    .write = vfio_ccw_crw_region_write,
    .release = vfio_ccw_crw_region_release,
    };
#[no_mangle]
pub unsafe extern "C" fn vfio_ccw_register_crw_dev_regions(private: *mut vfio_ccw_private) -> c_int {
    int vfio_ccw_register_crw_dev_regions(struct vfio_ccw_private *private)
    {
    return vfio_ccw_register_dev_region(private,
    VFIO_REGION_SUBTYPE_CCW_CRW,
    &vfio_ccw_crw_region_ops,
    sizeof(struct ccw_crw_region),
    VFIO_REGION_INFO_FLAG_READ,
    private.crw_region);
    }
