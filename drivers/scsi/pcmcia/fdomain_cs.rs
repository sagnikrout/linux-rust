//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/pcmcia/fdomain_cs.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MPL-1.1)
//
// Driver for Future Domain-compatible PCMCIA SCSI cards
// Copyright 2019 Ondrej Zary
//
// The initial developer of the original code is David A. Hinds
// <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
// are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
//

    MODULE_AUTHOR("Ondrej Zary, David Hinds");
    MODULE_DESCRIPTION("Future Domain PCMCIA SCSI driver");
    MODULE_LICENSE("Dual MPL/GPL");
#[no_mangle]
unsafe extern "C" fn fdomain_config_check(p_dev: *mut pcmcia_device, priv_data: *mut c_void) -> c_int {
    static int fdomain_config_check(struct pcmcia_device *p_dev, void *priv_data)
    {
    p_dev.io_lines = 10;
    p_dev.resource[0].end = FDOMAIN_REGION_SIZE;
    p_dev.resource[0].flags &= ~IO_DATA_PATH_WIDTH;
    p_dev.resource[0].flags |= IO_DATA_PATH_WIDTH_AUTO;
    return pcmcia_request_io(p_dev);
    }
#[no_mangle]
unsafe extern "C" fn fdomain_probe(link: *mut pcmcia_device) -> c_int {
    static int fdomain_probe(struct pcmcia_device *link)
    {
    int ret;
    struct Scsi_Host *sh;
    link.config_flags |= CONF_ENABLE_IRQ | CONF_AUTO_SET_IO;
    link.config_regs = PRESENT_OPTION;
    ret = pcmcia_loop_config(link, fdomain_config_check, core::ptr::null_mut());
    if (ret)
    return ret;
    ret = pcmcia_enable_device(link);
    if (ret)
    goto fail_disable;
    if (!request_region(link.resource[0].start, FDOMAIN_REGION_SIZE,
    "fdomain_cs")) {
    ret = -EBUSY;
    goto fail_disable;
    }
    sh = fdomain_create(link.resource[0].start, link.irq, 7, &link.dev);
    if (!sh) {
    dev_err(&link.dev, "Controller initialization failed");
    ret = -ENODEV;
    goto fail_release;
    }
    link.priv = sh;
    return 0;
    fail_release:
    release_region(link.resource[0].start, FDOMAIN_REGION_SIZE);
    fail_disable:
    pcmcia_disable_device(link);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fdomain_remove(link: *mut pcmcia_device) {
    static void fdomain_remove(struct pcmcia_device *link)
    {
    fdomain_destroy(link.priv);
    release_region(link.resource[0].start, FDOMAIN_REGION_SIZE);
    pcmcia_disable_device(link);
    }
    static const struct pcmcia_device_id fdomain_ids[] = {
    PCMCIA_DEVICE_PROD_ID12("IBM Corp.", "SCSI PCMCIA Card", 0xe3736c88,
    0x859cad20),
    PCMCIA_DEVICE_PROD_ID1("SCSI PCMCIA Adapter Card", 0x8dacb57e),
    PCMCIA_DEVICE_PROD_ID12(" SIMPLE TECHNOLOGY Corporation",
    "SCSI PCMCIA Credit Card Controller",
    0x182bdafe, 0xc80d106f),
    PCMCIA_DEVICE_NULL,
    };
    MODULE_DEVICE_TABLE(pcmcia, fdomain_ids);
    static struct pcmcia_driver fdomain_cs_driver = {
    .owner		= THIS_MODULE,
    .name		= "fdomain_cs",
    .probe		= fdomain_probe,
    .remove		= fdomain_remove,
    .id_table       = fdomain_ids,
    };
    module_pcmcia_driver(fdomain_cs_driver);
