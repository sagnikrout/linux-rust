//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-cfg-afdo.c
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
// Copyright(C) 2020 Linaro Limited. All rights reserved.
// Author: Mike Leach <mike.leach@linaro.org>
//

// ETMv4 includes and features

// preload configurations and features
// preload in features for ETMv4
// strobe feature
    static struct cscfg_parameter_desc strobe_params[] = {
    {
    .name = "window",
    .value = 5000,
    },
    {
    .name = "period",
    .value = 10000,
    },
    };
    static struct cscfg_regval_desc strobe_regs[] = {
// resource selectors
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCRSCTLRn(2),
    .hw_info = ETM4_CFG_RES_SEL,
    .val32 = 0x20001,
    },
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCRSCTLRn(3),
    .hw_info = ETM4_CFG_RES_SEQ,
    .val32 = 0x20002,
    },
// strobe window counter 0 - reload from param 0
    {
    .type = CS_CFG_REG_TYPE_RESOURCE | CS_CFG_REG_TYPE_VAL_SAVE,
    .offset = TRCCNTVRn(0),
    .hw_info = ETM4_CFG_RES_CTR,
    },
    {
    .type = CS_CFG_REG_TYPE_RESOURCE | CS_CFG_REG_TYPE_VAL_PARAM,
    .offset = TRCCNTRLDVRn(0),
    .hw_info = ETM4_CFG_RES_CTR,
    .val32 = 0,
    },
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCCNTCTLRn(0),
    .hw_info = ETM4_CFG_RES_CTR,
    .val32 = 0x10001,
    },
// strobe period counter 1 - reload from param 1
    {
    .type = CS_CFG_REG_TYPE_RESOURCE | CS_CFG_REG_TYPE_VAL_SAVE,
    .offset = TRCCNTVRn(1),
    .hw_info = ETM4_CFG_RES_CTR,
    },
    {
    .type = CS_CFG_REG_TYPE_RESOURCE | CS_CFG_REG_TYPE_VAL_PARAM,
    .offset = TRCCNTRLDVRn(1),
    .hw_info = ETM4_CFG_RES_CTR,
    .val32 = 1,
    },
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCCNTCTLRn(1),
    .hw_info = ETM4_CFG_RES_CTR,
    .val32 = 0x8102,
    },
// sequencer
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCSEQEVRn(0),
    .hw_info = ETM4_CFG_RES_SEQ,
    .val32 = 0x0081,
    },
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCSEQEVRn(1),
    .hw_info = ETM4_CFG_RES_SEQ,
    .val32 = 0x0000,
    },
// view-inst
    {
    .type = CS_CFG_REG_TYPE_STD | CS_CFG_REG_TYPE_VAL_MASK,
    .offset = TRCVICTLR,
    .val32 = 0x0003,
    .mask32 = 0x0003,
    },
// end of regs
    };
    struct cscfg_feature_desc strobe_etm4x = {
    .name = "strobing",
    .description = "Generate periodic trace capture windows.\n"
    "parameter \'window\': a number of CPU cycles (W)\n"
    "parameter \'period\': trace enabled for W cycles every period x W cycles\n",
    .match_flags = CS_CFG_MATCH_CLASS_SRC_ETM4,
    .nr_params = ARRAY_SIZE(strobe_params),
    .params_desc = strobe_params,
    .nr_regs = ARRAY_SIZE(strobe_regs),
    .regs_desc = strobe_regs,
    };
// create an autofdo configuration
// we will provide 9 sets of preset parameter values
pub const AFDO_NR_PRESETS: c_int = 9;
// the total number of parameters in used features

    static const char *afdo_ref_names[] = {
    "strobing",
    };
//
// set of presets leaves strobing window constant while varying period to allow
// experimentation with mark / space ratios for various workloads
//
    static u64 afdo_presets[AFDO_NR_PRESETS][AFDO_NR_PARAMS] = {
    { 5000, 2 },
    { 5000, 4 },
    { 5000, 8 },
    { 5000, 16 },
    { 5000, 64 },
    { 5000, 128 },
    { 5000, 512 },
    { 5000, 1024 },
    { 5000, 4096 },
    };
    struct cscfg_config_desc afdo_etm4x = {
    .name = "autofdo",
    .description = "Setup ETMs with strobing for autofdo\n"
    "Supplied presets allow experimentation with mark-space ratio for various loads\n",
    .nr_feat_refs = ARRAY_SIZE(afdo_ref_names),
    .feat_ref_names = afdo_ref_names,
    .nr_presets = AFDO_NR_PRESETS,
    .nr_total_params = AFDO_NR_PARAMS,
    .presets = &afdo_presets[0][0],
    };
// end of ETM4x configurations
