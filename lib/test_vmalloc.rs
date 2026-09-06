//! Automatically rewritten from C to Rust
//! Source: lib/test_vmalloc.c
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
// Test module for stress and analyze performance of vmalloc allocator.
// (C) 2018 Uladzislau Rezki (Sony) <urezki@gmail.com>
//

    static type name = init;				\
    module_param(name, type, 0444);			\
    MODULE_PARM_DESC(name, msg)				\
    __param(int, nr_threads, 0,
    "Number of workers to perform tests(min: 1 max: USHRT_MAX)");
    __param(bool, sequential_test_order, false,
    "Use sequential stress tests order");
    __param(int, test_repeat_count, 1,
    "Set test repeat counter");
    __param(int, test_loop_count, 1000000,
    "Set test loop counter");
    __param(int, nr_pages, 0,
    "Set number of pages for fix_size_alloc_test(default: 1)");
    __param(bool, use_huge, false,
    "Use vmalloc_huge in fix_size_alloc_test");
    __param(int, run_test_mask, 7,
    "Set tests specified in the mask.\n\n"
    "\t\tid: 1,    name: fix_size_alloc_test\n"
    "\t\tid: 2,    name: full_fit_alloc_test\n"
    "\t\tid: 4,    name: long_busy_list_alloc_test\n"
    "\t\tid: 8,    name: random_size_alloc_test\n"
    "\t\tid: 16,   name: fix_align_alloc_test\n"
    "\t\tid: 32,   name: random_size_align_alloc_test\n"
    "\t\tid: 64,   name: align_shift_alloc_test\n"
    "\t\tid: 128,  name: pcpu_alloc_test\n"
    "\t\tid: 256,  name: kvfree_rcu_1_arg_vmalloc_test\n"
    "\t\tid: 512,  name: kvfree_rcu_2_arg_vmalloc_test\n"
    "\t\tid: 1024, name: vm_map_ram_test\n"
    "\t\tid: 2048, name: no_block_alloc_test\n"
    "\t\tid: 4096, name: vrealloc_test\n"
// Add a new test case description here.
    );
    __param(int, nr_pcpu_objects, 35000,
    "Number of pcpu objects to allocate for pcpu_alloc_test");
//
// This is for synchronization of setup phase.
//
    DEFINE_STATIC_SRCU(prepare_for_test_srcu);
//
// Completion tracking for worker threads.
//
    static DECLARE_COMPLETION(test_all_done_comp);
    let mut test_n_undone: static atomic_t = ATOMIC_INIT(0);
    static inline void
    test_report_one_done(void)
    {
    if (atomic_dec_and_test(&test_n_undone))
    complete(&test_all_done_comp);
    }
