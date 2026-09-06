//! Automatically rewritten from C to Rust
//! Source: security/apparmor/policy_unpack_test.c
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
//
// KUnit tests for AppArmor's policy unpack.
//

    (3 + strlen(TEST_STRING_NAME) + 1)

    (TEST_STRING_BUF_OFFSET + 3 + strlen(TEST_STRING_DATA) + 1)

    (TEST_NAMED_U32_BUF_OFFSET + 3 + strlen(TEST_U32_NAME) + 1)

    (TEST_NAMED_U64_BUF_OFFSET + 3 + strlen(TEST_U64_NAME) + 1)

    (TEST_NAMED_BLOB_BUF_OFFSET + 3 + strlen(TEST_BLOB_NAME) + 1)

pub const TEST_ARRAY_SIZE: c_int = 16;

    (TEST_BLOB_BUF_OFFSET + 5 + TEST_BLOB_DATA_SIZE)

    (TEST_NAMED_ARRAY_BUF_OFFSET + 3 + strlen(TEST_ARRAY_NAME) + 1)
    MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct policy_unpack_fixture {
    pub e: *mut aa_ext,
    pub e_size: usize,
}

    static struct aa_ext *build_aa_ext_struct(struct policy_unpack_fixture *puf,
    struct kunit *test, size_t buf_size)
    {
    char *buf;
    struct aa_ext *e;
    buf = kunit_kzalloc(test, buf_size, GFP_USER);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, buf);
    e = kunit_kmalloc(test, sizeof(*e), GFP_USER);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, e);
    e.start = buf;
    e.end = e.start + buf_size;
    e.pos = e.start;
// buf = AA_NAME;
// (buf + 1) = strlen(TEST_STRING_NAME) + 1;
    strscpy(buf + 3, TEST_STRING_NAME, e.end - (void *)(buf + 3));
    buf = e.start + TEST_STRING_BUF_OFFSET;
// buf = AA_STRING;
// (buf + 1) = strlen(TEST_STRING_DATA) + 1;
    strscpy(buf + 3, TEST_STRING_DATA, e.end - (void *)(buf + 3));
    buf = e.start + TEST_NAMED_U32_BUF_OFFSET;
// buf = AA_NAME;
// (buf + 1) = strlen(TEST_U32_NAME) + 1;
    strscpy(buf + 3, TEST_U32_NAME, e.end - (void *)(buf + 3));
// (buf + 3 + strlen(TEST_U32_NAME) + 1) = AA_U32;
    put_unaligned_le32(TEST_U32_DATA, buf + 3 + strlen(TEST_U32_NAME) + 2);
    buf = e.start + TEST_NAMED_U64_BUF_OFFSET;
// buf = AA_NAME;
// (buf + 1) = strlen(TEST_U64_NAME) + 1;
    strscpy(buf + 3, TEST_U64_NAME, e.end - (void *)(buf + 3));
// (buf + 3 + strlen(TEST_U64_NAME) + 1) = AA_U64;
// ((__le64 *)(buf + 3 + strlen(TEST_U64_NAME) + 2)) = cpu_to_le64(TEST_U64_DATA);
    buf = e.start + TEST_NAMED_BLOB_BUF_OFFSET;
// buf = AA_NAME;
// (buf + 1) = strlen(TEST_BLOB_NAME) + 1;
    strscpy(buf + 3, TEST_BLOB_NAME, e.end - (void *)(buf + 3));
// (buf + 3 + strlen(TEST_BLOB_NAME) + 1) = AA_BLOB;
// (buf + 3 + strlen(TEST_BLOB_NAME) + 2) = TEST_BLOB_DATA_SIZE;
    memcpy(buf + 3 + strlen(TEST_BLOB_NAME) + 6,
    TEST_BLOB_DATA, TEST_BLOB_DATA_SIZE);
    buf = e.start + TEST_NAMED_ARRAY_BUF_OFFSET;
// buf = AA_NAME;
// (buf + 1) = strlen(TEST_ARRAY_NAME) + 1;
    strscpy(buf + 3, TEST_ARRAY_NAME, e.end - (void *)(buf + 3));
