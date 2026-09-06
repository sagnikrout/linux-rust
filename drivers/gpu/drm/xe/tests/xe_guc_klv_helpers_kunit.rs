//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/tests/xe_guc_klv_helpers_kunit.c
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
// Copyright © 2026 Intel Corporation
//

pub const TEST_PAD: c_uint = 0xdeadbeef;
#[no_mangle]
unsafe extern "C" fn fake_is_group_key(key: u16) -> bool {
    static bool fake_is_group_key(u16 key)
    {
    return is_reserved_key(key) && key >= TEST_GROUP_KEY;
    }
#[no_mangle]
unsafe extern "C" fn test_count(test: *mut kunit) {
    static void test_count(struct kunit *test)
    {
    let mut value: u32 = 0x12345678;
    let mut key: u16 = TEST_KEY;
    u32 klvs[] = {
    PREP_GUC_KLV(key + 0, 0),
    PREP_GUC_KLV(key + 1, 1), value,
    PREP_GUC_KLV(key + 2, 2), value, value,
    PREP_GUC_KLV(key + 3, 0),
    0, /* padding */
    };
    KUNIT_EXPECT_EQ(test, 0, xe_guc_klv_count(klvs, 0));
    KUNIT_EXPECT_EQ(test, 1, xe_guc_klv_count(klvs, 1));
    KUNIT_EXPECT_GT(test, 0, xe_guc_klv_count(klvs, 2));
    KUNIT_EXPECT_EQ(test, 2, xe_guc_klv_count(klvs, 3));
    KUNIT_EXPECT_GT(test, 0, xe_guc_klv_count(klvs, 4));
    KUNIT_EXPECT_GT(test, 0, xe_guc_klv_count(klvs, 5));
    KUNIT_EXPECT_EQ(test, 3, xe_guc_klv_count(klvs, 6));
    KUNIT_EXPECT_EQ(test, 4, xe_guc_klv_count(klvs, 7));
// 0 is treated as reserved KLV { KEY=0, LEN=0 }
    KUNIT_EXPECT_EQ(test, 5, xe_guc_klv_count(klvs, 8));
    }
#[no_mangle]
unsafe extern "C" fn test_encode_u32(test: *mut kunit) {
    static void test_encode_u32(struct kunit *test)
    {
    u32 *fail = ERR_PTR(-ENOMEM);
    let mut value: u32 = 0x12345678;
    let mut key: u16 = TEST_KEY;
    u32 klvs[16];
    memset32(klvs, TEST_PAD, ARRAY_SIZE(klvs));
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-ENOSPC), xe_guc_klv_encode_u32(klvs, 0, key, value));
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-ENOSPC), xe_guc_klv_encode_u32(klvs, 1, key, value));
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, xe_guc_klv_encode_u32(klvs, 2, key, value));
    KUNIT_EXPECT_EQ(test, klvs[0], PREP_GUC_KLV(key, 1));
    KUNIT_EXPECT_EQ(test, klvs[1], value);
    KUNIT_EXPECT_EQ(test, klvs[2], TEST_PAD);
    KUNIT_EXPECT_PTR_EQ(test, &klvs[2], xe_guc_klv_encode_u32(klvs, 2, key, value));
    KUNIT_EXPECT_PTR_EQ(test,
    xe_guc_klv_encode_u32(klvs, 2, key, value),
    xe_guc_klv_encode_u32(klvs, ARRAY_SIZE(klvs), key, value));
    KUNIT_ASSERT_PTR_EQ(test, fail, xe_guc_klv_encode_u32(fail, ARRAY_SIZE(klvs), key, value));
    }
