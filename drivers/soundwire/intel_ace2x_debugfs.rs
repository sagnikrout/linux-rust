//! Automatically rewritten from C to Rust
//! Source: drivers/soundwire/intel_ace2x_debugfs.c
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
// Copyright(c) 2023 Intel Corporation

//
// debugfs
//

    static ssize_t intel_sprintf(void __iomem *mem, bool l,
    char *buf, size_t pos, unsigned int reg)
    {
    int value;
    if (l)
    value = intel_readl(mem, reg);
    else
    value = intel_readw(mem, reg);
    return scnprintf(buf + pos, RD_BUF - pos, "%4x\t%4x\n", reg, value);
    }
#[no_mangle]
unsafe extern "C" fn intel_reg_show(s_file: *mut seq_file, data: *mut c_void) -> c_int {
    static int intel_reg_show(struct seq_file *s_file, void *data)
    {
    struct sdw_intel *sdw = s_file.private;
    void __iomem *s = sdw.link_res.shim;
    void __iomem *vs_s = sdw.link_res.shim_vs;
    ssize_t ret;
    u32 pcm_cap;
    int pcm_bd;
    char *buf;
    int j;
    buf = kzalloc(RD_BUF, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    ret = scnprintf(buf, RD_BUF, "Register  Value\n");
    ret += scnprintf(buf + ret, RD_BUF - ret, "\nShim\n");
    ret += intel_sprintf(s, true, buf, ret, SDW_SHIM2_LECAP);
    ret += intel_sprintf(s, false, buf, ret, SDW_SHIM2_PCMSCAP);
    pcm_cap = intel_readw(s, SDW_SHIM2_PCMSCAP);
    pcm_bd = FIELD_GET(SDW_SHIM2_PCMSCAP_BSS, pcm_cap);
    for (j = 0; j < pcm_bd; j++) {
    ret += intel_sprintf(s, false, buf, ret,
    SDW_SHIM2_PCMSYCHM(j));
    ret += intel_sprintf(s, false, buf, ret,
    SDW_SHIM2_PCMSYCHC(j));
    }
    ret += scnprintf(buf + ret, RD_BUF - ret, "\nVS CLK controls\n");
    ret += intel_sprintf(vs_s, true, buf, ret, SDW_SHIM2_INTEL_VS_LVSCTL);
    ret += scnprintf(buf + ret, RD_BUF - ret, "\nVS Wake registers\n");
    ret += intel_sprintf(vs_s, false, buf, ret, SDW_SHIM2_INTEL_VS_WAKEEN);
    ret += intel_sprintf(vs_s, false, buf, ret, SDW_SHIM2_INTEL_VS_WAKESTS);
    ret += scnprintf(buf + ret, RD_BUF - ret, "\nVS IOCTL, ACTMCTL\n");
    ret += intel_sprintf(vs_s, false, buf, ret, SDW_SHIM2_INTEL_VS_IOCTL);
    ret += intel_sprintf(vs_s, false, buf, ret, SDW_SHIM2_INTEL_VS_ACTMCTL);
    if (sdw.link_res.mic_privacy) {
    ret += scnprintf(buf + ret, RD_BUF - ret, "\nVS PVCCS\n");
    ret += intel_sprintf(vs_s, false, buf, ret,
    SDW_SHIM2_INTEL_VS_PVCCS);
    }
    seq_printf(s_file, "%s", buf);
    kfree(buf);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(intel_reg);
#[no_mangle]
unsafe extern "C" fn intel_set_m_datamode(data: *mut c_void, value: u64) -> c_int {
    static int intel_set_m_datamode(void *data, u64 value)
    {
    struct sdw_intel *sdw = data;
    struct sdw_bus *bus = &sdw.cdns.bus;
    if (value > SDW_PORT_DATA_MODE_STATIC_1)
    return -EINVAL;
// Userspace changed the hardware state behind the kernel's back
    add_taint(TAINT_USER, LOCKDEP_STILL_OK);
    bus.params.m_data_mode = value;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(intel_set_m_datamode_fops, core::ptr::null_mut(),
    intel_set_m_datamode, "%llu\n");
#[no_mangle]
unsafe extern "C" fn intel_set_s_datamode(data: *mut c_void, value: u64) -> c_int {
    static int intel_set_s_datamode(void *data, u64 value)
    {
    struct sdw_intel *sdw = data;
    struct sdw_bus *bus = &sdw.cdns.bus;
    if (value > SDW_PORT_DATA_MODE_STATIC_1)
    return -EINVAL;
// Userspace changed the hardware state behind the kernel's back
    add_taint(TAINT_USER, LOCKDEP_STILL_OK);
    bus.params.s_data_mode = value;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(intel_set_s_datamode_fops, core::ptr::null_mut(),
    intel_set_s_datamode, "%llu\n");
#[no_mangle]
pub unsafe extern "C" fn intel_ace2x_debugfs_init(sdw: *mut sdw_intel) {
    void intel_ace2x_debugfs_init(struct sdw_intel *sdw)
    {
    struct dentry *root = sdw.cdns.bus.debugfs;
    if (!root)
    return;
    sdw.debugfs = debugfs_create_dir("intel-sdw", root);
    debugfs_create_file("intel-registers", 0400, sdw.debugfs, sdw,
    &intel_reg_fops);
    debugfs_create_file("intel-m-datamode", 0200, sdw.debugfs, sdw,
    &intel_set_m_datamode_fops);
    debugfs_create_file("intel-s-datamode", 0200, sdw.debugfs, sdw,
    &intel_set_s_datamode_fops);
    sdw_cdns_debugfs_init(&sdw.cdns, sdw.debugfs);
    }
#[no_mangle]
pub unsafe extern "C" fn intel_ace2x_debugfs_exit(sdw: *mut sdw_intel) {
    void intel_ace2x_debugfs_exit(struct sdw_intel *sdw)
    {
    debugfs_remove_recursive(sdw.debugfs);
    }
