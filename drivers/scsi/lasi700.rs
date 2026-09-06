//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/lasi700.c
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
// PARISC LASI driver for the 53c700 chip
//
// Copyright (C) 2001 by James.Bottomley@HansenPartnership.com
// -----------------------------------------------------------------------------
//
// -----------------------------------------------------------------------------
//
// Many thanks to Richard Hirst <rhirst@linuxcare.com> for patiently
// debugging this driver on the parisc architecture and suggesting
// many improvements and bug fixes.
//
// Thanks also go to Linuxcare Inc. for providing several PARISC
// machines for me to debug the driver on.
//

    MODULE_AUTHOR("James Bottomley");
    MODULE_DESCRIPTION("lasi700 SCSI Driver");
    MODULE_LICENSE("GPL");
pub const LASI_700_SVERSION: c_uint = 0x00071;
pub const LASI_710_SVERSION: c_uint = 0x00082;

    .hw_type	= HPHW_FIO,		\
    .sversion	= LASI_700_SVERSION,	\
    .hversion	= HVERSION_ANY_ID,	\
    .hversion_rev	= HVERSION_REV_ANY_ID,	\
    }

    .hw_type	= HPHW_FIO,		\
    .sversion	= LASI_710_SVERSION,	\
    .hversion	= HVERSION_ANY_ID,	\
    .hversion_rev	= HVERSION_REV_ANY_ID,	\
    }
pub const LASI700_CLOCK: c_int = 25;
pub const LASI710_CLOCK: c_int = 40;
pub const LASI_SCSI_CORE_OFFSET: c_uint = 0x100;
    static const struct parisc_device_id lasi700_ids[] __initconst = {
    LASI700_ID_TABLE,
    LASI710_ID_TABLE,
    { 0 }
    };
    static struct scsi_host_template lasi700_template = {
    .name		= "LASI SCSI 53c700",
    .proc_name	= "lasi700",
    .this_id	= 7,
    .module		= THIS_MODULE,
    };
    MODULE_DEVICE_TABLE(parisc, lasi700_ids);
    static int __init
    lasi700_probe(struct parisc_device *dev)
    {
    let mut base: c_ulong = dev.hpa.start + LASI_SCSI_CORE_OFFSET;
    struct NCR_700_Host_Parameters *hostdata;
    struct Scsi_Host *host;
    hostdata = kzalloc_obj(*hostdata);
    if (!hostdata) {
    dev_printk(KERN_ERR, &dev.dev, "Failed to allocate host data\n");
    return -ENOMEM;
    }
    hostdata.dev = &dev.dev;
    dma_set_mask(&dev.dev, DMA_BIT_MASK(32));
    hostdata.base = ioremap(base, 0x100);
    hostdata.differential = 0;
    if (dev.id.sversion == LASI_700_SVERSION) {
    hostdata.clock = LASI700_CLOCK;
    hostdata.force_le_on_be = 1;
    } else {
    hostdata.clock = LASI710_CLOCK;
    hostdata.force_le_on_be = 0;
    hostdata.chip710 = 1;
    hostdata.dmode_extra = DMODE_FC2;
    hostdata.burst_length = 8;
    }
    host = NCR_700_detect(&lasi700_template, hostdata, &dev.dev);
    if (!host)
    goto out_kfree;
    host.this_id = 7;
    host.base = base;
    host.irq = dev.irq;
    if(request_irq(dev.irq, NCR_700_intr, IRQF_SHARED, "lasi700", host)) {
    printk(KERN_ERR "lasi700: request_irq failed!\n");
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
    static void __exit
    lasi700_driver_remove(struct parisc_device *dev)
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
    static struct parisc_driver lasi700_driver __refdata = {
    .name =		"lasi_scsi",
    .id_table =	lasi700_ids,
    .probe =	lasi700_probe,
    .remove =	__exit_p(lasi700_driver_remove),
    };
    static int __init
    lasi700_init(void)
    {
    return register_parisc_driver(&lasi700_driver);
    }
    static void __exit
    lasi700_exit(void)
    {
    unregister_parisc_driver(&lasi700_driver);
    }
    module_init(lasi700_init);
    module_exit(lasi700_exit);
