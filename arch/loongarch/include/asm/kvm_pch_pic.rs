//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kvm_pch_pic.h
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

pub const PCH_PIC_SIZE: c_uint = 0x3e8;
pub const PCH_PIC_INT_ID_START: c_uint = 0x0;
pub const PCH_PIC_INT_ID_END: c_uint = 0x7;
pub const PCH_PIC_MASK_START: c_uint = 0x20;
pub const PCH_PIC_MASK_END: c_uint = 0x27;
pub const PCH_PIC_HTMSI_EN_START: c_uint = 0x40;
pub const PCH_PIC_HTMSI_EN_END: c_uint = 0x47;
pub const PCH_PIC_EDGE_START: c_uint = 0x60;
pub const PCH_PIC_EDGE_END: c_uint = 0x67;
pub const PCH_PIC_CLEAR_START: c_uint = 0x80;
pub const PCH_PIC_CLEAR_END: c_uint = 0x87;
pub const PCH_PIC_AUTO_CTRL0_START: c_uint = 0xc0;
pub const PCH_PIC_AUTO_CTRL0_END: c_uint = 0xc7;
pub const PCH_PIC_AUTO_CTRL1_START: c_uint = 0xe0;
pub const PCH_PIC_AUTO_CTRL1_END: c_uint = 0xe7;
pub const PCH_PIC_ROUTE_ENTRY_START: c_uint = 0x100;
pub const PCH_PIC_ROUTE_ENTRY_END: c_uint = 0x13f;
pub const PCH_PIC_HTMSI_VEC_START: c_uint = 0x200;
pub const PCH_PIC_HTMSI_VEC_END: c_uint = 0x23f;
pub const PCH_PIC_INT_IRR_START: c_uint = 0x380;
pub const PCH_PIC_INT_IRR_END: c_uint = 0x38f;
pub const PCH_PIC_INT_ISR_START: c_uint = 0x3a0;
pub const PCH_PIC_INT_ISR_END: c_uint = 0x3af;
pub const PCH_PIC_POLARITY_START: c_uint = 0x3e0;
pub const PCH_PIC_POLARITY_END: c_uint = 0x3e7;
pub const PCH_PIC_INT_ID_VAL: c_uint = 0x7UL;
pub const PCH_PIC_INT_ID_VER: c_uint = 0x1UL;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pch_pic_id {
    pub reserved_0: [u8; 3],
    pub id: u8,
    pub version: u8,
    pub reserved_1: u8,
    pub irq_num: u8,
    pub reserved_2: u8,
    pub desc: },
    pub data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongarch_pch_pic {
    pub lock: spinlock_t,
    pub kvm: *mut kvm,
    pub device: kvm_io_device,
    pub id: pch_pic_id,
    pub /: *mut *mut uint64_t mask; / 1:disable irq, 0:enable irq,
    pub /: *mut *mut uint64_t htmsi_en; / 1:msi,
    pub /: *mut *mut uint64_t edge; / 1:edge triggered, 0:level triggered,
    pub /: *mut *mut uint64_t auto_ctrl0; / only use default value 00b,
    pub /: *mut *mut uint64_t auto_ctrl1; / only use default value 00b,
    pub /: *mut *mut uint64_t last_intirr; / edge detection,
    pub /: *mut *mut uint64_t irr; / interrupt request register,
    pub /: *mut *mut uint64_t isr; / interrupt service register,
    pub /: *mut *mut uint64_t polarity; / 0: high level trigger, 1: low level trigger,
    pub /: *mut *mut uint8_t route_entry[64]; / default value 0, route to int0: eiointc,
    pub /: *mut *mut uint8_t htmsi_vector[64]; / irq route table for routing to eiointc,
    pub pch_pic_base: u64,
}

extern "C" {
    pub fn kvm_loongarch_register_pch_pic_device() -> c_int;
}
extern "C" {
    pub fn kvm_loongarch_unregister_pch_pic_device();
}
extern "C" {
    pub fn pch_pic_set_irq(s: *mut loongarch_pch_pic, irq: c_int, level: c_int);
}
extern "C" {
    pub fn pch_msi_set_irq(kvm: *mut kvm, e: *mut kvm_kernel_irq_routing_entry, level: c_int) -> c_int;
}
