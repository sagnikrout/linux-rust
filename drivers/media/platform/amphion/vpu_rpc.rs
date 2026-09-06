//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/amphion/vpu_rpc.h
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
// Copyright 2020-2021 NXP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_rpc_buffer_desc {
    pub wptr: u32,
    pub rptr: u32,
    pub start: u32,
    pub end: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_shared_addr {
    pub iface: *mut c_void,
    pub cmd_desc: *mut vpu_rpc_buffer_desc,
    pub cmd_mem_vir: *mut c_void,
    pub msg_desc: *mut vpu_rpc_buffer_desc,
    pub msg_mem_vir: *mut c_void,
    pub boot_addr: c_ulong,
    pub core: *mut vpu_core,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_rpc_event_header {
    pub index: u32,
    pub id: u32,
    pub num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_rpc_event {
    pub hdr: vpu_rpc_event_header,
    pub data: [u32; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_iface_ops {
    pub type): *mut *mut bool (check_codec)(enum vpu_core_type,
    pub pixelfmt): *mut *mut bool (check_fmt)(enum vpu_core_type type, u32,
    pub (*get_data_size)(void): *mut u32,
    pub size): *mut *mut int (check_memory_region)(dma_addr_t base, dma_addr_t addr, u32,
    pub core): *mut *mut int (boot_core)(struct vpu_core,
    pub core): *mut *mut int (shutdown_core)(struct vpu_core,
    pub core): *mut *mut int (restore_core)(struct vpu_core,
    pub core): *mut *mut int (get_power_state)(struct vpu_core,
    pub core): *mut *mut int (on_firmware_loaded)(struct vpu_core,
    pub boot_addr): *mut *mut vpu_buffer rpc, dma_addr_t,
    pub log): *mut vpu_buffer,
    pub index): *mut *mut u32 regs_base, void __iomem regs, u32,
    pub index): *mut *mut *mut void (set_stream_cfg)(struct vpu_shared_addr shared, u32,
    pub shared): *mut *mut u32 (get_version)(struct vpu_shared_addr,
    pub shared): *mut *mut u32 (get_max_instance_count)(struct vpu_shared_addr,
    pub shared): *mut *mut int (get_stream_buffer_size)(struct vpu_shared_addr,
    pub cmd): *mut vpu_rpc_event,
    pub msg): *mut vpu_rpc_event,
    pub data): *mut *mut *mut int (pack_cmd)(struct vpu_rpc_event pkt, u32 index, u32 id, void,
    pub msg_id): *mut *mut int (convert_msg_id)(u32,
    pub data): *mut *mut *mut int (unpack_msg_data)(struct vpu_rpc_event pkt, void,
    pub vb): *mut *mut vpu_inst inst, vb2_buffer,
    pub buf): *mut vpu_buffer,
    pub buf): *mut vpu_buffer,
    pub write): u32 instance, u32 ptr, bool,
    pub desc): *mut vpu_rpc_buffer_desc,
    pub update): u32,
    pub update): u32,
    pub scode_type): u32,
    pub instance): *mut *mut *mut int (pre_send_cmd)(struct vpu_shared_addr shared, u32,
    pub instance): *mut *mut *mut int (post_send_cmd)(struct vpu_shared_addr shared, u32,
    pub instance): *mut *mut *mut int (init_instance)(struct vpu_shared_addr shared, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vpu_rpc_region_t {
    pub start: dma_addr_t,
    pub end: dma_addr_t,
    pub type: dma_addr_t,
}

extern "C" {
    pub fn vpu_iface_check_memory_region(core: *mut vpu_core, addr: dma_addr_t, size: u32) -> c_int;
}
