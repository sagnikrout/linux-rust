//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/zorro7xx.c
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
// Detection routine for the NCR53c710 based Amiga SCSI Controllers for Linux.
// Amiga MacroSystemUS WarpEngine SCSI controller.
// Amiga Technologies/DKB A4091 SCSI controller.
//
// Written 1997 by Alan Hourihane <alanh@fairlite.demon.co.uk>
// plus modifications of the 53c7xx.c driver to support the Amiga.
//
// Rewritten to use 53c700.c by Kars de Jong <jongk@linux-m68k.org>
//

    MODULE_AUTHOR("Alan Hourihane <alanh@fairlite.demon.co.uk> / Kars de Jong <jongk@linux-m68k.org>");
    MODULE_DESCRIPTION("Amiga Zorro NCR53C710 driver");
    MODULE_LICENSE("GPL");
    static struct scsi_host_template zorro7xx_scsi_driver_template = {
    .proc_name	= "zorro7xx",
    .this_id	= 7,
    .module		= THIS_MODULE,
    };
    static struct zorro_driver_data {
    const char *name;
    unsigned long offset;
    int absolute;	/* offset is absolute address */
    } zorro7xx_driver_data[] = {
    { .name = "PowerUP 603e+", .offset = 0xf40000, .absolute = 1 },
    { .name = "WarpEngine 40xx", .offset = 0x40000 },
    { .name = "A4091", .offset = 0x800000 },
    { .name = "GForce 040/060", .offset = 0x40000 },
    { 0 }
    };
    static struct zorro_device_id zorro7xx_zorro_tbl[] = {
    {
    .id = ZORRO_PROD_PHASE5_BLIZZARD_603E_PLUS,
    .driver_data_ptr = &zorro7xx_driver_data[0],
    },
    {
    .id = ZORRO_PROD_MACROSYSTEMS_WARP_ENGINE_40xx,
    .driver_data_ptr = &zorro7xx_driver_data[1],
    },
    {
    .id = ZORRO_PROD_CBM_A4091_1,
    .driver_data_ptr = &zorro7xx_driver_data[2],
    },
    {
    .id = ZORRO_PROD_CBM_A4091_2,
    .driver_data_ptr = &zorro7xx_driver_data[2],
    },
    {
    .id = ZORRO_PROD_GVP_GFORCE_040_060,
    .driver_data_ptr = &zorro7xx_driver_data[3],
    },
    { }
    };
    MODULE_DEVICE_TABLE(zorro, zorro7xx_zorro_tbl);
    static int zorro7xx_init_one(struct zorro_dev *z,
    const struct zorro_device_id *ent)
    {
    struct Scsi_Host *host;
    struct NCR_700_Host_Parameters *hostdata;
    const struct zorro_driver_data *zdd;
    unsigned long board, ioaddr;
    board = zorro_resource_start(z);
    zdd = ent.driver_data_ptr;
    if (zdd.absolute) {
    ioaddr = zdd.offset;
    } else {
    ioaddr = board + zdd.offset;
    }
    if (!zorro_request_device(z, zdd.name)) {
    printk(KERN_ERR "zorro7xx: cannot reserve region 0x%lx, abort\n",
    board);
    return -EBUSY;
    }
    hostdata = kzalloc_obj(struct NCR_700_Host_Parameters);
    if (!hostdata) {
    printk(KERN_ERR "zorro7xx: Failed to allocate host data\n");
    goto out_release;
    }
// Fill in the required pieces of hostdata
    if (ioaddr > 0x01000000)
    hostdata.base = ioremap(ioaddr, zorro_resource_len(z));
    else
    hostdata.base = ZTWO_VADDR(ioaddr);
    hostdata.clock = 50;
    hostdata.chip710 = 1;
// Settings for at least WarpEngine 40xx
    hostdata.ctest7_extra = CTEST7_TT1;
    zorro7xx_scsi_driver_template.name = zdd.name;
// and register the chip
    host = NCR_700_detect(&zorro7xx_scsi_driver_template, hostdata,
    &z.dev);
    if (!host) {
    printk(KERN_ERR "zorro7xx: No host detected; "
    "board configuration problem?\n");
    goto out_free;
    }
    host.this_id = 7;
    host.base = ioaddr;
    host.irq = IRQ_AMIGA_PORTS;
    if (request_irq(host.irq, NCR_700_intr, IRQF_SHARED, "zorro7xx-scsi",
    host)) {
    printk(KERN_ERR "zorro7xx: request_irq failed\n");
    goto out_put_host;
    }
    zorro_set_drvdata(z, host);
    scsi_scan_host(host);
    return 0;
    out_put_host:
    scsi_host_put(host);
    out_free:
    if (ioaddr > 0x01000000)
    iounmap(hostdata.base);
    kfree(hostdata);
    out_release:
    zorro_release_device(z);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn zorro7xx_remove_one(z: *mut zorro_dev) {
    static void zorro7xx_remove_one(struct zorro_dev *z)
    {
    struct Scsi_Host *host = zorro_get_drvdata(z);
    struct NCR_700_Host_Parameters *hostdata = shost_priv(host);
    scsi_remove_host(host);
    NCR_700_release(host);
    if (host.base > 0x01000000)
    iounmap(hostdata.base);
    kfree(hostdata);
    free_irq(host.irq, host);
    zorro_release_device(z);
    }
    static struct zorro_driver zorro7xx_driver = {
    .name	  = "zorro7xx-scsi",
    .id_table = zorro7xx_zorro_tbl,
    .probe	  = zorro7xx_init_one,
    .remove	  = zorro7xx_remove_one,
    };
#[no_mangle]
unsafe extern "C" fn zorro7xx_scsi_init() -> int __init {
    static int __init zorro7xx_scsi_init(void)
    {
    return zorro_register_driver(&zorro7xx_driver);
    }
#[no_mangle]
unsafe extern "C" fn zorro7xx_scsi_exit() -> void __exit {
    static void __exit zorro7xx_scsi_exit(void)
    {
    zorro_unregister_driver(&zorro7xx_driver);
    }
    module_init(zorro7xx_scsi_init);
    module_exit(zorro7xx_scsi_exit);
