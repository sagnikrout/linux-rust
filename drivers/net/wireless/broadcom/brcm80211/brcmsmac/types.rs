//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmsmac/types.h
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


//
// Copyright (c) 2010 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

pub const WL_CHAN_FREQ_RANGE_2G: c_int = 0;
pub const WL_CHAN_FREQ_RANGE_5GL: c_int = 1;
pub const WL_CHAN_FREQ_RANGE_5GM: c_int = 2;
pub const WL_CHAN_FREQ_RANGE_5GH: c_int = 3;
// boardflags
// Board has gpio 9 controlling the PA
pub const BFL_PACTRL: c_uint = 0x00000002;
// Not ok to power down the chip pll and oscillator
pub const BFL_NOPLLDOWN: c_uint = 0x00000020;
// Board supports the Front End Module
pub const BFL_FEM: c_uint = 0x00000800;
// Board has an external LNA in 2.4GHz band
pub const BFL_EXTLNA: c_uint = 0x00001000;
// Board has no PA
pub const BFL_NOPA: c_uint = 0x00010000;
// Power topology uses BUCKBOOST
pub const BFL_BUCKBOOST: c_uint = 0x00200000;
// Board has FEM and switch to share antenna w/ BT
pub const BFL_FEM_BT: c_uint = 0x00400000;
// Power topology doesn't use CBUCK
pub const BFL_NOCBUCK: c_uint = 0x00800000;
// Power topology uses PALDO
pub const BFL_PALDO: c_uint = 0x02000000;
// Board has an external LNA in 5GHz band
pub const BFL_EXTLNA_5GHz: c_uint = 0x10000000;
// boardflags2
// Board has an external rxbb regulator
pub const BFL2_RXBB_INT_REG_DIS: c_uint = 0x00000001;
// Flag to implement alternative A-band PLL settings
pub const BFL2_APLL_WAR: c_uint = 0x00000002;
// Board permits enabling TX Power Control
pub const BFL2_TXPWRCTRL_EN: c_uint = 0x00000004;
// Board supports the 2X4 diversity switch
pub const BFL2_2X4_DIV: c_uint = 0x00000008;
// Board supports 5G band power gain
pub const BFL2_5G_PWRGAIN: c_uint = 0x00000010;
// Board overrides ASPM and Clkreq settings
pub const BFL2_PCIEWAR_OVR: c_uint = 0x00000020;
pub const BFL2_LEGACY: c_uint = 0x00000080;
// 4321mcm93 board uses Skyworks FEM
pub const BFL2_SKWRKFEM_BRD: c_uint = 0x00000100;
// Board has a WAR for clock-harmonic spurs
pub const BFL2_SPUR_WAR: c_uint = 0x00000200;
// Flag to narrow G-band PLL loop b/w
pub const BFL2_GPLL_WAR: c_uint = 0x00000400;
// Tx CCK pkts on Ant 0 only
pub const BFL2_SINGLEANT_CCK: c_uint = 0x00001000;
// WAR to reduce and avoid clock-harmonic spurs in 2G
pub const BFL2_2G_SPUR_WAR: c_uint = 0x00002000;
// Flag to widen G-band PLL loop b/w
pub const BFL2_GPLL_WAR2: c_uint = 0x00010000;
pub const BFL2_IPALVLSHIFT_3P3: c_uint = 0x00020000;
// Use internal envelope detector for TX IQCAL
pub const BFL2_INTERNDET_TXIQCAL: c_uint = 0x00040000;
// Keep the buffered Xtal output from radio "ON". Most drivers will turn it
// off without this flag to save power.
pub const BFL2_XTALBUFOUTEN: c_uint = 0x00080000;
//
// board specific GPIO assignment, gpio 0-3 are also customer-configurable
// led
//
// bit 9 controls the PA on new 4306 boards
pub const BOARD_GPIO_PACTRL: c_uint = 0x200;
pub const BOARD_GPIO_12: c_uint = 0x1000;
pub const BOARD_GPIO_13: c_uint = 0x2000;
// **** Core type/rev defaults ****
pub const D11CONF: c_uint = 0x0fffffb0	/* Supported  D11 revs: 4, 5, 7-27;
// also need to update wlc.h MAXCOREREV
//
pub const NCONF: c_uint = 0x000001ff	/* Supported nphy revs:;
// 0       4321a0
// 1       4321a1
// 2       4321b0/b1/c0/c1
// 3       4322a0
// 4       4322a1
// 5       4716a0
// 6       43222a0, 43224a0
// 7       43226a0
// 8       5357a0, 43236a0
//
pub const LCNCONF: c_uint = 0x00000007	/* Supported lcnphy revs:;
// 0       4313a0, 4336a0, 4330a0
// 1
// 2       4330a0
//
pub const SSLPNCONF: c_uint = 0x0000000f	/* Supported sslpnphy revs:;
// 0       4329a0/k0
// 1       4329b0/4329C0
// 2       4319a0
// 3       5356a0
//
// Phy/Core Configuration.  Defines macros to check core phy/rev
// compile-time configuration.  Defines default core support.
//
// Basic macros to check a configuration bitmask

// Wrappers for some of the above, specific to config constants

// Set up PHYTYPE automatically: (depends on PHY_TYPE_X, from d11.h)

// Utility macro to identify 802.11n (HT) capable PHYs

// Last but not least: shorter wlc-specific var checks

//
// ------------- End of Core phy/rev configuration. -----------------
//

//
// bcm4716 (which includes 4717 & 4718), plus 4706 on PCIe can reorder
// transactions. As a fix, a read after write is performed on certain places
// in the code. Older chips and the newer 5357 family don't require this fix.
//

// multi-bool data type: set of bools, mbool is true if any is set
// set one bool

// clear one bool

// true if one bool is set

// forward declarations
// brcm_msg_level is a bit vector with defs in defs.h
