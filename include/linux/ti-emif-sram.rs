//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ti-emif-sram.h
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
// TI AM33XX EMIF Routines
//
// Copyright (C) 2016-2017 Texas Instruments Inc.
// Dave Gerlach
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emif_regs_amx3 {
    pub emif_sdcfg_val: u32,
    pub emif_timing1_val: u32,
    pub emif_timing2_val: u32,
    pub emif_timing3_val: u32,
    pub emif_ref_ctrl_val: u32,
    pub emif_zqcfg_val: u32,
    pub emif_pmcr_val: u32,
    pub emif_pmcr_shdw_val: u32,
    pub emif_rd_wr_level_ramp_ctrl: u32,
    pub emif_rd_wr_exec_thresh: u32,
    pub emif_cos_config: u32,
    pub emif_priority_to_cos_mapping: u32,
    pub emif_connect_id_serv_1_map: u32,
    pub emif_connect_id_serv_2_map: u32,
    pub emif_ocp_config_val: u32,
    pub emif_lpddr2_nvm_tim: u32,
    pub emif_lpddr2_nvm_tim_shdw: u32,
    pub emif_dll_calib_ctrl_val: u32,
    pub emif_dll_calib_ctrl_val_shdw: u32,
    pub emif_ddr_phy_ctlr_1: u32,
    pub emif_ext_phy_ctrl_vals: [u32; 120],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_emif_pm_data {
    pub ti_emif_base_addr_virt: *mut void __iomem,
    pub ti_emif_base_addr_phys: phys_addr_t,
    pub ti_emif_sram_config: c_ulong,
    pub regs_virt: *mut emif_regs_amx3,
    pub regs_phys: phys_addr_t,
    pub __aligned(8): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_emif_pm_functions {
    pub save_context: u32,
    pub restore_context: u32,
    pub run_hw_leveling: u32,
    pub enter_sr: u32,
    pub exit_sr: u32,
    pub abort_sr: u32,
    pub __aligned(8): } __packed,
    pub emif_sdcfg_val)): offsetof(struct emif_regs_amx3,,
    pub emif_timing1_val)): offsetof(struct emif_regs_amx3,,
    pub emif_timing2_val)): offsetof(struct emif_regs_amx3,,
    pub emif_timing3_val)): offsetof(struct emif_regs_amx3,,
    pub emif_ref_ctrl_val)): offsetof(struct emif_regs_amx3,,
    pub emif_zqcfg_val)): offsetof(struct emif_regs_amx3,,
    pub emif_pmcr_val)): offsetof(struct emif_regs_amx3,,
    pub emif_pmcr_shdw_val)): offsetof(struct emif_regs_amx3,,
    pub emif_rd_wr_level_ramp_ctrl)): offsetof(struct emif_regs_amx3,,
    pub emif_rd_wr_exec_thresh)): offsetof(struct emif_regs_amx3,,
    pub emif_cos_config)): offsetof(struct emif_regs_amx3,,
    pub emif_priority_to_cos_mapping)): offsetof(struct emif_regs_amx3,,
    pub emif_connect_id_serv_1_map)): offsetof(struct emif_regs_amx3,,
    pub emif_connect_id_serv_2_map)): offsetof(struct emif_regs_amx3,,
    pub emif_ocp_config_val)): offsetof(struct emif_regs_amx3,,
    pub emif_lpddr2_nvm_tim)): offsetof(struct emif_regs_amx3,,
    pub emif_lpddr2_nvm_tim_shdw)): offsetof(struct emif_regs_amx3,,
    pub emif_dll_calib_ctrl_val)): offsetof(struct emif_regs_amx3,,
    pub emif_dll_calib_ctrl_val_shdw)): offsetof(struct emif_regs_amx3,,
    pub emif_ddr_phy_ctlr_1)): offsetof(struct emif_regs_amx3,,
    pub emif_ext_phy_ctrl_vals)): offsetof(struct emif_regs_amx3,,
    pub emif_regs_amx3)): DEFINE(EMIF_REGS_AMX3_SIZE, sizeof(struct,
    pub ti_emif_base_addr_virt)): offsetof(struct ti_emif_pm_data,,
    pub ti_emif_base_addr_phys)): offsetof(struct ti_emif_pm_data,,
    pub ti_emif_sram_config)): offsetof(struct ti_emif_pm_data,,
    pub regs_virt)): offsetof(struct ti_emif_pm_data,,
    pub regs_phys)): offsetof(struct ti_emif_pm_data,,
    pub ti_emif_pm_data)): DEFINE(EMIF_PM_DATA_SIZE, sizeof(struct,
    pub save_context)): offsetof(struct ti_emif_pm_functions,,
    pub restore_context)): offsetof(struct ti_emif_pm_functions,,
    pub run_hw_leveling)): offsetof(struct ti_emif_pm_functions,,
    pub enter_sr)): offsetof(struct ti_emif_pm_functions,,
    pub exit_sr)): offsetof(struct ti_emif_pm_functions,,
    pub abort_sr)): offsetof(struct ti_emif_pm_functions,,
    pub ti_emif_pm_functions)): DEFINE(EMIF_PM_FUNCTIONS_SIZE, sizeof(struct,
    pub gen_pool: struct,
    pub dst): *mut *mut int ti_emif_copy_pm_function_table(struct gen_pool sram_pool, void,
    pub ti_emif_get_mem_type(void): c_int,

