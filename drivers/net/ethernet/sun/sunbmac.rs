//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sun/sunbmac.h
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
// $Id: sunbmac.h,v 1.7 2000/07/11 22:35:22 davem Exp $
// sunbmac.h: Defines for the Sun "Big MAC" 100baseT ethernet cards.
//
// Copyright (C) 1997 David S. Miller (davem@caip.rutgers.edu)
//
// QEC global registers.
pub const GLOB_CTRL: c_uint = 0x00UL	/* Control                  */;
pub const GLOB_STAT: c_uint = 0x04UL	/* Status                   */;
pub const GLOB_PSIZE: c_uint = 0x08UL	/* Packet Size              */;
pub const GLOB_MSIZE: c_uint = 0x0cUL	/* Local-mem size (64K)     */;
pub const GLOB_RSIZE: c_uint = 0x10UL	/* Receive partition size   */;
pub const GLOB_TSIZE: c_uint = 0x14UL	/* Transmit partition size  */;
pub const GLOB_REG_SIZE: c_uint = 0x18UL;
pub const GLOB_CTRL_MMODE: c_uint = 0x40000000 /* MACE qec mode            */;
pub const GLOB_CTRL_BMODE: c_uint = 0x10000000 /* BigMAC qec mode          */;
pub const GLOB_CTRL_EPAR: c_uint = 0x00000020 /* Enable parity            */;
pub const GLOB_CTRL_ACNTRL: c_uint = 0x00000018 /* SBUS arbitration control */;
pub const GLOB_CTRL_B64: c_uint = 0x00000004 /* 64 byte dvma bursts      */;
pub const GLOB_CTRL_B32: c_uint = 0x00000002 /* 32 byte dvma bursts      */;
pub const GLOB_CTRL_B16: c_uint = 0x00000000 /* 16 byte dvma bursts      */;
pub const GLOB_CTRL_RESET: c_uint = 0x00000001 /* Reset the QEC            */;
pub const GLOB_STAT_TX: c_uint = 0x00000008 /* BigMAC Transmit IRQ      */;
pub const GLOB_STAT_RX: c_uint = 0x00000004 /* BigMAC Receive IRQ       */;
pub const GLOB_STAT_BM: c_uint = 0x00000002 /* BigMAC Global IRQ        */;
pub const GLOB_STAT_ER: c_uint = 0x00000001 /* BigMAC Error IRQ         */;
pub const GLOB_PSIZE_2048: c_uint = 0x00       /* 2k packet size           */;
pub const GLOB_PSIZE_4096: c_uint = 0x01       /* 4k packet size           */;
pub const GLOB_PSIZE_6144: c_uint = 0x10       /* 6k packet size           */;
pub const GLOB_PSIZE_8192: c_uint = 0x11       /* 8k packet size           */;
// QEC BigMAC channel registers.
pub const CREG_CTRL: c_uint = 0x00UL	/* Control                   */;
pub const CREG_STAT: c_uint = 0x04UL	/* Status                    */;
pub const CREG_RXDS: c_uint = 0x08UL	/* RX descriptor ring ptr    */;
pub const CREG_TXDS: c_uint = 0x0cUL	/* TX descriptor ring ptr    */;
pub const CREG_RIMASK: c_uint = 0x10UL	/* RX Interrupt Mask         */;
pub const CREG_TIMASK: c_uint = 0x14UL	/* TX Interrupt Mask         */;
pub const CREG_QMASK: c_uint = 0x18UL	/* QEC Error Interrupt Mask  */;
pub const CREG_BMASK: c_uint = 0x1cUL	/* BigMAC Error Interrupt Mask*/;
pub const CREG_RXWBUFPTR: c_uint = 0x20UL	/* Local memory rx write ptr */;
pub const CREG_RXRBUFPTR: c_uint = 0x24UL	/* Local memory rx read ptr  */;
pub const CREG_TXWBUFPTR: c_uint = 0x28UL	/* Local memory tx write ptr */;
pub const CREG_TXRBUFPTR: c_uint = 0x2cUL	/* Local memory tx read ptr  */;
pub const CREG_CCNT: c_uint = 0x30UL	/* Collision Counter         */;
pub const CREG_REG_SIZE: c_uint = 0x34UL;
pub const CREG_CTRL_TWAKEUP: c_uint = 0x00000001  /* Transmitter Wakeup, 'go'. */;
pub const CREG_STAT_BERROR: c_uint = 0x80000000  /* BigMAC error              */;
pub const CREG_STAT_TXIRQ: c_uint = 0x00200000  /* Transmit Interrupt        */;
pub const CREG_STAT_TXDERROR: c_uint = 0x00080000  /* TX Descriptor is bogus    */;
pub const CREG_STAT_TXLERR: c_uint = 0x00040000  /* Late Transmit Error       */;
pub const CREG_STAT_TXPERR: c_uint = 0x00020000  /* Transmit Parity Error     */;
pub const CREG_STAT_TXSERR: c_uint = 0x00010000  /* Transmit SBUS error ack   */;
pub const CREG_STAT_RXIRQ: c_uint = 0x00000020  /* Receive Interrupt         */;
pub const CREG_STAT_RXDROP: c_uint = 0x00000010  /* Dropped a RX'd packet     */;
pub const CREG_STAT_RXSMALL: c_uint = 0x00000008  /* Receive buffer too small  */;
pub const CREG_STAT_RXLERR: c_uint = 0x00000004  /* Receive Late Error        */;
pub const CREG_STAT_RXPERR: c_uint = 0x00000002  /* Receive Parity Error      */;
pub const CREG_STAT_RXSERR: c_uint = 0x00000001  /* Receive SBUS Error ACK    */;

