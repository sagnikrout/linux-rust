//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcma/bcma_driver_pci.h
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
pub const BCMA_CORE_PCI_CTL: c_uint = 0x0000	/* PCI Control */;
pub const BCMA_CORE_PCI_CTL_RST_OE: c_uint = 0x00000001 /* PCI_RESET Output Enable */;
pub const BCMA_CORE_PCI_CTL_RST: c_uint = 0x00000002 /* PCI_RESET driven out to pin */;
pub const BCMA_CORE_PCI_CTL_CLK_OE: c_uint = 0x00000004 /* Clock gate Output Enable */;
pub const BCMA_CORE_PCI_CTL_CLK: c_uint = 0x00000008 /* Gate for clock driven out to pin */;
pub const BCMA_CORE_PCI_ARBCTL: c_uint = 0x0010	/* PCI Arbiter Control */;
pub const BCMA_CORE_PCI_ARBCTL_INTERN: c_uint = 0x00000001 /* Use internal arbiter */;
pub const BCMA_CORE_PCI_ARBCTL_EXTERN: c_uint = 0x00000002 /* Use external arbiter */;
pub const BCMA_CORE_PCI_ARBCTL_PARKID: c_uint = 0x00000006 /* Mask, selects which agent is parked on an idle bus */;
pub const BCMA_CORE_PCI_ARBCTL_PARKID_LAST: c_uint = 0x00000000 /* Last requestor */;
pub const BCMA_CORE_PCI_ARBCTL_PARKID_4710: c_uint = 0x00000002 /* 4710 */;
pub const BCMA_CORE_PCI_ARBCTL_PARKID_EXT0: c_uint = 0x00000004 /* External requestor 0 */;
pub const BCMA_CORE_PCI_ARBCTL_PARKID_EXT1: c_uint = 0x00000006 /* External requestor 1 */;
pub const BCMA_CORE_PCI_ISTAT: c_uint = 0x0020	/* Interrupt status */;
pub const BCMA_CORE_PCI_ISTAT_INTA: c_uint = 0x00000001 /* PCI INTA# */;
pub const BCMA_CORE_PCI_ISTAT_INTB: c_uint = 0x00000002 /* PCI INTB# */;
pub const BCMA_CORE_PCI_ISTAT_SERR: c_uint = 0x00000004 /* PCI SERR# (write to clear) */;
pub const BCMA_CORE_PCI_ISTAT_PERR: c_uint = 0x00000008 /* PCI PERR# (write to clear) */;
pub const BCMA_CORE_PCI_ISTAT_PME: c_uint = 0x00000010 /* PCI PME# */;
pub const BCMA_CORE_PCI_IMASK: c_uint = 0x0024	/* Interrupt mask */;
pub const BCMA_CORE_PCI_IMASK_INTA: c_uint = 0x00000001 /* PCI INTA# */;
pub const BCMA_CORE_PCI_IMASK_INTB: c_uint = 0x00000002 /* PCI INTB# */;
pub const BCMA_CORE_PCI_IMASK_SERR: c_uint = 0x00000004 /* PCI SERR# */;
pub const BCMA_CORE_PCI_IMASK_PERR: c_uint = 0x00000008 /* PCI PERR# */;
pub const BCMA_CORE_PCI_IMASK_PME: c_uint = 0x00000010 /* PCI PME# */;
pub const BCMA_CORE_PCI_MBOX: c_uint = 0x0028	/* Backplane to PCI Mailbox */;
pub const BCMA_CORE_PCI_MBOX_F0_0: c_uint = 0x00000100 /* PCI function 0, INT 0 */;
pub const BCMA_CORE_PCI_MBOX_F0_1: c_uint = 0x00000200 /* PCI function 0, INT 1 */;
pub const BCMA_CORE_PCI_MBOX_F1_0: c_uint = 0x00000400 /* PCI function 1, INT 0 */;
pub const BCMA_CORE_PCI_MBOX_F1_1: c_uint = 0x00000800 /* PCI function 1, INT 1 */;
pub const BCMA_CORE_PCI_MBOX_F2_0: c_uint = 0x00001000 /* PCI function 2, INT 0 */;
pub const BCMA_CORE_PCI_MBOX_F2_1: c_uint = 0x00002000 /* PCI function 2, INT 1 */;
pub const BCMA_CORE_PCI_MBOX_F3_0: c_uint = 0x00004000 /* PCI function 3, INT 0 */;
pub const BCMA_CORE_PCI_MBOX_F3_1: c_uint = 0x00008000 /* PCI function 3, INT 1 */;
pub const BCMA_CORE_PCI_BCAST_ADDR: c_uint = 0x0050	/* Backplane Broadcast Address */;
pub const BCMA_CORE_PCI_BCAST_ADDR_MASK: c_uint = 0x000000FF;
pub const BCMA_CORE_PCI_BCAST_DATA: c_uint = 0x0054	/* Backplane Broadcast Data */;
pub const BCMA_CORE_PCI_GPIO_IN: c_uint = 0x0060	/* rev >= 2 only */;
pub const BCMA_CORE_PCI_GPIO_OUT: c_uint = 0x0064	/* rev >= 2 only */;
pub const BCMA_CORE_PCI_GPIO_ENABLE: c_uint = 0x0068	/* rev >= 2 only */;
pub const BCMA_CORE_PCI_GPIO_CTL: c_uint = 0x006C	/* rev >= 2 only */;
pub const BCMA_CORE_PCI_SBTOPCI0: c_uint = 0x0100	/* Backplane to PCI translation 0 (sbtopci0) */;
pub const BCMA_CORE_PCI_SBTOPCI0_MASK: c_uint = 0xFC000000;
pub const BCMA_CORE_PCI_SBTOPCI1: c_uint = 0x0104	/* Backplane to PCI translation 1 (sbtopci1) */;
pub const BCMA_CORE_PCI_SBTOPCI1_MASK: c_uint = 0xFC000000;
pub const BCMA_CORE_PCI_SBTOPCI2: c_uint = 0x0108	/* Backplane to PCI translation 2 (sbtopci2) */;
pub const BCMA_CORE_PCI_SBTOPCI2_MASK: c_uint = 0xC0000000;
pub const BCMA_CORE_PCI_CONFIG_ADDR: c_uint = 0x0120	/* pcie config space access */;
pub const BCMA_CORE_PCI_CONFIG_DATA: c_uint = 0x0124	/* pcie config space access */;
pub const BCMA_CORE_PCI_MDIO_CONTROL: c_uint = 0x0128	/* controls the mdio access */;
pub const BCMA_CORE_PCI_MDIOCTL_DIVISOR_MASK: c_uint = 0x7f	/* clock to be used on MDIO */;
pub const BCMA_CORE_PCI_MDIOCTL_DIVISOR_VAL: c_uint = 0x2;
pub const BCMA_CORE_PCI_MDIOCTL_PREAM_EN: c_uint = 0x80	/* Enable preamble sequnce */;
pub const BCMA_CORE_PCI_MDIOCTL_ACCESS_DONE: c_uint = 0x100	/* Tranaction complete */;
pub const BCMA_CORE_PCI_MDIO_DATA: c_uint = 0x012c	/* Data to the mdio access */;
pub const BCMA_CORE_PCI_MDIODATA_MASK: c_uint = 0x0000ffff /* data 2 bytes */;
pub const BCMA_CORE_PCI_MDIODATA_TA: c_uint = 0x00020000 /* Turnaround */;

