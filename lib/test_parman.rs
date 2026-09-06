//! Automatically rewritten from C to Rust
//! Source: lib/test_parman.c
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


//
// lib/test_parman.c - Test module for parman
// Copyright (c) 2017 Mellanox Technologies. All rights reserved.
// Copyright (c) 2017 Jiri Pirko <jiri@mellanox.com>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the names of the copyright holders nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL") version 2 as published by the Free
// Software Foundation.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//

// of items for testing
//

pub const TEST_PARMAN_BASE_SHIFT: c_int = 8;

pub const TEST_PARMAN_RESIZE_STEP_SHIFT: c_int = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_parman_prio {
    pub parman_prio: parman_prio,
    pub priority: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_parman_item {
    pub parman_item: parman_item,
    pub prio: *mut test_parman_prio,
    pub used: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_parman {
    pub parman: *mut parman,
    pub prio_array: *mut test_parman_item,
    pub prio_array_limit: c_ulong,
    pub prios: [test_parman_prio; TEST_PARMAN_PRIO_COUNT],
    pub items: [test_parman_item; TEST_PARMAN_ITEM_COUNT],
    pub rnd: rnd_state,
    pub run_budget: c_ulong,
    pub bulk_budget: c_ulong,
    pub bulk_noop: bool,
    pub used_items: c_uint,
}

#[no_mangle]
unsafe extern "C" fn test_parman_resize(priv: *mut c_void, new_count: c_ulong) -> c_int {
    static int test_parman_resize(void *priv, unsigned long new_count)
    {
    struct test_parman *test_parman = priv;
    struct test_parman_item **prio_array;
    unsigned long old_count;
    prio_array = krealloc(test_parman.prio_array,
    ITEM_PTRS_SIZE(new_count), GFP_KERNEL);
    if (new_count == 0)
    return 0;
    if (!prio_array)
    return -ENOMEM;
    old_count = test_parman.prio_array_limit;
    if (new_count > old_count)
    memset(&prio_array[old_count], 0,
    ITEM_PTRS_SIZE(new_count - old_count));
    test_parman.prio_array = prio_array;
    test_parman.prio_array_limit = new_count;
    return 0;
    }
    static void test_parman_move(void *priv, unsigned long from_index,
    unsigned long to_index, unsigned long count)
    {
    struct test_parman *test_parman = priv;
    struct test_parman_item **prio_array = test_parman.prio_array;
    memmove(&prio_array[to_index], &prio_array[from_index],
    ITEM_PTRS_SIZE(count));
    memset(&prio_array[from_index], 0, ITEM_PTRS_SIZE(count));
    }
    static const struct parman_ops test_parman_lsort_ops = {
    .base_count	= TEST_PARMAN_BASE_COUNT,
    .resize_step	= TEST_PARMAN_RESIZE_STEP_COUNT,
    .resize		= test_parman_resize,
    .move		= test_parman_move,
    .algo		= PARMAN_ALGO_TYPE_LSORT,
    };
#[no_mangle]
unsafe extern "C" fn test_parman_rnd_init(test_parman: *mut test_parman) {
    static void test_parman_rnd_init(struct test_parman *test_parman)
    {
    prandom_seed_state(&test_parman.rnd, 3141592653589793238ULL);
    }
#[no_mangle]
unsafe extern "C" fn test_parman_rnd_get(test_parman: *mut test_parman) -> u32 {
    static u32 test_parman_rnd_get(struct test_parman *test_parman)
    {
    return prandom_u32_state(&test_parman.rnd);
    }
#[no_mangle]
unsafe extern "C" fn test_parman_priority_gen(test_parman: *mut test_parman) -> c_ulong {
    static unsigned long test_parman_priority_gen(struct test_parman *test_parman)
    {
    unsigned long priority;
    int i;
    again:
    priority = test_parman_rnd_get(test_parman);
    if (priority == 0)
    goto again;
    for (i = 0; i < TEST_PARMAN_PRIO_COUNT; i++) {
    struct test_parman_prio *prio = &test_parman.prios[i];
    if (prio.priority == 0)
    break;
    if (prio.priority == priority)
    goto again;
    }
    return priority;
    }
#[no_mangle]
unsafe extern "C" fn test_parman_prios_init(test_parman: *mut test_parman) {
    static void test_parman_prios_init(struct test_parman *test_parman)
    {
    int i;
    for (i = 0; i < TEST_PARMAN_PRIO_COUNT; i++) {
    struct test_parman_prio *prio = &test_parman.prios[i];
// Assign random uniqueue priority to each prio structure
    prio.priority = test_parman_priority_gen(test_parman);
    parman_prio_init(test_parman.parman, &prio.parman_prio,
    prio.priority);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_parman_prios_fini(test_parman: *mut test_parman) {
    static void test_parman_prios_fini(struct test_parman *test_parman)
    {
    int i;
    for (i = 0; i < TEST_PARMAN_PRIO_COUNT; i++) {
    struct test_parman_prio *prio = &test_parman.prios[i];
    parman_prio_fini(&prio.parman_prio);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_parman_items_init(test_parman: *mut test_parman) {
    static void test_parman_items_init(struct test_parman *test_parman)
    {
    int i;
    for (i = 0; i < TEST_PARMAN_ITEM_COUNT; i++) {
    struct test_parman_item *item = &test_parman.items[i];
    unsigned int prio_index = test_parman_rnd_get(test_parman) &
    TEST_PARMAN_PRIO_MASK;
// Assign random prio to each item structure
    item.prio = &test_parman.prios[prio_index];
    }
    }
#[no_mangle]
unsafe extern "C" fn test_parman_items_fini(test_parman: *mut test_parman) {
    static void test_parman_items_fini(struct test_parman *test_parman)
    {
    int i;
    for (i = 0; i < TEST_PARMAN_ITEM_COUNT; i++) {
    struct test_parman_item *item = &test_parman.items[i];
    if (!item.used)
    continue;
    parman_item_remove(test_parman.parman,
    &item.prio.parman_prio,
    &item.parman_item);
    }
    }
    static struct test_parman *test_parman_create(const struct parman_ops *ops)
    {
    struct test_parman *test_parman;
    int err;
    test_parman = kzalloc_obj(*test_parman);
    if (!test_parman)
    return ERR_PTR(-ENOMEM);
    err = test_parman_resize(test_parman, TEST_PARMAN_BASE_COUNT);
    if (err)
    goto err_resize;
    test_parman.parman = parman_create(ops, test_parman);
    if (!test_parman.parman) {
    err = -ENOMEM;
    goto err_parman_create;
    }
    test_parman_rnd_init(test_parman);
    test_parman_prios_init(test_parman);
    test_parman_items_init(test_parman);
    test_parman.run_budget = TEST_PARMAN_RUN_BUDGET;
    return test_parman;
    err_parman_create:
    test_parman_resize(test_parman, 0);
    err_resize:
    kfree(test_parman);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn test_parman_destroy(test_parman: *mut test_parman) {
    static void test_parman_destroy(struct test_parman *test_parman)
    {
    test_parman_items_fini(test_parman);
    test_parman_prios_fini(test_parman);
    parman_destroy(test_parman.parman);
    test_parman_resize(test_parman, 0);
    kfree(test_parman);
    }
#[no_mangle]
unsafe extern "C" fn test_parman_run_check_budgets(test_parman: *mut test_parman) -> bool {
    static bool test_parman_run_check_budgets(struct test_parman *test_parman)
    {
    if (test_parman.run_budget-- == 0)
    return false;
    if (test_parman.bulk_budget-- != 0)
    return true;
    test_parman.bulk_budget = test_parman_rnd_get(test_parman) &
    TEST_PARMAN_BULK_MAX_MASK;
    test_parman.bulk_noop = test_parman_rnd_get(test_parman) & 1;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn test_parman_run(test_parman: *mut test_parman) -> c_int {
    static int test_parman_run(struct test_parman *test_parman)
    {
    let mut i: c_uint = test_parman_rnd_get(test_parman);
    int err;
    while (test_parman_run_check_budgets(test_parman)) {
    let mut item_index: c_uint = i++ & TEST_PARMAN_ITEM_MASK;
    struct test_parman_item *item = &test_parman.items[item_index];
    if (test_parman.bulk_noop)
    continue;
    if (!item.used) {
    err = parman_item_add(test_parman.parman,
    &item.prio.parman_prio,
    &item.parman_item);
    if (err)
    return err;
    test_parman.prio_array[item.parman_item.index] = item;
    test_parman.used_items++;
    } else {
    test_parman.prio_array[item.parman_item.index] = core::ptr::null_mut();
    parman_item_remove(test_parman.parman,
    &item.prio.parman_prio,
    &item.parman_item);
    test_parman.used_items--;
    }
    item.used = !item.used;
    }
    return 0;
    }
    static int test_parman_check_array(struct test_parman *test_parman,
    bool gaps_allowed)
    {
    let mut last_unused_items: c_uint = 0;
    let mut last_priority: c_ulong = 0;
    let mut used_items: c_uint = 0;
    int i;
    if (test_parman.prio_array_limit < TEST_PARMAN_BASE_COUNT) {
    pr_err("Array limit is lower than the base count (%lu < %lu)\n",
    test_parman.prio_array_limit, TEST_PARMAN_BASE_COUNT);
    return -EINVAL;
    }
    for (i = 0; i < test_parman.prio_array_limit; i++) {
    struct test_parman_item *item = test_parman.prio_array[i];
    if (!item) {
    last_unused_items++;
    continue;
    }
    if (last_unused_items && !gaps_allowed) {
    pr_err("Gap found in array even though they are forbidden\n");
    return -EINVAL;
    }
    last_unused_items = 0;
    used_items++;
    if (item.prio.priority < last_priority) {
    pr_err("Item belongs under higher priority then the last one (current: %lu, previous: %lu)\n",
    item.prio.priority, last_priority);
    return -EINVAL;
    }
    last_priority = item.prio.priority;
    if (item.parman_item.index != i) {
    pr_err("Item has different index in compare to where it actually is (%lu != %d)\n",
    item.parman_item.index, i);
    return -EINVAL;
    }
    }
    if (used_items != test_parman.used_items) {
    pr_err("Number of used items in array does not match (%u != %u)\n",
    used_items, test_parman.used_items);
    return -EINVAL;
    }
    if (last_unused_items >= TEST_PARMAN_RESIZE_STEP_COUNT) {
    pr_err("Number of unused item at the end of array is bigger than resize step (%u >= %lu)\n",
    last_unused_items, TEST_PARMAN_RESIZE_STEP_COUNT);
    return -EINVAL;
    }
    pr_info("Priority array check successful\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_parman_lsort() -> c_int {
    static int test_parman_lsort(void)
    {
    struct test_parman *test_parman;
    int err;
    test_parman = test_parman_create(&test_parman_lsort_ops);
    if (IS_ERR(test_parman))
    return PTR_ERR(test_parman);
    err = test_parman_run(test_parman);
    if (err)
    goto out;
    err = test_parman_check_array(test_parman, false);
    if (err)
    goto out;
    out:
    test_parman_destroy(test_parman);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_parman_init() -> int __init {
    static int __init test_parman_init(void)
    {
    return test_parman_lsort();
    }
#[no_mangle]
unsafe extern "C" fn test_parman_exit() -> void __exit {
    static void __exit test_parman_exit(void)
    {
    }
    module_init(test_parman_init);
    module_exit(test_parman_exit);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_AUTHOR("Jiri Pirko <jiri@mellanox.com>");
    MODULE_DESCRIPTION("Test module for parman");
