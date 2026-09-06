//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/i825xx/sun3_82586.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Intel i82586 Ethernet definitions
//
// This is an extension to the Linux operating system, and is covered by the
// same Gnu Public License that covers that work.
//
// copyrights (c) 1994 by Michael Hipp (hippm@informatik.uni-tuebingen.de)
//
// I have done a look in the following sources:
// crynwr-packet-driver by Russ Nelson
// Garret A. Wollman's i82586-driver for BSD
//
// Cloned from ni52.h, copyright as above.
//
// Modified for Sun3 OBIO i82586 by Sam Creasey (sammy@sammy.net)
//
// defines for the obio chip (not vme)
pub const IEOB_NORSET: c_uint = 0x80        /* don't reset the board */;
pub const IEOB_ONAIR: c_uint = 0x40        /* put us on the air */;
pub const IEOB_ATTEN: c_uint = 0x20        /* attention! */;
pub const IEOB_IENAB: c_uint = 0x10        /* interrupt enable */;
pub const IEOB_XXXXX: c_uint = 0x08        /* free bit */;
pub const IEOB_XCVRL2: c_uint = 0x04        /* level 2 transceiver? */;
pub const IEOB_BUSERR: c_uint = 0x02        /* bus error */;
pub const IEOB_INT: c_uint = 0x01        /* interrupt */;
// where the obio one lives
pub const IE_OBIO: c_uint = 0xc0000;
pub const IE_IRQ: c_int = 3;
//
// where to find the System Configuration Pointer (SCP)
//
pub const SCP_DEFAULT_ADDRESS: c_uint = 0xfffff4;
//
// System Configuration Pointer Struct
//
// Intermediate System Configuration Pointer (ISCP)
//
// System Control Block (SCB)
//
// possible command values for the command word
//
pub const RUC_MASK: c_uint = 0x0070	/* mask for RU commands */;
pub const RUC_NOP: c_uint = 0x0000	/* NOP-command */;
pub const RUC_START: c_uint = 0x0010	/* start RU */;
pub const RUC_RESUME: c_uint = 0x0020	/* resume RU after suspend */;
pub const RUC_SUSPEND: c_uint = 0x0030	/* suspend RU */;
pub const RUC_ABORT: c_uint = 0x0040	/* abort receiver operation immediately */;
pub const CUC_MASK: c_uint = 0x07  /* mask for CU command */;
pub const CUC_NOP: c_uint = 0x00  /* NOP-command */;
pub const CUC_START: c_uint = 0x01  /* start execution of 1. cmd on the CBL */;
pub const CUC_RESUME: c_uint = 0x02  /* resume after suspend */;
pub const CUC_SUSPEND: c_uint = 0x03  /* Suspend CU */;
pub const CUC_ABORT: c_uint = 0x04  /* abort command operation immediately */;
pub const ACK_MASK: c_uint = 0xf0  /* mask for ACK command */;
pub const ACK_CX: c_uint = 0x80  /* acknowledges STAT_CX */;
pub const ACK_FR: c_uint = 0x40  /* ack. STAT_FR */;
pub const ACK_CNA: c_uint = 0x20  /* ack. STAT_CNA */;
pub const ACK_RNR: c_uint = 0x10  /* ack. STAT_RNR */;
//
// possible status values for the status word
//
pub const STAT_MASK: c_uint = 0xf0  /* mask for cause of interrupt */;
pub const STAT_CX: c_uint = 0x80  /* CU finished cmd with its I bit set */;
pub const STAT_FR: c_uint = 0x40  /* RU finished receiving a frame */;
pub const STAT_CNA: c_uint = 0x20  /* CU left active state */;
pub const STAT_RNR: c_uint = 0x10  /* RU left ready state */;
pub const CU_STATUS: c_uint = 0x7   /* CU status, 0=idle */;
pub const CU_SUSPEND: c_uint = 0x1   /* CU is suspended */;
pub const CU_ACTIVE: c_uint = 0x2   /* CU is active */;
pub const RU_STATUS: c_uint = 0x70	/* RU status, 0=idle */;
pub const RU_SUSPEND: c_uint = 0x10	/* RU suspended */;
pub const RU_NOSPACE: c_uint = 0x20	/* RU no resources */;
pub const RU_READY: c_uint = 0x40	/* RU is ready */;
//
// Receive Frame Descriptor (RFD)
//
pub const RFD_LAST: c_uint = 0x80	/* last: last rfd in the list */;
pub const RFD_SUSP: c_uint = 0x40	/* last: suspend RU after  */;
pub const RFD_COMPL: c_uint = 0x80;
pub const RFD_OK: c_uint = 0x20;
pub const RFD_BUSY: c_uint = 0x40;
pub const RFD_ERR_LEN: c_uint = 0x10     /* Length error (if enabled length-checking */;
pub const RFD_ERR_CRC: c_uint = 0x08     /* CRC error */;
pub const RFD_ERR_ALGN: c_uint = 0x04     /* Alignment error */;
pub const RFD_ERR_RNR: c_uint = 0x02     /* status: receiver out of resources */;
pub const RFD_ERR_OVR: c_uint = 0x01     /* DMA Overrun! */;
pub const RFD_ERR_FTS: c_uint = 0x0080	/* Frame too short */;
pub const RFD_ERR_NEOP: c_uint = 0x0040	/* No EOP flag (for bitstuffing only) */;
pub const RFD_ERR_TRUN: c_uint = 0x0020	/* (82596 only/SF mode) indicates truncated frame */;
pub const RFD_MATCHADD: c_uint = 0x0002     /* status: Destinationaddress !matches IA (only 82596) */;
pub const RFD_COLLDET: c_uint = 0x0001	/* Detected collision during reception */;
//
// Receive Buffer Descriptor (RBD)
//
pub const RBD_LAST: c_uint = 0x8000	/* last buffer */;
pub const RBD_USED: c_uint = 0x4000	/* this buffer has data */;
pub const RBD_MASK: c_uint = 0x3fff	/* size-mask for length */;
//
// Statusvalues for Commands/RFD
//
pub const STAT_COMPL: c_uint = 0x8000	/* status: frame/command is complete */;
pub const STAT_BUSY: c_uint = 0x4000	/* status: frame/command is busy */;
pub const STAT_OK: c_uint = 0x2000	/* status: frame/command is ok */;
//
// Action-Commands
//
pub const CMD_NOP: c_uint = 0x0000	/* NOP */;
pub const CMD_IASETUP: c_uint = 0x0001	/* initial address setup command */;
pub const CMD_CONFIGURE: c_uint = 0x0002	/* configure command */;
pub const CMD_MCSETUP: c_uint = 0x0003	/* MC setup command */;
pub const CMD_XMIT: c_uint = 0x0004	/* transmit command */;
pub const CMD_TDR: c_uint = 0x0005	/* time domain reflectometer (TDR) command */;
pub const CMD_DUMP: c_uint = 0x0006	/* dump command */;
pub const CMD_DIAGNOSE: c_uint = 0x0007	/* diagnose command */;
//
// Action command bits
//
pub const CMD_LAST: c_uint = 0x8000	/* indicates last command in the CBL */;
pub const CMD_SUSPEND: c_uint = 0x4000	/* suspend CU after this CB */;
pub const CMD_INT: c_uint = 0x2000	/* generate interrupt after execution */;
//
// NOP - command
//
// IA Setup command
//
// Configure command
//
// Multicast Setup command
//
// DUMP command
//
// transmit command
//
pub const TCMD_ERRMASK: c_uint = 0x0fa0;
pub const TCMD_MAXCOLLMASK: c_uint = 0x000f;
pub const TCMD_MAXCOLL: c_uint = 0x0020;
pub const TCMD_HEARTBEAT: c_uint = 0x0040;
pub const TCMD_DEFERRED: c_uint = 0x0080;
pub const TCMD_UNDERRUN: c_uint = 0x0100;
pub const TCMD_LOSTCTS: c_uint = 0x0200;
pub const TCMD_NOCARRIER: c_uint = 0x0400;
pub const TCMD_LATECOLL: c_uint = 0x0800;
pub const TDR_LNK_OK: c_uint = 0x8000	/* No link problem identified */;
pub const TDR_XCVR_PRB: c_uint = 0x4000	/* indicates a transceiver problem */;
pub const TDR_ET_OPN: c_uint = 0x2000	/* open, no correct termination */;
pub const TDR_ET_SRT: c_uint = 0x1000	/* TDR detected a short circuit */;
pub const TDR_TIMEMASK: c_uint = 0x07ff	/* mask for the time field */;
//
// Transmit Buffer Descriptor (TBD)
//
pub const TBD_LAST: c_uint = 0x8000         /* EOF-Flag, indicates last buffer in list */;
