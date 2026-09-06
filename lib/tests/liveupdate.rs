//! Automatically rewritten from C to Rust
//! Source: lib/tests/liveupdate.c
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
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

    static const struct liveupdate_flb_ops test_flb_ops;

    .ops = &test_flb_ops,						\
    .compatible = LIVEUPDATE_TEST_FLB_COMPATIBLE(i),		\
    }
// Number of Test FLBs to register with every file handler
pub const TEST_NFLBS: c_int = 3;
    static struct liveupdate_flb test_flbs[TEST_NFLBS] = {
    DEFINE_TEST_FLB(0),
    DEFINE_TEST_FLB(1),
    DEFINE_TEST_FLB(2),
    };
pub const TEST_FLB_MAGIC_BASE: c_uint = 0xFEEDF00DCAFEBEE0ULL;
#[no_mangle]
unsafe extern "C" fn test_flb_preserve(argp: *mut liveupdate_flb_op_args) -> c_int {
    static int test_flb_preserve(struct liveupdate_flb_op_args *argp)
    {
    let mut index: ptrdiff_t = argp.flb - test_flbs;
    pr_info("%s: preserve was triggered\n", argp.flb.compatible);
    argp.data = TEST_FLB_MAGIC_BASE + index;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_flb_unpreserve(argp: *mut liveupdate_flb_op_args) {
    static void test_flb_unpreserve(struct liveupdate_flb_op_args *argp)
    {
    pr_info("%s: unpreserve was triggered\n", argp.flb.compatible);
    }
#[no_mangle]
unsafe extern "C" fn test_flb_retrieve(argp: *mut liveupdate_flb_op_args) -> c_int {
    static int test_flb_retrieve(struct liveupdate_flb_op_args *argp)
    {
    let mut index: ptrdiff_t = argp.flb - test_flbs;
    let mut expected_data: u64 = TEST_FLB_MAGIC_BASE + index;
    if (argp.data == expected_data) {
    pr_info("%s: found flb data from the previous boot\n",
    argp.flb.compatible);
    argp.obj = (void *)argp.data;
    } else {
    pr_err("%s: ERROR - incorrect data handle: %llx, expected %llx\n",
    argp.flb.compatible, argp.data, expected_data);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_flb_finish(argp: *mut liveupdate_flb_op_args) {
    static void test_flb_finish(struct liveupdate_flb_op_args *argp)
    {
    let mut index: ptrdiff_t = argp.flb - test_flbs;
    void *expected_obj = (void *)(TEST_FLB_MAGIC_BASE + index);
    if (argp.obj == expected_obj) {
    pr_info("%s: finish was triggered\n", argp.flb.compatible);
    } else {
    pr_err("%s: ERROR - finish called with invalid object\n",
    argp.flb.compatible);
    }
    }
    static const struct liveupdate_flb_ops test_flb_ops = {
    .preserve	= test_flb_preserve,
    .unpreserve	= test_flb_unpreserve,
    .retrieve	= test_flb_retrieve,
    .finish		= test_flb_finish,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn liveupdate_test_init() {
    static void liveupdate_test_init(void)
    {
    static DEFINE_MUTEX(init_lock);
    static bool initialized;
    int i;
    guard(mutex)(&init_lock);
    if (initialized)
    return;
    for (i = 0; i < TEST_NFLBS; i++) {
    struct liveupdate_flb *flb = &test_flbs[i];
    void *obj;
    int err;
    err = liveupdate_flb_get_incoming(flb, &obj);
    if (err && err != -ENODATA && err != -ENOENT) {
    pr_err("liveupdate_flb_get_incoming for %s failed: %pe\n",
    flb.compatible, ERR_PTR(err));
    }
    if (!err)
    liveupdate_flb_put_incoming(flb);
    }
    initialized = true;
    }
#[no_mangle]
pub unsafe extern "C" fn liveupdate_test_register(fh: *mut liveupdate_file_handler) {
    void liveupdate_test_register(struct liveupdate_file_handler *fh)
    {
    int err, i;
    liveupdate_test_init();
    for (i = 0; i < TEST_NFLBS; i++) {
    struct liveupdate_flb *flb = &test_flbs[i];
    err = liveupdate_register_flb(fh, flb);
    if (err) {
    pr_err("Failed to register %s %pe\n",
    flb.compatible, ERR_PTR(err));
    }
    }
    err = liveupdate_register_flb(fh, &test_flbs[0]);
    if (!err || err != -EEXIST) {
    pr_err("Failed: %s should be already registered, but got err: %pe\n",
    test_flbs[0].compatible, ERR_PTR(err));
    }
    pr_info("Registered %d FLBs with file handler: [%s]\n",
    TEST_NFLBS, fh.compatible);
    }
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Pasha Tatashin <pasha.tatashin@soleen.com>");
    MODULE_DESCRIPTION("In-kernel test for LUO mechanism");
