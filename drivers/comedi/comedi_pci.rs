//! Automatically rewritten from C to Rust
//! Source: drivers/comedi/comedi_pci.c
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
// comedi_pci.c
// Comedi PCI driver specific functions.
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1997-2000 David A. Schleef <ds@schleef.org>
//

//
// comedi_to_pci_dev() - Return PCI device attached to COMEDI device
// @dev: COMEDI device.
//
// Assuming @dev->hw_dev is non-%NULL, it is assumed to be pointing to a
// a &struct device embedded in a &struct pci_dev.
//
// Return: Attached PCI device if @dev->hw_dev is non-%NULL.
// Return %NULL if @dev->hw_dev is %NULL.
//
    struct pci_dev *comedi_to_pci_dev(struct comedi_device *dev)
    {
    return dev.hw_dev ? to_pci_dev(dev.hw_dev) : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(comedi_to_pci_dev);
//
// comedi_pci_enable() - Enable the PCI device and request the regions
// @dev: COMEDI device.
//
// Assuming @dev->hw_dev is non-%NULL, it is assumed to be pointing to a
// a &struct device embedded in a &struct pci_dev.  Enable the PCI device
// and request its regions.  Set @dev->ioenabled to %true if successful,
// otherwise undo what was done.
//
// Calls to comedi_pci_enable() and comedi_pci_disable() cannot be nested.
//
// Return:
// 0 on success,
// -%ENODEV if @dev->hw_dev is %NULL,
// -%EBUSY if regions busy,
// or some negative error number if failed to enable PCI device.
//
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_enable(dev: *mut comedi_device) -> c_int {
    int comedi_pci_enable(struct comedi_device *dev)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    int rc;
    if (!pcidev)
    return -ENODEV;
    rc = pci_enable_device(pcidev);
    if (rc < 0)
    return rc;
    rc = pci_request_regions(pcidev, dev.board_name);
    if (rc < 0)
    pci_disable_device(pcidev);
    else
    dev.ioenabled = true;
    return rc;
    }
    EXPORT_SYMBOL_GPL(comedi_pci_enable);
//
// comedi_pci_disable() - Release the regions and disable the PCI device
// @dev: COMEDI device.
//
// Assuming @dev->hw_dev is non-%NULL, it is assumed to be pointing to a
// a &struct device embedded in a &struct pci_dev.  If the earlier call
// to comedi_pci_enable() was successful, release the PCI device's regions
// and disable it.  Reset @dev->ioenabled back to %false.
//
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_disable(dev: *mut comedi_device) {
    void comedi_pci_disable(struct comedi_device *dev)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    if (pcidev && dev.ioenabled) {
    pci_release_regions(pcidev);
    pci_disable_device(pcidev);
    }
    dev.ioenabled = false;
    }
    EXPORT_SYMBOL_GPL(comedi_pci_disable);
//
// comedi_pci_detach() - A generic "detach" handler for PCI COMEDI drivers
// @dev: COMEDI device.
//
// COMEDI drivers for PCI devices that need no special clean-up of private data
// and have no ioremapped regions other than that pointed to by @dev->mmio may
// use this function as its "detach" handler called by the COMEDI core when a
// COMEDI device is being detached from the low-level driver.  It may be also
// called from a more specific "detach" handler that does additional clean-up.
//
// Free the IRQ if @dev->irq is non-zero, iounmap @dev->mmio if it is
// non-%NULL, and call comedi_pci_disable() to release the PCI device's regions
// and disable it.
//
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_detach(dev: *mut comedi_device) {
    void comedi_pci_detach(struct comedi_device *dev)
    {
    struct pci_dev *pcidev = comedi_to_pci_dev(dev);
    if (!pcidev || !dev.ioenabled)
    return;
    if (dev.irq) {
    free_irq(dev.irq, dev);
    dev.irq = 0;
    }
    if (dev.mmio) {
    iounmap(dev.mmio);
    dev.mmio = core::ptr::null_mut();
    }
    comedi_pci_disable(dev);
    }
    EXPORT_SYMBOL_GPL(comedi_pci_detach);
//
// comedi_pci_auto_config() - Configure/probe a PCI COMEDI device
// @pcidev: PCI device.
// @driver: Registered COMEDI driver.
// @context: Driver specific data, passed to comedi_auto_config().
//
// Typically called from the pci_driver (*probe) function.  Auto-configure
// a COMEDI device, using the &struct device embedded in *@pcidev as the
// hardware device.  The @context value gets passed through to @driver's
// "auto_attach" handler.  The "auto_attach" handler may call
// comedi_to_pci_dev() on the passed in COMEDI device to recover @pcidev.
//
// Return: The result of calling comedi_auto_config() (0 on success, or
// a negative error number on failure).
//
    int comedi_pci_auto_config(struct pci_dev *pcidev,
    struct comedi_driver *driver,
    unsigned long context)
    {
    return comedi_auto_config(&pcidev.dev, driver, context);
    }
    EXPORT_SYMBOL_GPL(comedi_pci_auto_config);
//
// comedi_pci_auto_unconfig() - Unconfigure/remove a PCI COMEDI device
// @pcidev: PCI device.
//
// Typically called from the pci_driver (*remove) function.  Auto-unconfigure
// a COMEDI device attached to this PCI device, using a pointer to the
// &struct device embedded in *@pcidev as the hardware device.  The COMEDI
// driver's "detach" handler will be called during unconfiguration of the
// COMEDI device.
//
// Note that the COMEDI device may have already been unconfigured using the
// %COMEDI_DEVCONFIG ioctl, in which case this attempt to unconfigure it
// again should be ignored.
//
#[no_mangle]
pub unsafe extern "C" fn comedi_pci_auto_unconfig(pcidev: *mut pci_dev) {
    void comedi_pci_auto_unconfig(struct pci_dev *pcidev)
    {
    comedi_auto_unconfig(&pcidev.dev);
    }
    EXPORT_SYMBOL_GPL(comedi_pci_auto_unconfig);
//
// comedi_pci_driver_register() - Register a PCI COMEDI driver
// @comedi_driver: COMEDI driver to be registered.
// @pci_driver: PCI driver to be registered.
//
// This function is called from the module_init() of PCI COMEDI driver modules
// to register the COMEDI driver and the PCI driver.  Do not call it directly,
// use the module_comedi_pci_driver() helper macro instead.
//
// Return: 0 on success, or a negative error number on failure.
//
    int comedi_pci_driver_register(struct comedi_driver *comedi_driver,
    struct pci_driver *pci_driver)
    {
    int ret;
    ret = comedi_driver_register(comedi_driver);
    if (ret < 0)
    return ret;
    ret = pci_register_driver(pci_driver);
    if (ret < 0) {
    comedi_driver_unregister(comedi_driver);
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(comedi_pci_driver_register);
//
// comedi_pci_driver_unregister() - Unregister a PCI COMEDI driver
// @comedi_driver: COMEDI driver to be unregistered.
// @pci_driver: PCI driver to be unregistered.
//
// This function is called from the module_exit() of PCI COMEDI driver modules
// to unregister the PCI driver and the COMEDI driver.  Do not call it
// directly, use the module_comedi_pci_driver() helper macro instead.
//
    void comedi_pci_driver_unregister(struct comedi_driver *comedi_driver,
    struct pci_driver *pci_driver)
    {
    pci_unregister_driver(pci_driver);
    comedi_driver_unregister(comedi_driver);
    }
    EXPORT_SYMBOL_GPL(comedi_pci_driver_unregister);
    MODULE_AUTHOR("https://www.comedi.org");
    MODULE_DESCRIPTION("Comedi PCI interface module");
    MODULE_LICENSE("GPL");