pub const BCMA_CORE_PCI_MDIODATA_REGADDR_MASK_OLD: c_uint = 0x003c0000 /* Regaddr Mask (rev < 10) */;

pub const BCMA_CORE_PCI_MDIODATA_DEVADDR_MASK_OLD: c_uint = 0x0fc00000 /* Physmedia devaddr Mask (rev < 10) */;

pub const BCMA_CORE_PCI_MDIODATA_REGADDR_MASK: c_uint = 0x007c0000 /* Regaddr Mask */;

pub const BCMA_CORE_PCI_MDIODATA_DEVADDR_MASK: c_uint = 0x0f800000 /* Physmedia devaddr Mask */;
pub const BCMA_CORE_PCI_MDIODATA_WRITE: c_uint = 0x10000000 /* write Transaction */;
pub const BCMA_CORE_PCI_MDIODATA_READ: c_uint = 0x20000000 /* Read Transaction */;
pub const BCMA_CORE_PCI_MDIODATA_START: c_uint = 0x40000000 /* start of Transaction */;
pub const BCMA_CORE_PCI_MDIODATA_DEV_ADDR: c_uint = 0x0	/* dev address for serdes */;
pub const BCMA_CORE_PCI_MDIODATA_BLK_ADDR: c_uint = 0x1F	/* blk address for serdes */;
pub const BCMA_CORE_PCI_MDIODATA_DEV_PLL: c_uint = 0x1d	/* SERDES PLL Dev */;
pub const BCMA_CORE_PCI_MDIODATA_DEV_TX: c_uint = 0x1e	/* SERDES TX Dev */;
pub const BCMA_CORE_PCI_MDIODATA_DEV_RX: c_uint = 0x1f	/* SERDES RX Dev */;
pub const BCMA_CORE_PCI_PCIEIND_ADDR: c_uint = 0x0130	/* indirect access to the internal register */;
pub const BCMA_CORE_PCI_PCIEIND_DATA: c_uint = 0x0134	/* Data to/from the internal register */;
pub const BCMA_CORE_PCI_CLKREQENCTRL: c_uint = 0x0138	/*  >= rev 6, Clkreq rdma control */;
pub const BCMA_CORE_PCI_PCICFG0: c_uint = 0x0400	/* PCI config space 0 (rev >= 8) */;
pub const BCMA_CORE_PCI_PCICFG1: c_uint = 0x0500	/* PCI config space 1 (rev >= 8) */;
pub const BCMA_CORE_PCI_PCICFG2: c_uint = 0x0600	/* PCI config space 2 (rev >= 8) */;
pub const BCMA_CORE_PCI_PCICFG3: c_uint = 0x0700	/* PCI config space 3 (rev >= 8) */;

