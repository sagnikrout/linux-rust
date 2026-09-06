//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/reboot_fixups_32.c
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
//
// This is a good place to put board specific reboot fixups.
//
// List of supported fixups:
// geode-gx1/cs5530a - Jaya Kumar <jayalk@intworks.biz>
// geode-gx/lx/cs5536 - Andres Salomon <dilinger@debian.org>
//

#[no_mangle]
unsafe extern "C" fn cs5530a_warm_reset(dev: *mut pci_dev) {
    static void cs5530a_warm_reset(struct pci_dev *dev)
    {
// writing 1 to the reset control register, 0x44 causes the
    cs5530a to perform a system warm reset */
    pci_write_config_byte(dev, 0x44, 0x1);
    udelay(50); /* shouldn't get here but be safe and spin-a-while */
    return;
    }
#[no_mangle]
unsafe extern "C" fn cs5536_warm_reset(dev: *mut pci_dev) {
    static void cs5536_warm_reset(struct pci_dev *dev)
    {
// writing 1 to the LSB of this MSR causes a hard reset
    wrmsrq(MSR_DIVIL_SOFT_RESET, 1ULL);
    udelay(50); /* shouldn't get here but be safe and spin a while */
    }
#[no_mangle]
unsafe extern "C" fn rdc321x_reset(dev: *mut pci_dev) {
    static void rdc321x_reset(struct pci_dev *dev)
    {
    unsigned i;
// Voluntary reset the watchdog timer
    outl(0x80003840, 0xCF8);
// Generate a CPU reset on next tick
    i = inl(0xCFC);
// Use the minimum timer resolution
    i |= 0x1600;
    outl(i, 0xCFC);
    outb(1, 0x92);
    }
#[no_mangle]
unsafe extern "C" fn ce4100_reset(dev: *mut pci_dev) {
    static void ce4100_reset(struct pci_dev *dev)
    {
    int i;
    for (i = 0; i < 10; i++) {
    outb(0x2, 0xcf9);
    udelay(50);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_fixup {
    pub vendor: c_uint,
    pub device: c_uint,
    pub ): *mut *mut void (reboot_fixup)(struct pci_dev,
}

//
// PCI ids solely used for fixups_table go here
//
pub const PCI_DEVICE_ID_INTEL_CE4100: c_uint = 0x0708;
    static const struct device_fixup fixups_table[] = {
    { PCI_VENDOR_ID_CYRIX, PCI_DEVICE_ID_CYRIX_5530_LEGACY, cs5530a_warm_reset },
    { PCI_VENDOR_ID_AMD, PCI_DEVICE_ID_AMD_CS5536_ISA, cs5536_warm_reset },
    { PCI_VENDOR_ID_NS, PCI_DEVICE_ID_NS_SC1100_BRIDGE, cs5530a_warm_reset },
    { PCI_VENDOR_ID_RDC, PCI_DEVICE_ID_RDC_R6030, rdc321x_reset },
    { PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_CE4100, ce4100_reset },
    };
//
// we see if any fixup is available for our current hardware. if there
// is a fixup, we call it and we expect to never return from it. if we
// do return, we keep looking and then eventually fall back to the
// standard mach_reboot on return.
//
#[no_mangle]
pub unsafe extern "C" fn mach_reboot_fixups() {
    void mach_reboot_fixups(void)
    {
    const struct device_fixup *cur;
    struct pci_dev *dev;
    int i;
// we can be called from sysrq-B code. In such a case it is
// prohibited to dig PCI
    if (in_interrupt())
    return;
    for (i=0; i < ARRAY_SIZE(fixups_table); i++) {
    cur = &(fixups_table[i]);
    dev = pci_get_device(cur.vendor, cur.device, core::ptr::null_mut());
    if (!dev)
    continue;
    cur.reboot_fixup(dev);
    pci_dev_put(dev);
    }
    }
