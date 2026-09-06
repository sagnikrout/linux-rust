//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/tegra/mc.h
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
// Copyright (C) 2014-2026 NVIDIA Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_timing {
    pub rate: c_ulong,
    pub emem_data: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_client {
    pub id: c_uint,
    pub bpmp_id: c_uint,
    pub type: tegra_icc_client_type,
    pub name: *const c_char,
//
// For Tegra210 and earlier, this is the SWGROUP ID used for IOVA translations in the
// Tegra SMMU, whereas on Tegra186 and later this is the ID used to override the ARM SMMU
// stream ID used for IOVA translations for the given memory client.
//
    pub swgroup: c_uint,
    pub sid: c_uint,
}

// Tegra SMMU enable (Tegra210 and earlier)
// latency allowance
// stream ID overrides (Tegra186 and later)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_smmu_swgroup {
    pub name: *const c_char,
    pub swgroup: c_uint,
    pub reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_smmu_group_soc {
    pub name: *const c_char,
    pub swgroups: *const c_uint,
    pub num_swgroups: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_smmu_soc {
    pub clients: *const tegra_mc_client,
    pub num_clients: c_uint,
    pub swgroups: *const tegra_smmu_swgroup,
    pub num_swgroups: c_uint,
    pub groups: *const tegra_smmu_group_soc,
    pub num_groups: c_uint,
    pub supports_round_robin_arbitration: bool,
    pub supports_request_limit: bool,
    pub num_tlb_lines: c_uint,
    pub num_asids: c_uint,
}

extern "C" {
    pub fn tegra_smmu_remove(smmu: *mut tegra_smmu);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_reset {
    pub name: *const c_char,
    pub id: c_ulong,
    pub control: c_uint,
    pub status: c_uint,
    pub reset: c_uint,
    pub bit: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_reset_ops {
    pub rst): *const tegra_mc_reset,
    pub rst): *const tegra_mc_reset,
    pub rst): *const tegra_mc_reset,
    pub rst): *const tegra_mc_reset,
    pub rst): *const tegra_mc_reset,
    pub rst): *const tegra_mc_reset,
}

pub const TEGRA_MC_ICC_TAG_DEFAULT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_icc_ops {
    pub dst): *mut *mut *mut int (set)(struct icc_node src, struct icc_node,
    pub agg_peak): *mut *mut u32 peak_bw, u32 agg_avg, u32,
    pub data): *const *const *const *const icc_node (xlate)(of_phandle_args spec, void,
    pub data): *mut c_void,
    pub peak): *mut *mut *mut *mut int (get_bw)(struct icc_node node, u32 avg, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_ops {
//
// @probe: Callback to set up SoC-specific bits of the memory controller. This is called
// after basic, common set up that is done by the SoC-agnostic bits.
//
    pub mc): *mut *mut int (probe)(struct tegra_mc,
    pub mc): *mut *mut void (remove)(struct tegra_mc,
    pub mc): *mut *mut void (resume)(struct tegra_mc,
    pub dev): *mut *mut *mut int (probe_device)(struct tegra_mc mc, struct device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_regs {
    pub cfg_channel_enable: c_uint,
    pub err_status: c_uint,
    pub err_add: c_uint,
    pub err_add_hi: c_uint,
    pub err_vpr_status: c_uint,
    pub err_vpr_add: c_uint,
    pub err_sec_status: c_uint,
    pub err_sec_add: c_uint,
    pub err_mts_status: c_uint,
    pub err_mts_add: c_uint,
    pub err_gen_co_status: c_uint,
    pub err_gen_co_add: c_uint,
    pub err_route_status: c_uint,
    pub err_route_add: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_intmask {
    pub reg: u32,
    pub mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc_soc {
    pub clients: *const tegra_mc_client,
    pub num_clients: c_uint,
    pub emem_regs: *const c_ulong,
    pub num_emem_regs: c_uint,
    pub num_address_bits: c_uint,
    pub atom_size: c_uint,
    pub num_carveouts: c_uint,
    pub client_id_mask: u16,
    pub num_channels: u8,
    pub smmu: *const tegra_smmu_soc,
    pub ch_intmask: u32,
    pub global_intstatus_channel_shift: u32,
    pub has_addr_hi_reg: bool,
    pub reset_ops: *const tegra_mc_reset_ops,
    pub resets: *const tegra_mc_reset,
    pub num_resets: c_uint,
    pub icc_ops: *const tegra_mc_icc_ops,
    pub ops: *const tegra_mc_ops,
    pub regs: *const tegra_mc_regs,
    pub handle_irq: *const irq_handler_t,
    pub num_interrupts: c_uint,
    pub mc_addr_hi_mask: c_uint,
    pub mc_err_status_type_mask: c_uint,
    pub intmasks: *const tegra_mc_intmask,
    pub num_intmasks: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_mc {
    pub bpmp: *mut tegra_bpmp,
    pub dev: *mut device,
    pub smmu: *mut tegra_smmu,
    pub regs: *mut void __iomem,
    pub bcast_ch_regs: *mut void __iomem,
    pub ch_regs: *mut void __iomem,
    pub clk: *mut clk,
    pub soc: *const tegra_mc_soc,
    pub tick: c_ulong,
    pub timings: *mut tegra_mc_timing,
    pub num_timings: c_uint,
    pub num_channels: c_uint,
    pub bwmgr_mrq_supported: bool,
    pub reset: reset_controller_dev,
    pub provider: icc_provider,
    pub lock: spinlock_t,
    pub root: *mut dentry,
    pub debugfs: },
}

extern "C" {
    pub fn tegra_mc_write_emem_configuration(mc: *mut tegra_mc, rate: c_ulong) -> c_int;
}
extern "C" {
    pub fn tegra_mc_get_emem_device_count(mc: *mut tegra_mc) -> c_uint;
}

extern "C" {
    pub fn tegra_mc_probe_device(mc: *mut tegra_mc, dev: *mut device) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

