//! Automatically rewritten from C to Rust
//! Source: drivers/pci/hotplug/shpchp_sysfs.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Compaq Hot Plug Controller Driver
//
// Copyright (c) 1995,2001 Compaq Computer Corporation
// Copyright (c) 2001,2003 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (c) 2001 IBM Corp.
//
// All rights reserved.
//
// Send feedback to <greg@kroah.com>
//

// A few routines that create sysfs entries for the hot plug controller
#[no_mangle]
unsafe extern "C" fn show_ctrl(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t show_ctrl(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct pci_dev *pdev;
    struct resource *res;
    struct pci_bus *bus;
    let mut len: usize = 0;
    int busnr;
    pdev = to_pci_dev(dev);
    bus = pdev.subordinate;
    len += sysfs_emit_at(buf, len, "Free resources: memory\n");
    pci_bus_for_each_resource(bus, res) {
    if (res && (res.flags & IORESOURCE_MEM) &&
    !(res.flags & IORESOURCE_PREFETCH)) {
    len += sysfs_emit_at(buf, len,
    "start = %8.8llx, length = %8.8llx\n",
    (unsigned long long)res.start,
    (unsigned long long)resource_size(res));
    }
    }
    len += sysfs_emit_at(buf, len, "Free resources: prefetchable memory\n");
    pci_bus_for_each_resource(bus, res) {
    if (res && (res.flags & IORESOURCE_MEM) &&
    (res.flags & IORESOURCE_PREFETCH)) {
    len += sysfs_emit_at(buf, len,
    "start = %8.8llx, length = %8.8llx\n",
    (unsigned long long)res.start,
    (unsigned long long)resource_size(res));
    }
    }
    len += sysfs_emit_at(buf, len, "Free resources: IO\n");
    pci_bus_for_each_resource(bus, res) {
    if (res && (res.flags & IORESOURCE_IO)) {
    len += sysfs_emit_at(buf, len,
    "start = %8.8llx, length = %8.8llx\n",
    (unsigned long long)res.start,
    (unsigned long long)resource_size(res));
    }
    }
    len += sysfs_emit_at(buf, len, "Free resources: bus numbers\n");
    for (busnr = bus.busn_res.start; busnr <= bus.busn_res.end; busnr++) {
    if (!pci_find_bus(pci_domain_nr(bus), busnr))
    break;
    }
    if (busnr < bus.busn_res.end)
    len += sysfs_emit_at(buf, len,
    "start = %8.8x, length = %8.8x\n",
    busnr, (int)(bus.busn_res.end - busnr));
    return len;
    }
    static DEVICE_ATTR(ctrl, S_IRUGO, show_ctrl, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn shpchp_create_ctrl_files(ctrl: *mut controller) -> c_int {
    int shpchp_create_ctrl_files(struct controller *ctrl)
    {
    return device_create_file(&ctrl.pci_dev.dev, &dev_attr_ctrl);
    }
#[no_mangle]
pub unsafe extern "C" fn shpchp_remove_ctrl_files(ctrl: *mut controller) {
    void shpchp_remove_ctrl_files(struct controller *ctrl)
    {
    device_remove_file(&ctrl.pci_dev.dev, &dev_attr_ctrl);
    }
