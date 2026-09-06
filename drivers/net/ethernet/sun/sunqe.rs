//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sun/sunqe.h
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
// $Id: sunqe.h,v 1.13 2000/02/09 11:15:42 davem Exp $
// sunqe.h: Definitions for the Sun QuadEthernet driver.
//
// Copyright (C) 1996 David S. Miller (davem@caip.rutgers.edu)
//
// QEC global registers.
pub const GLOB_CTRL: c_uint = 0x00UL		/* Control			*/;
pub const GLOB_STAT: c_uint = 0x04UL		/* Status			*/;
pub const GLOB_PSIZE: c_uint = 0x08UL		/* Packet Size			*/;
pub const GLOB_MSIZE: c_uint = 0x0cUL		/* Local-memory Size		*/;
pub const GLOB_RSIZE: c_uint = 0x10UL		/* Receive partition size	*/;
pub const GLOB_TSIZE: c_uint = 0x14UL		/* Transmit partition size	*/;
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
// In MACE mode, there are four qe channels.  Each channel has its own
// status bits in the QEC status register.  This macro picks out the
// ones you want.
//

// The following registers are for per-qe channel information/status.
pub const CREG_CTRL: c_uint = 0x00UL	/* Control                   */;
pub const CREG_STAT: c_uint = 0x04UL	/* Status                    */;
pub const CREG_RXDS: c_uint = 0x08UL	/* RX descriptor ring ptr    */;
pub const CREG_TXDS: c_uint = 0x0cUL	/* TX descriptor ring ptr    */;
pub const CREG_RIMASK: c_uint = 0x10UL	/* RX Interrupt Mask         */;
pub const CREG_TIMASK: c_uint = 0x14UL	/* TX Interrupt Mask         */;
pub const CREG_QMASK: c_uint = 0x18UL	/* QEC Error Interrupt Mask  */;
pub const CREG_MMASK: c_uint = 0x1cUL	/* MACE Error Interrupt Mask */;
pub const CREG_RXWBUFPTR: c_uint = 0x20UL	/* Local memory rx write ptr */;
pub const CREG_RXRBUFPTR: c_uint = 0x24UL	/* Local memory rx read ptr  */;
pub const CREG_TXWBUFPTR: c_uint = 0x28UL	/* Local memory tx write ptr */;
pub const CREG_TXRBUFPTR: c_uint = 0x2cUL	/* Local memory tx read ptr  */;
pub const CREG_CCNT: c_uint = 0x30UL	/* Collision Counter         */;
pub const CREG_PIPG: c_uint = 0x34UL	/* Inter-Frame Gap           */;
pub const CREG_REG_SIZE: c_uint = 0x38UL;
pub const CREG_CTRL_RXOFF: c_uint = 0x00000004  /* Disable this qe's receiver*/;
pub const CREG_CTRL_RESET: c_uint = 0x00000002  /* Reset this qe channel     */;
pub const CREG_CTRL_TWAKEUP: c_uint = 0x00000001  /* Transmitter Wakeup, 'go'. */;
pub const CREG_STAT_EDEFER: c_uint = 0x10000000  /* Excessive Defers          */;
pub const CREG_STAT_CLOSS: c_uint = 0x08000000  /* Carrier Loss              */;
pub const CREG_STAT_ERETRIES: c_uint = 0x04000000  /* More than 16 retries      */;
pub const CREG_STAT_LCOLL: c_uint = 0x02000000  /* Late TX Collision         */;
pub const CREG_STAT_FUFLOW: c_uint = 0x01000000  /* FIFO Underflow            */;
pub const CREG_STAT_JERROR: c_uint = 0x00800000  /* Jabber Error              */;
pub const CREG_STAT_BERROR: c_uint = 0x00400000  /* Babble Error              */;
pub const CREG_STAT_TXIRQ: c_uint = 0x00200000  /* Transmit Interrupt        */;
pub const CREG_STAT_CCOFLOW: c_uint = 0x00100000  /* TX Coll-counter Overflow  */;
pub const CREG_STAT_TXDERROR: c_uint = 0x00080000  /* TX Descriptor is bogus    */;
pub const CREG_STAT_TXLERR: c_uint = 0x00040000  /* Late Transmit Error       */;
pub const CREG_STAT_TXPERR: c_uint = 0x00020000  /* Transmit Parity Error     */;
pub const CREG_STAT_TXSERR: c_uint = 0x00010000  /* Transmit SBUS error ack   */;
pub const CREG_STAT_RCCOFLOW: c_uint = 0x00001000  /* RX Coll-counter Overflow  */;
pub const CREG_STAT_RUOFLOW: c_uint = 0x00000800  /* Runt Counter Overflow     */;
pub const CREG_STAT_MCOFLOW: c_uint = 0x00000400  /* Missed Counter Overflow   */;
pub const CREG_STAT_RXFOFLOW: c_uint = 0x00000200  /* RX FIFO Overflow          */;
pub const CREG_STAT_RLCOLL: c_uint = 0x00000100  /* RX Late Collision         */;
pub const CREG_STAT_FCOFLOW: c_uint = 0x00000080  /* Frame Counter Overflow    */;
pub const CREG_STAT_CECOFLOW: c_uint = 0x00000040  /* CRC Error-counter Overflow*/;
pub const CREG_STAT_RXIRQ: c_uint = 0x00000020  /* Receive Interrupt         */;
pub const CREG_STAT_RXDROP: c_uint = 0x00000010  /* Dropped a RX'd packet     */;
pub const CREG_STAT_RXSMALL: c_uint = 0x00000008  /* Receive buffer too small  */;
pub const CREG_STAT_RXLERR: c_uint = 0x00000004  /* Receive Late Error        */;
pub const CREG_STAT_RXPERR: c_uint = 0x00000002  /* Receive Parity Error      */;
pub const CREG_STAT_RXSERR: c_uint = 0x00000001  /* Receive SBUS Error ACK    */;