#[no_mangle]
unsafe extern "C" fn test_encode_u64(test: *mut kunit) {
    static void test_encode_u64(struct kunit *test)
    {
    let mut value: u64 = 0x123456789abcdef0;
    u32 *fail = ERR_PTR(-ENOMEM);
    let mut key: u16 = TEST_KEY;
    u32 klvs[16];
    memset32(klvs, TEST_PAD, ARRAY_SIZE(klvs));
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-ENOSPC), xe_guc_klv_encode_u64(klvs, 0, key, value));
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-ENOSPC), xe_guc_klv_encode_u64(klvs, 1, key, value));
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-ENOSPC), xe_guc_klv_encode_u64(klvs, 2, key, value));
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, xe_guc_klv_encode_u64(klvs, 3, key, value));
    KUNIT_EXPECT_EQ(test, klvs[0], PREP_GUC_KLV(key, 2));
    KUNIT_EXPECT_EQ(test, klvs[1], lower_32_bits(value));
    KUNIT_EXPECT_EQ(test, klvs[2], upper_32_bits(value));
    KUNIT_EXPECT_EQ(test, klvs[3], TEST_PAD);
    KUNIT_EXPECT_PTR_EQ(test, &klvs[3], xe_guc_klv_encode_u64(klvs, 3, key, value));
    KUNIT_EXPECT_PTR_EQ(test,
    xe_guc_klv_encode_u64(klvs, 3, key, value),
    xe_guc_klv_encode_u64(klvs, ARRAY_SIZE(klvs), key, value));
    KUNIT_ASSERT_PTR_EQ(test, fail, xe_guc_klv_encode_u64(fail, ARRAY_SIZE(klvs), key, value));
    }
#[no_mangle]
unsafe extern "C" fn str_klv_size(string: *const c_char) -> u32 {
    static u32 str_klv_size(const char *string)
    {
    return GUC_KLV_LEN_MIN + to_num_dwords(strlen(string) + 1);
    }
#[no_mangle]
unsafe extern "C" fn test_encode_string(test: *mut kunit) {
    static void test_encode_string(struct kunit *test)
    {
    let mut longest_str: usize = to_num_bytes(FIELD_MAX(GUC_KLV_0_LEN)) - 1;
    let mut avail: u32 = GUC_KLV_LEN_MIN + FIELD_MAX(GUC_KLV_0_LEN) + 1;
    const char *string = "abcdefghijklmnopqrstvwxyz";
    let mut key: u16 = TEST_KEY;
    u32 *klvs;
    u32 *next;
    char *buf;
    u32 n;
    klvs = kunit_kcalloc(test, avail, sizeof(u32), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, klvs);
    buf = kunit_kzalloc(test, longest_str + 2, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, buf);
// empty string, no space, must fail
    for (n = 0; n < str_klv_size(""); n++) {
    klvs[0] = TEST_PAD;
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-ENOSPC),
    xe_guc_klv_encode_string(klvs, n, key, ""));
    KUNIT_EXPECT_EQ(test, klvs[0], TEST_PAD);
    }
// empty string, must pass
    KUNIT_EXPECT_PTR_EQ(test, klvs + str_klv_size(""),
    xe_guc_klv_encode_string(klvs, str_klv_size(""), key, ""));
    KUNIT_EXPECT_PTR_EQ(test, klvs + str_klv_size(""),
    xe_guc_klv_encode_string(klvs, avail, key, ""));
// demo string, no space, must fail
    for (n = 0; n < str_klv_size(string); n++) {
    klvs[0] = TEST_PAD;
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-ENOSPC),
    xe_guc_klv_encode_string(klvs, n, key, string));
    KUNIT_EXPECT_EQ(test, klvs[0], TEST_PAD);
    }
// different string len, must pass
    for (n = 0; n <= strlen(string); n++) {
    strscpy(buf, string, n + 1);
    kunit_info(test, "%u: '%s'\n", n, buf);
    KUNIT_ASSERT_EQ(test, n, strlen(buf));
    memset32(klvs, TEST_PAD, avail);
    next = xe_guc_klv_encode_string(klvs, str_klv_size(buf), key, buf);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, next);
    KUNIT_EXPECT_PTR_EQ(test, next, klvs + str_klv_size(buf));
    KUNIT_EXPECT_STREQ_MSG(test, buf, (char *)(klvs + GUC_KLV_LEN_MIN), "n=%u", n);
    kunit_info(test, "%u: %*ph\n", n, (int)to_num_bytes(next - klvs), klvs);
    KUNIT_EXPECT_NE(test, *(next - 1), TEST_PAD);
    KUNIT_ASSERT_EQ(test, *next, TEST_PAD);
// bigger buf doesn't matter
    KUNIT_EXPECT_PTR_EQ(test,
    xe_guc_klv_encode_string(klvs, str_klv_size(buf), key, buf),
    xe_guc_klv_encode_string(klvs, avail, key, buf));
    }
// don't crash if already failed
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-EROFS),
    xe_guc_klv_encode_string(ERR_PTR(-EROFS), avail, key, ""));
// too long string, must fail
    memset(buf, 'X', longest_str + 1);
    buf[longest_str + 1] = '\0';
    KUNIT_EXPECT_LT(test, longest_str, strlen(buf));
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-E2BIG),
    xe_guc_klv_encode_string(klvs, avail, key, buf));
