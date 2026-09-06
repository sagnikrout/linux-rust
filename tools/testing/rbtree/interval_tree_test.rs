//! Automatically rewritten from C to Rust
//! Source: tools/testing/rbtree/interval_tree_test.c
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
// interval_tree.c: Userspace Interval Tree test-suite
// Copyright (c) 2025 Wei Yang <richard.weiyang@gmail.com>
//

#[no_mangle]
pub unsafe extern "C" fn usage() -> c_int {
    int usage(void)
    {
    fprintf(stderr, "Userland interval tree test cases\n");
    fprintf(stderr, "  -n: Number of nodes in the interval tree\n");
    fprintf(stderr, "  -p: Number of iterations modifying the tree\n");
    fprintf(stderr, "  -q: Number of searches to the interval tree\n");
    fprintf(stderr, "  -s: Number of iterations searching the tree\n");
    fprintf(stderr, "  -a: Searches will iterate all nodes in the tree\n");
    fprintf(stderr, "  -m: Largest value for the interval's endpoint\n");
    fprintf(stderr, "  -r: Random seed\n");
    exit(-1);
    }
#[no_mangle]
pub unsafe extern "C" fn interval_tree_tests() {
    void interval_tree_tests(void)
    {
    interval_tree_test_init();
    interval_tree_test_exit();
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int opt;
    while ((opt = getopt(argc, argv, "n:p:q:s:am:r:")) != -1) {
    if (opt == 'n')
    nnodes = strtoul(optarg, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if('p': opt ==) -> else {
    else if (opt == 'p')
    perf_loops = strtoul(optarg, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if('q': opt ==) -> else {
    else if (opt == 'q')
    nsearches = strtoul(optarg, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if('s': opt ==) -> else {
    else if (opt == 's')
    search_loops = strtoul(optarg, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if('a': opt ==) -> else {
    else if (opt == 'a')
    search_all = true;
#[no_mangle]
pub unsafe extern "C" fn if('m': opt ==) -> else {
    else if (opt == 'm')
    max_endpoint = strtoul(optarg, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn if('r': opt ==) -> else {
    else if (opt == 'r')
    seed = strtoul(optarg, core::ptr::null_mut(), 0);
    else
    usage();
    }
    maple_tree_init();
    interval_tree_tests();
    return 0;
    }
