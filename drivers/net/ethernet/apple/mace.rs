//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apple/mace.h
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
//
// mace.h - definitions for the registers in the Am79C940 MACE
// (Medium Access Control for Ethernet) controller.
//
// Copyright (C) 1996 Paul Mackerras.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mace {
    pub /: *mut *mut REG(rcvfifo); / receive FIFO,
    pub /: *mut *mut REG(xmtfifo); / transmit FIFO,
    pub /: *mut *mut REG(xmtfc); / transmit frame control,
    pub /: *mut *mut REG(xmtfs); / transmit frame status,
    pub /: *mut *mut REG(xmtrc); / transmit retry count,
    pub /: *mut *mut REG(rcvfc); / receive frame control,
    pub /: *mut *mut REG(rcvfs); / receive frame status (4 bytes),
    pub /: *mut *mut REG(fifofc); / FIFO frame count,
    pub /: *mut *mut REG(ir); / interrupt register,
    pub /: *mut *mut REG(imr); / interrupt mask register,
    pub /: *mut *mut REG(pr); / poll register,
    pub /: *mut *mut REG(biucc); / bus interface unit config control,
    pub /: *mut *mut REG(fifocc); / FIFO configuration control,
    pub /: *mut *mut REG(maccc); / medium access control config control,
    pub /: *mut *mut REG(plscc); / phys layer signalling config control,
    pub /: *mut *mut REG(phycc); / physical configuration control,
    pub /: *mut *mut REG(chipid_lo); / chip ID, lsb,
    pub /: *mut *mut REG(chipid_hi); / chip ID, msb,
    pub /: *mut *mut REG(iac); / internal address config,
    pub /: *mut *mut REG(ladrf); / logical address filter (8 bytes),
    pub /: *mut *mut REG(padr); / physical address (6 bytes),
    pub /: *mut *mut REG(mpc); / missed packet count (clears when read),
    pub /: *mut *mut REG(rntpc); / runt packet count (clears when read),
    pub /: *mut *mut REG(rcvcc); / recv collision count (clears when read),
    pub /: *mut *mut REG(utr); / user test reg,
}

// Bits in XMTFC
pub const DRTRY: c_uint = 0x80	/* don't retry transmission after collision */;
pub const DXMTFCS: c_uint = 0x08	/* don't append FCS to transmitted frame */;
pub const AUTO_PAD_XMIT: c_uint = 0x01	/* auto-pad short packets on transmission */;
// Bits in XMTFS: only valid when XMTSV is set in PR and XMTFS
pub const XMTSV: c_uint = 0x80	/* transmit status (i.e. XMTFS) valid */;
pub const UFLO: c_uint = 0x40	/* underflow - xmit fifo ran dry */;
pub const LCOL: c_uint = 0x20	/* late collision (transmission aborted) */;
pub const MORE: c_uint = 0x10	/* 2 or more retries needed to xmit frame */;
pub const ONE: c_uint = 0x08	/* 1 retry needed to xmit frame */;
pub const DEFER: c_uint = 0x04	/* MACE had to defer xmission (enet busy) */;
pub const LCAR: c_uint = 0x02	/* loss of carrier (transmission aborted) */;
pub const RTRY: c_uint = 0x01	/* too many retries (transmission aborted) */;
// Bits in XMTRC: only valid when XMTSV is set in PR (and XMTFS)
pub const EXDEF: c_uint = 0x80	/* had to defer for excessive time */;
pub const RETRY_MASK: c_uint = 0x0f	/* number of retries (0 - 15) */;
// Bits in RCVFC
pub const LLRCV: c_uint = 0x08	/* low latency receive: early DMA request */;
pub const M_RBAR: c_uint = 0x04	/* sets function of EAM/R pin */;
pub const AUTO_STRIP_RCV: c_uint = 0x01	/* auto-strip short LLC frames on recv */;
//
// Bits in RCVFS.  After a frame is received, four bytes of status
// are automatically read from this register and appended to the frame
// data in memory.  These are:
// Byte 0 and 1: message byte count and frame status
// Byte 2: runt packet count
// Byte 3: receive collision count
//
pub const RS_OFLO: c_uint = 0x8000	/* receive FIFO overflowed */;
pub const RS_CLSN: c_uint = 0x4000	/* received frame suffered (late) collision */;
pub const RS_FRAMERR: c_uint = 0x2000	/* framing error flag */;
pub const RS_FCSERR: c_uint = 0x1000	/* frame had FCS error */;
pub const RS_COUNT: c_uint = 0x0fff	/* mask for byte count field */;
// Bits (fields) in FIFOFC

pub const RCVFC_MASK: c_uint = 0x0f;

