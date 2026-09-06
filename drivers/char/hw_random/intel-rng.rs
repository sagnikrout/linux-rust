//! Automatically rewritten from C to Rust
//! Source: drivers/char/hw_random/intel-rng.c
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


//
// RNG driver for Intel RNGs
//
// Copyright 2005 (c) MontaVista Software, Inc.
//
// with the majority of the code coming from:
//
// Hardware driver for the Intel/AMD/VIA Random Number Generators (RNG)
// (c) Copyright 2003 Red Hat Inc <jgarzik@redhat.com>
//
// derived from
//
// Hardware driver for the AMD 768 Random Number Generator (RNG)
// (c) Copyright 2001 Red Hat Inc
//
// derived from
//
// Hardware driver for Intel i810 Random Number Generator (RNG)
// Copyright 2000,2001 Jeff Garzik <jgarzik@pobox.com>
// Copyright 2000,2001 Philipp Rumpf <prumpf@mandrakesoft.com>
//
// This file is licensed under  the terms of the GNU General Public
// License version 2. This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

//
// RNG registers
//
pub const INTEL_RNG_HW_STATUS: c_int = 0;
pub const INTEL_RNG_PRESENT: c_uint = 0x40;
pub const INTEL_RNG_ENABLED: c_uint = 0x01;
pub const INTEL_RNG_STATUS: c_int = 1;
pub const INTEL_RNG_DATA_PRESENT: c_uint = 0x01;
pub const INTEL_RNG_DATA: c_int = 2;
//
// Magic address at which Intel PCI bridges locate the RNG
//
pub const INTEL_RNG_ADDR: c_uint = 0xFFBC015F;
pub const INTEL_RNG_ADDR_LEN: c_int = 3;
//
// LPC bridge PCI config space registers
//
pub const FWH_DEC_EN1_REG_OLD: c_uint = 0xe3;
pub const FWH_DEC_EN1_REG_NEW: c_uint = 0xd9 /* high byte of 16-bit register */;
pub const FWH_F8_EN_MASK: c_uint = 0x80;
pub const BIOS_CNTL_REG_OLD: c_uint = 0x4e;
pub const BIOS_CNTL_REG_NEW: c_uint = 0xdc;
pub const BIOS_CNTL_WRITE_ENABLE_MASK: c_uint = 0x01;
pub const BIOS_CNTL_LOCK_ENABLE_MASK: c_uint = 0x02;
//
// Magic address at which Intel Firmware Hubs get accessed
//
pub const INTEL_FWH_ADDR: c_uint = 0xffff0000;
pub const INTEL_FWH_ADDR_LEN: c_int = 2;
//
// Intel Firmware Hub command codes (write to any address inside the device)
//
pub const INTEL_FWH_RESET_CMD: c_uint = 0xff /* aka READ_ARRAY */;
pub const INTEL_FWH_READ_ID_CMD: c_uint = 0x90;
//
// Intel Firmware Hub Read ID command result addresses
//
pub const INTEL_FWH_MANUFACTURER_CODE_ADDRESS: c_uint = 0x000000;
pub const INTEL_FWH_DEVICE_CODE_ADDRESS: c_uint = 0x000001;
//
// Intel Firmware Hub Read ID command result values
//
pub const INTEL_FWH_MANUFACTURER_CODE: c_uint = 0x89;
pub const INTEL_FWH_DEVICE_CODE_8M: c_uint = 0xac;
pub const INTEL_FWH_DEVICE_CODE_4M: c_uint = 0xad;
//
// Data for PCI driver interface
//
// This data only exists for exporting the supported
// PCI ids via MODULE_DEVICE_TABLE.  We do not actually
// register a pci_driver, because someone else might one day
// want to register another driver on the same PCI id.
//
    static const struct pci_device_id pci_tbl[] = {
// AA
    { PCI_DEVICE(0x8086, 0x2418) }, */
    { PCI_DEVICE(0x8086, 0x2410) }, /* AA */
// AB
    { PCI_DEVICE(0x8086, 0x2428) }, */
    { PCI_DEVICE(0x8086, 0x2420) }, /* AB */
// ??
    { PCI_DEVICE(0x8086, 0x2430) }, */
// BAM, CAM, DBM, FBM, GxM
    { PCI_DEVICE(0x8086, 0x2448) }, */
    { PCI_DEVICE(0x8086, 0x244c) }, /* BAM */
    { PCI_DEVICE(0x8086, 0x248c) }, /* CAM */
    { PCI_DEVICE(0x8086, 0x24cc) }, /* DBM */
    { PCI_DEVICE(0x8086, 0x2641) }, /* FBM */
    { PCI_DEVICE(0x8086, 0x27b9) }, /* GxM */
    { PCI_DEVICE(0x8086, 0x27bd) }, /* GxM DH */
// BA, CA, DB, Ex, 6300, Fx, 631x/632x, Gx
    { PCI_DEVICE(0x8086, 0x244e) }, */
    { PCI_DEVICE(0x8086, 0x2440) }, /* BA */
    { PCI_DEVICE(0x8086, 0x2480) }, /* CA */
    { PCI_DEVICE(0x8086, 0x24c0) }, /* DB */
    { PCI_DEVICE(0x8086, 0x24d0) }, /* Ex */
    { PCI_DEVICE(0x8086, 0x25a1) }, /* 6300 */
    { PCI_DEVICE(0x8086, 0x2640) }, /* Fx */
    { PCI_DEVICE(0x8086, 0x2670) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2671) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2672) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2673) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2674) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2675) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2676) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2677) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2678) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x2679) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x267a) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x267b) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x267c) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x267d) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x267e) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x267f) }, /* 631x/632x */
    { PCI_DEVICE(0x8086, 0x27b8) }, /* Gx */
