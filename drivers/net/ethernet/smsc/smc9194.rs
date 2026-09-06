//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/smsc/smc9194.h
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


// ------------------------------------------------------------------------
// I want some simple types
pub type byte = c_uchar;
pub type word = c_ushort;
pub type dword = unsigned long int;
// Because of bank switching, the SMC91xxx uses only 16 I/O ports
pub const SMC_IO_EXTENT: c_int = 16;
// ---------------------------------------------------------------
//
pub const BANK_SELECT: c_int = 14;
// BANK 0

pub const TCR_ENABLE: c_uint = 0x0001	/* if this is 1, we can transmit */;
pub const TCR_FDUPLX: c_uint = 0x0800  /* receive packets sent out */;
pub const TCR_STP_SQET: c_uint = 0x1000	/* stop transmitting if Signal quality error */;
pub const TCR_MON_CNS: c_uint = 0x0400	/* monitors the carrier status */;
pub const TCR_PAD_ENABLE: c_uint = 0x0080	/* pads short packets to 64 bytes */;

// the normal settings for the TCR register :
// QUESTION: do I want to enable padding of short packets ?

pub const EPH_STATUS: c_int = 2;
pub const ES_LINK_OK: c_uint = 0x4000	/* is the link integrity ok ? */;
pub const RCR: c_int = 4;
pub const RCR_SOFTRESET: c_uint = 0x8000 	/* resets the chip */;
pub const RCR_STRIP_CRC: c_uint = 0x200	/* strips CRC */;
pub const RCR_ENABLE: c_uint = 0x100	/* IFF this is set, we can receive packets */;
pub const RCR_ALMUL: c_uint = 0x4 	/* receive all multicast packets */;
pub const RCR_PROMISC: c_uint = 0x2	/* enable promiscuous mode */;
// the normal settings for the RCR register :

pub const RCR_CLEAR: c_uint = 0x0		/* set it to a base state */;
pub const COUNTER: c_int = 6;
pub const MIR: c_int = 8;
pub const MCR: c_int = 10;
// 12 is reserved
// BANK 1
pub const CONFIG: c_int = 0;
pub const CFG_AUI_SELECT: c_uint = 0x100;
pub const BASE: c_int = 2;
pub const ADDR0: c_int = 4;
pub const ADDR1: c_int = 6;
pub const ADDR2: c_int = 8;
pub const GENERAL: c_int = 10;
pub const CONTROL: c_int = 12;
pub const CTL_POWERDOWN: c_uint = 0x2000;
pub const CTL_LE_ENABLE: c_uint = 0x80;
pub const CTL_CR_ENABLE: c_uint = 0x40;
pub const CTL_TE_ENABLE: c_uint = 0x0020;
pub const CTL_AUTO_RELEASE: c_uint = 0x0800;
pub const CTL_EPROM_ACCESS: c_uint = 0x0003 /* high if Eprom is being read */;
// BANK 2
pub const MMU_CMD: c_int = 0;

pub const MC_NOP: c_int = 0;
pub const MC_ALLOC: c_uint = 0x20  	/* or with number of 256 byte packets */;
pub const MC_RESET: c_uint = 0x40;
pub const MC_REMOVE: c_uint = 0x60  	/* remove the current rx packet */;
pub const MC_RELEASE: c_uint = 0x80  	/* remove and release the current rx packet */;
pub const MC_FREEPKT: c_uint = 0xA0  	/* Release packet in PNR register */;
pub const MC_ENQUEUE: c_uint = 0xC0 	/* Enqueue the packet for transmit */;
pub const PNR_ARR: c_int = 2;
pub const FIFO_PORTS: c_int = 4;
pub const FP_RXEMPTY: c_uint = 0x8000;
pub const FP_TXEMPTY: c_uint = 0x80;
pub const POINTER: c_int = 6;
pub const PTR_READ: c_uint = 0x2000;
pub const PTR_RCV: c_uint = 0x8000;
pub const PTR_AUTOINC: c_uint = 0x4000;
pub const PTR_AUTO_INC: c_uint = 0x0040;
pub const DATA_1: c_int = 8;
pub const DATA_2: c_int = 10;
pub const INTERRUPT: c_int = 12;
pub const INT_MASK: c_int = 13;
pub const IM_RCV_INT: c_uint = 0x1;
pub const IM_TX_INT: c_uint = 0x2;
pub const IM_TX_EMPTY_INT: c_uint = 0x4;
pub const IM_ALLOC_INT: c_uint = 0x8;
pub const IM_RX_OVRN_INT: c_uint = 0x10;
pub const IM_EPH_INT: c_uint = 0x20;
pub const IM_ERCV_INT: c_uint = 0x40 /* not on SMC9192 */;
// BANK 3
pub const MULTICAST1: c_int = 0;
pub const MULTICAST2: c_int = 2;
pub const MULTICAST3: c_int = 4;
pub const MULTICAST4: c_int = 6;
pub const MGMT: c_int = 8;

// this is NOT on SMC9192
pub const ERCV: c_int = 12;
pub const CHIP_9190: c_int = 3;
pub const CHIP_9194: c_int = 4;
pub const CHIP_9195: c_int = 5;
pub const CHIP_91100: c_int = 7;
// 3 */ "SMC91C90/91C92",
// 4 */ "SMC91C94",
// 5 */ "SMC91C95",
// 7 */ "SMC91C100",
// 8 */ "SMC91C100FD",
//
pub const TS_SUCCESS: c_uint = 0x0001;
pub const TS_LOSTCAR: c_uint = 0x0400;
pub const TS_LATCOL: c_uint = 0x0200;
pub const TS_16COL: c_uint = 0x0010;
//
pub const RS_ALGNERR: c_uint = 0x8000;
pub const RS_BADCRC: c_uint = 0x2000;
pub const RS_ODDFRAME: c_uint = 0x1000;
pub const RS_TOOLONG: c_uint = 0x0800;
pub const RS_TOOSHORT: c_uint = 0x0400;
pub const RS_MULTICAST: c_uint = 0x0001;

// -------------------------------------------------------------------------
// select a register bank, 0 to 3

// define a small delay for the reset

// this enables an interrupt in the interrupt mask register

// this disables an interrupt from the interrupt mask register

// ----------------------------------------------------------------------

