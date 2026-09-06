//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/bench/page_pool/bench_page_pool_simple.c
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
// Benchmark module for page_pool.
//

    let mut verbose: static int = 1;
pub const MY_POOL_SIZE: c_int = 1024;
// Makes tests selectable. Useful for perf-record to analyze a single test.
// Hint: Bash shells support writing binary number like: $((2#101010)
//
// # modprobe bench_page_pool_simple run_flags=$((2#100))
//
    let mut run_flags: static unsigned long = 0xFFFFFFFF;
    module_param(run_flags, ulong, 0);
    MODULE_PARM_DESC(run_flags, "Limit which bench test that runs");
// Count the bit number from the enum
    enum benchmark_bit {
    bit_run_bench_baseline,
    bit_run_bench_no_softirq01,
    bit_run_bench_no_softirq02,
    bit_run_bench_no_softirq03,
    };

// notice time_bench is limited to U32_MAX nr loops
    let mut loops: static unsigned long = 10000000;
    module_param(loops, ulong, 0);
    MODULE_PARM_DESC(loops, "Specify loops bench will run");
// Timing at the nanosec level, we need to know the overhead
// introduced by the for loop itself
//
#[no_mangle]
unsafe extern "C" fn time_bench_for_loop(rec: *mut time_bench_record, data: *mut c_void) -> c_int {
    static int time_bench_for_loop(struct time_bench_record *rec, void *data)
    {
    let mut loops_cnt: u64 = 0;
    int i;
    time_bench_start(rec);
// Loop to measure
    for (i = 0; i < rec.loops; i++) {
    loops_cnt++;
    barrier(); /* avoid compiler to optimize this loop */
    }
    time_bench_stop(rec, loops_cnt);
    return loops_cnt;
    }
#[no_mangle]
unsafe extern "C" fn time_bench_atomic_inc(rec: *mut time_bench_record, data: *mut c_void) -> c_int {
    static int time_bench_atomic_inc(struct time_bench_record *rec, void *data)
    {
    let mut loops_cnt: u64 = 0;
    atomic_t cnt;
    int i;
    atomic_set(&cnt, 0);
    time_bench_start(rec);
// Loop to measure
    for (i = 0; i < rec.loops; i++) {
    atomic_inc(&cnt);
    barrier(); /* avoid compiler to optimize this loop */
    }
    loops_cnt = atomic_read(&cnt);
    time_bench_stop(rec, loops_cnt);
    return loops_cnt;
    }
// The ptr_ping in page_pool uses a spinlock. We need to know the minimum
// overhead of taking+releasing a spinlock, to know the cycles that can be saved
// by e.g. amortizing this via bulking.
//
#[no_mangle]
unsafe extern "C" fn time_bench_lock(rec: *mut time_bench_record, data: *mut c_void) -> c_int {
    static int time_bench_lock(struct time_bench_record *rec, void *data)
    {
    let mut loops_cnt: u64 = 0;
    spinlock_t lock;
    int i;
    spin_lock_init(&lock);
    time_bench_start(rec);
// Loop to measure
    for (i = 0; i < rec.loops; i++) {
    spin_lock(&lock);
    loops_cnt++;
    barrier(); /* avoid compiler to optimize this loop */
    spin_unlock(&lock);
    }
    time_bench_stop(rec, loops_cnt);
    return loops_cnt;
    }
// Helper for filling some page's into ptr_ring
#[no_mangle]
unsafe extern "C" fn pp_fill_ptr_ring(pp: *mut page_pool, elems: c_int) {
    static void pp_fill_ptr_ring(struct page_pool *pp, int elems)
    {
// GFP_ATOMIC needed when under run softirq
    let mut gfp_mask: gfp_t = GFP_ATOMIC;
    struct page **array;
    int i;
    array = kcalloc(elems, sizeof(struct page *), gfp_mask);
    for (i = 0; i < elems; i++)
    array[i] = page_pool_alloc_pages(pp, gfp_mask);
    for (i = 0; i < elems; i++)
    page_pool_put_page(pp, array[i], -1, false);
    kfree(array);
    }
    enum test_type { type_fast_path, type_ptr_ring, type_page_allocator };
// Depends on compile optimizing this function
    static int time_bench_page_pool(struct time_bench_record *rec, void *data,
    enum test_type type, const char *func)
    {
    let mut loops_cnt: u64 = 0;
    gfp_t gfp_mask = GFP_ATOMIC; /* GFP_ATOMIC is not really needed */
    int i, err;
    struct page_pool *pp;
    struct page *page;
    struct page_pool_params pp_params = {
    .order = 0,
    .flags = 0,
    .pool_size = MY_POOL_SIZE,
    .nid = NUMA_NO_NODE,
    .dev = core::ptr::null_mut(), /* Only use for DMA mapping */
    .dma_dir = DMA_BIDIRECTIONAL,
    };
    pp = page_pool_create(&pp_params);
    if (IS_ERR(pp)) {
    err = PTR_ERR(pp);
    pr_warn("%s: Error(%d) creating page_pool\n", func, err);
    goto out;
    }
    pp_fill_ptr_ring(pp, 64);
    if (in_serving_softirq())
    pr_warn("%s(): in_serving_softirq fast-path\n", func);
    else
    pr_warn("%s(): Cannot use page_pool fast-path\n", func);
    time_bench_start(rec);
// Loop to measure
    for (i = 0; i < rec.loops; i++) {
// Common fast-path alloc that depend on in_serving_softirq()
    page = page_pool_alloc_pages(pp, gfp_mask);
    if (!page)
    break;
    loops_cnt++;
    barrier(); /* avoid compiler to optimize this loop */
// The benchmarks purpose it to test different return paths.
// Compiler should inline optimize other function calls out
//
    if (type == type_fast_path) {
// Fast-path recycling e.g. XDP_DROP use-case
    page_pool_recycle_direct(pp, page);
    } else if (type == type_ptr_ring) {
// Normal return path
    page_pool_put_page(pp, page, -1, false);
    } else if (type == type_page_allocator) {
// Test if not pages are recycled, but instead
// returned back into systems page allocator
//
    get_page(page); /* cause no-recycling */
    page_pool_put_page(pp, page, -1, false);
    put_page(page);
    } else {
    BUILD_BUG();
    }
    }
    time_bench_stop(rec, loops_cnt);
    out:
    page_pool_destroy(pp);
    return loops_cnt;
    }
    static int time_bench_page_pool01_fast_path(struct time_bench_record *rec,
    void *data)
    {
    return time_bench_page_pool(rec, data, type_fast_path, __func__);
    }
    static int time_bench_page_pool02_ptr_ring(struct time_bench_record *rec,
    void *data)
    {
    return time_bench_page_pool(rec, data, type_ptr_ring, __func__);
    }
    static int time_bench_page_pool03_slow(struct time_bench_record *rec,
    void *data)
    {
    return time_bench_page_pool(rec, data, type_page_allocator, __func__);
    }
#[no_mangle]
unsafe extern "C" fn run_benchmark_tests() -> c_int {
    static int run_benchmark_tests(void)
    {
    let mut nr_loops: u32 = loops;
// Baseline tests
    if (enabled(bit_run_bench_baseline)) {
    time_bench_loop(nr_loops * 10, 0, "for_loop", core::ptr::null_mut(),
    time_bench_for_loop);
    time_bench_loop(nr_loops * 10, 0, "atomic_inc", core::ptr::null_mut(),
    time_bench_atomic_inc);
    time_bench_loop(nr_loops, 0, "lock", core::ptr::null_mut(), time_bench_lock);
    }
// This test cannot activate correct code path, due to no-softirq ctx
    if (enabled(bit_run_bench_no_softirq01))
    time_bench_loop(nr_loops, 0, "no-softirq-page_pool01", core::ptr::null_mut(),
    time_bench_page_pool01_fast_path);
    if (enabled(bit_run_bench_no_softirq02))
    time_bench_loop(nr_loops, 0, "no-softirq-page_pool02", core::ptr::null_mut(),
    time_bench_page_pool02_ptr_ring);
    if (enabled(bit_run_bench_no_softirq03))
    time_bench_loop(nr_loops, 0, "no-softirq-page_pool03", core::ptr::null_mut(),
    time_bench_page_pool03_slow);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bench_page_pool_simple_module_init() -> int __init {
    static int __init bench_page_pool_simple_module_init(void)
    {
    if (verbose)
    pr_info("Loaded\n");
    if (loops > U32_MAX) {
    pr_err("Module param loops(%lu) exceeded U32_MAX(%u)\n", loops,
    U32_MAX);
    return -ECHRNG;
    }
    run_benchmark_tests();
    return 0;
    }
    module_init(bench_page_pool_simple_module_init);
#[no_mangle]
unsafe extern "C" fn bench_page_pool_simple_module_exit() -> void __exit {
    static void __exit bench_page_pool_simple_module_exit(void)
    {
    if (verbose)
    pr_info("Unloaded\n");
    }
    module_exit(bench_page_pool_simple_module_exit);
    MODULE_DESCRIPTION("Benchmark of page_pool simple cases");
    MODULE_AUTHOR("Jesper Dangaard Brouer <netoptimizer@brouer.com>");
    MODULE_LICENSE("GPL");