pub const CREG_QMASK_TXDERROR: c_uint = 0x00080000  /* TXD error                 */;
pub const CREG_QMASK_TXLERR: c_uint = 0x00040000  /* TX late error             */;
pub const CREG_QMASK_TXPERR: c_uint = 0x00020000  /* TX parity error           */;
pub const CREG_QMASK_TXSERR: c_uint = 0x00010000  /* TX sbus error ack         */;
pub const CREG_QMASK_RXDROP: c_uint = 0x00000010  /* RX drop                   */;
pub const CREG_QMASK_RXBERROR: c_uint = 0x00000008  /* RX buffer error           */;
pub const CREG_QMASK_RXLEERR: c_uint = 0x00000004  /* RX late error             */;
pub const CREG_QMASK_RXPERR: c_uint = 0x00000002  /* RX parity error           */;
pub const CREG_QMASK_RXSERR: c_uint = 0x00000001  /* RX sbus error ack         */;
// BIGMAC core registers
pub const BMAC_XIFCFG: c_uint = 0x000UL	/* XIF config register                */;
// 0x004-->0x0fc, reserved
pub const BMAC_STATUS: c_uint = 0x100UL	/* Status register, clear on read     */;
pub const BMAC_IMASK: c_uint = 0x104UL	/* Interrupt mask register            */;
// 0x108-->0x204, reserved
pub const BMAC_TXSWRESET: c_uint = 0x208UL	/* Transmitter software reset         */;
pub const BMAC_TXCFG: c_uint = 0x20cUL	/* Transmitter config register        */;
pub const BMAC_IGAP1: c_uint = 0x210UL	/* Inter-packet gap 1                 */;
pub const BMAC_IGAP2: c_uint = 0x214UL	/* Inter-packet gap 2                 */;
pub const BMAC_ALIMIT: c_uint = 0x218UL	/* Transmit attempt limit             */;
pub const BMAC_STIME: c_uint = 0x21cUL	/* Transmit slot time                 */;
pub const BMAC_PLEN: c_uint = 0x220UL	/* Size of transmit preamble          */;
pub const BMAC_PPAT: c_uint = 0x224UL	/* Pattern for transmit preamble      */;
pub const BMAC_TXDELIM: c_uint = 0x228UL	/* Transmit delimiter                 */;
pub const BMAC_JSIZE: c_uint = 0x22cUL	/* Toe jam...                         */;
pub const BMAC_TXPMAX: c_uint = 0x230UL	/* Transmit max pkt size              */;
pub const BMAC_TXPMIN: c_uint = 0x234UL	/* Transmit min pkt size              */;
pub const BMAC_PATTEMPT: c_uint = 0x238UL	/* Count of transmit peak attempts    */;
pub const BMAC_DTCTR: c_uint = 0x23cUL	/* Transmit defer timer               */;
pub const BMAC_NCCTR: c_uint = 0x240UL	/* Transmit normal-collision counter  */;
pub const BMAC_FCCTR: c_uint = 0x244UL	/* Transmit first-collision counter   */;
pub const BMAC_EXCTR: c_uint = 0x248UL	/* Transmit excess-collision counter  */;
pub const BMAC_LTCTR: c_uint = 0x24cUL	/* Transmit late-collision counter    */;
pub const BMAC_RSEED: c_uint = 0x250UL	/* Transmit random number seed        */;
pub const BMAC_TXSMACHINE: c_uint = 0x254UL /* Transmit state machine             */;
// 0x258-->0x304, reserved
pub const BMAC_RXSWRESET: c_uint = 0x308UL	/* Receiver software reset            */;
pub const BMAC_RXCFG: c_uint = 0x30cUL	/* Receiver config register           */;
pub const BMAC_RXPMAX: c_uint = 0x310UL	/* Receive max pkt size               */;
pub const BMAC_RXPMIN: c_uint = 0x314UL	/* Receive min pkt size               */;
pub const BMAC_MACADDR2: c_uint = 0x318UL	/* Ether address register 2           */;
pub const BMAC_MACADDR1: c_uint = 0x31cUL	/* Ether address register 1           */;
pub const BMAC_MACADDR0: c_uint = 0x320UL	/* Ether address register 0           */;
pub const BMAC_FRCTR: c_uint = 0x324UL	/* Receive frame receive counter      */;
pub const BMAC_GLECTR: c_uint = 0x328UL	/* Receive giant-length error counter */;
pub const BMAC_UNALECTR: c_uint = 0x32cUL	/* Receive unaligned error counter    */;
pub const BMAC_RCRCECTR: c_uint = 0x330UL	/* Receive CRC error counter          */;
pub const BMAC_RXSMACHINE: c_uint = 0x334UL	/* Receiver state machine             */;
pub const BMAC_RXCVALID: c_uint = 0x338UL	/* Receiver code violation            */;
// 0x33c, reserved
pub const BMAC_HTABLE3: c_uint = 0x340UL	/* Hash table 3                       */;
pub const BMAC_HTABLE2: c_uint = 0x344UL	/* Hash table 2                       */;
pub const BMAC_HTABLE1: c_uint = 0x348UL	/* Hash table 1                       */;
pub const BMAC_HTABLE0: c_uint = 0x34cUL	/* Hash table 0                       */;
pub const BMAC_AFILTER2: c_uint = 0x350UL	/* Address filter 2                   */;
pub const BMAC_AFILTER1: c_uint = 0x354UL	/* Address filter 1                   */;
pub const BMAC_AFILTER0: c_uint = 0x358UL	/* Address filter 0                   */;
pub const BMAC_AFMASK: c_uint = 0x35cUL	/* Address filter mask                */;
pub const BMAC_REG_SIZE: c_uint = 0x360UL;
// BigMac XIF config register.
pub const BIGMAC_XCFG_ODENABLE: c_uint = 0x00000001 /* Output driver enable                     */;
pub const BIGMAC_XCFG_RESV: c_uint = 0x00000002 /* Reserved, write always as 1              */;
pub const BIGMAC_XCFG_MLBACK: c_uint = 0x00000004 /* Loopback-mode MII enable                 */;
pub const BIGMAC_XCFG_SMODE: c_uint = 0x00000008 /* Enable serial mode                       */;
// BigMAC status register.
pub const BIGMAC_STAT_GOTFRAME: c_uint = 0x00000001 /* Received a frame                         */;
pub const BIGMAC_STAT_RCNTEXP: c_uint = 0x00000002 /* Receive frame counter expired            */;
pub const BIGMAC_STAT_ACNTEXP: c_uint = 0x00000004 /* Align-error counter expired              */;
pub const BIGMAC_STAT_CCNTEXP: c_uint = 0x00000008 /* CRC-error counter expired                */;
pub const BIGMAC_STAT_LCNTEXP: c_uint = 0x00000010 /* Length-error counter expired             */;
pub const BIGMAC_STAT_RFIFOVF: c_uint = 0x00000020 /* Receive FIFO overflow                    */;
pub const BIGMAC_STAT_CVCNTEXP: c_uint = 0x00000040 /* Code-violation counter expired           */;
pub const BIGMAC_STAT_SENTFRAME: c_uint = 0x00000100 /* Transmitted a frame                      */;
pub const BIGMAC_STAT_TFIFO_UND: c_uint = 0x00000200 /* Transmit FIFO underrun                   */;
pub const BIGMAC_STAT_MAXPKTERR: c_uint = 0x00000400 /* Max-packet size error                    */;
pub const BIGMAC_STAT_NCNTEXP: c_uint = 0x00000800 /* Normal-collision counter expired         */;
pub const BIGMAC_STAT_ECNTEXP: c_uint = 0x00001000 /* Excess-collision counter expired         */;
pub const BIGMAC_STAT_LCCNTEXP: c_uint = 0x00002000 /* Late-collision counter expired           */;
pub const BIGMAC_STAT_FCNTEXP: c_uint = 0x00004000 /* First-collision counter expired          */;
pub const BIGMAC_STAT_DTIMEXP: c_uint = 0x00008000 /* Defer-timer expired                      */;
// BigMAC interrupt mask register.
pub const BIGMAC_IMASK_GOTFRAME: c_uint = 0x00000001 /* Received a frame                         */;
pub const BIGMAC_IMASK_RCNTEXP: c_uint = 0x00000002 /* Receive frame counter expired            */;
pub const BIGMAC_IMASK_ACNTEXP: c_uint = 0x00000004 /* Align-error counter expired              */;
pub const BIGMAC_IMASK_CCNTEXP: c_uint = 0x00000008 /* CRC-error counter expired                */;
pub const BIGMAC_IMASK_LCNTEXP: c_uint = 0x00000010 /* Length-error counter expired             */;
pub const BIGMAC_IMASK_RFIFOVF: c_uint = 0x00000020 /* Receive FIFO overflow                    */;
pub const BIGMAC_IMASK_CVCNTEXP: c_uint = 0x00000040 /* Code-violation counter expired           */;
pub const BIGMAC_IMASK_SENTFRAME: c_uint = 0x00000100 /* Transmitted a frame                      */;
pub const BIGMAC_IMASK_TFIFO_UND: c_uint = 0x00000200 /* Transmit FIFO underrun                   */;
pub const BIGMAC_IMASK_MAXPKTERR: c_uint = 0x00000400 /* Max-packet size error                    */;
pub const BIGMAC_IMASK_NCNTEXP: c_uint = 0x00000800 /* Normal-collision counter expired         */;
pub const BIGMAC_IMASK_ECNTEXP: c_uint = 0x00001000 /* Excess-collision counter expired         */;
pub const BIGMAC_IMASK_LCCNTEXP: c_uint = 0x00002000 /* Late-collision counter expired           */;
pub const BIGMAC_IMASK_FCNTEXP: c_uint = 0x00004000 /* First-collision counter expired          */;
pub const BIGMAC_IMASK_DTIMEXP: c_uint = 0x00008000 /* Defer-timer expired                      */;
// BigMac transmit config register.
pub const BIGMAC_TXCFG_ENABLE: c_uint = 0x00000001 /* Enable the transmitter                   */;
pub const BIGMAC_TXCFG_FIFO: c_uint = 0x00000010 /* Default tx fthresh...                    */;
pub const BIGMAC_TXCFG_SMODE: c_uint = 0x00000020 /* Enable slow transmit mode                */;
pub const BIGMAC_TXCFG_CIGN: c_uint = 0x00000040 /* Ignore transmit collisions               */;
pub const BIGMAC_TXCFG_FCSOFF: c_uint = 0x00000080 /* Do not emit FCS                          */;
pub const BIGMAC_TXCFG_DBACKOFF: c_uint = 0x00000100 /* Disable backoff                          */;
pub const BIGMAC_TXCFG_FULLDPLX: c_uint = 0x00000200 /* Enable full-duplex                       */;
// BigMac receive config register.
pub const BIGMAC_RXCFG_ENABLE: c_uint = 0x00000001 /* Enable the receiver                      */;
pub const BIGMAC_RXCFG_FIFO: c_uint = 0x0000000e /* Default rx fthresh...                    */;
pub const BIGMAC_RXCFG_PSTRIP: c_uint = 0x00000020 /* Pad byte strip enable                    */;
pub const BIGMAC_RXCFG_PMISC: c_uint = 0x00000040 /* Enable promiscuous mode                   */;
pub const BIGMAC_RXCFG_DERR: c_uint = 0x00000080 /* Disable error checking                   */;
pub const BIGMAC_RXCFG_DCRCS: c_uint = 0x00000100 /* Disable CRC stripping                    */;
pub const BIGMAC_RXCFG_ME: c_uint = 0x00000200 /* Receive packets addressed to me          */;
pub const BIGMAC_RXCFG_PGRP: c_uint = 0x00000400 /* Enable promisc group mode                */;
pub const BIGMAC_RXCFG_HENABLE: c_uint = 0x00000800 /* Enable the hash filter                   */;
pub const BIGMAC_RXCFG_AENABLE: c_uint = 0x00001000 /* Enable the address filter                */;
// The BigMAC PHY transceiver.  Not nearly as sophisticated as the happy meal
// one.  But it does have the "bit banger", oh baby.
//
pub const TCVR_TPAL: c_uint = 0x00UL;
pub const TCVR_MPAL: c_uint = 0x04UL;
pub const TCVR_REG_SIZE: c_uint = 0x08UL;
// Frame commands.
pub const FRAME_WRITE: c_uint = 0x50020000;
pub const FRAME_READ: c_uint = 0x60020000;
// Transceiver registers.
pub const TCVR_PAL_SERIAL: c_uint = 0x00000001 /* Enable serial mode              */;
pub const TCVR_PAL_EXTLBACK: c_uint = 0x00000002 /* Enable external loopback        */;
pub const TCVR_PAL_MSENSE: c_uint = 0x00000004 /* Media sense                     */;
pub const TCVR_PAL_LTENABLE: c_uint = 0x00000008 /* Link test enable                */;
pub const TCVR_PAL_LTSTATUS: c_uint = 0x00000010 /* Link test status  (P1 only)     */;
// Management PAL.
pub const MGMT_PAL_DCLOCK: c_uint = 0x00000001 /* Data clock                      */;
pub const MGMT_PAL_OENAB: c_uint = 0x00000002 /* Output enabler                  */;
pub const MGMT_PAL_MDIO: c_uint = 0x00000004 /* MDIO Data/attached              */;
pub const MGMT_PAL_TIMEO: c_uint = 0x00000008 /* Transmit enable timeout error   */;