pub const BCMA_CORE_PCI_SPROM_PI_MASK: c_uint = 0xf000	/* bit 15:12 */;

pub const BCMA_CORE_PCI_SPROM_L23READY_EXIT_NOPERST: c_uint = 0x8000	/* bit 15 */;

pub const BCMA_CORE_PCI_SPROM_CLKREQ_ENB: c_uint = 0x0800	/* bit 11 */;
// SBtoPCIx
pub const BCMA_CORE_PCI_SBTOPCI_MEM: c_uint = 0x00000000;
pub const BCMA_CORE_PCI_SBTOPCI_IO: c_uint = 0x00000001;
pub const BCMA_CORE_PCI_SBTOPCI_CFG0: c_uint = 0x00000002;
pub const BCMA_CORE_PCI_SBTOPCI_CFG1: c_uint = 0x00000003;
pub const BCMA_CORE_PCI_SBTOPCI_PREF: c_uint = 0x00000004 /* Prefetch enable */;
pub const BCMA_CORE_PCI_SBTOPCI_BURST: c_uint = 0x00000008 /* Burst enable */;
pub const BCMA_CORE_PCI_SBTOPCI_MRM: c_uint = 0x00000020 /* Memory Read Multiple */;
pub const BCMA_CORE_PCI_SBTOPCI_RC: c_uint = 0x00000030 /* Read Command mask (rev >= 11) */;
pub const BCMA_CORE_PCI_SBTOPCI_RC_READ: c_uint = 0x00000000 /* Memory read */;
pub const BCMA_CORE_PCI_SBTOPCI_RC_READL: c_uint = 0x00000010 /* Memory read line */;
pub const BCMA_CORE_PCI_SBTOPCI_RC_READM: c_uint = 0x00000020 /* Memory read multiple */;
// PCIE protocol PHY diagnostic registers
pub const BCMA_CORE_PCI_PLP_MODEREG: c_uint = 0x200	/* Mode */;
pub const BCMA_CORE_PCI_PLP_STATUSREG: c_uint = 0x204	/* Status */;
pub const BCMA_CORE_PCI_PLP_POLARITYINV_STAT: c_uint = 0x10	/* Status reg PCIE_PLP_STATUSREG */;
pub const BCMA_CORE_PCI_PLP_LTSSMCTRLREG: c_uint = 0x208	/* LTSSM control */;
pub const BCMA_CORE_PCI_PLP_LTLINKNUMREG: c_uint = 0x20c	/* Link Training Link number */;
pub const BCMA_CORE_PCI_PLP_LTLANENUMREG: c_uint = 0x210	/* Link Training Lane number */;
pub const BCMA_CORE_PCI_PLP_LTNFTSREG: c_uint = 0x214	/* Link Training N_FTS */;
pub const BCMA_CORE_PCI_PLP_ATTNREG: c_uint = 0x218	/* Attention */;
pub const BCMA_CORE_PCI_PLP_ATTNMASKREG: c_uint = 0x21C	/* Attention Mask */;
pub const BCMA_CORE_PCI_PLP_RXERRCTR: c_uint = 0x220	/* Rx Error */;
pub const BCMA_CORE_PCI_PLP_RXFRMERRCTR: c_uint = 0x224	/* Rx Framing Error */;
pub const BCMA_CORE_PCI_PLP_RXERRTHRESHREG: c_uint = 0x228	/* Rx Error threshold */;
pub const BCMA_CORE_PCI_PLP_TESTCTRLREG: c_uint = 0x22C	/* Test Control reg */;
pub const BCMA_CORE_PCI_PLP_SERDESCTRLOVRDREG: c_uint = 0x230	/* SERDES Control Override */;
pub const BCMA_CORE_PCI_PLP_TIMINGOVRDREG: c_uint = 0x234	/* Timing param override */;
pub const BCMA_CORE_PCI_PLP_RXTXSMDIAGREG: c_uint = 0x238	/* RXTX State Machine Diag */;
pub const BCMA_CORE_PCI_PLP_LTSSMDIAGREG: c_uint = 0x23C	/* LTSSM State Machine Diag */;
// PCIE protocol DLLP diagnostic registers
pub const BCMA_CORE_PCI_DLLP_LCREG: c_uint = 0x100	/* Link Control */;
pub const BCMA_CORE_PCI_DLLP_LSREG: c_uint = 0x104	/* Link Status */;
pub const BCMA_CORE_PCI_DLLP_LAREG: c_uint = 0x108	/* Link Attention */;

