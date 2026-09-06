//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kvm_eiointc.h
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
// Copyright (C) 2024 Loongson Technology Corporation Limited
//

pub const EIOINTC_IRQS: c_int = 256;
pub const EIOINTC_ROUTE_MAX_VCPUS: c_int = 256;

// map to ipnum per 32 irqs
pub const EIOINTC_IRQS_NODETYPE_COUNT: c_int = 16;
pub const EIOINTC_BASE: c_uint = 0x1400;
pub const EIOINTC_SIZE: c_uint = 0x900;
pub const EIOINTC_NODETYPE_START: c_uint = 0xa0;
pub const EIOINTC_NODETYPE_END: c_uint = 0xbf;
pub const EIOINTC_IPMAP_START: c_uint = 0xc0;
pub const EIOINTC_IPMAP_END: c_uint = 0xc7;
pub const EIOINTC_ENABLE_START: c_uint = 0x200;
pub const EIOINTC_ENABLE_END: c_uint = 0x21f;
pub const EIOINTC_BOUNCE_START: c_uint = 0x280;
pub const EIOINTC_BOUNCE_END: c_uint = 0x29f;
pub const EIOINTC_ISR_START: c_uint = 0x300;
pub const EIOINTC_ISR_END: c_uint = 0x31f;
pub const EIOINTC_COREISR_START: c_uint = 0x400;
pub const EIOINTC_COREISR_END: c_uint = 0x41f;
pub const EIOINTC_COREMAP_START: c_uint = 0x800;
pub const EIOINTC_COREMAP_END: c_uint = 0x8ff;

pub const LOONGSON_IP_NUM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongarch_eiointc {
    pub lock: spinlock_t,
    pub kvm: *mut kvm,
    pub device: kvm_io_device,
    pub device_vext: kvm_io_device,
    pub num_cpu: u32,
    pub features: u32,
    pub status: u32,
// hardware state
    pub 4]: u64 nodetype[EIOINTC_IRQS_NODETYPE_COUNT /,
// one bit shows the state of one irq
    pub bounce: [u64; EIOINTC_IRQS_U64_NUMS],
    pub isr: [u64; EIOINTC_IRQS_U64_NUMS],
    pub coreisr: [u64; EIOINTC_ROUTE_MAX_VCPUS][EIOINTC_IRQS_U64_NUMS],
    pub enable: [u64; EIOINTC_IRQS_U64_NUMS],
// use one byte to config ipmap for 32 irqs at once
    pub ipmap: u64,
// use one byte to config coremap for one irq
    pub 8]: u64 coremap[EIOINTC_IRQS /,
    pub EIOINTC_IRQS): DECLARE_BITMAP(sw_coreisr[EIOINTC_ROUTE_MAX_VCPUS][LOONGSON_IP_NUM],,
    pub sw_coremap: [u8; EIOINTC_IRQS],
}

extern "C" {
    pub fn kvm_loongarch_register_eiointc_device() -> c_int;
}
extern "C" {
    pub fn kvm_loongarch_unregister_eiointc_device();
}
extern "C" {
    pub fn eiointc_set_irq(s: *mut loongarch_eiointc, irq: c_int, level: c_int);
}
