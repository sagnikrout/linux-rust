//! Automatically rewritten from C to Rust
//! Source: drivers/accel/amdxdna/npu6_regs.c
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
// Copyright (C) 2024, Advanced Micro Devices, Inc.
//

// NPU Public Registers on MpNPUAxiXbar (refer to Diag npu_registers.h)
pub const MPNPU_PWAITMODE: c_uint = 0x301003C;
pub const MPNPU_PUB_SEC_INTR: c_uint = 0x3010060;
pub const MPNPU_PUB_PWRMGMT_INTR: c_uint = 0x3010064;
pub const MPNPU_PUB_SCRATCH0: c_uint = 0x301006C;
pub const MPNPU_PUB_SCRATCH1: c_uint = 0x3010070;
pub const MPNPU_PUB_SCRATCH2: c_uint = 0x3010074;
pub const MPNPU_PUB_SCRATCH3: c_uint = 0x3010078;
pub const MPNPU_PUB_SCRATCH4: c_uint = 0x301007C;
pub const MPNPU_PUB_SCRATCH5: c_uint = 0x3010080;
pub const MPNPU_PUB_SCRATCH6: c_uint = 0x3010084;
pub const MPNPU_PUB_SCRATCH7: c_uint = 0x3010088;
pub const MPNPU_PUB_SCRATCH8: c_uint = 0x301008C;
pub const MPNPU_PUB_SCRATCH9: c_uint = 0x3010090;
pub const MPNPU_PUB_SCRATCH10: c_uint = 0x3010094;
pub const MPNPU_PUB_SCRATCH11: c_uint = 0x3010098;
pub const MPNPU_PUB_SCRATCH12: c_uint = 0x301009C;
pub const MPNPU_PUB_SCRATCH13: c_uint = 0x30100A0;
pub const MPNPU_PUB_SCRATCH14: c_uint = 0x30100A4;
pub const MPNPU_PUB_SCRATCH15: c_uint = 0x30100A8;
pub const MP0_C2PMSG_73: c_uint = 0x3810A24;
pub const MP0_C2PMSG_123: c_uint = 0x3810AEC;
pub const MP1_C2PMSG_0: c_uint = 0x3B10900;
pub const MP1_C2PMSG_60: c_uint = 0x3B109F0;
pub const MP1_C2PMSG_61: c_uint = 0x3B109F4;
pub const MPNPU_SRAM_X2I_MAILBOX_0: c_uint = 0x3600000;
pub const MPNPU_SRAM_X2I_MAILBOX_15: c_uint = 0x361E000;
pub const MPNPU_SRAM_X2I_MAILBOX_31: c_uint = 0x363E000;
pub const MPNPU_SRAM_I2X_MAILBOX_31: c_uint = 0x363F000;
pub const MMNPU_APERTURE0_BASE: c_uint = 0x3000000;
pub const MMNPU_APERTURE1_BASE: c_uint = 0x3600000;
pub const MMNPU_APERTURE3_BASE: c_uint = 0x3810000;
pub const MMNPU_APERTURE4_BASE: c_uint = 0x3B10000;
// PCIe BAR Index for NPU6
pub const NPU6_REG_BAR_INDEX: c_int = 0;
pub const NPU6_MBOX_BAR_INDEX: c_int = 0;
pub const NPU6_PSP_BAR_INDEX: c_int = 4;
pub const NPU6_SMU_BAR_INDEX: c_int = 5;
pub const NPU6_SRAM_BAR_INDEX: c_int = 2;
// Associated BARs and Apertures

    static const struct amdxdna_dev_priv npu6_dev_priv = {
    .fw_path        = "amdnpu/17f0_10/",
    .rt_config	= npu4_default_rt_cfg,
    .dpm_clk_tbl	= npu4_dpm_clk_table,
    .col_align	= COL_ALIGN_NATURE,
    .col_opc	= 4096,
    .mbox_dev_addr  = NPU6_MBOX_BAR_BASE,
    .mbox_size      = 0, /* Use BAR size */
    .sram_dev_addr  = NPU6_SRAM_BAR_BASE,
    .hwctx_limit    = 16,
    .sram_offs      = {
    DEFINE_BAR_OFFSET(MBOX_CHANN_OFF, NPU6_SRAM, MPNPU_SRAM_X2I_MAILBOX_0),
    DEFINE_BAR_OFFSET(FW_ALIVE_OFF,   NPU6_SRAM, MPNPU_SRAM_X2I_MAILBOX_15),
    },
    .psp_regs_off   = {
    DEFINE_BAR_OFFSET(PSP_CMD_REG,    NPU6_PSP, MP0_C2PMSG_123),
    DEFINE_BAR_OFFSET(PSP_ARG0_REG,   NPU6_REG, MPNPU_PUB_SCRATCH3),
    DEFINE_BAR_OFFSET(PSP_ARG1_REG,   NPU6_REG, MPNPU_PUB_SCRATCH4),
    DEFINE_BAR_OFFSET(PSP_ARG2_REG,   NPU6_REG, MPNPU_PUB_SCRATCH9),
    DEFINE_BAR_OFFSET(PSP_INTR_REG,   NPU6_PSP, MP0_C2PMSG_73),
    DEFINE_BAR_OFFSET(PSP_STATUS_REG, NPU6_PSP, MP0_C2PMSG_123),
    DEFINE_BAR_OFFSET(PSP_RESP_REG,   NPU6_REG, MPNPU_PUB_SCRATCH3),
    DEFINE_BAR_OFFSET(PSP_PWAITMODE_REG, NPU6_REG, MPNPU_PWAITMODE),
    },
    .smu_regs_off   = {
    DEFINE_BAR_OFFSET(SMU_CMD_REG,  NPU6_SMU, MP1_C2PMSG_0),
    DEFINE_BAR_OFFSET(SMU_ARG_REG,  NPU6_SMU, MP1_C2PMSG_60),
    DEFINE_BAR_OFFSET(SMU_INTR_REG, NPU6_SMU, MMNPU_APERTURE4_BASE),
    DEFINE_BAR_OFFSET(SMU_RESP_REG, NPU6_SMU, MP1_C2PMSG_61),
    DEFINE_BAR_OFFSET(SMU_OUT_REG,  NPU6_SMU, MP1_C2PMSG_60),
    },
    .hw_ops         = &npu4_hw_ops
    };
    const struct amdxdna_dev_info dev_npu6_info = {
    .reg_bar           = NPU6_REG_BAR_INDEX,
    .mbox_bar          = NPU6_MBOX_BAR_INDEX,
    .sram_bar          = NPU6_SRAM_BAR_INDEX,
    .psp_bar           = NPU6_PSP_BAR_INDEX,
    .smu_bar           = NPU6_SMU_BAR_INDEX,
    .first_col         = 0,
    .dev_mem_buf_shift = 15, /* 32 KiB aligned */
    .dev_mem_base      = AIE2_DEVM_BASE,
    .dev_mem_size      = AIE2_DEVM_SIZE,
    .default_vbnv      = "RyzenAI-npu6",
    .dev_heap_max_size = AIE2_DEVM_MAX_SIZE,
    .device_type       = AMDXDNA_DEV_TYPE_KMQ,
    .rev_vbnv_tbl      = npu4_rev_vbnv_tbl,
    .dev_priv          = &npu6_dev_priv,
    .fw_feature_tbl    = npu4_fw_feature_table,
    .ops               = &aie2_ops,
    };
