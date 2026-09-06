//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/miscdevice.h
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

//
// These allocations are managed by device@lanana.org. If you need
// an entry that is not assigned here, it can be moved and
// reassigned or dynamically set if a fixed value is not justified.
//
pub const PSMOUSE_MINOR: c_int = 1;

// #define AMIGAMOUSE_MINOR	4	FIXME OBSOLETE

// #define ADB_MOUSE_MINOR	10	FIXME OBSOLETE

pub const APM_MINOR_DEV: c_int = 134;
pub const RTC_MINOR: c_int = 135;
// #define EFI_RTC_MINOR		136	was EFI Time services
pub const VHCI_MINOR: c_int = 137;
pub const SUN_OPENPROM_MINOR: c_int = 139;

pub const NVRAM_MINOR: c_int = 144;
pub const SBUS_FLASH_MINOR: c_int = 152;
pub const SGI_MMTIMER: c_int = 153;
pub const PMU_MINOR: c_int = 154;

pub const LCD_MINOR: c_int = 156;
pub const AC_MINOR: c_int = 157;

pub const ENVCTRL_MINOR: c_int = 162;
pub const I2O_MINOR: c_int = 166;
pub const UCTRL_MINOR: c_int = 174;
pub const AGPGART_MINOR: c_int = 175;
pub const TOSH_MINOR_DEV: c_int = 181;
pub const HWRNG_MINOR: c_int = 183;
// #define MICROCODE_MINOR	184	unused
pub const KEYPAD_MINOR: c_int = 185;
pub const IRNET_MINOR: c_int = 187;
pub const D7S_MINOR: c_int = 193;
pub const VFIO_MINOR: c_int = 196;
pub const PXA3XX_GCU_MINOR: c_int = 197;
pub const TUN_MINOR: c_int = 200;
pub const CUSE_MINOR: c_int = 203;
pub const MPT_MINOR: c_int = 220;
pub const MPT2SAS_MINOR: c_int = 221;
pub const MPT3SAS_MINOR: c_int = 222;
pub const UINPUT_MINOR: c_int = 223;
pub const MISC_MCELOG_MINOR: c_int = 227;
pub const HPET_MINOR: c_int = 228;
pub const FUSE_MINOR: c_int = 229;
pub const SNAPSHOT_MINOR: c_int = 231;
pub const KVM_MINOR: c_int = 232;
pub const BTRFS_MINOR: c_int = 234;
pub const AUTOFS_MINOR: c_int = 235;
pub const MAPPER_CTRL_MINOR: c_int = 236;
pub const LOOP_CTRL_MINOR: c_int = 237;
pub const VHOST_NET_MINOR: c_int = 238;
pub const UHID_MINOR: c_int = 239;
pub const USERIO_MINOR: c_int = 240;
pub const VHOST_VSOCK_MINOR: c_int = 241;
pub const EISA_EEPROM_MINOR: c_int = 241;
pub const RFKILL_MINOR: c_int = 242;
//
// Misc char device minor code space division related to below macro:
//
// <  255  : Fixed minor code
// == 255  : Indicator to request dynamic minor code
// >  255  : Dynamic minor code requested, 1048320 minor codes totally.
//
pub const MISC_DYNAMIC_MINOR: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct miscdevice {
    pub minor: c_int,
    pub name: *const c_char,
    pub fops: *const file_operations,
    pub list: list_head,
    pub parent: *mut device,
    pub this_device: *mut device,
    pub groups: *const attribute_group,
    pub nodename: *const c_char,
    pub mode: umode_t,
}

extern "C" {
    pub fn misc_register(misc: *mut miscdevice) -> c_int;
}
extern "C" {
    pub fn misc_deregister(misc: *mut miscdevice);
}
//
// Helper macro for drivers that don't do anything special in the initcall.
// This helps to eliminate boilerplate code.
//

//
// Helper macro for drivers that don't do anything special in module init / exit
// call. This helps to eliminate boilerplate code.
//

