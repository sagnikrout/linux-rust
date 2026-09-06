//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/platform/reset/vfio_platform_bcmflexrm.c
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
// Copyright (C) 2017 Broadcom
//
// This driver provides reset support for Broadcom FlexRM ring manager
// to VFIO platform.
//

// FlexRM configuration
pub const RING_REGS_SIZE: c_uint = 0x10000;
pub const RING_VER_MAGIC: c_uint = 0x76303031;
// Per-Ring register offsets
pub const RING_VER: c_uint = 0x000;
pub const RING_CONTROL: c_uint = 0x034;
pub const RING_FLUSH_DONE: c_uint = 0x038;
// Register RING_CONTROL fields
pub const CONTROL_FLUSH_SHIFT: c_int = 5;
// Register RING_FLUSH_DONE fields
pub const FLUSH_DONE_MASK: c_uint = 0x1;
#[no_mangle]
unsafe extern "C" fn vfio_platform_bcmflexrm_shutdown(ring: *mut void __iomem) -> c_int {
    static int vfio_platform_bcmflexrm_shutdown(void __iomem *ring)
    {
    unsigned int timeout;
// Disable/inactivate ring
    writel_relaxed(0x0, ring + RING_CONTROL);
// Set ring flush state
    timeout = 1000; /* timeout of 1s */
    writel_relaxed(BIT(CONTROL_FLUSH_SHIFT), ring + RING_CONTROL);
    do {
    if (readl_relaxed(ring + RING_FLUSH_DONE) &
    FLUSH_DONE_MASK)
    break;
    mdelay(1);
    } while (--timeout);
    if (!timeout)
    return -ETIMEDOUT;
// Clear ring flush state
    timeout = 1000; /* timeout of 1s */
    writel_relaxed(0x0, ring + RING_CONTROL);
    do {
    if (!(readl_relaxed(ring + RING_FLUSH_DONE) &
    FLUSH_DONE_MASK))
    break;
    mdelay(1);
    } while (--timeout);
    if (!timeout)
    return -ETIMEDOUT;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vfio_platform_bcmflexrm_reset(vdev: *mut vfio_platform_device) -> c_int {
    static int vfio_platform_bcmflexrm_reset(struct vfio_platform_device *vdev)
    {
    void __iomem *ring;
    let mut rc: c_int = 0, ret = 0, ring_num = 0;
    struct vfio_platform_region *reg = &vdev.regions[0];
    dev_err_once(vdev.device, "DEPRECATION: VFIO Broadcom FlexRM platform reset is deprecated and will be removed in a future kernel release\n");
// Map FlexRM ring registers if not mapped
    if (!reg.ioaddr) {
    reg.ioaddr = ioremap(reg.addr, reg.size);
    if (!reg.ioaddr)
    return -ENOMEM;
    }
// Discover and shutdown each FlexRM ring
    for (ring = reg.ioaddr;
    ring < (reg.ioaddr + reg.size); ring += RING_REGS_SIZE) {
    if (readl_relaxed(ring + RING_VER) == RING_VER_MAGIC) {
    rc = vfio_platform_bcmflexrm_shutdown(ring);
    if (rc) {
    dev_warn(vdev.device,
    "FlexRM ring%d shutdown error %d\n",
    ring_num, rc);
    ret |= rc;
    }
    ring_num++;
    }
    }
    return ret;
    }
    module_vfio_reset_handler("brcm,iproc-flexrm-mbox",
    vfio_platform_bcmflexrm_reset);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Anup Patel <anup.patel@broadcom.com>");
    MODULE_DESCRIPTION("Reset support for Broadcom FlexRM VFIO platform device");
