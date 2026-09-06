//! Automatically rewritten from C to Rust
//! Source: arch/x86/virt/vmx/tdx/tdx_global_metadata.c
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
// Automatically generated functions to read TDX global metadata.
//
// This file doesn't compile on its own as it lacks of inclusion
// of SEAMCALL wrapper primitive which reads global metadata.
// Include this file to other C file instead.
//
#[no_mangle]
unsafe extern "C" fn get_tdx_sys_info_version(sysinfo_version: *mut tdx_sys_info_version) -> c_int {
    static int get_tdx_sys_info_version(struct tdx_sys_info_version *sysinfo_version)
    {
    let mut ret: c_int = 0;
    u64 val;
    if (!ret && !(ret = read_sys_metadata_field(0x0800000100000003, &val)))
    sysinfo_version.minor_version = val;
    if (!ret && !(ret = read_sys_metadata_field(0x0800000100000004, &val)))
    sysinfo_version.major_version = val;
    if (!ret && !(ret = read_sys_metadata_field(0x0800000100000005, &val)))
    sysinfo_version.update_version = val;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_tdx_sys_info_features(sysinfo_features: *mut tdx_sys_info_features) -> __init int {
    static __init int get_tdx_sys_info_features(struct tdx_sys_info_features *sysinfo_features)
    {
    let mut ret: c_int = 0;
    u64 val;
    if (!ret && !(ret = read_sys_metadata_field(0x0A00000300000008, &val)))
    sysinfo_features.tdx_features0 = val;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_tdx_sys_info_tdmr(sysinfo_tdmr: *mut tdx_sys_info_tdmr) -> __init int {
    static __init int get_tdx_sys_info_tdmr(struct tdx_sys_info_tdmr *sysinfo_tdmr)
    {
    let mut ret: c_int = 0;
    u64 val;
    if (!ret && !(ret = read_sys_metadata_field(0x9100000100000008, &val)))
    sysinfo_tdmr.max_tdmrs = val;
    if (!ret && !(ret = read_sys_metadata_field(0x9100000100000009, &val)))
    sysinfo_tdmr.max_reserved_per_tdmr = val;
    if (!ret && !(ret = read_sys_metadata_field(0x9100000100000010, &val)))
    sysinfo_tdmr.pamt_4k_entry_size = val;
    if (!ret && !(ret = read_sys_metadata_field(0x9100000100000011, &val)))
    sysinfo_tdmr.pamt_2m_entry_size = val;
    if (!ret && !(ret = read_sys_metadata_field(0x9100000100000012, &val)))
    sysinfo_tdmr.pamt_1g_entry_size = val;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_tdx_sys_info_td_ctrl(sysinfo_td_ctrl: *mut tdx_sys_info_td_ctrl) -> __init int {
    static __init int get_tdx_sys_info_td_ctrl(struct tdx_sys_info_td_ctrl *sysinfo_td_ctrl)
    {
    let mut ret: c_int = 0;
    u64 val;
    if (!ret && !(ret = read_sys_metadata_field(0x9800000100000000, &val)))
    sysinfo_td_ctrl.tdr_base_size = val;
    if (!ret && !(ret = read_sys_metadata_field(0x9800000100000100, &val)))
    sysinfo_td_ctrl.tdcs_base_size = val;
    if (!ret && !(ret = read_sys_metadata_field(0x9800000100000200, &val)))
    sysinfo_td_ctrl.tdvps_base_size = val;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_tdx_sys_info_td_conf(sysinfo_td_conf: *mut tdx_sys_info_td_conf) -> __init int {
    static __init int get_tdx_sys_info_td_conf(struct tdx_sys_info_td_conf *sysinfo_td_conf)
    {
    let mut ret: c_int = 0;
    u64 val;
    int i, j;
    if (!ret && !(ret = read_sys_metadata_field(0x1900000300000000, &val)))
    sysinfo_td_conf.attributes_fixed0 = val;
    if (!ret && !(ret = read_sys_metadata_field(0x1900000300000001, &val)))
    sysinfo_td_conf.attributes_fixed1 = val;
    if (!ret && !(ret = read_sys_metadata_field(0x1900000300000002, &val)))
    sysinfo_td_conf.xfam_fixed0 = val;
    if (!ret && !(ret = read_sys_metadata_field(0x1900000300000003, &val)))
    sysinfo_td_conf.xfam_fixed1 = val;
    if (!ret && !(ret = read_sys_metadata_field(0x9900000100000004, &val)))
    sysinfo_td_conf.num_cpuid_config = val;
    if (!ret && !(ret = read_sys_metadata_field(0x9900000100000008, &val)))
    sysinfo_td_conf.max_vcpus_per_td = val;
    if (sysinfo_td_conf.num_cpuid_config > ARRAY_SIZE(sysinfo_td_conf.cpuid_config_leaves))
    return -EINVAL;
    for (i = 0; i < sysinfo_td_conf.num_cpuid_config; i++)
    if (!ret && !(ret = read_sys_metadata_field(0x9900000300000400 + i, &val)))
    sysinfo_td_conf.cpuid_config_leaves[i] = val;
    if (sysinfo_td_conf.num_cpuid_config > ARRAY_SIZE(sysinfo_td_conf.cpuid_config_values))
    return -EINVAL;
    for (i = 0; i < sysinfo_td_conf.num_cpuid_config; i++)
    for (j = 0; j < 2; j++)
    if (!ret && !(ret = read_sys_metadata_field(0x9900000300000500 + i * 2 + j, &val)))
    sysinfo_td_conf.cpuid_config_values[i][j] = val;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn get_tdx_sys_info_handoff(sysinfo_handoff: *mut tdx_sys_info_handoff) -> c_int {
    static int get_tdx_sys_info_handoff(struct tdx_sys_info_handoff *sysinfo_handoff)
    {
    int ret;
    u64 val;
    ret = read_sys_metadata_field(0x8900000100000000, &val);
    if (ret)
    return ret;
    sysinfo_handoff.module_hv = val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_tdx_sys_info(sysinfo: *mut tdx_sys_info) -> __init int {
    static __init int get_tdx_sys_info(struct tdx_sys_info *sysinfo)
    {
    let mut ret: c_int = 0;
    ret = ret ?: get_tdx_sys_info_version(&sysinfo.version);
    pr_info("Module version: " TDX_VERSION_FMT "\n",
    sysinfo.version.major_version,
    sysinfo.version.minor_version,
    sysinfo.version.update_version);
    ret = ret ?: get_tdx_sys_info_features(&sysinfo.features);
    ret = ret ?: get_tdx_sys_info_tdmr(&sysinfo.tdmr);
    ret = ret ?: get_tdx_sys_info_td_ctrl(&sysinfo.td_ctrl);
    ret = ret ?: get_tdx_sys_info_td_conf(&sysinfo.td_conf);
    return ret;
    }
