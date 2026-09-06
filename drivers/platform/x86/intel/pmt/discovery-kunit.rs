//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/pmt/discovery-kunit.c
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
// Intel Platform Monitory Technology Discovery KUNIT tests
//
// Copyright (c) 2025, Intel Corporation.
// All Rights Reserved.
//

    static void
    validate_pmt_regions(struct kunit *test, struct pmt_feature_group *feature_group, int feature_id)
    {
    int i;
    kunit_info(test, "Feature ID %d [%s] has %d regions.\n", feature_id,
    pmt_feature_names[feature_id], feature_group.count);
    for (i = 0; i < feature_group.count; i++) {
    struct telemetry_region *region = &feature_group.regions[i];
    kunit_info(test, "  - Region %d: cdie_mask=%u, package_id=%u, partition=%u, segment=%u,",
    i, region.plat_info.cdie_mask, region.plat_info.package_id,
    region.plat_info.partition, region.plat_info.segment);
    kunit_info(test, "\t\tbus=%u, device=%u, function=%u, guid=0x%x,",
    region.plat_info.bus_number, region.plat_info.device_number,
    region.plat_info.function_number, region.guid);
    kunit_info(test, "\t\taddr=%p, size=%zu, num_rmids=%u", region.addr, region.size,
    region.num_rmids);
    KUNIT_ASSERT_GE(test, region.plat_info.cdie_mask, 0);
    KUNIT_ASSERT_GE(test, region.plat_info.package_id, 0);
    KUNIT_ASSERT_GE(test, region.plat_info.partition, 0);
    KUNIT_ASSERT_GE(test, region.plat_info.segment, 0);
    KUNIT_ASSERT_GE(test, region.plat_info.bus_number, 0);
    KUNIT_ASSERT_GE(test, region.plat_info.device_number, 0);
    KUNIT_ASSERT_GE(test, region.plat_info.function_number, 0);
    KUNIT_ASSERT_NE(test, region.guid, 0);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ( const void *)region.addr);
    }
    }
#[no_mangle]
unsafe extern "C" fn linebreak(test: *mut kunit) {
    static void linebreak(struct kunit *test)
    {
    kunit_info(test, "*****************************************************************************\n");
    }
#[no_mangle]
unsafe extern "C" fn test_intel_pmt_get_regions_by_feature(test: *mut kunit) {
    static void test_intel_pmt_get_regions_by_feature(struct kunit *test)
    {
    struct pmt_feature_group *feature_group;
    let mut num_available: c_int = 0;
    int feature_id;
// Iterate through all possible feature IDs
    for (feature_id = 1; feature_id < PMT_FEATURE_COUNT; feature_id++, linebreak(test)) {
    const char *name;
    if (!pmt_feature_id_is_valid(feature_id))
    continue;
    name = pmt_feature_names[feature_id];
    feature_group = intel_pmt_get_regions_by_feature(feature_id);
    if (IS_ERR(feature_group)) {
    if (PTR_ERR(feature_group) == -ENOENT)
    kunit_warn(test, "intel_pmt_get_regions_by_feature() reporting feature %d [%s] is not present.\n",
    feature_id, name);
    else
    kunit_warn(test, "intel_pmt_get_regions_by_feature() returned error %ld while attempt to lookup %d [%s].\n",
    PTR_ERR(feature_group), feature_id, name);
    continue;
    }
    if (!feature_group) {
    kunit_warn(test, "Feature ID %d: %s is not available.\n", feature_id, name);
    continue;
    }
    num_available++;
    validate_pmt_regions(test, feature_group, feature_id);
    intel_pmt_put_feature_group(feature_group);
    }
    if (num_available == 0)
    kunit_warn(test, "No PMT region groups were available for any feature ID (0-10).\n");
    }
    static struct kunit_case intel_pmt_discovery_test_cases[] = {
    KUNIT_CASE(test_intel_pmt_get_regions_by_feature),
    {}
    };
    static struct kunit_suite intel_pmt_discovery_test_suite = {
    .name = "pmt_discovery_test",
    .test_cases = intel_pmt_discovery_test_cases,
    };
    kunit_test_suite(intel_pmt_discovery_test_suite);
    MODULE_IMPORT_NS("INTEL_PMT_DISCOVERY");
    MODULE_AUTHOR("David E. Box <david.e.box@linux.intel.com>");
    MODULE_DESCRIPTION("Intel PMT Discovery KUNIT test driver");
    MODULE_LICENSE("GPL");
