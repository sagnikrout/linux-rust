//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ssb/ssb_driver_pci.h
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

// PCI core registers.
pub const SSB_PCICORE_CTL: c_uint = 0x0000	/* PCI Control */;
pub const SSB_PCICORE_CTL_RST_OE: c_uint = 0x00000001 /* PCI_RESET Output Enable */;
pub const SSB_PCICORE_CTL_RST: c_uint = 0x00000002 /* PCI_RESET driven out to pin */;
pub const SSB_PCICORE_CTL_CLK_OE: c_uint = 0x00000004 /* Clock gate Output Enable */;
pub const SSB_PCICORE_CTL_CLK: c_uint = 0x00000008 /* Gate for clock driven out to pin */;
pub const SSB_PCICORE_ARBCTL: c_uint = 0x0010	/* PCI Arbiter Control */;
pub const SSB_PCICORE_ARBCTL_INTERN: c_uint = 0x00000001 /* Use internal arbiter */;
pub const SSB_PCICORE_ARBCTL_EXTERN: c_uint = 0x00000002 /* Use external arbiter */;
pub const SSB_PCICORE_ARBCTL_PARKID: c_uint = 0x00000006 /* Mask, selects which agent is parked on an idle bus */;
pub const SSB_PCICORE_ARBCTL_PARKID_LAST: c_uint = 0x00000000 /* Last requestor */;
pub const SSB_PCICORE_ARBCTL_PARKID_4710: c_uint = 0x00000002 /* 4710 */;
pub const SSB_PCICORE_ARBCTL_PARKID_EXT0: c_uint = 0x00000004 /* External requestor 0 */;
pub const SSB_PCICORE_ARBCTL_PARKID_EXT1: c_uint = 0x00000006 /* External requestor 1 */;
pub const SSB_PCICORE_ISTAT: c_uint = 0x0020	/* Interrupt status */;
pub const SSB_PCICORE_ISTAT_INTA: c_uint = 0x00000001 /* PCI INTA# */;
pub const SSB_PCICORE_ISTAT_INTB: c_uint = 0x00000002 /* PCI INTB# */;
pub const SSB_PCICORE_ISTAT_SERR: c_uint = 0x00000004 /* PCI SERR# (write to clear) */;
pub const SSB_PCICORE_ISTAT_PERR: c_uint = 0x00000008 /* PCI PERR# (write to clear) */;
pub const SSB_PCICORE_ISTAT_PME: c_uint = 0x00000010 /* PCI PME# */;
pub const SSB_PCICORE_IMASK: c_uint = 0x0024	/* Interrupt mask */;
pub const SSB_PCICORE_IMASK_INTA: c_uint = 0x00000001 /* PCI INTA# */;
pub const SSB_PCICORE_IMASK_INTB: c_uint = 0x00000002 /* PCI INTB# */;
pub const SSB_PCICORE_IMASK_SERR: c_uint = 0x00000004 /* PCI SERR# */;
pub const SSB_PCICORE_IMASK_PERR: c_uint = 0x00000008 /* PCI PERR# */;
pub const SSB_PCICORE_IMASK_PME: c_uint = 0x00000010 /* PCI PME# */;
pub const SSB_PCICORE_MBOX: c_uint = 0x0028	/* Backplane to PCI Mailbox */;
pub const SSB_PCICORE_MBOX_F0_0: c_uint = 0x00000100 /* PCI function 0, INT 0 */;
pub const SSB_PCICORE_MBOX_F0_1: c_uint = 0x00000200 /* PCI function 0, INT 1 */;
pub const SSB_PCICORE_MBOX_F1_0: c_uint = 0x00000400 /* PCI function 1, INT 0 */;
pub const SSB_PCICORE_MBOX_F1_1: c_uint = 0x00000800 /* PCI function 1, INT 1 */;
pub const SSB_PCICORE_MBOX_F2_0: c_uint = 0x00001000 /* PCI function 2, INT 0 */;
pub const SSB_PCICORE_MBOX_F2_1: c_uint = 0x00002000 /* PCI function 2, INT 1 */;
pub const SSB_PCICORE_MBOX_F3_0: c_uint = 0x00004000 /* PCI function 3, INT 0 */;
pub const SSB_PCICORE_MBOX_F3_1: c_uint = 0x00008000 /* PCI function 3, INT 1 */;
pub const SSB_PCICORE_BCAST_ADDR: c_uint = 0x0050	/* Backplane Broadcast Address */;
pub const SSB_PCICORE_BCAST_ADDR_MASK: c_uint = 0x000000FF;
pub const SSB_PCICORE_BCAST_DATA: c_uint = 0x0054	/* Backplane Broadcast Data */;
pub const SSB_PCICORE_GPIO_IN: c_uint = 0x0060	/* rev >= 2 only */;
pub const SSB_PCICORE_GPIO_OUT: c_uint = 0x0064	/* rev >= 2 only */;
pub const SSB_PCICORE_GPIO_ENABLE: c_uint = 0x0068	/* rev >= 2 only */;
pub const SSB_PCICORE_GPIO_CTL: c_uint = 0x006C	/* rev >= 2 only */;
pub const SSB_PCICORE_SBTOPCI0: c_uint = 0x0100	/* Backplane to PCI translation 0 (sbtopci0) */;
pub const SSB_PCICORE_SBTOPCI0_MASK: c_uint = 0xFC000000;
pub const SSB_PCICORE_SBTOPCI1: c_uint = 0x0104	/* Backplane to PCI translation 1 (sbtopci1) */;
pub const SSB_PCICORE_SBTOPCI1_MASK: c_uint = 0xFC000000;
pub const SSB_PCICORE_SBTOPCI2: c_uint = 0x0108	/* Backplane to PCI translation 2 (sbtopci2) */;
pub const SSB_PCICORE_SBTOPCI2_MASK: c_uint = 0xC0000000;
pub const SSB_PCICORE_PCICFG0: c_uint = 0x0400	/* PCI config space 0 (rev >= 8) */;
pub const SSB_PCICORE_PCICFG1: c_uint = 0x0500	/* PCI config space 1 (rev >= 8) */;
pub const SSB_PCICORE_PCICFG2: c_uint = 0x0600	/* PCI config space 2 (rev >= 8) */;
pub const SSB_PCICORE_PCICFG3: c_uint = 0x0700	/* PCI config space 3 (rev >= 8) */;

