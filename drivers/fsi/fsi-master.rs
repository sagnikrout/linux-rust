//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fsi/fsi-master.h
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
// FSI master definitions. These comprise the core <--> master interface,
// to allow the core to interact with the (hardware-specific) masters.
//
// Copyright (C) IBM Corporation 2016
//

//
// Master registers
//
// These are used by hardware masters, such as the one in the FSP2, AST2600 and
// the hub master in POWER processors.
//
// Control Registers
pub const FSI_MMODE: c_uint = 0x0		/* R/W: mode */;
pub const FSI_MDLYR: c_uint = 0x4		/* R/W: delay */;
pub const FSI_MCRSP: c_uint = 0x8		/* R/W: clock rate */;
pub const FSI_MENP0: c_uint = 0x10		/* R/W: enable */;
pub const FSI_MLEVP0: c_uint = 0x18		/* R: plug detect */;
pub const FSI_MSENP0: c_uint = 0x18		/* S: Set enable */;
pub const FSI_MCENP0: c_uint = 0x20		/* C: Clear enable */;
pub const FSI_MAEB: c_uint = 0x70		/* R: Error address */;
pub const FSI_MVER: c_uint = 0x74		/* R: master version/type */;
pub const FSI_MSTAP0: c_uint = 0xd0		/* R: Port status */;
pub const FSI_MRESP0: c_uint = 0xd0		/* W: Port reset */;
pub const FSI_MESRB0: c_uint = 0x1d0		/* R: Master error status */;
pub const FSI_MRESB0: c_uint = 0x1d0		/* W: Reset bridge */;
pub const FSI_MSCSB0: c_uint = 0x1d4		/* R: Master sub command stack */;
pub const FSI_MATRB0: c_uint = 0x1d8		/* R: Master address trace */;
pub const FSI_MDTRB0: c_uint = 0x1dc		/* R: Master data trace */;
pub const FSI_MECTRL: c_uint = 0x2e0		/* W: Error control */;
// MMODE: Mode control
pub const FSI_MMODE_EIP: c_uint = 0x80000000	/* Enable interrupt polling */;
pub const FSI_MMODE_ECRC: c_uint = 0x40000000	/* Enable error recovery */;
pub const FSI_MMODE_RELA: c_uint = 0x20000000	/* Enable relative address commands */;
pub const FSI_MMODE_EPC: c_uint = 0x10000000	/* Enable parity checking */;
pub const FSI_MMODE_P8_TO_LSB: c_uint = 0x00000010	/* Timeout value LSB */;
// MSB=1, LSB=0 is 0.8 ms
// MSB=0, LSB=1 is 0.9 ms

pub const FSI_MMODE_CRS0MASK: c_uint = 0x3ff		/* Clk rate selection 0 mask */;

pub const FSI_MMODE_CRS1MASK: c_uint = 0x3ff		/* Clk rate selection 1 mask */;
// MRESB: Reset bridge
pub const FSI_MRESB_RST_GEN: c_uint = 0x80000000	/* General reset */;
pub const FSI_MRESB_RST_ERR: c_uint = 0x40000000	/* Error Reset */;
// MRESP: Reset port
pub const FSI_MRESP_RST_ALL_MASTER: c_uint = 0x20000000	/* Reset all FSI masters */;
pub const FSI_MRESP_RST_ALL_LINK: c_uint = 0x10000000	/* Reset all FSI port contr. */;
pub const FSI_MRESP_RST_MCR: c_uint = 0x08000000	/* Reset FSI master reg. */;
pub const FSI_MRESP_RST_PYE: c_uint = 0x04000000	/* Reset FSI parity error */;
pub const FSI_MRESP_RST_ALL: c_uint = 0xfc000000	/* Reset any error */;
// MECTRL: Error control
pub const FSI_MECTRL_EOAE: c_uint = 0x8000		/* Enable machine check when */;
// master 0 in error
pub const FSI_MECTRL_P8_AUTO_TERM: c_uint = 0x4000		/* Auto terminate */;
pub const FSI_HUB_LINK_OFFSET: c_uint = 0x80000;
pub const FSI_HUB_LINK_SIZE: c_uint = 0x80000;
pub const FSI_HUB_MASTER_MAX_LINKS: c_int = 8;
//
// Protocol definitions
//
// These are used by low level masters that bit-bang out the protocol
//
// Various protocol delays

// Various retry maximums
pub const FSI_CRC_ERR_RETRIES: c_int = 10;
pub const FSI_MASTER_MAX_BUSY: c_int = 200;
pub const FSI_MASTER_MTOE_COUNT: c_int = 1000;
// Command encodings
pub const FSI_CMD_DPOLL: c_uint = 0x2;
pub const FSI_CMD_EPOLL: c_uint = 0x3;
pub const FSI_CMD_TERM: c_uint = 0x3f;
pub const FSI_CMD_ABS_AR: c_uint = 0x4;
pub const FSI_CMD_REL_AR: c_uint = 0x5;
pub const FSI_CMD_SAME_AR: c_uint = 0x3	/* but only a 2-bit opcode... */;
// Slave responses

// Misc
pub const FSI_CRC_SIZE: c_int = 4;
// fsi-master definition and flags
pub const FSI_MASTER_FLAG_SWCLOCK: c_uint = 0x1;
//
// Structures and function prototypes
//
// These are common to all masters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsi_master {
    pub dev: device,
    pub idx: c_int,
    pub n_links: c_int,
    pub flags: c_int,
    pub scan_lock: mutex,
    pub size): *mut *mut uint32_t addr, void val, size_t,
    pub size): *const *const uint32_t addr, void val, size_t,
    pub id): *mut *mut *mut int (term)(struct fsi_master , int link, uint8_t,
    pub link): *mut *mut *mut int (send_break)(struct fsi_master , int,
    pub enable): bool,
    pub t_echo_delay): u8 t_send_delay, u8,
}

//
// fsi_master registration & lifetime: the fsi_master_register() and
// fsi_master_unregister() functions will take ownership of the master, and
// ->dev in particular. The registration path performs a get_device(), which
// takes the first reference on the device. Similarly, the unregistration path
// performs a put_device(), which may well drop the last reference.
//
// This means that master implementations *may* need to hold their own
// reference (via get_device()) on master->dev. In particular, if the device's
// ->release callback frees the fsi_master, then fsi_master_unregister will
// invoke this free if no other reference is held.
//
// The same applies for the error path of fsi_master_register; if the call
// fails, dev->release will have been invoked.
//
extern "C" {
    pub fn fsi_master_register(master: *mut fsi_master) -> c_int;
}
extern "C" {
    pub fn fsi_master_unregister(master: *mut fsi_master);
}
extern "C" {
    pub fn fsi_master_rescan(master: *mut fsi_master) -> c_int;
}
