//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/arm/oak.c
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
// Oak Generic NCR5380 driver
//
// Copyright 1995-2002, Russell King
//

    static inline int oakscsi_pwrite(struct NCR5380_hostdata *hostdata,
    unsigned char *addr, int len)
    {
    u8 __iomem *base = hostdata.io;
    printk("writing %p len %d\n",addr, len);
    while(1)
    {
    int status;
    while (((status = readw(base + STAT)) & 0x100)==0);
    }
    return 0;
    }
    static inline int oakscsi_pread(struct NCR5380_hostdata *hostdata,
    unsigned char *addr, int len)
    {
    u8 __iomem *base = hostdata.io;
    printk("reading %p len %d\n", addr, len);
    while(len > 0)
    {
    unsigned int status, timeout;
    unsigned long b;
    timeout = 0x01FFFFFF;
    while (((status = readw(base + STAT)) & 0x100)==0)
    {
    timeout--;
    if(status & 0x200 || !timeout)
    {
    printk("status = %08X\n", status);
    return -1;
    }
    }
    if(len >= 128)
    {
    readsw(base + DATA, addr, 128);
    addr += 128;
    len -= 128;
    }
    else
    {
    b = (unsigned long) readw(base + DATA);
// addr ++ = b;
    len -= 1;
    if(len)
// addr ++ = b>>8;
    len -= 1;
    }
    }
    return 0;
    }

    static const struct scsi_host_template oakscsi_template = {
    .module			= THIS_MODULE,
    .name			= "Oak 16-bit SCSI",
    .info			= oakscsi_info,
    .queuecommand		= oakscsi_queue_command,
    .eh_abort_handler	= NCR5380_abort,
    .eh_host_reset_handler	= NCR5380_host_reset,
    .can_queue		= 16,
    .this_id		= 7,
    .sg_tablesize		= SG_ALL,
    .cmd_per_lun		= 2,
    .dma_boundary		= PAGE_SIZE - 1,
    .proc_name		= "oakscsi",
    .cmd_size		= sizeof(struct NCR5380_cmd),
    .max_sectors		= 128,
    };
#[no_mangle]
unsafe extern "C" fn oakscsi_probe(ec: *mut expansion_card, id: *const ecard_id) -> c_int {
    static int oakscsi_probe(struct expansion_card *ec, const struct ecard_id *id)
    {
    struct Scsi_Host *host;
    int ret;
    ret = ecard_request_resources(ec);
    if (ret)
    goto out;
    host = scsi_host_alloc(&oakscsi_template, sizeof(struct NCR5380_hostdata));
    if (!host) {
    ret = -ENOMEM;
    goto release;
    }
    priv(host).io = ioremap(ecard_resource_start(ec, ECARD_RES_MEMC),
    ecard_resource_len(ec, ECARD_RES_MEMC));
    if (!priv(host).io) {
    ret = -ENOMEM;
    goto unreg;
    }
    host.irq = NO_IRQ;
    ret = NCR5380_init(host, FLAG_DMA_FIXUP | FLAG_LATE_DMA_SETUP);
    if (ret)
    goto out_unmap;
    NCR5380_maybe_reset_bus(host);
    ret = scsi_add_host(host, &ec.dev);
    if (ret)
    goto out_exit;
    scsi_scan_host(host);
    goto out;
    out_exit:
    NCR5380_exit(host);
    out_unmap:
    iounmap(priv(host).io);
    unreg:
    scsi_host_put(host);
    release:
    ecard_release_resources(ec);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn oakscsi_remove(ec: *mut expansion_card) {
    static void oakscsi_remove(struct expansion_card *ec)
    {
    struct Scsi_Host *host = ecard_get_drvdata(ec);
    void __iomem *base = priv(host).io;
    ecard_set_drvdata(ec, core::ptr::null_mut());
    scsi_remove_host(host);
    NCR5380_exit(host);
    scsi_host_put(host);
    iounmap(base);
    ecard_release_resources(ec);
    }
    static const struct ecard_id oakscsi_cids[] = {
    { MANU_OAK, PROD_OAK_SCSI },
    { 0xffff, 0xffff }
    };
    static struct ecard_driver oakscsi_driver = {
    .probe		= oakscsi_probe,
    .remove		= oakscsi_remove,
    .id_table	= oakscsi_cids,
    .drv = {
    .name		= "oakscsi",
    },
    };
#[no_mangle]
unsafe extern "C" fn oakscsi_init() -> int __init {
    static int __init oakscsi_init(void)
    {
    return ecard_register_driver(&oakscsi_driver);
    }
#[no_mangle]
unsafe extern "C" fn oakscsi_exit() -> void __exit {
    static void __exit oakscsi_exit(void)
    {
    ecard_remove_driver(&oakscsi_driver);
    }
    module_init(oakscsi_init);
    module_exit(oakscsi_exit);
    MODULE_AUTHOR("Russell King");
    MODULE_DESCRIPTION("Oak SCSI driver");
    MODULE_LICENSE("GPL");
