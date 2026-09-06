//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/fdomain_isa.c
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


// SPDX-License-Identifier: GPL-2.0

pub const MAXBOARDS_PARAM: c_int = 4;
    static int io[MAXBOARDS_PARAM] = { 0, 0, 0, 0 };
    module_param_hw_array(io, int, ioport, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(io, "base I/O address of controller (0x140, 0x150, 0x160, 0x170)");
    static int irq[MAXBOARDS_PARAM] = { 0, 0, 0, 0 };
    module_param_hw_array(irq, int, irq, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(irq, "IRQ of controller (0=auto [default])");
    static int scsi_id[MAXBOARDS_PARAM] = { 0, 0, 0, 0 };
    module_param_hw_array(scsi_id, int, other, core::ptr::null_mut(), 0);
    MODULE_PARM_DESC(scsi_id, "SCSI ID of controller (default = 7)");
    static unsigned long addresses[] = {
    0xc8000,
    0xca000,
    0xce000,
    0xde000,
    };

    static unsigned short ports[] = { 0x140, 0x150, 0x160, 0x170 };

    static unsigned short irqs[] = { 3, 5, 10, 11, 12, 14, 15, 0 };
// This driver works *ONLY* for Future Domain cards using the TMC-1800,
// TMC-18C50, or TMC-18C30 chip.  This includes models TMC-1650, 1660, 1670,
// and 1680. These are all 16-bit cards.
// BIOS versions prior to 3.2 assigned SCSI ID 6 to SCSI adapter.
//
// The following BIOS signature signatures are for boards which do *NOT
// work with this driver (these TMC-8xx and TMC-9xx boards may work with the
// Seagate driver):
//
// FUTURE DOMAIN CORP. (C) 1986-1988 V4.0I 03/16/88
// FUTURE DOMAIN CORP. (C) 1986-1989 V5.0C2/14/89
// FUTURE DOMAIN CORP. (C) 1986-1989 V6.0A7/28/89
// FUTURE DOMAIN CORP. (C) 1986-1990 V6.0105/31/90
// FUTURE DOMAIN CORP. (C) 1986-1990 V6.0209/18/90
// FUTURE DOMAIN CORP. (C) 1986-1990 V7.009/18/90
// FUTURE DOMAIN CORP. (C) 1992 V8.00.004/02/92
//
// (The cards which do *NOT* work are all 8-bit cards -- although some of
// them have a 16-bit form-factor, the upper 8-bits are used only for IRQs
// and are *NOT* used for data. You can tell the difference by following
// the tracings on the circuit board -- if only the IRQ lines are involved,
// you have a "8-bit" card, and should *NOT* use this driver.)
//
    static struct signature {
    const char *signature;
    int offset;
    int length;
    int this_id;
    int base_offset;
    } signatures[] = {
// 1         2         3         4         5         6
// 123456789012345678901234567890123456789012345678901234567890
    { "FUTURE DOMAIN CORP. (C) 1986-1990 1800-V2.07/28/89",	 5, 50,  6, 0x1fcc },
    { "FUTURE DOMAIN CORP. (C) 1986-1990 1800-V1.07/28/89",	 5, 50,  6, 0x1fcc },
    { "FUTURE DOMAIN CORP. (C) 1986-1990 1800-V2.07/28/89", 72, 50,  6, 0x1fa2 },
    { "FUTURE DOMAIN CORP. (C) 1986-1990 1800-V2.0",	73, 43,  6, 0x1fa2 },
    { "FUTURE DOMAIN CORP. (C) 1991 1800-V2.0.",		72, 39,  6, 0x1fa3 },
    { "FUTURE DOMAIN CORP. (C) 1992 V3.00.004/02/92",	 5, 44,  6, 0 },
    { "FUTURE DOMAIN TMC-18XX (C) 1993 V3.203/12/93",	 5, 44,  7, 0 },
    { "IBM F1 P2 BIOS v1.0011/09/92",			 5, 28,  7, 0x1ff3 },
    { "IBM F1 P2 BIOS v1.0104/29/93",			 5, 28,  7, 0 },
    { "Future Domain Corp. V1.0008/18/93",			 5, 33,  7, 0 },
    { "Future Domain Corp. V2.0108/18/93",			 5, 33,  7, 0 },
    { "FUTURE DOMAIN CORP.  V3.5008/18/93",			 5, 34,  7, 0 },
    { "FUTURE DOMAIN 18c30/18c50/1800 (C) 1994 V3.5",	 5, 44,  7, 0 },
    { "FUTURE DOMAIN CORP.  V3.6008/18/93",			 5, 34,  7, 0 },
    { "FUTURE DOMAIN CORP.  V3.6108/18/93",			 5, 34,  7, 0 },
    };

#[no_mangle]
unsafe extern "C" fn fdomain_isa_match(dev: *mut device, ndev: c_uint) -> c_int {
    static int fdomain_isa_match(struct device *dev, unsigned int ndev)
    {
    struct Scsi_Host *sh;
    int i, base = 0, irq = 0;
    let mut bios_base: c_ulong = 0;
    struct signature *sig = core::ptr::null_mut();
    void __iomem *p;
    static struct signature *saved_sig;
    let mut this_id: c_int = 7;
    if (ndev < ADDRESS_COUNT) {	/* scan supported ISA BIOS addresses */
    p = ioremap(addresses[ndev], FDOMAIN_BIOS_SIZE);
    if (!p)
    return 0;
    for (i = 0; i < SIGNATURE_COUNT; i++)
    if (check_signature(p + signatures[i].offset,
    signatures[i].signature,
    signatures[i].length))
    break;
    if (i == SIGNATURE_COUNT)	/* no signature found */
    goto fail_unmap;
    sig = &signatures[i];
    bios_base = addresses[ndev];
// read I/O base from BIOS area
    if (sig.base_offset)
    base = readb(p + sig.base_offset) +
    (readb(p + sig.base_offset + 1) << 8);
    iounmap(p);
    if (base) {
    dev_info(dev, "BIOS at 0x%lx specifies I/O base 0x%x\n",
    bios_base, base);
    } else { /* no I/O base in BIOS area */
    dev_info(dev, "BIOS at 0x%lx\n", bios_base);
// save BIOS signature for later use in port probing
    saved_sig = sig;
    return 0;
    }
    } else	/* scan supported I/O ports */
    base = ports[ndev - ADDRESS_COUNT];
// use saved BIOS signature if present
    if (!sig && saved_sig)
    sig = saved_sig;
    if (!request_region(base, FDOMAIN_REGION_SIZE, "fdomain_isa"))
    return 0;
    irq = irqs[(inb(base + REG_CFG1) & CFG1_IRQ_MASK) >> 1];
    if (sig)
    this_id = sig.this_id;
    sh = fdomain_create(base, irq, this_id, dev);
    if (!sh) {
    release_region(base, FDOMAIN_REGION_SIZE);
    return 0;
    }
    dev_set_drvdata(dev, sh);
    return 1;
    fail_unmap:
    iounmap(p);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fdomain_isa_param_match(dev: *mut device, ndev: c_uint) -> c_int {
    static int fdomain_isa_param_match(struct device *dev, unsigned int ndev)
    {
    struct Scsi_Host *sh;
    let mut irq_: c_int = irq[ndev];
    if (!io[ndev])
    return 0;
    if (!request_region(io[ndev], FDOMAIN_REGION_SIZE, "fdomain_isa")) {
    dev_err(dev, "base 0x%x already in use", io[ndev]);
    return 0;
    }
    if (irq_ <= 0)
    irq_ = irqs[(inb(io[ndev] + REG_CFG1) & CFG1_IRQ_MASK) >> 1];
    sh = fdomain_create(io[ndev], irq_, scsi_id[ndev], dev);
    if (!sh) {
    dev_err(dev, "controller not found at base 0x%x", io[ndev]);
    release_region(io[ndev], FDOMAIN_REGION_SIZE);
    return 0;
    }
    dev_set_drvdata(dev, sh);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn fdomain_isa_remove(dev: *mut device, ndev: c_uint) {
    static void fdomain_isa_remove(struct device *dev, unsigned int ndev)
    {
    struct Scsi_Host *sh = dev_get_drvdata(dev);
    let mut base: c_int = sh.io_port;
    fdomain_destroy(sh);
    release_region(base, FDOMAIN_REGION_SIZE);
    dev_set_drvdata(dev, core::ptr::null_mut());
    }
    static struct isa_driver fdomain_isa_driver = {
    .match		= fdomain_isa_match,
    .remove		= fdomain_isa_remove,
    .driver = {
    .name	= "fdomain_isa",
    .pm	= FDOMAIN_PM_OPS,
    },
    };
#[no_mangle]
unsafe extern "C" fn fdomain_isa_init() -> int __init {
    static int __init fdomain_isa_init(void)
    {
    let mut isa_probe_count: c_int = ADDRESS_COUNT + PORT_COUNT;
    if (io[0]) {	/* use module parameters if present */
    fdomain_isa_driver.match = fdomain_isa_param_match;
    isa_probe_count = MAXBOARDS_PARAM;
    }
    return isa_register_driver(&fdomain_isa_driver, isa_probe_count);
    }
#[no_mangle]
unsafe extern "C" fn fdomain_isa_exit() -> void __exit {
    static void __exit fdomain_isa_exit(void)
    {
    isa_unregister_driver(&fdomain_isa_driver);
    }
    module_init(fdomain_isa_init);
    module_exit(fdomain_isa_exit);
    MODULE_AUTHOR("Ondrej Zary, Rickard E. Faith");
    MODULE_DESCRIPTION("Future Domain TMC-16x0 ISA SCSI driver");
    MODULE_LICENSE("GPL");
