//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_bonding.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
//
// Bond several ethernet interfaces into a Cisco, running 'Etherchannel'.
//
// Portions are (c) Copyright 1995 Simon "Guru Aleph-Null" Janes
// NCM: Network and Communications Management, Inc.
//
// BUT, I'm the one who modified it for ethernet, so:
// (c) Copyright 1999, Thomas Davis, tadavis@lbl.gov
//
// This software may be used and distributed according to the terms
// of the GNU Public License, incorporated herein by reference.
//
// 2003/03/18 - Amir Noam <amir.noam at intel dot com>
// - Added support for getting slave's speed and duplex via ethtool.
// Needed for 802.3ad and other future modes.
//
// 2003/03/18 - Tsippy Mendelson <tsippy.mendelson at intel dot com> and
// Shmulik Hen <shmulik.hen at intel dot com>
// - Enable support of modes that need to use the unique mac address of
// each slave.
//
// 2003/03/18 - Tsippy Mendelson <tsippy.mendelson at intel dot com> and
// Amir Noam <amir.noam at intel dot com>
// - Moved driver's private data types to bonding.h
//
// 2003/03/18 - Amir Noam <amir.noam at intel dot com>,
// Tsippy Mendelson <tsippy.mendelson at intel dot com> and
// Shmulik Hen <shmulik.hen at intel dot com>
// - Added support for IEEE 802.3ad Dynamic link aggregation mode.
//
// 2003/05/01 - Amir Noam <amir.noam at intel dot com>
// - Added ABI version control to restore compatibility between
// new/old ifenslave and new/old bonding.
//
// 2003/12/01 - Shmulik Hen <shmulik.hen at intel dot com>
// - Code cleanup and style changes
//
// 2005/05/05 - Jason Gabler <jygabler at lbl dot gov>
// - added definitions for various XOR hashing policies
//

// userland - kernel ABI version (2003/05/08)
pub const BOND_ABI_VERSION: c_int = 2;
//
// We can remove these ioctl definitions in 2.5.  People should use the
// SIOC*** versions of them instead
//

pub const BOND_MODE_ROUNDROBIN: c_int = 0;
pub const BOND_MODE_ACTIVEBACKUP: c_int = 1;
pub const BOND_MODE_XOR: c_int = 2;
pub const BOND_MODE_BROADCAST: c_int = 3;
pub const BOND_MODE_8023AD: c_int = 4;
pub const BOND_MODE_TLB: c_int = 5;

// each slave's link has 4 states

// each slave has several states

// hashing types

// 802.3ad port state definitions (43.4.2.2 in the 802.3ad standard)
pub const LACP_STATE_LACP_ACTIVITY: c_uint = 0x1;
pub const LACP_STATE_LACP_TIMEOUT: c_uint = 0x2;
pub const LACP_STATE_AGGREGATION: c_uint = 0x4;
pub const LACP_STATE_SYNCHRONIZATION: c_uint = 0x8;
pub const LACP_STATE_COLLECTING: c_uint = 0x10;
pub const LACP_STATE_DISTRIBUTING: c_uint = 0x20;
pub const LACP_STATE_DEFAULTED: c_uint = 0x40;
pub const LACP_STATE_EXPIRED: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_info {
    pub aggregator_id: __u16,
    pub ports: __u16,
    pub actor_key: __u16,
    pub partner_key: __u16,
    pub partner_system: [__u8; ETH_ALEN],
}

// Embedded inside LINK_XSTATS_TYPE_BOND

// Embedded inside BOND_XSTATS_3AD

