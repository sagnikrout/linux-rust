//! Automatically rewritten from C to Rust
//! Source: drivers/ssb/pcihost_wrapper.c
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


//
// Sonics Silicon Backplane
// PCI Hostdevice wrapper
//
// Copyright (c) 2005 Martin Langer <martin-langer@gmx.de>
// Copyright (c) 2005 Stefano Brivio <st3@riseup.net>
// Copyright (c) 2005 Danny van Dyk <kugelfang@gentoo.org>
// Copyright (c) 2005 Andreas Jaggi <andreas.jaggi@waterwave.ch>
// Copyright (c) 2005-2007 Michael Buesch <m@bues.ch>
//
// Licensed under the GNU/GPL. See COPYING for details.
//

#[no_mangle]
unsafe extern "C" fn ssb_pcihost_suspend(d: *mut device) -> c_int {
    static int ssb_pcihost_suspend(struct device *d)
    {
    struct pci_dev *dev = to_pci_dev(d);
    struct ssb_bus *ssb = pci_get_drvdata(dev);
    int err;
    err = ssb_bus_suspend(ssb);
    if (err)
    return err;
    pci_save_state(dev);
    pci_disable_device(dev);
// if there is a wakeup enabled child device on ssb bus,
    enable pci wakeup posibility. */
    device_set_wakeup_enable(d, d.power.wakeup_path);
    pci_prepare_to_sleep(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssb_pcihost_resume(d: *mut device) -> c_int {
    static int ssb_pcihost_resume(struct device *d)
    {
    struct pci_dev *dev = to_pci_dev(d);
    struct ssb_bus *ssb = pci_get_drvdata(dev);
    int err;
    pci_back_from_sleep(dev);
    err = pci_enable_device(dev);
    if (err)
    return err;
    pci_restore_state(dev);
    err = ssb_bus_resume(ssb);
    if (err)
    return err;
    return 0;
    }
    static const struct dev_pm_ops ssb_pcihost_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(ssb_pcihost_suspend, ssb_pcihost_resume)
    };

    static int ssb_pcihost_probe(struct pci_dev *dev,
    const struct pci_device_id *id)
    {
    struct ssb_bus *ssb;
    let mut err: c_int = -ENOMEM;
    u32 val;
    ssb = kzalloc_obj(*ssb);
    if (!ssb)
    goto out;
    err = pci_enable_device(dev);
    if (err)
    goto err_kfree_ssb;
    err = pci_request_regions(dev, dev_driver_string(&dev.dev));
    if (err)
    goto err_pci_disable;
    pci_set_master(dev);
// Disable the RETRY_TIMEOUT register (0x41) to keep
// PCI Tx retries from interfering with C3 CPU state
    pci_read_config_dword(dev, 0x40, &val);
    if ((val & 0x0000ff00) != 0)
    pci_write_config_dword(dev, 0x40, val & 0xffff00ff);
    err = ssb_bus_pcibus_register(ssb, dev);
    if (err)
    goto err_pci_release_regions;
    pci_set_drvdata(dev, ssb);
    out:
    return err;
    err_pci_release_regions:
    pci_release_regions(dev);
    err_pci_disable:
    pci_disable_device(dev);
    err_kfree_ssb:
    kfree(ssb);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ssb_pcihost_remove(dev: *mut pci_dev) {
    static void ssb_pcihost_remove(struct pci_dev *dev)
    {
    struct ssb_bus *ssb = pci_get_drvdata(dev);
    ssb_bus_unregister(ssb);
    pci_release_regions(dev);
    pci_disable_device(dev);
    kfree(ssb);
    pci_set_drvdata(dev, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn ssb_pcihost_register(driver: *mut pci_driver) -> c_int {
    int ssb_pcihost_register(struct pci_driver *driver)
    {
    driver.probe = ssb_pcihost_probe;
    driver.remove = ssb_pcihost_remove;

    driver.driver.pm = &ssb_pcihost_pm_ops;

    return pci_register_driver(driver);
    }
    EXPORT_SYMBOL(ssb_pcihost_register);
