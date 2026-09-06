//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/lsm/lsm_get_self_attr_test.c
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
// Linux Security Module infrastructure tests
// Tests for the lsm_get_self_attr system call
//
// Copyright © 2022 Casey Schaufler <casey@schaufler-ca.com>
//
// Macro flag: #define _GNU_SOURCE

    static struct lsm_ctx *next_ctx(struct lsm_ctx *ctxp)
    {
    void *vp;
    vp = (void *)ctxp + sizeof(*ctxp) + ctxp.ctx_len;
    return (struct lsm_ctx *)vp;
    }
    TEST(size_null_lsm_get_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    struct lsm_ctx *ctx = calloc(page_size, 1);
    ASSERT_NE(core::ptr::null_mut(), ctx);
    errno = 0;
    ASSERT_EQ(-1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, core::ptr::null_mut(), 0));
    ASSERT_EQ(EINVAL, errno);
    free(ctx);
    }
    TEST(ctx_null_lsm_get_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    let mut size: __u32 = page_size;
    int rc;
    rc = lsm_get_self_attr(LSM_ATTR_CURRENT, core::ptr::null_mut(), &size, 0);
    if (attr_lsm_count()) {
    ASSERT_NE(-1, rc);
    ASSERT_NE(1, size);
    } else {
    ASSERT_EQ(-1, rc);
    }
    }
    TEST(size_too_small_lsm_get_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    struct lsm_ctx *ctx = calloc(page_size, 1);
    let mut size: __u32 = 1;
    ASSERT_NE(core::ptr::null_mut(), ctx);
    errno = 0;
    ASSERT_EQ(-1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &size, 0));
    if (attr_lsm_count()) {
    ASSERT_EQ(E2BIG, errno);
    } else {
    ASSERT_EQ(EOPNOTSUPP, errno);
    }
    ASSERT_NE(1, size);
    free(ctx);
    }
    TEST(flags_zero_lsm_get_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    struct lsm_ctx *ctx = calloc(page_size, 1);
    __u64 *syscall_lsms = calloc(page_size, 1);
    __u32 size;
    int lsmcount;
    int i;
    ASSERT_NE(core::ptr::null_mut(), ctx);
    errno = 0;
    size = page_size;
    ASSERT_EQ(-1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &size,
    LSM_FLAG_SINGLE));
    ASSERT_EQ(EINVAL, errno);
    ASSERT_EQ(page_size, size);
    lsmcount = syscall(__NR_lsm_list_modules, syscall_lsms, &size, 0);
    ASSERT_LE(1, lsmcount);
    ASSERT_NE(core::ptr::null_mut(), syscall_lsms);
    for (i = 0; i < lsmcount; i++) {
    errno = 0;
    size = page_size;
    ctx.id = syscall_lsms[i];
    if (syscall_lsms[i] == LSM_ID_SELINUX ||
    syscall_lsms[i] == LSM_ID_SMACK ||
    syscall_lsms[i] == LSM_ID_APPARMOR) {
    ASSERT_EQ(1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx,
    &size, LSM_FLAG_SINGLE));
    } else {
    ASSERT_EQ(-1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx,
    &size,
    LSM_FLAG_SINGLE));
    }
    }
    free(ctx);
    }
    TEST(flags_overset_lsm_get_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    struct lsm_ctx *ctx = calloc(page_size, 1);
    __u32 size;
    ASSERT_NE(core::ptr::null_mut(), ctx);
    errno = 0;
    size = page_size;
    ASSERT_EQ(-1, lsm_get_self_attr(LSM_ATTR_CURRENT | LSM_ATTR_PREV, ctx,
    &size, 0));
    ASSERT_EQ(EOPNOTSUPP, errno);
    errno = 0;
    size = page_size;
    ASSERT_EQ(-1, lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &size,
    LSM_FLAG_SINGLE |
    (LSM_FLAG_SINGLE << 1)));
    ASSERT_EQ(EINVAL, errno);
    free(ctx);
    }
    TEST(basic_lsm_get_self_attr)
    {
    let mut page_size: c_long = sysconf(_SC_PAGESIZE);
    let mut size: __u32 = page_size;
    struct lsm_ctx *ctx = calloc(page_size, 1);
    struct lsm_ctx *tctx = core::ptr::null_mut();
    __u64 *syscall_lsms = calloc(page_size, 1);
    char *attr = calloc(page_size, 1);
    let mut cnt_current: c_int = 0;
    let mut cnt_exec: c_int = 0;
    let mut cnt_fscreate: c_int = 0;
    let mut cnt_keycreate: c_int = 0;
    let mut cnt_prev: c_int = 0;
    let mut cnt_sockcreate: c_int = 0;
    int lsmcount;
    int count;
    int i;
    ASSERT_NE(core::ptr::null_mut(), ctx);
    ASSERT_NE(core::ptr::null_mut(), syscall_lsms);
    lsmcount = syscall(__NR_lsm_list_modules, syscall_lsms, &size, 0);
    ASSERT_LE(1, lsmcount);
    for (i = 0; i < lsmcount; i++) {
    switch (syscall_lsms[i]) {
    case LSM_ID_SELINUX:
    cnt_current++;
    cnt_exec++;
    cnt_fscreate++;
    cnt_keycreate++;
    cnt_prev++;
    cnt_sockcreate++;
    break;
    case LSM_ID_SMACK:
    cnt_current++;
    break;
    case LSM_ID_APPARMOR:
    cnt_current++;
    cnt_exec++;
    cnt_prev++;
    break;
    default:
    break;
    }
    }
    if (cnt_current) {
    size = page_size;
    count = lsm_get_self_attr(LSM_ATTR_CURRENT, ctx, &size, 0);
    ASSERT_EQ(cnt_current, count);
    tctx = ctx;
    ASSERT_EQ(0, read_proc_attr("current", attr, page_size));
    ASSERT_EQ(0, strcmp((char *)tctx.ctx, attr));
    for (i = 1; i < count; i++) {
    tctx = next_ctx(tctx);
    ASSERT_NE(0, strcmp((char *)tctx.ctx, attr));
    }
    }
    if (cnt_exec) {
    size = page_size;
    count = lsm_get_self_attr(LSM_ATTR_EXEC, ctx, &size, 0);
    ASSERT_GE(cnt_exec, count);
    if (count > 0) {
    tctx = ctx;
    if (read_proc_attr("exec", attr, page_size) == 0)
    ASSERT_EQ(0, strcmp((char *)tctx.ctx, attr));
    }
    for (i = 1; i < count; i++) {
    tctx = next_ctx(tctx);
    ASSERT_NE(0, strcmp((char *)tctx.ctx, attr));
    }
    }
    if (cnt_fscreate) {
    size = page_size;
    count = lsm_get_self_attr(LSM_ATTR_FSCREATE, ctx, &size, 0);
    ASSERT_GE(cnt_fscreate, count);
    if (count > 0) {
    tctx = ctx;
    if (read_proc_attr("fscreate", attr, page_size) == 0)
    ASSERT_EQ(0, strcmp((char *)tctx.ctx, attr));
    }
    for (i = 1; i < count; i++) {
    tctx = next_ctx(tctx);
    ASSERT_NE(0, strcmp((char *)tctx.ctx, attr));
    }
    }
    if (cnt_keycreate) {
    size = page_size;
    count = lsm_get_self_attr(LSM_ATTR_KEYCREATE, ctx, &size, 0);
    ASSERT_GE(cnt_keycreate, count);
    if (count > 0) {
    tctx = ctx;
    if (read_proc_attr("keycreate", attr, page_size) == 0)
    ASSERT_EQ(0, strcmp((char *)tctx.ctx, attr));
    }
    for (i = 1; i < count; i++) {
    tctx = next_ctx(tctx);
    ASSERT_NE(0, strcmp((char *)tctx.ctx, attr));
    }
    }
    if (cnt_prev) {
    size = page_size;
    count = lsm_get_self_attr(LSM_ATTR_PREV, ctx, &size, 0);
    ASSERT_GE(cnt_prev, count);
    if (count > 0) {
    tctx = ctx;
    ASSERT_EQ(0, read_proc_attr("prev", attr, page_size));
    ASSERT_EQ(0, strcmp((char *)tctx.ctx, attr));
    for (i = 1; i < count; i++) {
    tctx = next_ctx(tctx);
    ASSERT_NE(0, strcmp((char *)tctx.ctx, attr));
    }
    }
    }
    if (cnt_sockcreate) {
    size = page_size;
    count = lsm_get_self_attr(LSM_ATTR_SOCKCREATE, ctx, &size, 0);
    ASSERT_GE(cnt_sockcreate, count);
    if (count > 0) {
    tctx = ctx;
    if (read_proc_attr("sockcreate", attr, page_size) == 0)
    ASSERT_EQ(0, strcmp((char *)tctx.ctx, attr));
    }
    for (i = 1; i < count; i++) {
    tctx = next_ctx(tctx);
    ASSERT_NE(0, strcmp((char *)tctx.ctx, attr));
    }
    }
    free(ctx);
    free(attr);
    free(syscall_lsms);
    }
    TEST_HARNESS_MAIN