// Here are some PHY addresses.

// Ring descriptors and such, same as Quad Ethernet.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_rxd {
    pub rx_flags: u32,
    pub rx_addr: u32,
}

pub const RXD_OWN: c_uint = 0x80000000 /* Ownership.      */;
pub const RXD_UPDATE: c_uint = 0x10000000 /* Being Updated?  */;
pub const RXD_LENGTH: c_uint = 0x000007ff /* Packet Length.  */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_txd {
    pub tx_flags: u32,
    pub tx_addr: u32,
}

pub const TXD_OWN: c_uint = 0x80000000 /* Ownership.      */;
pub const TXD_SOP: c_uint = 0x40000000 /* Start Of Packet */;
pub const TXD_EOP: c_uint = 0x20000000 /* End Of Packet   */;
pub const TXD_UPDATE: c_uint = 0x10000000 /* Being Updated?  */;
pub const TXD_LENGTH: c_uint = 0x000007ff /* Packet Length.  */;
pub const TX_RING_MAXSIZE: c_int = 256;
pub const RX_RING_MAXSIZE: c_int = 256;
pub const TX_RING_SIZE: c_int = 256;
pub const RX_RING_SIZE: c_int = 256;

pub const RX_COPY_THRESHOLD: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmac_init_block {
    pub be_rxd: [be_rxd; RX_RING_MAXSIZE],
    pub be_txd: [be_txd; TX_RING_MAXSIZE],
}

