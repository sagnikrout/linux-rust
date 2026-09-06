//! Automatically rewritten from C to Rust
//! Source: samples/coresight/coresight-cfg-sample.c
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

// create an alternate autofdo configuration
// we will provide 4 sets of preset parameter values
pub const AFDO2_NR_PRESETS: c_int = 4;
// the total number of parameters in used features - strobing has 2
pub const AFDO2_NR_PARAM_SUM: c_int = 2;
    static const char *afdo2_ref_names[] = {
    "strobing",
    };
//
// set of presets leaves strobing window constant while varying period to allow
// experimentation with mark / space ratios for various workloads
//
    static u64 afdo2_presets[AFDO2_NR_PRESETS][AFDO2_NR_PARAM_SUM] = {
    { 1000, 100 },
    { 1000, 1000 },
    { 1000, 5000 },
    { 1000, 10000 },
    };
    struct cscfg_config_desc afdo2 = {
    .name = "autofdo2",
    .description = "Setup ETMs with strobing for autofdo\n"
    "Supplied presets allow experimentation with mark-space ratio for various loads\n",
    .nr_feat_refs = ARRAY_SIZE(afdo2_ref_names),
    .feat_ref_names = afdo2_ref_names,
    .nr_presets = AFDO2_NR_PRESETS,
    .nr_total_params = AFDO2_NR_PARAM_SUM,
    .presets = &afdo2_presets[0][0],
    };
    static struct cscfg_feature_desc *sample_feats[] = {
    core::ptr::null_mut()
    };
    static struct cscfg_config_desc *sample_cfgs[] = {
    &afdo2,
    core::ptr::null_mut()
    };
    static struct cscfg_load_owner_info mod_owner = {
    .type = CSCFG_OWNER_MODULE,
    .owner_handle = THIS_MODULE,
    };
// module init and exit - just load and unload configs
#[no_mangle]
unsafe extern "C" fn cscfg_sample_init() -> int __init {
    static int __init cscfg_sample_init(void)
    {
    return cscfg_load_config_sets(sample_cfgs, sample_feats, &mod_owner);
    }
#[no_mangle]
unsafe extern "C" fn cscfg_sample_exit() -> void __exit {
    static void __exit cscfg_sample_exit(void)
    {
    cscfg_unload_config_sets(&mod_owner);
    }
    module_init(cscfg_sample_init);
    module_exit(cscfg_sample_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Mike Leach <mike.leach@linaro.org>");
    MODULE_DESCRIPTION("CoreSight Syscfg Example");
