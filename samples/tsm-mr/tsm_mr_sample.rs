//! Automatically rewritten from C to Rust
//! Source: samples/tsm-mr/tsm_mr_sample.c
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
// Copyright(c) 2024-2005 Intel Corporation. All rights reserved.

    static struct {
    u8 static_mr[SHA384_DIGEST_SIZE];
    u8 config_mr[SHA512_DIGEST_SIZE];
    u8 rtmr0[SHA256_DIGEST_SIZE];
    u8 rtmr1[SHA384_DIGEST_SIZE];
    u8 report_digest[SHA512_DIGEST_SIZE];
    } sample_report = {
    .static_mr = "static_mr",
    .config_mr = "config_mr",
    .rtmr0 = "rtmr0",
    .rtmr1 = "rtmr1",
    };
#[no_mangle]
unsafe extern "C" fn sample_report_refresh(tm: *const tsm_measurements) -> c_int {
    static int sample_report_refresh(const struct tsm_measurements *tm)
    {
    sha512((const u8 *)&sample_report,
    offsetof(typeof(sample_report), report_digest),
    sample_report.report_digest);
    return 0;
    }
    static int sample_report_extend_mr(const struct tsm_measurements *tm,
    const struct tsm_measurement_register *mr,
    const u8 *data)
    {
    union {
    struct sha256_ctx sha256;
    struct sha384_ctx sha384;
    struct sha512_ctx sha512;
    } ctx;
    switch (mr.mr_hash) {
    case HASH_ALGO_SHA256:
    sha256_init(&ctx.sha256);
    sha256_update(&ctx.sha256, mr.mr_value, mr.mr_size);
    sha256_update(&ctx.sha256, data, mr.mr_size);
    sha256_final(&ctx.sha256, mr.mr_value);
    return 0;
    case HASH_ALGO_SHA384:
    sha384_init(&ctx.sha384);
    sha384_update(&ctx.sha384, mr.mr_value, mr.mr_size);
    sha384_update(&ctx.sha384, data, mr.mr_size);
    sha384_final(&ctx.sha384, mr.mr_value);
    return 0;
    case HASH_ALGO_SHA512:
    sha512_init(&ctx.sha512);
    sha512_update(&ctx.sha512, mr.mr_value, mr.mr_size);
    sha512_update(&ctx.sha512, data, mr.mr_size);
    sha512_final(&ctx.sha512, mr.mr_value);
    return 0;
    default:
    pr_err("Unsupported hash algorithm: %d\n", mr.mr_hash);
    return -EOPNOTSUPP;
    }
    }

    static const struct tsm_measurement_register sample_mrs[] = {
// static MR, read-only
    { MR_(static_mr, SHA384) },
// config MR, read-only
    { MR_(config_mr, SHA512) | TSM_MR_F_NOHASH },
// RTMR, direct extension prohibited
    { MR_(rtmr0, SHA256) | TSM_MR_F_LIVE },
// RTMR, direct extension allowed
    { MR_(rtmr1, SHA384) | TSM_MR_F_RTMR },
// RTMR, crypto agile, alaised to rtmr0 and rtmr1, respectively
    { .mr_value = &sample_report.rtmr0,
    TSM_MR_(rtmr_crypto_agile, SHA256) | TSM_MR_F_RTMR },
    { .mr_value = &sample_report.rtmr1,
    TSM_MR_(rtmr_crypto_agile, SHA384) | TSM_MR_F_RTMR },
// sha512 digest of the whole structure
    { MR_(report_digest, SHA512) | TSM_MR_F_LIVE },
    };

    static struct tsm_measurements sample_tm = {
    .mrs = sample_mrs,
    .nr_mrs = ARRAY_SIZE(sample_mrs),
    .refresh = sample_report_refresh,
    .write = sample_report_extend_mr,
    };
    static const struct attribute_group *sample_groups[] = {
    core::ptr::null_mut(),
    core::ptr::null_mut(),
    };
    static struct miscdevice sample_misc_dev = {
    .name = KBUILD_MODNAME,
    .minor = MISC_DYNAMIC_MINOR,
    .groups = sample_groups,
    };
#[no_mangle]
unsafe extern "C" fn tsm_mr_sample_init() -> int __init {
    static int __init tsm_mr_sample_init(void)
    {
    int rc;
    sample_groups[0] = tsm_mr_create_attribute_group(&sample_tm);
    if (IS_ERR(sample_groups[0]))
    return PTR_ERR(sample_groups[0]);
    rc = misc_register(&sample_misc_dev);
    if (rc)
    tsm_mr_free_attribute_group(sample_groups[0]);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn tsm_mr_sample_exit() -> void __exit {
    static void __exit tsm_mr_sample_exit(void)
    {
    misc_deregister(&sample_misc_dev);
    tsm_mr_free_attribute_group(sample_groups[0]);
    }
    module_init(tsm_mr_sample_init);
    module_exit(tsm_mr_sample_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Sample module using tsm-mr to expose emulated MRs");
