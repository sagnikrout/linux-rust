//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/vgic/vgic-mmio.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2015, 2016 ARM Ltd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_register_region {
    pub reg_offset: c_uint,
    pub len: c_uint,
    pub bits_per_irq: c_uint,
    pub access_flags: c_uint,
    pub len): c_uint,
    pub len): gpa_t addr, unsigned int,
}

pub const VGIC_ACCESS_8bit: c_int = 1;
pub const VGIC_ACCESS_32bit: c_int = 2;
pub const VGIC_ACCESS_64bit: c_int = 4;
//
// Generate a mask that covers the number of bytes required to address
// up to 1024 interrupts, each represented by <bits> bits. This assumes
// that <bits> is a power of two.
//

//
// (addr & mask) gives us the _byte_ offset for the INT ID.
// We multiply this by 8 the get the _bit_ offset, then divide this by
// the number of bits to learn the actual INT ID.
// But instead of a division (which requires a "long long div" implementation),
// we shift by the binary logarithm of <bits>.
// This assumes that <bits> is a power of two.
//

//
// Some VGIC registers store per-IRQ information, with a different number
// of bits per IRQ. For those registers this macro is used.
// The _WITH_LENGTH version instantiates registers with a fixed length
// and is mutually exclusive with the _PER_IRQ version.
//

extern "C" {
    pub fn vgic_data_mmio_bus_to_host(val: *const c_void, len: c_uint) -> c_ulong;
}
extern "C" {
    pub fn vgic_read_irq_line_level_info(vcpu: *mut kvm_vcpu, intid: u32) -> u32;
}
extern "C" {
    pub fn vgic_v2_init_dist_iodev(dev: *mut vgic_io_device) -> c_uint;
}
extern "C" {
    pub fn vgic_v2_init_cpuif_iodev(dev: *mut vgic_io_device) -> c_uint;
}
extern "C" {
    pub fn vgic_v3_init_dist_iodev(dev: *mut vgic_io_device) -> c_uint;
}
extern "C" {
    pub fn vgic_sanitise_outer_cacheability(reg: u64) -> u64;
}
extern "C" {
    pub fn vgic_sanitise_inner_cacheability(reg: u64) -> u64;
}
extern "C" {
    pub fn vgic_sanitise_shareability(reg: u64) -> u64;
}
// Find the proper register handler entry given a certain address offset
