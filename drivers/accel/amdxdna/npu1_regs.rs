//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/npu1_regs.c
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
// Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
//

// Address definition from NPU1 docs
pub const MPNPU_PWAITMODE: c_uint = 0x3010034;
pub const MPNPU_PUB_SEC_INTR: c_uint = 0x3010090;
pub const MPNPU_PUB_PWRMGMT_INTR: c_uint = 0x3010094;
pub const MPNPU_PUB_SCRATCH2: c_uint = 0x30100A0;
pub const MPNPU_PUB_SCRATCH3: c_uint = 0x30100A4;
pub const MPNPU_PUB_SCRATCH4: c_uint = 0x30100A8;
pub const MPNPU_PUB_SCRATCH5: c_uint = 0x30100AC;
pub const MPNPU_PUB_SCRATCH6: c_uint = 0x30100B0;
pub const MPNPU_PUB_SCRATCH7: c_uint = 0x30100B4;
pub const MPNPU_PUB_SCRATCH9: c_uint = 0x30100BC;
pub const MPNPU_SRAM_X2I_MAILBOX_0: c_uint = 0x30A0000;
pub const MPNPU_SRAM_X2I_MAILBOX_1: c_uint = 0x30A2000;
pub const MPNPU_SRAM_I2X_MAILBOX_15: c_uint = 0x30BF000;
pub const MPNPU_APERTURE0_BASE: c_uint = 0x3000000;
pub const MPNPU_APERTURE1_BASE: c_uint = 0x3080000;
pub const MPNPU_APERTURE2_BASE: c_uint = 0x30C0000;
// PCIe BAR Index for NPU1
pub const NPU1_REG_BAR_INDEX: c_int = 0;
pub const NPU1_MBOX_BAR_INDEX: c_int = 4;
pub const NPU1_PSP_BAR_INDEX: c_int = 0;
pub const NPU1_SMU_BAR_INDEX: c_int = 0;
pub const NPU1_SRAM_BAR_INDEX: c_int = 2;
// Associated BARs and Apertures

    const struct rt_config npu1_default_rt_cfg[] = {
    { 2, 1, AIE2_RT_CFG_INIT }, /* PDI APP LOAD MODE */
    { 4, 1, AIE2_RT_CFG_INIT }, /* Debug BO */
    { 1, 1, AIE2_RT_CFG_CLK_GATING }, /* Clock gating on */
    { 0 },
    };
    const struct dpm_clk_freq npu1_dpm_clk_table[] = {
    {400, 800},
    {600, 1024},
    {600, 1024},
    {600, 1024},
    {600, 1024},
    {720, 1309},
    {720, 1309},
    {847, 1600},
    { 0 }
    };
    static const struct amdxdna_fw_feature_tbl npu1_fw_feature_table[] = {
    { .major = 5, .min_minor = 7 },
    { .features = BIT_U64(AIE2_NPU_COMMAND), .major = 5, .min_minor = 8 },
    { 0 }
    };
