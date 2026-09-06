//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sun/sungem.h
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
// $Id: sungem.h,v 1.10.2.4 2002/03/11 08:54:48 davem Exp $
// sungem.h: Definitions for Sun GEM ethernet driver.
//
// Copyright (C) 2000 David S. Miller (davem@redhat.com)
//
// Global Registers
pub const GREG_SEBSTATE: c_uint = 0x0000UL	/* SEB State Register		*/;
pub const GREG_CFG: c_uint = 0x0004UL	/* Configuration Register	*/;
pub const GREG_STAT: c_uint = 0x000CUL	/* Status Register		*/;
pub const GREG_IMASK: c_uint = 0x0010UL	/* Interrupt Mask Register	*/;
pub const GREG_IACK: c_uint = 0x0014UL	/* Interrupt ACK Register	*/;
pub const GREG_STAT2: c_uint = 0x001CUL	/* Alias of GREG_STAT		*/;
pub const GREG_PCIESTAT: c_uint = 0x1000UL	/* PCI Error Status Register	*/;
pub const GREG_PCIEMASK: c_uint = 0x1004UL	/* PCI Error Mask Register	*/;
pub const GREG_BIFCFG: c_uint = 0x1008UL	/* BIF Configuration Register	*/;
pub const GREG_BIFDIAG: c_uint = 0x100CUL	/* BIF Diagnostics Register	*/;
pub const GREG_SWRST: c_uint = 0x1010UL	/* Software Reset Register	*/;
// Global SEB State Register
pub const GREG_SEBSTATE_ARB: c_uint = 0x00000003	/* State of Arbiter		*/;
pub const GREG_SEBSTATE_RXWON: c_uint = 0x00000004	/* RX won internal arbitration	*/;
// Global Configuration Register
pub const GREG_CFG_IBURST: c_uint = 0x00000001	/* Infinite Burst		*/;
pub const GREG_CFG_TXDMALIM: c_uint = 0x0000003e	/* TX DMA grant limit		*/;
pub const GREG_CFG_RXDMALIM: c_uint = 0x000007c0	/* RX DMA grant limit		*/;
pub const GREG_CFG_RONPAULBIT: c_uint = 0x00000800	/* Use mem read multiple for PCI read;
// after infinite burst (Apple)
pub const GREG_CFG_ENBUG2FIX: c_uint = 0x00001000	/* Fix Rx hang after overflow */;
// Global Interrupt Status Register.
//
// Reading this register automatically clears bits 0 through 6.
// This auto-clearing does not occur when the alias at GREG_STAT2
// is read instead.  The rest of the interrupt bits only clear when
// the secondary interrupt status register corresponding to that
// bit is read (ie. if GREG_STAT_PCS is set, it will be cleared by
// reading PCS_ISTAT).
//
pub const GREG_STAT_TXINTME: c_uint = 0x00000001	/* TX INTME frame transferred	*/;
pub const GREG_STAT_TXALL: c_uint = 0x00000002	/* All TX frames transferred	*/;
pub const GREG_STAT_TXDONE: c_uint = 0x00000004	/* One TX frame transferred	*/;
pub const GREG_STAT_RXDONE: c_uint = 0x00000010	/* One RX frame arrived		*/;
pub const GREG_STAT_RXNOBUF: c_uint = 0x00000020	/* No free RX buffers available	*/;
pub const GREG_STAT_RXTAGERR: c_uint = 0x00000040	/* RX tag framing is corrupt	*/;
pub const GREG_STAT_PCS: c_uint = 0x00002000	/* PCS signalled interrupt	*/;
pub const GREG_STAT_TXMAC: c_uint = 0x00004000	/* TX MAC signalled interrupt	*/;
pub const GREG_STAT_RXMAC: c_uint = 0x00008000	/* RX MAC signalled interrupt	*/;
pub const GREG_STAT_MAC: c_uint = 0x00010000	/* MAC Control signalled irq	*/;
pub const GREG_STAT_MIF: c_uint = 0x00020000	/* MIF signalled interrupt	*/;
pub const GREG_STAT_PCIERR: c_uint = 0x00040000	/* PCI Error interrupt		*/;
pub const GREG_STAT_TXNR: c_uint = 0xfff80000	/* == TXDMA_TXDONE reg val	*/;
pub const GREG_STAT_TXNR_SHIFT: c_int = 19;

