//! Automatically rewritten from C to Rust
//! Source: drivers/watchdog/i6300esb.c
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
// i6300esb:	Watchdog timer driver for Intel 6300ESB chipset
//
// (c) Copyright 2004 Google Inc.
// (c) Copyright 2005 David Härdeman <david@2gen.com>
//
// based on i810-tco.c which is in turn based on softdog.c
//
// The timer is implemented in the following I/O controller hubs:
// (See the intel documentation on http://developer.intel.com.)
// 6300ESB chip : document number 300641-004
//
// 2004YYZZ Ross Biro
// Initial version 0.01
// 2004YYZZ Ross Biro
// Version 0.02
// 20050210 David Härdeman <david@2gen.com>
// Ported driver to kernel 2.6
// 20171016 Radu Rendec <rrendec@arista.com>
// Change driver to use the watchdog subsystem
// Add support for multiple 6300ESB devices
//
// Includes, defines, variables, module parameters, ...
//

// Module and version information

// PCI configuration registers
pub const ESB_CONFIG_REG: c_uint = 0x60            /* Config register                   */;
pub const ESB_LOCK_REG: c_uint = 0x68            /* WDT lock register                 */;
// Memory mapped registers

// Lock register bits

// Config register bits

// Reload register bits

// Magic constants
pub const ESB_UNLOCK1: c_uint = 0x80            /* Step 1 to unlock reset registers  */;
pub const ESB_UNLOCK2: c_uint = 0x86            /* Step 2 to unlock reset registers  */;
// module parameters
// 30 sec default heartbeat (1 < heartbeat < 2*1023)
pub const ESB_HEARTBEAT_MIN: c_int = 1;
pub const ESB_HEARTBEAT_MAX: c_int = 2046;
pub const ESB_HEARTBEAT_DEFAULT: c_int = 30;

    "<heartbeat<" __MODULE_STRING(ESB_HEARTBEAT_MAX)
    static int heartbeat; /* in seconds */
    module_param(heartbeat, int, 0);
    MODULE_PARM_DESC(heartbeat,
    "Watchdog heartbeat in seconds. (" ESB_HEARTBEAT_RANGE
    ", default=" __MODULE_STRING(ESB_HEARTBEAT_DEFAULT) ")");
    let mut nowayout: static bool = WATCHDOG_NOWAYOUT;
    module_param(nowayout, bool, 0);
    MODULE_PARM_DESC(nowayout,
    "Watchdog cannot be stopped once started (default="
    __MODULE_STRING(WATCHDOG_NOWAYOUT) ")");
// internal variables
#[repr(C)]
#[derive(Copy, Clone)]
pub struct esb_dev {
    pub wdd: watchdog_device,
    pub base: *mut void __iomem,
    pub pdev: *mut pci_dev,
}