#[no_mangle]
unsafe extern "C" fn npu1_set_dpm(ndev: *mut amdxdna_dev_hdl, dpm_level: u32) -> c_int {
    static int npu1_set_dpm(struct amdxdna_dev_hdl *ndev, u32 dpm_level)
    {
    u32 npuclk, hclk;
    int ret;
    npuclk = ndev.priv.dpm_clk_tbl[dpm_level].npuclk;
    hclk = ndev.priv.dpm_clk_tbl[dpm_level].hclk;
    ret = aie_smu_set_clocks(ndev.aie.smu_hdl, &npuclk, &hclk);
    if (ret)
    return ret;
    ndev.npuclk_freq = npuclk;
    ndev.hclk_freq = hclk;
    ndev.max_tops = 2 * ndev.total_col;
    ndev.curr_tops = ndev.max_tops * hclk / 1028;
    XDNA_DBG(ndev.aie.xdna, "MP-NPU clock %d, H clock %d\n",
    ndev.npuclk_freq, ndev.hclk_freq);
    return 0;
    }
    static const struct amdxdna_dev_priv npu1_dev_priv = {
    .fw_path        = "amdnpu/1502_00/",
    .rt_config	= npu1_default_rt_cfg,
    .dpm_clk_tbl	= npu1_dpm_clk_table,
    .col_align	= COL_ALIGN_NONE,
    .col_opc	= 2048,
    .mbox_dev_addr  = NPU1_MBOX_BAR_BASE,
    .mbox_size      = 0, /* Use BAR size */
    .sram_dev_addr  = NPU1_SRAM_BAR_BASE,
    .hwctx_limit    = 6,
    .sram_offs      = {
    DEFINE_BAR_OFFSET(MBOX_CHANN_OFF, NPU1_SRAM, MPNPU_SRAM_X2I_MAILBOX_0),
    DEFINE_BAR_OFFSET(FW_ALIVE_OFF,   NPU1_SRAM, MPNPU_SRAM_I2X_MAILBOX_15),
    },
    .psp_regs_off   = {
    DEFINE_BAR_OFFSET(PSP_CMD_REG,    NPU1_PSP, MPNPU_PUB_SCRATCH2),
    DEFINE_BAR_OFFSET(PSP_ARG0_REG,   NPU1_PSP, MPNPU_PUB_SCRATCH3),
    DEFINE_BAR_OFFSET(PSP_ARG1_REG,   NPU1_PSP, MPNPU_PUB_SCRATCH4),
    DEFINE_BAR_OFFSET(PSP_ARG2_REG,   NPU1_PSP, MPNPU_PUB_SCRATCH9),
    DEFINE_BAR_OFFSET(PSP_INTR_REG,   NPU1_PSP, MPNPU_PUB_SEC_INTR),
    DEFINE_BAR_OFFSET(PSP_STATUS_REG, NPU1_PSP, MPNPU_PUB_SCRATCH2),
    DEFINE_BAR_OFFSET(PSP_RESP_REG,   NPU1_PSP, MPNPU_PUB_SCRATCH3),
    DEFINE_BAR_OFFSET(PSP_PWAITMODE_REG, NPU1_PSP, MPNPU_PWAITMODE),
    },
    .smu_regs_off   = {
    DEFINE_BAR_OFFSET(SMU_CMD_REG,  NPU1_SMU, MPNPU_PUB_SCRATCH5),
    DEFINE_BAR_OFFSET(SMU_ARG_REG,  NPU1_SMU, MPNPU_PUB_SCRATCH7),
    DEFINE_BAR_OFFSET(SMU_INTR_REG, NPU1_SMU, MPNPU_PUB_PWRMGMT_INTR),
    DEFINE_BAR_OFFSET(SMU_RESP_REG, NPU1_SMU, MPNPU_PUB_SCRATCH6),
    DEFINE_BAR_OFFSET(SMU_OUT_REG,  NPU1_SMU, MPNPU_PUB_SCRATCH7),
    },
    .hw_ops		= &(const struct aie2_hw_ops) {
    .set_dpm = npu1_set_dpm,
    },
    };
    const struct amdxdna_dev_info dev_npu1_info = {
    .reg_bar           = NPU1_REG_BAR_INDEX,
    .mbox_bar          = NPU1_MBOX_BAR_INDEX,
    .sram_bar          = NPU1_SRAM_BAR_INDEX,
    .psp_bar           = NPU1_PSP_BAR_INDEX,
    .smu_bar           = NPU1_SMU_BAR_INDEX,
    .first_col         = 1,
    .dev_mem_buf_shift = 15, /* 32 KiB aligned */
    .dev_mem_base      = AIE2_DEVM_BASE,
    .dev_mem_size      = AIE2_DEVM_SIZE,
    .default_vbnv      = "RyzenAI-npu1",
    .dev_heap_max_size = AIE2_DEVM_SIZE,
    .device_type       = AMDXDNA_DEV_TYPE_KMQ,
    .dev_priv          = &npu1_dev_priv,
    .fw_feature_tbl    = npu1_fw_feature_table,
    .ops               = &aie2_ops,
    };
