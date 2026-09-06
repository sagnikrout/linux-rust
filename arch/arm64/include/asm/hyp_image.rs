//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/hyp_image.h
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
// Copyright (C) 2020 Google LLC.
// Written by David Brazdil <dbrazdil@google.com>
//

//
// KVM nVHE code has its own symbol namespace prefixed with __kvm_nvhe_,
// to separate it from the kernel proper.
//

//
// KVM nVHE ELF section names are prefixed with .hyp, to separate them
// from the kernel proper.
//

// Symbol defined at the beginning of each hyp section.

//
// Helper to generate linker script statements starting a hyp section.
//
// A symbol with a well-known name is defined at the first byte. This
// is used as a base for hyp relocations (see gen-hyprel.c). It must
// be defined inside the section so the linker of `vmlinux` cannot
// separate it from the section data.
//

// Helper to generate linker script statements ending a hyp section.

// Defines an ELF hyp section from input section @NAME and its subsections.

// (NAME NAME##.*)		\
//
// Defines a linker script alias of a kernel-proper symbol referenced by
// KVM nVHE hyp code.
//

// Defines a linker script alias for KVM nVHE hyp symbols