// longest string, should pass
    buf[longest_str] = '\0';
    KUNIT_EXPECT_EQ(test, longest_str, strlen(buf));
    KUNIT_EXPECT_PTR_EQ(test, klvs + str_klv_size(buf),
    xe_guc_klv_encode_string(klvs, avail, key, buf));
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct some_object {
    pub value1: u32,
    pub value2: u64,
    pub __packed: },
    static u32 *obj_raw_encoder(u32 *klvs, u32 avail, const void *arg)
    {
    pub arg: *const *const some_object obj =,
    pub sizeof(*obj): *mut size_t sz =,
    pub to_num_dwords(sz): u32 dwords =,
    if (IS_ERR(klvs))
    pub klvs: return,
    if (dwords > avail)
    pub ERR_PTR(-ENOSPC): return,
    pub sz): memcpy(klvs, obj,,
    pub dwords: return klvs +,
    }
    static u32 *obj_klv_encoder(u32 *klvs, u32 avail, const void *arg)
    {
    pub arg: *const *const some_object obj =,
    pub avail: *mut *mut u32 end = klvs +,
    pub obj->value1): klvs = xe_guc_klv_encode_u32(klvs, end - klvs, TEST_KEY + 1,,
    pub obj->value2): klvs = xe_guc_klv_encode_u64(klvs, end - klvs, TEST_KEY + 2,,
    pub klvs: return,
    }
    static u32 *obj_nested_encoder(u32 *klvs, u32 avail, const void *arg)
    {
    pub avail: *mut *mut u32 end = klvs +,
    klvs = xe_guc_klv_encode_object(klvs, end - klvs, TEST_GROUP_KEY + 1,
    pub obj_klv_encoder): arg,,
    klvs = xe_guc_klv_encode_object(klvs, end - klvs, TEST_GROUP_KEY + 2,
    pub obj_klv_encoder): arg,,
    pub klvs: return,
    }
#[no_mangle]
unsafe extern "C" fn test_encode_object_raw(test: *mut kunit) {
    static void test_encode_object_raw(struct kunit *test)
    {
    const struct some_object obj = {
    .value1 = 0xdead1234,
    .value2 = 0xdead87654321dead,
}

    let mut payload: u32 = to_num_dwords(sizeof(obj));
    u32 *err = ERR_PTR(-ENOSPC);
    let mut key: u16 = TEST_KEY;
    u32 klvs[16];
    u32 n;
// too small, must fail
    for (n = 0; n < GUC_KLV_LEN_MIN + payload; n++) {
    memset32(klvs, TEST_PAD, ARRAY_SIZE(klvs));
    KUNIT_EXPECT_PTR_EQ_MSG(test, ERR_PTR(-ENOSPC),
    xe_guc_klv_encode_object(klvs, n, key, &obj,
    obj_raw_encoder),
    "buf size=%u dwords", n);
    }
// must pass
    memset32(klvs, TEST_PAD, ARRAY_SIZE(klvs));
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test,
    xe_guc_klv_encode_object(klvs, GUC_KLV_LEN_MIN + payload,
    key, &obj, obj_raw_encoder));
    KUNIT_EXPECT_EQ(test, klvs[0], PREP_GUC_KLV(key, payload));
    KUNIT_EXPECT_MEMEQ(test, &klvs[1], &obj, sizeof(obj));
// already failed, must fail
    KUNIT_ASSERT_PTR_EQ(test, err,
    xe_guc_klv_encode_object(err, ARRAY_SIZE(klvs), key,
    &obj, obj_raw_encoder));
    }
