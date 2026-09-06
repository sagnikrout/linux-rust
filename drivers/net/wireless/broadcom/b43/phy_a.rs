//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/phy_a.h
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

// OFDM (A) PHY Registers

pub const B43_PHY_BBANDCFG_RXANT: c_uint = 0x180	/* RX Antenna selection */;
pub const B43_PHY_BBANDCFG_RXANT_SHIFT: c_int = 7;

pub const B43_PHY_CRS0_EN: c_uint = 0x4000;

pub const B43_PHY_ANTDWELL_AUTODIV1: c_uint = 0x0100	/* Automatic RX diversity start antenna */;

pub const B43_PHY_ENCORE_EN: c_uint = 0x0200	/* Encore enable */;

pub const B43_PHY_OFDM61_10: c_uint = 0x0010	/* FIXME rename */;

pub const B43_PHY_OTABLEOFF: c_uint = 0x03FF	/* OFDM table offset (see below) */;
pub const B43_PHY_OTABLENR: c_uint = 0xFC00	/* OFDM table number (see below) */;
pub const B43_PHY_OTABLENR_SHIFT: c_int = 10;

pub const B43_PHY_ANTWRSETT_ARXDIV: c_uint = 0x2000	/* Automatic RX diversity enabled */;

// OFDM table numbers

extern "C" {
    pub fn b43_ofdmtab_read16(dev: *mut b43_wldev, table: u16, offset: u16) -> u16;
}
extern "C" {
    pub fn b43_ofdmtab_read32(dev: *mut b43_wldev, table: u16, offset: u16) -> u32;
}
