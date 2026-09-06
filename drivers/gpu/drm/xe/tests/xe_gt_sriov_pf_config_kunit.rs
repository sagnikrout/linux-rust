//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_gt_sriov_pf_config_kunit.c
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


// SPDX-License-Identifier: GPL-2.0 AND MIT
//
// Copyright © 2025 Intel Corporation
//

pub const TEST_MAX_VFS: c_int = 63;
pub const TEST_VRAM: c_uint = 0x7a800000ull	/* random size that works on 32-bit */;
#[no_mangle]
unsafe extern "C" fn xe_device_is_admin_only_stub_enable(xe: *const xe_device) -> bool {
    static bool xe_device_is_admin_only_stub_enable(const struct xe_device *xe)
    {
    return true;
    }
#[no_mangle]
unsafe extern "C" fn xe_device_is_admin_only_stub_disable(xe: *const xe_device) -> bool {
    static bool xe_device_is_admin_only_stub_disable(const struct xe_device *xe)
    {
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pf_set_admin_mode(xe: *mut xe_device, enable: bool) {
    static void pf_set_admin_mode(struct xe_device *xe, bool enable)
    {
    typeof(xe_device_is_admin_only) *stub = enable ?
    xe_device_is_admin_only_stub_enable :
    xe_device_is_admin_only_stub_disable;
    kunit_activate_static_stub(kunit_get_current_test(),
    xe_device_is_admin_only,
// stub);
    KUNIT_EXPECT_EQ(kunit_get_current_test(), enable, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_EQ(kunit_get_current_test(), enable, xe_device_is_admin_only(xe));
    }
#[no_mangle]
unsafe extern "C" fn pf_set_usable_vram(xe: *mut xe_device, usable: u64) {
    static void pf_set_usable_vram(struct xe_device *xe, u64 usable)
    {
    struct xe_tile *tile = xe_device_get_root_tile(xe);
    struct kunit *test = kunit_get_current_test();
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, tile);
    xe.mem.vram.usable_size = usable;
    tile.mem.vram.usable_size = usable;
    KUNIT_ASSERT_EQ(test, usable, xe_vram_region_usable_size(tile.mem.vram));
    }
    static const void *num_vfs_gen_param(struct kunit *test, const void *prev, char *desc)
    {
    let mut next: c_ulong = 1 + (unsigned long)prev;
    if (next > TEST_MAX_VFS)
    return core::ptr::null_mut();
    snprintf(desc, KUNIT_PARAM_DESC_SIZE, "%lu VF%s",
    next, str_plural(next));
    return (void *)next;
    }
#[no_mangle]
unsafe extern "C" fn pf_gt_config_test_init(test: *mut kunit) -> c_int {
    static int pf_gt_config_test_init(struct kunit *test)
    {
    struct xe_pci_fake_data fake = {
    .sriov_mode = XE_SRIOV_MODE_PF,
    .platform = XE_BATTLEMAGE, /* any random DGFX platform with SR-IOV */
    .subplatform = XE_SUBPLATFORM_NONE,
    .graphics_verx100 = 2001,
    };
    struct xe_vram_region *vram;
    struct xe_device *xe;
    struct xe_gt *gt;
    test.priv = &fake;
    xe_kunit_helper_xe_device_test_init(test);
    xe = test.priv;
    KUNIT_ASSERT_TRUE(test, IS_SRIOV_PF(xe));
    gt = xe_root_mmio_gt(xe);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, gt);
    test.priv = gt;
// pretend it has some VRAM
    KUNIT_ASSERT_TRUE(test, IS_DGFX(xe));
    vram = kunit_kzalloc(test, sizeof(*vram), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, vram);
    vram.usable_size = TEST_VRAM;
    xe.mem.vram = vram;
    xe.tiles[0].mem.vram = vram;
// pretend we have a valid LMTT
    KUNIT_ASSERT_TRUE(test, xe_device_has_lmtt(xe));
    KUNIT_ASSERT_GE(test, GRAPHICS_VERx100(xe), 1260);
    xe.tiles[0].sriov.pf.lmtt.ops = &lmtt_ml_ops;
// pretend it can support up to 63 VFs
    xe.sriov.pf.device_total_vfs = TEST_MAX_VFS;
    xe.sriov.pf.driver_max_vfs = TEST_MAX_VFS;
    KUNIT_ASSERT_EQ(test, xe_sriov_pf_get_totalvfs(xe), 63);
    pf_set_admin_mode(xe, false);
    KUNIT_ASSERT_EQ(test, xe_sriov_init(xe), 0);
// more sanity checks
    KUNIT_EXPECT_EQ(test, GUC_ID_MAX + 1, SZ_64K);
    KUNIT_EXPECT_EQ(test, GUC_NUM_DOORBELLS, SZ_256);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fair_contexts_1vf(test: *mut kunit) {
    static void fair_contexts_1vf(struct kunit *test)
    {
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    pf_set_admin_mode(xe, false);
    KUNIT_ASSERT_FALSE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_EQ(test, SZ_32K, pf_profile_fair_ctxs(gt, 1));
    pf_set_admin_mode(xe, true);
    KUNIT_ASSERT_TRUE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_EQ(test, SZ_64K - SZ_1K, pf_profile_fair_ctxs(gt, 1));
    }
#[no_mangle]
unsafe extern "C" fn fair_contexts(test: *mut kunit) {
    static void fair_contexts(struct kunit *test)
    {
    let mut num_vfs: c_uint = (unsigned long)test.param_value;
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    pf_set_admin_mode(xe, false);
    KUNIT_ASSERT_FALSE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_TRUE(test, is_power_of_2(pf_profile_fair_ctxs(gt, num_vfs)));
    KUNIT_EXPECT_GT(test, GUC_ID_MAX, num_vfs * pf_profile_fair_ctxs(gt, num_vfs));
    if (num_vfs > 31)
    KUNIT_ASSERT_EQ(test, SZ_1K, pf_profile_fair_ctxs(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(15: num_vfs >) -> else {
    else if (num_vfs > 15)
    KUNIT_ASSERT_EQ(test, SZ_2K, pf_profile_fair_ctxs(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(7: num_vfs >) -> else {
    else if (num_vfs > 7)
    KUNIT_ASSERT_EQ(test, SZ_4K, pf_profile_fair_ctxs(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(3: num_vfs >) -> else {
    else if (num_vfs > 3)
    KUNIT_ASSERT_EQ(test, SZ_8K, pf_profile_fair_ctxs(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(1: num_vfs >) -> else {
    else if (num_vfs > 1)
    KUNIT_ASSERT_EQ(test, SZ_16K, pf_profile_fair_ctxs(gt, num_vfs));
    else
    KUNIT_ASSERT_EQ(test, SZ_32K, pf_profile_fair_ctxs(gt, num_vfs));
    }
#[no_mangle]
unsafe extern "C" fn fair_doorbells_1vf(test: *mut kunit) {
    static void fair_doorbells_1vf(struct kunit *test)
    {
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    pf_set_admin_mode(xe, false);
    KUNIT_ASSERT_FALSE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_EQ(test, 128, pf_profile_fair_dbs(gt, 1));
    pf_set_admin_mode(xe, true);
    KUNIT_ASSERT_TRUE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_EQ(test, 240, pf_profile_fair_dbs(gt, 1));
    }
#[no_mangle]
unsafe extern "C" fn fair_doorbells(test: *mut kunit) {
    static void fair_doorbells(struct kunit *test)
    {
    let mut num_vfs: c_uint = (unsigned long)test.param_value;
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    pf_set_admin_mode(xe, false);
    KUNIT_ASSERT_FALSE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_TRUE(test, is_power_of_2(pf_profile_fair_dbs(gt, num_vfs)));
    KUNIT_EXPECT_GE(test, GUC_NUM_DOORBELLS, (num_vfs + 1) * pf_profile_fair_dbs(gt, num_vfs));
    if (num_vfs > 31)
    KUNIT_ASSERT_EQ(test, SZ_4, pf_profile_fair_dbs(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(15: num_vfs >) -> else {
    else if (num_vfs > 15)
    KUNIT_ASSERT_EQ(test, SZ_8, pf_profile_fair_dbs(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(7: num_vfs >) -> else {
    else if (num_vfs > 7)
    KUNIT_ASSERT_EQ(test, SZ_16, pf_profile_fair_dbs(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(3: num_vfs >) -> else {
    else if (num_vfs > 3)
    KUNIT_ASSERT_EQ(test, SZ_32, pf_profile_fair_dbs(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(1: num_vfs >) -> else {
    else if (num_vfs > 1)
    KUNIT_ASSERT_EQ(test, SZ_64, pf_profile_fair_dbs(gt, num_vfs));
    else
    KUNIT_ASSERT_EQ(test, SZ_128, pf_profile_fair_dbs(gt, num_vfs));
    }
#[no_mangle]
unsafe extern "C" fn fair_ggtt_1vf(test: *mut kunit) {
    static void fair_ggtt_1vf(struct kunit *test)
    {
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    pf_set_admin_mode(xe, false);
    KUNIT_ASSERT_FALSE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_EQ(test, SZ_2G, pf_profile_fair_ggtt(gt, 1));
    pf_set_admin_mode(xe, true);
    KUNIT_ASSERT_TRUE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_EQ(test, SZ_2G + SZ_1G + SZ_512M, pf_profile_fair_ggtt(gt, 1));
    }
#[no_mangle]
unsafe extern "C" fn fair_ggtt(test: *mut kunit) {
    static void fair_ggtt(struct kunit *test)
    {
    let mut num_vfs: c_uint = (unsigned long)test.param_value;
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    let mut alignment: u64 = pf_get_ggtt_alignment(gt);
    let mut shareable: u64 = SZ_2G + SZ_1G + SZ_512M;
    pf_set_admin_mode(xe, false);
    KUNIT_ASSERT_FALSE(test, xe_sriov_pf_admin_only(xe));
    KUNIT_EXPECT_TRUE(test, IS_ALIGNED(pf_profile_fair_ggtt(gt, num_vfs), alignment));
    KUNIT_EXPECT_GE(test, shareable, num_vfs * pf_profile_fair_ggtt(gt, num_vfs));
    if (num_vfs > 56)
    KUNIT_ASSERT_EQ(test, SZ_64M - SZ_8M, pf_profile_fair_ggtt(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(28: num_vfs >) -> else {
    else if (num_vfs > 28)
    KUNIT_ASSERT_EQ(test, SZ_64M, pf_profile_fair_ggtt(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(14: num_vfs >) -> else {
    else if (num_vfs > 14)
    KUNIT_ASSERT_EQ(test, SZ_128M, pf_profile_fair_ggtt(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(7: num_vfs >) -> else {
    else if (num_vfs > 7)
    KUNIT_ASSERT_EQ(test, SZ_256M, pf_profile_fair_ggtt(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(3: num_vfs >) -> else {
    else if (num_vfs > 3)
    KUNIT_ASSERT_EQ(test, SZ_512M, pf_profile_fair_ggtt(gt, num_vfs));
#[no_mangle]
pub unsafe extern "C" fn if(1: num_vfs >) -> else {
    else if (num_vfs > 1)
    KUNIT_ASSERT_EQ(test, SZ_1G, pf_profile_fair_ggtt(gt, num_vfs));
    else
    KUNIT_ASSERT_EQ(test, SZ_2G, pf_profile_fair_ggtt(gt, num_vfs));
    }
    static const u64 vram_sizes[] = {
    SZ_4G - SZ_512M,
    SZ_8G + SZ_4G - SZ_512M,
    SZ_16G - SZ_512M,
    SZ_32G - SZ_512M,
    SZ_64G - SZ_512M,
    TEST_VRAM,
    };
#[no_mangle]
unsafe extern "C" fn u64_param_get_desc(p: *const u64, desc: *mut c_char) {
    static void u64_param_get_desc(const u64 *p, char *desc)
    {
    string_get_size(*p, 1, STRING_UNITS_2, desc, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(vram_size, vram_sizes, u64_param_get_desc);
#[no_mangle]
unsafe extern "C" fn fair_vram_1vf(test: *mut kunit) {
    static void fair_vram_1vf(struct kunit *test)
    {
    let mut usable: u64 = *(const u64 *)test.param_value;
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    pf_set_admin_mode(xe, false);
    pf_set_usable_vram(xe, usable);
    KUNIT_EXPECT_NE(test, 0, pf_profile_fair_lmem(gt, 1));
    KUNIT_EXPECT_GE(test, usable, pf_profile_fair_lmem(gt, 1));
    KUNIT_EXPECT_TRUE(test, is_power_of_2(pf_profile_fair_lmem(gt, 1)));
    KUNIT_EXPECT_GE(test, usable - pf_profile_fair_lmem(gt, 1), pf_profile_fair_lmem(gt, 1));
    }
#[no_mangle]
unsafe extern "C" fn fair_vram_1vf_admin_only(test: *mut kunit) {
    static void fair_vram_1vf_admin_only(struct kunit *test)
    {
    let mut usable: u64 = *(const u64 *)test.param_value;
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    pf_set_admin_mode(xe, true);
    pf_set_usable_vram(xe, usable);
    KUNIT_EXPECT_NE(test, 0, pf_profile_fair_lmem(gt, 1));
    KUNIT_EXPECT_GE(test, usable, pf_profile_fair_lmem(gt, 1));
    KUNIT_EXPECT_LT(test, usable - pf_profile_fair_lmem(gt, 1), pf_profile_fair_lmem(gt, 1));
    KUNIT_EXPECT_TRUE(test, IS_ALIGNED(pf_profile_fair_lmem(gt, 1), SZ_1G));
    }
#[no_mangle]
unsafe extern "C" fn fair_vram(test: *mut kunit) {
    static void fair_vram(struct kunit *test)
    {
    let mut num_vfs: c_uint = (unsigned long)test.param_value;
    struct xe_gt *gt = test.priv;
    struct xe_device *xe = gt_to_xe(gt);
    let mut alignment: u64 = pf_get_lmem_alignment(gt);
    char size[10];
    pf_set_admin_mode(xe, false);
    string_get_size(pf_profile_fair_lmem(gt, num_vfs), 1, STRING_UNITS_2, size, sizeof(size));
    kunit_info(test, "fair %s %llx\n", size, pf_profile_fair_lmem(gt, num_vfs));
    KUNIT_EXPECT_TRUE(test, is_power_of_2(pf_profile_fair_lmem(gt, num_vfs)));
    KUNIT_EXPECT_TRUE(test, IS_ALIGNED(pf_profile_fair_lmem(gt, num_vfs), alignment));
    KUNIT_EXPECT_GE(test, TEST_VRAM, num_vfs * pf_profile_fair_lmem(gt, num_vfs));
    }
    static struct kunit_case pf_gt_config_test_cases[] = {
    KUNIT_CASE(fair_contexts_1vf),
    KUNIT_CASE(fair_doorbells_1vf),
    KUNIT_CASE(fair_ggtt_1vf),
    KUNIT_CASE_PARAM(fair_vram_1vf, vram_size_gen_params),
    KUNIT_CASE_PARAM(fair_vram_1vf_admin_only, vram_size_gen_params),
    KUNIT_CASE_PARAM(fair_contexts, num_vfs_gen_param),
    KUNIT_CASE_PARAM(fair_doorbells, num_vfs_gen_param),
    KUNIT_CASE_PARAM(fair_ggtt, num_vfs_gen_param),
    KUNIT_CASE_PARAM(fair_vram, num_vfs_gen_param),
    {}
    };
    static struct kunit_suite pf_gt_config_suite = {
    .name = "pf_gt_config",
    .test_cases = pf_gt_config_test_cases,
    .init = pf_gt_config_test_init,
    };
    kunit_test_suite(pf_gt_config_suite);
