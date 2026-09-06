//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/bvme6000_scsi.c
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
// Detection routine for the NCR53c710 based BVME6000 SCSI Controllers for Linux.
//
// Based on work by Alan Hourihane and Kars de Jong
//
// Rewritten to use 53c700.c by Richard Hirst <richard@sleepie.demon.co.uk>
//

    MODULE_AUTHOR("Richard Hirst <richard@sleepie.demon.co.uk>");
    MODULE_DESCRIPTION("BVME6000 NCR53C710 driver");
    MODULE_LICENSE("GPL");
    static struct scsi_host_template bvme6000_scsi_driver_template = {
    .name			= "BVME6000 NCR53c710 SCSI",
    .proc_name		= "BVME6000",
    .this_id		= 7,
    .module			= THIS_MODULE,
    };
    static struct platform_device *bvme6000_scsi_device;
    static int
    bvme6000_probe(struct platform_device *dev)
    {
    struct Scsi_Host *host;
    struct NCR_700_Host_Parameters *hostdata;
    if (!MACH_IS_BVME6000)
    goto out;
    hostdata = kzalloc_obj(struct NCR_700_Host_Parameters);
    if (!hostdata) {
    printk(KERN_ERR "bvme6000-scsi: "
    "Failed to allocate host data\n");
    goto out;
    }
// Fill in the required pieces of hostdata
    hostdata.base = (void __iomem *)BVME_NCR53C710_BASE;
    hostdata.clock = 40;	/* XXX - depends on the CPU clock! */
    hostdata.chip710 = 1;
    hostdata.dmode_extra = DMODE_FC2;
    hostdata.dcntl_extra = EA_710;
    hostdata.ctest7_extra = CTEST7_TT1;
// and register the chip
    host = NCR_700_detect(&bvme6000_scsi_driver_template, hostdata,
    &dev.dev);
    if (!host) {
    printk(KERN_ERR "bvme6000-scsi: No host detected; "
    "board configuration problem?\n");
    goto out_free;
    }
    host.base = BVME_NCR53C710_BASE;
    host.this_id = 7;
    host.irq = BVME_IRQ_SCSI;
    if (request_irq(BVME_IRQ_SCSI, NCR_700_intr, 0, "bvme6000-scsi",
    host)) {
    printk(KERN_ERR "bvme6000-scsi: request_irq failed\n");
    goto out_put_host;
    }
    platform_set_drvdata(dev, host);
    scsi_scan_host(host);
    return 0;
    out_put_host:
    scsi_host_put(host);
    out_free:
    kfree(hostdata);
    out:
    return -ENODEV;
    }
    static void
    bvme6000_device_remove(struct platform_device *dev)
    {
    struct Scsi_Host *host = platform_get_drvdata(dev);
    struct NCR_700_Host_Parameters *hostdata = shost_priv(host);
    scsi_remove_host(host);
    NCR_700_release(host);
    kfree(hostdata);
    free_irq(host.irq, host);
    }
    static struct platform_driver bvme6000_scsi_driver = {
    .driver = {
    .name		= "bvme6000-scsi",
    },
    .probe		= bvme6000_probe,
    .remove		= bvme6000_device_remove,
    };
#[no_mangle]
unsafe extern "C" fn bvme6000_scsi_init() -> int __init {
    static int __init bvme6000_scsi_init(void)
    {
    int err;
    err = platform_driver_register(&bvme6000_scsi_driver);
    if (err)
    return err;
    bvme6000_scsi_device = platform_device_register_simple("bvme6000-scsi",
    -1, core::ptr::null_mut(), 0);
    if (IS_ERR(bvme6000_scsi_device)) {
    platform_driver_unregister(&bvme6000_scsi_driver);
    return PTR_ERR(bvme6000_scsi_device);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bvme6000_scsi_exit() -> void __exit {
    static void __exit bvme6000_scsi_exit(void)
    {
    platform_device_unregister(bvme6000_scsi_device);
    platform_driver_unregister(&bvme6000_scsi_driver);
    }
    module_init(bvme6000_scsi_init);
    module_exit(bvme6000_scsi_exit);