// The layout of GREG_IMASK and GREG_IACK is identical to GREG_STAT.
// Bits set in GREG_IMASK will prevent that interrupt type from being
// signalled to the cpu.  GREG_IACK can be used to clear specific top-level
// interrupt conditions in GREG_STAT, ie. it only works for bits 0 through 6.
// Setting the bit will clear that interrupt, clear bits will have no effect
// on GREG_STAT.
//
// Global PCI Error Status Register
pub const GREG_PCIESTAT_BADACK: c_uint = 0x00000001	/* No ACK64# during ABS64 cycle	*/;
pub const GREG_PCIESTAT_DTRTO: c_uint = 0x00000002	/* Delayed transaction timeout	*/;
pub const GREG_PCIESTAT_OTHER: c_uint = 0x00000004	/* Other PCI error, check cfg space */;
// The layout of the GREG_PCIEMASK is identical to that of GREG_PCIESTAT.
// Bits set in GREG_PCIEMASK will prevent that interrupt type from being
// signalled to the cpu.
//
// Global BIF Configuration Register
pub const GREG_BIFCFG_SLOWCLK: c_uint = 0x00000001	/* Set if PCI runs < 25Mhz	*/;
pub const GREG_BIFCFG_B64DIS: c_uint = 0x00000002	/* Disable 64bit wide data cycle*/;
pub const GREG_BIFCFG_M66EN: c_uint = 0x00000004	/* Set if on 66Mhz PCI segment	*/;
// Global BIF Diagnostics Register
pub const GREG_BIFDIAG_BURSTSM: c_uint = 0x007f0000	/* PCI Burst state machine	*/;
pub const GREG_BIFDIAG_BIFSM: c_uint = 0xff000000	/* BIF state machine		*/;
// Global Software Reset Register.
//
// This register is used to perform a global reset of the RX and TX portions
// of the GEM asic.  Setting the RX or TX reset bit will start the reset.
// The driver _MUST_ poll these bits until they clear.  One may not attempt
// to program any other part of GEM until the bits clear.
//
pub const GREG_SWRST_TXRST: c_uint = 0x00000001	/* TX Software Reset		*/;
pub const GREG_SWRST_RXRST: c_uint = 0x00000002	/* RX Software Reset		*/;
pub const GREG_SWRST_RSTOUT: c_uint = 0x00000004	/* Force RST# pin active	*/;
pub const GREG_SWRST_CACHESIZE: c_uint = 0x00ff0000	/* RIO only: cache line size	*/;
pub const GREG_SWRST_CACHE_SHIFT: c_int = 16;
// TX DMA Registers
pub const TXDMA_KICK: c_uint = 0x2000UL	/* TX Kick Register		*/;
pub const TXDMA_CFG: c_uint = 0x2004UL	/* TX Configuration Register	*/;
pub const TXDMA_DBLOW: c_uint = 0x2008UL	/* TX Desc. Base Low		*/;
pub const TXDMA_DBHI: c_uint = 0x200CUL	/* TX Desc. Base High		*/;
pub const TXDMA_FWPTR: c_uint = 0x2014UL	/* TX FIFO Write Pointer	*/;
pub const TXDMA_FSWPTR: c_uint = 0x2018UL	/* TX FIFO Shadow Write Pointer	*/;
pub const TXDMA_FRPTR: c_uint = 0x201CUL	/* TX FIFO Read Pointer		*/;
pub const TXDMA_FSRPTR: c_uint = 0x2020UL	/* TX FIFO Shadow Read Pointer	*/;
pub const TXDMA_PCNT: c_uint = 0x2024UL	/* TX FIFO Packet Counter	*/;
pub const TXDMA_SMACHINE: c_uint = 0x2028UL	/* TX State Machine Register	*/;
pub const TXDMA_DPLOW: c_uint = 0x2030UL	/* TX Data Pointer Low		*/;
pub const TXDMA_DPHI: c_uint = 0x2034UL	/* TX Data Pointer High		*/;
pub const TXDMA_TXDONE: c_uint = 0x2100UL	/* TX Completion Register	*/;
pub const TXDMA_FADDR: c_uint = 0x2104UL	/* TX FIFO Address		*/;
pub const TXDMA_FTAG: c_uint = 0x2108UL	/* TX FIFO Tag			*/;
pub const TXDMA_DLOW: c_uint = 0x210CUL	/* TX FIFO Data Low		*/;
pub const TXDMA_DHIT1: c_uint = 0x2110UL	/* TX FIFO Data HighT1		*/;
pub const TXDMA_DHIT0: c_uint = 0x2114UL	/* TX FIFO Data HighT0		*/;
pub const TXDMA_FSZ: c_uint = 0x2118UL	/* TX FIFO Size			*/;
// TX Kick Register.
//
// This 13-bit register is programmed by the driver to hold the descriptor
// entry index which follows the last valid transmit descriptor.
//
// TX Completion Register.
//
// This 13-bit register is updated by GEM to hold to descriptor entry index
// which follows the last descriptor already processed by GEM.  Note that
// this value is mirrored in GREG_STAT which eliminates the need to even
// access this register in the driver during interrupt processing.
//
// TX Configuration Register.
//
// Note that TXDMA_CFG_FTHRESH, the TX FIFO Threshold, is an obsolete feature
// that was meant to be used with jumbo packets.  It should be set to the
// maximum value of 0x4ff, else one risks getting TX MAC Underrun errors.
//
pub const TXDMA_CFG_ENABLE: c_uint = 0x00000001	/* Enable TX DMA channel	*/;
pub const TXDMA_CFG_RINGSZ: c_uint = 0x0000001e	/* TX descriptor ring size	*/;
pub const TXDMA_CFG_RINGSZ_32: c_uint = 0x00000000	/* 32 TX descriptors		*/;
pub const TXDMA_CFG_RINGSZ_64: c_uint = 0x00000002	/* 64 TX descriptors		*/;
pub const TXDMA_CFG_RINGSZ_128: c_uint = 0x00000004	/* 128 TX descriptors		*/;
pub const TXDMA_CFG_RINGSZ_256: c_uint = 0x00000006	/* 256 TX descriptors		*/;
pub const TXDMA_CFG_RINGSZ_512: c_uint = 0x00000008	/* 512 TX descriptors		*/;
pub const TXDMA_CFG_RINGSZ_1K: c_uint = 0x0000000a	/* 1024 TX descriptors		*/;
pub const TXDMA_CFG_RINGSZ_2K: c_uint = 0x0000000c	/* 2048 TX descriptors		*/;
pub const TXDMA_CFG_RINGSZ_4K: c_uint = 0x0000000e	/* 4096 TX descriptors		*/;
pub const TXDMA_CFG_RINGSZ_8K: c_uint = 0x00000010	/* 8192 TX descriptors		*/;
pub const TXDMA_CFG_PIOSEL: c_uint = 0x00000020	/* Enable TX FIFO PIO from cpu	*/;
pub const TXDMA_CFG_FTHRESH: c_uint = 0x001ffc00	/* TX FIFO Threshold, obsolete	*/;
pub const TXDMA_CFG_PMODE: c_uint = 0x00200000	/* TXALL irq means TX FIFO empty*/;
// TX Descriptor Base Low/High.
//
// These two registers store the 53 most significant bits of the base address
// of the TX descriptor table.  The 11 least significant bits are always
// zero.  As a result, the TX descriptor table must be 2K aligned.
//
// The rest of the TXDMA_* registers are for diagnostics and debug, I will document
// them later. -DaveM
//
// WakeOnLan Registers
pub const WOL_MATCH0: c_uint = 0x3000UL;
pub const WOL_MATCH1: c_uint = 0x3004UL;
pub const WOL_MATCH2: c_uint = 0x3008UL;
pub const WOL_MCOUNT: c_uint = 0x300CUL;
pub const WOL_WAKECSR: c_uint = 0x3010UL;
// WOL Match count register
//
pub const WOL_MCOUNT_N: c_uint = 0x00000010;
pub const WOL_MCOUNT_M: c_uint = 0x00000000 /* 0 << 8 */;
pub const WOL_WAKECSR_ENABLE: c_uint = 0x00000001;
pub const WOL_WAKECSR_MII: c_uint = 0x00000002;
pub const WOL_WAKECSR_SEEN: c_uint = 0x00000004;
pub const WOL_WAKECSR_FILT_UCAST: c_uint = 0x00000008;
pub const WOL_WAKECSR_FILT_MCAST: c_uint = 0x00000010;
pub const WOL_WAKECSR_FILT_BCAST: c_uint = 0x00000020;
pub const WOL_WAKECSR_FILT_SEEN: c_uint = 0x00000040;
// Receive DMA Registers
pub const RXDMA_CFG: c_uint = 0x4000UL	/* RX Configuration Register	*/;
pub const RXDMA_DBLOW: c_uint = 0x4004UL	/* RX Descriptor Base Low	*/;
pub const RXDMA_DBHI: c_uint = 0x4008UL	/* RX Descriptor Base High	*/;
pub const RXDMA_FWPTR: c_uint = 0x400CUL	/* RX FIFO Write Pointer	*/;
pub const RXDMA_FSWPTR: c_uint = 0x4010UL	/* RX FIFO Shadow Write Pointer	*/;
pub const RXDMA_FRPTR: c_uint = 0x4014UL	/* RX FIFO Read Pointer		*/;
pub const RXDMA_PCNT: c_uint = 0x4018UL	/* RX FIFO Packet Counter	*/;
pub const RXDMA_SMACHINE: c_uint = 0x401CUL	/* RX State Machine Register	*/;
pub const RXDMA_PTHRESH: c_uint = 0x4020UL	/* Pause Thresholds		*/;
pub const RXDMA_DPLOW: c_uint = 0x4024UL	/* RX Data Pointer Low		*/;
pub const RXDMA_DPHI: c_uint = 0x4028UL	/* RX Data Pointer High		*/;
pub const RXDMA_KICK: c_uint = 0x4100UL	/* RX Kick Register		*/;
pub const RXDMA_DONE: c_uint = 0x4104UL	/* RX Completion Register	*/;
pub const RXDMA_BLANK: c_uint = 0x4108UL	/* RX Blanking Register		*/;
pub const RXDMA_FADDR: c_uint = 0x410CUL	/* RX FIFO Address		*/;
pub const RXDMA_FTAG: c_uint = 0x4110UL	/* RX FIFO Tag			*/;
pub const RXDMA_DLOW: c_uint = 0x4114UL	/* RX FIFO Data Low		*/;
pub const RXDMA_DHIT1: c_uint = 0x4118UL	/* RX FIFO Data HighT0		*/;
pub const RXDMA_DHIT0: c_uint = 0x411CUL	/* RX FIFO Data HighT1		*/;
pub const RXDMA_FSZ: c_uint = 0x4120UL	/* RX FIFO Size			*/;
// RX Configuration Register.
pub const RXDMA_CFG_ENABLE: c_uint = 0x00000001	/* Enable RX DMA channel	*/;
pub const RXDMA_CFG_RINGSZ: c_uint = 0x0000001e	/* RX descriptor ring size	*/;
pub const RXDMA_CFG_RINGSZ_32: c_uint = 0x00000000	/* - 32   entries		*/;
pub const RXDMA_CFG_RINGSZ_64: c_uint = 0x00000002	/* - 64   entries		*/;
pub const RXDMA_CFG_RINGSZ_128: c_uint = 0x00000004	/* - 128  entries		*/;
pub const RXDMA_CFG_RINGSZ_256: c_uint = 0x00000006	/* - 256  entries		*/;
pub const RXDMA_CFG_RINGSZ_512: c_uint = 0x00000008	/* - 512  entries		*/;
pub const RXDMA_CFG_RINGSZ_1K: c_uint = 0x0000000a	/* - 1024 entries		*/;
pub const RXDMA_CFG_RINGSZ_2K: c_uint = 0x0000000c	/* - 2048 entries		*/;
pub const RXDMA_CFG_RINGSZ_4K: c_uint = 0x0000000e	/* - 4096 entries		*/;
pub const RXDMA_CFG_RINGSZ_8K: c_uint = 0x00000010	/* - 8192 entries		*/;
pub const RXDMA_CFG_RINGSZ_BDISAB: c_uint = 0x00000020	/* Disable RX desc batching	*/;
pub const RXDMA_CFG_FBOFF: c_uint = 0x00001c00	/* Offset of first data byte	*/;
pub const RXDMA_CFG_CSUMOFF: c_uint = 0x000fe000	/* Skip bytes before csum calc	*/;
pub const RXDMA_CFG_FTHRESH: c_uint = 0x07000000	/* RX FIFO dma start threshold	*/;
pub const RXDMA_CFG_FTHRESH_64: c_uint = 0x00000000	/* - 64   bytes			*/;
pub const RXDMA_CFG_FTHRESH_128: c_uint = 0x01000000	/* - 128  bytes			*/;
pub const RXDMA_CFG_FTHRESH_256: c_uint = 0x02000000	/* - 256  bytes			*/;
pub const RXDMA_CFG_FTHRESH_512: c_uint = 0x03000000	/* - 512  bytes			*/;
pub const RXDMA_CFG_FTHRESH_1K: c_uint = 0x04000000	/* - 1024 bytes			*/;
pub const RXDMA_CFG_FTHRESH_2K: c_uint = 0x05000000	/* - 2048 bytes			*/;
// RX Descriptor Base Low/High.
//
// These two registers store the 53 most significant bits of the base address
// of the RX descriptor table.  The 11 least significant bits are always
// zero.  As a result, the RX descriptor table must be 2K aligned.
//
// RX PAUSE Thresholds.
//
// These values determine when XOFF and XON PAUSE frames are emitted by
// GEM.  The thresholds measure RX FIFO occupancy in units of 64 bytes.
//
pub const RXDMA_PTHRESH_OFF: c_uint = 0x000001ff	/* XOFF emitted w/FIFO > this	*/;
pub const RXDMA_PTHRESH_ON: c_uint = 0x001ff000	/* XON emitted w/FIFO < this	*/;
// RX Kick Register.
//
// This 13-bit register is written by the host CPU and holds the last
// valid RX descriptor number plus one.  This is, if 'N' is written to
// this register, it means that all RX descriptors up to but excluding
// 'N' are valid.
//
// The hardware requires that RX descriptors are posted in increments
// of 4.  This means 'N' must be a multiple of four.  For the best
// performance, the first new descriptor being posted should be (PCI)
// cache line aligned.
//
// RX Completion Register.
//
// This 13-bit register is updated by GEM to indicate which RX descriptors
// have already been used for receive frames.  All descriptors up to but
// excluding the value in this register are ready to be processed.  GEM
// updates this register value after the RX FIFO empties completely into
// the RX descriptor's buffer, but before the RX_DONE bit is set in the
// interrupt status register.
//
// RX Blanking Register.
pub const RXDMA_BLANK_IPKTS: c_uint = 0x000001ff	/* RX_DONE asserted after this;
// many packets received since
// previous RX_DONE.
//
pub const RXDMA_BLANK_ITIME: c_uint = 0x000ff000	/* RX_DONE asserted after this;
// many clocks (measured in 2048
// PCI clocks) were counted since
// the previous RX_DONE.
//
// RX FIFO Size.
//
// This 11-bit read-only register indicates how large, in units of 64-bytes,
// the RX FIFO is.  The driver uses this to properly configure the RX PAUSE
// thresholds.
//
// The rest of the RXDMA_* registers are for diagnostics and debug, I will document
// them later. -DaveM
//
// MAC Registers
pub const MAC_TXRST: c_uint = 0x6000UL	/* TX MAC Software Reset Command*/;
pub const MAC_RXRST: c_uint = 0x6004UL	/* RX MAC Software Reset Command*/;
pub const MAC_SNDPAUSE: c_uint = 0x6008UL	/* Send Pause Command Register	*/;
pub const MAC_TXSTAT: c_uint = 0x6010UL	/* TX MAC Status Register	*/;
pub const MAC_RXSTAT: c_uint = 0x6014UL	/* RX MAC Status Register	*/;
pub const MAC_CSTAT: c_uint = 0x6018UL	/* MAC Control Status Register	*/;
pub const MAC_TXMASK: c_uint = 0x6020UL	/* TX MAC Mask Register		*/;
pub const MAC_RXMASK: c_uint = 0x6024UL	/* RX MAC Mask Register		*/;
pub const MAC_MCMASK: c_uint = 0x6028UL	/* MAC Control Mask Register	*/;
pub const MAC_TXCFG: c_uint = 0x6030UL	/* TX MAC Configuration Register*/;
pub const MAC_RXCFG: c_uint = 0x6034UL	/* RX MAC Configuration Register*/;
pub const MAC_MCCFG: c_uint = 0x6038UL	/* MAC Control Config Register	*/;
pub const MAC_XIFCFG: c_uint = 0x603CUL	/* XIF Configuration Register	*/;
pub const MAC_IPG0: c_uint = 0x6040UL	/* InterPacketGap0 Register	*/;
pub const MAC_IPG1: c_uint = 0x6044UL	/* InterPacketGap1 Register	*/;
pub const MAC_IPG2: c_uint = 0x6048UL	/* InterPacketGap2 Register	*/;
pub const MAC_STIME: c_uint = 0x604CUL	/* SlotTime Register		*/;
pub const MAC_MINFSZ: c_uint = 0x6050UL	/* MinFrameSize Register	*/;
pub const MAC_MAXFSZ: c_uint = 0x6054UL	/* MaxFrameSize Register	*/;
pub const MAC_PASIZE: c_uint = 0x6058UL	/* PA Size Register		*/;
pub const MAC_JAMSIZE: c_uint = 0x605CUL	/* JamSize Register		*/;
pub const MAC_ATTLIM: c_uint = 0x6060UL	/* Attempt Limit Register	*/;
pub const MAC_MCTYPE: c_uint = 0x6064UL	/* MAC Control Type Register	*/;
pub const MAC_ADDR0: c_uint = 0x6080UL	/* MAC Address 0 Register	*/;
pub const MAC_ADDR1: c_uint = 0x6084UL	/* MAC Address 1 Register	*/;
pub const MAC_ADDR2: c_uint = 0x6088UL	/* MAC Address 2 Register	*/;
pub const MAC_ADDR3: c_uint = 0x608CUL	/* MAC Address 3 Register	*/;
pub const MAC_ADDR4: c_uint = 0x6090UL	/* MAC Address 4 Register	*/;
pub const MAC_ADDR5: c_uint = 0x6094UL	/* MAC Address 5 Register	*/;
pub const MAC_ADDR6: c_uint = 0x6098UL	/* MAC Address 6 Register	*/;
pub const MAC_ADDR7: c_uint = 0x609CUL	/* MAC Address 7 Register	*/;
pub const MAC_ADDR8: c_uint = 0x60A0UL	/* MAC Address 8 Register	*/;
pub const MAC_AFILT0: c_uint = 0x60A4UL	/* Address Filter 0 Register	*/;
pub const MAC_AFILT1: c_uint = 0x60A8UL	/* Address Filter 1 Register	*/;
pub const MAC_AFILT2: c_uint = 0x60ACUL	/* Address Filter 2 Register	*/;
pub const MAC_AF21MSK: c_uint = 0x60B0UL	/* Address Filter 2&1 Mask Reg	*/;
pub const MAC_AF0MSK: c_uint = 0x60B4UL	/* Address Filter 0 Mask Reg	*/;
pub const MAC_HASH0: c_uint = 0x60C0UL	/* Hash Table 0 Register	*/;
pub const MAC_HASH1: c_uint = 0x60C4UL	/* Hash Table 1 Register	*/;
pub const MAC_HASH2: c_uint = 0x60C8UL	/* Hash Table 2 Register	*/;
pub const MAC_HASH3: c_uint = 0x60CCUL	/* Hash Table 3 Register	*/;
pub const MAC_HASH4: c_uint = 0x60D0UL	/* Hash Table 4 Register	*/;
pub const MAC_HASH5: c_uint = 0x60D4UL	/* Hash Table 5 Register	*/;
pub const MAC_HASH6: c_uint = 0x60D8UL	/* Hash Table 6 Register	*/;
pub const MAC_HASH7: c_uint = 0x60DCUL	/* Hash Table 7 Register	*/;
pub const MAC_HASH8: c_uint = 0x60E0UL	/* Hash Table 8 Register	*/;
pub const MAC_HASH9: c_uint = 0x60E4UL	/* Hash Table 9 Register	*/;
pub const MAC_HASH10: c_uint = 0x60E8UL	/* Hash Table 10 Register	*/;
pub const MAC_HASH11: c_uint = 0x60ECUL	/* Hash Table 11 Register	*/;
pub const MAC_HASH12: c_uint = 0x60F0UL	/* Hash Table 12 Register	*/;
pub const MAC_HASH13: c_uint = 0x60F4UL	/* Hash Table 13 Register	*/;
pub const MAC_HASH14: c_uint = 0x60F8UL	/* Hash Table 14 Register	*/;
pub const MAC_HASH15: c_uint = 0x60FCUL	/* Hash Table 15 Register	*/;
pub const MAC_NCOLL: c_uint = 0x6100UL	/* Normal Collision Counter	*/;
pub const MAC_FASUCC: c_uint = 0x6104UL	/* First Attempt. Succ Coll Ctr.*/;
pub const MAC_ECOLL: c_uint = 0x6108UL	/* Excessive Collision Counter	*/;
pub const MAC_LCOLL: c_uint = 0x610CUL	/* Late Collision Counter	*/;
pub const MAC_DTIMER: c_uint = 0x6110UL	/* Defer Timer			*/;
pub const MAC_PATMPS: c_uint = 0x6114UL	/* Peak Attempts Register	*/;
pub const MAC_RFCTR: c_uint = 0x6118UL	/* Receive Frame Counter	*/;
pub const MAC_LERR: c_uint = 0x611CUL	/* Length Error Counter		*/;
pub const MAC_AERR: c_uint = 0x6120UL	/* Alignment Error Counter	*/;
pub const MAC_FCSERR: c_uint = 0x6124UL	/* FCS Error Counter		*/;
pub const MAC_RXCVERR: c_uint = 0x6128UL	/* RX code Violation Error Ctr	*/;
pub const MAC_RANDSEED: c_uint = 0x6130UL	/* Random Number Seed Register	*/;
pub const MAC_SMACHINE: c_uint = 0x6134UL	/* State Machine Register	*/;
// TX MAC Software Reset Command.
pub const MAC_TXRST_CMD: c_uint = 0x00000001	/* Start sw reset, self-clears	*/;
// RX MAC Software Reset Command.
pub const MAC_RXRST_CMD: c_uint = 0x00000001	/* Start sw reset, self-clears	*/;
// Send Pause Command.
pub const MAC_SNDPAUSE_TS: c_uint = 0x0000ffff	/* The pause_time operand used in;
// Send_Pause and flow-control
// handshakes.
//
pub const MAC_SNDPAUSE_SP: c_uint = 0x00010000	/* Setting this bit instructs the MAC;
// to send a Pause Flow Control
// frame onto the network.
//
// TX MAC Status Register.
pub const MAC_TXSTAT_XMIT: c_uint = 0x00000001	/* Frame Transmitted		*/;
pub const MAC_TXSTAT_URUN: c_uint = 0x00000002	/* TX Underrun			*/;
pub const MAC_TXSTAT_MPE: c_uint = 0x00000004	/* Max Packet Size Error	*/;
pub const MAC_TXSTAT_NCE: c_uint = 0x00000008	/* Normal Collision Cntr Expire	*/;
pub const MAC_TXSTAT_ECE: c_uint = 0x00000010	/* Excess Collision Cntr Expire	*/;
pub const MAC_TXSTAT_LCE: c_uint = 0x00000020	/* Late Collision Cntr Expire	*/;
pub const MAC_TXSTAT_FCE: c_uint = 0x00000040	/* First Collision Cntr Expire	*/;
pub const MAC_TXSTAT_DTE: c_uint = 0x00000080	/* Defer Timer Expire		*/;
pub const MAC_TXSTAT_PCE: c_uint = 0x00000100	/* Peak Attempts Cntr Expire	*/;
// RX MAC Status Register.
pub const MAC_RXSTAT_RCV: c_uint = 0x00000001	/* Frame Received		*/;
pub const MAC_RXSTAT_OFLW: c_uint = 0x00000002	/* Receive Overflow		*/;
pub const MAC_RXSTAT_FCE: c_uint = 0x00000004	/* Frame Cntr Expire		*/;
pub const MAC_RXSTAT_ACE: c_uint = 0x00000008	/* Align Error Cntr Expire	*/;
pub const MAC_RXSTAT_CCE: c_uint = 0x00000010	/* CRC Error Cntr Expire	*/;
pub const MAC_RXSTAT_LCE: c_uint = 0x00000020	/* Length Error Cntr Expire	*/;
pub const MAC_RXSTAT_VCE: c_uint = 0x00000040	/* Code Violation Cntr Expire	*/;
// MAC Control Status Register.
pub const MAC_CSTAT_PRCV: c_uint = 0x00000001	/* Pause Received		*/;
pub const MAC_CSTAT_PS: c_uint = 0x00000002	/* Paused State			*/;
pub const MAC_CSTAT_NPS: c_uint = 0x00000004	/* Not Paused State		*/;
pub const MAC_CSTAT_PTR: c_uint = 0xffff0000	/* Pause Time Received		*/;
// The layout of the MAC_{TX,RX,C}MASK registers is identical to that
// of MAC_{TX,RX,C}STAT.  Bits set in MAC_{TX,RX,C}MASK will prevent
// that interrupt type from being signalled to front end of GEM.  For
// the interrupt to actually get sent to the cpu, it is necessary to
// properly set the appropriate GREG_IMASK_{TX,RX,}MAC bits as well.
//
// TX MAC Configuration Register.
//
// NOTE: The TX MAC Enable bit must be cleared and polled until
// zero before any other bits in this register are changed.
//
// Also, enabling the Carrier Extension feature of GEM is
// a 3 step process 1) Set TX Carrier Extension 2) Set
// RX Carrier Extension 3) Set Slot Time to 0x200.  This
// mode must be enabled when in half-duplex at 1Gbps, else
// it must be disabled.
//
pub const MAC_TXCFG_ENAB: c_uint = 0x00000001	/* TX MAC Enable		*/;
pub const MAC_TXCFG_ICS: c_uint = 0x00000002	/* Ignore Carrier Sense		*/;
pub const MAC_TXCFG_ICOLL: c_uint = 0x00000004	/* Ignore Collisions		*/;
pub const MAC_TXCFG_EIPG0: c_uint = 0x00000008	/* Enable IPG0			*/;
pub const MAC_TXCFG_NGU: c_uint = 0x00000010	/* Never Give Up		*/;
pub const MAC_TXCFG_NGUL: c_uint = 0x00000020	/* Never Give Up Limit		*/;
pub const MAC_TXCFG_NBO: c_uint = 0x00000040	/* No Backoff			*/;
pub const MAC_TXCFG_SD: c_uint = 0x00000080	/* Slow Down			*/;
pub const MAC_TXCFG_NFCS: c_uint = 0x00000100	/* No FCS			*/;
pub const MAC_TXCFG_TCE: c_uint = 0x00000200	/* TX Carrier Extension		*/;
// RX MAC Configuration Register.
//
// NOTE: The RX MAC Enable bit must be cleared and polled until
// zero before any other bits in this register are changed.
//
// Similar rules apply to the Hash Filter Enable bit when
// programming the hash table registers, and the Address Filter
// Enable bit when programming the address filter registers.
//
pub const MAC_RXCFG_ENAB: c_uint = 0x00000001	/* RX MAC Enable		*/;
pub const MAC_RXCFG_SPAD: c_uint = 0x00000002	/* Strip Pad			*/;
pub const MAC_RXCFG_SFCS: c_uint = 0x00000004	/* Strip FCS			*/;
pub const MAC_RXCFG_PROM: c_uint = 0x00000008	/* Promiscuous Mode		*/;
pub const MAC_RXCFG_PGRP: c_uint = 0x00000010	/* Promiscuous Group		*/;
pub const MAC_RXCFG_HFE: c_uint = 0x00000020	/* Hash Filter Enable		*/;
pub const MAC_RXCFG_AFE: c_uint = 0x00000040	/* Address Filter Enable	*/;
pub const MAC_RXCFG_DDE: c_uint = 0x00000080	/* Disable Discard on Error	*/;
pub const MAC_RXCFG_RCE: c_uint = 0x00000100	/* RX Carrier Extension		*/;
// MAC Control Config Register.
pub const MAC_MCCFG_SPE: c_uint = 0x00000001	/* Send Pause Enable		*/;
pub const MAC_MCCFG_RPE: c_uint = 0x00000002	/* Receive Pause Enable		*/;
pub const MAC_MCCFG_PMC: c_uint = 0x00000004	/* Pass MAC Control		*/;
// XIF Configuration Register.
//
// NOTE: When leaving or entering loopback mode, a global hardware
// init of GEM should be performed.
//
pub const MAC_XIFCFG_OE: c_uint = 0x00000001	/* MII TX Output Driver Enable	*/;
pub const MAC_XIFCFG_LBCK: c_uint = 0x00000002	/* Loopback TX to RX		*/;
pub const MAC_XIFCFG_DISE: c_uint = 0x00000004	/* Disable RX path during TX	*/;
pub const MAC_XIFCFG_GMII: c_uint = 0x00000008	/* Use GMII clocks + datapath	*/;
pub const MAC_XIFCFG_MBOE: c_uint = 0x00000010	/* Controls MII_BUF_EN pin	*/;
pub const MAC_XIFCFG_LLED: c_uint = 0x00000020	/* Force LINKLED# active (low)	*/;
pub const MAC_XIFCFG_FLED: c_uint = 0x00000040	/* Force FDPLXLED# active (low)	*/;
// InterPacketGap0 Register.  This 8-bit value is used as an extension
// to the InterPacketGap1 Register.  Specifically it contributes to the
// timing of the RX-to-TX IPG.  This value is ignored and presumed to
// be zero for TX-to-TX IPG calculations and/or when the Enable IPG0 bit
// is cleared in the TX MAC Configuration Register.
//
// This value in this register in terms of media byte time.
//
// Recommended value: 0x00
//
// InterPacketGap1 Register.  This 8-bit value defines the first 2/3
// portion of the Inter Packet Gap.
//
// This value in this register in terms of media byte time.
//
// Recommended value: 0x08
//
// InterPacketGap2 Register.  This 8-bit value defines the second 1/3
// portion of the Inter Packet Gap.
//
// This value in this register in terms of media byte time.
//
// Recommended value: 0x04
//
// Slot Time Register.  This 10-bit value specifies the slot time
// parameter in units of media byte time.  It determines the physical
// span of the network.
//
// Recommended value: 0x40
//
// Minimum Frame Size Register.  This 10-bit register specifies the
// smallest sized frame the TXMAC will send onto the medium, and the
// RXMAC will receive from the medium.
//
// Recommended value: 0x40
//
// Maximum Frame and Burst Size Register.
//
// This register specifies two things.  First it specifies the maximum
// sized frame the TXMAC will send and the RXMAC will recognize as
// valid.  Second, it specifies the maximum run length of a burst of
// packets sent in half-duplex gigabit modes.
//
// Recommended value: 0x200005ee
//
pub const MAC_MAXFSZ_MFS: c_uint = 0x00007fff	/* Max Frame Size		*/;
pub const MAC_MAXFSZ_MBS: c_uint = 0x7fff0000	/* Max Burst Size		*/;
// PA Size Register.  This 10-bit register specifies the number of preamble
// bytes which will be transmitted at the beginning of each frame.  A
// value of two or greater should be programmed here.
//
// Recommended value: 0x07
//
// Jam Size Register.  This 4-bit register specifies the duration of
// the jam in units of media byte time.
//
// Recommended value: 0x04
//
// Attempts Limit Register.  This 8-bit register specifies the number
// of attempts that the TXMAC will make to transmit a frame, before it
// resets its Attempts Counter.  After reaching the Attempts Limit the
// TXMAC may or may not drop the frame, as determined by the NGU
// (Never Give Up) and NGUL (Never Give Up Limit) bits in the TXMAC
// Configuration Register.
//
// Recommended value: 0x10
//
// MAX Control Type Register.  This 16-bit register specifies the
// "type" field of a MAC Control frame.  The TXMAC uses this field to
// encapsulate the MAC Control frame for transmission, and the RXMAC
// uses it for decoding valid MAC Control frames received from the
// network.
//
// Recommended value: 0x8808
//
// MAC Address Registers.  Each of these registers specify the
// ethernet MAC of the interface, 16-bits at a time.  Register
// 0 specifies bits [47:32], register 1 bits [31:16], and register
// 2 bits [15:0].
//
// Registers 3 through and including 5 specify an alternate
// MAC address for the interface.
//
// Registers 6 through and including 8 specify the MAC Control
// Address, which must be the reserved multicast address for MAC
// Control frames.
//
// Example: To program primary station address a:b:c:d:e:f into
// the chip.
// MAC_Address_2 = (a << 8) | b
// MAC_Address_1 = (c << 8) | d
// MAC_Address_0 = (e << 8) | f
//
// Address Filter Registers.  Registers 0 through 2 specify bit
// fields [47:32] through [15:0], respectively, of the address
// filter.  The Address Filter 2&1 Mask Register denotes the 8-bit
// nibble mask for Address Filter Registers 2 and 1.  The Address
// Filter 0 Mask Register denotes the 16-bit mask for the Address
// Filter Register 0.
//
// Hash Table Registers.  Registers 0 through 15 specify bit fields
// [255:240] through [15:0], respectively, of the hash table.
//
// Statistics Registers.  All of these registers are 16-bits and
// track occurrences of a specific event.  GEM can be configured
// to interrupt the host cpu when any of these counters overflow.
// They should all be explicitly initialized to zero when the interface
// is brought up.
//
// Random Number Seed Register.  This 10-bit value is used as the
// RNG seed inside GEM for the CSMA/CD backoff algorithm.  It is
// recommended to program this register to the 10 LSB of the
// interfaces MAC address.
//
// Pause Timer, read-only.  This 16-bit timer is used to time the pause
// interval as indicated by a received pause flow control frame.
// A non-zero value in this timer indicates that the MAC is currently in
// the paused state.
//
// MIF Registers
pub const MIF_BBCLK: c_uint = 0x6200UL	/* MIF Bit-Bang Clock		*/;
pub const MIF_BBDATA: c_uint = 0x6204UL	/* MIF Bit-Band Data		*/;
pub const MIF_BBOENAB: c_uint = 0x6208UL	/* MIF Bit-Bang Output Enable	*/;
pub const MIF_FRAME: c_uint = 0x620CUL	/* MIF Frame/Output Register	*/;
pub const MIF_CFG: c_uint = 0x6210UL	/* MIF Configuration Register	*/;
pub const MIF_MASK: c_uint = 0x6214UL	/* MIF Mask Register		*/;
pub const MIF_STATUS: c_uint = 0x6218UL	/* MIF Status Register		*/;
pub const MIF_SMACHINE: c_uint = 0x621CUL	/* MIF State Machine Register	*/;
// MIF Bit-Bang Clock.  This 1-bit register is used to generate the
// MDC clock waveform on the MII Management Interface when the MIF is
// programmed in the "Bit-Bang" mode.  Writing a '1' after a '0' into
// this register will create a rising edge on the MDC, while writing
// a '0' after a '1' will create a falling edge.  For every bit that
// is transferred on the management interface, both edges have to be
// generated.
//
// MIF Bit-Bang Data.  This 1-bit register is used to generate the
// outgoing data (MDO) on the MII Management Interface when the MIF
// is programmed in the "Bit-Bang" mode.  The daa will be steered to the
// appropriate MDIO based on the state of the PHY_Select bit in the MIF
// Configuration Register.
//
// MIF Big-Band Output Enable.  THis 1-bit register is used to enable
// ('1') or disable ('0') the I-directional driver on the MII when the
// MIF is programmed in the "Bit-Bang" mode.  The MDIO should be enabled
// when data bits are transferred from the MIF to the transceiver, and it
// should be disabled when the interface is idle or when data bits are
// transferred from the transceiver to the MIF (data portion of a read
// instruction).  Only one MDIO will be enabled at a given time, depending
// on the state of the PHY_Select bit in the MIF Configuration Register.
//
// MIF Configuration Register.  This 15-bit register controls the operation
// of the MIF.
//
pub const MIF_CFG_PSELECT: c_uint = 0x00000001	/* Xcvr slct: 0=mdio0 1=mdio1	*/;
pub const MIF_CFG_POLL: c_uint = 0x00000002	/* Enable polling mechanism	*/;
pub const MIF_CFG_BBMODE: c_uint = 0x00000004	/* 1=bit-bang 0=frame mode	*/;
pub const MIF_CFG_PRADDR: c_uint = 0x000000f8	/* Xcvr poll register address	*/;
pub const MIF_CFG_MDI0: c_uint = 0x00000100	/* MDIO_0 present or read-bit	*/;
pub const MIF_CFG_MDI1: c_uint = 0x00000200	/* MDIO_1 present or read-bit	*/;
pub const MIF_CFG_PPADDR: c_uint = 0x00007c00	/* Xcvr poll PHY address	*/;
// MIF Frame/Output Register.  This 32-bit register allows the host to
// communicate with a transceiver in frame mode (as opposed to big-bang
// mode).  Writes by the host specify an instruction.  After being issued
// the host must poll this register for completion.  Also, after
// completion this register holds the data returned by the transceiver
// if applicable.
//
pub const MIF_FRAME_ST: c_uint = 0xc0000000	/* STart of frame		*/;
pub const MIF_FRAME_OP: c_uint = 0x30000000	/* OPcode			*/;
pub const MIF_FRAME_PHYAD: c_uint = 0x0f800000	/* PHY ADdress			*/;
pub const MIF_FRAME_REGAD: c_uint = 0x007c0000	/* REGister ADdress		*/;
pub const MIF_FRAME_TAMSB: c_uint = 0x00020000	/* Turn Around MSB		*/;
pub const MIF_FRAME_TALSB: c_uint = 0x00010000	/* Turn Around LSB		*/;
pub const MIF_FRAME_DATA: c_uint = 0x0000ffff	/* Instruction Payload		*/;
// MIF Status Register.  This register reports status when the MIF is
// operating in the poll mode.  The poll status field is auto-clearing
// on read.
//
pub const MIF_STATUS_DATA: c_uint = 0xffff0000	/* Live image of XCVR reg	*/;
pub const MIF_STATUS_STAT: c_uint = 0x0000ffff	/* Which bits have changed	*/;
// MIF Mask Register.  This 16-bit register is used when in poll mode
// to say which bits of the polled register will cause an interrupt
// when changed.
//
// PCS/Serialink Registers
pub const PCS_MIICTRL: c_uint = 0x9000UL	/* PCS MII Control Register	*/;
pub const PCS_MIISTAT: c_uint = 0x9004UL	/* PCS MII Status Register	*/;
pub const PCS_MIIADV: c_uint = 0x9008UL	/* PCS MII Advertisement Reg	*/;
pub const PCS_MIILP: c_uint = 0x900CUL	/* PCS MII Link Partner Ability	*/;
pub const PCS_CFG: c_uint = 0x9010UL	/* PCS Configuration Register	*/;
pub const PCS_SMACHINE: c_uint = 0x9014UL	/* PCS State Machine Register	*/;
pub const PCS_ISTAT: c_uint = 0x9018UL	/* PCS Interrupt Status Reg	*/;
pub const PCS_DMODE: c_uint = 0x9050UL	/* Datapath Mode Register	*/;
pub const PCS_SCTRL: c_uint = 0x9054UL	/* Serialink Control Register	*/;
pub const PCS_SOS: c_uint = 0x9058UL	/* Shared Output Select Reg	*/;
pub const PCS_SSTATE: c_uint = 0x905CUL	/* Serialink State Register	*/;
// PCD MII Control Register.
pub const PCS_MIICTRL_SPD: c_uint = 0x00000040	/* Read as one, writes ignored	*/;
pub const PCS_MIICTRL_CT: c_uint = 0x00000080	/* Force COL signal active	*/;
pub const PCS_MIICTRL_DM: c_uint = 0x00000100	/* Duplex mode, forced low	*/;
pub const PCS_MIICTRL_RAN: c_uint = 0x00000200	/* Restart auto-neg, self clear	*/;
pub const PCS_MIICTRL_ISO: c_uint = 0x00000400	/* Read as zero, writes ignored	*/;
pub const PCS_MIICTRL_PD: c_uint = 0x00000800	/* Read as zero, writes ignored	*/;
pub const PCS_MIICTRL_ANE: c_uint = 0x00001000	/* Auto-neg enable		*/;
pub const PCS_MIICTRL_SS: c_uint = 0x00002000	/* Read as zero, writes ignored	*/;
pub const PCS_MIICTRL_WB: c_uint = 0x00004000	/* Wrapback, loopback at 10-bit;
// input side of Serialink
//
pub const PCS_MIICTRL_RST: c_uint = 0x00008000	/* Resets PCS, self clearing	*/;
// PCS MII Status Register.
pub const PCS_MIISTAT_EC: c_uint = 0x00000001	/* Ext Capability: Read as zero	*/;
pub const PCS_MIISTAT_JD: c_uint = 0x00000002	/* Jabber Detect: Read as zero	*/;
pub const PCS_MIISTAT_LS: c_uint = 0x00000004	/* Link Status: 1=up 0=down	*/;
pub const PCS_MIISTAT_ANA: c_uint = 0x00000008	/* Auto-neg Ability, always 1	*/;
pub const PCS_MIISTAT_RF: c_uint = 0x00000010	/* Remote Fault			*/;
pub const PCS_MIISTAT_ANC: c_uint = 0x00000020	/* Auto-neg complete		*/;
pub const PCS_MIISTAT_ES: c_uint = 0x00000100	/* Extended Status, always 1	*/;
// PCS MII Advertisement Register.
pub const PCS_MIIADV_FD: c_uint = 0x00000020	/* Advertise Full Duplex	*/;
pub const PCS_MIIADV_HD: c_uint = 0x00000040	/* Advertise Half Duplex	*/;
pub const PCS_MIIADV_SP: c_uint = 0x00000080	/* Advertise Symmetric Pause	*/;
pub const PCS_MIIADV_AP: c_uint = 0x00000100	/* Advertise Asymmetric Pause	*/;
pub const PCS_MIIADV_RF: c_uint = 0x00003000	/* Remote Fault			*/;
pub const PCS_MIIADV_ACK: c_uint = 0x00004000	/* Read-only			*/;
pub const PCS_MIIADV_NP: c_uint = 0x00008000	/* Next-page, forced low	*/;
// PCS MII Link Partner Ability Register.   This register is equivalent
// to the Link Partnet Ability Register of the standard MII register set.
// It's layout corresponds to the PCS MII Advertisement Register.
//
// PCS Configuration Register.
pub const PCS_CFG_ENABLE: c_uint = 0x00000001	/* Must be zero while changing;
// PCS MII advertisement reg.
//
pub const PCS_CFG_SDO: c_uint = 0x00000002	/* Signal detect override	*/;
pub const PCS_CFG_SDL: c_uint = 0x00000004	/* Signal detect active low	*/;
pub const PCS_CFG_JS: c_uint = 0x00000018	/* Jitter-study:;
// 0 = normal operation
// 1 = high-frequency test pattern
// 2 = low-frequency test pattern
// 3 = reserved
//
pub const PCS_CFG_TO: c_uint = 0x00000020	/* 10ms auto-neg timer override	*/;
// PCS Interrupt Status Register.  This register is self-clearing
// when read.
//
pub const PCS_ISTAT_LSC: c_uint = 0x00000004	/* Link Status Change		*/;
// Datapath Mode Register.
pub const PCS_DMODE_SM: c_uint = 0x00000001	/* 1 = use internal Serialink	*/;
pub const PCS_DMODE_ESM: c_uint = 0x00000002	/* External SERDES mode		*/;
pub const PCS_DMODE_MGM: c_uint = 0x00000004	/* MII/GMII mode		*/;
pub const PCS_DMODE_GMOE: c_uint = 0x00000008	/* GMII Output Enable		*/;
// Serialink Control Register.
//
// NOTE: When in SERDES mode, the loopback bit has inverse logic.
//
pub const PCS_SCTRL_LOOP: c_uint = 0x00000001	/* Loopback enable		*/;
pub const PCS_SCTRL_ESCD: c_uint = 0x00000002	/* Enable sync char detection	*/;
pub const PCS_SCTRL_LOCK: c_uint = 0x00000004	/* Lock to reference clock	*/;
pub const PCS_SCTRL_EMP: c_uint = 0x00000018	/* Output driver emphasis	*/;
pub const PCS_SCTRL_STEST: c_uint = 0x000001c0	/* Self test patterns		*/;
pub const PCS_SCTRL_PDWN: c_uint = 0x00000200	/* Software power-down		*/;
pub const PCS_SCTRL_RXZ: c_uint = 0x00000c00	/* PLL input to Serialink	*/;
pub const PCS_SCTRL_RXP: c_uint = 0x00003000	/* PLL input to Serialink	*/;
pub const PCS_SCTRL_TXZ: c_uint = 0x0000c000	/* PLL input to Serialink	*/;
pub const PCS_SCTRL_TXP: c_uint = 0x00030000	/* PLL input to Serialink	*/;
// Shared Output Select Register.  For test and debug, allows multiplexing
// test outputs into the PROM address pins.  Set to zero for normal
// operation.
//
pub const PCS_SOS_PADDR: c_uint = 0x00000003	/* PROM Address			*/;
// PROM Image Space
pub const PROM_START: c_uint = 0x100000UL	/* Expansion ROM run time access*/;
pub const PROM_SIZE: c_uint = 0x0fffffUL	/* Size of ROM			*/;
pub const PROM_END: c_uint = 0x200000UL	/* End of ROM			*/;
// MII definitions missing from mii.h
pub const BMCR_SPD2: c_uint = 0x0040		/* Gigabit enable? (bcm5411)	*/;
pub const LPA_PAUSE: c_uint = 0x0400;
// More PHY registers (specific to Broadcom models)
// MII BCM5201 MULTIPHY interrupt register
pub const MII_BCM5201_INTERRUPT: c_uint = 0x1A;
pub const MII_BCM5201_INTERRUPT_INTENABLE: c_uint = 0x4000;
pub const MII_BCM5201_AUXMODE2: c_uint = 0x1B;
pub const MII_BCM5201_AUXMODE2_LOWPOWER: c_uint = 0x0008;
pub const MII_BCM5201_MULTIPHY: c_uint = 0x1E;
// MII BCM5201 MULTIPHY register bits
pub const MII_BCM5201_MULTIPHY_SERIALMODE: c_uint = 0x0002;
pub const MII_BCM5201_MULTIPHY_SUPERISOLATE: c_uint = 0x0008;
// MII BCM5400 1000-BASET Control register
pub const MII_BCM5400_GB_CONTROL: c_uint = 0x09;
pub const MII_BCM5400_GB_CONTROL_FULLDUPLEXCAP: c_uint = 0x0200;
// MII BCM5400 AUXCONTROL register
pub const MII_BCM5400_AUXCONTROL: c_uint = 0x18;
pub const MII_BCM5400_AUXCONTROL_PWR10BASET: c_uint = 0x0004;
// MII BCM5400 AUXSTATUS register
pub const MII_BCM5400_AUXSTATUS: c_uint = 0x19;
pub const MII_BCM5400_AUXSTATUS_LINKMODE_MASK: c_uint = 0x0700;
pub const MII_BCM5400_AUXSTATUS_LINKMODE_SHIFT: c_int = 8;
// When it can, GEM internally caches 4 aligned TX descriptors
// at a time, so that it can use full cacheline DMA reads.
//
// Note that unlike HME, there is no ownership bit in the descriptor
// control word.  The same functionality is obtained via the TX-Kick
// and TX-Complete registers.  As a result, GEM need not write back
// updated values to the TX descriptor ring, it only performs reads.
//
// Since TX descriptors are never modified by GEM, the driver can
// use the buffer DMA address as a place to keep track of allocated
// DMA mappings for a transmitted packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gem_txd {
    pub control_word: __le64,
    pub buffer: __le64,
}

