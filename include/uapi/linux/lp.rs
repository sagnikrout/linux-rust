//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/lp.h
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
// usr/include/linux/lp.h c.1991-1992 James Wiegand
// many modifications copyright (C) 1992 Michael K. Johnson
// Interrupt support added 1993 Nigel Gamble
// Removed 8255 status defines from inside __KERNEL__ Marcelo Tosatti
//

//
// Per POSIX guidelines, this module reserves the LP and lp prefixes
// These are the lp_table[minor].flags flags...
//
pub const LP_EXIST: c_uint = 0x0001;
pub const LP_SELEC: c_uint = 0x0002;
pub const LP_BUSY: c_uint = 0x0004;
pub const LP_BUSY_BIT_POS: c_int = 2;
pub const LP_OFFL: c_uint = 0x0008;
pub const LP_NOPA: c_uint = 0x0010;
pub const LP_ERR: c_uint = 0x0020;
pub const LP_ABORT: c_uint = 0x0040;
pub const LP_CAREFUL: c_uint = 0x0080 /* obsoleted -arca */;
pub const LP_ABORTOPEN: c_uint = 0x0100;
pub const LP_TRUST_IRQ_: c_uint = 0x0200 /* obsolete */;
pub const LP_NO_REVERSE: c_uint = 0x0400 /* No reverse mode available. */;
pub const LP_DATA_AVAIL: c_uint = 0x0800 /* Data is available. */;
//
// bit defines for 8255 status port
// base + 1
// accessed with LP_S(minor), which gets the byte...
//
pub const LP_PBUSY: c_uint = 0x80  /* inverted input, active high */;
pub const LP_PACK: c_uint = 0x40  /* unchanged input, active low */;
pub const LP_POUTPA: c_uint = 0x20  /* unchanged input, active high */;
pub const LP_PSELECD: c_uint = 0x10  /* unchanged input, active high */;
pub const LP_PERRORP: c_uint = 0x08  /* unchanged input, active low */;
// timeout for each character.  This is relative to bus cycles -- it
// is the count in a busy loop.  THIS IS THE VALUE TO CHANGE if you
// have extremely slow printing, or if the machine seems to slow down
// a lot when you print.  If you have slow printing, increase this
// number and recompile, and if your system gets bogged down, decrease
// this number.  This can be changed with the tunelp(8) command as well.
//
pub const LP_INIT_CHAR: c_int = 1000;
// The parallel port specs apparently say that there needs to be
// a .5usec wait before and after the strobe.
//
pub const LP_INIT_WAIT: c_int = 1;
// This is the amount of time that the driver waits for the printer to
// catch up when the printer's buffer appears to be filled.  If you
// want to tune this and have a fast printer (i.e. HPIIIP), decrease
// this number, and if you have a slow printer, increase this number.
// This is in hundredths of a second, the default 2 being .05 second.
// Or use the tunelp(8) command, which is especially nice if you want
// change back and forth between character and graphics printing, which
// are wildly different...
//
pub const LP_INIT_TIME: c_int = 2;
// IOCTL numbers
pub const LPCHAR: c_uint = 0x0601  /* corresponds to LP_INIT_CHAR */;
pub const LPTIME: c_uint = 0x0602  /* corresponds to LP_INIT_TIME */;
pub const LPABORT: c_uint = 0x0604  /* call with TRUE arg to abort on error,;
pub const LPSETIRQ: c_uint = 0x0605  /* call with new IRQ number,;
pub const LPGETIRQ: c_uint = 0x0606  /* get the current IRQ number */;
pub const LPWAIT: c_uint = 0x0608  /* corresponds to LP_INIT_WAIT */;
// NOTE: LPCAREFUL is obsoleted and it' s always the default right now -arca
pub const LPCAREFUL: c_uint = 0x0609  /* call with TRUE arg to require out-of-paper, off-;
pub const LPABORTOPEN: c_uint = 0x060a  /* call with TRUE arg to abort open() on error,;
pub const LPGETSTATUS: c_uint = 0x060b  /* return LP_S(minor) */;
pub const LPRESET: c_uint = 0x060c  /* reset printer */;

pub const LPGETSTATS: c_uint = 0x060d  /* get statistics (struct lp_stats) */;

pub const LPGETFLAGS: c_uint = 0x060e  /* get status flags */;
pub const LPSETTIMEOUT_OLD: c_uint = 0x060f /* set parport timeout */;

// timeout for printk'ing a timeout, in jiffies (100ths of a second).