// SBtoPCIx
pub const SSB_PCICORE_SBTOPCI_MEM: c_uint = 0x00000000;
pub const SSB_PCICORE_SBTOPCI_IO: c_uint = 0x00000001;
pub const SSB_PCICORE_SBTOPCI_CFG0: c_uint = 0x00000002;
pub const SSB_PCICORE_SBTOPCI_CFG1: c_uint = 0x00000003;
pub const SSB_PCICORE_SBTOPCI_PREF: c_uint = 0x00000004 /* Prefetch enable */;
pub const SSB_PCICORE_SBTOPCI_BURST: c_uint = 0x00000008 /* Burst enable */;
pub const SSB_PCICORE_SBTOPCI_MRM: c_uint = 0x00000020 /* Memory Read Multiple */;
pub const SSB_PCICORE_SBTOPCI_RC: c_uint = 0x00000030 /* Read Command mask (rev >= 11) */;
pub const SSB_PCICORE_SBTOPCI_RC_READ: c_uint = 0x00000000 /* Memory read */;
pub const SSB_PCICORE_SBTOPCI_RC_READL: c_uint = 0x00000010 /* Memory read line */;
pub const SSB_PCICORE_SBTOPCI_RC_READM: c_uint = 0x00000020 /* Memory read multiple */;
// PCIcore specific boardflags
pub const SSB_PCICORE_BFL_NOPCI: c_uint = 0x00000400 /* Board leaves PCI floating */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_pcicore {
    pub dev: *mut ssb_device,
    pub setup_done:1: u8,
    pub hostmode:1: u8,
    pub cardbusmode:1: u8,
}

extern "C" {
    pub fn ssb_pcicore_init(pc: *mut ssb_pcicore);
}
// Enable IRQ routing for a specific device
extern "C" {
    pub fn ssb_pcicore_plat_dev_init(d: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn ssb_pcicore_pcibios_map_irq(dev: *const pci_dev, slot: u8, pin: u8) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssb_pcicore {
}

