//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kho/abi/block.h
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
// Copyright (c) 2026, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//
// DOC: KHO Serialization Blocks ABI
//
// Subsystems using the KHO Serialization Blocks framework rely on the stable
// Application Binary Interface defined below to pass serialized state from a
// pre-update kernel to a post-update kernel.
//
// This interface is a contract. Any modification to the structure fields,
// compatible strings, or the layout of the `__packed` serialization
// structures defined here constitutes a breaking change. Such changes require
// incrementing the version number in the `KHO_FDT_COMPATIBLE` string to
// prevent a new kernel from misinterpreting data from an old kernel.
//
// Changes are allowed provided the compatibility version is incremented;
// however, backward/forward compatibility is only guaranteed for kernels
// supporting the same ABI version.
//

//
// KHO_BLOCK_SIZE - The size of each serialization block.
//
// This is defined as PAGE_SIZE. PAGE_SIZE is ABI compliant because live
// update between kernels with different page sizes is not supported by KHO.
//

//
// struct kho_block_header_ser - Header for the serialized data block.
// @next:  Physical address of the next struct kho_block_header_ser.
// @count: The number of entries that immediately follow this header in the
// memory block.
//
// This structure is located at the beginning of a block of physical memory
// preserved across a kexec. It provides the necessary metadata to interpret
// the array of entries that follow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_block_header_ser {
    pub next: u64,
    pub count: u64,
    pub __packed: },
