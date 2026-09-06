//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_device_info.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// struct pvr_device_features - Hardware feature information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_device_features {
    pub has_axi_acelite: bool,
    pub has_cdm_control_stream_format: bool,
    pub has_cluster_grouping: bool,
    pub has_common_store_size_in_dwords: bool,
    pub has_compute: bool,
    pub has_compute_morton_capable: bool,
    pub has_compute_overlap: bool,
    pub has_coreid_per_os: bool,
    pub has_dynamic_dust_power: bool,
    pub has_ecc_rams: bool,
    pub has_fbcdc: bool,
    pub has_fbcdc_algorithm: bool,
    pub has_fbcdc_architecture: bool,
    pub has_fbc_max_default_descriptors: bool,
    pub has_fbc_max_large_descriptors: bool,
    pub has_fb_cdc_v4: bool,
    pub has_gpu_multicore_support: bool,
    pub has_gpu_virtualisation: bool,
    pub has_gs_rta_support: bool,
    pub has_irq_per_os: bool,
    pub has_isp_max_tiles_in_flight: bool,
    pub has_isp_samples_per_pixel: bool,
    pub has_isp_zls_d24_s8_packing_ogl_mode: bool,
    pub has_layout_mars: bool,
    pub has_max_partitions: bool,
    pub has_meta: bool,
    pub has_meta_coremem_size: bool,
    pub has_mips: bool,
    pub has_num_clusters: bool,
    pub has_num_isp_ipp_pipes: bool,
    pub has_num_osids: bool,
    pub has_num_raster_pipes: bool,
    pub has_pbe2_in_xe: bool,
    pub has_pbvnc_coreid_reg: bool,
    pub has_perfbus: bool,
    pub has_perf_counter_batch: bool,
    pub has_phys_bus_width: bool,
    pub has_riscv_fw_processor: bool,
    pub has_roguexe: bool,
    pub has_s7_top_infrastructure: bool,
    pub has_simple_internal_parameter_format: bool,
    pub has_simple_internal_parameter_format_v2: bool,
    pub has_simple_parameter_format_version: bool,
    pub has_slc_banks: bool,
    pub has_slc_cache_line_size_bits: bool,
    pub has_slc_size_configurable: bool,
    pub has_slc_size_in_kilobytes: bool,
    pub has_soc_timer: bool,
    pub has_sys_bus_secure_reset: bool,
    pub has_tessellation: bool,
    pub has_tile_region_protection: bool,
    pub has_tile_size_x: bool,
    pub has_tile_size_y: bool,
    pub has_tla: bool,
    pub has_tpu_cem_datamaster_global_registers: bool,
    pub has_tpu_dm_global_registers: bool,
    pub has_tpu_filtering_mode_control: bool,
    pub has_usc_min_output_registers_per_pix: bool,
    pub has_vdm_drawindirect: bool,
    pub has_vdm_object_level_lls: bool,
    pub has_virtual_address_space_bits: bool,
    pub has_watchdog_timer: bool,
    pub has_workgroup_protection: bool,
    pub has_xe_architecture: bool,
    pub has_xe_memory_hierarchy: bool,
    pub has_xe_tpu2: bool,
    pub has_xpu_max_regbanks_addr_width: bool,
    pub has_xpu_max_slaves: bool,
    pub has_xpu_register_broadcast: bool,
    pub has_xt_top_infrastructure: bool,
    pub has_zls_subtile: bool,
    pub cdm_control_stream_format: u64,
    pub common_store_size_in_dwords: u64,
    pub ecc_rams: u64,
    pub fbc_max_default_descriptors: u64,
    pub fbc_max_large_descriptors: u64,
    pub fbcdc: u64,
    pub fbcdc_algorithm: u64,
    pub fbcdc_architecture: u64,
    pub isp_max_tiles_in_flight: u64,
    pub isp_samples_per_pixel: u64,
    pub layout_mars: u64,
    pub max_partitions: u64,
    pub meta: u64,
    pub meta_coremem_size: u64,
    pub num_clusters: u64,
    pub num_isp_ipp_pipes: u64,
    pub num_osids: u64,
    pub num_raster_pipes: u64,
    pub phys_bus_width: u64,
    pub simple_parameter_format_version: u64,
    pub slc_banks: u64,
    pub slc_cache_line_size_bits: u64,
    pub slc_size_in_kilobytes: u64,
    pub tile_size_x: u64,
    pub tile_size_y: u64,
    pub usc_min_output_registers_per_pix: u64,
    pub virtual_address_space_bits: u64,
    pub xe_architecture: u64,
    pub xpu_max_regbanks_addr_width: u64,
    pub xpu_max_slaves: u64,
    pub xpu_register_broadcast: u64,
}

//
// struct pvr_device_quirks - Hardware quirk information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_device_quirks {
    pub has_brn44079: bool,
    pub has_brn47217: bool,
    pub has_brn48492: bool,
    pub has_brn48545: bool,
    pub has_brn49927: bool,
    pub has_brn50767: bool,
    pub has_brn51764: bool,
    pub has_brn62269: bool,
    pub has_brn63142: bool,
    pub has_brn63553: bool,
    pub has_brn66011: bool,
    pub has_brn71242: bool,
}

//
// struct pvr_device_enhancements - Hardware enhancement information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_device_enhancements {
    pub has_ern35421: bool,
    pub has_ern38020: bool,
    pub has_ern38748: bool,
    pub has_ern42064: bool,
    pub has_ern42290: bool,
    pub has_ern42606: bool,
    pub has_ern47025: bool,
    pub has_ern57596: bool,
}

//
// Meta cores
//
// These are the values for the 'meta' feature when the feature is present
// (as per &struct pvr_device_features)
//