pub const TXDCTRL_BUFSZ: c_uint = 0x0000000000007fffULL	/* Buffer Size		*/;
pub const TXDCTRL_CSTART: c_uint = 0x00000000001f8000ULL	/* CSUM Start Offset	*/;
pub const TXDCTRL_COFF: c_uint = 0x000000001fe00000ULL	/* CSUM Stuff Offset	*/;
pub const TXDCTRL_CENAB: c_uint = 0x0000000020000000ULL	/* CSUM Enable		*/;
pub const TXDCTRL_EOF: c_uint = 0x0000000040000000ULL	/* End of Frame		*/;
pub const TXDCTRL_SOF: c_uint = 0x0000000080000000ULL	/* Start of Frame	*/;
pub const TXDCTRL_INTME: c_uint = 0x0000000100000000ULL	/* "Interrupt Me"	*/;
pub const TXDCTRL_NOCRC: c_uint = 0x0000000200000000ULL	/* No CRC Present	*/;
// GEM requires that RX descriptors are provided four at a time,
// aligned.  Also, the RX ring may not wrap around.  This means that
// there will be at least 4 unused descriptor entries in the middle
// of the RX ring at all times.
//
// Similar to HME, GEM assumes that it can write garbage bytes before
// the beginning of the buffer and right after the end in order to DMA
// whole cachelines.
//
// Unlike for TX, GEM does update the status word in the RX descriptors
// when packets arrive.  Therefore an ownership bit does exist in the
// RX descriptors.  It is advisory, GEM clears it but does not check
// it in any way.  So when buffers are posted to the RX ring (via the
// RX Kick register) by the driver it must make sure the buffers are
// truly ready and that the ownership bits are set properly.
//
// Even though GEM modifies the RX descriptors, it guarantees that the
// buffer DMA address field will stay the same when it performs these
// updates.  Therefore it can be used to keep track of DMA mappings
// by the host driver just as in the TX descriptor case above.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gem_rxd {
    pub status_word: __le64,
    pub buffer: __le64,
}

