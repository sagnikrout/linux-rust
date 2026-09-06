//! Automatically rewritten from C to Rust
//! Source: tools/testing/rbtree/rbtree_test.c
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
// rbtree_test.c: Userspace Red Black Tree test-suite
// Copyright (c) 2025 Wei Yang <richard.weiyang@gmail.com>
//

#[no_mangle]
pub unsafe extern "C" fn usage() -> c_int {
    int usage(void)
    {
    fprintf(stderr, "Userland rbtree test cases\n");
    fprintf(stderr, "  -n: Number of nodes in the rb-tree\n");
    fprintf(stderr, "  -p: Number of iterations modifying the rb-tree\n");
    fprintf(stderr, "  -c: Number of iterations modifying and verifying the rb-tree\n");
    fprintf(stderr, "  -r: Random seed\n");
    exit(-1);
    }
#[no_mangle]
pub unsafe extern "C" fn rbtree_tests() {
    void rbtree_tests(void)
    {
    rbtree_test_init();
    rbtree_test_exit();
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int opt;
    while ((opt = getopt(argc, argv, "n:p:c:r:")) != -1) {
    if (opt == 'n')
    nnodes = strtoul(optarg, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if('p': opt ==) -> else {
    else if (opt == 'p')
    perf_loops = strtoul(optarg, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if('c': opt ==) -> else {
    else if (opt == 'c')
    check_loops = strtoul(optarg, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if('r': opt ==) -> else {
    else if (opt == 'r')
    seed = strtoul(optarg, core::ptr::null_mut(), 0);
    else
    usage();
    }
    rbtree_tests();
    return 0;
    }
