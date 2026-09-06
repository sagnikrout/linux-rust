//! Automatically rewritten from C to Rust
//! Source: drivers/of/of_test.c
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
// KUnit tests for OF APIs
//

//
// Test that the root node "/" can be found by path.
//
#[no_mangle]
unsafe extern "C" fn of_dtb_root_node_found_by_path(test: *mut kunit) {
    static void of_dtb_root_node_found_by_path(struct kunit *test)
    {
    struct device_node *np;
    np = of_find_node_by_path("/");
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, np);
    of_node_put(np);
    }
//
// Test that the 'of_root' global variable is always populated when DT code is
// enabled. Remove this test once of_root is removed from global access.
//
#[no_mangle]
unsafe extern "C" fn of_dtb_root_node_populates_of_root(test: *mut kunit) {
    static void of_dtb_root_node_populates_of_root(struct kunit *test)
    {
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, of_root);
    }
    static struct kunit_case of_dtb_test_cases[] = {
    KUNIT_CASE(of_dtb_root_node_found_by_path),
    KUNIT_CASE(of_dtb_root_node_populates_of_root),
    {}
    };
#[no_mangle]
unsafe extern "C" fn of_dtb_test_init(test: *mut kunit) -> c_int {
    static int of_dtb_test_init(struct kunit *test)
    {
    of_root_kunit_skip(test);
    if (!IS_ENABLED(CONFIG_OF_EARLY_FLATTREE))
    kunit_skip(test, "requires CONFIG_OF_EARLY_FLATTREE");
    return 0;
    }
//
// Test suite to confirm a DTB is loaded.
//
    static struct kunit_suite of_dtb_suite = {
    .name = "of_dtb",
    .test_cases = of_dtb_test_cases,
    .init = of_dtb_test_init,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_address_resource_bounds_case {
    pub start: u64,
    pub size: u64,
    pub ret: c_int,
    pub res_start: u64,
    pub res_end: u64,
}

    static void of_address_resource_bounds_case_desc(const struct of_address_resource_bounds_case *p,
    char *name)
    {
    snprintf(name, KUNIT_PARAM_DESC_SIZE, "start=0x%016llx,size=0x%016llx", p.start, p.size);
    }
    static const struct of_address_resource_bounds_case of_address_resource_bounds_cases[] = {
    {
    .start = 0,
    .size = 0,
    .ret = 0,
    .res_start = 0,
    .res_end = -1,
    },
    {
    .start = 0,
    .size = 0x1000,
    .ret = 0,
    .res_start = 0,
    .res_end = 0xfff,
    },
    {
    .start = 0x1000,
    .size = 0,
    .ret = 0,
    .res_start = 0x1000,
    .res_end = 0xfff,
    },
    {
    .start = 0x1000,
    .size = 0x1000,
    .ret = 0,
    .res_start = 0x1000,
    .res_end = 0x1fff,
    },
    {
    .start = 1,
    .size = RESOURCE_SIZE_MAX,
    .ret = 0,
    .res_start = 1,
    .res_end = RESOURCE_SIZE_MAX,
    },
    {
    .start = RESOURCE_SIZE_MAX,
    .size = 1,
    .ret = 0,
    .res_start = RESOURCE_SIZE_MAX,
    .res_end = RESOURCE_SIZE_MAX,
    },
    {
    .start = 2,
    .size = RESOURCE_SIZE_MAX,
    .ret = -EOVERFLOW,
    },
    {
    .start = RESOURCE_SIZE_MAX,
    .size = 2,
    .ret = -EOVERFLOW,
    },
    {
    .start = ULL(0x100000000),
    .size = 1,
    .ret = sizeof(resource_size_t) > sizeof(u32) ? 0 : -EOVERFLOW,
    .res_start = ULL(0x100000000),
    .res_end = ULL(0x100000000),
    },
    {
    .start = 0x1000,
    .size = 0xffffffff,
    .ret = sizeof(resource_size_t) > sizeof(u32) ? 0 : -EOVERFLOW,
    .res_start = 0x1000,
    .res_end = ULL(0x100000ffe),
    },
    };
    KUNIT_ARRAY_PARAM(of_address_resource_bounds,
    of_address_resource_bounds_cases, of_address_resource_bounds_case_desc);
#[no_mangle]
unsafe extern "C" fn of_address_resource_bounds(test: *mut kunit) {
    static void of_address_resource_bounds(struct kunit *test)
    {
    const struct of_address_resource_bounds_case *param = test.param_value;
    struct resource r; /* Intentionally uninitialized */
    int ret;
    if (!IS_ENABLED(CONFIG_OF_ADDRESS))
    kunit_skip(test, "CONFIG_OF_ADDRESS not enabled\n");
    ret = __of_address_resource_bounds(&r, param.start, param.size);
    KUNIT_EXPECT_EQ(test, param.ret, ret);
    if (ret == 0) {
    KUNIT_EXPECT_EQ(test, (resource_size_t)param.res_start, r.start);
    KUNIT_EXPECT_EQ(test, (resource_size_t)param.res_end, r.end);
    KUNIT_EXPECT_EQ(test, param.size, resource_size(&r));
    }
    }
    static struct kunit_case of_address_test_cases[] = {
    KUNIT_CASE_PARAM(of_address_resource_bounds, of_address_resource_bounds_gen_params),
    {}
    };
    static struct kunit_suite of_address_suite = {
    .name = "of_address",
    .test_cases = of_address_test_cases,
    };
    kunit_test_suites(
    &of_dtb_suite, &of_address_suite,
    );
    MODULE_DESCRIPTION("KUnit tests for OF APIs");
    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
    MODULE_LICENSE("GPL");