pub const BCMA_CORE_PCI_DLLP_LAMASKREG: c_uint = 0x10C	/* Link Attention Mask */;
pub const BCMA_CORE_PCI_DLLP_NEXTTXSEQNUMREG: c_uint = 0x110	/* Next Tx Seq Num */;
pub const BCMA_CORE_PCI_DLLP_ACKEDTXSEQNUMREG: c_uint = 0x114	/* Acked Tx Seq Num */;
pub const BCMA_CORE_PCI_DLLP_PURGEDTXSEQNUMREG: c_uint = 0x118	/* Purged Tx Seq Num */;
pub const BCMA_CORE_PCI_DLLP_RXSEQNUMREG: c_uint = 0x11C	/* Rx Sequence Number */;
pub const BCMA_CORE_PCI_DLLP_LRREG: c_uint = 0x120	/* Link Replay */;
pub const BCMA_CORE_PCI_DLLP_LACKTOREG: c_uint = 0x124	/* Link Ack Timeout */;
pub const BCMA_CORE_PCI_DLLP_PMTHRESHREG: c_uint = 0x128	/* Power Management Threshold */;
pub const BCMA_CORE_PCI_ASPMTIMER_EXTEND: c_uint = 0x01000000 /* > rev7: enable extend ASPM timer */;
pub const BCMA_CORE_PCI_DLLP_RTRYWPREG: c_uint = 0x12C	/* Retry buffer write ptr */;
pub const BCMA_CORE_PCI_DLLP_RTRYRPREG: c_uint = 0x130	/* Retry buffer Read ptr */;
pub const BCMA_CORE_PCI_DLLP_RTRYPPREG: c_uint = 0x134	/* Retry buffer Purged ptr */;
pub const BCMA_CORE_PCI_DLLP_RTRRWREG: c_uint = 0x138	/* Retry buffer Read/Write */;
pub const BCMA_CORE_PCI_DLLP_ECTHRESHREG: c_uint = 0x13C	/* Error Count Threshold */;
pub const BCMA_CORE_PCI_DLLP_TLPERRCTRREG: c_uint = 0x140	/* TLP Error Counter */;
pub const BCMA_CORE_PCI_DLLP_ERRCTRREG: c_uint = 0x144	/* Error Counter */;
pub const BCMA_CORE_PCI_DLLP_NAKRXCTRREG: c_uint = 0x148	/* NAK Received Counter */;
pub const BCMA_CORE_PCI_DLLP_TESTREG: c_uint = 0x14C	/* Test */;
pub const BCMA_CORE_PCI_DLLP_PKTBIST: c_uint = 0x150	/* Packet BIST */;
pub const BCMA_CORE_PCI_DLLP_PCIE11: c_uint = 0x154	/* DLLP PCIE 1.1 reg */;
// SERDES RX registers