// (buf + 3 + strlen(TEST_ARRAY_NAME) + 1) = AA_ARRAY;
    put_unaligned_le16(TEST_ARRAY_SIZE, buf + 3 + strlen(TEST_ARRAY_NAME) + 2);
    return e;
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_init(test: *mut kunit) -> c_int {
    static int policy_unpack_test_init(struct kunit *test)
    {
    let mut e_size: usize = TEST_ARRAY_BUF_OFFSET + sizeof(u16) + 1;
    struct policy_unpack_fixture *puf;
    puf = kunit_kmalloc(test, sizeof(*puf), GFP_USER);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, puf);
    puf.e_size = e_size;
    puf.e = build_aa_ext_struct(puf, test, e_size);
    test.priv = puf;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_inbounds_when_inbounds(test: *mut kunit) {
    static void policy_unpack_test_inbounds_when_inbounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    KUNIT_EXPECT_TRUE(test, aa_inbounds(puf.e, 0));
    KUNIT_EXPECT_TRUE(test, aa_inbounds(puf.e, puf.e_size / 2));
    KUNIT_EXPECT_TRUE(test, aa_inbounds(puf.e, puf.e_size));
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_inbounds_when_out_of_bounds(test: *mut kunit) {
    static void policy_unpack_test_inbounds_when_out_of_bounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    KUNIT_EXPECT_FALSE(test, aa_inbounds(puf.e, puf.e_size + 1));
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_array_with_null_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_array_with_null_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    let mut array_size: u16 = 0;
    puf.e.pos += TEST_ARRAY_BUF_OFFSET;
    KUNIT_EXPECT_TRUE(test, aa_unpack_array(puf.e, core::ptr::null_mut(), &array_size));
    KUNIT_EXPECT_EQ(test, array_size, (u16)TEST_ARRAY_SIZE);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_ARRAY_BUF_OFFSET + sizeof(u16) + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_array_with_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_array_with_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char name[] = TEST_ARRAY_NAME;
    let mut array_size: u16 = 0;
    puf.e.pos += TEST_NAMED_ARRAY_BUF_OFFSET;
    KUNIT_EXPECT_TRUE(test, aa_unpack_array(puf.e, name, &array_size));
    KUNIT_EXPECT_EQ(test, array_size, (u16)TEST_ARRAY_SIZE);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_ARRAY_BUF_OFFSET + sizeof(u16) + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_array_out_of_bounds(test: *mut kunit) {
    static void policy_unpack_test_unpack_array_out_of_bounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char name[] = TEST_ARRAY_NAME;
    u16 array_size;
    puf.e.pos += TEST_NAMED_ARRAY_BUF_OFFSET;
    puf.e.end = puf.e.start + TEST_ARRAY_BUF_OFFSET + sizeof(u16);
    KUNIT_EXPECT_FALSE(test, aa_unpack_array(puf.e, name, &array_size));
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_NAMED_ARRAY_BUF_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_blob_with_null_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_blob_with_null_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    char *blob = core::ptr::null_mut();
    size_t size;
    puf.e.pos += TEST_BLOB_BUF_OFFSET;
    size = aa_unpack_blob(puf.e, &blob, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, size, TEST_BLOB_DATA_SIZE);
    KUNIT_EXPECT_TRUE(test,
    memcmp(blob, TEST_BLOB_DATA, TEST_BLOB_DATA_SIZE) == 0);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_blob_with_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_blob_with_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    char *blob = core::ptr::null_mut();
    size_t size;
    puf.e.pos += TEST_NAMED_BLOB_BUF_OFFSET;
    size = aa_unpack_blob(puf.e, &blob, TEST_BLOB_NAME);
    KUNIT_ASSERT_EQ(test, size, TEST_BLOB_DATA_SIZE);
    KUNIT_EXPECT_TRUE(test,
    memcmp(blob, TEST_BLOB_DATA, TEST_BLOB_DATA_SIZE) == 0);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_blob_out_of_bounds(test: *mut kunit) {
    static void policy_unpack_test_unpack_blob_out_of_bounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    char *blob = core::ptr::null_mut();
    void *start;
    int size;
    puf.e.pos += TEST_NAMED_BLOB_BUF_OFFSET;
    start = puf.e.pos;
    puf.e.end = puf.e.start + TEST_BLOB_BUF_OFFSET
    + TEST_BLOB_DATA_SIZE - 1;
    size = aa_unpack_blob(puf.e, &blob, TEST_BLOB_NAME);
    KUNIT_EXPECT_EQ(test, size, 0);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos, start);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_str_with_null_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_str_with_null_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char *string = core::ptr::null_mut();
    size_t size;
    puf.e.pos += TEST_STRING_BUF_OFFSET;
    size = aa_unpack_str(puf.e, &string, core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, size, strlen(TEST_STRING_DATA) + 1);
    KUNIT_EXPECT_STREQ(test, string, TEST_STRING_DATA);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_str_with_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_str_with_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char *string = core::ptr::null_mut();
    size_t size;
    size = aa_unpack_str(puf.e, &string, TEST_STRING_NAME);
    KUNIT_EXPECT_EQ(test, size, strlen(TEST_STRING_DATA) + 1);
    KUNIT_EXPECT_STREQ(test, string, TEST_STRING_DATA);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_str_out_of_bounds(test: *mut kunit) {
    static void policy_unpack_test_unpack_str_out_of_bounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char *string = core::ptr::null_mut();
    void *start = puf.e.pos;
    int size;
    puf.e.end = puf.e.pos + TEST_STRING_BUF_OFFSET
    + strlen(TEST_STRING_DATA) - 1;
    size = aa_unpack_str(puf.e, &string, TEST_STRING_NAME);
    KUNIT_EXPECT_EQ(test, size, 0);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos, start);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_strdup_with_null_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_strdup_with_null_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    char *string = core::ptr::null_mut();
    size_t size;
    puf.e.pos += TEST_STRING_BUF_OFFSET;
    size = aa_unpack_strdup(puf.e, &string, core::ptr::null_mut());
    KUNIT_EXPECT_EQ(test, size, strlen(TEST_STRING_DATA) + 1);
    KUNIT_EXPECT_FALSE(test,
    ((uintptr_t)puf.e.start <= (uintptr_t)string)
    && ((uintptr_t)string <= (uintptr_t)puf.e.end));
    KUNIT_EXPECT_STREQ(test, string, TEST_STRING_DATA);
    kfree(string);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_strdup_with_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_strdup_with_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    char *string = core::ptr::null_mut();
    size_t size;
    size = aa_unpack_strdup(puf.e, &string, TEST_STRING_NAME);
    KUNIT_EXPECT_EQ(test, size, strlen(TEST_STRING_DATA) + 1);
    KUNIT_EXPECT_FALSE(test,
    ((uintptr_t)puf.e.start <= (uintptr_t)string)
    && ((uintptr_t)string <= (uintptr_t)puf.e.end));
    KUNIT_EXPECT_STREQ(test, string, TEST_STRING_DATA);
    kfree(string);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_strdup_out_of_bounds(test: *mut kunit) {
    static void policy_unpack_test_unpack_strdup_out_of_bounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    void *start = puf.e.pos;
    char *string = core::ptr::null_mut();
    int size;
    puf.e.end = puf.e.pos + TEST_STRING_BUF_OFFSET
    + strlen(TEST_STRING_DATA) - 1;
    size = aa_unpack_strdup(puf.e, &string, TEST_STRING_NAME);
    KUNIT_EXPECT_EQ(test, size, 0);
    KUNIT_EXPECT_NULL(test, string);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos, start);
    kfree(string);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_nameX_with_null_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_nameX_with_null_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    bool success;
    puf.e.pos += TEST_U32_BUF_OFFSET;
    success = aa_unpack_nameX(puf.e, AA_U32, core::ptr::null_mut());
    KUNIT_EXPECT_TRUE(test, success);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_U32_BUF_OFFSET + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_nameX_with_wrong_code(test: *mut kunit) {
    static void policy_unpack_test_unpack_nameX_with_wrong_code(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    bool success;
    puf.e.pos += TEST_U32_BUF_OFFSET;
    success = aa_unpack_nameX(puf.e, AA_BLOB, core::ptr::null_mut());
    KUNIT_EXPECT_FALSE(test, success);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_U32_BUF_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_nameX_with_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_nameX_with_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char name[] = TEST_U32_NAME;
    bool success;
    puf.e.pos += TEST_NAMED_U32_BUF_OFFSET;
    success = aa_unpack_nameX(puf.e, AA_U32, name);
    KUNIT_EXPECT_TRUE(test, success);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_U32_BUF_OFFSET + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_nameX_with_wrong_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_nameX_with_wrong_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    static const char name[] = "12345678";
    bool success;
    puf.e.pos += TEST_NAMED_U32_BUF_OFFSET;
    success = aa_unpack_nameX(puf.e, AA_U32, name);
    KUNIT_EXPECT_FALSE(test, success);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_NAMED_U32_BUF_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_u16_chunk_basic(test: *mut kunit) {
    static void policy_unpack_test_unpack_u16_chunk_basic(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    char *chunk = core::ptr::null_mut();
    size_t size;
    puf.e.pos += TEST_U16_OFFSET;
//
// WARNING: For unit testing purposes, we're pushing puf->e->end past
// the end of the allocated memory. Doing anything other than comparing
// memory addresses is dangerous.
//
    puf.e.end += TEST_U16_DATA;
    size = aa_unpack_u16_chunk(puf.e, &chunk);
    KUNIT_EXPECT_PTR_EQ(test, chunk,
    puf.e.start + TEST_U16_OFFSET + 2);
    KUNIT_EXPECT_EQ(test, size, TEST_U16_DATA);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos, (chunk + TEST_U16_DATA));
    }
    static void policy_unpack_test_unpack_u16_chunk_out_of_bounds_1(
    struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    char *chunk = core::ptr::null_mut();
    size_t size;
    puf.e.pos = puf.e.end - 1;
    size = aa_unpack_u16_chunk(puf.e, &chunk);
    KUNIT_EXPECT_EQ(test, size, 0);
    KUNIT_EXPECT_NULL(test, chunk);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos, puf.e.end - 1);
    }
    static void policy_unpack_test_unpack_u16_chunk_out_of_bounds_2(
    struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    char *chunk = core::ptr::null_mut();
    size_t size;
    puf.e.pos += TEST_U16_OFFSET;
//
// WARNING: For unit testing purposes, we're pushing puf->e->end past
// the end of the allocated memory. Doing anything other than comparing
// memory addresses is dangerous.
//
    puf.e.end = puf.e.pos + TEST_U16_DATA - 1;
    size = aa_unpack_u16_chunk(puf.e, &chunk);
    KUNIT_EXPECT_EQ(test, size, 0);
    KUNIT_EXPECT_NULL(test, chunk);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos, puf.e.start + TEST_U16_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_u32_with_null_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_u32_with_null_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    bool success;
    let mut data: u32 = 0;
    puf.e.pos += TEST_U32_BUF_OFFSET;
    success = aa_unpack_u32(puf.e, &data, core::ptr::null_mut());
    KUNIT_EXPECT_TRUE(test, success);
    KUNIT_EXPECT_EQ(test, data, TEST_U32_DATA);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_U32_BUF_OFFSET + sizeof(u32) + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_u32_with_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_u32_with_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char name[] = TEST_U32_NAME;
    bool success;
    let mut data: u32 = 0;
    puf.e.pos += TEST_NAMED_U32_BUF_OFFSET;
    success = aa_unpack_u32(puf.e, &data, name);
    KUNIT_EXPECT_TRUE(test, success);
    KUNIT_EXPECT_EQ(test, data, TEST_U32_DATA);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_U32_BUF_OFFSET + sizeof(u32) + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_u32_out_of_bounds(test: *mut kunit) {
    static void policy_unpack_test_unpack_u32_out_of_bounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char name[] = TEST_U32_NAME;
    bool success;
    let mut data: u32 = 0;
    puf.e.pos += TEST_NAMED_U32_BUF_OFFSET;
    puf.e.end = puf.e.start + TEST_U32_BUF_OFFSET + sizeof(u32);
    success = aa_unpack_u32(puf.e, &data, name);
    KUNIT_EXPECT_FALSE(test, success);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_NAMED_U32_BUF_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_u64_with_null_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_u64_with_null_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    bool success;
    let mut data: u64 = 0;
    puf.e.pos += TEST_U64_BUF_OFFSET;
    success = aa_unpack_u64(puf.e, &data, core::ptr::null_mut());
    KUNIT_EXPECT_TRUE(test, success);
    KUNIT_EXPECT_EQ(test, data, TEST_U64_DATA);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_U64_BUF_OFFSET + sizeof(u64) + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_u64_with_name(test: *mut kunit) {
    static void policy_unpack_test_unpack_u64_with_name(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char name[] = TEST_U64_NAME;
    bool success;
    let mut data: u64 = 0;
    puf.e.pos += TEST_NAMED_U64_BUF_OFFSET;
    success = aa_unpack_u64(puf.e, &data, name);
    KUNIT_EXPECT_TRUE(test, success);
    KUNIT_EXPECT_EQ(test, data, TEST_U64_DATA);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_U64_BUF_OFFSET + sizeof(u64) + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_u64_out_of_bounds(test: *mut kunit) {
    static void policy_unpack_test_unpack_u64_out_of_bounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    const char name[] = TEST_U64_NAME;
    bool success;
    let mut data: u64 = 0;
    puf.e.pos += TEST_NAMED_U64_BUF_OFFSET;
    puf.e.end = puf.e.start + TEST_U64_BUF_OFFSET + sizeof(u64);
    success = aa_unpack_u64(puf.e, &data, name);
    KUNIT_EXPECT_FALSE(test, success);
    KUNIT_EXPECT_PTR_EQ(test, puf.e.pos,
    puf.e.start + TEST_NAMED_U64_BUF_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_X_code_match(test: *mut kunit) {
    static void policy_unpack_test_unpack_X_code_match(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    let mut success: bool = aa_unpack_X(puf.e, AA_NAME);
    KUNIT_EXPECT_TRUE(test, success);
    KUNIT_EXPECT_TRUE(test, puf.e.pos == puf.e.start + 1);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_X_code_mismatch(test: *mut kunit) {
    static void policy_unpack_test_unpack_X_code_mismatch(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    let mut success: bool = aa_unpack_X(puf.e, AA_STRING);
    KUNIT_EXPECT_FALSE(test, success);
    KUNIT_EXPECT_TRUE(test, puf.e.pos == puf.e.start);
    }
#[no_mangle]
unsafe extern "C" fn policy_unpack_test_unpack_X_out_of_bounds(test: *mut kunit) {
    static void policy_unpack_test_unpack_X_out_of_bounds(struct kunit *test)
    {
    struct policy_unpack_fixture *puf = test.priv;
    bool success;
    puf.e.pos = puf.e.end;
    success = aa_unpack_X(puf.e, AA_NAME);
    KUNIT_EXPECT_FALSE(test, success);
    }
    static struct kunit_case apparmor_policy_unpack_test_cases[] = {
    KUNIT_CASE(policy_unpack_test_inbounds_when_inbounds),
    KUNIT_CASE(policy_unpack_test_inbounds_when_out_of_bounds),
    KUNIT_CASE(policy_unpack_test_unpack_array_with_null_name),
    KUNIT_CASE(policy_unpack_test_unpack_array_with_name),
    KUNIT_CASE(policy_unpack_test_unpack_array_out_of_bounds),
    KUNIT_CASE(policy_unpack_test_unpack_blob_with_null_name),
    KUNIT_CASE(policy_unpack_test_unpack_blob_with_name),
    KUNIT_CASE(policy_unpack_test_unpack_blob_out_of_bounds),
    KUNIT_CASE(policy_unpack_test_unpack_nameX_with_null_name),
    KUNIT_CASE(policy_unpack_test_unpack_nameX_with_wrong_code),
    KUNIT_CASE(policy_unpack_test_unpack_nameX_with_name),
    KUNIT_CASE(policy_unpack_test_unpack_nameX_with_wrong_name),
    KUNIT_CASE(policy_unpack_test_unpack_str_with_null_name),
    KUNIT_CASE(policy_unpack_test_unpack_str_with_name),
    KUNIT_CASE(policy_unpack_test_unpack_str_out_of_bounds),
    KUNIT_CASE(policy_unpack_test_unpack_strdup_with_null_name),
    KUNIT_CASE(policy_unpack_test_unpack_strdup_with_name),
    KUNIT_CASE(policy_unpack_test_unpack_strdup_out_of_bounds),
    KUNIT_CASE(policy_unpack_test_unpack_u16_chunk_basic),
    KUNIT_CASE(policy_unpack_test_unpack_u16_chunk_out_of_bounds_1),
    KUNIT_CASE(policy_unpack_test_unpack_u16_chunk_out_of_bounds_2),
    KUNIT_CASE(policy_unpack_test_unpack_u32_with_null_name),
    KUNIT_CASE(policy_unpack_test_unpack_u32_with_name),
    KUNIT_CASE(policy_unpack_test_unpack_u32_out_of_bounds),
    KUNIT_CASE(policy_unpack_test_unpack_u64_with_null_name),
    KUNIT_CASE(policy_unpack_test_unpack_u64_with_name),
    KUNIT_CASE(policy_unpack_test_unpack_u64_out_of_bounds),
    KUNIT_CASE(policy_unpack_test_unpack_X_code_match),
    KUNIT_CASE(policy_unpack_test_unpack_X_code_mismatch),
    KUNIT_CASE(policy_unpack_test_unpack_X_out_of_bounds),
    {},
    };
    static struct kunit_suite apparmor_policy_unpack_test_module = {
    .name = "apparmor_policy_unpack",
    .init = policy_unpack_test_init,
    .test_cases = apparmor_policy_unpack_test_cases,
    };
    kunit_test_suite(apparmor_policy_unpack_test_module);
    MODULE_DESCRIPTION("KUnit tests for AppArmor's policy unpack");
    MODULE_LICENSE("GPL");
