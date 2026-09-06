//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/isp1760/isp1760-hcd.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp1760_slotinfo {
    pub qh: *mut isp1760_qh,
    pub qtd: *mut isp1760_qtd,
    pub timestamp: c_ulong,
}

// chip memory management

pub const ISP176x_BLOCK_NUM: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp1760_memory_layout {
    pub blocks: [c_uint; ISP176x_BLOCK_NUM],
    pub blocks_size: [c_uint; ISP176x_BLOCK_NUM],
    pub slot_num: c_uint,
    pub payload_blocks: c_uint,
    pub payload_area_size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp1760_memory_chunk {
    pub start: c_uint,
    pub size: c_uint,
    pub free: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp1760_queue_head_types {
    QH_CONTROL,
    QH_BULK,
    QH_INTERRUPT,
    QH_END
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp1760_hcd {
    pub hcd: *mut usb_hcd,
    pub base: *mut void __iomem,
    pub regs: *mut regmap,
    pub fields: [*mut regmap_field; HC_FIELD_MAX],
    pub is_isp1763: bool,
    pub memory_layout: *const isp1760_memory_layout,
    pub lock: spinlock_t,
    pub atl_slots: *mut isp1760_slotinfo,
    pub atl_done_map: c_int,
    pub int_slots: *mut isp1760_slotinfo,
    pub int_done_map: c_int,
    pub memory_pool: [isp1760_memory_chunk; ISP176x_BLOCK_MAX],
    pub qh_list: [list_head; QH_END],
// periodic schedule support
pub const DEFAULT_I_TDPS: c_int = 1024;
    pub periodic_size: unsigned,
    pub i_thresh: unsigned,
    pub reset_done: c_ulong,
    pub next_statechange: c_ulong,
}

extern "C" {
    pub fn isp1760_hcd_unregister(priv: *mut isp1760_hcd);
}
extern "C" {
    pub fn isp1760_init_kmem_once() -> c_int;
}
extern "C" {
    pub fn isp1760_deinit_kmem_cache();
}