pub const CREG_QMASK_COFLOW: c_uint = 0x00100000  /* CollCntr overflow         */;
pub const CREG_QMASK_TXDERROR: c_uint = 0x00080000  /* TXD error                 */;
pub const CREG_QMASK_TXLERR: c_uint = 0x00040000  /* TX late error             */;
pub const CREG_QMASK_TXPERR: c_uint = 0x00020000  /* TX parity error           */;
pub const CREG_QMASK_TXSERR: c_uint = 0x00010000  /* TX sbus error ack         */;
pub const CREG_QMASK_RXDROP: c_uint = 0x00000010  /* RX drop                   */;
pub const CREG_QMASK_RXBERROR: c_uint = 0x00000008  /* RX buffer error           */;
pub const CREG_QMASK_RXLEERR: c_uint = 0x00000004  /* RX late error             */;
pub const CREG_QMASK_RXPERR: c_uint = 0x00000002  /* RX parity error           */;
pub const CREG_QMASK_RXSERR: c_uint = 0x00000001  /* RX sbus error ack         */;
pub const CREG_MMASK_EDEFER: c_uint = 0x10000000  /* Excess defer              */;
pub const CREG_MMASK_CLOSS: c_uint = 0x08000000  /* Carrier loss              */;
pub const CREG_MMASK_ERETRY: c_uint = 0x04000000  /* Excess retry              */;
pub const CREG_MMASK_LCOLL: c_uint = 0x02000000  /* Late collision error      */;
pub const CREG_MMASK_UFLOW: c_uint = 0x01000000  /* Underflow                 */;
pub const CREG_MMASK_JABBER: c_uint = 0x00800000  /* Jabber error              */;
pub const CREG_MMASK_BABBLE: c_uint = 0x00400000  /* Babble error              */;
pub const CREG_MMASK_OFLOW: c_uint = 0x00000800  /* Overflow                  */;
pub const CREG_MMASK_RXCOLL: c_uint = 0x00000400  /* RX Coll-Cntr overflow     */;
pub const CREG_MMASK_RPKT: c_uint = 0x00000200  /* Runt pkt overflow         */;
pub const CREG_MMASK_MPKT: c_uint = 0x00000100  /* Missed pkt overflow       */;
pub const CREG_PIPG_TENAB: c_uint = 0x00000020  /* Enable Throttle           */;
pub const CREG_PIPG_MMODE: c_uint = 0x00000010  /* Manual Mode               */;
pub const CREG_PIPG_WMASK: c_uint = 0x0000000f  /* SBUS Wait Mask            */;
// Per-channel AMD 79C940 MACE registers.
pub const MREGS_RXFIFO: c_uint = 0x00UL	/* Receive FIFO                   */;
pub const MREGS_TXFIFO: c_uint = 0x01UL	/* Transmit FIFO                  */;
pub const MREGS_TXFCNTL: c_uint = 0x02UL	/* Transmit Frame Control         */;
pub const MREGS_TXFSTAT: c_uint = 0x03UL	/* Transmit Frame Status          */;
pub const MREGS_TXRCNT: c_uint = 0x04UL	/* Transmit Retry Count           */;
pub const MREGS_RXFCNTL: c_uint = 0x05UL	/* Receive Frame Control          */;
pub const MREGS_RXFSTAT: c_uint = 0x06UL	/* Receive Frame Status           */;
pub const MREGS_FFCNT: c_uint = 0x07UL	/* FIFO Frame Count               */;
pub const MREGS_IREG: c_uint = 0x08UL	/* Interrupt Register             */;
pub const MREGS_IMASK: c_uint = 0x09UL	/* Interrupt Mask                 */;
pub const MREGS_POLL: c_uint = 0x0aUL	/* POLL Register                  */;
pub const MREGS_BCONFIG: c_uint = 0x0bUL	/* BIU Config                     */;
pub const MREGS_FCONFIG: c_uint = 0x0cUL	/* FIFO Config                    */;
pub const MREGS_MCONFIG: c_uint = 0x0dUL	/* MAC Config                     */;
pub const MREGS_PLSCONFIG: c_uint = 0x0eUL	/* PLS Config                     */;
pub const MREGS_PHYCONFIG: c_uint = 0x0fUL	/* PHY Config                     */;
pub const MREGS_CHIPID1: c_uint = 0x10UL	/* Chip-ID, low bits              */;
pub const MREGS_CHIPID2: c_uint = 0x11UL	/* Chip-ID, high bits             */;
pub const MREGS_IACONFIG: c_uint = 0x12UL	/* Internal Address Config        */;
// 0x13UL, reserved
pub const MREGS_FILTER: c_uint = 0x14UL	/* Logical Address Filter         */;
pub const MREGS_ETHADDR: c_uint = 0x15UL	/* Our Ethernet Address           */;
// 0x16UL, reserved
// 0x17UL, reserved
pub const MREGS_MPCNT: c_uint = 0x18UL	/* Missed Packet Count            */;
// 0x19UL, reserved
pub const MREGS_RPCNT: c_uint = 0x1aUL	/* Runt Packet Count              */;
pub const MREGS_RCCNT: c_uint = 0x1bUL	/* RX Collision Count             */;
// 0x1cUL, reserved
pub const MREGS_UTEST: c_uint = 0x1dUL	/* User Test                      */;
pub const MREGS_RTEST1: c_uint = 0x1eUL	/* Reserved Test 1                */;
pub const MREGS_RTEST2: c_uint = 0x1fUL	/* Reserved Test 2                */;
pub const MREGS_REG_SIZE: c_uint = 0x20UL;
pub const MREGS_TXFCNTL_DRETRY: c_uint = 0x80 /* Retry disable                  */;
pub const MREGS_TXFCNTL_DFCS: c_uint = 0x08 /* Disable TX FCS                 */;
pub const MREGS_TXFCNTL_AUTOPAD: c_uint = 0x01 /* TX auto pad                    */;
pub const MREGS_TXFSTAT_VALID: c_uint = 0x80 /* TX valid                       */;
pub const MREGS_TXFSTAT_UNDERFLOW: c_uint = 0x40 /* TX underflow                   */;
pub const MREGS_TXFSTAT_LCOLL: c_uint = 0x20 /* TX late collision              */;
pub const MREGS_TXFSTAT_MRETRY: c_uint = 0x10 /* TX > 1 retries                 */;
pub const MREGS_TXFSTAT_ORETRY: c_uint = 0x08 /* TX 1 retry                     */;
pub const MREGS_TXFSTAT_PDEFER: c_uint = 0x04 /* TX pkt deferred                */;
pub const MREGS_TXFSTAT_CLOSS: c_uint = 0x02 /* TX carrier lost                */;
pub const MREGS_TXFSTAT_RERROR: c_uint = 0x01 /* TX retry error                 */;
pub const MREGS_TXRCNT_EDEFER: c_uint = 0x80 /* TX Excess defers               */;
pub const MREGS_TXRCNT_CMASK: c_uint = 0x0f /* TX retry count                 */;
pub const MREGS_RXFCNTL_LOWLAT: c_uint = 0x08 /* RX low latency                 */;
pub const MREGS_RXFCNTL_AREJECT: c_uint = 0x04 /* RX addr match rej              */;
pub const MREGS_RXFCNTL_AUTOSTRIP: c_uint = 0x01 /* RX auto strip                  */;
pub const MREGS_RXFSTAT_OVERFLOW: c_uint = 0x80 /* RX overflow                    */;
pub const MREGS_RXFSTAT_LCOLL: c_uint = 0x40 /* RX late collision              */;
pub const MREGS_RXFSTAT_FERROR: c_uint = 0x20 /* RX framing error               */;
pub const MREGS_RXFSTAT_FCSERROR: c_uint = 0x10 /* RX FCS error                   */;
pub const MREGS_RXFSTAT_RBCNT: c_uint = 0x0f /* RX msg byte count              */;
pub const MREGS_FFCNT_RX: c_uint = 0xf0 /* RX FIFO frame cnt              */;
pub const MREGS_FFCNT_TX: c_uint = 0x0f /* TX FIFO frame cnt              */;
pub const MREGS_IREG_JABBER: c_uint = 0x80 /* IRQ Jabber error               */;
pub const MREGS_IREG_BABBLE: c_uint = 0x40 /* IRQ Babble error               */;
pub const MREGS_IREG_COLL: c_uint = 0x20 /* IRQ Collision error            */;
pub const MREGS_IREG_RCCO: c_uint = 0x10 /* IRQ Collision cnt overflow     */;
pub const MREGS_IREG_RPKTCO: c_uint = 0x08 /* IRQ Runt packet count overflow */;
pub const MREGS_IREG_MPKTCO: c_uint = 0x04 /* IRQ missed packet cnt overflow */;
pub const MREGS_IREG_RXIRQ: c_uint = 0x02 /* IRQ RX'd a packet              */;
pub const MREGS_IREG_TXIRQ: c_uint = 0x01 /* IRQ TX'd a packet              */;
pub const MREGS_IMASK_BABBLE: c_uint = 0x40 /* IMASK Babble errors            */;
pub const MREGS_IMASK_COLL: c_uint = 0x20 /* IMASK Collision errors         */;
pub const MREGS_IMASK_MPKTCO: c_uint = 0x04 /* IMASK Missed pkt cnt overflow  */;
pub const MREGS_IMASK_RXIRQ: c_uint = 0x02 /* IMASK RX interrupts            */;
pub const MREGS_IMASK_TXIRQ: c_uint = 0x01 /* IMASK TX interrupts            */;
pub const MREGS_POLL_TXVALID: c_uint = 0x80 /* TX is valid                    */;
pub const MREGS_POLL_TDTR: c_uint = 0x40 /* TX data transfer request       */;
pub const MREGS_POLL_RDTR: c_uint = 0x20 /* RX data transfer request       */;
pub const MREGS_BCONFIG_BSWAP: c_uint = 0x40 /* Byte Swap                      */;
pub const MREGS_BCONFIG_4TS: c_uint = 0x00 /* 4byte transmit start point     */;
pub const MREGS_BCONFIG_16TS: c_uint = 0x10 /* 16byte transmit start point    */;
pub const MREGS_BCONFIG_64TS: c_uint = 0x20 /* 64byte transmit start point    */;
pub const MREGS_BCONFIG_112TS: c_uint = 0x30 /* 112byte transmit start point   */;
pub const MREGS_BCONFIG_RESET: c_uint = 0x01 /* SW-Reset the MACE              */;
pub const MREGS_FCONFIG_TXF8: c_uint = 0x00 /* TX fifo 8 write cycles         */;
pub const MREGS_FCONFIG_TXF32: c_uint = 0x80 /* TX fifo 32 write cycles        */;
pub const MREGS_FCONFIG_TXF16: c_uint = 0x40 /* TX fifo 16 write cycles        */;
pub const MREGS_FCONFIG_RXF64: c_uint = 0x20 /* RX fifo 64 write cycles        */;
pub const MREGS_FCONFIG_RXF32: c_uint = 0x10 /* RX fifo 32 write cycles        */;
pub const MREGS_FCONFIG_RXF16: c_uint = 0x00 /* RX fifo 16 write cycles        */;
pub const MREGS_FCONFIG_TFWU: c_uint = 0x08 /* TX fifo watermark update       */;
pub const MREGS_FCONFIG_RFWU: c_uint = 0x04 /* RX fifo watermark update       */;
pub const MREGS_FCONFIG_TBENAB: c_uint = 0x02 /* TX burst enable                */;
pub const MREGS_FCONFIG_RBENAB: c_uint = 0x01 /* RX burst enable                */;
pub const MREGS_MCONFIG_PROMISC: c_uint = 0x80 /* Promiscuous mode enable        */;
pub const MREGS_MCONFIG_TPDDISAB: c_uint = 0x40 /* TX 2part deferral enable       */;
pub const MREGS_MCONFIG_MBAENAB: c_uint = 0x20 /* Modified backoff enable        */;
pub const MREGS_MCONFIG_RPADISAB: c_uint = 0x08 /* RX physical addr disable       */;
pub const MREGS_MCONFIG_RBDISAB: c_uint = 0x04 /* RX broadcast disable           */;
pub const MREGS_MCONFIG_TXENAB: c_uint = 0x02 /* Enable transmitter             */;
pub const MREGS_MCONFIG_RXENAB: c_uint = 0x01 /* Enable receiver                */;
pub const MREGS_PLSCONFIG_TXMS: c_uint = 0x08 /* TX mode select                 */;
pub const MREGS_PLSCONFIG_GPSI: c_uint = 0x06 /* Use GPSI connector             */;
pub const MREGS_PLSCONFIG_DAI: c_uint = 0x04 /* Use DAI connector              */;
pub const MREGS_PLSCONFIG_TP: c_uint = 0x02 /* Use TwistedPair connector      */;
pub const MREGS_PLSCONFIG_AUI: c_uint = 0x00 /* Use AUI connector              */;
pub const MREGS_PLSCONFIG_IOENAB: c_uint = 0x01 /* PLS I/O enable                 */;
pub const MREGS_PHYCONFIG_LSTAT: c_uint = 0x80 /* Link status                    */;
pub const MREGS_PHYCONFIG_LTESTDIS: c_uint = 0x40 /* Disable link test logic        */;
pub const MREGS_PHYCONFIG_RXPOLARITY: c_uint = 0x20 /* RX polarity                    */;
pub const MREGS_PHYCONFIG_APCDISAB: c_uint = 0x10 /* AutoPolarityCorrect disab      */;
pub const MREGS_PHYCONFIG_LTENAB: c_uint = 0x08 /* Select low threshold           */;
pub const MREGS_PHYCONFIG_AUTO: c_uint = 0x04 /* Connector port auto-sel        */;
pub const MREGS_PHYCONFIG_RWU: c_uint = 0x02 /* Remote WakeUp                  */;
pub const MREGS_PHYCONFIG_AW: c_uint = 0x01 /* Auto Wakeup                    */;
pub const MREGS_IACONFIG_ACHNGE: c_uint = 0x80 /* Do address change              */;
pub const MREGS_IACONFIG_PARESET: c_uint = 0x04 /* Physical address reset         */;
pub const MREGS_IACONFIG_LARESET: c_uint = 0x02 /* Logical address reset          */;
pub const MREGS_UTEST_RTRENAB: c_uint = 0x80 /* Enable resv test register      */;
pub const MREGS_UTEST_RTRDISAB: c_uint = 0x40 /* Disab resv test register       */;
pub const MREGS_UTEST_RPACCEPT: c_uint = 0x20 /* Accept runt packets            */;
pub const MREGS_UTEST_FCOLL: c_uint = 0x10 /* Force collision status         */;
pub const MREGS_UTEST_FCSENAB: c_uint = 0x08 /* Enable FCS on RX               */;
pub const MREGS_UTEST_INTLOOPM: c_uint = 0x06 /* Intern lpback w/MENDEC         */;
pub const MREGS_UTEST_INTLOOP: c_uint = 0x04 /* Intern lpback                  */;
pub const MREGS_UTEST_EXTLOOP: c_uint = 0x02 /* Extern lpback                  */;
pub const MREGS_UTEST_NOLOOP: c_uint = 0x00 /* No loopback                    */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_rxd {
    pub rx_flags: u32,
    pub rx_addr: u32,
}

