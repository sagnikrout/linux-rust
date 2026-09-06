//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/sni_53c710.c
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
// SNI RM driver
//
// Copyright (C) 2001 by James.Bottomley@HansenPartnership.com
// -----------------------------------------------------------------------------
//
// -----------------------------------------------------------------------------
//
// Based on lasi700.c
//

    MODULE_AUTHOR("Thomas Bogendörfer");
    MODULE_DESCRIPTION("SNI RM 53c710 SCSI Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:snirm_53c710");
pub const SNIRM710_CLOCK: c_int = 32;
    static struct scsi_host_template snirm710_template = {
    .name		= "SNI RM SCSI 53c710",
    .proc_name	= "snirm_53c710",
    .this_id	= 7,
    .module		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn snirm710_probe(dev: *mut platform_device) -> c_int {
    static int snirm710_probe(struct platform_device *dev)
    {
    unsigned long base;
    struct NCR_700_Host_Parameters *hostdata;
    struct Scsi_Host *host;
    struct  resource *res;
    int rc;
    res = platform_get_resource(dev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    base = res.start;
    hostdata = kzalloc_obj(*hostdata);
    if (!hostdata)
    return -ENOMEM;
    hostdata.dev = &dev.dev;
    dma_set_mask(&dev.dev, DMA_BIT_MASK(32));
    hostdata.base = ioremap(base, 0x100);
    hostdata.differential = 0;
    hostdata.clock = SNIRM710_CLOCK;
    hostdata.force_le_on_be = 1;
    hostdata.chip710 = 1;
    hostdata.burst_length = 4;
    host = NCR_700_detect(&snirm710_template, hostdata, &dev.dev);
    if (!host)
    goto out_kfree;
    host.this_id = 7;
    host.base = base;
    host.irq = rc = platform_get_irq(dev, 0);
    if (rc < 0)
    goto out_put_host;
    if(request_irq(host.irq, NCR_700_intr, IRQF_SHARED, "snirm710", host)) {
    printk(KERN_ERR "snirm710: request_irq failed!\n");
    goto out_put_host;
    }
    dev_set_drvdata(&dev.dev, host);
    scsi_scan_host(host);
    return 0;
    out_put_host:
    scsi_host_put(host);
    out_kfree:
    iounmap(hostdata.base);
    kfree(hostdata);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn snirm710_driver_remove(dev: *mut platform_device) {
    static void snirm710_driver_remove(struct platform_device *dev)
    {
    struct Scsi_Host *host = dev_get_drvdata(&dev.dev);
    struct NCR_700_Host_Parameters *hostdata =
    (struct NCR_700_Host_Parameters *)host.hostdata[0];
    scsi_remove_host(host);
    NCR_700_release(host);
    free_irq(host.irq, host);
    iounmap(hostdata.base);
    kfree(hostdata);
    }
    static struct platform_driver snirm710_driver = {
    .probe	= snirm710_probe,
    .remove	= snirm710_driver_remove,
    .driver	= {
    .name	= "snirm_53c710",
    },
    };
    module_platform_driver(snirm710_driver);
