//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/natsemi/ns83820.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

// ns83820.c by Benjamin LaHaise with contributions.
//
// Questions/comments/discussion to linux-ns83820@kvack.org.
//
// $Revision: 1.34.2.23 $
//
// Copyright 2001 Benjamin LaHaise.
// Copyright 2001, 2002 Red Hat.
//
// Mmmm, chocolate vanilla mocha...
//
// ChangeLog
// =========
// 20010414	0.1 - created
// 20010622	0.2 - basic rx and tx.
// 20010711	0.3 - added duplex and link state detection support.
// 20010713	0.4 - zero copy, no hangs.
// 0.5 - 64 bit dma support (davem will hate me for this)
// - disable jumbo frames to avoid tx hangs
// - work around tx deadlocks on my 1.02 card via
// fiddling with TXCFG
// 20010810	0.6 - use pci dma api for ringbuffers, work on ia64
// 20010816	0.7 - misc cleanups
// 20010826	0.8 - fix critical zero copy bugs
// 0.9 - internal experiment
// 20010827	0.10 - fix ia64 unaligned access.
// 20010906	0.11 - accept all packets with checksum errors as
// otherwise fragments get lost
// - fix >> 32 bugs
// 0.12 - add statistics counters
// - add allmulti/promisc support
// 20011009	0.13 - hotplug support, other smaller pci api cleanups
// 20011204	0.13a - optical transceiver support added
// by Michael Clark <michael@metaparadigm.com>
// 20011205	0.13b - call register_netdev earlier in initialization
// suppress duplicate link status messages
// 20011117 	0.14 - ethtool GDRVINFO, GLINK support from jgarzik
// 20011204 	0.15	get ppc (big endian) working
// 20011218	0.16	various cleanups
// 20020310	0.17	speedups
// 20020610	0.18 -	actually use the pci dma api for highmem
// -	remove pci latency register fiddling
// 0.19 -	better bist support
// -	add ihr and reset_phy parameters
// -	gmii bus probing
// -	fix missed txok introduced during performance
// tuning
// 0.20 -	fix stupid RFEN thinko.  i am such a smurf.
// 20040828	0.21 -	add hardware vlan accleration
// by Neil Horman <nhorman@redhat.com>
// 20050406	0.22 -	improved DAC ifdefs from Andi Kleen
// -	removal of dead code from Adrian Bunk
// -	fix half duplex collision behaviour
// Driver Overview
// ===============
//
// This driver was originally written for the National Semiconductor
// 83820 chip, a 10/100/1000 Mbps 64 bit PCI ethernet NIC.  Hopefully
// this code will turn out to be a) clean, b) correct, and c) fast.
// With that in mind, I'm aiming to split the code up as much as
// reasonably possible.  At present there are X major sections that
// break down into a) packet receive, b) packet transmit, c) link
// management, d) initialization and configuration.  Where possible,
// these code paths are designed to run in parallel.
//
// This driver has been tested and found to work with the following
// cards (in no particular order):
//
// Cameo		SOHO-GA2000T	SOHO-GA2500T
// D-Link		DGE-500T
// PureData	PDP8023Z-TG
// SMC		SMC9452TX	SMC9462TX
// Netgear		GA621
//
// Special thanks to SMC for providing hardware to test this driver on.
//
// Reports of success or failure would be greatly appreciated.
//
// #define dprintk		printk

// Global parameters.  See module_param near the bottom.
    let mut ihr: static int = 2;
    let mut reset_phy: static int = 0;
    static int lnksts = 0;		/* CFG_LNKSTS bit polarity */
// Dprintk is used for more interesting debug events

// tunables

// Macro flag: #define NS83820_VLAN_ACCEL_SUPPORT

// Must not exceed ~65000.
pub const NR_RX_DESC: c_int = 64;
pub const NR_TX_DESC: c_int = 128;
// not tunable

pub const MIN_TX_DESC_FREE: c_int = 8;
// register defines
pub const CFGCS: c_uint = 0x04;
pub const CR_TXE: c_uint = 0x00000001;
pub const CR_TXD: c_uint = 0x00000002;
// Ramit : Here's a tip, don't do a RXD immediately followed by an RXE
// The Receive engine skips one descriptor and moves
// onto the next one!!
pub const CR_RXE: c_uint = 0x00000004;
pub const CR_RXD: c_uint = 0x00000008;
pub const CR_TXR: c_uint = 0x00000010;
pub const CR_RXR: c_uint = 0x00000020;
pub const CR_SWI: c_uint = 0x00000080;
pub const CR_RST: c_uint = 0x00000100;
pub const PTSCR_EEBIST_FAIL: c_uint = 0x00000001;
pub const PTSCR_EEBIST_EN: c_uint = 0x00000002;
pub const PTSCR_EELOAD_EN: c_uint = 0x00000004;
pub const PTSCR_RBIST_FAIL: c_uint = 0x000001b8;
pub const PTSCR_RBIST_DONE: c_uint = 0x00000200;
pub const PTSCR_RBIST_EN: c_uint = 0x00000400;
pub const PTSCR_RBIST_RST: c_uint = 0x00002000;
pub const MEAR_EEDI: c_uint = 0x00000001;
pub const MEAR_EEDO: c_uint = 0x00000002;
pub const MEAR_EECLK: c_uint = 0x00000004;
pub const MEAR_EESEL: c_uint = 0x00000008;
pub const MEAR_MDIO: c_uint = 0x00000010;
pub const MEAR_MDDIR: c_uint = 0x00000020;
pub const MEAR_MDC: c_uint = 0x00000040;
pub const ISR_TXDESC3: c_uint = 0x40000000;
pub const ISR_TXDESC2: c_uint = 0x20000000;
pub const ISR_TXDESC1: c_uint = 0x10000000;
pub const ISR_TXDESC0: c_uint = 0x08000000;
pub const ISR_RXDESC3: c_uint = 0x04000000;
pub const ISR_RXDESC2: c_uint = 0x02000000;
pub const ISR_RXDESC1: c_uint = 0x01000000;
pub const ISR_RXDESC0: c_uint = 0x00800000;
pub const ISR_TXRCMP: c_uint = 0x00400000;
pub const ISR_RXRCMP: c_uint = 0x00200000;
pub const ISR_DPERR: c_uint = 0x00100000;
pub const ISR_SSERR: c_uint = 0x00080000;
pub const ISR_RMABT: c_uint = 0x00040000;
pub const ISR_RTABT: c_uint = 0x00020000;
pub const ISR_RXSOVR: c_uint = 0x00010000;
pub const ISR_HIBINT: c_uint = 0x00008000;
pub const ISR_PHY: c_uint = 0x00004000;
pub const ISR_PME: c_uint = 0x00002000;
pub const ISR_SWI: c_uint = 0x00001000;
pub const ISR_MIB: c_uint = 0x00000800;
pub const ISR_TXURN: c_uint = 0x00000400;
pub const ISR_TXIDLE: c_uint = 0x00000200;
pub const ISR_TXERR: c_uint = 0x00000100;
pub const ISR_TXDESC: c_uint = 0x00000080;
pub const ISR_TXOK: c_uint = 0x00000040;
pub const ISR_RXORN: c_uint = 0x00000020;
pub const ISR_RXIDLE: c_uint = 0x00000010;
pub const ISR_RXEARLY: c_uint = 0x00000008;
pub const ISR_RXERR: c_uint = 0x00000004;
pub const ISR_RXDESC: c_uint = 0x00000002;
pub const ISR_RXOK: c_uint = 0x00000001;
pub const TXCFG_CSI: c_uint = 0x80000000;
pub const TXCFG_HBI: c_uint = 0x40000000;
pub const TXCFG_MLB: c_uint = 0x20000000;
pub const TXCFG_ATP: c_uint = 0x10000000;
pub const TXCFG_ECRETRY: c_uint = 0x00800000;
pub const TXCFG_BRST_DIS: c_uint = 0x00080000;
pub const TXCFG_MXDMA1024: c_uint = 0x00000000;
pub const TXCFG_MXDMA512: c_uint = 0x00700000;
pub const TXCFG_MXDMA256: c_uint = 0x00600000;
pub const TXCFG_MXDMA128: c_uint = 0x00500000;
pub const TXCFG_MXDMA64: c_uint = 0x00400000;
pub const TXCFG_MXDMA32: c_uint = 0x00300000;
pub const TXCFG_MXDMA16: c_uint = 0x00200000;
pub const TXCFG_MXDMA8: c_uint = 0x00100000;
pub const CFG_LNKSTS: c_uint = 0x80000000;
pub const CFG_SPDSTS: c_uint = 0x60000000;
pub const CFG_SPDSTS1: c_uint = 0x40000000;
pub const CFG_SPDSTS0: c_uint = 0x20000000;
pub const CFG_DUPSTS: c_uint = 0x10000000;
pub const CFG_TBI_EN: c_uint = 0x01000000;
pub const CFG_MODE_1000: c_uint = 0x00400000;
// Ramit : Dont' ever use AUTO_1000, it never works and is buggy.
// Read the Phy response and then configure the MAC accordingly
pub const CFG_AUTO_1000: c_uint = 0x00200000;
pub const CFG_PINT_CTL: c_uint = 0x001c0000;
pub const CFG_PINT_DUPSTS: c_uint = 0x00100000;
pub const CFG_PINT_LNKSTS: c_uint = 0x00080000;
pub const CFG_PINT_SPDSTS: c_uint = 0x00040000;
pub const CFG_TMRTEST: c_uint = 0x00020000;
pub const CFG_MRM_DIS: c_uint = 0x00010000;
pub const CFG_MWI_DIS: c_uint = 0x00008000;
pub const CFG_T64ADDR: c_uint = 0x00004000;
pub const CFG_PCI64_DET: c_uint = 0x00002000;
pub const CFG_DATA64_EN: c_uint = 0x00001000;
pub const CFG_M64ADDR: c_uint = 0x00000800;
pub const CFG_PHY_RST: c_uint = 0x00000400;
pub const CFG_PHY_DIS: c_uint = 0x00000200;
pub const CFG_EXTSTS_EN: c_uint = 0x00000100;
pub const CFG_REQALG: c_uint = 0x00000080;
pub const CFG_SB: c_uint = 0x00000040;
pub const CFG_POW: c_uint = 0x00000020;
pub const CFG_EXD: c_uint = 0x00000010;
pub const CFG_PESEL: c_uint = 0x00000008;
pub const CFG_BROM_DIS: c_uint = 0x00000004;
pub const CFG_EXT_125: c_uint = 0x00000002;
pub const CFG_BEM: c_uint = 0x00000001;
pub const EXTSTS_UDPPKT: c_uint = 0x00200000;
pub const EXTSTS_TCPPKT: c_uint = 0x00080000;
pub const EXTSTS_IPPKT: c_uint = 0x00020000;
pub const EXTSTS_VPKT: c_uint = 0x00010000;
pub const EXTSTS_VTG_MASK: c_uint = 0x0000ffff;

pub const MIBC_MIBS: c_uint = 0x00000008;
pub const MIBC_ACLR: c_uint = 0x00000004;
pub const MIBC_FRZ: c_uint = 0x00000002;
pub const MIBC_WRN: c_uint = 0x00000001;

