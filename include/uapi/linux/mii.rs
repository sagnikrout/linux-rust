//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mii.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// linux/mii.h: definitions for MII-compatible transceivers
// Originally drivers/net/sunhme.h.
//
// Copyright (C) 1996, 1999, 2001 David S. Miller (davem@redhat.com)
//

// Generic MII registers.
pub const MII_BMCR: c_uint = 0x00	/* Basic mode control register */;
pub const MII_BMSR: c_uint = 0x01	/* Basic mode status register  */;
pub const MII_PHYSID1: c_uint = 0x02	/* PHYS ID 1                   */;
pub const MII_PHYSID2: c_uint = 0x03	/* PHYS ID 2                   */;
pub const MII_ADVERTISE: c_uint = 0x04	/* Advertisement control reg   */;
pub const MII_LPA: c_uint = 0x05	/* Link partner ability reg    */;
pub const MII_EXPANSION: c_uint = 0x06	/* Expansion register          */;
pub const MII_CTRL1000: c_uint = 0x09	/* 1000BASE-T control          */;
pub const MII_STAT1000: c_uint = 0x0a	/* 1000BASE-T status           */;
pub const MII_MMD_CTRL: c_uint = 0x0d	/* MMD Access Control Register */;
pub const MII_MMD_DATA: c_uint = 0x0e	/* MMD Access Data Register */;
pub const MII_ESTATUS: c_uint = 0x0f	/* Extended Status             */;
pub const MII_DCOUNTER: c_uint = 0x12	/* Disconnect counter          */;
pub const MII_FCSCOUNTER: c_uint = 0x13	/* False carrier counter       */;
pub const MII_NWAYTEST: c_uint = 0x14	/* N-way auto-neg test reg     */;
pub const MII_RERRCOUNTER: c_uint = 0x15	/* Receive error counter       */;
pub const MII_SREVISION: c_uint = 0x16	/* Silicon revision            */;
pub const MII_RESV1: c_uint = 0x17	/* Reserved...                 */;
pub const MII_LBRERROR: c_uint = 0x18	/* Lpback, rx, bypass error    */;
pub const MII_PHYADDR: c_uint = 0x19	/* PHY address                 */;
pub const MII_RESV2: c_uint = 0x1a	/* Reserved...                 */;
pub const MII_TPISTATUS: c_uint = 0x1b	/* TPI status for 10mbps       */;
pub const MII_NCONFIG: c_uint = 0x1c	/* Network interface config    */;
// Basic mode control register.
pub const BMCR_RESV: c_uint = 0x003f	/* Unused...                   */;
pub const BMCR_SPEED1000: c_uint = 0x0040	/* MSB of Speed (1000)         */;
pub const BMCR_CTST: c_uint = 0x0080	/* Collision test              */;
pub const BMCR_FULLDPLX: c_uint = 0x0100	/* Full duplex                 */;
pub const BMCR_ANRESTART: c_uint = 0x0200	/* Auto negotiation restart    */;
pub const BMCR_ISOLATE: c_uint = 0x0400	/* Isolate data paths from MII */;
pub const BMCR_PDOWN: c_uint = 0x0800	/* Enable low power state      */;
pub const BMCR_ANENABLE: c_uint = 0x1000	/* Enable auto negotiation     */;
pub const BMCR_SPEED100: c_uint = 0x2000	/* Select 100Mbps              */;
pub const BMCR_LOOPBACK: c_uint = 0x4000	/* TXD loopback bits           */;
pub const BMCR_RESET: c_uint = 0x8000	/* Reset to default state      */;
pub const BMCR_SPEED10: c_uint = 0x0000	/* Select 10Mbps               */;
// Basic mode status register.
pub const BMSR_ERCAP: c_uint = 0x0001	/* Ext-reg capability          */;
pub const BMSR_JCD: c_uint = 0x0002	/* Jabber detected             */;
pub const BMSR_LSTATUS: c_uint = 0x0004	/* Link status                 */;
pub const BMSR_ANEGCAPABLE: c_uint = 0x0008	/* Able to do auto-negotiation */;
pub const BMSR_RFAULT: c_uint = 0x0010	/* Remote fault detected       */;
pub const BMSR_ANEGCOMPLETE: c_uint = 0x0020	/* Auto-negotiation complete   */;
pub const BMSR_RESV: c_uint = 0x00c0	/* Unused...                   */;
pub const BMSR_ESTATEN: c_uint = 0x0100	/* Extended Status in R15      */;
pub const BMSR_100HALF2: c_uint = 0x0200	/* Can do 100BASE-T2 HDX       */;
pub const BMSR_100FULL2: c_uint = 0x0400	/* Can do 100BASE-T2 FDX       */;
pub const BMSR_10HALF: c_uint = 0x0800	/* Can do 10mbps, half-duplex  */;
pub const BMSR_10FULL: c_uint = 0x1000	/* Can do 10mbps, full-duplex  */;
pub const BMSR_100HALF: c_uint = 0x2000	/* Can do 100mbps, half-duplex */;
pub const BMSR_100FULL: c_uint = 0x4000	/* Can do 100mbps, full-duplex */;
pub const BMSR_100BASE4: c_uint = 0x8000	/* Can do 100mbps, 4k packets  */;
// Advertisement control register.
pub const ADVERTISE_SLCT: c_uint = 0x001f	/* Selector bits               */;
pub const ADVERTISE_CSMA: c_uint = 0x0001	/* Only selector supported     */;
pub const ADVERTISE_10HALF: c_uint = 0x0020	/* Try for 10mbps half-duplex  */;
pub const ADVERTISE_1000XFULL: c_uint = 0x0020	/* Try for 1000BASE-X full-duplex */;
pub const ADVERTISE_10FULL: c_uint = 0x0040	/* Try for 10mbps full-duplex  */;
pub const ADVERTISE_1000XHALF: c_uint = 0x0040	/* Try for 1000BASE-X half-duplex */;
pub const ADVERTISE_100HALF: c_uint = 0x0080	/* Try for 100mbps half-duplex */;
pub const ADVERTISE_1000XPAUSE: c_uint = 0x0080	/* Try for 1000BASE-X pause    */;
pub const ADVERTISE_100FULL: c_uint = 0x0100	/* Try for 100mbps full-duplex */;
pub const ADVERTISE_1000XPSE_ASYM: c_uint = 0x0100	/* Try for 1000BASE-X asym pause */;
pub const ADVERTISE_100BASE4: c_uint = 0x0200	/* Try for 100mbps 4k packets  */;
pub const ADVERTISE_PAUSE_CAP: c_uint = 0x0400	/* Try for pause               */;
pub const ADVERTISE_PAUSE_ASYM: c_uint = 0x0800	/* Try for asymetric pause     */;
pub const ADVERTISE_XNP: c_uint = 0x1000  /* Extended Next Page */;