#[no_mangle]
unsafe extern "C" fn random_size_align_alloc_test() -> c_int {
    static int random_size_align_alloc_test(void)
    {
    unsigned long size, align;
    unsigned int rnd;
    void *ptr;
    int i;
    for (i = 0; i < test_loop_count; i++) {
    rnd = get_random_u8();
//
// Maximum 1024 pages, if PAGE_SIZE is 4096.
//
    align = 1 << (rnd % 23);
//
// Maximum 10 pages.
//
    size = ((rnd % 10) + 1) * PAGE_SIZE;
    ptr = __vmalloc_node(size, align, GFP_KERNEL | __GFP_ZERO, 0,
    __builtin_return_address(0));
    if (!ptr)
    return -1;
    vfree(ptr);
    }
    return 0;
    }
//
// This test case is supposed to be failed.
//
#[no_mangle]
unsafe extern "C" fn align_shift_alloc_test() -> c_int {
    static int align_shift_alloc_test(void)
    {
    unsigned long align;
    void *ptr;
    int i;
    for (i = 0; i < BITS_PER_LONG; i++) {
    align = 1UL << i;
    ptr = __vmalloc_node(PAGE_SIZE, align, GFP_KERNEL|__GFP_ZERO, 0,
    __builtin_return_address(0));
    if (!ptr)
    return -1;
    vfree(ptr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fix_align_alloc_test() -> c_int {
    static int fix_align_alloc_test(void)
    {
    void *ptr;
    int i;
    for (i = 0; i < test_loop_count; i++) {
    ptr = __vmalloc_node(5 * PAGE_SIZE, THREAD_ALIGN << 1,
    GFP_KERNEL | __GFP_ZERO, 0,
    __builtin_return_address(0));
    if (!ptr)
    return -1;
    vfree(ptr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn random_size_alloc_test() -> c_int {
    static int random_size_alloc_test(void)
    {
    unsigned int n;
    void *p;
    int i;
    for (i = 0; i < test_loop_count; i++) {
    n = get_random_u32_inclusive(1, 100);
    p = vmalloc(n * PAGE_SIZE);
    if (!p)
    return -1;
// ((__u8 *)p) = 1;
    vfree(p);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn long_busy_list_alloc_test() -> c_int {
    static int long_busy_list_alloc_test(void)
    {
    void *ptr_1, *ptr_2;
    void **ptr;
    let mut rv: c_int = -1;
    int i;
    ptr = vmalloc(sizeof(void *) * 15000);
    if (!ptr)
    return rv;
    for (i = 0; i < 15000; i++)
    ptr[i] = vmalloc(1 * PAGE_SIZE);
    for (i = 0; i < test_loop_count; i++) {
    ptr_1 = vmalloc(100 * PAGE_SIZE);
    if (!ptr_1)
    goto leave;
    ptr_2 = vmalloc(1 * PAGE_SIZE);
    if (!ptr_2) {
    vfree(ptr_1);
    goto leave;
    }
// ((__u8 *)ptr_1) = 0;
// ((__u8 *)ptr_2) = 1;
    vfree(ptr_1);
    vfree(ptr_2);
    }
// Success
    rv = 0;
    leave:
    for (i = 0; i < 15000; i++)
    vfree(ptr[i]);
    vfree(ptr);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn full_fit_alloc_test() -> c_int {
    static int full_fit_alloc_test(void)
    {
    void **ptr, **junk_ptr, *tmp;
    int junk_length;
    let mut rv: c_int = -1;
    int i;
    junk_length = fls(num_online_cpus());
    junk_length *= (32 * 1024 * 1024 / PAGE_SIZE);
    ptr = vmalloc(sizeof(void *) * junk_length);
    if (!ptr)
    return rv;
    junk_ptr = vmalloc(sizeof(void *) * junk_length);
    if (!junk_ptr) {
    vfree(ptr);
    return rv;
    }
    for (i = 0; i < junk_length; i++) {
    ptr[i] = vmalloc(1 * PAGE_SIZE);
    junk_ptr[i] = vmalloc(1 * PAGE_SIZE);
    }
    for (i = 0; i < junk_length; i++)
    vfree(junk_ptr[i]);
    for (i = 0; i < test_loop_count; i++) {
    tmp = vmalloc(1 * PAGE_SIZE);
    if (!tmp)
    goto error;
// ((__u8 *)tmp) = 1;
    vfree(tmp);
    }
// Success
    rv = 0;
    error:
    for (i = 0; i < junk_length; i++)
    vfree(ptr[i]);
    vfree(ptr);
    vfree(junk_ptr);
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn fix_size_alloc_test() -> c_int {
    static int fix_size_alloc_test(void)
    {
    void *ptr;
    int i;
    for (i = 0; i < test_loop_count; i++) {
    if (use_huge)
    ptr = vmalloc_huge((nr_pages > 0 ? nr_pages:1) * PAGE_SIZE, GFP_KERNEL);
    else
    ptr = vmalloc((nr_pages > 0 ? nr_pages:1) * PAGE_SIZE);
    if (!ptr)
    return -1;
// ((__u8 *)ptr) = 0;
    vfree(ptr);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn no_block_alloc_test() -> c_int {
    static int no_block_alloc_test(void)
    {
    void *ptr;
    int i;
    for (i = 0; i < test_loop_count; i++) {
    let mut use_atomic: bool = !!(get_random_u8() % 2);
    let mut gfp: gfp_t = use_atomic ? GFP_ATOMIC : GFP_NOWAIT;
    let mut size: c_ulong = (nr_pages > 0 ? nr_pages : 1) * PAGE_SIZE;
    preempt_disable();
    ptr = __vmalloc(size, gfp);
    preempt_enable();
    if (!ptr)
    return -1;
// ((__u8 *)ptr) = 0;
    vfree(ptr);
    }
    return 0;
    }
    static int
    pcpu_alloc_test(void)
    {
    let mut rv: c_int = 0;

    void __percpu **pcpu;
    size_t size, align;
    int i;
    pcpu = vmalloc(sizeof(void __percpu *) * nr_pcpu_objects);
    if (!pcpu)
    return -1;
    for (i = 0; i < nr_pcpu_objects; i++) {
    size = get_random_u32_inclusive(1, PAGE_SIZE / 4);
//
// Maximum PAGE_SIZE
//
    align = 1 << get_random_u32_inclusive(1, PAGE_SHIFT - 1);
    pcpu[i] = __alloc_percpu(size, align);
    if (!pcpu[i])
    rv = -1;
    }
    for (i = 0; i < nr_pcpu_objects; i++)
    free_percpu(pcpu[i]);
    vfree(pcpu);

    return rv;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_kvfree_rcu {
    pub rcu: rcu_head,
    pub array: [c_uchar; 20],
}

    static int
    kvfree_rcu_1_arg_vmalloc_test(void)
    {
    struct test_kvfree_rcu *p;
    int i;
    for (i = 0; i < test_loop_count; i++) {
    p = vmalloc(1 * PAGE_SIZE);
    if (!p)
    return -1;
    p.array[0] = 'a';
    kvfree_rcu_mightsleep(p);
    }
    return 0;
    }
    static int
    kvfree_rcu_2_arg_vmalloc_test(void)
    {
    struct test_kvfree_rcu *p;
    int i;
    for (i = 0; i < test_loop_count; i++) {
    p = vmalloc(1 * PAGE_SIZE);
    if (!p)
    return -1;
    p.array[0] = 'a';
    kvfree_rcu(p, rcu);
    }
    return 0;
    }
    static int
    vm_map_ram_test(void)
    {
    unsigned long nr_allocated;
    unsigned int map_nr_pages;
    unsigned char *v_ptr;
    struct page **pages;
    int i;
    map_nr_pages = nr_pages > 0 ? nr_pages:1;
    pages = kzalloc_objs(struct page *, map_nr_pages);
    if (!pages)
    return -1;
    nr_allocated = alloc_pages_bulk(GFP_KERNEL, map_nr_pages, pages);
    if (nr_allocated != map_nr_pages)
    goto cleanup;
// Run the test loop.
    for (i = 0; i < test_loop_count; i++) {
    v_ptr = vm_map_ram(pages, map_nr_pages, NUMA_NO_NODE);
// v_ptr = 'a';
    vm_unmap_ram(v_ptr, map_nr_pages);
    }
    cleanup:
    for (i = 0; i < nr_allocated; i++)
    __free_page(pages[i]);
    kfree(pages);
// 0 indicates success.
    return nr_allocated != map_nr_pages;
    }
#[no_mangle]
unsafe extern "C" fn vrealloc_test() -> c_int {
    static int vrealloc_test(void)
    {
    void *ptr, *tmp;
    int i;
    for (i = 0; i < test_loop_count; i++) {
    let mut err: c_int = -1;
    ptr = vrealloc(core::ptr::null_mut(), PAGE_SIZE, GFP_KERNEL);
    if (!ptr)
    return -1;
// ((__u8 *)ptr) = 'a';
// Grow: beyond allocated pages, triggers full realloc.
    tmp = vrealloc(ptr, 4 * PAGE_SIZE, GFP_KERNEL);
    if (!tmp)
    goto error;
    ptr = tmp;
    if (*((__u8 *)ptr) != 'a')
    goto error;
// Shrink: crosses page boundary, frees tail pages.
    tmp = vrealloc(ptr, PAGE_SIZE, GFP_KERNEL);
    if (!tmp)
    goto error;
    ptr = tmp;
    if (*((__u8 *)ptr) != 'a')
    goto error;
// Shrink: within same page, no page freeing.
    tmp = vrealloc(ptr, PAGE_SIZE / 2, GFP_KERNEL);
    if (!tmp)
    goto error;
    ptr = tmp;
    if (*((__u8 *)ptr) != 'a')
    goto error;
// Grow: within allocated page, in-place, no realloc.
    tmp = vrealloc(ptr, PAGE_SIZE, GFP_KERNEL);
    if (!tmp)
    goto error;
    ptr = tmp;
    if (*((__u8 *)ptr) != 'a')
    goto error;
    err = 0;
    error:
    vfree(ptr);
    if (err)
    return err;
    }
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_case_desc {
    pub test_name: *const c_char,
    pub (*test_func)(void): *mut c_int,
    pub xfail: bool,
}

    static struct test_case_desc test_case_array[] = {
    { "fix_size_alloc_test", fix_size_alloc_test, },
    { "full_fit_alloc_test", full_fit_alloc_test, },
    { "long_busy_list_alloc_test", long_busy_list_alloc_test, },
    { "random_size_alloc_test", random_size_alloc_test, },
    { "fix_align_alloc_test", fix_align_alloc_test, },
    { "random_size_align_alloc_test", random_size_align_alloc_test, },
    { "align_shift_alloc_test", align_shift_alloc_test, true },
    { "pcpu_alloc_test", pcpu_alloc_test, },
    { "kvfree_rcu_1_arg_vmalloc_test", kvfree_rcu_1_arg_vmalloc_test, },
    { "kvfree_rcu_2_arg_vmalloc_test", kvfree_rcu_2_arg_vmalloc_test, },
    { "vm_map_ram_test", vm_map_ram_test, },
    { "no_block_alloc_test", no_block_alloc_test, true },
    { "vrealloc_test", vrealloc_test, },
// Add a new test case here.
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_case_data {
    pub test_failed: c_int,
    pub test_xfailed: c_int,
    pub test_passed: c_int,
    pub time: u64,
}

    static struct test_driver {
    struct task_struct *task;
    struct test_case_data data[ARRAY_SIZE(test_case_array)];
    unsigned long start;
    unsigned long stop;
    } *tdriver;
#[no_mangle]
unsafe extern "C" fn shuffle_array(arr: *mut c_int, n: c_int) {
    static void shuffle_array(int *arr, int n)
    {
    int i, j;
    for (i = n - 1; i > 0; i--)  {
// Cut the range.
    j = get_random_u32_below(i);
// Swap indexes.
    swap(arr[i], arr[j]);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_func(private: *mut c_void) -> c_int {
    static int test_func(void *private)
    {
    struct test_driver *t = private;
    int random_array[ARRAY_SIZE(test_case_array)];
    int index, i, j, ret;
    ktime_t kt;
    u64 delta;
    for (i = 0; i < ARRAY_SIZE(test_case_array); i++)
    random_array[i] = i;
    if (!sequential_test_order)
    shuffle_array(random_array, ARRAY_SIZE(test_case_array));
//
// Block until initialization is done.
//
    synchronize_srcu(&prepare_for_test_srcu);
    t.start = get_cycles();
    for (i = 0; i < ARRAY_SIZE(test_case_array); i++) {
    index = random_array[i];
//
// Skip tests if run_test_mask has been specified.
//
    if (!((run_test_mask & (1 << index)) >> index))
    continue;
    kt = ktime_get();
    for (j = 0; j < test_repeat_count; j++) {
    ret = test_case_array[index].test_func();
    if (!ret)
    t.data[index].test_passed++;
#[no_mangle]
pub unsafe extern "C" fn if(test_case_array[index].xfail: ret &&) -> else {
    else if (ret && test_case_array[index].xfail)
    t.data[index].test_xfailed++;
    else
    t.data[index].test_failed++;
    }
//
// Take an average time that test took.
//
    delta = (u64) ktime_us_delta(ktime_get(), kt);
    do_div(delta, (u32) test_repeat_count);
    t.data[index].time = delta;
    }
    t.stop = get_cycles();
    test_report_one_done();
//
// Wait for the kthread_stop() call.
//
    while (!kthread_should_stop())
    msleep(10);
    return 0;
    }
    static int
    init_test_configuration(void)
    {
//
// A maximum number of workers is defined as hard-coded
// value and set to USHRT_MAX. We add such gap just in
// case and for potential heavy stressing.
//
    nr_threads = clamp(nr_threads, 1, (int) USHRT_MAX);
// Allocate the space for test instances.
    tdriver = kvzalloc_objs(*tdriver, nr_threads);
    if (tdriver == core::ptr::null_mut())
    return -1;
    if (test_repeat_count <= 0)
    test_repeat_count = 1;
    if (test_loop_count <= 0)
    test_loop_count = 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_concurrent_test() {
    static void do_concurrent_test(void)
    {
    int i, ret, idx;
//
// Set some basic configurations plus sanity check.
//
    ret = init_test_configuration();
    if (ret < 0)
    return;
//
// Put on hold all workers.
//
    idx = srcu_read_lock(&prepare_for_test_srcu);
    for (i = 0; i < nr_threads; i++) {
    struct test_driver *t = &tdriver[i];
    t.task = kthread_run(test_func, t, "vmalloc_test/%d", i);
    if (!IS_ERR(t.task))
// Success.
    atomic_inc(&test_n_undone);
    else
    pr_err("Failed to start %d kthread\n", i);
    }
//
// Now let the workers do their job.
//
    srcu_read_unlock(&prepare_for_test_srcu, idx);
//
// Sleep quiet until all workers are done with 1 second
// interval. Since the test can take a lot of time we
// can run into a stack trace of the hung task. That is
// why we go with completion_timeout and HZ value.
//
    do {
    ret = wait_for_completion_timeout(&test_all_done_comp, HZ);
    } while (!ret);
    for (i = 0; i < nr_threads; i++) {
    struct test_driver *t = &tdriver[i];
    int j;
    if (!IS_ERR(t.task))
    kthread_stop(t.task);
    for (j = 0; j < ARRAY_SIZE(test_case_array); j++) {
    if (!((run_test_mask & (1 << j)) >> j))
    continue;
    pr_info(
    "Summary: %s passed: %d failed: %d xfailed: %d repeat: %d loops: %d avg: %llu usec\n",
    test_case_array[j].test_name,
    t.data[j].test_passed,
    t.data[j].test_failed,
    t.data[j].test_xfailed,
    test_repeat_count, test_loop_count,
    t.data[j].time);
    }
    pr_info("All test took worker%d=%lu cycles\n",
    i, t.stop - t.start);
    }
    kvfree(tdriver);
    }
#[no_mangle]
unsafe extern "C" fn vmalloc_test_init() -> int __init {
    static int __init vmalloc_test_init(void)
    {
    do_concurrent_test();
// Fail will directly unload the module
    return IS_BUILTIN(CONFIG_TEST_VMALLOC) ? 0:-EAGAIN;
    }

    module_init(vmalloc_test_init)

    late_initcall(vmalloc_test_init);

    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Uladzislau Rezki");
    MODULE_DESCRIPTION("vmalloc test module");