pub const PCR_PAUSE_CNT: c_uint = 0xFFFE;
pub const RXCFG_AEP: c_uint = 0x80000000;
pub const RXCFG_ARP: c_uint = 0x40000000;
pub const RXCFG_STRIPCRC: c_uint = 0x20000000;
pub const RXCFG_RX_FD: c_uint = 0x10000000;
pub const RXCFG_ALP: c_uint = 0x08000000;
pub const RXCFG_AIRL: c_uint = 0x04000000;
pub const RXCFG_MXDMA512: c_uint = 0x00700000;
pub const RXCFG_DRTH: c_uint = 0x0000003e;
pub const RXCFG_DRTH0: c_uint = 0x00000002;
pub const RFCR_RFEN: c_uint = 0x80000000;
pub const RFCR_AAB: c_uint = 0x40000000;
pub const RFCR_AAM: c_uint = 0x20000000;
pub const RFCR_AAU: c_uint = 0x10000000;
pub const RFCR_APM: c_uint = 0x08000000;
pub const RFCR_APAT: c_uint = 0x07800000;
pub const RFCR_APAT3: c_uint = 0x04000000;
pub const RFCR_APAT2: c_uint = 0x02000000;
pub const RFCR_APAT1: c_uint = 0x01000000;
pub const RFCR_APAT0: c_uint = 0x00800000;
pub const RFCR_AARP: c_uint = 0x00400000;
pub const RFCR_MHEN: c_uint = 0x00200000;
pub const RFCR_UHEN: c_uint = 0x00100000;
pub const RFCR_ULM: c_uint = 0x00080000;
pub const VRCR_RUDPE: c_uint = 0x00000080;
pub const VRCR_RTCPE: c_uint = 0x00000040;
pub const VRCR_RIPE: c_uint = 0x00000020;
pub const VRCR_IPEN: c_uint = 0x00000010;
pub const VRCR_DUTF: c_uint = 0x00000008;
pub const VRCR_DVTF: c_uint = 0x00000004;
pub const VRCR_VTREN: c_uint = 0x00000002;
pub const VRCR_VTDEN: c_uint = 0x00000001;
pub const VTCR_PPCHK: c_uint = 0x00000008;
pub const VTCR_GCHK: c_uint = 0x00000004;
pub const VTCR_VPPTI: c_uint = 0x00000002;
pub const VTCR_VGTI: c_uint = 0x00000001;
pub const CR: c_uint = 0x00;
pub const CFG: c_uint = 0x04;
pub const MEAR: c_uint = 0x08;
pub const PTSCR: c_uint = 0x0c;
pub const ISR: c_uint = 0x10;
pub const IMR: c_uint = 0x14;
pub const IER: c_uint = 0x18;
pub const IHR: c_uint = 0x1c;
pub const TXDP: c_uint = 0x20;
pub const TXDP_HI: c_uint = 0x24;
pub const TXCFG: c_uint = 0x28;
pub const GPIOR: c_uint = 0x2c;
pub const RXDP: c_uint = 0x30;
pub const RXDP_HI: c_uint = 0x34;
pub const RXCFG: c_uint = 0x38;
pub const PQCR: c_uint = 0x3c;
pub const WCSR: c_uint = 0x40;
pub const PCR: c_uint = 0x44;
pub const RFCR: c_uint = 0x48;
pub const RFDR: c_uint = 0x4c;
pub const SRR: c_uint = 0x58;
pub const VRCR: c_uint = 0xbc;
pub const VTCR: c_uint = 0xc0;
pub const VDR: c_uint = 0xc4;
pub const CCSR: c_uint = 0xcc;
pub const TBICR: c_uint = 0xe0;
pub const TBISR: c_uint = 0xe4;
pub const TANAR: c_uint = 0xe8;
pub const TANLPAR: c_uint = 0xec;
pub const TANER: c_uint = 0xf0;
pub const TESR: c_uint = 0xf4;
pub const TBICR_MR_AN_ENABLE: c_uint = 0x00001000;
pub const TBICR_MR_RESTART_AN: c_uint = 0x00000200;
pub const TBISR_MR_LINK_STATUS: c_uint = 0x00000020;
pub const TBISR_MR_AN_COMPLETE: c_uint = 0x00000004;
pub const TANAR_PS2: c_uint = 0x00000100;
pub const TANAR_PS1: c_uint = 0x00000080;
pub const TANAR_HALF_DUP: c_uint = 0x00000040;
pub const TANAR_FULL_DUP: c_uint = 0x00000020;
pub const GPIOR_GP5_OE: c_uint = 0x00000200;
pub const GPIOR_GP4_OE: c_uint = 0x00000100;
pub const GPIOR_GP3_OE: c_uint = 0x00000080;
pub const GPIOR_GP2_OE: c_uint = 0x00000040;
pub const GPIOR_GP1_OE: c_uint = 0x00000020;
pub const GPIOR_GP3_OUT: c_uint = 0x00000004;
pub const GPIOR_GP1_OUT: c_uint = 0x00000001;
pub const LINK_AUTONEGOTIATE: c_uint = 0x01;
pub const LINK_DOWN: c_uint = 0x02;
pub const LINK_UP: c_uint = 0x04;

    do {							\
    ((desc)[0] = cpu_to_le32(addr));		\
    if (HW_ADDR_LEN == 8)		 		\
    (desc)[1] = cpu_to_le32(((u64)addr) >> 32);	\
    } while(0)

    (le32_to_cpu((desc)[0]) | \
    (HW_ADDR_LEN == 8 ? ((dma_addr_t)le32_to_cpu((desc)[1]))<<32 : 0))
pub const DESC_LINK: c_int = 0;

