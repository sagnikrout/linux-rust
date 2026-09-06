//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/vdso.h
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
// Author: Huacai Chen <chenhuacai@loongson.cn>
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// struct loongarch_vdso_info - Details of a VDSO image.
// @vdso: Pointer to VDSO image (page-aligned).
// @size: Size of the VDSO image (page-aligned).
// @off_rt_sigreturn: Offset of the rt_sigreturn() trampoline.
// @code_mapping: Special mapping structure for vdso code.
// @code_mapping: Special mapping structure for vdso data.
//
// This structure contains details of a VDSO image, including the image data
// and offsets of certain symbols required by the kernel. It is generated as
// part of the VDSO build process, aside from the mapping page array, which is
// populated at runtime.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongarch_vdso_info {
    pub vdso: *mut c_void,
    pub size: c_ulong,
    pub offset_sigreturn: c_ulong,
    pub code_mapping: vm_special_mapping,
}