#[no_mangle]
unsafe extern "C" fn test_encode_object_klv(test: *mut kunit) {
    static void test_encode_object_klv(struct kunit *test)
    {
    const struct some_object obj = {
    .value1 = 0xdead1234,
    .value2 = 0xdead87654321dead,
    };
    let mut key: u16 = TEST_GROUP_KEY;
    let mut payload: u32 = 0;
    u32 klvs[16];
    u32 n;
    payload += GUC_KLV_LEN_MIN + to_num_dwords(sizeof(obj.value1));
    payload += GUC_KLV_LEN_MIN + to_num_dwords(sizeof(obj.value2));
// too small, must fail
    for (n = 0; n < GUC_KLV_LEN_MIN + payload; n++) {
    memset32(klvs, TEST_PAD, ARRAY_SIZE(klvs));
    KUNIT_EXPECT_PTR_EQ_MSG(test, ERR_PTR(-ENOSPC),
    xe_guc_klv_encode_object(klvs, n, key, &obj,
    obj_klv_encoder),
    "buf size=%u dwords", n);
    }
// must pass
    memset32(klvs, TEST_PAD, ARRAY_SIZE(klvs));
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test,
    xe_guc_klv_encode_object(klvs, GUC_KLV_LEN_MIN + payload,
    key, &obj, obj_klv_encoder));
    KUNIT_EXPECT_EQ(test, klvs[0], PREP_GUC_KLV(key, payload));
    KUNIT_EXPECT_EQ(test, klvs[1], PREP_GUC_KLV(TEST_KEY + 1, 1));
    KUNIT_EXPECT_EQ(test, klvs[2], obj.value1);
    KUNIT_EXPECT_EQ(test, klvs[3], PREP_GUC_KLV(TEST_KEY + 2, 2));
    KUNIT_EXPECT_EQ(test, klvs[4], lower_32_bits(obj.value2));
    KUNIT_EXPECT_EQ(test, klvs[5], upper_32_bits(obj.value2));
    KUNIT_EXPECT_EQ(test, klvs[6], TEST_PAD);
    }
#[no_mangle]
unsafe extern "C" fn test_encode_object_nested(test: *mut kunit) {
    static void test_encode_object_nested(struct kunit *test)
    {
    const struct some_object obj = {
    .value1 = 0xdead1234,
    .value2 = 0xdead87654321dead,
    };
    let mut key: u16 = TEST_GROUP_KEY;
    let mut payload: u32 = 0;
    u32 klvs[16];
    u32 n;
    payload += GUC_KLV_LEN_MIN;
    payload += GUC_KLV_LEN_MIN + to_num_dwords(sizeof(obj.value1));
    payload += GUC_KLV_LEN_MIN + to_num_dwords(sizeof(obj.value2));
    payload *= 2;
// too small, must fail
    for (n = 0; n < GUC_KLV_LEN_MIN + payload; n++) {
    memset32(klvs, TEST_PAD, ARRAY_SIZE(klvs));
    KUNIT_EXPECT_PTR_EQ_MSG(test, ERR_PTR(-ENOSPC),
    xe_guc_klv_encode_object(klvs, n, key, &obj,
    obj_nested_encoder),
    "buf size=%u dwords", n);
    }
// must pass
    memset32(klvs, TEST_PAD, ARRAY_SIZE(klvs));
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test,
    xe_guc_klv_encode_object(klvs, GUC_KLV_LEN_MIN + payload,
    key, &obj, obj_nested_encoder));
    KUNIT_EXPECT_EQ(test, klvs[0], PREP_GUC_KLV(key, payload));
    KUNIT_EXPECT_EQ(test, klvs[1], PREP_GUC_KLV(TEST_GROUP_KEY + 1, 5));
    KUNIT_EXPECT_EQ(test, klvs[2], PREP_GUC_KLV(TEST_KEY + 1, 1));
    KUNIT_EXPECT_EQ(test, klvs[3], obj.value1);
    KUNIT_EXPECT_EQ(test, klvs[4], PREP_GUC_KLV(TEST_KEY + 2, 2));
    KUNIT_EXPECT_EQ(test, klvs[5], lower_32_bits(obj.value2));
    KUNIT_EXPECT_EQ(test, klvs[6], upper_32_bits(obj.value2));
    KUNIT_EXPECT_EQ(test, klvs[7], PREP_GUC_KLV(TEST_GROUP_KEY + 2, 5));
    KUNIT_EXPECT_EQ(test, klvs[8], PREP_GUC_KLV(TEST_KEY + 1, 1));
    KUNIT_EXPECT_EQ(test, klvs[9], obj.value1);
    KUNIT_EXPECT_EQ(test, klvs[10], PREP_GUC_KLV(TEST_KEY + 2, 2));
    KUNIT_EXPECT_EQ(test, klvs[11], lower_32_bits(obj.value2));
    KUNIT_EXPECT_EQ(test, klvs[12], upper_32_bits(obj.value2));
    KUNIT_EXPECT_EQ(test, klvs[13], TEST_PAD);
    }
    static u32 *obj_echo_encoder(u32 *klvs, u32 avail, const void *arg)
    {
    return ERR_CAST(arg);
    }
