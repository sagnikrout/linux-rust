//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/imm.h
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
// Driver for the Iomega MatchMaker parallel port SCSI HBA embedded in
// the Iomega ZIP Plus drive
//
// (c) 1998     David Campbell
//
// Please note that I live in Perth, Western Australia. GMT+0800
//

//
// 10 Apr 1998 (Good Friday) - Received EN144302 by email from Iomega.
// Scarry thing is the level of support from one of their managers.
// The onus is now on us (the developers) to shut up and start coding.
// 11Apr98 [ 0.10 ]
//
// --- SNIP ---
//
// It manages to find the drive which is a good start. Writing data during
// data phase is known to be broken (due to requirements of two byte writes).
// Removing "Phase" debug messages.
//
// PS: Took four hours of coding after I bought a drive.
// ANZAC Day (Aus "War Veterans Holiday")  25Apr98 [ 0.14 ]
//
// Ten minutes later after a few fixes.... (LITERALLY!!!)
// Have mounted disk, copied file, dismounted disk, remount disk, diff file
// -----  It actually works!!! -----
// 25Apr98 [ 0.15 ]
//
// Twenty minutes of mucking around, rearanged the IEEE negotiate mechanism.
// Now have byte mode working (only EPP and ECP to go now... :=)
// 26Apr98 [ 0.16 ]
//
// Thirty minutes of further coding results in EPP working on my machine.
// 27Apr98 [ 0.17 ]
//
// Due to work commitments and inability to get a "true" ECP mode functioning
// I have decided to code the parport support into imm.
// 09Jun98 [ 0.18 ]
//
// Driver is now out of beta testing.
// Support for parport has been added.
// Now distributed with the ppa driver.
// 12Jun98 [ 2.00 ]
//
// Err.. It appears that imm-2.00 was broken....
// 18Jun98 [ 2.01 ]
//
// Patch applied to sync this against the Linux 2.1.x kernel code
// Included qboot_zip.sh
// 21Jun98 [ 2.02 ]
//
// Other clean ups include the follow changes:
// CONFIG_SCSI_PPA_HAVE_PEDANTIC => CONFIG_SCSI_IZIP_EPP16
// added CONFIG_SCSI_IZIP_SLOW_CTR option
// [2.03]
// Fix kernel panic on scsi timeout.		20Aug00 [2.04]
//
// Avoid io_request_lock problems.
// John Cavan <johncavan@home.com>		16Nov00 [2.05]
//
// ------ END OF USER CONFIGURABLE PARAMETERS -----

// batteries not included :-)
//
// modes in which the driver can operate
//

// other options

// args to imm_connect
pub const CONNECT_EPP_MAYBE: c_int = 1;
pub const CONNECT_NORMAL: c_int = 0;

// On PCI is: base+0x400 != base_hi

extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}
extern "C" {
    pub fn imm_engine(: *mut imm_struct, : *mut scsi_cmnd) -> static int;
}