pub const RXDCTRL_TCPCSUM: c_uint = 0x000000000000ffffULL	/* TCP Pseudo-CSUM	*/;
pub const RXDCTRL_BUFSZ: c_uint = 0x000000007fff0000ULL	/* Buffer Size		*/;
pub const RXDCTRL_OWN: c_uint = 0x0000000080000000ULL	/* GEM owns this entry	*/;
pub const RXDCTRL_HASHVAL: c_uint = 0x0ffff00000000000ULL	/* Hash Value		*/;
pub const RXDCTRL_HPASS: c_uint = 0x1000000000000000ULL	/* Passed Hash Filter	*/;
pub const RXDCTRL_ALTMAC: c_uint = 0x2000000000000000ULL	/* Matched ALT MAC	*/;
pub const RXDCTRL_BAD: c_uint = 0x4000000000000000ULL	/* Frame has bad CRC	*/;

pub const TX_RING_SIZE: c_int = 128;
pub const RX_RING_SIZE: c_int = 128;

pub const RX_OFFSET: c_int = 2;

pub const RX_COPY_THRESHOLD: c_int = 256;

pub const INIT_BLOCK_TX_RING_SIZE: c_int = 128;

pub const INIT_BLOCK_RX_RING_SIZE: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gem_init_block {
    pub txd: [gem_txd; INIT_BLOCK_TX_RING_SIZE],
    pub rxd: [gem_rxd; INIT_BLOCK_RX_RING_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gem_phy_type {
    phy_mii_mdio0,
    phy_mii_mdio1,
    phy_serialink,
    phy_serdes,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_state {
    link_down = 0,	/* No link, will retry */
    link_aneg,	/* Autoneg in progress */
    link_force_try,	/* Try Forced link speed */
    link_force_ret,	/* Forced mode worked, retrying autoneg */
    link_force_ok,	/* Stay in forced mode */
    link_up		/* Link is up */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gem {
    pub regs: *mut void __iomem,
    pub rx_old: int rx_new,,
    pub tx_old: int tx_new,,
    pub /: *mut *mut unsigned int has_wol : 1; / chip supports wake-on-lan,
    pub /: *mut *mut unsigned int asleep_wol : 1; / was asleep with WOL enabled,
    pub cell_enabled: c_int,
    pub msg_enable: u32,
    pub status: u32,
    pub napi: napi_struct,
    pub tx_fifo_sz: c_int,
    pub rx_fifo_sz: c_int,
    pub rx_pause_off: c_int,
    pub rx_pause_on: c_int,
    pub rx_buf_sz: c_int,
    pub pause_entered: u64,
    pub pause_last_time_recvd: u16,
    pub mac_rx_cfg: u32,
    pub swrst_base: u32,
    pub want_autoneg: c_int,
    pub last_forced_speed: c_int,
    pub lstate: link_state,
    pub link_timer: timer_list,
    pub timer_ticks: c_int,
    pub wake_on_lan: c_int,
    pub reset_task: work_struct,
    pub reset_task_pending: volatile int,
    pub phy_type: gem_phy_type,
    pub phy_mii: mii_phy,
    pub mii_phy_addr: c_int,
    pub init_block: *mut gem_init_block,
    pub rx_skbs: [*mut sk_buff; RX_RING_SIZE],
    pub tx_skbs: [*mut sk_buff; TX_RING_SIZE],
    pub gblock_dvma: dma_addr_t,
    pub pdev: *mut pci_dev,
    pub dev: *mut net_device,

    pub of_node: *mut device_node,

}