pub const RXD_OWN: c_uint = 0x80000000 /* Ownership.      */;
pub const RXD_UPDATE: c_uint = 0x10000000 /* Being Updated?  */;
pub const RXD_LENGTH: c_uint = 0x000007ff /* Packet Length.  */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_txd {
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
pub const TX_RING_SIZE: c_int = 16;
pub const RX_RING_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qe_init_block {
    pub qe_rxd: [qe_rxd; RX_RING_MAXSIZE],
    pub qe_txd: [qe_txd; TX_RING_MAXSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunqec {
    pub /: *mut *mut *mut void __iomem gregs; / QEC Global Registers,
    pub /: *mut *mut *mut sunqe qes[4]; / Each child MACE,
    pub /: *mut *mut unsigned int qec_bursts; / Support burst sizes,
    pub /: *mut *mut *mut platform_device op; / QEC's OF device,
    pub /: *mut *mut *mut sunqec next_module; / List of all QECs in system,
}

pub const PKT_BUF_SZ: c_int = 1664;
pub const RXD_PKT_SZ: c_int = 1664;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunqe_buffers {
    pub tx_buf: [u8; TX_RING_SIZE][PKT_BUF_SZ],
    pub __pad: [u8; 2],
    pub rx_buf: [u8; RX_RING_SIZE][PKT_BUF_SZ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunqe {
    pub /: *mut *mut *mut void __iomem qcregs; / QEC per-channel Registers,
    pub /: *mut *mut *mut void __iomem mregs; / Per-channel MACE Registers,
    pub /: *mut *mut *mut qe_init_block qe_block; / RX and TX descriptors,
    pub /: *mut *mut dma_addr_t qblock_dvma; / RX and TX descriptors,
    pub /: *mut *mut spinlock_t lock; / Protects txfull state,
    pub /: *mut *mut int rx_new, rx_old; / RX ring extents,
    pub /: *mut *mut int tx_new, tx_old; / TX ring extents,
    pub /: *mut *mut *mut sunqe_buffers buffers; / CPU visible address.,
    pub /: *mut *mut dma_addr_t buffers_dvma; / DVMA visible address.,
    pub parent: *mut sunqec,
    pub /: *mut *mut u8 mconfig; / Base MACE mconfig value,
    pub /: *mut *mut *mut platform_device op; / QE's OF device struct,
    pub /: *mut *mut *mut net_device dev; / QE's netdevice struct,
    pub /: *mut *mut int channel; / Who am I?,
}
