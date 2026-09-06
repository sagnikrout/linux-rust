//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kho/abi/memblock.h
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
// DOC: memblock kexec handover ABI
//
// Memblock can serialize its current memory reservations created with
// reserve_mem command line option across kexec through KHO.
// The post-KHO kernel can then consume these reservations and they are
// guaranteed to have the same physical address.
//
// The state is serialized using Flattened Device Tree (FDT) format. Any
// modification to the FDT structure, node properties, or the compatible
// strings constitutes a breaking change. Such changes require incrementing the
// version number in the relevant `_COMPATIBLE` string to prevent a new kernel
// from misinterpreting data from an old kernel.
//
// Changes are allowed provided the compatibility version is incremented.
// However, backward/forward compatibility is only guaranteed for kernels
// supporting the same ABI version.
//
// FDT Structure Overview:
// The entire memblock state is encapsulated within a single KHO entry named
// "memblock".
// This entry contains an FDT with the following layout:
//
// .. code-block:: none
//
// / {
// compatible = "memblock-v1";
//
// n1 {
// compatible = "reserve-mem-v1";
// start = <0xc06b 0x4000000>;
// size = <0x04 0x00>;
// };
//
// Main memblock node (/):
//
// - compatible: "memblock-v1"
// Identifies the overall memblock ABI version.
//
// reserved_mem node:
// These nodes describe all reserve_mem regions. The node name is the name
// defined by the user for a reserve_mem region.
//
// - compatible: "reserve-mem-v1"
//
// Identifies the ABI version of reserve_mem descriptions
//
// - start: u64
//
// Physical address of the reserved memory region.
//
// - size: u64
//
// size in bytes of the reserved memory region.
//
// Top level memblock FDT node name.

// The compatible string for the memblock FDT root node.

// The compatible string for the reserve_mem FDT nodes.