pub const BCMA_CORE_PCI_SERDES_RX_CTRL_FORCE: c_uint = 0x80	/* rxpolarity_force */;
pub const BCMA_CORE_PCI_SERDES_RX_CTRL_POLARITY: c_uint = 0x40	/* rxpolarity_value */;

// SERDES PLL registers

pub const BCMA_CORE_PCI_PLL_CTRL_FREQDET_EN: c_uint = 0x4000	/* bit 14 is FREQDET on */;
// PCIcore specific boardflags
pub const BCMA_CORE_PCI_BFL_NOPCI: c_uint = 0x00000400 /* Board leaves PCI floating */;
// PCIE Config space accessing MACROS

pub const BCMA_CORE_PCI_CFG_BUS_MASK: c_uint = 0xff	/* Bus mask */;
pub const BCMA_CORE_PCI_CFG_SLOT_MASK: c_uint = 0x1f	/* Slot/Device mask */;

pub const BCMA_CORE_PCI_CFG_OFF_MASK: c_uint = 0xfff	/* Register mask */;
pub const BCMA_CORE_PCI_CFG_DEVCTRL: c_uint = 0xd8;
// Macro flag: #define BCMA_CORE_PCI_
// MDIO devices (SERDES modules)
pub const BCMA_CORE_PCI_MDIO_IEEE0: c_uint = 0x000;
pub const BCMA_CORE_PCI_MDIO_IEEE1: c_uint = 0x001;
pub const BCMA_CORE_PCI_MDIO_BLK0: c_uint = 0x800;
pub const BCMA_CORE_PCI_MDIO_BLK1: c_uint = 0x801;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT0: c_uint = 0x16;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT1: c_uint = 0x17;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT2: c_uint = 0x18;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT3: c_uint = 0x19;
pub const BCMA_CORE_PCI_MDIO_BLK1_MGMT4: c_uint = 0x1A;
pub const BCMA_CORE_PCI_MDIO_BLK2: c_uint = 0x802;
pub const BCMA_CORE_PCI_MDIO_BLK3: c_uint = 0x803;
pub const BCMA_CORE_PCI_MDIO_BLK4: c_uint = 0x804;
pub const BCMA_CORE_PCI_MDIO_TXPLL: c_uint = 0x808	/* TXPLL register block idx */;
pub const BCMA_CORE_PCI_MDIO_TXCTRL0: c_uint = 0x820;
pub const BCMA_CORE_PCI_MDIO_SERDESID: c_uint = 0x831;
pub const BCMA_CORE_PCI_MDIO_RXCTRL0: c_uint = 0x840;
// PCIE Root Capability Register bits (Host mode only)
pub const BCMA_CORE_PCI_RC_RRS_VISIBILITY: c_uint = 0x0001;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_drv_pci_host {
    pub pdev: *mut bcma_drv_pci,
    pub host_cfg_addr: u32,
    pub cfgspace_lock: spinlock_t,
    pub pci_controller: pci_controller,
    pub pci_ops: pci_ops,
    pub mem_resource: resource,
    pub io_resource: resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_drv_pci {
    pub core: *mut bcma_device,
    pub early_setup_done:1: u8,
    pub setup_done:1: u8,
    pub hostmode:1: u8,

    pub host_controller: *mut bcma_drv_pci_host,

}

// Register access

extern "C" {
    pub fn bcma_core_pci_power_save(bus: *mut bcma_bus, up: bool);
}

extern "C" {
    pub fn bcma_core_pci_pcibios_map_irq(dev: *const pci_dev) -> c_int;
}
extern "C" {
    pub fn bcma_core_pci_plat_dev_init(dev: *mut pci_dev) -> c_int;
}