// Now software state stuff.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bigmac_transceiver {
    external = 0,
    internal = 1,
    none     = 2,
}

// Timer state engine.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bigmac_timer_state {
    ltrywait = 1,  /* Forcing try of all modes, from fastest to slowest. */
    asleep   = 2,  /* Timer inactive.                                    */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bigmac {
    pub /: *mut *mut *mut void __iomem gregs; / QEC Global Registers,
    pub /: *mut *mut *mut void __iomem creg; / QEC BigMAC Channel Registers,
    pub /: *mut *mut *mut void __iomem bregs; / BigMAC Registers,
    pub /: *mut *mut *mut void __iomem tregs; / BigMAC Transceiver,
    pub /: *mut *mut *mut bmac_init_block bmac_block; / RX and TX descriptors,
    pub /: *mut *mut dma_addr_t bblock_dvma; / RX and TX descriptors,
    pub lock: spinlock_t,
    pub rx_skbs: [*mut sk_buff; RX_RING_SIZE],
    pub tx_skbs: [*mut sk_buff; TX_RING_SIZE],
    pub tx_old: int rx_new, tx_new, rx_old,,
    pub /: *mut *mut int board_rev; / BigMAC board revision.,
    pub tcvr_type: bigmac_transceiver,
    pub bigmac_bursts: c_uint,
    pub paddr: c_uint,
    pub /: *mut *mut unsigned short sw_bmsr; / SW copy of PHY BMSR,
    pub /: *mut *mut unsigned short sw_bmcr; / SW copy of PHY BMCR,
    pub bigmac_timer: timer_list,
    pub timer_state: bigmac_timer_state,
    pub timer_ticks: c_uint,
    pub qec_op: *mut platform_device,
    pub bigmac_op: *mut platform_device,
    pub dev: *mut net_device,
}

// We use this to acquire receive skb's that we can DMA directly into.

