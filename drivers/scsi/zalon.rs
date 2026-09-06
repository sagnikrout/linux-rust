//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/zalon.c
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
// Zalon 53c7xx device driver.
// By Richard Hirst (rhirst@linuxcare.com)
//

    MODULE_AUTHOR("Richard Hirst");
    MODULE_DESCRIPTION("Bluefish/Zalon 720 SCSI Driver");
    MODULE_LICENSE("GPL");
pub const GSC_SCSI_ZALON_OFFSET: c_uint = 0x800;

pub const IOSTATUS_RY: c_uint = 0x40;
pub const IOSTATUS_FE: c_uint = 0x80;
pub const IOIIDATA_SMINT5L: c_uint = 0x40000000;
pub const IOIIDATA_MINT5EN: c_uint = 0x20000000;
pub const IOIIDATA_PACKEN: c_uint = 0x10000000;
pub const IOIIDATA_PREFETCHEN: c_uint = 0x08000000;
pub const IOIIDATA_IOII: c_uint = 0x00000020;
pub const CMD_RESET: c_int = 5;
    static struct ncr_chip zalon720_chip __initdata = {
    .revision_id =	0x0f,
    .burst_max =	3,
    .offset_max =	8,
    .nr_divisor =	4,
    .features =	FE_WIDE | FE_DIFF | FE_EHP| FE_MUX | FE_EA,
    };

// FIXME:
// Is this function dead code? or is someone planning on using it in the
// future.  The clock = (int) pdc_result[16] does not look correct to
// me ... I think it should be iodc_data[16].  Since this cause a compile
// error with the new encapsulated PDC, I'm not compiling in this function.
// - RB
//
// poke SCSI clock out of iodc data
    static u8 iodc_data[32] __attribute__ ((aligned (64)));
    static unsigned long pdc_result[32] __attribute__ ((aligned (16))) ={0,0,0,0};
    static int
    lasi_scsi_clock(void * hpa, int defaultclock)
    {
    int clock, status;
    status = pdc_iodc_read(&pdc_result, hpa, 0, &iodc_data, 32 );
    if (status == PDC_RET_OK) {
    clock = (int) pdc_result[16];
    } else {
    printk(KERN_WARNING "%s: pdc_iodc_read returned %d\n", __func__, status);
    clock = defaultclock;
    }
    printk(KERN_DEBUG "%s: SCSI clock %d\n", __func__, clock);
    return clock;
    }

    static struct scsi_host_template zalon7xx_template = {
    .module		= THIS_MODULE,
    .proc_name	= "zalon7xx",
    .cmd_size	= sizeof(struct ncr_cmd_priv),
    };
    static int __init
    zalon_probe(struct parisc_device *dev)
    {
    struct gsc_irq gsc_irq;
    u32 zalon_vers;
    let mut error: c_int = -ENODEV;
    void __iomem *zalon = ioremap(dev.hpa.start, 4096);
    void __iomem *io_port = zalon + GSC_SCSI_ZALON_OFFSET;
    let mut unit: static int = 0;
    struct Scsi_Host *host;
    struct ncr_device device;
    __raw_writel(CMD_RESET, zalon + IO_MODULE_IO_COMMAND);
    while (!(__raw_readl(zalon + IO_MODULE_IO_STATUS) & IOSTATUS_RY))
    cpu_relax();
    __raw_writel(IOIIDATA_MINT5EN | IOIIDATA_PACKEN | IOIIDATA_PREFETCHEN,
    zalon + IO_MODULE_II_CDATA);
// XXX: Save the Zalon version for bug workarounds?
    zalon_vers = (__raw_readl(zalon + IO_MODULE_II_CDATA) >> 24) & 0x07;
// Setup the interrupts first.
// Later on request_irq() will register the handler.
//
    dev.irq = gsc_alloc_irq(&gsc_irq);
    printk(KERN_INFO "%s: Zalon version %d, IRQ %d\n", __func__,
    zalon_vers, dev.irq);
    __raw_writel(gsc_irq.txn_addr | gsc_irq.txn_data, zalon + IO_MODULE_EIM);
    if (zalon_vers == 0)
    printk(KERN_WARNING "%s: Zalon 1.1 or earlier\n", __func__);
    memset(&device, 0, sizeof(struct ncr_device));
// The following three are needed before any other access.
    __raw_writeb(0x20, io_port + 0x38); /* DCNTL_REG,  EA  */
    __raw_writeb(0x04, io_port + 0x1b); /* CTEST0_REG, EHP */
    __raw_writeb(0x80, io_port + 0x22); /* CTEST4_REG, MUX */
// Initialise ncr_device structure with items required by ncr_attach.
    device.chip		= zalon720_chip;
    device.host_id		= 7;
    device.dev		= &dev.dev;
    device.slot.base	= dev.hpa.start + GSC_SCSI_ZALON_OFFSET;
    device.slot.base_v	= io_port;
    device.slot.irq		= dev.irq;
    device.differential	= 2;
    host = ncr_attach(&zalon7xx_template, unit, &device);
    if (!host)
    return -ENODEV;
    if (request_irq(dev.irq, ncr53c8xx_intr, IRQF_SHARED, "zalon", host)) {
    dev_printk(KERN_ERR, &dev.dev, "irq problem with %d, detaching\n",
    dev.irq);
    goto fail;
    }
    unit++;
    dev_set_drvdata(&dev.dev, host);
    error = scsi_add_host(host, &dev.dev);
    if (error)
    goto fail_free_irq;
    scsi_scan_host(host);
    return 0;
    fail_free_irq:
    free_irq(dev.irq, host);
    fail:
    ncr53c8xx_release(host);
    return error;
    }
    static const struct parisc_device_id zalon_tbl[] __initconst = {
    { HPHW_A_DMA, HVERSION_REV_ANY_ID, HVERSION_ANY_ID, 0x00089 },
    { 0, }
    };
    MODULE_DEVICE_TABLE(parisc, zalon_tbl);
#[no_mangle]
unsafe extern "C" fn zalon_remove(dev: *mut parisc_device) -> void __exit {
    static void __exit zalon_remove(struct parisc_device *dev)
    {
    struct Scsi_Host *host = dev_get_drvdata(&dev.dev);
    scsi_remove_host(host);
    ncr53c8xx_release(host);
    free_irq(dev.irq, host);
    }
    static struct parisc_driver zalon_driver __refdata = {
    .name =		"zalon",
    .id_table =	zalon_tbl,
    .probe =	zalon_probe,
    .remove =	__exit_p(zalon_remove),
    };
#[no_mangle]
unsafe extern "C" fn zalon7xx_init() -> int __init {
    static int __init zalon7xx_init(void)
    {
    let mut ret: c_int = ncr53c8xx_init();
    if (!ret)
    ret = register_parisc_driver(&zalon_driver);
    if (ret)
    ncr53c8xx_exit();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn zalon7xx_exit() -> void __exit {
    static void __exit zalon7xx_exit(void)
    {
    unregister_parisc_driver(&zalon_driver);
    ncr53c8xx_exit();
    }
    module_init(zalon7xx_init);
    module_exit(zalon7xx_exit);