pub const XMTFC_MASK: c_uint = 0x0f;
//
// Bits in IR and IMR.  The IR clears itself when read.
// Setting a bit in the IMR will disable the corresponding interrupt.
//
pub const JABBER: c_uint = 0x80	/* jabber error - 10baseT xmission too long */;
pub const BABBLE: c_uint = 0x40	/* babble - xmitter xmitting for too long */;
pub const CERR: c_uint = 0x20	/* collision err - no SQE test (heartbeat) */;
pub const RCVCCO: c_uint = 0x10	/* RCVCC overflow */;
pub const RNTPCO: c_uint = 0x08	/* RNTPC overflow */;
pub const MPCO: c_uint = 0x04	/* MPC overflow */;
pub const RCVINT: c_uint = 0x02	/* receive interrupt */;
pub const XMTINT: c_uint = 0x01	/* transmitter interrupt */;
// Bits in PR
pub const XMTSV: c_uint = 0x80	/* XMTFS valid (same as in XMTFS) */;
pub const TDTREQ: c_uint = 0x40	/* set when xmit fifo is requesting data */;
pub const RDTREQ: c_uint = 0x20	/* set when recv fifo requests data xfer */;
// Bits in BIUCC
pub const BSWP: c_uint = 0x40	/* byte swap, i.e. big-endian bus */;
pub const XMTSP_4: c_uint = 0x00	/* start xmitting when 4 bytes in FIFO */;
pub const XMTSP_16: c_uint = 0x10	/* start xmitting when 16 bytes in FIFO */;
pub const XMTSP_64: c_uint = 0x20	/* start xmitting when 64 bytes in FIFO */;
pub const XMTSP_112: c_uint = 0x30	/* start xmitting when 112 bytes in FIFO */;
pub const SWRST: c_uint = 0x01	/* software reset */;
// Bits in FIFOCC
pub const XMTFW_8: c_uint = 0x00	/* xmit fifo watermark = 8 words free */;
pub const XMTFW_16: c_uint = 0x40	/*  16 words free */;
pub const XMTFW_32: c_uint = 0x80	/*  32 words free */;
pub const RCVFW_16: c_uint = 0x00	/* recv fifo watermark = 16 bytes avail */;
pub const RCVFW_32: c_uint = 0x10	/*  32 bytes avail */;
pub const RCVFW_64: c_uint = 0x20	/*  64 bytes avail */;
pub const XMTFWU: c_uint = 0x08	/* xmit fifo watermark update enable */;
pub const RCVFWU: c_uint = 0x04	/* recv fifo watermark update enable */;
pub const XMTBRST: c_uint = 0x02	/* enable transmit burst mode */;
pub const RCVBRST: c_uint = 0x01	/* enable receive burst mode */;
// Bits in MACCC
pub const PROM: c_uint = 0x80	/* promiscuous mode */;
pub const DXMT2PD: c_uint = 0x40	/* disable xmit two-part deferral algorithm */;
pub const EMBA: c_uint = 0x20	/* enable modified backoff algorithm */;
pub const DRCVPA: c_uint = 0x08	/* disable receiving physical address */;
pub const DRCVBC: c_uint = 0x04	/* disable receiving broadcasts */;
pub const ENXMT: c_uint = 0x02	/* enable transmitter */;
pub const ENRCV: c_uint = 0x01	/* enable receiver */;
// Bits in PLSCC
pub const XMTSEL: c_uint = 0x08	/* select DO+/DO- state when idle */;
pub const PORTSEL_AUI: c_uint = 0x00	/* select AUI port */;
pub const PORTSEL_10T: c_uint = 0x02	/* select 10Base-T port */;
pub const PORTSEL_DAI: c_uint = 0x04	/* select DAI port */;
pub const PORTSEL_GPSI: c_uint = 0x06	/* select GPSI port */;
pub const ENPLSIO: c_uint = 0x01	/* enable optional PLS I/O pins */;
// Bits in PHYCC
pub const LNKFL: c_uint = 0x80	/* reports 10Base-T link failure */;
pub const DLNKTST: c_uint = 0x40	/* disable 10Base-T link test */;
pub const REVPOL: c_uint = 0x20	/* 10Base-T receiver polarity reversed */;
pub const DAPC: c_uint = 0x10	/* disable auto receiver polarity correction */;
pub const LRT: c_uint = 0x08	/* low receive threshold for long links */;
pub const ASEL: c_uint = 0x04	/* auto-select AUI or 10Base-T port */;
pub const RWAKE: c_uint = 0x02	/* remote wake function */;
pub const AWAKE: c_uint = 0x01	/* auto wake function */;
// Bits in IAC
pub const ADDRCHG: c_uint = 0x80	/* request address change */;
pub const PHYADDR: c_uint = 0x04	/* access physical address */;
pub const LOGADDR: c_uint = 0x02	/* access multicast filter */;
// Bits in UTR
pub const RTRE: c_uint = 0x80	/* reserved test register enable. DON'T SET. */;
pub const RTRD: c_uint = 0x40	/* reserved test register disable.  Sticky */;
pub const RPAC: c_uint = 0x20	/* accept runt packets */;
pub const FCOLL: c_uint = 0x10	/* force collision */;
pub const RCVFCSE: c_uint = 0x08	/* receive FCS enable */;
pub const LOOP_NONE: c_uint = 0x00	/* no loopback */;
pub const LOOP_EXT: c_uint = 0x02	/* external loopback */;
pub const LOOP_INT: c_uint = 0x04	/* internal loopback, excludes MENDEC */;
pub const LOOP_MENDEC: c_uint = 0x06	/* internal loopback, includes MENDEC */;
