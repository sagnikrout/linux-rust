//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/power/bq27xxx_battery.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bq27xxx_chip {
    BQ27000 = 1, /* bq27000, bq27200 */
    BQ27010, /* bq27010, bq27210 */
    BQ2750X, /* bq27500 deprecated alias */
    BQ2751X, /* bq27510, bq27520 deprecated alias */
    BQ2752X,
    BQ27500, /* bq27500/1 */
    BQ27510G1, /* bq27510G1 */
    BQ27510G2, /* bq27510G2 */
    BQ27510G3, /* bq27510G3 */
    BQ27520G1, /* bq27520G1 */
    BQ27520G2, /* bq27520G2 */
    BQ27520G3, /* bq27520G3 */
    BQ27520G4, /* bq27520G4 */
    BQ27521, /* bq27521 */
    BQ27530, /* bq27530, bq27531 */
    BQ27531,
    BQ27541, /* bq27541, bq27542, bq27546, bq27742 */
    BQ27542,
    BQ27546,
    BQ27742,
    BQ27545, /* bq27545 */
    BQ27411,
    BQ27421, /* bq27421, bq27441, bq27621 */
    BQ27425,
    BQ27426,
    BQ27441,
    BQ27621,
    BQ27Z561,
    BQ28Z610,
    BQ34Z100,
    BQ78Z100,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq27xxx_access_methods {
    pub single): *mut *mut *mut int (read)(struct bq27xxx_device_info di, u8 reg, bool,
    pub single): *mut *mut *mut int (write)(struct bq27xxx_device_info di, u8 reg, int value, bool,
    pub len): *mut *mut *mut *mut int (read_bulk)(struct bq27xxx_device_info di, u8 reg, u8 data, int,
    pub len): *mut *mut *mut *mut int (write_bulk)(struct bq27xxx_device_info di, u8 reg, u8 data, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq27xxx_reg_cache {
    pub capacity: c_int,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bq27xxx_device_info {
    pub dev: *mut device,
    pub chip: bq27xxx_chip,
    pub opts: u32,
    pub name: *const c_char,
    pub dm_regs: *mut bq27xxx_dm_reg,
    pub unseal_key: u32,
    pub bus: bq27xxx_access_methods,
    pub cache: bq27xxx_reg_cache,
    pub charge_design_full: c_int,
    pub voltage_min_design: c_int,
    pub voltage_max_design: c_int,
    pub removed: bool,
    pub last_update: c_ulong,
    pub last_status: power_supply_propval,
    pub work: delayed_work,
    pub bat: *mut power_supply,
    pub list: list_head,
    pub lock: mutex,
    pub regs: *mut u8,
}

extern "C" {
    pub fn bq27xxx_battery_update(di: *mut bq27xxx_device_info);
}
extern "C" {
    pub fn bq27xxx_battery_setup(di: *mut bq27xxx_device_info) -> c_int;
}
extern "C" {
    pub fn bq27xxx_battery_teardown(di: *mut bq27xxx_device_info);
}
