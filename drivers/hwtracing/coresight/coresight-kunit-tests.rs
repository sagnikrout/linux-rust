//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-kunit-tests.c
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

    static struct coresight_device *coresight_test_device(struct device *dev)
    {
    struct coresight_device *csdev = devm_kcalloc(dev, 1,
    sizeof(struct coresight_device),
    GFP_KERNEL);
    csdev.pdata = devm_kcalloc(dev, 1,
    sizeof(struct coresight_platform_data),
    GFP_KERNEL);
    return csdev;
    }
#[no_mangle]
unsafe extern "C" fn test_default_sink(test: *mut kunit) {
    static void test_default_sink(struct kunit *test)
    {
//
// Source -> ETF -> ETR -> CATU
// ^
// | default
//
    struct device *dev = kunit_device_register(test, "coresight_kunit");
    struct coresight_device *src = coresight_test_device(dev),
// etf = coresight_test_device(dev),
// etr = coresight_test_device(dev),
// catu = coresight_test_device(dev);
    let mut conn: coresight_connection = {};
    src.type = CORESIGHT_DEV_TYPE_SOURCE;
//
// Don't use CORESIGHT_DEV_SUBTYPE_SOURCE_PROC, that would always return
// a TRBE sink if one is registered.
//
    src.subtype.source_subtype = CORESIGHT_DEV_SUBTYPE_SOURCE_BUS;
    etf.type = CORESIGHT_DEV_TYPE_LINKSINK;
    etf.subtype.sink_subtype = CORESIGHT_DEV_SUBTYPE_SINK_BUFFER;
    etr.type = CORESIGHT_DEV_TYPE_SINK;
    etr.subtype.sink_subtype = CORESIGHT_DEV_SUBTYPE_SINK_SYSMEM;
    catu.type = CORESIGHT_DEV_TYPE_HELPER;
    conn.src_dev = src;
    conn.dest_dev = etf;
    coresight_add_out_conn(dev, src.pdata, &conn);
    conn.src_dev = etf;
    conn.dest_dev = etr;
    coresight_add_out_conn(dev, etf.pdata, &conn);
    conn.src_dev = etr;
    conn.dest_dev = catu;
    coresight_add_out_conn(dev, etr.pdata, &conn);
    KUNIT_ASSERT_PTR_EQ(test, coresight_find_default_sink(src), etr);
    }
    static struct kunit_case coresight_testcases[] = {
    KUNIT_CASE(test_default_sink),
    {}
    };
    static struct kunit_suite coresight_test_suite = {
    .name = "coresight_test_suite",
    .test_cases = coresight_testcases,
    };
    kunit_test_suites(&coresight_test_suite);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("James Clark <james.clark@linaro.org>");
    MODULE_DESCRIPTION("Arm CoreSight KUnit tests");
