//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/dev.h
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
// Copyright (c) 2012-2015, NVIDIA Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_channel_ops {
    pub id): c_uint,
    pub job): *mut *mut int (submit)(struct host1x_job,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_cdma_ops {
    pub cdma): *mut *mut void (start)(struct host1x_cdma,
    pub cdma): *mut *mut void (stop)(struct host1x_cdma,
    pub cdma): *mut *mut void (flush)(struct host1x_cdma,
    pub cdma): *mut *mut int (timeout_init)(struct host1x_cdma,
    pub cdma): *mut *mut void (timeout_destroy)(struct host1x_cdma,
    pub cdma): *mut *mut void (freeze)(struct host1x_cdma,
    pub getptr): *mut *mut *mut void (resume)(struct host1x_cdma cdma, u32,
    pub nr_slots): u32 syncpt_incrs, u32 syncval, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_pushbuffer_ops {
    pub pb): *mut *mut void (init)(struct push_buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_debug_ops {
    pub de): *mut *mut void (debug_init)(struct dentry,
    pub o): *mut output,
    pub o): *mut output,
    pub output): *mut *mut *mut void (show_mlocks)(struct host1x host, struct output,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_syncpt_ops {
    pub syncpt): *mut *mut void (restore)(struct host1x_syncpt,
    pub syncpt): *mut *mut void (restore_wait_base)(struct host1x_syncpt,
    pub syncpt): *mut *mut void (load_wait_base)(struct host1x_syncpt,
    pub syncpt): *mut *mut u32 (load)(struct host1x_syncpt,
    pub syncpt): *mut *mut int (cpu_incr)(struct host1x_syncpt,
    pub channel): *mut host1x_channel,
    pub host): *mut *mut void (enable_protection)(struct host1x,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_intr_ops {
    pub cpm): *mut *mut *mut int (init_host_sync)(struct host1x host, u32,
    pub thresh): *mut *mut host1x host, unsigned int id, u32,
    pub id): *mut *mut *mut void (enable_syncpt_intr)(struct host1x host, unsigned int,
    pub id): *mut *mut *mut void (disable_syncpt_intr)(struct host1x host, unsigned int,
    pub host): *mut *mut void (disable_all_syncpt_intrs)(struct host1x,
    pub host): *mut *mut int (free_syncpt_irq)(struct host1x,
    pub dev_id): *mut *mut irqreturn_t (isr)(int irq, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_sid_entry {
    pub base: c_uint,
    pub offset: c_uint,
    pub limit: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_table_desc {
    pub base: c_uint,
    pub count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_info {
    pub /: *mut *mut unsigned int nb_channels; / host1x: number of channels supported,
    pub /: *mut *mut unsigned int nb_pts; / host1x: number of syncpoints supported,
    pub /: *mut *mut unsigned int nb_bases; / host1x: number of syncpoint bases supported,
    pub /: *mut *mut unsigned int nb_mlocks; / host1x: number of mlocks supported,
    pub /: *mut *mut *mut *mut int (init)(struct host1x host1x); / initialize per SoC ops,
    pub /: *mut *mut unsigned int sync_offset; / offset of syncpoint registers,
    pub /: *mut *mut u64 dma_mask; / mask of addressable memory,
    pub /: *mut *mut bool has_wide_gather; / supports GATHER_W opcode,
    pub /: *mut *mut bool has_hypervisor; / has hypervisor registers,
    pub /: *mut *mut bool has_common; / has common registers separate from hypervisor,
    pub num_sid_entries: c_uint,
    pub sid_table: *const host1x_sid_entry,
    pub streamid_vm_table: host1x_table_desc,
    pub classid_vm_table: host1x_table_desc,
    pub mmio_vm_table: host1x_table_desc,
//
// On T20-T148, the boot chain may setup DC to increment syncpoints
// 26/27 on VBLANK. As such we cannot use these syncpoints until
// the display driver disables VBLANK increments.
//
    pub reserve_vblank_syncpts: bool,
//
// On Tegra186, secure world applications may require access to
// host1x during suspend/resume. To allow this, we need to leave
// host1x not in reset.
//
    pub skip_reset_assert: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x {
    pub info: *const host1x_info,
    pub regs: *mut void __iomem,
    pub /: *mut *mut *mut void __iomem hv_regs; / hypervisor region,
    pub common_regs: *mut void __iomem,
    pub syncpt_irqs: [c_int; 8],
    pub num_syncpt_irqs: c_int,
    pub syncpt: *mut host1x_syncpt,
    pub bases: *mut host1x_syncpt_base,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub resets: [reset_control_bulk_data; 2],
    pub nresets: c_uint,
    pub group: *mut iommu_group,
    pub domain: *mut iommu_domain,
    pub iova: iova_domain,
    pub iova_end: dma_addr_t,
    pub intr_mutex: mutex,
    pub syncpt_op: *const host1x_syncpt_ops,
    pub intr_op: *const host1x_intr_ops,
    pub channel_op: *const host1x_channel_ops,
    pub cdma_op: *const host1x_cdma_ops,
    pub cdma_pb_op: *const host1x_pushbuffer_ops,
    pub debug_op: *const host1x_debug_ops,
    pub nop_sp: *mut host1x_syncpt,
    pub syncpt_mutex: mutex,
    pub channel_list: host1x_channel_list,
    pub context_list: host1x_memory_context_list,
    pub debugfs: *mut dentry,
    pub devices_lock: mutex,
    pub devices: list_head,
    pub list: list_head,
    pub dma_parms: device_dma_parameters,
    pub cache: host1x_bo_cache,
}

extern "C" {
    pub fn host1x_common_writel(host1x: *mut host1x, v: u32, r: u32);
}
extern "C" {
    pub fn host1x_hypervisor_writel(host1x: *mut host1x, v: u32, r: u32);
}
extern "C" {
    pub fn host1x_hypervisor_readl(host1x: *mut host1x, r: u32) -> u32;
}
extern "C" {
    pub fn host1x_sync_writel(host1x: *mut host1x, v: u32, r: u32);
}
extern "C" {
    pub fn host1x_sync_readl(host1x: *mut host1x, r: u32) -> u32;
}

extern "C" {
    pub fn host1x_sync_readq(host1x: *mut host1x, r: u32) -> u64;
}

extern "C" {
    pub fn host1x_ch_writel(ch: *mut host1x_channel, v: u32, r: u32);
}
extern "C" {
    pub fn host1x_ch_readl(ch: *mut host1x_channel, r: u32) -> u32;
}
