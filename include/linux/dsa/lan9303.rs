//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dsa/lan9303.h
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


// Included by drivers/net/dsa/lan9303.h and net/dsa/tag_lan9303.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan9303_phy_ops {
// PHY 1 and 2 access
    pub regnum): *mut *mut *mut int (phy_read)(struct lan9303 chip, int addr, int,
    pub val): int regnum, u16,
}

pub const LAN9303_NUM_ALR_RECORDS: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan9303_alr_cache_entry {
    pub mac_addr: [u8; ETH_ALEN],
    pub /: *mut *mut u8 port_map; / Bitmap of ports. Zero if unused entry,
    pub /: *mut *mut u8 stp_override; / non zero if set LAN9303_ALR_DAT1_AGE_OVERRID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan9303 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub reset_gpio: *mut gpio_desc,
    pub /: *mut *mut u32 reset_duration; / in [ms],
    pub phy_addr_base: c_int,
    pub ds: *mut dsa_switch,
    pub /: *mut *mut mutex indirect_mutex; / protect indexed register access,
    pub /: *mut *mut mutex alr_mutex; / protect ALR access,
    pub ops: *const lan9303_phy_ops,
    pub /: *mut *mut bool is_bridged; / true if port 1 and 2 are bridged,
// remember LAN9303_SWE_PORT_STATE while not bridged
    pub swe_port_state: u32,
// LAN9303 do not offer reading specific ALR entry. Cache all
// static entries in a flat table
//
    pub alr_cache: [lan9303_alr_cache_entry; LAN9303_NUM_ALR_RECORDS],
}
