//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/catpt/core.h
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
// Copyright(c) 2020 Intel Corporation
//
// Author: Cezary Rojewski <cezary.rojewski@intel.com>
//

extern "C" {
    pub fn catpt_sram_free(sram: *mut resource);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_ipc_msg {
    pub header: u32,
    pub rsp: catpt_global_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_ipc {
    pub dev: *mut device,
    pub rx: catpt_ipc_msg,
    pub config: catpt_fw_ready,
    pub default_timeout: u32,
    pub ready: bool,
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub done_completion: completion,
    pub busy_completion: completion,
}

extern "C" {
    pub fn catpt_ipc_init(ipc: *mut catpt_ipc, dev: *mut device);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_module_type {
    pub loaded: bool,
    pub entry_point: u32,
    pub persistent_size: u32,
    pub scratch_size: u32,
// DRAM, initial module state
    pub state_offset: u32,
    pub state_size: u32,
    pub node: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_spec {
    pub machines: *mut snd_soc_acpi_mach,
    pub core_id: u8,
    pub fw_name: *const c_char,
    pub host_dram_offset: u32,
    pub host_iram_offset: u32,
    pub host_shim_offset: u32,
    pub host_dma_offset: [u32; CATPT_DMA_COUNT],
    pub host_ssp_offset: [u32; CATPT_SSP_COUNT],
    pub dram_mask: u32,
    pub iram_mask: u32,
    pub d3srampgd_bit: u32,
    pub d3pgd_bit: u32,
    pub enable): *mut *mut *mut void (pll_shutdown)(struct catpt_dev cdev, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_dev {
    pub dev: *mut device,
    pub dmac: *mut dw_dma_chip,
    pub ipc: catpt_ipc,
    pub pci_ba: *mut void __iomem,
    pub lpe_ba: *mut void __iomem,
    pub lpe_base: u32,
    pub irq: c_int,
    pub spec: *const catpt_spec,
    pub fw_ready: completion,
    pub dram: resource,
    pub iram: resource,
    pub scratch: *mut resource,
    pub mixer: catpt_mixer_stream_info,
    pub modules: [catpt_module_type; CATPT_MODULE_COUNT],
    pub devfmt: [catpt_ssp_device_format; CATPT_SSP_COUNT],
    pub stream_list: list_head,
    pub stream_mutex: mutex,
    pub clk_mutex: mutex,
    pub dx_ctx: catpt_dx_context,
    pub dxbuf_vaddr: *mut c_void,
    pub dxbuf_paddr: dma_addr_t,
}

extern "C" {
    pub fn catpt_dmac_probe(cdev: *mut catpt_dev) -> c_int;
}
extern "C" {
    pub fn catpt_dmac_remove(cdev: *mut catpt_dev);
}
extern "C" {
    pub fn lpt_dsp_pll_shutdown(cdev: *mut catpt_dev, enable: bool);
}
extern "C" {
    pub fn wpt_dsp_pll_shutdown(cdev: *mut catpt_dev, enable: bool);
}
extern "C" {
    pub fn catpt_dsp_power_up(cdev: *mut catpt_dev) -> c_int;
}
extern "C" {
    pub fn catpt_dsp_power_down(cdev: *mut catpt_dev) -> c_int;
}
extern "C" {
    pub fn catpt_dsp_stall(cdev: *mut catpt_dev, stall: bool) -> c_int;
}
extern "C" {
    pub fn catpt_dsp_update_lpclock(cdev: *mut catpt_dev) -> c_int;
}
extern "C" {
    pub fn catpt_dsp_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn catpt_dsp_irq_thread(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
//
// IPC handlers may return positive values which denote successful
// HOST <-> DSP communication yet failure to process specific request.
// Use below macro to convert returned non-zero values appropriately
//

extern "C" {
    pub fn catpt_first_boot_firmware(cdev: *mut catpt_dev) -> c_int;
}
extern "C" {
    pub fn catpt_boot_firmware(cdev: *mut catpt_dev, restore: bool) -> c_int;
}
extern "C" {
    pub fn catpt_store_firmware_context(cdev: *mut catpt_dev) -> c_int;
}
extern "C" {
    pub fn catpt_coredump(cdev: *mut catpt_dev) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct catpt_stream_runtime {
    pub substream: *mut snd_pcm_substream,
    pub template: *mut catpt_stream_template,
    pub info: catpt_stream_info,
    pub persistent: *mut resource,
    pub pgtbl: snd_dma_buffer,
    pub allocated: bool,
    pub prepared: bool,
    pub node: list_head,
}

extern "C" {
    pub fn catpt_register_plat_component(cdev: *mut catpt_dev) -> c_int;
}
extern "C" {
    pub fn catpt_arm_stream_templates(cdev: *mut catpt_dev) -> c_int;
}
