//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/dz.h
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
//
// dz.h: Serial port driver for DECstations equipped
// with the DZ chipset.
//
// Copyright (C) 1998 Olivier A. D. Lebaillif
//
// Email: olivier.lebaillif@ifrsys.com
//
// Copyright (C) 2004, 2006  Maciej W. Rozycki
//
// Definitions for the Control and Status Register.
//
pub const DZ_TRDY: c_uint = 0x8000                 /* Transmitter empty */;
pub const DZ_TIE: c_uint = 0x4000                 /* Transmitter Interrupt Enbl */;
pub const DZ_TLINE: c_uint = 0x0300                 /* Transmitter Line Number */;
pub const DZ_RDONE: c_uint = 0x0080                 /* Receiver data ready */;
pub const DZ_RIE: c_uint = 0x0040                 /* Receive Interrupt Enable */;
pub const DZ_MSE: c_uint = 0x0020                 /* Master Scan Enable */;
pub const DZ_CLR: c_uint = 0x0010                 /* Master reset */;
pub const DZ_MAINT: c_uint = 0x0008                 /* Loop Back Mode */;
//
// Definitions for the Receiver Buffer Register.
//
pub const DZ_RBUF_MASK: c_uint = 0x00FF                 /* Data Mask */;
pub const DZ_LINE_MASK: c_uint = 0x0300                 /* Line Mask */;
pub const DZ_DVAL: c_uint = 0x8000                 /* Valid Data indicator */;
pub const DZ_OERR: c_uint = 0x4000                 /* Overrun error indicator */;
pub const DZ_FERR: c_uint = 0x2000                 /* Frame error indicator */;
pub const DZ_PERR: c_uint = 0x1000                 /* Parity error indicator */;
pub const DZ_BREAK: c_uint = 0x0800                 /* BREAK event software flag */;

//
// Definitions for the Transmit Control Register.
//
pub const DZ_LINE_KEYBOARD: c_uint = 0x0001;
pub const DZ_LINE_MOUSE: c_uint = 0x0002;
pub const DZ_LINE_MODEM: c_uint = 0x0004;
pub const DZ_LINE_PRINTER: c_uint = 0x0008;
pub const DZ_MODEM_RTS: c_uint = 0x0800               /* RTS for the modem line (2) */;
pub const DZ_MODEM_DTR: c_uint = 0x0400               /* DTR for the modem line (2) */;
pub const DZ_PRINT_RTS: c_uint = 0x0200               /* RTS for the prntr line (3) */;
pub const DZ_PRINT_DTR: c_uint = 0x0100               /* DTR for the prntr line (3) */;
pub const DZ_LNENB: c_uint = 0x000f               /* Transmitter Line Enable */;
//
// Definitions for the Modem Status Register.
//
pub const DZ_MODEM_RI: c_uint = 0x0800               /* RI for the modem line (2) */;
pub const DZ_MODEM_CD: c_uint = 0x0400               /* CD for the modem line (2) */;
pub const DZ_MODEM_DSR: c_uint = 0x0200               /* DSR for the modem line (2) */;
pub const DZ_MODEM_CTS: c_uint = 0x0100               /* CTS for the modem line (2) */;
pub const DZ_PRINT_RI: c_uint = 0x0008               /* RI for the printer line (3) */;
pub const DZ_PRINT_CD: c_uint = 0x0004               /* CD for the printer line (3) */;
pub const DZ_PRINT_DSR: c_uint = 0x0002               /* DSR for the prntr line (3) */;
pub const DZ_PRINT_CTS: c_uint = 0x0001               /* CTS for the prntr line (3) */;
//
// Definitions for the Transmit Data Register.
//
pub const DZ_BRK0: c_uint = 0x0100               /* Break assertion for line 0 */;
pub const DZ_BRK1: c_uint = 0x0200               /* Break assertion for line 1 */;
pub const DZ_BRK2: c_uint = 0x0400               /* Break assertion for line 2 */;
pub const DZ_BRK3: c_uint = 0x0800               /* Break assertion for line 3 */;
//
// Definitions for the Line Parameter Register.
//
pub const DZ_KEYBOARD: c_uint = 0x0000               /* line 0 = keyboard */;
pub const DZ_MOUSE: c_uint = 0x0001               /* line 1 = mouse */;
pub const DZ_MODEM: c_uint = 0x0002               /* line 2 = modem */;
pub const DZ_PRINTER: c_uint = 0x0003               /* line 3 = printer */;
pub const DZ_CSIZE: c_uint = 0x0018               /* Number of bits per byte (mask) */;
pub const DZ_CS5: c_uint = 0x0000               /* 5 bits per byte */;
pub const DZ_CS6: c_uint = 0x0008               /* 6 bits per byte */;
pub const DZ_CS7: c_uint = 0x0010               /* 7 bits per byte */;
pub const DZ_CS8: c_uint = 0x0018               /* 8 bits per byte */;
pub const DZ_CSTOPB: c_uint = 0x0020               /* 2 stop bits instead of one */;
pub const DZ_PARENB: c_uint = 0x0040               /* Parity enable */;
pub const DZ_PARODD: c_uint = 0x0080               /* Odd parity instead of even */;
pub const DZ_CBAUD: c_uint = 0x0E00               /* Baud Rate (mask) */;
pub const DZ_B50: c_uint = 0x0000;
pub const DZ_B75: c_uint = 0x0100;
pub const DZ_B110: c_uint = 0x0200;
pub const DZ_B134: c_uint = 0x0300;
pub const DZ_B150: c_uint = 0x0400;
pub const DZ_B300: c_uint = 0x0500;
pub const DZ_B600: c_uint = 0x0600;
pub const DZ_B1200: c_uint = 0x0700;
pub const DZ_B1800: c_uint = 0x0800;
pub const DZ_B2000: c_uint = 0x0900;
pub const DZ_B2400: c_uint = 0x0A00;
pub const DZ_B3600: c_uint = 0x0B00;
pub const DZ_B4800: c_uint = 0x0C00;
pub const DZ_B7200: c_uint = 0x0D00;
pub const DZ_B9600: c_uint = 0x0E00;
pub const DZ_RXENAB: c_uint = 0x1000               /* Receiver Enable */;
//
// Addresses for the DZ registers
//
pub const DZ_CSR: c_uint = 0x00            /* Control and Status Register */;
pub const DZ_RBUF: c_uint = 0x08            /* Receive Buffer */;
pub const DZ_LPR: c_uint = 0x08            /* Line Parameters Register */;
pub const DZ_TCR: c_uint = 0x10            /* Transmitter Control Register */;
pub const DZ_MSR: c_uint = 0x18            /* Modem Status Register */;
pub const DZ_TDR: c_uint = 0x18            /* Transmit Data Register */;
pub const DZ_NB_PORT: c_int = 4;