// E
    { PCI_DEVICE(0x8086, 0x245e) }, */
    { PCI_DEVICE(0x8086, 0x2450) }, /* E  */
    { 0, },	/* terminate list */
    };
    MODULE_DEVICE_TABLE(pci, pci_tbl);
    static __initdata int no_fwh_detect;
    module_param(no_fwh_detect, int, 0);
    MODULE_PARM_DESC(no_fwh_detect, "Skip FWH detection:\n"
    " positive value - skip if FWH space locked read-only\n"
    " negative value - skip always");
#[no_mangle]
pub unsafe extern "C" fn hwstatus_get(mem: *mut void __iomem) -> u8 {
    static inline u8 hwstatus_get(void __iomem *mem)
    {
    return readb(mem + INTEL_RNG_HW_STATUS);
    }
    static inline u8 hwstatus_set(void __iomem *mem,
    u8 hw_status)
    {
    writeb(hw_status, mem + INTEL_RNG_HW_STATUS);
    return hwstatus_get(mem);
    }
#[no_mangle]
unsafe extern "C" fn intel_rng_data_present(rng: *mut hwrng, wait: c_int) -> c_int {
    static int intel_rng_data_present(struct hwrng *rng, int wait)
    {
    void __iomem *mem = (void __iomem *)rng.priv;
    int data, i;
    for (i = 0; i < 20; i++) {
    data = !!(readb(mem + INTEL_RNG_STATUS) &
    INTEL_RNG_DATA_PRESENT);
    if (data || !wait)
    break;
    udelay(10);
    }
    return data;
    }
#[no_mangle]
unsafe extern "C" fn intel_rng_data_read(rng: *mut hwrng, data: *mut u32) -> c_int {
    static int intel_rng_data_read(struct hwrng *rng, u32 *data)
    {
    void __iomem *mem = (void __iomem *)rng.priv;
// data = readb(mem + INTEL_RNG_DATA);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn intel_rng_init(rng: *mut hwrng) -> c_int {
    static int intel_rng_init(struct hwrng *rng)
    {
    void __iomem *mem = (void __iomem *)rng.priv;
    u8 hw_status;
    let mut err: c_int = -EIO;
    hw_status = hwstatus_get(mem);
// turn RNG h/w on, if it's off
    if ((hw_status & INTEL_RNG_ENABLED) == 0)
    hw_status = hwstatus_set(mem, hw_status | INTEL_RNG_ENABLED);
    if ((hw_status & INTEL_RNG_ENABLED) == 0) {
    pr_err(PFX "cannot enable RNG, aborting\n");
    goto out;
    }
    err = 0;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn intel_rng_cleanup(rng: *mut hwrng) {
    static void intel_rng_cleanup(struct hwrng *rng)
    {
    void __iomem *mem = (void __iomem *)rng.priv;
    u8 hw_status;
    hw_status = hwstatus_get(mem);
    if (hw_status & INTEL_RNG_ENABLED)
    hwstatus_set(mem, hw_status & ~INTEL_RNG_ENABLED);
    else
    pr_warn(PFX "unusual: RNG already disabled\n");
    }
    static struct hwrng intel_rng = {
    .name		= "intel",
    .init		= intel_rng_init,
    .cleanup	= intel_rng_cleanup,
    .data_present	= intel_rng_data_present,
    .data_read	= intel_rng_data_read,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_rng_hw {
    pub dev: *mut pci_dev,
    pub mem: *mut void __iomem,
    pub bios_cntl_off: u8,
    pub bios_cntl_val: u8,
    pub fwh_dec_en1_off: u8,
    pub fwh_dec_en1_val: u8,
}

#[no_mangle]
unsafe extern "C" fn intel_rng_hw_init(_intel_rng_hw: *mut c_void) -> int __init {
    static int __init intel_rng_hw_init(void *_intel_rng_hw)
    {
    struct intel_rng_hw *intel_rng_hw = _intel_rng_hw;
    u8 mfc, dvc;
// interrupts disabled in stop_machine call
    if (!(intel_rng_hw.fwh_dec_en1_val & FWH_F8_EN_MASK))
    pci_write_config_byte(intel_rng_hw.dev,
    intel_rng_hw.fwh_dec_en1_off,
    intel_rng_hw.fwh_dec_en1_val |
    FWH_F8_EN_MASK);
    if (!(intel_rng_hw.bios_cntl_val & BIOS_CNTL_WRITE_ENABLE_MASK))
    pci_write_config_byte(intel_rng_hw.dev,
    intel_rng_hw.bios_cntl_off,
    intel_rng_hw.bios_cntl_val |
    BIOS_CNTL_WRITE_ENABLE_MASK);
    writeb(INTEL_FWH_RESET_CMD, intel_rng_hw.mem);
    writeb(INTEL_FWH_READ_ID_CMD, intel_rng_hw.mem);
    mfc = readb(intel_rng_hw.mem + INTEL_FWH_MANUFACTURER_CODE_ADDRESS);
    dvc = readb(intel_rng_hw.mem + INTEL_FWH_DEVICE_CODE_ADDRESS);
    writeb(INTEL_FWH_RESET_CMD, intel_rng_hw.mem);
    if (!(intel_rng_hw.bios_cntl_val &
    (BIOS_CNTL_LOCK_ENABLE_MASK|BIOS_CNTL_WRITE_ENABLE_MASK)))
    pci_write_config_byte(intel_rng_hw.dev,
    intel_rng_hw.bios_cntl_off,
    intel_rng_hw.bios_cntl_val);
    if (!(intel_rng_hw.fwh_dec_en1_val & FWH_F8_EN_MASK))
    pci_write_config_byte(intel_rng_hw.dev,
    intel_rng_hw.fwh_dec_en1_off,
    intel_rng_hw.fwh_dec_en1_val);
    if (mfc != INTEL_FWH_MANUFACTURER_CODE ||
    (dvc != INTEL_FWH_DEVICE_CODE_8M &&
    dvc != INTEL_FWH_DEVICE_CODE_4M)) {
    pr_notice(PFX "FWH not detected\n");
    return -ENODEV;
    }
    return 0;
    }
    static int __init intel_init_hw_struct(struct intel_rng_hw *intel_rng_hw,
    struct pci_dev *dev)
    {
    intel_rng_hw.bios_cntl_val = 0xff;
    intel_rng_hw.fwh_dec_en1_val = 0xff;
    intel_rng_hw.dev = dev;
// Check for Intel 82802
    if (dev.device < 0x2640) {
    intel_rng_hw.fwh_dec_en1_off = FWH_DEC_EN1_REG_OLD;
    intel_rng_hw.bios_cntl_off = BIOS_CNTL_REG_OLD;
    } else {
    intel_rng_hw.fwh_dec_en1_off = FWH_DEC_EN1_REG_NEW;
    intel_rng_hw.bios_cntl_off = BIOS_CNTL_REG_NEW;
    }
    pci_read_config_byte(dev, intel_rng_hw.fwh_dec_en1_off,
    &intel_rng_hw.fwh_dec_en1_val);
    pci_read_config_byte(dev, intel_rng_hw.bios_cntl_off,
    &intel_rng_hw.bios_cntl_val);
    if ((intel_rng_hw.bios_cntl_val &
    (BIOS_CNTL_LOCK_ENABLE_MASK|BIOS_CNTL_WRITE_ENABLE_MASK))
    == BIOS_CNTL_LOCK_ENABLE_MASK) {
    static __initdata /*const*/ char warning[] =
    PFX "Firmware space is locked read-only. If you can't or\n"
    PFX "don't want to disable this in firmware setup, and if\n"
    PFX "you are certain that your system has a functional\n"
    PFX "RNG, try using the 'no_fwh_detect' option.\n";
    if (no_fwh_detect)
    return -ENODEV;
    pr_warn("%s", warning);
    return -EBUSY;
    }
    intel_rng_hw.mem = ioremap(INTEL_FWH_ADDR, INTEL_FWH_ADDR_LEN);
    if (intel_rng_hw.mem == core::ptr::null_mut())
    return -EBUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn intel_rng_mod_init() -> int __init {
    static int __init intel_rng_mod_init(void)
    {
    let mut err: c_int = -ENODEV;
    int i;
    struct pci_dev *dev = core::ptr::null_mut();
    void __iomem *mem;
    u8 hw_status;
    struct intel_rng_hw *intel_rng_hw;
    for (i = 0; !dev && pci_tbl[i].vendor; ++i)
    dev = pci_get_device(pci_tbl[i].vendor, pci_tbl[i].device,
    core::ptr::null_mut());
    if (!dev)
    goto out; /* Device not found. */
    if (no_fwh_detect < 0) {
    pci_dev_put(dev);
    goto fwh_done;
    }
    intel_rng_hw = kmalloc_obj(*intel_rng_hw);
    if (!intel_rng_hw) {
    pci_dev_put(dev);
    goto out;
    }
    err = intel_init_hw_struct(intel_rng_hw, dev);
    if (err) {
    pci_dev_put(dev);
    kfree(intel_rng_hw);
    if (err == -ENODEV)
    goto fwh_done;
    goto out;
    }
//
// Since the BIOS code/data is going to disappear from its normal
// location with the Read ID command, all activity on the system
// must be stopped until the state is back to normal.
//
// Use stop_machine because IPIs can be blocked by disabling
// interrupts.
//
    err = stop_machine(intel_rng_hw_init, intel_rng_hw, core::ptr::null_mut());
    pci_dev_put(dev);
    iounmap(intel_rng_hw.mem);
    kfree(intel_rng_hw);
    if (err)
    goto out;
    fwh_done:
    err = -ENOMEM;
    mem = ioremap(INTEL_RNG_ADDR, INTEL_RNG_ADDR_LEN);
    if (!mem)
    goto out;
    intel_rng.priv = (unsigned long)mem;
// Check for Random Number Generator
    err = -ENODEV;
    hw_status = hwstatus_get(mem);
    if ((hw_status & INTEL_RNG_PRESENT) == 0) {
    iounmap(mem);
    goto out;
    }
    pr_info("Intel 82802 RNG detected\n");
    err = hwrng_register(&intel_rng);
    if (err) {
    pr_err(PFX "RNG registering failed (%d)\n",
    err);
    iounmap(mem);
    }
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn intel_rng_mod_exit() -> void __exit {
    static void __exit intel_rng_mod_exit(void)
    {
    void __iomem *mem = (void __iomem *)intel_rng.priv;
    hwrng_unregister(&intel_rng);
    iounmap(mem);
    }
    module_init(intel_rng_mod_init);
    module_exit(intel_rng_mod_exit);
    MODULE_DESCRIPTION("H/W RNG driver for Intel chipsets");
    MODULE_LICENSE("GPL");
