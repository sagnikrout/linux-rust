//! Automatically rewritten from C to Rust
//! Source: drivers/mtd/maps/l440gx.c
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
// BIOS Flash chip on Intel 440GX board.
//
// Bugs this currently does not work under linuxBIOS.
//

pub const PIIXE_IOBASE_RESOURCE: c_int = 11;
pub const WINDOW_ADDR: c_uint = 0xfff00000;
pub const WINDOW_SIZE: c_uint = 0x00100000;
pub const BUSWIDTH: c_int = 1;
    static u32 iobase;

    static struct mtd_info *mymtd;
// Is this really the vpp port?
    static DEFINE_SPINLOCK(l440gx_vpp_lock);
    static int l440gx_vpp_refcnt;
#[no_mangle]
unsafe extern "C" fn l440gx_set_vpp(map: *mut map_info, vpp: c_int) {
    static void l440gx_set_vpp(struct map_info *map, int vpp)
    {
    unsigned long flags;
    spin_lock_irqsave(&l440gx_vpp_lock, flags);
    if (vpp) {
    if (++l440gx_vpp_refcnt == 1)   /* first nested 'on' */
    outl(inl(VPP_PORT) | 1, VPP_PORT);
    } else {
    if (--l440gx_vpp_refcnt == 0)   /* last nested 'off' */
    outl(inl(VPP_PORT) & ~1, VPP_PORT);
    }
    spin_unlock_irqrestore(&l440gx_vpp_lock, flags);
    }
    static struct map_info l440gx_map = {
    .name = "L440GX BIOS",
    .size = WINDOW_SIZE,
    .bankwidth = BUSWIDTH,
    .phys = WINDOW_ADDR,

// FIXME verify that this is the
// appripriate code for vpp enable/disable
//
    .set_vpp = l440gx_set_vpp

    };
#[no_mangle]
unsafe extern "C" fn init_l440gx() -> int __init {
    static int __init init_l440gx(void)
    {
    struct pci_dev *dev, *pm_dev;
    struct resource *pm_iobase;
    __u16 word;
    dev = pci_get_device(PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_82371AB_0, core::ptr::null_mut());
    pm_dev = pci_get_device(PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_82371AB_3, core::ptr::null_mut());
    pci_dev_put(dev);
    if (!dev || !pm_dev) {
    printk(KERN_NOTICE "L440GX flash mapping: failed to find PIIX4 ISA bridge, cannot continue\n");
    pci_dev_put(pm_dev);
    return -ENODEV;
    }
    l440gx_map.virt = ioremap(WINDOW_ADDR, WINDOW_SIZE);
    if (!l440gx_map.virt) {
    printk(KERN_WARNING "Failed to ioremap L440GX flash region\n");
    pci_dev_put(pm_dev);
    return -ENOMEM;
    }
    simple_map_init(&l440gx_map);
    pr_debug("window_addr = %p\n", l440gx_map.virt);
// Setup the pm iobase resource
// This code should move into some kind of generic bridge
// driver but for the moment I'm content with getting the
// allocation correct.
//
    pm_iobase = &pm_dev.resource[PIIXE_IOBASE_RESOURCE];
    if (!(pm_iobase.flags & IORESOURCE_IO)) {
    pm_iobase.name = "pm iobase";
    pm_iobase.start = 0;
    pm_iobase.end = 63;
    pm_iobase.flags = IORESOURCE_IO;
// Put the current value in the resource
    pci_read_config_dword(pm_dev, 0x40, &iobase);
    iobase &= ~1;
    pm_iobase.start += iobase & ~1;
    pm_iobase.end += iobase & ~1;
    pci_dev_put(pm_dev);
// Allocate the resource region
    if (pci_assign_resource(pm_dev, PIIXE_IOBASE_RESOURCE) != 0) {
    pci_dev_put(dev);
    pci_dev_put(pm_dev);
    printk(KERN_WARNING "Could not allocate pm iobase resource\n");
    iounmap(l440gx_map.virt);
    return -ENXIO;
    }
    }
// Set the iobase
    iobase = pm_iobase.start;
    pci_write_config_dword(pm_dev, 0x40, iobase | 1);
// Set XBCS#
    pci_read_config_word(dev, 0x4e, &word);
    word |= 0x4;
    pci_write_config_word(dev, 0x4e, word);
// Supply write voltage to the chip
    l440gx_set_vpp(&l440gx_map, 1);
// Enable the gate on the WE line
    outb(inb(TRIBUF_PORT) & ~1, TRIBUF_PORT);
    printk(KERN_NOTICE "Enabled WE line to L440GX BIOS flash chip.\n");
    mymtd = do_map_probe("jedec_probe", &l440gx_map);
    if (!mymtd) {
    printk(KERN_NOTICE "JEDEC probe on BIOS chip failed. Using ROM\n");
    mymtd = do_map_probe("map_rom", &l440gx_map);
    }
    if (mymtd) {
    mymtd.owner = THIS_MODULE;
    mtd_device_register(mymtd, core::ptr::null_mut(), 0);
    return 0;
    }
    iounmap(l440gx_map.virt);
    return -ENXIO;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_l440gx() -> void __exit {
    static void __exit cleanup_l440gx(void)
    {
    mtd_device_unregister(mymtd);
    map_destroy(mymtd);
    iounmap(l440gx_map.virt);
    }
    module_init(init_l440gx);
    module_exit(cleanup_l440gx);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_DESCRIPTION("MTD map driver for BIOS chips on Intel L440GX motherboards");
