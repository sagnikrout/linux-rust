//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/arm/cumana_1.c
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
// Generic Generic NCR5380 driver
//
// Copyright 1995-2002, Russell King
//

    unsigned ctrl
    struct NCR5380_hostdata;
    static u8 cumanascsi_read(struct NCR5380_hostdata *, unsigned int);
    static void cumanascsi_write(struct NCR5380_hostdata *, unsigned int, u8);

pub const CTRL: c_uint = 0x16fc;
pub const STAT: c_uint = 0x2004;

    static inline int cumanascsi_pwrite(struct NCR5380_hostdata *hostdata,
    unsigned char *addr, int len)
    {
    unsigned long *laddr;
    u8 __iomem *base = hostdata.io;
    u8 __iomem *dma = hostdata.pdma_io + 0x2000;
    if(!len) return 0;
    writeb(0x02, base + CTRL);
    laddr = (unsigned long *)addr;
    while(len >= 32)
    {
    unsigned int status;
    unsigned long v;
    status = readb(base + STAT);
    if(status & 0x80)
    goto end;
    if(!(status & 0x40))
    continue;
    v=*laddr++; writew(L(v), dma); writew(H(v), dma);
    v=*laddr++; writew(L(v), dma); writew(H(v), dma);
    v=*laddr++; writew(L(v), dma); writew(H(v), dma);
    v=*laddr++; writew(L(v), dma); writew(H(v), dma);
    v=*laddr++; writew(L(v), dma); writew(H(v), dma);
    v=*laddr++; writew(L(v), dma); writew(H(v), dma);
    v=*laddr++; writew(L(v), dma); writew(H(v), dma);
    v=*laddr++; writew(L(v), dma); writew(H(v), dma);
    len -= 32;
    if(len == 0)
    break;
    }
    addr = (unsigned char *)laddr;
    writeb(0x12, base + CTRL);
    while(len > 0)
    {
    unsigned int status;
    status = readb(base + STAT);
    if(status & 0x80)
    goto end;
    if(status & 0x40)
    {
    writeb(*addr++, dma);
    if(--len == 0)
    break;
    }
    status = readb(base + STAT);
    if(status & 0x80)
    goto end;
    if(status & 0x40)
    {
    writeb(*addr++, dma);
    if(--len == 0)
    break;
    }
    }
    end:
    writeb(hostdata.ctrl | 0x40, base + CTRL);
    if (len)
    return -1;
    return 0;
    }
    static inline int cumanascsi_pread(struct NCR5380_hostdata *hostdata,
    unsigned char *addr, int len)
    {
    unsigned long *laddr;
    u8 __iomem *base = hostdata.io;
    u8 __iomem *dma = hostdata.pdma_io + 0x2000;
    if(!len) return 0;
    writeb(0x00, base + CTRL);
    laddr = (unsigned long *)addr;
    while(len >= 32)
    {
    unsigned int status;
    status = readb(base + STAT);
    if(status & 0x80)
    goto end;
    if(!(status & 0x40))
    continue;
// laddr++ = readw(dma) | (readw(dma) << 16);
    len -= 32;
    if(len == 0)
    break;
    }
    addr = (unsigned char *)laddr;
    writeb(0x10, base + CTRL);
    while(len > 0)
    {
    unsigned int status;
    status = readb(base + STAT);
    if(status & 0x80)
    goto end;
    if(status & 0x40)
    {
// addr++ = readb(dma);
    if(--len == 0)
    break;
    }
    status = readb(base + STAT);
    if(status & 0x80)
    goto end;
    if(status & 0x40)
    {
// addr++ = readb(dma);
    if(--len == 0)
    break;
    }
    }
    end:
    writeb(hostdata.ctrl | 0x40, base + CTRL);
    if (len)
    return -1;
    return 0;
    }
    static int cumanascsi_dma_xfer_len(struct NCR5380_hostdata *hostdata,
    struct scsi_cmnd *cmd)
    {
    return cmd.transfersize;
    }
    static u8 cumanascsi_read(struct NCR5380_hostdata *hostdata,
    unsigned int reg)
    {
    u8 __iomem *base = hostdata.io;
    u8 val;
    writeb(0, base + CTRL);
    val = readb(base + 0x2100 + (reg << 2));
    hostdata.ctrl = 0x40;
    writeb(0x40, base + CTRL);
    return val;
    }
    static void cumanascsi_write(struct NCR5380_hostdata *hostdata,
    unsigned int reg, u8 value)
    {
    u8 __iomem *base = hostdata.io;
    writeb(0, base + CTRL);
    writeb(value, base + 0x2100 + (reg << 2));
    hostdata.ctrl = 0x40;
    writeb(0x40, base + CTRL);
    }

    static const struct scsi_host_template cumanascsi_template = {
    .module			= THIS_MODULE,
    .name			= "Cumana 16-bit SCSI",
    .info			= cumanascsi_info,
    .queuecommand		= cumanascsi_queue_command,
    .eh_abort_handler	= NCR5380_abort,
    .eh_host_reset_handler	= NCR5380_host_reset,
    .can_queue		= 16,
    .this_id		= 7,
    .sg_tablesize		= SG_ALL,
    .cmd_per_lun		= 2,
    .proc_name		= "CumanaSCSI-1",
    .cmd_size		= sizeof(struct NCR5380_cmd),
    .max_sectors		= 128,
    .dma_boundary		= PAGE_SIZE - 1,
    };
    static int cumanascsi1_probe(struct expansion_card *ec,
    const struct ecard_id *id)
    {
    struct Scsi_Host *host;
    int ret;
    ret = ecard_request_resources(ec);
    if (ret)
    goto out;
    host = scsi_host_alloc(&cumanascsi_template, sizeof(struct NCR5380_hostdata));
    if (!host) {
    ret = -ENOMEM;
    goto out_release;
    }
    priv(host).io = ioremap(ecard_resource_start(ec, ECARD_RES_IOCSLOW),
    ecard_resource_len(ec, ECARD_RES_IOCSLOW));
    priv(host).pdma_io = ioremap(ecard_resource_start(ec, ECARD_RES_MEMC),
    ecard_resource_len(ec, ECARD_RES_MEMC));
    if (!priv(host).io || !priv(host).pdma_io) {
    ret = -ENOMEM;
    goto out_unmap;
    }
    host.irq = ec.irq;
    ret = NCR5380_init(host, FLAG_DMA_FIXUP | FLAG_LATE_DMA_SETUP);
    if (ret)
    goto out_unmap;
    NCR5380_maybe_reset_bus(host);
    priv(host).ctrl = 0;
    writeb(0, priv(host).io + CTRL);
    ret = request_irq(host.irq, cumanascsi_intr, 0,
    "CumanaSCSI-1", host);
    if (ret) {
    printk("scsi%d: IRQ%d not free: %d\n",
    host.host_no, host.irq, ret);
    goto out_exit;
    }
    ret = scsi_add_host(host, &ec.dev);
    if (ret)
    goto out_free_irq;
    scsi_scan_host(host);
    goto out;
    out_free_irq:
    free_irq(host.irq, host);
    out_exit:
    NCR5380_exit(host);
    out_unmap:
    iounmap(priv(host).io);
    iounmap(priv(host).pdma_io);
    scsi_host_put(host);
    out_release:
    ecard_release_resources(ec);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cumanascsi1_remove(ec: *mut expansion_card) {
    static void cumanascsi1_remove(struct expansion_card *ec)
    {
    struct Scsi_Host *host = ecard_get_drvdata(ec);
    void __iomem *base = priv(host).io;
    void __iomem *dma = priv(host).pdma_io;
    ecard_set_drvdata(ec, core::ptr::null_mut());
    scsi_remove_host(host);
    free_irq(host.irq, host);
    NCR5380_exit(host);
    scsi_host_put(host);
    iounmap(base);
    iounmap(dma);
    ecard_release_resources(ec);
    }
    static const struct ecard_id cumanascsi1_cids[] = {
    { MANU_CUMANA, PROD_CUMANA_SCSI_1 },
    { 0xffff, 0xffff }
    };
    static struct ecard_driver cumanascsi1_driver = {
    .probe		= cumanascsi1_probe,
    .remove		= cumanascsi1_remove,
    .id_table	= cumanascsi1_cids,
    .drv = {
    .name		= "cumanascsi1",
    },
    };
#[no_mangle]
unsafe extern "C" fn cumanascsi_init() -> int __init {
    static int __init cumanascsi_init(void)
    {
    return ecard_register_driver(&cumanascsi1_driver);
    }
#[no_mangle]
unsafe extern "C" fn cumanascsi_exit() -> void __exit {
    static void __exit cumanascsi_exit(void)
    {
    ecard_remove_driver(&cumanascsi1_driver);
    }
    module_init(cumanascsi_init);
    module_exit(cumanascsi_exit);
    MODULE_DESCRIPTION("Cumana SCSI-1 driver for Acorn machines");
    MODULE_LICENSE("GPL");
