//! Automatically rewritten from C to Rust
//! Source: drivers/input/gameport/emu10k1-gp.c
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
// Copyright (c) 2001 Vojtech Pavlik
//
// EMU10k1 - SB Live / Audigy - gameport driver for Linux
//

    MODULE_AUTHOR("Vojtech Pavlik <vojtech@ucw.cz>");
    MODULE_DESCRIPTION("EMU10k1 gameport driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emu {
    pub dev: *mut pci_dev,
    pub gameport: *mut gameport,
    pub io: c_int,
    pub size: c_int,
}

    static const struct pci_device_id emu_tbl[] = {
    { 0x1102, 0x7002, PCI_ANY_ID, PCI_ANY_ID }, /* SB Live gameport */
    { 0x1102, 0x7003, PCI_ANY_ID, PCI_ANY_ID }, /* Audigy gameport */
    { 0x1102, 0x7004, PCI_ANY_ID, PCI_ANY_ID }, /* Dell SB Live */
    { 0x1102, 0x7005, PCI_ANY_ID, PCI_ANY_ID }, /* Audigy LS gameport */
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, emu_tbl);
#[no_mangle]
unsafe extern "C" fn emu_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int emu_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    struct emu *emu;
    struct gameport *port;
    int error;
    emu = kzalloc_obj(*emu);
    port = gameport_allocate_port();
    if (!emu || !port) {
    printk(KERN_ERR "emu10k1-gp: Memory allocation failed\n");
    error = -ENOMEM;
    goto err_out_free;
    }
    error = pci_enable_device(pdev);
    if (error)
    goto err_out_free;
    emu.io = pci_resource_start(pdev, 0);
    emu.size = pci_resource_len(pdev, 0);
    emu.dev = pdev;
    emu.gameport = port;
    gameport_set_name(port, "EMU10K1");
    gameport_set_phys(port, "pci%s/gameport0", pci_name(pdev));
    port.dev.parent = &pdev.dev;
    port.io = emu.io;
    if (!request_region(emu.io, emu.size, "emu10k1-gp")) {
    printk(KERN_ERR "emu10k1-gp: unable to grab region 0x%x-0x%x\n",
    emu.io, emu.io + emu.size - 1);
    error = -EBUSY;
    goto err_out_disable_dev;
    }
    pci_set_drvdata(pdev, emu);
    gameport_register_port(port);
    return 0;
    err_out_disable_dev:
    pci_disable_device(pdev);
    err_out_free:
    gameport_free_port(port);
    kfree(emu);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn emu_remove(pdev: *mut pci_dev) {
    static void emu_remove(struct pci_dev *pdev)
    {
    struct emu *emu = pci_get_drvdata(pdev);
    gameport_unregister_port(emu.gameport);
    release_region(emu.io, emu.size);
    kfree(emu);
    pci_disable_device(pdev);
    }
    static struct pci_driver emu_driver = {
    .name =         "Emu10k1_gameport",
    .id_table =     emu_tbl,
    .probe =        emu_probe,
    .remove =	emu_remove,
    };
    module_pci_driver(emu_driver);
