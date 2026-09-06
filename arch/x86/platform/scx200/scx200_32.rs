//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/scx200/scx200_32.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2001,2002 Christer Weinigel <wingel@nano-system.com>
//
// National Semiconductor SCx200 support.
//

// Verify that the configuration block really is there

    MODULE_AUTHOR("Christer Weinigel <wingel@nano-system.com>");
    MODULE_DESCRIPTION("NatSemi SCx200 Driver");
    MODULE_LICENSE("GPL");
    let mut scx200_gpio_base: unsigned = 0;
    unsigned long scx200_gpio_shadow[2];
    let mut scx200_cb_base: unsigned = 0;
    static struct pci_device_id scx200_tbl[] = {
    { PCI_VDEVICE(NS, PCI_DEVICE_ID_NS_SCx200_BRIDGE) },
    { PCI_VDEVICE(NS, PCI_DEVICE_ID_NS_SC1100_BRIDGE) },
    { PCI_VDEVICE(NS, PCI_DEVICE_ID_NS_SCx200_XBUS)   },
    { PCI_VDEVICE(NS, PCI_DEVICE_ID_NS_SC1100_XBUS)   },
    { },
    };
    MODULE_DEVICE_TABLE(pci,scx200_tbl);
    static int scx200_probe(struct pci_dev *, const struct pci_device_id *);
    static struct pci_driver scx200_pci_driver = {
    .name = "scx200",
    .id_table = scx200_tbl,
    .probe = scx200_probe,
    };
    static DEFINE_MUTEX(scx200_gpio_config_lock);
#[no_mangle]
unsafe extern "C" fn scx200_init_shadow() {
    static void scx200_init_shadow(void)
    {
    int bank;
// read the current values driven on the GPIO signals
    for (bank = 0; bank < 2; ++bank)
    scx200_gpio_shadow[bank] = inl(scx200_gpio_base + 0x10 * bank);
    }
#[no_mangle]
unsafe extern "C" fn scx200_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int scx200_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    unsigned base;
    if (pdev.device == PCI_DEVICE_ID_NS_SCx200_BRIDGE ||
    pdev.device == PCI_DEVICE_ID_NS_SC1100_BRIDGE) {
    base = pci_resource_start(pdev, 0);
    pr_info("GPIO base 0x%x\n", base);
    if (!request_region(base, SCx200_GPIO_SIZE,
    "NatSemi SCx200 GPIO")) {
    pr_err("can't allocate I/O for GPIOs\n");
    return -EBUSY;
    }
    scx200_gpio_base = base;
    scx200_init_shadow();
    } else {
// find the base of the Configuration Block
    if (scx200_cb_probe(SCx200_CB_BASE_FIXED)) {
    scx200_cb_base = SCx200_CB_BASE_FIXED;
    } else {
    pci_read_config_dword(pdev, SCx200_CBA_SCRATCH, &base);
    if (scx200_cb_probe(base)) {
    scx200_cb_base = base;
    } else {
    pr_warn("Configuration Block not found\n");
    return -ENODEV;
    }
    }
    pr_info("Configuration Block base 0x%x\n", scx200_cb_base);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn scx200_gpio_configure(index: unsigned, mask: u32, bits: u32) -> u32 {
    u32 scx200_gpio_configure(unsigned index, u32 mask, u32 bits)
    {
    u32 config, new_config;
    mutex_lock(&scx200_gpio_config_lock);
    outl(index, scx200_gpio_base + 0x20);
    config = inl(scx200_gpio_base + 0x24);
    new_config = (config & mask) | bits;
    outl(new_config, scx200_gpio_base + 0x24);
    mutex_unlock(&scx200_gpio_config_lock);
    return config;
    }
#[no_mangle]
unsafe extern "C" fn scx200_init() -> int __init {
    static int __init scx200_init(void)
    {
    pr_info("NatSemi SCx200 Driver\n");
    return pci_register_driver(&scx200_pci_driver);
    }
#[no_mangle]
unsafe extern "C" fn scx200_cleanup() -> void __exit {
    static void __exit scx200_cleanup(void)
    {
    pci_unregister_driver(&scx200_pci_driver);
    release_region(scx200_gpio_base, SCx200_GPIO_SIZE);
    }
    module_init(scx200_init);
    module_exit(scx200_cleanup);
    EXPORT_SYMBOL(scx200_gpio_base);
    EXPORT_SYMBOL(scx200_gpio_shadow);
    EXPORT_SYMBOL(scx200_gpio_configure);
    EXPORT_SYMBOL(scx200_cb_base);
