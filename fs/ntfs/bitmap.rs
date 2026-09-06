//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/bitmap.h
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
// Defines for NTFS kernel bitmap handling.
//
// Copyright (c) 2004 Anton Altaparmakov
//

extern "C" {
    pub fn ntfs_trim_fs(vol: *mut ntfs_volume, range: *mut fstrim_range) -> c_int;
}
//
// ntfs_bitmap_set_bits_in_run - set a run of bits in a bitmap to a value
// @vi:			vfs inode describing the bitmap
// @start_bit:		first bit to set
// @count:		number of bits to set
// @value:		value to set the bits to (i.e. 0 or 1)
//
// Set @count bits starting at bit @start_bit in the bitmap described by the
// vfs inode @vi to @value, where @value is either 0 or 1.
//
// Return 0 on success and -errno on error.
//
// ntfs_bitmap_set_run - set a run of bits in a bitmap
// @vi:		vfs inode describing the bitmap
// @start_bit:	first bit to set
// @count:	number of bits to set
//
// Set @count bits starting at bit @start_bit in the bitmap described by the
// vfs inode @vi.
//
// Return 0 on success and -errno on error.
//
extern "C" {
    pub fn ntfs_bitmap_set_bits_in_run(_arg: vi, _arg: start_bit, _arg: count, _arg: 1) -> return;
}
//
// ntfs_bitmap_clear_run - clear a run of bits in a bitmap
// @vi:		vfs inode describing the bitmap
// @start_bit:	first bit to clear
// @count:	number of bits to clear
//
// Clear @count bits starting at bit @start_bit in the bitmap described by the
// vfs inode @vi.
//
// Return 0 on success and -errno on error.
//
extern "C" {
    pub fn ntfs_bitmap_set_bits_in_run(_arg: vi, _arg: start_bit, _arg: count, _arg: 0) -> return;
}
//
// ntfs_bitmap_set_bit - set a bit in a bitmap
// @vi:		vfs inode describing the bitmap
// @bit:	bit to set
//
// Set bit @bit in the bitmap described by the vfs inode @vi.
//
// Return 0 on success and -errno on error.
//
extern "C" {
    pub fn ntfs_bitmap_set_run(_arg: vi, _arg: bit, _arg: 1) -> return;
}
//
// ntfs_bitmap_clear_bit - clear a bit in a bitmap
// @vi:		vfs inode describing the bitmap
// @bit:	bit to clear
//
// Clear bit @bit in the bitmap described by the vfs inode @vi.
//
// Return 0 on success and -errno on error.
//
extern "C" {
    pub fn ntfs_bitmap_clear_run(_arg: vi, _arg: bit, _arg: 1) -> return;
}