pub const CMDSTS_OWN: c_uint = 0x80000000;
pub const CMDSTS_MORE: c_uint = 0x40000000;
pub const CMDSTS_INTR: c_uint = 0x20000000;
pub const CMDSTS_ERR: c_uint = 0x10000000;
pub const CMDSTS_OK: c_uint = 0x08000000;
pub const CMDSTS_RUNT: c_uint = 0x00200000;
pub const CMDSTS_LEN_MASK: c_uint = 0x0000ffff;
pub const CMDSTS_DEST_MASK: c_uint = 0x01800000;
pub const CMDSTS_DEST_SELF: c_uint = 0x00800000;
pub const CMDSTS_DEST_MULTI: c_uint = 0x01000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_info {
    pub lock: spinlock_t,
    pub up: c_int,
    pub idle: c_ulong,
    pub skbs: [*mut sk_buff; NR_RX_DESC],
    pub next_rx_desc: *mut __le32,
    pub next_empty: u16 next_rx,,
    pub descs: *mut __le32,
    pub phy_descs: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ns83820 {
    pub base: *mut u8 __iomem,
    pub pci_dev: *mut pci_dev,
    pub ndev: *mut net_device,
    pub rx_info: rx_info,
    pub rx_tasklet: tasklet_struct,
    pub ihr: unsigned,
    pub tq_refill: work_struct,
// protects everything below.  irqsave when using.
    pub misc_lock: spinlock_t,
    pub CFG_cache: u32,
    pub MEAR_cache: u32,
    pub IMR_cache: u32,
    pub linkstate: unsigned,
    pub tx_lock: spinlock_t,
    pub tx_done_idx: u16,
    pub tx_idx: u16,
    pub /: *mut *mut volatile u16 tx_free_idx; / idx of free desc chain,
    pub tx_intr_idx: u16,
    pub nr_tx_skbs: core::sync::atomic::AtomicI32,
    pub tx_skbs: [*mut sk_buff; NR_TX_DESC],
    pub __attribute__((aligned(16))): char pad[16],
    pub tx_descs: *mut __le32,
    pub tx_phy_descs: dma_addr_t,
    pub tx_watchdog: timer_list,
}

    static inline struct ns83820 *PRIV(struct net_device *dev)
    {
    return netdev_priv(dev);
    }

#[no_mangle]
pub unsafe extern "C" fn kick_rx(ndev: *mut net_device) {
    static inline void kick_rx(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    dprintk("kick_rx: maybe kicking\n");
    if (test_and_clear_bit(0, &dev.rx_info.idle)) {
    dprintk("actually kicking\n");
    writel(dev.rx_info.phy_descs +
    (4 * DESC_SIZE * dev.rx_info.next_rx),
    dev.base + RXDP);
    if (dev.rx_info.next_rx == dev.rx_info.next_empty)
    printk(KERN_DEBUG "%s: uh-oh: next_rx == next_empty???\n",
    ndev.name);
    __kick_rx(dev);
    }
    }
// free = (tx_done_idx + NR_TX_DESC-2 - free_idx) % NR_TX_DESC

    (((NR_TX_DESC-2 + dev.tx_done_idx - dev.tx_free_idx) % NR_TX_DESC) > MIN_TX_DESC_FREE)
// Packet Receiver
//
// The hardware supports linked lists of receive descriptors for
// which ownership is transferred back and forth by means of an
// ownership bit.  While the hardware does support the use of a
// ring for receive descriptors, we only make use of a chain in
// an attempt to reduce bus traffic under heavy load scenarios.
// This will also make bugs a bit more obvious.  The current code
// only makes use of a single rx chain; I hope to implement
// priority based rx for version 1.0.  Goal: even under overload
// conditions, still route realtime traffic with as low jitter as
// possible.
//
#[no_mangle]
pub unsafe extern "C" fn build_rx_desc(dev: *mut ns83820, desc: *mut __le32, link: dma_addr_t, buf: dma_addr_t, cmdsts: u32, extsts: u32) {
    static inline void build_rx_desc(struct ns83820 *dev, __le32 *desc, dma_addr_t link, dma_addr_t buf, u32 cmdsts, u32 extsts)
    {
    desc_addr_set(desc + DESC_LINK, link);
    desc_addr_set(desc + DESC_BUFPTR, buf);
    desc[DESC_EXTSTS] = cpu_to_le32(extsts);
    mb();
    desc[DESC_CMDSTS] = cpu_to_le32(cmdsts);
    }

#[no_mangle]
pub unsafe extern "C" fn ns83820_add_rx_skb(dev: *mut ns83820, skb: *mut sk_buff) -> c_int {
    static inline int ns83820_add_rx_skb(struct ns83820 *dev, struct sk_buff *skb)
    {
    unsigned next_empty;
    u32 cmdsts;
    __le32 *sg;
    dma_addr_t buf;
    next_empty = dev.rx_info.next_empty;
// don't overrun last rx marker
    if (unlikely(nr_rx_empty(dev) <= 2)) {
    kfree_skb(skb);
    return 1;
    }

    dprintk("next_empty[%d] nr_used[%d] next_rx[%d]\n",
    dev.rx_info.next_empty,
    dev.rx_info.nr_used,
    dev.rx_info.next_rx
    );

    sg = dev.rx_info.descs + (next_empty * DESC_SIZE);
    BUG_ON(core::ptr::null_mut() != dev.rx_info.skbs[next_empty]);
    dev.rx_info.skbs[next_empty] = skb;
    dev.rx_info.next_empty = (next_empty + 1) % NR_RX_DESC;
    cmdsts = REAL_RX_BUF_SIZE | CMDSTS_INTR;
    buf = dma_map_single(&dev.pci_dev.dev, skb.data, REAL_RX_BUF_SIZE,
    DMA_FROM_DEVICE);
    build_rx_desc(dev, sg, 0, buf, cmdsts, 0);
// update link of previous rx
    if (likely(next_empty != dev.rx_info.next_rx))
    dev.rx_info.descs[((NR_RX_DESC + next_empty - 1) % NR_RX_DESC) * DESC_SIZE] = cpu_to_le32(dev.rx_info.phy_descs + (next_empty * DESC_SIZE * 4));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rx_refill(ndev: *mut net_device, gfp: gfp_t) -> c_int {
    static inline int rx_refill(struct net_device *ndev, gfp_t gfp)
    {
    struct ns83820 *dev = PRIV(ndev);
    unsigned i;
    let mut flags: c_ulong = 0;
    if (unlikely(nr_rx_empty(dev) <= 2))
    return 0;
    dprintk("rx_refill(%p)\n", ndev);
    if (gfp == GFP_ATOMIC)
    spin_lock_irqsave(&dev.rx_info.lock, flags);
    for (i=0; i<NR_RX_DESC; i++) {
    struct sk_buff *skb;
    long res;
// extra 16 bytes for alignment
    skb = __netdev_alloc_skb(ndev, REAL_RX_BUF_SIZE+16, gfp);
    if (unlikely(!skb))
    break;
    skb_reserve(skb, skb.data - PTR_ALIGN(skb.data, 16));
    if (gfp != GFP_ATOMIC)
    spin_lock_irqsave(&dev.rx_info.lock, flags);
    res = ns83820_add_rx_skb(dev, skb);
    if (gfp != GFP_ATOMIC)
    spin_unlock_irqrestore(&dev.rx_info.lock, flags);
    if (res) {
    i = 1;
    break;
    }
    }
    if (gfp == GFP_ATOMIC)
    spin_unlock_irqrestore(&dev.rx_info.lock, flags);
    return i ? 0 : -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn rx_refill_atomic(ndev: *mut net_device) {
    static void rx_refill_atomic(struct net_device *ndev)
    {
    rx_refill(ndev, GFP_ATOMIC);
    }
// REFILL
#[no_mangle]
pub unsafe extern "C" fn queue_refill(work: *mut work_struct) {
    static inline void queue_refill(struct work_struct *work)
    {
    struct ns83820 *dev = container_of(work, struct ns83820, tq_refill);
    struct net_device *ndev = dev.ndev;
    rx_refill(ndev, GFP_KERNEL);
    if (dev.rx_info.up)
    kick_rx(ndev);
    }
#[no_mangle]
pub unsafe extern "C" fn clear_rx_desc(dev: *mut ns83820, i: unsigned) {
    static inline void clear_rx_desc(struct ns83820 *dev, unsigned i)
    {
    build_rx_desc(dev, dev.rx_info.descs + (DESC_SIZE * i), 0, 0, CMDSTS_OWN, 0);
    }
#[no_mangle]
unsafe extern "C" fn phy_intr(ndev: *mut net_device) {
    static void phy_intr(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    static const char *speeds[] = { "10", "100", "1000", "1000(?)", "1000F" };
    u32 cfg, new_cfg;
    u32 tanar, tanlpar;
    int speed, fullduplex, newlinkstate;
    cfg = readl(dev.base + CFG) ^ SPDSTS_POLARITY;
    if (dev.CFG_cache & CFG_TBI_EN) {
    u32 __maybe_unused tbisr;
// we have an optical transceiver
    tbisr = readl(dev.base + TBISR);
    tanar = readl(dev.base + TANAR);
    tanlpar = readl(dev.base + TANLPAR);
    dprintk("phy_intr: tbisr=%08x, tanar=%08x, tanlpar=%08x\n",
    tbisr, tanar, tanlpar);
    if ( (fullduplex = (tanlpar & TANAR_FULL_DUP) &&
    (tanar & TANAR_FULL_DUP)) ) {
// both of us are full duplex
    writel(readl(dev.base + TXCFG)
    | TXCFG_CSI | TXCFG_HBI | TXCFG_ATP,
    dev.base + TXCFG);
    writel(readl(dev.base + RXCFG) | RXCFG_RX_FD,
    dev.base + RXCFG);
// Light up full duplex LED
    writel(readl(dev.base + GPIOR) | GPIOR_GP1_OUT,
    dev.base + GPIOR);
    } else if (((tanlpar & TANAR_HALF_DUP) &&
    (tanar & TANAR_HALF_DUP)) ||
    ((tanlpar & TANAR_FULL_DUP) &&
    (tanar & TANAR_HALF_DUP)) ||
    ((tanlpar & TANAR_HALF_DUP) &&
    (tanar & TANAR_FULL_DUP))) {
// one or both of us are half duplex
    writel((readl(dev.base + TXCFG)
    & ~(TXCFG_CSI | TXCFG_HBI)) | TXCFG_ATP,
    dev.base + TXCFG);
    writel(readl(dev.base + RXCFG) & ~RXCFG_RX_FD,
    dev.base + RXCFG);
// Turn off full duplex LED
    writel(readl(dev.base + GPIOR) & ~GPIOR_GP1_OUT,
    dev.base + GPIOR);
    }
    speed = 4; /* 1000F */
    } else {
// we have a copper transceiver
    new_cfg = dev.CFG_cache & ~(CFG_SB | CFG_MODE_1000 | CFG_SPDSTS);
    if (cfg & CFG_SPDSTS1)
    new_cfg |= CFG_MODE_1000;
    else
    new_cfg &= ~CFG_MODE_1000;
    speed = ((cfg / CFG_SPDSTS0) & 3);
    fullduplex = (cfg & CFG_DUPSTS);
    if (fullduplex) {
    new_cfg |= CFG_SB;
    writel(readl(dev.base + TXCFG)
    | TXCFG_CSI | TXCFG_HBI,
    dev.base + TXCFG);
    writel(readl(dev.base + RXCFG) | RXCFG_RX_FD,
    dev.base + RXCFG);
    } else {
    writel(readl(dev.base + TXCFG)
    & ~(TXCFG_CSI | TXCFG_HBI),
    dev.base + TXCFG);
    writel(readl(dev.base + RXCFG) & ~(RXCFG_RX_FD),
    dev.base + RXCFG);
    }
    if ((cfg & CFG_LNKSTS) &&
    ((new_cfg ^ dev.CFG_cache) != 0)) {
    writel(new_cfg, dev.base + CFG);
    dev.CFG_cache = new_cfg;
    }
    dev.CFG_cache &= ~CFG_SPDSTS;
    dev.CFG_cache |= cfg & CFG_SPDSTS;
    }
    newlinkstate = (cfg & CFG_LNKSTS) ? LINK_UP : LINK_DOWN;
    if (newlinkstate & LINK_UP &&
    dev.linkstate != newlinkstate) {
    netif_start_queue(ndev);
    netif_wake_queue(ndev);
    printk(KERN_INFO "%s: link now %s mbps, %s duplex and up.\n",
    ndev.name,
    speeds[speed],
    fullduplex ? "full" : "half");
    } else if (newlinkstate & LINK_DOWN &&
    dev.linkstate != newlinkstate) {
    netif_stop_queue(ndev);
    printk(KERN_INFO "%s: link now down.\n", ndev.name);
    }
    dev.linkstate = newlinkstate;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_setup_rx(ndev: *mut net_device) -> c_int {
    static int ns83820_setup_rx(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    unsigned i;
    int ret;
    dprintk("ns83820_setup_rx(%p)\n", ndev);
    dev.rx_info.idle = 1;
    dev.rx_info.next_rx = 0;
    dev.rx_info.next_rx_desc = dev.rx_info.descs;
    dev.rx_info.next_empty = 0;
    for (i=0; i<NR_RX_DESC; i++)
    clear_rx_desc(dev, i);
    writel(0, dev.base + RXDP_HI);
    writel(dev.rx_info.phy_descs, dev.base + RXDP);
    ret = rx_refill(ndev, GFP_KERNEL);
    if (!ret) {
    dprintk("starting receiver\n");
// prevent the interrupt handler from stomping on us
    spin_lock_irq(&dev.rx_info.lock);
    writel(0x0001, dev.base + CCSR);
    writel(0, dev.base + RFCR);
    writel(0x7fc00000, dev.base + RFCR);
    writel(0xffc00000, dev.base + RFCR);
    dev.rx_info.up = 1;
    phy_intr(ndev);
// Okay, let it rip
    spin_lock(&dev.misc_lock);
    dev.IMR_cache |= ISR_PHY;
    dev.IMR_cache |= ISR_RXRCMP;
// dev->IMR_cache |= ISR_RXERR;
// dev->IMR_cache |= ISR_RXOK;
    dev.IMR_cache |= ISR_RXORN;
    dev.IMR_cache |= ISR_RXSOVR;
    dev.IMR_cache |= ISR_RXDESC;
    dev.IMR_cache |= ISR_RXIDLE;
    dev.IMR_cache |= ISR_TXDESC;
    dev.IMR_cache |= ISR_TXIDLE;
    writel(dev.IMR_cache, dev.base + IMR);
    writel(1, dev.base + IER);
    spin_unlock(&dev.misc_lock);
    kick_rx(ndev);
    spin_unlock_irq(&dev.rx_info.lock);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_cleanup_rx(dev: *mut ns83820) {
    static void ns83820_cleanup_rx(struct ns83820 *dev)
    {
    unsigned i;
    unsigned long flags;
    dprintk("ns83820_cleanup_rx(%p)\n", dev);
// disable receive interrupts
    spin_lock_irqsave(&dev.misc_lock, flags);
    dev.IMR_cache &= ~(ISR_RXOK | ISR_RXDESC | ISR_RXERR | ISR_RXEARLY | ISR_RXIDLE);
    writel(dev.IMR_cache, dev.base + IMR);
    spin_unlock_irqrestore(&dev.misc_lock, flags);
// synchronize with the interrupt handler and kill it
    dev.rx_info.up = 0;
    synchronize_irq(dev.pci_dev.irq);
// touch the pci bus...
    readl(dev.base + IMR);
// assumes the transmitter is already disabled and reset
    writel(0, dev.base + RXDP_HI);
    writel(0, dev.base + RXDP);
    for (i=0; i<NR_RX_DESC; i++) {
    struct sk_buff *skb = dev.rx_info.skbs[i];
    dev.rx_info.skbs[i] = core::ptr::null_mut();
    clear_rx_desc(dev, i);
    kfree_skb(skb);
    }
    }
#[no_mangle]
unsafe extern "C" fn ns83820_rx_kick(ndev: *mut net_device) {
    static void ns83820_rx_kick(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
// if (nr_rx_empty(dev) >= NR_RX_DESC/4)*/ {
    if (dev.rx_info.up) {
    rx_refill_atomic(ndev);
    kick_rx(ndev);
    }
    }
    if (dev.rx_info.up && nr_rx_empty(dev) > NR_RX_DESC*3/4)
    schedule_work(&dev.tq_refill);
    else
    kick_rx(ndev);
    if (dev.rx_info.idle)
    printk(KERN_DEBUG "%s: BAD\n", ndev.name);
    }
// rx_irq
//
#[no_mangle]
unsafe extern "C" fn rx_irq(ndev: *mut net_device) {
    static void rx_irq(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    struct rx_info *info = &dev.rx_info;
    unsigned next_rx;
    int len;
    u32 cmdsts;
    __le32 *desc;
    unsigned long flags;
    let mut nr: c_int = 0;
    dprintk("rx_irq(%p)\n", ndev);
    dprintk("rxdp: %08x, descs: %08lx next_rx[%d]: %p next_empty[%d]: %p\n",
    readl(dev.base + RXDP),
    (long)(dev.rx_info.phy_descs),
    (int)dev.rx_info.next_rx,
    (dev.rx_info.descs + (DESC_SIZE * dev.rx_info.next_rx)),
    (int)dev.rx_info.next_empty,
    (dev.rx_info.descs + (DESC_SIZE * dev.rx_info.next_empty))
    );
    spin_lock_irqsave(&info.lock, flags);
    if (!info.up)
    goto out;
    dprintk("walking descs\n");
    next_rx = info.next_rx;
    desc = info.next_rx_desc;
    while ((CMDSTS_OWN & (cmdsts = le32_to_cpu(desc[DESC_CMDSTS]))) &&
    (cmdsts != CMDSTS_OWN)) {
    struct sk_buff *skb;
    let mut extsts: u32 = le32_to_cpu(desc[DESC_EXTSTS]);
    let mut bufptr: dma_addr_t = desc_addr_get(desc + DESC_BUFPTR);
    dprintk("cmdsts: %08x\n", cmdsts);
    dprintk("link: %08x\n", cpu_to_le32(desc[DESC_LINK]));
    dprintk("extsts: %08x\n", extsts);
    skb = info.skbs[next_rx];
    info.skbs[next_rx] = core::ptr::null_mut();
    info.next_rx = (next_rx + 1) % NR_RX_DESC;
    mb();
    clear_rx_desc(dev, next_rx);
    dma_unmap_single(&dev.pci_dev.dev, bufptr, RX_BUF_SIZE,
    DMA_FROM_DEVICE);
    len = cmdsts & CMDSTS_LEN_MASK;

// NH: As was mentioned below, this chip is kinda
// brain dead about vlan tag stripping.  Frames
// that are 64 bytes with a vlan header appended
// like arp frames, or pings, are flagged as Runts
// when the tag is stripped and hardware.  This
// also means that the OK bit in the descriptor
// is cleared when the frame comes in so we have
// to do a specific length check here to make sure
// the frame would have been ok, had we not stripped
// the tag.
//
    if (likely((CMDSTS_OK & cmdsts) ||
    ((cmdsts & CMDSTS_RUNT) && len >= 56))) {

    if (likely(CMDSTS_OK & cmdsts)) {

    skb_put(skb, len);
    if (unlikely(!skb)) {
    ndev.stats.rx_dropped++;
    goto netdev_mangle_me_harder_failed;
    }
    if (cmdsts & CMDSTS_DEST_MULTI)
    ndev.stats.multicast++;
    ndev.stats.rx_packets++;
    ndev.stats.rx_bytes += len;
    if ((extsts & 0x002a0000) && !(extsts & 0x00540000)) {
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    } else {
    skb_checksum_none_assert(skb);
    }
    skb.protocol = eth_type_trans(skb, ndev);

    if(extsts & EXTSTS_VPKT) {
    unsigned short tag;
    tag = ntohs(extsts & EXTSTS_VTG_MASK);
    __vlan_hwaccel_put_tag(skb, htons(ETH_P_IPV6), tag);
    }

    netif_rx(skb);
    } else {
    dev_kfree_skb_irq(skb);
    }
    netdev_mangle_me_harder_failed:
    nr++;
    next_rx = info.next_rx;
    desc = info.descs + (DESC_SIZE * next_rx);
    }
    info.next_rx = next_rx;
    info.next_rx_desc = info.descs + (DESC_SIZE * next_rx);
    out:
    if (0 && !nr) {
    Dprintk("dazed: cmdsts_f: %08x\n", cmdsts);
    }
    spin_unlock_irqrestore(&info.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn rx_action(t: *mut tasklet_struct) {
    static void rx_action(struct tasklet_struct *t)
    {
    struct ns83820 *dev = from_tasklet(dev, t, rx_tasklet);
    struct net_device *ndev = dev.ndev;
    rx_irq(ndev);
    writel(ihr, dev.base + IHR);
    spin_lock_irq(&dev.misc_lock);
    dev.IMR_cache |= ISR_RXDESC;
    writel(dev.IMR_cache, dev.base + IMR);
    spin_unlock_irq(&dev.misc_lock);
    rx_irq(ndev);
    ns83820_rx_kick(ndev);
    }
// Packet Transmit code
//
#[no_mangle]
pub unsafe extern "C" fn kick_tx(dev: *mut ns83820) {
    static inline void kick_tx(struct ns83820 *dev)
    {
    dprintk("kick_tx(%p): tx_idx=%d free_idx=%d\n",
    dev, dev.tx_idx, dev.tx_free_idx);
    writel(CR_TXE, dev.base + CR);
    }
// No spinlock needed on the transmit irq path as the interrupt handler is
// serialized.
//
#[no_mangle]
unsafe extern "C" fn do_tx_done(ndev: *mut net_device) {
    static void do_tx_done(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    u32 cmdsts, tx_done_idx;
    __le32 *desc;
    dprintk("do_tx_done(%p)\n", ndev);
    tx_done_idx = dev.tx_done_idx;
    desc = dev.tx_descs + (tx_done_idx * DESC_SIZE);
    dprintk("tx_done_idx=%d free_idx=%d cmdsts=%08x\n",
    tx_done_idx, dev.tx_free_idx, le32_to_cpu(desc[DESC_CMDSTS]));
    while ((tx_done_idx != dev.tx_free_idx) &&
    !(CMDSTS_OWN & (cmdsts = le32_to_cpu(desc[DESC_CMDSTS]))) ) {
    struct sk_buff *skb;
    unsigned len;
    dma_addr_t addr;
    if (cmdsts & CMDSTS_ERR)
    ndev.stats.tx_errors++;
    if (cmdsts & CMDSTS_OK)
    ndev.stats.tx_packets++;
    if (cmdsts & CMDSTS_OK)
    ndev.stats.tx_bytes += cmdsts & 0xffff;
    dprintk("tx_done_idx=%d free_idx=%d cmdsts=%08x\n",
    tx_done_idx, dev.tx_free_idx, cmdsts);
    skb = dev.tx_skbs[tx_done_idx];
    dev.tx_skbs[tx_done_idx] = core::ptr::null_mut();
    dprintk("done(%p)\n", skb);
    len = cmdsts & CMDSTS_LEN_MASK;
    addr = desc_addr_get(desc + DESC_BUFPTR);
    if (skb) {
    dma_unmap_single(&dev.pci_dev.dev, addr, len,
    DMA_TO_DEVICE);
    dev_consume_skb_irq(skb);
    atomic_dec(&dev.nr_tx_skbs);
    } else
    dma_unmap_page(&dev.pci_dev.dev, addr, len,
    DMA_TO_DEVICE);
    tx_done_idx = (tx_done_idx + 1) % NR_TX_DESC;
    dev.tx_done_idx = tx_done_idx;
    desc[DESC_CMDSTS] = cpu_to_le32(0);
    mb();
    desc = dev.tx_descs + (tx_done_idx * DESC_SIZE);
    }
// Allow network stack to resume queueing packets after we've
// finished transmitting at least 1/4 of the packets in the queue.
//
    if (netif_queue_stopped(ndev) && start_tx_okay(dev)) {
    dprintk("start_queue(%p)\n", ndev);
    netif_start_queue(ndev);
    netif_wake_queue(ndev);
    }
    }
#[no_mangle]
unsafe extern "C" fn ns83820_cleanup_tx(dev: *mut ns83820) {
    static void ns83820_cleanup_tx(struct ns83820 *dev)
    {
    unsigned i;
    for (i=0; i<NR_TX_DESC; i++) {
    struct sk_buff *skb = dev.tx_skbs[i];
    dev.tx_skbs[i] = core::ptr::null_mut();
    if (skb) {
    __le32 *desc = dev.tx_descs + (i * DESC_SIZE);
    dma_unmap_single(&dev.pci_dev.dev,
    desc_addr_get(desc + DESC_BUFPTR),
    le32_to_cpu(desc[DESC_CMDSTS]) & CMDSTS_LEN_MASK,
    DMA_TO_DEVICE);
    dev_kfree_skb_irq(skb);
    atomic_dec(&dev.nr_tx_skbs);
    }
    }
    memset(dev.tx_descs, 0, NR_TX_DESC * DESC_SIZE * 4);
    }
// transmit routine.  This code relies on the network layer serializing
// its calls in, but will run happily in parallel with the interrupt
// handler.  This code currently has provisions for fragmenting tx buffers
// while trying to track down a bug in either the zero copy code or
// the tx fifo (hence the MAX_FRAG_LEN).
//
    static netdev_tx_t ns83820_hard_start_xmit(struct sk_buff *skb,
    struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    u32 free_idx, cmdsts, extsts;
    int nr_free, nr_frags;
    unsigned tx_done_idx, last_idx;
    dma_addr_t buf;
    unsigned len;
    skb_frag_t *frag;
    let mut stopped: c_int = 0;
    let mut do_intr: c_int = 0;
    volatile __le32 *first_desc;
    dprintk("ns83820_hard_start_xmit\n");
    nr_frags =  skb_shinfo(skb).nr_frags;
    again:
    if (unlikely(dev.CFG_cache & CFG_LNKSTS)) {
    netif_stop_queue(ndev);
    if (unlikely(dev.CFG_cache & CFG_LNKSTS))
    return NETDEV_TX_BUSY;
    netif_start_queue(ndev);
    }
    last_idx = free_idx = dev.tx_free_idx;
    tx_done_idx = dev.tx_done_idx;
    nr_free = (tx_done_idx + NR_TX_DESC-2 - free_idx) % NR_TX_DESC;
    nr_free -= 1;
    if (nr_free <= nr_frags) {
    dprintk("stop_queue - not enough(%p)\n", ndev);
    netif_stop_queue(ndev);
// Check again: we may have raced with a tx done irq
    if (dev.tx_done_idx != tx_done_idx) {
    dprintk("restart queue(%p)\n", ndev);
    netif_start_queue(ndev);
    goto again;
    }
    return NETDEV_TX_BUSY;
    }
    if (free_idx == dev.tx_intr_idx) {
    do_intr = 1;
    dev.tx_intr_idx = (dev.tx_intr_idx + NR_TX_DESC/4) % NR_TX_DESC;
    }
    nr_free -= nr_frags;
    if (nr_free < MIN_TX_DESC_FREE) {
    dprintk("stop_queue - last entry(%p)\n", ndev);
    netif_stop_queue(ndev);
    stopped = 1;
    }
    frag = skb_shinfo(skb).frags;
    if (!nr_frags)
    frag = core::ptr::null_mut();
    extsts = 0;
    if (skb.ip_summed == CHECKSUM_PARTIAL) {
    extsts |= EXTSTS_IPPKT;
    if (IPPROTO_TCP == ip_hdr(skb).protocol)
    extsts |= EXTSTS_TCPPKT;
#[no_mangle]
pub unsafe extern "C" fn if(ip_hdr(skb)->protocol: IPPROTO_UDP ==) -> else {
    else if (IPPROTO_UDP == ip_hdr(skb).protocol)
    extsts |= EXTSTS_UDPPKT;
    }

    if (skb_vlan_tag_present(skb)) {
// fetch the vlan tag info out of the
// ancillary data if the vlan code
// is using hw vlan acceleration
//
    let mut tag: c_short = skb_vlan_tag_get(skb);
    extsts |= (EXTSTS_VPKT | htons(tag));
    }

    len = skb.len;
    if (nr_frags)
    len -= skb.data_len;
    buf = dma_map_single(&dev.pci_dev.dev, skb.data, len,
    DMA_TO_DEVICE);
    first_desc = dev.tx_descs + (free_idx * DESC_SIZE);
    for (;;) {
    volatile __le32 *desc = dev.tx_descs + (free_idx * DESC_SIZE);
    dprintk("frag[%3u]: %4u @ 0x%08Lx\n", free_idx, len,
    (unsigned long long)buf);
    last_idx = free_idx;
    free_idx = (free_idx + 1) % NR_TX_DESC;
    desc[DESC_LINK] = cpu_to_le32(dev.tx_phy_descs + (free_idx * DESC_SIZE * 4));
    desc_addr_set(desc + DESC_BUFPTR, buf);
    desc[DESC_EXTSTS] = cpu_to_le32(extsts);
    cmdsts = ((nr_frags) ? CMDSTS_MORE : do_intr ? CMDSTS_INTR : 0);
    cmdsts |= (desc == first_desc) ? 0 : CMDSTS_OWN;
    cmdsts |= len;
    desc[DESC_CMDSTS] = cpu_to_le32(cmdsts);
    if (!nr_frags)
    break;
    buf = skb_frag_dma_map(&dev.pci_dev.dev, frag, 0,
    skb_frag_size(frag), DMA_TO_DEVICE);
    dprintk("frag: buf=%08Lx  page=%08lx offset=%08lx\n",
    (long long)buf, (long) page_to_pfn(frag.page),
    frag.page_offset);
    len = skb_frag_size(frag);
    frag++;
    nr_frags--;
    }
    dprintk("done pkt\n");
    spin_lock_irq(&dev.tx_lock);
    dev.tx_skbs[last_idx] = skb;
    first_desc[DESC_CMDSTS] |= cpu_to_le32(CMDSTS_OWN);
    dev.tx_free_idx = free_idx;
    atomic_inc(&dev.nr_tx_skbs);
    spin_unlock_irq(&dev.tx_lock);
    kick_tx(dev);
// Check again: we may have raced with a tx done irq
    if (stopped && (dev.tx_done_idx != tx_done_idx) && start_tx_okay(dev))
    netif_start_queue(ndev);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_update_stats(dev: *mut ns83820) {
    static void ns83820_update_stats(struct ns83820 *dev)
    {
    struct net_device *ndev = dev.ndev;
    u8 __iomem *base = dev.base;
// the DP83820 will freeze counters, so we need to read all of them
    ndev.stats.rx_errors		+= readl(base + 0x60) & 0xffff;
    ndev.stats.rx_crc_errors	+= readl(base + 0x64) & 0xffff;
    ndev.stats.rx_missed_errors	+= readl(base + 0x68) & 0xffff;
    ndev.stats.rx_frame_errors	+= readl(base + 0x6c) & 0xffff;
// ndev->stats.rx_symbol_errors +=*/ readl(base + 0x70);
    ndev.stats.rx_length_errors	+= readl(base + 0x74) & 0xffff;
    ndev.stats.rx_length_errors	+= readl(base + 0x78) & 0xffff;
// ndev->stats.rx_badopcode_errors += */ readl(base + 0x7c);
// ndev->stats.rx_pause_count += */  readl(base + 0x80);
// ndev->stats.tx_pause_count += */  readl(base + 0x84);
    ndev.stats.tx_carrier_errors	+= readl(base + 0x88) & 0xff;
    }
    static struct net_device_stats *ns83820_get_stats(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
// somewhat overkill
    spin_lock_irq(&dev.misc_lock);
    ns83820_update_stats(dev);
    spin_unlock_irq(&dev.misc_lock);
    return &ndev.stats;
    }
// Let ethtool retrieve info
    static int ns83820_get_link_ksettings(struct net_device *ndev,
    struct ethtool_link_ksettings *cmd)
    {
    struct ns83820 *dev = PRIV(ndev);
    u32 cfg, tbicr;
    let mut fullduplex: c_int = 0;
    u32 supported;
//
// Here's the list of available ethtool commands from other drivers:
// cmd->advertising =
// ethtool_cmd_speed_set(cmd, ...)
// cmd->duplex =
// cmd->port = 0;
// cmd->phy_address =
// cmd->transceiver = 0;
// cmd->autoneg =
// cmd->maxtxpkt = 0;
// cmd->maxrxpkt = 0;
//
// read current configuration
    cfg   = readl(dev.base + CFG) ^ SPDSTS_POLARITY;
    readl(dev.base + TANAR);
    tbicr = readl(dev.base + TBICR);
    fullduplex = (cfg & CFG_DUPSTS) ? 1 : 0;
    supported = SUPPORTED_Autoneg;
    if (dev.CFG_cache & CFG_TBI_EN) {
// we have optical interface
    supported |= SUPPORTED_1000baseT_Half |
    SUPPORTED_1000baseT_Full |
    SUPPORTED_FIBRE;
    cmd.base.port       = PORT_FIBRE;
    } else {
// we have copper
    supported |= SUPPORTED_10baseT_Half |
    SUPPORTED_10baseT_Full | SUPPORTED_100baseT_Half |
    SUPPORTED_100baseT_Full | SUPPORTED_1000baseT_Half |
    SUPPORTED_1000baseT_Full |
    SUPPORTED_MII;
    cmd.base.port = PORT_MII;
    }
    ethtool_convert_legacy_u32_to_link_mode(cmd.link_modes.supported,
    supported);
    cmd.base.duplex = fullduplex ? DUPLEX_FULL : DUPLEX_HALF;
    switch (cfg / CFG_SPDSTS0 & 3) {
    case 2:
    cmd.base.speed = SPEED_1000;
    break;
    case 1:
    cmd.base.speed = SPEED_100;
    break;
    default:
    cmd.base.speed = SPEED_10;
    break;
    }
    cmd.base.autoneg = (tbicr & TBICR_MR_AN_ENABLE)
    ? AUTONEG_ENABLE : AUTONEG_DISABLE;
    return 0;
    }
// Let ethool change settings
    static int ns83820_set_link_ksettings(struct net_device *ndev,
    const struct ethtool_link_ksettings *cmd)
    {
    struct ns83820 *dev = PRIV(ndev);
    u32 cfg, tanar;
    let mut have_optical: c_int = 0;
    let mut fullduplex: c_int = 0;
// read current configuration
    cfg = readl(dev.base + CFG) ^ SPDSTS_POLARITY;
    tanar = readl(dev.base + TANAR);
    if (dev.CFG_cache & CFG_TBI_EN) {
// we have optical
    have_optical = 1;
    fullduplex   = (tanar & TANAR_FULL_DUP);
    } else {
// we have copper
    fullduplex = cfg & CFG_DUPSTS;
    }
    spin_lock_irq(&dev.misc_lock);
    spin_lock(&dev.tx_lock);
// Set duplex
    if (cmd.base.duplex != fullduplex) {
    if (have_optical) {
// set full duplex
    if (cmd.base.duplex == DUPLEX_FULL) {
// force full duplex
    writel(readl(dev.base + TXCFG)
    | TXCFG_CSI | TXCFG_HBI | TXCFG_ATP,
    dev.base + TXCFG);
    writel(readl(dev.base + RXCFG) | RXCFG_RX_FD,
    dev.base + RXCFG);
// Light up full duplex LED
    writel(readl(dev.base + GPIOR) | GPIOR_GP1_OUT,
    dev.base + GPIOR);
    } else {
// TODO: set half duplex
    }
    } else {
// we have copper
// TODO: Set duplex for copper cards
    }
    printk(KERN_INFO "%s: Duplex set via ethtool\n",
    ndev.name);
    }
// Set autonegotiation
    if (1) {
    if (cmd.base.autoneg == AUTONEG_ENABLE) {
// restart auto negotiation
    writel(TBICR_MR_AN_ENABLE | TBICR_MR_RESTART_AN,
    dev.base + TBICR);
    writel(TBICR_MR_AN_ENABLE, dev.base + TBICR);
    dev.linkstate = LINK_AUTONEGOTIATE;
    printk(KERN_INFO "%s: autoneg enabled via ethtool\n",
    ndev.name);
    } else {
// disable auto negotiation
    writel(0x00000000, dev.base + TBICR);
    }
    printk(KERN_INFO "%s: autoneg %s via ethtool\n", ndev.name,
    cmd.base.autoneg ? "ENABLED" : "DISABLED");
    }
    phy_intr(ndev);
    spin_unlock(&dev.tx_lock);
    spin_unlock_irq(&dev.misc_lock);
    return 0;
    }
// end ethtool get/set support -df
#[no_mangle]
unsafe extern "C" fn ns83820_get_drvinfo(ndev: *mut net_device, info: *mut ethtool_drvinfo) {
    static void ns83820_get_drvinfo(struct net_device *ndev, struct ethtool_drvinfo *info)
    {
    struct ns83820 *dev = PRIV(ndev);
    strscpy(info.driver, "ns83820", sizeof(info.driver));
    strscpy(info.version, VERSION, sizeof(info.version));
    strscpy(info.bus_info, pci_name(dev.pci_dev), sizeof(info.bus_info));
    }
#[no_mangle]
unsafe extern "C" fn ns83820_get_link(ndev: *mut net_device) -> u32 {
    static u32 ns83820_get_link(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    let mut cfg: u32 = readl(dev.base + CFG) ^ SPDSTS_POLARITY;
    return cfg & CFG_LNKSTS ? 1 : 0;
    }
    static const struct ethtool_ops ops = {
    .get_drvinfo     = ns83820_get_drvinfo,
    .get_link        = ns83820_get_link,
    .get_link_ksettings = ns83820_get_link_ksettings,
    .set_link_ksettings = ns83820_set_link_ksettings,
    };
#[no_mangle]
pub unsafe extern "C" fn ns83820_disable_interrupts(dev: *mut ns83820) {
    static inline void ns83820_disable_interrupts(struct ns83820 *dev)
    {
    writel(0, dev.base + IMR);
    writel(0, dev.base + IER);
    readl(dev.base + IER);
    }
// this function is called in irq context from the ISR
#[no_mangle]
unsafe extern "C" fn ns83820_mib_isr(dev: *mut ns83820) {
    static void ns83820_mib_isr(struct ns83820 *dev)
    {
    unsigned long flags;
    spin_lock_irqsave(&dev.misc_lock, flags);
    ns83820_update_stats(dev);
    spin_unlock_irqrestore(&dev.misc_lock, flags);
    }
    static void ns83820_do_isr(struct net_device *ndev, u32 isr);
#[no_mangle]
unsafe extern "C" fn ns83820_irq(foo: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t ns83820_irq(int foo, void *data)
    {
    struct net_device *ndev = data;
    struct ns83820 *dev = PRIV(ndev);
    u32 isr;
    dprintk("ns83820_irq(%p)\n", ndev);
    dev.ihr = 0;
    isr = readl(dev.base + ISR);
    dprintk("irq: %08x\n", isr);
    ns83820_do_isr(ndev, isr);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_do_isr(ndev: *mut net_device, isr: u32) {
    static void ns83820_do_isr(struct net_device *ndev, u32 isr)
    {
    struct ns83820 *dev = PRIV(ndev);
    unsigned long flags;

    if (isr & ~(ISR_PHY | ISR_RXDESC | ISR_RXEARLY | ISR_RXOK | ISR_RXERR | ISR_TXIDLE | ISR_TXOK | ISR_TXDESC))
    Dprintk("odd isr? 0x%08x\n", isr);

    if (ISR_RXIDLE & isr) {
    dev.rx_info.idle = 1;
    Dprintk("oh dear, we are idle\n");
    ns83820_rx_kick(ndev);
    }
    if ((ISR_RXDESC | ISR_RXOK) & isr) {
    prefetch(dev.rx_info.next_rx_desc);
    spin_lock_irqsave(&dev.misc_lock, flags);
    dev.IMR_cache &= ~(ISR_RXDESC | ISR_RXOK);
    writel(dev.IMR_cache, dev.base + IMR);
    spin_unlock_irqrestore(&dev.misc_lock, flags);
    tasklet_schedule(&dev.rx_tasklet);
// rx_irq(ndev);
// writel(4, dev->base + IHR);
    }
    if ((ISR_RXIDLE | ISR_RXORN | ISR_RXDESC | ISR_RXOK | ISR_RXERR) & isr)
    ns83820_rx_kick(ndev);
    if (unlikely(ISR_RXSOVR & isr)) {
// printk("overrun: rxsovr\n");
    ndev.stats.rx_fifo_errors++;
    }
    if (unlikely(ISR_RXORN & isr)) {
// printk("overrun: rxorn\n");
    ndev.stats.rx_fifo_errors++;
    }
    if ((ISR_RXRCMP & isr) && dev.rx_info.up)
    writel(CR_RXE, dev.base + CR);
    if (ISR_TXIDLE & isr) {
    u32 txdp;
    txdp = readl(dev.base + TXDP);
    dprintk("txdp: %08x\n", txdp);
    txdp -= dev.tx_phy_descs;
    dev.tx_idx = txdp / (DESC_SIZE * 4);
    if (dev.tx_idx >= NR_TX_DESC) {
    printk(KERN_ALERT "%s: BUG -- txdp out of range\n", ndev.name);
    dev.tx_idx = 0;
    }
// The may have been a race between a pci originated read
// and the descriptor update from the cpu.  Just in case,
// kick the transmitter if the hardware thinks it is on a
// different descriptor than we are.
//
    if (dev.tx_idx != dev.tx_free_idx)
    kick_tx(dev);
    }
// Defer tx ring processing until more than a minimum amount of
// work has accumulated
//
    if ((ISR_TXDESC | ISR_TXIDLE | ISR_TXOK | ISR_TXERR) & isr) {
    spin_lock_irqsave(&dev.tx_lock, flags);
    do_tx_done(ndev);
    spin_unlock_irqrestore(&dev.tx_lock, flags);
// Disable TxOk if there are no outstanding tx packets.
//
    if ((dev.tx_done_idx == dev.tx_free_idx) &&
    (dev.IMR_cache & ISR_TXOK)) {
    spin_lock_irqsave(&dev.misc_lock, flags);
    dev.IMR_cache &= ~ISR_TXOK;
    writel(dev.IMR_cache, dev.base + IMR);
    spin_unlock_irqrestore(&dev.misc_lock, flags);
    }
    }
// The TxIdle interrupt can come in before the transmit has
// completed.  Normally we reap packets off of the combination
// of TxDesc and TxIdle and leave TxOk disabled (since it
// occurs on every packet), but when no further irqs of this
// nature are expected, we must enable TxOk.
//
    if ((ISR_TXIDLE & isr) && (dev.tx_done_idx != dev.tx_free_idx)) {
    spin_lock_irqsave(&dev.misc_lock, flags);
    dev.IMR_cache |= ISR_TXOK;
    writel(dev.IMR_cache, dev.base + IMR);
    spin_unlock_irqrestore(&dev.misc_lock, flags);
    }
// MIB interrupt: one of the statistics counters is about to overflow
    if (unlikely(ISR_MIB & isr))
    ns83820_mib_isr(dev);
// PHY: Link up/down/negotiation state change
    if (unlikely(ISR_PHY & isr))
    phy_intr(ndev);

    if (dev.ihr)
    writel(dev.ihr, dev.base + IHR);

    }
#[no_mangle]
unsafe extern "C" fn ns83820_do_reset(dev: *mut ns83820, which: u32) {
    static void ns83820_do_reset(struct ns83820 *dev, u32 which)
    {
    Dprintk("resetting chip...\n");
    writel(which, dev.base + CR);
    do {
    schedule();
    } while (readl(dev.base + CR) & which);
    Dprintk("okay!\n");
    }
#[no_mangle]
unsafe extern "C" fn ns83820_stop(ndev: *mut net_device) -> c_int {
    static int ns83820_stop(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
// FIXME: protect against interrupt handler?
    timer_delete_sync(&dev.tx_watchdog);
    ns83820_disable_interrupts(dev);
    dev.rx_info.up = 0;
    synchronize_irq(dev.pci_dev.irq);
    ns83820_do_reset(dev, CR_RST);
    synchronize_irq(dev.pci_dev.irq);
    spin_lock_irq(&dev.misc_lock);
    dev.IMR_cache &= ~(ISR_TXURN | ISR_TXIDLE | ISR_TXERR | ISR_TXDESC | ISR_TXOK);
    spin_unlock_irq(&dev.misc_lock);
    ns83820_cleanup_rx(dev);
    ns83820_cleanup_tx(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_tx_timeout(ndev: *mut net_device, txqueue: c_uint) {
    static void ns83820_tx_timeout(struct net_device *ndev, unsigned int txqueue)
    {
    struct ns83820 *dev = PRIV(ndev);
    u32 tx_done_idx;
    __le32 *desc;
    unsigned long flags;
    spin_lock_irqsave(&dev.tx_lock, flags);
    tx_done_idx = dev.tx_done_idx;
    desc = dev.tx_descs + (tx_done_idx * DESC_SIZE);
    printk(KERN_INFO "%s: tx_timeout: tx_done_idx=%d free_idx=%d cmdsts=%08x\n",
    ndev.name,
    tx_done_idx, dev.tx_free_idx, le32_to_cpu(desc[DESC_CMDSTS]));

    {
    u32 isr;
    isr = readl(dev.base + ISR);
    printk("irq: %08x imr: %08x\n", isr, dev.IMR_cache);
    ns83820_do_isr(ndev, isr);
    }

    do_tx_done(ndev);
    tx_done_idx = dev.tx_done_idx;
    desc = dev.tx_descs + (tx_done_idx * DESC_SIZE);
    printk(KERN_INFO "%s: after: tx_done_idx=%d free_idx=%d cmdsts=%08x\n",
    ndev.name,
    tx_done_idx, dev.tx_free_idx, le32_to_cpu(desc[DESC_CMDSTS]));
    spin_unlock_irqrestore(&dev.tx_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn ns83820_tx_watch(t: *mut timer_list) {
    static void ns83820_tx_watch(struct timer_list *t)
    {
    struct ns83820 *dev = timer_container_of(dev, t, tx_watchdog);
    struct net_device *ndev = dev.ndev;

    printk("ns83820_tx_watch: %u %u %d\n",
    dev.tx_done_idx, dev.tx_free_idx, atomic_read(&dev.nr_tx_skbs)
    );

    if (time_after(jiffies, dev_trans_start(ndev) + 1*HZ) &&
    dev.tx_done_idx != dev.tx_free_idx) {
    printk(KERN_DEBUG "%s: ns83820_tx_watch: %u %u %d\n",
    ndev.name,
    dev.tx_done_idx, dev.tx_free_idx,
    atomic_read(&dev.nr_tx_skbs));
    ns83820_tx_timeout(ndev, UINT_MAX);
    }
    mod_timer(&dev.tx_watchdog, jiffies + 2*HZ);
    }
#[no_mangle]
unsafe extern "C" fn ns83820_open(ndev: *mut net_device) -> c_int {
    static int ns83820_open(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    unsigned i;
    u32 desc;
    int ret;
    dprintk("ns83820_open\n");
    writel(0, dev.base + PQCR);
    ret = ns83820_setup_rx(ndev);
    if (ret)
    goto failed;
    memset(dev.tx_descs, 0, 4 * NR_TX_DESC * DESC_SIZE);
    for (i=0; i<NR_TX_DESC; i++) {
    dev.tx_descs[(i * DESC_SIZE) + DESC_LINK]
    = cpu_to_le32(
    dev.tx_phy_descs
    + ((i+1) % NR_TX_DESC) * DESC_SIZE * 4);
    }
    dev.tx_idx = 0;
    dev.tx_done_idx = 0;
    desc = dev.tx_phy_descs;
    writel(0, dev.base + TXDP_HI);
    writel(desc, dev.base + TXDP);
    timer_setup(&dev.tx_watchdog, ns83820_tx_watch, 0);
    mod_timer(&dev.tx_watchdog, jiffies + 2*HZ);
    netif_start_queue(ndev);	/* FIXME: wait for phy to come up */
    return 0;
    failed:
    ns83820_stop(ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_getmac(dev: *mut ns83820, ndev: *mut net_device) {
    static void ns83820_getmac(struct ns83820 *dev, struct net_device *ndev)
    {
    u8 mac[ETH_ALEN];
    unsigned i;
    for (i=0; i<3; i++) {
    u32 data;
// Read from the perfect match memory: this is loaded by
// the chip from the EEPROM via the EELOAD self test.
//
    writel(i*2, dev.base + RFCR);
    data = readl(dev.base + RFDR);
    mac[i * 2] = data;
    mac[i * 2 + 1] = data >> 8;
    }
    eth_hw_addr_set(ndev, mac);
    }
#[no_mangle]
unsafe extern "C" fn ns83820_set_multicast(ndev: *mut net_device) {
    static void ns83820_set_multicast(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    u8 __iomem *rfcr = dev.base + RFCR;
    let mut and_mask: u32 = 0xffffffff;
    let mut or_mask: u32 = 0;
    u32 val;
    if (ndev.flags & IFF_PROMISC)
    or_mask |= RFCR_AAU | RFCR_AAM;
    else
    and_mask &= ~(RFCR_AAU | RFCR_AAM);
    if (ndev.flags & IFF_ALLMULTI || netdev_mc_count(ndev))
    or_mask |= RFCR_AAM;
    else
    and_mask &= ~RFCR_AAM;
    spin_lock_irq(&dev.misc_lock);
    val = (readl(rfcr) & and_mask) | or_mask;
// Ramit : RFCR Write Fix doc says RFEN must be 0 modify other bits
    writel(val & ~RFCR_RFEN, rfcr);
    writel(val, rfcr);
    spin_unlock_irq(&dev.misc_lock);
    }
#[no_mangle]
unsafe extern "C" fn ns83820_run_bist(ndev: *mut net_device, name: *const c_char, enable: u32, done: u32, fail: u32) {
    static void ns83820_run_bist(struct net_device *ndev, const char *name, u32 enable, u32 done, u32 fail)
    {
    struct ns83820 *dev = PRIV(ndev);
    let mut timed_out: c_int = 0;
    unsigned long start;
    u32 status;
    let mut loops: c_int = 0;
    dprintk("%s: start %s\n", ndev.name, name);
    start = jiffies;
    writel(enable, dev.base + PTSCR);
    for (;;) {
    loops++;
    status = readl(dev.base + PTSCR);
    if (!(status & enable))
    break;
    if (status & done)
    break;
    if (status & fail)
    break;
    if (time_after_eq(jiffies, start + HZ)) {
    timed_out = 1;
    break;
    }
    schedule_timeout_uninterruptible(1);
    }
    if (status & fail)
    printk(KERN_INFO "%s: %s failed! (0x%08x & 0x%08x)\n",
    ndev.name, name, status, fail);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: timed_out) -> else {
    else if (timed_out)
    printk(KERN_INFO "%s: run_bist %s timed out! (%08x)\n",
    ndev.name, name, status);
    dprintk("%s: done %s in %d loops\n", ndev.name, name, loops);
    }

#[no_mangle]
unsafe extern "C" fn ns83820_mii_write_bit(dev: *mut ns83820, bit: c_int) {
    static void ns83820_mii_write_bit(struct ns83820 *dev, int bit)
    {
// drive MDC low
    dev.MEAR_cache &= ~MEAR_MDC;
    writel(dev.MEAR_cache, dev.base + MEAR);
    readl(dev.base + MEAR);
// enable output, set bit
    dev.MEAR_cache |= MEAR_MDDIR;
    if (bit)
    dev.MEAR_cache |= MEAR_MDIO;
    else
    dev.MEAR_cache &= ~MEAR_MDIO;
// set the output bit
    writel(dev.MEAR_cache, dev.base + MEAR);
    readl(dev.base + MEAR);
// Wait.  Max clock rate is 2.5MHz, this way we come in under 1MHz
    udelay(1);
// drive MDC high causing the data bit to be latched
    dev.MEAR_cache |= MEAR_MDC;
    writel(dev.MEAR_cache, dev.base + MEAR);
    readl(dev.base + MEAR);
// Wait again...
    udelay(1);
    }
#[no_mangle]
unsafe extern "C" fn ns83820_mii_read_bit(dev: *mut ns83820) -> c_int {
    static int ns83820_mii_read_bit(struct ns83820 *dev)
    {
    int bit;
// drive MDC low, disable output
    dev.MEAR_cache &= ~MEAR_MDC;
    dev.MEAR_cache &= ~MEAR_MDDIR;
    writel(dev.MEAR_cache, dev.base + MEAR);
    readl(dev.base + MEAR);
// Wait.  Max clock rate is 2.5MHz, this way we come in under 1MHz
    udelay(1);
// drive MDC high causing the data bit to be latched
    bit = (readl(dev.base + MEAR) & MEAR_MDIO) ? 1 : 0;
    dev.MEAR_cache |= MEAR_MDC;
    writel(dev.MEAR_cache, dev.base + MEAR);
// Wait again...
    udelay(1);
    return bit;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_mii_read_reg(dev: *mut ns83820, phy: unsigned, reg: unsigned) -> unsigned {
    static unsigned ns83820_mii_read_reg(struct ns83820 *dev, unsigned phy, unsigned reg)
    {
    let mut data: unsigned = 0;
    int i;
// read some garbage so that we eventually sync up
    for (i=0; i<64; i++)
    ns83820_mii_read_bit(dev);
    ns83820_mii_write_bit(dev, 0);	/* start */
    ns83820_mii_write_bit(dev, 1);
    ns83820_mii_write_bit(dev, 1);	/* opcode read */
    ns83820_mii_write_bit(dev, 0);
// write out the phy address: 5 bits, msb first
    for (i=0; i<5; i++)
    ns83820_mii_write_bit(dev, phy & (0x10 >> i));
// write out the register address, 5 bits, msb first
    for (i=0; i<5; i++)
    ns83820_mii_write_bit(dev, reg & (0x10 >> i));
    ns83820_mii_read_bit(dev);	/* turn around cycles */
    ns83820_mii_read_bit(dev);
// read in the register data, 16 bits msb first
    for (i=0; i<16; i++) {
    data <<= 1;
    data |= ns83820_mii_read_bit(dev);
    }
    return data;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_mii_write_reg(dev: *mut ns83820, phy: unsigned, reg: unsigned, data: unsigned) -> unsigned {
    static unsigned ns83820_mii_write_reg(struct ns83820 *dev, unsigned phy, unsigned reg, unsigned data)
    {
    int i;
// read some garbage so that we eventually sync up
    for (i=0; i<64; i++)
    ns83820_mii_read_bit(dev);
    ns83820_mii_write_bit(dev, 0);	/* start */
    ns83820_mii_write_bit(dev, 1);
    ns83820_mii_write_bit(dev, 0);	/* opcode read */
    ns83820_mii_write_bit(dev, 1);
// write out the phy address: 5 bits, msb first
    for (i=0; i<5; i++)
    ns83820_mii_write_bit(dev, phy & (0x10 >> i));
// write out the register address, 5 bits, msb first
    for (i=0; i<5; i++)
    ns83820_mii_write_bit(dev, reg & (0x10 >> i));
    ns83820_mii_read_bit(dev);	/* turn around cycles */
    ns83820_mii_read_bit(dev);
// read in the register data, 16 bits msb first
    for (i=0; i<16; i++)
    ns83820_mii_write_bit(dev, (data >> (15 - i)) & 1);
    return data;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_probe_phy(ndev: *mut net_device) {
    static void ns83820_probe_phy(struct net_device *ndev)
    {
    struct ns83820 *dev = PRIV(ndev);
    int j;
    unsigned a, b;
    for (j = 0; j < 0x16; j += 4) {
    dprintk("%s: [0x%02x] %04x %04x %04x %04x\n",
    ndev.name, j,
    ns83820_mii_read_reg(dev, 1, 0 + j),
    ns83820_mii_read_reg(dev, 1, 1 + j),
    ns83820_mii_read_reg(dev, 1, 2 + j),
    ns83820_mii_read_reg(dev, 1, 3 + j)
    );
    }
// read firmware version: memory addr is 0x8402 and 0x8403
    ns83820_mii_write_reg(dev, 1, 0x16, 0x000d);
    ns83820_mii_write_reg(dev, 1, 0x1e, 0x810e);
    a = ns83820_mii_read_reg(dev, 1, 0x1d);
    ns83820_mii_write_reg(dev, 1, 0x16, 0x000d);
    ns83820_mii_write_reg(dev, 1, 0x1e, 0x810e);
    b = ns83820_mii_read_reg(dev, 1, 0x1d);
    dprintk("version: 0x%04x 0x%04x\n", a, b);
    }

    static const struct net_device_ops netdev_ops = {
    .ndo_open		= ns83820_open,
    .ndo_stop		= ns83820_stop,
    .ndo_start_xmit		= ns83820_hard_start_xmit,
    .ndo_get_stats		= ns83820_get_stats,
    .ndo_set_rx_mode	= ns83820_set_multicast,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= eth_mac_addr,
    .ndo_tx_timeout		= ns83820_tx_timeout,
    };
    static int ns83820_init_one(struct pci_dev *pci_dev,
    const struct pci_device_id *id)
    {
    struct net_device *ndev;
    struct ns83820 *dev;
    long addr;
    int err;
    let mut using_dac: c_int = 0;
// See if we can set the dma mask early on; failure is fatal.
    if (sizeof(dma_addr_t) == 8 &&
    !dma_set_mask(&pci_dev.dev, DMA_BIT_MASK(64))) {
    using_dac = 1;
    } else if (!dma_set_mask(&pci_dev.dev, DMA_BIT_MASK(32))) {
    using_dac = 0;
    } else {
    dev_warn(&pci_dev.dev, "dma_set_mask failed!\n");
    return -ENODEV;
    }
    ndev = alloc_etherdev(sizeof(struct ns83820));
    err = -ENOMEM;
    if (!ndev)
    goto out;
    dev = PRIV(ndev);
    dev.ndev = ndev;
    spin_lock_init(&dev.rx_info.lock);
    spin_lock_init(&dev.tx_lock);
    spin_lock_init(&dev.misc_lock);
    dev.pci_dev = pci_dev;
    SET_NETDEV_DEV(ndev, &pci_dev.dev);
    INIT_WORK(&dev.tq_refill, queue_refill);
    tasklet_setup(&dev.rx_tasklet, rx_action);
    err = pci_enable_device(pci_dev);
    if (err) {
    dev_info(&pci_dev.dev, "pci_enable_dev failed: %d\n", err);
    goto out_free;
    }
    pci_set_master(pci_dev);
    addr = pci_resource_start(pci_dev, 1);
    dev.base = ioremap(addr, PAGE_SIZE);
    dev.tx_descs = dma_alloc_coherent(&pci_dev.dev,
    4 * DESC_SIZE * NR_TX_DESC,
    &dev.tx_phy_descs, GFP_KERNEL);
    dev.rx_info.descs = dma_alloc_coherent(&pci_dev.dev,
    4 * DESC_SIZE * NR_RX_DESC,
    &dev.rx_info.phy_descs, GFP_KERNEL);
    err = -ENOMEM;
    if (!dev.base || !dev.tx_descs || !dev.rx_info.descs)
    goto out_disable;
    dprintk("%p: %08lx  %p: %08lx\n",
    dev.tx_descs, (long)dev.tx_phy_descs,
    dev.rx_info.descs, (long)dev.rx_info.phy_descs);
    ns83820_disable_interrupts(dev);
    dev.IMR_cache = 0;
    err = request_irq(pci_dev.irq, ns83820_irq, IRQF_SHARED,
    DRV_NAME, ndev);
    if (err) {
    dev_info(&pci_dev.dev, "unable to register irq %d, err %d\n",
    pci_dev.irq, err);
    goto out_disable;
    }
//
// FIXME: we are holding rtnl_lock() over obscenely long area only
// because some of the setup code uses dev->name.  It's Wrong(tm) -
// we should be using driver-specific names for all that stuff.
// For now that will do, but we really need to come back and kill
// most of the dev_alloc_name() users later.
//
    rtnl_lock();
    err = dev_alloc_name(ndev, ndev.name);
    if (err < 0) {
    dev_info(&pci_dev.dev, "unable to get netdev name: %d\n", err);
    goto out_free_irq;
    }
    printk("%s: ns83820.c: 0x22c: %08x, subsystem: %04x:%04x\n",
    ndev.name, le32_to_cpu(readl(dev.base + 0x22c)),
    pci_dev.subsystem_vendor, pci_dev.subsystem_device);
    ndev.netdev_ops = &netdev_ops;
    ndev.ethtool_ops = &ops;
    ndev.watchdog_timeo = 5 * HZ;
    pci_set_drvdata(pci_dev, ndev);
    ns83820_do_reset(dev, CR_RST);
// Must reset the ram bist before running it
    writel(PTSCR_RBIST_RST, dev.base + PTSCR);
    ns83820_run_bist(ndev, "sram bist",   PTSCR_RBIST_EN,
    PTSCR_RBIST_DONE, PTSCR_RBIST_FAIL);
    ns83820_run_bist(ndev, "eeprom bist", PTSCR_EEBIST_EN, 0,
    PTSCR_EEBIST_FAIL);
    ns83820_run_bist(ndev, "eeprom load", PTSCR_EELOAD_EN, 0, 0);
// I love config registers
    dev.CFG_cache = readl(dev.base + CFG);
    if ((dev.CFG_cache & CFG_PCI64_DET)) {
    printk(KERN_INFO "%s: detected 64 bit PCI data bus.\n",
    ndev.name);
// dev->CFG_cache |= CFG_DATA64_EN;
    if (!(dev.CFG_cache & CFG_DATA64_EN))
    printk(KERN_INFO "%s: EEPROM did not enable 64 bit bus.  Disabled.\n",
    ndev.name);
    } else
    dev.CFG_cache &= ~(CFG_DATA64_EN);
    dev.CFG_cache &= (CFG_TBI_EN  | CFG_MRM_DIS   | CFG_MWI_DIS |
    CFG_T64ADDR | CFG_DATA64_EN | CFG_EXT_125 |
    CFG_M64ADDR);
    dev.CFG_cache |= CFG_PINT_DUPSTS | CFG_PINT_LNKSTS | CFG_PINT_SPDSTS |
    CFG_EXTSTS_EN   | CFG_EXD         | CFG_PESEL;
    dev.CFG_cache |= CFG_REQALG;
    dev.CFG_cache |= CFG_POW;
    dev.CFG_cache |= CFG_TMRTEST;
// When compiled with 64 bit addressing, we must always enable
// the 64 bit descriptor format.
//
    if (sizeof(dma_addr_t) == 8)
    dev.CFG_cache |= CFG_M64ADDR;
    if (using_dac)
    dev.CFG_cache |= CFG_T64ADDR;
// Big endian mode does not seem to do what the docs suggest
    dev.CFG_cache &= ~CFG_BEM;
// setup optical transceiver if we have one
    if (dev.CFG_cache & CFG_TBI_EN) {
    printk(KERN_INFO "%s: enabling optical transceiver\n",
    ndev.name);
    writel(readl(dev.base + GPIOR) | 0x3e8, dev.base + GPIOR);
// setup auto negotiation feature advertisement
    writel(readl(dev.base + TANAR)
    | TANAR_HALF_DUP | TANAR_FULL_DUP,
    dev.base + TANAR);
// start auto negotiation
    writel(TBICR_MR_AN_ENABLE | TBICR_MR_RESTART_AN,
    dev.base + TBICR);
    writel(TBICR_MR_AN_ENABLE, dev.base + TBICR);
    dev.linkstate = LINK_AUTONEGOTIATE;
    dev.CFG_cache |= CFG_MODE_1000;
    }
    writel(dev.CFG_cache, dev.base + CFG);
    dprintk("CFG: %08x\n", dev.CFG_cache);
    if (reset_phy) {
    printk(KERN_INFO "%s: resetting phy\n", ndev.name);
    writel(dev.CFG_cache | CFG_PHY_RST, dev.base + CFG);
    msleep(10);
    writel(dev.CFG_cache, dev.base + CFG);
    }

// the PCI layer.  FIXME.
//
    if (readl(dev.base + SRR))
    writel(readl(dev.base+0x20c) | 0xfe00, dev.base + 0x20c);

// Note!  The DMA burst size interacts with packet
// transmission, such that the largest packet that
// can be transmitted is 8192 - FLTH - burst size.
// If only the transmit fifo was larger...
//
// Ramit : 1024 DMA is not a good idea, it ends up banging
// some DELL and COMPAQ SMP systems
    writel(TXCFG_CSI | TXCFG_HBI | TXCFG_ATP | TXCFG_MXDMA512
    | ((1600 / 32) * 0x100),
    dev.base + TXCFG);
// Flush the interrupt holdoff timer
    writel(0x000, dev.base + IHR);
    writel(0x100, dev.base + IHR);
    writel(0x000, dev.base + IHR);
// Set Rx to full duplex, don't accept runt, errored, long or length
// range errored packets.  Use 512 byte DMA.
//
// Ramit : 1024 DMA is not a good idea, it ends up banging
// some DELL and COMPAQ SMP systems
// Turn on ALP, only we are accepting Jumbo Packets
    writel(RXCFG_AEP | RXCFG_ARP | RXCFG_AIRL | RXCFG_RX_FD
    | RXCFG_STRIPCRC
// | RXCFG_ALP
    | (RXCFG_MXDMA512) | 0, dev.base + RXCFG);
// Disable priority queueing
    writel(0, dev.base + PQCR);
// Enable IP checksum validation and detetion of VLAN headers.
// Note: do not set the reject options as at least the 0x102
// revision of the chip does not properly accept IP fragments
// at least for UDP.
//
// Ramit : Be sure to turn on RXCFG_ARP if VLAN's are enabled, since
// the MAC it calculates the packetsize AFTER stripping the VLAN
// header, and if a VLAN Tagged packet of 64 bytes is received (like
// a ping with a VLAN header) then the card, strips the 4 byte VLAN
// tag and then checks the packet size, so if RXCFG_ARP is not enabled,
// it discrards it!.  These guys......
// also turn on tag stripping if hardware acceleration is enabled
//

    writel(VRCR_INIT_VALUE, dev.base + VRCR);
// Enable per-packet TCP/UDP/IP checksumming
// and per packet vlan tag insertion if
// vlan hardware acceleration is enabled
//

    writel(VTCR_INIT_VALUE, dev.base + VTCR);
// Ramit : Enable async and sync pause frames
// writel(0, dev->base + PCR);
    writel((PCR_PS_MCAST | PCR_PS_DA | PCR_PSEN | PCR_FFLO_4K |
    PCR_FFHI_8K | PCR_STLO_4 | PCR_STHI_8 | PCR_PAUSE_CNT),
    dev.base + PCR);
// Disable Wake On Lan
    writel(0, dev.base + WCSR);
    ns83820_getmac(dev, ndev);
// Yes, we support dumb IP checksum on transmit
    ndev.features |= NETIF_F_SG;
    ndev.features |= NETIF_F_IP_CSUM;
    ndev.min_mtu = 0;

// We also support hardware vlan acceleration
    ndev.features |= NETIF_F_HW_VLAN_CTAG_TX | NETIF_F_HW_VLAN_CTAG_RX;

    if (using_dac) {
    printk(KERN_INFO "%s: using 64 bit addressing.\n",
    ndev.name);
    ndev.features |= NETIF_F_HIGHDMA;
    }
    printk(KERN_INFO "%s: ns83820 v" VERSION ": DP83820 v%u.%u: %pM io=0x%08lx irq=%d f=%s\n",
    ndev.name,
    (unsigned)readl(dev.base + SRR) >> 8,
    (unsigned)readl(dev.base + SRR) & 0xff,
    ndev.dev_addr, addr, pci_dev.irq,
    (ndev.features & NETIF_F_HIGHDMA) ? "h,sg" : "sg"
    );

    ns83820_probe_phy(ndev);

    err = register_netdevice(ndev);
    if (err) {
    printk(KERN_INFO "ns83820: unable to register netdev: %d\n", err);
    goto out_cleanup;
    }
    rtnl_unlock();
    return 0;
    out_cleanup:
    ns83820_disable_interrupts(dev); /* paranoia */
    out_free_irq:
    rtnl_unlock();
    free_irq(pci_dev.irq, ndev);
    out_disable:
    if (dev.base)
    iounmap(dev.base);
    dma_free_coherent(&pci_dev.dev, 4 * DESC_SIZE * NR_TX_DESC,
    dev.tx_descs, dev.tx_phy_descs);
    dma_free_coherent(&pci_dev.dev, 4 * DESC_SIZE * NR_RX_DESC,
    dev.rx_info.descs, dev.rx_info.phy_descs);
    pci_disable_device(pci_dev);
    out_free:
    free_netdev(ndev);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ns83820_remove_one(pci_dev: *mut pci_dev) {
    static void ns83820_remove_one(struct pci_dev *pci_dev)
    {
    struct net_device *ndev = pci_get_drvdata(pci_dev);
    struct ns83820 *dev = PRIV(ndev); /* ok even if core::ptr::null_mut() */
    if (!ndev)			/* paranoia */
    return;
    ns83820_disable_interrupts(dev); /* paranoia */
    unregister_netdev(ndev);
    free_irq(dev.pci_dev.irq, ndev);
    iounmap(dev.base);
    dma_free_coherent(&dev.pci_dev.dev, 4 * DESC_SIZE * NR_TX_DESC,
    dev.tx_descs, dev.tx_phy_descs);
    dma_free_coherent(&dev.pci_dev.dev, 4 * DESC_SIZE * NR_RX_DESC,
    dev.rx_info.descs, dev.rx_info.phy_descs);
    pci_disable_device(dev.pci_dev);
    free_netdev(ndev);
    }
    static const struct pci_device_id ns83820_pci_tbl[] = {
    { 0x100b, 0x0022, PCI_ANY_ID, PCI_ANY_ID, 0, .driver_data = 0, },
    { 0, },
    };
    static struct pci_driver driver = {
    .name		= "ns83820",
    .id_table	= ns83820_pci_tbl,
    .probe		= ns83820_init_one,
    .remove		= ns83820_remove_one,

    .suspend	= ,
    .resume		= ,

    };
#[no_mangle]
unsafe extern "C" fn ns83820_init() -> int __init {
    static int __init ns83820_init(void)
    {
    printk(KERN_INFO "ns83820.c: National Semiconductor DP83820 10/100/1000 driver.\n");
    return pci_register_driver(&driver);
    }
#[no_mangle]
unsafe extern "C" fn ns83820_exit() -> void __exit {
    static void __exit ns83820_exit(void)
    {
    pci_unregister_driver(&driver);
    }
    MODULE_AUTHOR("Benjamin LaHaise <bcrl@kvack.org>");
    MODULE_DESCRIPTION("National Semiconductor DP83820 10/100/1000 driver");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, ns83820_pci_tbl);
    module_param(lnksts, int, 0);
    MODULE_PARM_DESC(lnksts, "Polarity of LNKSTS bit");
    module_param(ihr, int, 0);
    MODULE_PARM_DESC(ihr, "Time in 100 us increments to delay interrupts (range 0-127)");
    module_param(reset_phy, int, 0);
    MODULE_PARM_DESC(reset_phy, "Set to 1 to reset the PHY on startup");
    module_init(ns83820_init);
    module_exit(ns83820_exit);
