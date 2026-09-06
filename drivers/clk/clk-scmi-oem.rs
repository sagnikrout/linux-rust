//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-scmi-oem.c
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
// The Vendor OEM extension for System Control and Power Interface (SCMI)
// Protocol based clock driver
//
// Copyright 2025 NXP
//

pub const SCMI_CLOCK_CFG_IMX_SSC: c_uint = 0x80;

//
// Selection is based on SCMI vendor_id/sub_vendor_id and optional machine
// compatible string, without involving impl_ver. impl_ver‑specific behavior
// should be considered a bug and handled via SCMI Quirk framework.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clk_oem_info {
    pub vendor_id: *mut c_char,
    pub sub_vendor_id: *mut c_char,
    pub compatible: *mut c_char,
    pub data: *const c_void,
}

    static int
    scmi_clk_imx_set_spread_spectrum(struct clk_hw *hw,
    const struct clk_spread_spectrum *ss_conf)
    {
    struct scmi_clk *clk = to_scmi_clk(hw);
    int ret;
    u32 val;
//
// extConfigValue[7:0]   - spread percentage (%)
// extConfigValue[23:8]  - Modulation Frequency
// extConfigValue[24]    - Enable/Disable
// extConfigValue[31:25] - Reserved
//
    val = FIELD_PREP(SCMI_CLOCK_IMX_SS_PERCENTAGE_MASK, ss_conf.spread_bp / 10000);
    val |= FIELD_PREP(SCMI_CLOCK_IMX_SS_MOD_FREQ_MASK, ss_conf.modfreq_hz);
    if (ss_conf.method != CLK_SPREAD_NO)
    val |= SCMI_CLOCK_IMX_SS_ENABLE_MASK;
    ret = scmi_proto_clk_ops.config_oem_set(clk.ph, clk.id,
    SCMI_CLOCK_CFG_IMX_SSC,
    val, false);
    if (ret)
    dev_warn(clk.dev,
    "Failed to set spread spectrum(%u,%u,%u) for clock ID %d\n",
    ss_conf.modfreq_hz, ss_conf.spread_bp, ss_conf.method,
    clk.id);
    return ret;
    }
    static int
    scmi_clk_imx_query_oem_feats(const struct scmi_protocol_handle *ph, u32 id,
    unsigned int *feats_key)
    {
    int ret;
    u32 val;
    ret = scmi_proto_clk_ops.config_oem_get(ph, id,
    SCMI_CLOCK_CFG_IMX_SSC,
    &val, core::ptr::null_mut(), false);
    if (!ret)
// feats_key |= BIT(SCMI_CLK_EXT_OEM_SSC_SUPPORTED);
    return 0;
    }
    static const struct scmi_clk_oem scmi_clk_oem_imx = {
    .query_ext_oem_feats = scmi_clk_imx_query_oem_feats,
    .set_spread_spectrum = scmi_clk_imx_set_spread_spectrum,
    };
    static const struct scmi_clk_oem_info info[] = {
    { SCMI_IMX_VENDOR, SCMI_IMX_SUBVENDOR, core::ptr::null_mut(), &scmi_clk_oem_imx },
    };
#[no_mangle]
pub unsafe extern "C" fn scmi_clk_oem_init(sdev: *mut scmi_device) -> c_int {
    int scmi_clk_oem_init(struct scmi_device *sdev)
    {
    const struct scmi_handle *handle = sdev.handle;
    int i, size = ARRAY_SIZE(info);
    for (i = 0; i < size; i++) {
    if (strcmp(handle.version.vendor_id, info[i].vendor_id) ||
    strcmp(handle.version.sub_vendor_id, info[i].sub_vendor_id))
    continue;
    if (info[i].compatible &&
    !of_machine_is_compatible(info[i].compatible))
    continue;
    break;
    }
    if (i < size)
    dev_set_drvdata(&sdev.dev, (void *)info[i].data);
    return 0;
    }
