//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/mem-functions.c
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
// mem-memcpy.c
//
// Simple memcpy() and memset() benchmarks
//
// Written by Hitoshi Mitake <mitake@dcl.info.waseda.ac.jp>
//

pub const K: c_int = 1024;
pub const PAGE_SHIFT_4KB: c_int = 12;
pub const PAGE_SHIFT_2MB: c_int = 21;
pub const PAGE_SHIFT_1GB: c_int = 30;
    static const char	*size_str	= "1MB";
    static const char	*function_str	= "all";
    static const char	*page_size_str	= "4KB";
    static const char	*chunk_size_str	= "0";
    let mut nr_loops: static unsigned int = 1;
    static bool		use_cycles;
    static int		cycles_fd;
    static unsigned int	seed;
    let mut nr_threads: static unsigned int = 1;
    static const struct option bench_common_options[] = {
    OPT_STRING('s', "size", &size_str, "1MB",
    "Specify the size of the memory buffers. "
    "Available units: B, KB, MB, GB and TB (case insensitive)"),
    OPT_STRING('p', "page", &page_size_str, "4KB",
    "Specify page-size for mapping memory buffers. "
    "Available sizes: 4KB, 2MB, 1GB (case insensitive)"),
    OPT_STRING('f', "function", &function_str, "all",
    "Specify the function to run, \"all\" runs all available functions, \"help\" lists them"),
    OPT_UINTEGER('l', "nr_loops", &nr_loops,
    "Specify the number of loops to run. (default: 1)"),
    OPT_BOOLEAN('c', "cycles", &use_cycles,
    "Use a cycles event instead of gettimeofday() to measure performance"),
    OPT_END()
    };
    static const struct option bench_mem_options[] = {
    OPT_STRING('k', "chunk", &chunk_size_str, "0",
    "Specify the chunk-size for each invocation. "
    "Available units: B, KB, MB, GB and TB (case insensitive)"),
    OPT_PARENT(bench_common_options),
    OPT_END()
    };
    union bench_clock {
    u64		cycles;
    struct timeval	tv;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bench_params {
    pub size: usize,
    pub size_total: usize,
    pub chunk_size: usize,
    pub nr_loops: c_uint,
    pub page_shift: c_uint,
    pub seed: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bench_mem_info {
    pub functions: *const function,
    int (*do_op)(const struct function *r, struct bench_params *p,
    pub rt): *mut *mut *mut void src, void dst, union bench_clock,
    pub usage: *const *const c_char,
    pub options: *const option,
    pub alloc_src: bool,
}

    typedef bool (*mem_init_t)(struct bench_mem_info *, struct bench_params *,
    void **, void **);
    typedef void (*mem_fini_t)(struct bench_mem_info *, struct bench_params *,
    void **, void **);
    typedef void *(*memcpy_t)(void *, const void *, size_t);
    typedef void *(*memset_t)(void *, int, size_t);
    typedef void (*mmap_op_t)(void *, size_t, unsigned int, bool);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct function {
    pub name: *const c_char,
    pub desc: *const c_char,
    struct {
    pub init: mem_init_t,
    pub fini: mem_fini_t,
    union {
    pub memcpy: memcpy_t,
    pub memset: memset_t,
    pub mmap_op: mmap_op_t,
}

    } fn;
    };
    static struct perf_event_attr cycle_attr = {
    .type		= PERF_TYPE_HARDWARE,
    .config		= PERF_COUNT_HW_CPU_CYCLES
    };
    static struct stats stats;
#[no_mangle]
unsafe extern "C" fn init_cycles() -> c_int {
    static int init_cycles(void)
    {
    cycles_fd = sys_perf_event_open(&cycle_attr, getpid(), -1, -1, perf_event_open_cloexec_flag());
    if (cycles_fd < 0 && errno == ENOSYS) {
    pr_debug("No CONFIG_PERF_EVENTS=y kernel support configured?\n");
    return -1;
    }
    return cycles_fd;
    }
#[no_mangle]
unsafe extern "C" fn get_cycles() -> u64 {
    static u64 get_cycles(void)
    {
    int ret;
    u64 clk;
    ret = read(cycles_fd, &clk, sizeof(u64));
    BUG_ON(ret != sizeof(u64));
    return clk;
    }
#[no_mangle]
unsafe extern "C" fn clock_get(t: *mut union bench_clock) {
    static void clock_get(union bench_clock *t)
    {
    if (use_cycles)
    t.cycles = get_cycles();
    else
    BUG_ON(gettimeofday(&t.tv, core::ptr::null_mut()));
    }
#[no_mangle]
unsafe extern "C" fn clock_diff(s: *mut union bench_clock, e: *mut union bench_clock) -> union bench_clock {
    static union bench_clock clock_diff(union bench_clock *s, union bench_clock *e)
    {
    union bench_clock t;
    if (use_cycles)
    t.cycles = e.cycles - s.cycles;
    else
    timersub(&e.tv, &s.tv, &t.tv);
    return t;
    }
#[no_mangle]
unsafe extern "C" fn clock_accum(a: *mut union bench_clock, b: *mut union bench_clock) {
    static void clock_accum(union bench_clock *a, union bench_clock *b)
    {
    if (use_cycles)
    a.cycles += b.cycles;
    else
    timeradd(&a.tv, &b.tv, &a.tv);
    }
#[no_mangle]
unsafe extern "C" fn timeval2double(ts: *mut timeval) -> double {
    static double timeval2double(struct timeval *ts)
    {
    return ((double)ts.tv_sec + (double)ts.tv_usec / (double)USEC_PER_SEC) / nr_threads;
    }

    if (x < K)						\
    printf(" %14lf bytes/sec", x);			\
    else if (x < K * K)					\
    printf(" %14lfd KB/sec", x / K);		\
    else if (x < K * K * K)					\
    printf(" %14lf MB/sec", x / K / K);		\
    else							\
    printf(" %14lf GB/sec", x / K / K / K);	\
    } while (0)
    static void __bench_mem_function(struct bench_mem_info *info, struct bench_params *p,
    int r_idx)
    {
    const struct function *r = &info.functions[r_idx];
    let mut result_bps: double = 0.0;
    let mut rt: union bench_clock = { 0 };
    void *src = core::ptr::null_mut(), *dst = core::ptr::null_mut();
    init_stats(&stats);
    printf("# function '%s' (%s)\n", r.name, r.desc);
    if (r.fn.init && r.fn.init(info, p, &src, &dst))
    goto out_init_failed;
    if (bench_format == BENCH_FORMAT_DEFAULT)
    printf("# Copying %s bytes ...\n\n", size_str);
    if (info.do_op(r, p, src, dst, &rt))
    goto out_test_failed;
    switch (bench_format) {
    case BENCH_FORMAT_DEFAULT:
    if (use_cycles) {
    printf(" %14lf cycles/byte", (double)rt.cycles/(double)p.size_total);
    } else {
    result_bps = (double)p.size_total/timeval2double(&rt.tv);
    print_bps(result_bps);
    }
    if (nr_threads > 1) {
    printf("/thread\t( +- %6.2f%% )",
    rel_stddev_stats(stddev_stats(&stats), avg_stats(&stats)));
    }
    printf("\n");
    break;
    case BENCH_FORMAT_SIMPLE:
    if (use_cycles) {
    printf("%lf\n", (double)rt.cycles/(double)p.size_total);
    } else {
    result_bps = (double)p.size_total/timeval2double(&rt.tv);
    printf("%lf\n", result_bps);
    }
    break;
    default:
    BUG_ON(1);
    break;
    }
    out_test_failed:
    out_free:
    if (r.fn.fini) r.fn.fini(info, p, &src, &dst);
    return;
    out_init_failed:
    printf("# Memory allocation failed - maybe size (%s) %s?\n", size_str,
    p.page_shift != PAGE_SHIFT_4KB ? "has insufficient hugepages" : "is too large");
    goto out_free;
    }
#[no_mangle]
unsafe extern "C" fn bench_mem_common(argc: c_int, argv: *const c_char, info: *mut bench_mem_info) -> c_int {
    static int bench_mem_common(int argc, const char **argv, struct bench_mem_info *info)
    {
    int i;
    let mut p: bench_params = { 0 };
    unsigned int page_size;
    argc = parse_options(argc, argv, info.options, info.usage, 0);
    if (use_cycles) {
    i = init_cycles();
    if (i < 0) {
    fprintf(stderr, "Failed to open cycles counter\n");
    return i;
    }
    }
    p.nr_loops = nr_loops;
    p.size = (size_t)perf_atoll((char *)size_str);
    if ((s64)p.size <= 0) {
    fprintf(stderr, "Invalid size:%s\n", size_str);
    return 1;
    }
    p.size_total = p.size * p.nr_loops;
    p.chunk_size = (size_t)perf_atoll((char *)chunk_size_str);
    if ((s64)p.chunk_size < 0 || (s64)p.chunk_size > (s64)p.size) {
    fprintf(stderr, "Invalid chunk_size:%s\n", chunk_size_str);
    return 1;
    }
    if (!p.chunk_size)
    p.chunk_size = p.size;
    page_size = (unsigned int)perf_atoll((char *)page_size_str);
    if (page_size != (1 << PAGE_SHIFT_4KB) &&
    page_size != (1 << PAGE_SHIFT_2MB) &&
    page_size != (1 << PAGE_SHIFT_1GB)) {
    fprintf(stderr, "Invalid page-size:%s\n", page_size_str);
    return 1;
    }
    p.page_shift = ilog2(page_size);
    p.seed = seed;
    if (!strncmp(function_str, "all", 3)) {
    for (i = 0; info.functions[i].name; i++)
    __bench_mem_function(info, &p, i);
    return 0;
    }
    for (i = 0; info.functions[i].name; i++) {
    if (!strcmp(info.functions[i].name, function_str))
    break;
    }
    if (!info.functions[i].name) {
    if (strcmp(function_str, "help") && strcmp(function_str, "h"))
    printf("Unknown function: %s\n", function_str);
    printf("Available functions:\n");
    for (i = 0; info.functions[i].name; i++) {
    printf("\t%s ... %s\n",
    info.functions[i].name, info.functions[i].desc);
    }
    return 1;
    }
    __bench_mem_function(info, &p, i);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn memcpy_prefault(fn: memcpy_t, size: usize, src: *mut c_void, dst: *mut c_void) {
    static void memcpy_prefault(memcpy_t fn, size_t size, void *src, void *dst)
    {
// Make sure to always prefault zero pages even if MMAP_THRESH is crossed:
    memset(src, 0, size);
//
// We prefault the freshly allocated memory range here,
// to not measure page fault overhead:
//
    fn(dst, src, size);
    }
    static int do_memcpy(const struct function *r, struct bench_params *p,
    void *src, void *dst, union bench_clock *rt)
    {
    union bench_clock start, end;
    let mut fn: memcpy_t = r.fn.memcpy;
    memcpy_prefault(fn, p.size, src, dst);
    clock_get(&start);
    for (unsigned int i = 0; i < p.nr_loops; ++i)
    for (size_t off = 0; off < p.size; off += p.chunk_size)
    fn(dst + off, src + off, min(p.chunk_size, p.size - off));
    clock_get(&end);
// rt = clock_diff(&start, &end);
    return 0;
    }
    static void *bench_mmap(size_t size, bool populate, unsigned int page_shift)
    {
    void *p;
    let mut extra: c_int = populate ? MAP_POPULATE : 0;
    if (page_shift != PAGE_SHIFT_4KB)
    extra |= MAP_HUGETLB | (page_shift << MAP_HUGE_SHIFT);
    p = mmap(core::ptr::null_mut(), size, PROT_READ|PROT_WRITE,
    extra | MAP_PRIVATE | MAP_ANONYMOUS, 0, 0);
    let mut p: return = = MAP_FAILED ? core::ptr::null_mut() : p;
    }
#[no_mangle]
unsafe extern "C" fn bench_munmap(p: *mut c_void, size: usize) {
    static void bench_munmap(void *p, size_t size)
    {
    if (p)
    munmap(p, size);
    }
    static bool mem_alloc(struct bench_mem_info *info, struct bench_params *p,
    void **src, void **dst)
    {
    bool failed;
// dst = bench_mmap(p->size, true, p->page_shift);
    failed = *dst == core::ptr::null_mut();
    if (info.alloc_src) {
// src = bench_mmap(p->size, true, p->page_shift);
    failed = failed || *src == core::ptr::null_mut();
    }
    return failed;
    }
    static void mem_free(struct bench_mem_info *info __maybe_unused,
    struct bench_params *p __maybe_unused,
    void **src, void **dst)
    {
    bench_munmap(*dst, p.size);
    bench_munmap(*src, p.size);
// dst = *src = NULL;
    }
    static struct function memcpy_functions[] = {
    { .name		= "default",
    .desc		= "Default memcpy() provided by glibc",
    .fn.init	= mem_alloc,
    .fn.fini	= mem_free,
    .fn.memcpy	= memcpy },

    {.name = _name, .desc = _desc, .fn.memcpy = _fn, .fn.init = _init, .fn.fini = _fini },

    { .name = core::ptr::null_mut(), }
    };
    static const char * const bench_mem_memcpy_usage[] = {
    "perf bench mem memcpy <options>",
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn bench_mem_memcpy(argc: c_int, argv: *const c_char) -> c_int {
    int bench_mem_memcpy(int argc, const char **argv)
    {
    struct bench_mem_info info = {
    .functions		= memcpy_functions,
    .do_op			= do_memcpy,
    .usage			= bench_mem_memcpy_usage,
    .options		= bench_mem_options,
    .alloc_src              = true,
    };
    return bench_mem_common(argc, argv, &info);
    }
    static int do_memset(const struct function *r, struct bench_params *p,
    void *src __maybe_unused, void *dst, union bench_clock *rt)
    {
    union bench_clock start, end;
    let mut fn: memset_t = r.fn.memset;
//
// We prefault the freshly allocated memory range here,
// to not measure page fault overhead:
//
    fn(dst, -1, p.size);
    clock_get(&start);
    for (unsigned int i = 0; i < p.nr_loops; ++i)
    for (size_t off = 0; off < p.size; off += p.chunk_size)
    fn(dst + off, i, min(p.chunk_size, p.size - off));
    clock_get(&end);
// rt = clock_diff(&start, &end);
    return 0;
    }
    static const char * const bench_mem_memset_usage[] = {
    "perf bench mem memset <options>",
    core::ptr::null_mut()
    };
    static const struct function memset_functions[] = {
    { .name		= "default",
    .desc		= "Default memset() provided by glibc",
    .fn.init	= mem_alloc,
    .fn.fini	= mem_free,
    .fn.memset	= memset },

    {.name = _name, .desc = _desc, .fn.memset = _fn, .fn.init = _init, .fn.fini = _fini },

    { .name = core::ptr::null_mut(), }
    };
#[no_mangle]
pub unsafe extern "C" fn bench_mem_memset(argc: c_int, argv: *const c_char) -> c_int {
    int bench_mem_memset(int argc, const char **argv)
    {
    struct bench_mem_info info = {
    .functions		= memset_functions,
    .do_op			= do_memset,
    .usage			= bench_mem_memset_usage,
    .options		= bench_mem_options,
    };
    return bench_mem_common(argc, argv, &info);
    }
#[no_mangle]
unsafe extern "C" fn mmap_page_touch(dst: *mut c_void, size: usize, page_shift: c_uint, random: bool) {
    static void mmap_page_touch(void *dst, size_t size, unsigned int page_shift, bool random)
    {
    let mut npages: c_ulong = size / (1 << page_shift);
    let mut offset: c_ulong = 0, r = 0;
    for (unsigned long i = 0; i < npages; i++) {
    if (random)
    r = rand() % (1 << page_shift);
// ((char *)dst + offset + r) = *(char *)(dst + offset + r) + i;
    offset += 1 << page_shift;
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_data {
    pub id: pthread_t,
    pub func: *const function,
    pub params: *mut bench_params,
    pub result: union bench_clock,
    pub seed: c_uint,
    pub error: c_int,
}

    static void *do_mmap_thread(void *arg)
    {
    struct mmap_data *data = arg;
    const struct function *r = data.func;
    struct bench_params *p = data.params;
    union bench_clock start, end, diff;
    let mut fn: mmap_op_t = r.fn.mmap_op;
    let mut populate: bool = strcmp(r.name, "populate") == 0;
    void *dst;
    if (data.seed)
    srand(data.seed);
    for (unsigned int i = 0; i < p.nr_loops; i++) {
    clock_get(&start);
    dst = bench_mmap(p.size, populate, p.page_shift);
    if (!dst)
    goto out;
    fn(dst, p.size, p.page_shift, p.seed);
    clock_get(&end);
    diff = clock_diff(&start, &end);
    clock_accum(&data.result, &diff);
    bench_munmap(dst, p.size);
    }
    return data;
    out:
    data.error = -ENOMEM;
    return core::ptr::null_mut();
    }
    static int do_mmap(const struct function *r, struct bench_params *p,
    void *src __maybe_unused, void *dst __maybe_unused,
    union bench_clock *accum)
    {
    struct mmap_data *data;
    let mut error: c_int = 0;
    data = calloc(nr_threads, sizeof(*data));
    if (!data) {
    printf("# Failed to allocate thread resources\n");
    return -1;
    }
    for (unsigned int i = 0; i < nr_threads; i++) {
    data[i].func = r;
    data[i].params = p;
    if (p.seed)
    data[i].seed = p.seed + i;
    if (pthread_create(&data[i].id, core::ptr::null_mut(), do_mmap_thread, &data[i]) < 0)
    data[i].error = -errno;
    }
    for (unsigned int i = 0; i < nr_threads; i++) {
    union bench_clock *t = &data[i].result;
    pthread_join(data[i].id, core::ptr::null_mut());
    clock_accum(accum, t);
    if (use_cycles)
    update_stats(&stats, t.cycles);
    else
    update_stats(&stats, t.tv.tv_sec * 1e6 + t.tv.tv_usec);
    error |= data[i].error;
    }
    free(data);
    if (error) {
    printf("# Memory allocation failed - maybe size (%s) %s?\n", size_str,
    p.page_shift != PAGE_SHIFT_4KB ? "has insufficient hugepages" : "is too large");
    }
    return error ? -1 : 0;
    }
    static const char * const bench_mem_mmap_usage[] = {
    "perf bench mem mmap <options>",
    core::ptr::null_mut()
    };
    static const struct function mmap_functions[] = {
    { .name		= "demand",
    .desc		= "Demand loaded mmap()",
    .fn.mmap_op	= mmap_page_touch },
    { .name		= "populate",
    .desc		= "Eagerly populated mmap()",
    .fn.mmap_op	= mmap_page_touch },
    { .name = core::ptr::null_mut(), }
    };
#[no_mangle]
pub unsafe extern "C" fn bench_mem_mmap(argc: c_int, argv: *const c_char) -> c_int {
    int bench_mem_mmap(int argc, const char **argv)
    {
    static const struct option bench_mmap_options[] = {
    OPT_UINTEGER('r', "randomize", &seed,
    "Seed to randomize page access offset."),
    OPT_UINTEGER('t', "threads", &nr_threads,
    "Number of threads to run concurrently (default: 1)."),
    OPT_PARENT(bench_common_options),
    OPT_END()
    };
    struct bench_mem_info info = {
    .functions		= mmap_functions,
    .do_op			= do_mmap,
    .usage			= bench_mem_mmap_usage,
    .options		= bench_mmap_options,
    };
    return bench_mem_common(argc, argv, &info);
    }