pub const ADVERTISE_RFAULT: c_uint = 0x2000	/* Say we can detect faults    */;
pub const ADVERTISE_LPACK: c_uint = 0x4000	/* Ack link partners response  */;
pub const ADVERTISE_NPAGE: c_uint = 0x8000	/* Next page bit               */;

// Link partner ability register.
pub const LPA_SLCT: c_uint = 0x001f	/* Same as advertise selector  */;
pub const LPA_10HALF: c_uint = 0x0020	/* Can do 10mbps half-duplex   */;
pub const LPA_1000XFULL: c_uint = 0x0020	/* Can do 1000BASE-X full-duplex */;
pub const LPA_10FULL: c_uint = 0x0040	/* Can do 10mbps full-duplex   */;
pub const LPA_1000XHALF: c_uint = 0x0040	/* Can do 1000BASE-X half-duplex */;
pub const LPA_100HALF: c_uint = 0x0080	/* Can do 100mbps half-duplex  */;
pub const LPA_1000XPAUSE: c_uint = 0x0080	/* Can do 1000BASE-X pause     */;
pub const LPA_100FULL: c_uint = 0x0100	/* Can do 100mbps full-duplex  */;
pub const LPA_1000XPAUSE_ASYM: c_uint = 0x0100	/* Can do 1000BASE-X pause asym*/;
pub const LPA_100BASE4: c_uint = 0x0200	/* Can do 100mbps 4k packets   */;
pub const LPA_PAUSE_CAP: c_uint = 0x0400	/* Can pause                   */;
pub const LPA_PAUSE_ASYM: c_uint = 0x0800	/* Can pause asymetrically     */;
pub const LPA_RESV: c_uint = 0x1000	/* Unused...                   */;
pub const LPA_RFAULT: c_uint = 0x2000	/* Link partner faulted        */;
pub const LPA_LPACK: c_uint = 0x4000	/* Link partner acked us       */;
pub const LPA_NPAGE: c_uint = 0x8000	/* Next page bit               */;