#[no_mangle]
unsafe extern "C" fn test_encode_object_basic(test: *mut kunit) {
    static void test_encode_object_basic(struct kunit *test)
    {
    let mut longest: u32 = GUC_KLV_LEN_MIN + FIELD_MAX(GUC_KLV_0_LEN);
    let mut avail: u32 = GUC_KLV_LEN_MIN + longest;
    let mut key: u16 = TEST_GROUP_KEY;
    u32 *klvs;
    klvs = kunit_kcalloc(test, avail, sizeof(u32), GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, klvs);
// smallest
    KUNIT_EXPECT_PTR_EQ(test, klvs + GUC_KLV_LEN_MIN,
    xe_guc_klv_encode_object(klvs, avail, key,
    klvs + GUC_KLV_LEN_MIN,
    obj_echo_encoder));
// largest
    KUNIT_EXPECT_PTR_EQ(test, klvs + longest,
    xe_guc_klv_encode_object(klvs, avail, key,
    klvs + longest,
    obj_echo_encoder));
// already failed
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-EROFS),
    xe_guc_klv_encode_object(ERR_PTR(-EROFS), avail, key,
    klvs + GUC_KLV_LEN_MIN,
    obj_echo_encoder));
// encoding error
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-EUCLEAN),
    xe_guc_klv_encode_object(klvs, avail, key,
    ERR_PTR(-EUCLEAN),
    obj_echo_encoder));
// no space
    KUNIT_EXPECT_PTR_EQ(test, ERR_PTR(-ENOSPC),
    xe_guc_klv_encode_object(klvs, 0, key,
    klvs + GUC_KLV_LEN_MIN,
    obj_echo_encoder));
    }
#[no_mangle]
unsafe extern "C" fn __drm_printfn_kunit(p: *mut drm_printer, vaf: *mut va_format) {
    static void __drm_printfn_kunit(struct drm_printer *p, struct va_format *vaf)
    {
    struct kunit *test = p.arg;
    kunit_info(test, "%pV", vaf);
    }
#[no_mangle]
unsafe extern "C" fn drm_kunit_printer() -> drm_printer {
    static struct drm_printer drm_kunit_printer(void)
    {
    struct drm_printer p = {
    .printfn = __drm_printfn_kunit,
    .arg = kunit_get_current_test(),
    };
    return p;
    }
#[no_mangle]
unsafe extern "C" fn test_print(test: *mut kunit) {
    static void test_print(struct kunit *test)
    {
    let mut p: drm_printer = drm_kunit_printer();
    u32 zeros[] = { 0, 0, 0, /* padding */ };
    u32 klvs[] = {
    PREP_GUC_KLV(GUC_KLV_OPT_IN_FEATURE_EXT_CAT_ERR_TYPE_KEY, 0),
    PREP_GUC_KLV(GUC_KLV_VF_CFG_NUM_CONTEXTS_KEY, 1), 1234,
    PREP_GUC_KLV(GUC_KLV_VF_CFG_GGTT_SIZE_KEY, 2), 0x4000, 0x0123,
    PREP_GUC_KLV(TEST_KEY, 3), 1, 2, 3,
    PREP_GUC_KLV(TEST_GROUP_KEY, 5),
    PREP_GUC_KLV(TEST_KEY + 1, 1), 1,
    PREP_GUC_KLV(TEST_KEY + 2, 2), 1, 2,
    };
    kunit_activate_static_stub(test, is_group_key, fake_is_group_key);
    xe_guc_klv_print(zeros, ARRAY_SIZE(zeros), &p);
    xe_guc_klv_print(klvs, ARRAY_SIZE(klvs), &p);
    }
    static struct kunit_case guc_klv_helpers_test_cases[] = {
    KUNIT_CASE(test_count),
    KUNIT_CASE(test_encode_u32),
    KUNIT_CASE(test_encode_u64),
    KUNIT_CASE(test_encode_string),
    KUNIT_CASE(test_encode_object_raw),
    KUNIT_CASE(test_encode_object_klv),
    KUNIT_CASE(test_encode_object_nested),
    KUNIT_CASE(test_encode_object_basic),
    KUNIT_CASE(test_print),
    {}
    };
    static struct kunit_suite guc_klv_helpers_suite = {
    .name = "guc_klv_helpers",
    .test_cases = guc_klv_helpers_test_cases,
    };
    kunit_test_suite(guc_klv_helpers_suite);