//
// Some i6300ESB specific functions
//
// Prepare for reloading the timer by unlocking the proper registers.
// This is performed by first writing 0x80 followed by 0x86 to the
// reload register. After this the appropriate registers can be written
// to once before they need to be unlocked again.
//
#[no_mangle]
pub unsafe extern "C" fn esb_unlock_registers(edev: *mut esb_dev) {
    static inline void esb_unlock_registers(struct esb_dev *edev)
    {
    writew(ESB_UNLOCK1, ESB_RELOAD_REG(edev));
    writew(ESB_UNLOCK2, ESB_RELOAD_REG(edev));
    }
#[no_mangle]
unsafe extern "C" fn esb_timer_start(wdd: *mut watchdog_device) -> c_int {
    static int esb_timer_start(struct watchdog_device *wdd)
    {
    struct esb_dev *edev = to_esb_dev(wdd);
    let mut _wdd_nowayout: c_int = test_bit(WDOG_NO_WAY_OUT, &wdd.status);
    u8 val;
    esb_unlock_registers(edev);
    writew(ESB_WDT_RELOAD, ESB_RELOAD_REG(edev));
// Enable or Enable + Lock?
    val = ESB_WDT_ENABLE | (_wdd_nowayout ? ESB_WDT_LOCK : 0x00);
    pci_write_config_byte(edev.pdev, ESB_LOCK_REG, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn esb_timer_stop(wdd: *mut watchdog_device) -> c_int {
    static int esb_timer_stop(struct watchdog_device *wdd)
    {
    struct esb_dev *edev = to_esb_dev(wdd);
    u8 val;
// First, reset timers as suggested by the docs
    esb_unlock_registers(edev);
    writew(ESB_WDT_RELOAD, ESB_RELOAD_REG(edev));
// Then disable the WDT
    pci_write_config_byte(edev.pdev, ESB_LOCK_REG, 0x0);
    pci_read_config_byte(edev.pdev, ESB_LOCK_REG, &val);
// Returns 0 if the timer was disabled, non-zero otherwise
    return val & ESB_WDT_ENABLE;
    }
#[no_mangle]
unsafe extern "C" fn esb_timer_keepalive(wdd: *mut watchdog_device) -> c_int {
    static int esb_timer_keepalive(struct watchdog_device *wdd)
    {
    struct esb_dev *edev = to_esb_dev(wdd);
    esb_unlock_registers(edev);
    writew(ESB_WDT_RELOAD, ESB_RELOAD_REG(edev));
// FIXME: Do we need to flush anything here?
    return 0;
    }
    static int esb_timer_set_heartbeat(struct watchdog_device *wdd,
    unsigned int time)
    {
    struct esb_dev *edev = to_esb_dev(wdd);
    u32 val;
// We shift by 9, so if we are passed a value of 1 sec,
// val will be 1 << 9 = 512, then write that to two
// timers => 2 * 512 = 1024 (which is decremented at 1KHz)
//
    val = time << 9;
// Write timer 1
    esb_unlock_registers(edev);
    writel(val, ESB_TIMER1_REG(edev));
// Write timer 2
    esb_unlock_registers(edev);
    writel(val, ESB_TIMER2_REG(edev));
// Reload
    esb_unlock_registers(edev);
    writew(ESB_WDT_RELOAD, ESB_RELOAD_REG(edev));
// FIXME: Do we need to flush everything out?
// Done
    wdd.timeout = time;
    return 0;
    }
//
// Watchdog Subsystem Interfaces
//
    static struct watchdog_info esb_info = {
    .identity = ESB_MODULE_NAME,
    .options = WDIOF_SETTIMEOUT | WDIOF_KEEPALIVEPING | WDIOF_MAGICCLOSE,
    };
    static const struct watchdog_ops esb_ops = {
    .owner = THIS_MODULE,
    .start = esb_timer_start,
    .stop = esb_timer_stop,
    .set_timeout = esb_timer_set_heartbeat,
    .ping = esb_timer_keepalive,
    };
//
// Data for PCI driver interface
//
    static const struct pci_device_id esb_pci_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_ESB_9), },
    { 0, },                 /* End of list */
    };
    MODULE_DEVICE_TABLE(pci, esb_pci_tbl);
//
// Init & exit routines
//
#[no_mangle]
unsafe extern "C" fn esb_getdevice(edev: *mut esb_dev) -> c_uchar {
    static unsigned char esb_getdevice(struct esb_dev *edev)
    {
    if (pci_enable_device(edev.pdev)) {
    dev_err(&edev.pdev.dev, "failed to enable device\n");
    goto err_devput;
    }
    if (pci_request_region(edev.pdev, 0, ESB_MODULE_NAME)) {
    dev_err(&edev.pdev.dev, "failed to request region\n");
    goto err_disable;
    }
    edev.base = pci_ioremap_bar(edev.pdev, 0);
    if (edev.base == core::ptr::null_mut()) {
// Something's wrong here, BASEADDR has to be set
    dev_err(&edev.pdev.dev, "failed to get BASEADDR\n");
    goto err_release;
    }
// Done
    dev_set_drvdata(&edev.pdev.dev, edev);
    return 1;
    err_release:
    pci_release_region(edev.pdev, 0);
    err_disable:
    pci_disable_device(edev.pdev);
    err_devput:
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn esb_initdevice(edev: *mut esb_dev) {
    static void esb_initdevice(struct esb_dev *edev)
    {
    u8 val1;
    u16 val2;
//
// Config register:
// Bit    5 : 0 = Enable WDT_OUTPUT
// Bit    2 : 0 = set the timer frequency to the PCI clock
// divided by 2^15 (approx 1KHz).
// Bits 1:0 : 11 = WDT_INT_TYPE Disabled.
// The watchdog has two timers, it can be setup so that the
// expiry of timer1 results in an interrupt and the expiry of
// timer2 results in a reboot. We set it to not generate
// any interrupts as there is not much we can do with it
// right now.
//
    pci_write_config_word(edev.pdev, ESB_CONFIG_REG, 0x0003);
// Check that the WDT isn't already locked
    pci_read_config_byte(edev.pdev, ESB_LOCK_REG, &val1);
    if (val1 & ESB_WDT_LOCK)
    dev_warn(&edev.pdev.dev, "nowayout already set\n");
// Set the timer to watchdog mode and disable it for now
    pci_write_config_byte(edev.pdev, ESB_LOCK_REG, 0x00);
// Check if the watchdog was previously triggered
    esb_unlock_registers(edev);
    val2 = readw(ESB_RELOAD_REG(edev));
    if (val2 & ESB_WDT_TIMEOUT)
    edev.wdd.bootstatus = WDIOF_CARDRESET;
// Reset WDT_TIMEOUT flag and timers
    esb_unlock_registers(edev);
    writew((ESB_WDT_TIMEOUT | ESB_WDT_RELOAD), ESB_RELOAD_REG(edev));
// And set the correct timeout value
    esb_timer_set_heartbeat(&edev.wdd, edev.wdd.timeout);
    }
    static int esb_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct esb_dev *edev;
    int ret;
    edev = devm_kzalloc(&pdev.dev, sizeof(*edev), GFP_KERNEL);
    if (!edev)
    return -ENOMEM;
// Check whether or not the hardware watchdog is there
    edev.pdev = pdev;
    if (!esb_getdevice(edev))
    return -ENODEV;
// Initialize the watchdog and make sure it does not run
    edev.wdd.info = &esb_info;
    edev.wdd.ops = &esb_ops;
    edev.wdd.min_timeout = ESB_HEARTBEAT_MIN;
    edev.wdd.max_timeout = ESB_HEARTBEAT_MAX;
    edev.wdd.timeout = ESB_HEARTBEAT_DEFAULT;
    watchdog_init_timeout(&edev.wdd, heartbeat, core::ptr::null_mut());
    watchdog_set_nowayout(&edev.wdd, nowayout);
    watchdog_stop_on_reboot(&edev.wdd);
    watchdog_stop_on_unregister(&edev.wdd);
    esb_initdevice(edev);
// Register the watchdog so that userspace has access to it
    ret = watchdog_register_device(&edev.wdd);
    if (ret != 0)
    goto err_unmap;
    dev_info(&pdev.dev,
    "initialized. heartbeat=%d sec (nowayout=%d)\n",
    edev.wdd.timeout, nowayout);
    return 0;
    err_unmap:
    iounmap(edev.base);
    pci_release_region(edev.pdev, 0);
    pci_disable_device(edev.pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn esb_remove(pdev: *mut pci_dev) {
    static void esb_remove(struct pci_dev *pdev)
    {
    struct esb_dev *edev = dev_get_drvdata(&pdev.dev);
    watchdog_unregister_device(&edev.wdd);
    iounmap(edev.base);
    pci_release_region(edev.pdev, 0);
    pci_disable_device(edev.pdev);
    }
    static struct pci_driver esb_driver = {
    .name		= ESB_MODULE_NAME,
    .id_table	= esb_pci_tbl,
    .probe          = esb_probe,
    .remove         = esb_remove,
    };
    module_pci_driver(esb_driver);
    MODULE_AUTHOR("Ross Biro and David Härdeman");
    MODULE_DESCRIPTION("Watchdog driver for Intel 6300ESB chipsets");
    MODULE_LICENSE("GPL");
