//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/video/uvesafb.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v86_regs {
    pub ebx: __u32,
    pub ecx: __u32,
    pub edx: __u32,
    pub esi: __u32,
    pub edi: __u32,
    pub ebp: __u32,
    pub eax: __u32,
    pub eip: __u32,
    pub eflags: __u32,
    pub esp: __u32,
    pub cs: __u16,
    pub ss: __u16,
    pub es: __u16,
    pub ds: __u16,
    pub fs: __u16,
    pub gs: __u16,
}

// Task flags
pub const TF_VBEIB: c_uint = 0x01;
pub const TF_BUF_ESDI: c_uint = 0x02;
pub const TF_BUF_ESBX: c_uint = 0x04;
pub const TF_BUF_RET: c_uint = 0x08;
pub const TF_EXIT: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvesafb_task {
    pub flags: __u8,
    pub buf_len: c_int,
    pub regs: v86_regs,
}

// Constants for the capabilities field
// in vbe_ib
pub const VBE_CAP_CAN_SWITCH_DAC: c_uint = 0x01;
pub const VBE_CAP_VGACOMPAT: c_uint = 0x02;
// The VBE Info Block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vbe_ib {
    pub vbe_signature: [c_char; 4],
    pub vbe_version: __u16,
    pub oem_string_ptr: __u32,
    pub capabilities: __u32,
    pub mode_list_ptr: __u32,
    pub total_memory: __u16,
    pub oem_software_rev: __u16,
    pub oem_vendor_name_ptr: __u32,
    pub oem_product_name_ptr: __u32,
    pub oem_product_rev_ptr: __u32,
    pub reserved: [__u8; 222],
    pub oem_data: [c_char; 256],
    pub misc_data: [c_char; 512],
// C attribute field omitted
