//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/msdos_partition.h
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
pub const MSDOS_LABEL_MAGIC: c_uint = 0xAA55;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msdos_partition {
    pub /: *mut *mut u8 boot_ind; / 0x80 - active,
    pub /: *mut *mut u8 head; / starting head,
    pub /: *mut *mut u8 sector; / starting sector,
    pub /: *mut *mut u8 cyl; / starting cylinder,
    pub /: *mut *mut u8 sys_ind; / What partition type,
    pub /: *mut *mut u8 end_head; / end head,
    pub /: *mut *mut u8 end_sector; / end sector,
    pub /: *mut *mut u8 end_cyl; / end cylinder,
    pub /: *mut *mut __le32 start_sect; / starting sector counting from 0,
    pub /: *mut *mut __le32 nr_sects; / nr of sectors in partition,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msdos_sys_ind {
//
// These three have identical behaviour; use the second one if DOS FDISK
// gets confused about extended/logical partitions starting past
// cylinder 1023.
//
    DOS_EXTENDED_PARTITION = 5,
    LINUX_EXTENDED_PARTITION = 0x85,
    WIN98_EXTENDED_PARTITION = 0x0f,

    LINUX_DATA_PARTITION = 0x83,
    LINUX_LVM_PARTITION = 0x8e,
    LINUX_RAID_PARTITION = 0xfd,	/* autodetect RAID partition */

    SOLARIS_X86_PARTITION =	0x82,	/* also Linux swap partitions */
    NEW_SOLARIS_X86_PARTITION = 0xbf,

    DM6_AUX1PARTITION = 0x51,	/* no DDO:  use xlated geom */
    DM6_AUX3PARTITION = 0x53,	/* no DDO:  use xlated geom */
    DM6_PARTITION =	0x54,		/* has DDO: use xlated geom & offset */
    EZD_PARTITION =	0x55,		/* EZ-DRIVE */

    FREEBSD_PARTITION = 0xa5,	/* FreeBSD Partition ID */
    OPENBSD_PARTITION = 0xa6,	/* OpenBSD Partition ID */
    NETBSD_PARTITION = 0xa9,	/* NetBSD Partition ID */
    BSDI_PARTITION = 0xb7,		/* BSDI Partition ID */
    MINIX_PARTITION = 0x81,		/* Minix Partition ID */
    UNIXWARE_PARTITION = 0x63,	/* Same as GNU_HURD and SCO Unix */
}
