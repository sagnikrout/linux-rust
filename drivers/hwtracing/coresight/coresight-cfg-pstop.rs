//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-cfg-pstop.c
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
// Copyright(C) 2023  Marvell.
// Based on coresight-cfg-afdo.c
//

// ETMv4 includes and features

// preload configurations and features
// preload in features for ETMv4
// panic_stop feature
    static struct cscfg_parameter_desc gen_etrig_params[] = {
    {
    .name = "address",
    .value = (u64)panic,
    },
    };
    static struct cscfg_regval_desc gen_etrig_regs[] = {
// resource selector
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCRSCTLRn(2),
    .hw_info = ETM4_CFG_RES_SEL,
    .val32 = 0x40001,
    },
// single address comparator
    {
    .type = CS_CFG_REG_TYPE_RESOURCE | CS_CFG_REG_TYPE_VAL_64BIT |
    CS_CFG_REG_TYPE_VAL_PARAM,
    .offset =  TRCACVRn(0),
    .val32 = 0x0,
    },
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCACATRn(0),
    .val64 = 0xf00,
    },
// Driver external output[0] with comparator out
    {
    .type = CS_CFG_REG_TYPE_RESOURCE,
    .offset = TRCEVENTCTL0R,
    .val32 = 0x2,
    },
// end of regs
    };
    struct cscfg_feature_desc gen_etrig_etm4x = {
    .name = "gen_etrig",
    .description = "Generate external trigger on address match\n"
    "parameter \'address\': address of kernel address\n",
    .match_flags = CS_CFG_MATCH_CLASS_SRC_ETM4,
    .nr_params = ARRAY_SIZE(gen_etrig_params),
    .params_desc = gen_etrig_params,
    .nr_regs = ARRAY_SIZE(gen_etrig_regs),
    .regs_desc = gen_etrig_regs,
    };
// create a panic stop configuration
// the total number of parameters in used features

    static const char *pstop_ref_names[] = {
    "gen_etrig",
    };
    struct cscfg_config_desc pstop_etm4x = {
    .name = "panicstop",
    .description = "Stop ETM on kernel panic\n",
    .nr_feat_refs = ARRAY_SIZE(pstop_ref_names),
    .feat_ref_names = pstop_ref_names,
    .nr_total_params = PSTOP_NR_PARAMS,
    };
// end of ETM4x configurations
