//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/bootparam_utils.h
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
// This file is included from multiple environments.  Do not
// add completing #includes to make it standalone.
//
// Deal with bootloaders which fail to initialize unknown fields in
// boot_params to zero.  The list fields in this list are taken from
// analysis of kexec-tools; if other broken bootloaders initialize a
// different set of fields we will need to figure out how to disambiguate.
//
// Note: efi_info is commonly left uninitialized, but that field has a
// private magic, so it is better to leave it unchanged.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_params_to_save {
    pub start: c_uint,
    pub len: c_uint,
}

//
// IMPORTANT NOTE TO BOOTLOADER AUTHORS: do not simply clear
// this field.  The purpose of this field is to guarantee
// compliance with the x86 boot spec located in
// Documentation/arch/x86/boot.rst .  That spec says that the
// *whole* structure should be cleared, after which only the
// portion defined by struct setup_header (boot_params->hdr)
// should be copied in.
//
// If you're having an issue because the sentinel is set, you
// need to change the whole structure to be cleared, not this
// (or any other) individual field, or you will soon have
// problems again.
//
