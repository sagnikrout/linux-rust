//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/netdev_features.h
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
// Network device features.
//

pub type netdev_features_t = u64;
// NETIF_F_GSO_SHIFT,		/* keep the order of SKB_GSO_* bits
// in hardware and all other
// headers in software.
//
// NETIF_F_GSO_LAST =		/* last bit, see GSO_MASK
//
// Add your fresh new feature above and remember to update
// netdev_features_strings[] in net/ethtool/common.c and maybe
// some feature mask #defines below. Please also describe it
// in Documentation/networking/netdev-features.rst.
//
// NETDEV_FEATURE_COUNT
// copy'n'paste compression ;)

// Finds the next feature with the highest number of the range of start-1 till 0.
//
// like BITMAP_LAST_WORD_MASK() for u64
// this sets the most significant 64 - start to 0.
//
// This goes for the MSB to the LSB through the set feature bits,
// mask_addr should be a u64 and bit an int
//

// Features valid for ethtool to change
// = all defined minus driver/device-class-related

// remember that ((t)1 << t_BITS) is undefined in C99

// Segmentation offload feature mask

// List of IP checksum features. Note that NETIF_F_HW_CSUM should not be
// set in features when NETIF_F_IP_CSUM or NETIF_F_IPV6_CSUM are set--
// this would be contradictory
//

// List of features with software fallbacks.

//
// If one device supports one of these features, then enable them
// for all in netdev_increment_features.
//

//
// If one device doesn't support one of these features, then disable it
// for all in netdev_increment_features.
//

//
// If upper/master device has these features disabled, they must be disabled
// on all lower/slave devices as well.
//

// changeable features with no special hardware requirements

// Changeable features with no special hardware requirements that defaults to off.

// virtual device features