// Expansion register for auto-negotiation.
pub const EXPANSION_NWAY: c_uint = 0x0001	/* Can do N-way auto-nego      */;
pub const EXPANSION_LCWP: c_uint = 0x0002	/* Got new RX page code word   */;
pub const EXPANSION_ENABLENPAGE: c_uint = 0x0004	/* This enables npage words    */;
pub const EXPANSION_NPCAPABLE: c_uint = 0x0008	/* Link partner supports npage */;
pub const EXPANSION_MFAULTS: c_uint = 0x0010	/* Multiple faults detected    */;
pub const EXPANSION_RESV: c_uint = 0xffe0	/* Unused...                   */;
pub const ESTATUS_1000_XFULL: c_uint = 0x8000	/* Can do 1000BaseX Full       */;
pub const ESTATUS_1000_XHALF: c_uint = 0x4000	/* Can do 1000BaseX Half       */;
pub const ESTATUS_1000_TFULL: c_uint = 0x2000	/* Can do 1000BT Full          */;
pub const ESTATUS_1000_THALF: c_uint = 0x1000	/* Can do 1000BT Half          */;
// N-way test register.
pub const NWAYTEST_RESV1: c_uint = 0x00ff	/* Unused...                   */;
pub const NWAYTEST_LOOPBACK: c_uint = 0x0100	/* Enable loopback for N-way   */;
pub const NWAYTEST_RESV2: c_uint = 0xfe00	/* Unused...                   */;
// MAC and PHY tx_config_Reg[15:0] for SGMII in-band auto-negotiation.
pub const ADVERTISE_SGMII: c_uint = 0x0001	/* MAC can do SGMII            */;
pub const LPA_SGMII: c_uint = 0x0001	/* PHY can do SGMII            */;
pub const LPA_SGMII_SPD_MASK: c_uint = 0x0c00	/* SGMII speed mask            */;
pub const LPA_SGMII_FULL_DUPLEX: c_uint = 0x1000	/* SGMII full duplex           */;
pub const LPA_SGMII_DPX_SPD_MASK: c_uint = 0x1C00	/* SGMII duplex and speed bits */;
pub const LPA_SGMII_10: c_uint = 0x0000	/* 10Mbps                      */;
pub const LPA_SGMII_10HALF: c_uint = 0x0000	/* Can do 10mbps half-duplex   */;
pub const LPA_SGMII_10FULL: c_uint = 0x1000	/* Can do 10mbps full-duplex   */;
pub const LPA_SGMII_100: c_uint = 0x0400	/* 100Mbps                     */;
pub const LPA_SGMII_100HALF: c_uint = 0x0400	/* Can do 100mbps half-duplex  */;
pub const LPA_SGMII_100FULL: c_uint = 0x1400	/* Can do 100mbps full-duplex  */;
pub const LPA_SGMII_1000: c_uint = 0x0800	/* 1000Mbps                    */;
pub const LPA_SGMII_1000HALF: c_uint = 0x0800	/* Can do 1000mbps half-duplex */;
pub const LPA_SGMII_1000FULL: c_uint = 0x1800	/* Can do 1000mbps full-duplex */;
pub const LPA_SGMII_LINK: c_uint = 0x8000	/* PHY link with copper-side partner */;
// 1000BASE-T Control register
pub const ADVERTISE_1000FULL: c_uint = 0x0200  /* Advertise 1000BASE-T full duplex */;
pub const ADVERTISE_1000HALF: c_uint = 0x0100  /* Advertise 1000BASE-T half duplex */;
pub const CTL1000_PREFER_MASTER: c_uint = 0x0400  /* prefer to operate as master */;
pub const CTL1000_AS_MASTER: c_uint = 0x0800;
pub const CTL1000_ENABLE_MASTER: c_uint = 0x1000;
// 1000BASE-T Status register
pub const LPA_1000MSFAIL: c_uint = 0x8000	/* Master/Slave resolution failure */;
pub const LPA_1000MSRES: c_uint = 0x4000	/* Master/Slave resolution status */;
pub const LPA_1000LOCALRXOK: c_uint = 0x2000	/* Link partner local receiver status */;
pub const LPA_1000REMRXOK: c_uint = 0x1000	/* Link partner remote receiver status */;
pub const LPA_1000FULL: c_uint = 0x0800	/* Link partner 1000BASE-T full duplex */;
pub const LPA_1000HALF: c_uint = 0x0400	/* Link partner 1000BASE-T half duplex */;
// Flow control flags
pub const FLOW_CTRL_TX: c_uint = 0x01;
pub const FLOW_CTRL_RX: c_uint = 0x02;
// MMD Access Control register fields
pub const MII_MMD_CTRL_DEVAD_MASK: c_uint = 0x1f	/* Mask MMD DEVAD*/;
pub const MII_MMD_CTRL_ADDR: c_uint = 0x0000	/* Address */;
pub const MII_MMD_CTRL_NOINCR: c_uint = 0x4000	/* no post increment */;
pub const MII_MMD_CTRL_INCR_RDWT: c_uint = 0x8000	/* post increment on reads & writes */;
pub const MII_MMD_CTRL_INCR_ON_WT: c_uint = 0xC000	/* post increment on writes only */;
// This structure is used in all SIOCxMIIxxx ioctl calls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mii_ioctl_data {
    pub phy_id: __u16,
    pub reg_num: __u16,
    pub val_in: __u16,
    pub val_out: __u16,
}
