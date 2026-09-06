//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/benchs/bench_trigger.c
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
// Copyright (c) 2020 Facebook
// Macro flag: #define _GNU_SOURCE

pub const MAX_TRIG_BATCH_ITERS: c_int = 1000;
    static struct {
    __u32 batch_iters;
    } args = {
    .batch_iters = 100,
    };
    enum {
    ARG_TRIG_BATCH_ITERS = 7000,
    };
    static const struct argp_option opts[] = {
    { "trig-batch-iters", ARG_TRIG_BATCH_ITERS, "BATCH_ITER_CNT", 0,
    "Number of in-kernel iterations per one driver test run"},
    {},
    };
#[no_mangle]
unsafe extern "C" fn parse_arg(key: c_int, arg: *mut c_char, state: *mut argp_state) -> error_t {
    static error_t parse_arg(int key, char *arg, struct argp_state *state)
    {
    long ret;
    switch (key) {
    case ARG_TRIG_BATCH_ITERS:
    ret = strtol(arg, core::ptr::null_mut(), 10);
    if (ret < 1 || ret > MAX_TRIG_BATCH_ITERS) {
    fprintf(stderr, "invalid --trig-batch-iters value (should be between %d and %d)\n",
    1, MAX_TRIG_BATCH_ITERS);
    argp_usage(state);
    }
    args.batch_iters = ret;
    break;
    default:
    return ARGP_ERR_UNKNOWN;
    }
    return 0;
    }
    const struct argp bench_trigger_batch_argp = {
    .options = opts,
    .parser = parse_arg,
    };
// adjust slot shift in inc_hits() if changing
pub const MAX_BUCKETS: c_int = 256;

// BPF triggering benchmarks
    static struct trigger_ctx {
    struct trigger_bench *skel;
    bool usermode_counters;
    int driver_prog_fd;
    } ctx;
    static struct counter base_hits[MAX_BUCKETS];
#[no_mangle]
unsafe extern "C" fn inc_counter(counters: *mut counter) -> __always_inline void {
    static __always_inline void inc_counter(struct counter *counters)
    {
    let mut tid: static __thread int = 0;
    unsigned slot;
    if (unlikely(tid == 0))
    tid = sys_gettid();
// multiplicative hashing, it's fast
    slot = 2654435769U * tid;
    slot >>= 24;
    atomic_inc(&base_hits[slot].value); /* use highest byte as an index */
    }
#[no_mangle]
unsafe extern "C" fn sum_and_reset_counters(counters: *mut counter) -> c_long {
    static long sum_and_reset_counters(struct counter *counters)
    {
    int i;
    let mut sum: c_long = 0;
    for (i = 0; i < MAX_BUCKETS; i++)
    sum += atomic_swap(&counters[i].value, 0);
    return sum;
    }
#[no_mangle]
unsafe extern "C" fn trigger_validate() {
    static void trigger_validate(void)
    {
    if (env.consumer_cnt != 0) {
    fprintf(stderr, "benchmark doesn't support consumer!\n");
    exit(1);
    }
    }
    static void *trigger_producer(void *input)
    {
    if (ctx.usermode_counters) {
    while (true) {
    (void)syscall(__NR_getpgid);
    inc_counter(base_hits);
    }
    } else {
    while (true)
    (void)syscall(__NR_getpgid);
    }
    return core::ptr::null_mut();
    }
    static void *trigger_producer_batch(void *input)
    {
    let mut fd: c_int = ctx.driver_prog_fd ?: bpf_program__fd(ctx.skel.progs.trigger_driver);
    while (true)
    bpf_prog_test_run_opts(fd, core::ptr::null_mut());
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn trigger_measure(res: *mut bench_res) {
    static void trigger_measure(struct bench_res *res)
    {
    if (ctx.usermode_counters)
    res.hits = sum_and_reset_counters(base_hits);
    else
    res.hits = sum_and_reset_counters(ctx.skel.bss.hits);
    }
#[no_mangle]
unsafe extern "C" fn setup_ctx() {
    static void setup_ctx(void)
    {
    setup_libbpf();
    ctx.skel = trigger_bench__open();
    if (!ctx.skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
// default "driver" BPF program
    bpf_program__set_autoload(ctx.skel.progs.trigger_driver, true);
    ctx.skel.rodata.batch_iters = args.batch_iters;
    ctx.skel.rodata.stacktrace = env.stacktrace;
    }
#[no_mangle]
unsafe extern "C" fn load_ctx() {
    static void load_ctx(void)
    {
    int err;
    err = trigger_bench__load(ctx.skel);
    if (err) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn attach_bpf(prog: *mut bpf_program) {
    static void attach_bpf(struct bpf_program *prog)
    {
    struct bpf_link *link;
    link = bpf_program__attach(prog);
    if (!link) {
    fprintf(stderr, "failed to attach program!\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn trigger_syscall_count_setup() {
    static void trigger_syscall_count_setup(void)
    {
    ctx.usermode_counters = true;
    }
// Batched, staying mostly in-kernel triggering setups
#[no_mangle]
unsafe extern "C" fn trigger_kernel_count_setup() {
    static void trigger_kernel_count_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.trigger_driver, false);
    bpf_program__set_autoload(ctx.skel.progs.trigger_kernel_count, true);
    load_ctx();
// override driver program
    ctx.driver_prog_fd = bpf_program__fd(ctx.skel.progs.trigger_kernel_count);
    }
#[no_mangle]
unsafe extern "C" fn trigger_kprobe_setup() {
    static void trigger_kprobe_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_kprobe, true);
    load_ctx();
    attach_bpf(ctx.skel.progs.bench_trigger_kprobe);
    }
#[no_mangle]
unsafe extern "C" fn trigger_kretprobe_setup() {
    static void trigger_kretprobe_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_kretprobe, true);
    load_ctx();
    attach_bpf(ctx.skel.progs.bench_trigger_kretprobe);
    }
#[no_mangle]
unsafe extern "C" fn trigger_kprobe_multi_setup() {
    static void trigger_kprobe_multi_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_kprobe_multi, true);
    load_ctx();
    attach_bpf(ctx.skel.progs.bench_trigger_kprobe_multi);
    }
#[no_mangle]
unsafe extern "C" fn trigger_kretprobe_multi_setup() {
    static void trigger_kretprobe_multi_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_kretprobe_multi, true);
    load_ctx();
    attach_bpf(ctx.skel.progs.bench_trigger_kretprobe_multi);
    }
#[no_mangle]
unsafe extern "C" fn trigger_fentry_setup() {
    static void trigger_fentry_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_fentry, true);
    load_ctx();
    attach_bpf(ctx.skel.progs.bench_trigger_fentry);
    }
#[no_mangle]
unsafe extern "C" fn attach_ksyms_all(empty: *mut bpf_program, kretprobe: bool) {
    static void attach_ksyms_all(struct bpf_program *empty, bool kretprobe)
    {
    LIBBPF_OPTS(bpf_kprobe_multi_opts, opts);
    struct bpf_link *link = core::ptr::null_mut();
    struct ksyms *ksyms = core::ptr::null_mut();
// Some recursive functions will be skipped in
// bpf_get_ksyms -> skip_entry, as they can introduce sufficient
// overhead. However, it's difficut to skip all the recursive
// functions for a debug kernel.
//
// So, don't run the kprobe-multi-all and kretprobe-multi-all on
// a debug kernel.
//
    if (bpf_get_ksyms(&ksyms, true)) {
    fprintf(stderr, "failed to get ksyms\n");
    exit(1);
    }
    opts.syms = (const char **)ksyms.filtered_syms;
    opts.cnt = ksyms.filtered_cnt;
    opts.retprobe = kretprobe;
// attach empty to all the kernel functions except bpf_get_numa_node_id.
    link = bpf_program__attach_kprobe_multi_opts(empty, core::ptr::null_mut(), &opts);
    free_kallsyms_local(ksyms);
    if (!link) {
    fprintf(stderr, "failed to attach bpf_program__attach_kprobe_multi_opts to all\n");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn trigger_kprobe_multi_all_setup() {
    static void trigger_kprobe_multi_all_setup(void)
    {
    struct bpf_program *prog, *empty;
    setup_ctx();
    empty = ctx.skel.progs.bench_kprobe_multi_empty;
    prog = ctx.skel.progs.bench_trigger_kprobe_multi;
    bpf_program__set_autoload(empty, true);
    bpf_program__set_autoload(prog, true);
    load_ctx();
    attach_ksyms_all(empty, false);
    attach_bpf(prog);
    }
#[no_mangle]
unsafe extern "C" fn trigger_kretprobe_multi_all_setup() {
    static void trigger_kretprobe_multi_all_setup(void)
    {
    struct bpf_program *prog, *empty;
    setup_ctx();
    empty = ctx.skel.progs.bench_kretprobe_multi_empty;
    prog = ctx.skel.progs.bench_trigger_kretprobe_multi;
    bpf_program__set_autoload(empty, true);
    bpf_program__set_autoload(prog, true);
    load_ctx();
    attach_ksyms_all(empty, true);
    attach_bpf(prog);
    }
#[no_mangle]
unsafe extern "C" fn trigger_fexit_setup() {
    static void trigger_fexit_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_fexit, true);
    load_ctx();
    attach_bpf(ctx.skel.progs.bench_trigger_fexit);
    }
#[no_mangle]
unsafe extern "C" fn trigger_fmodret_setup() {
    static void trigger_fmodret_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.trigger_driver, false);
    bpf_program__set_autoload(ctx.skel.progs.trigger_driver_kfunc, true);
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_fmodret, true);
    load_ctx();
// override driver program
    ctx.driver_prog_fd = bpf_program__fd(ctx.skel.progs.trigger_driver_kfunc);
    attach_bpf(ctx.skel.progs.bench_trigger_fmodret);
    }
#[no_mangle]
unsafe extern "C" fn trigger_tp_setup() {
    static void trigger_tp_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.trigger_driver, false);
    bpf_program__set_autoload(ctx.skel.progs.trigger_driver_kfunc, true);
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_tp, true);
    load_ctx();
// override driver program
    ctx.driver_prog_fd = bpf_program__fd(ctx.skel.progs.trigger_driver_kfunc);
    attach_bpf(ctx.skel.progs.bench_trigger_tp);
    }
#[no_mangle]
unsafe extern "C" fn trigger_rawtp_setup() {
    static void trigger_rawtp_setup(void)
    {
    setup_ctx();
    bpf_program__set_autoload(ctx.skel.progs.trigger_driver, false);
    bpf_program__set_autoload(ctx.skel.progs.trigger_driver_kfunc, true);
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_rawtp, true);
    load_ctx();
// override driver program
    ctx.driver_prog_fd = bpf_program__fd(ctx.skel.progs.trigger_driver_kfunc);
    attach_bpf(ctx.skel.progs.bench_trigger_rawtp);
    }
// make sure call is not inlined and not avoided by compiler, so __weak and
// inline asm volatile in the body of the function
//
// There is a performance difference between uprobing at nop location vs other
// instructions. So use two different targets, one of which starts with nop
// and another doesn't.
//
// GCC doesn't generate stack setup preamble for these functions due to them
// having no input arguments and doing nothing in the body.
//
#[no_mangle]
pub unsafe extern "C" fn uprobe_target_nop() -> __nocf_check __weak void {
    __nocf_check __weak void uprobe_target_nop(void)
    {
    asm volatile ("nop");
    }
#[no_mangle]
pub unsafe extern "C" fn opaque_noop_func() -> __weak void {
    __weak void opaque_noop_func(void)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_target_push() -> __nocf_check __weak int {
    __nocf_check __weak int uprobe_target_push(void)
    {
// overhead of function call is negligible compared to uprobe
// triggering, so this shouldn't affect benchmark results much
//
    opaque_noop_func();
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn uprobe_target_ret() -> __nocf_check __weak void {
    __nocf_check __weak void uprobe_target_ret(void)
    {
    asm volatile ("");
    }
    static void *uprobe_producer_count(void *input)
    {
    while (true) {
    uprobe_target_nop();
    inc_counter(base_hits);
    }
    return core::ptr::null_mut();
    }
    static void *uprobe_producer_nop(void *input)
    {
    while (true)
    uprobe_target_nop();
    return core::ptr::null_mut();
    }
    static void *uprobe_producer_push(void *input)
    {
    while (true)
    uprobe_target_push();
    return core::ptr::null_mut();
    }
    static void *uprobe_producer_ret(void *input)
    {
    while (true)
    uprobe_target_ret();
    return core::ptr::null_mut();
    }

#[no_mangle]
pub unsafe extern "C" fn uprobe_target_nop10() -> __nocf_check __weak void {
    __nocf_check __weak void uprobe_target_nop10(void)
    {
    asm volatile (".byte 0x66, 0x2e, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00");
    }
    static void *uprobe_producer_nop10(void *input)
    {
    while (true)
    uprobe_target_nop10();
    return core::ptr::null_mut();
    }
    void usdt_1(void);
    void usdt_2(void);
    static void *uprobe_producer_usdt_nop(void *input)
    {
    while (true)
    usdt_1();
    return core::ptr::null_mut();
    }
    static void *uprobe_producer_usdt_nop10(void *input)
    {
    while (true)
    usdt_2();
    return core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn usetup(use_retprobe: bool, use_multi: bool, target_addr: *mut c_void) {
    static void usetup(bool use_retprobe, bool use_multi, void *target_addr)
    {
    size_t uprobe_offset;
    struct bpf_link *link;
    int err;
    setup_libbpf();
    ctx.skel = trigger_bench__open();
    if (!ctx.skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    if (use_multi)
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_uprobe_multi, true);
    else
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_uprobe, true);
    err = trigger_bench__load(ctx.skel);
    if (err) {
    fprintf(stderr, "failed to load skeleton\n");
    exit(1);
    }
    uprobe_offset = get_uprobe_offset(target_addr);
    if (use_multi) {
    LIBBPF_OPTS(bpf_uprobe_multi_opts, opts,
    .retprobe = use_retprobe,
    .cnt = 1,
    .offsets = &uprobe_offset,
    );
    link = bpf_program__attach_uprobe_multi(
    ctx.skel.progs.bench_trigger_uprobe_multi,
    -1 /* all PIDs */, "/proc/self/exe", core::ptr::null_mut(), &opts);
    ctx.skel.links.bench_trigger_uprobe_multi = link;
    } else {
    link = bpf_program__attach_uprobe(ctx.skel.progs.bench_trigger_uprobe,
    use_retprobe,
    -1 /* all PIDs */,
    "/proc/self/exe",
    uprobe_offset);
    ctx.skel.links.bench_trigger_uprobe = link;
    }
    if (!link) {
    fprintf(stderr, "failed to attach %s!\n", use_multi ? "multi-uprobe" : "uprobe");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn usermode_count_setup() {
    static void usermode_count_setup(void)
    {
    ctx.usermode_counters = true;
    }
#[no_mangle]
unsafe extern "C" fn uprobe_nop_setup() {
    static void uprobe_nop_setup(void)
    {
    usetup(false, false /* !use_multi */, &uprobe_target_nop);
    }
#[no_mangle]
unsafe extern "C" fn uretprobe_nop_setup() {
    static void uretprobe_nop_setup(void)
    {
    usetup(true, false /* !use_multi */, &uprobe_target_nop);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_push_setup() {
    static void uprobe_push_setup(void)
    {
    usetup(false, false /* !use_multi */, &uprobe_target_push);
    }
#[no_mangle]
unsafe extern "C" fn uretprobe_push_setup() {
    static void uretprobe_push_setup(void)
    {
    usetup(true, false /* !use_multi */, &uprobe_target_push);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_ret_setup() {
    static void uprobe_ret_setup(void)
    {
    usetup(false, false /* !use_multi */, &uprobe_target_ret);
    }
#[no_mangle]
unsafe extern "C" fn uretprobe_ret_setup() {
    static void uretprobe_ret_setup(void)
    {
    usetup(true, false /* !use_multi */, &uprobe_target_ret);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_multi_nop_setup() {
    static void uprobe_multi_nop_setup(void)
    {
    usetup(false, true /* use_multi */, &uprobe_target_nop);
    }
#[no_mangle]
unsafe extern "C" fn uretprobe_multi_nop_setup() {
    static void uretprobe_multi_nop_setup(void)
    {
    usetup(true, true /* use_multi */, &uprobe_target_nop);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_multi_push_setup() {
    static void uprobe_multi_push_setup(void)
    {
    usetup(false, true /* use_multi */, &uprobe_target_push);
    }
#[no_mangle]
unsafe extern "C" fn uretprobe_multi_push_setup() {
    static void uretprobe_multi_push_setup(void)
    {
    usetup(true, true /* use_multi */, &uprobe_target_push);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_multi_ret_setup() {
    static void uprobe_multi_ret_setup(void)
    {
    usetup(false, true /* use_multi */, &uprobe_target_ret);
    }
#[no_mangle]
unsafe extern "C" fn uretprobe_multi_ret_setup() {
    static void uretprobe_multi_ret_setup(void)
    {
    usetup(true, true /* use_multi */, &uprobe_target_ret);
    }

#[no_mangle]
unsafe extern "C" fn uprobe_nop10_setup() {
    static void uprobe_nop10_setup(void)
    {
    usetup(false, false /* !use_multi */, &uprobe_target_nop10);
    }
#[no_mangle]
unsafe extern "C" fn uretprobe_nop10_setup() {
    static void uretprobe_nop10_setup(void)
    {
    usetup(true, false /* !use_multi */, &uprobe_target_nop10);
    }
#[no_mangle]
unsafe extern "C" fn uprobe_multi_nop10_setup() {
    static void uprobe_multi_nop10_setup(void)
    {
    usetup(false, true /* use_multi */, &uprobe_target_nop10);
    }
#[no_mangle]
unsafe extern "C" fn uretprobe_multi_nop10_setup() {
    static void uretprobe_multi_nop10_setup(void)
    {
    usetup(true, true /* use_multi */, &uprobe_target_nop10);
    }
#[no_mangle]
unsafe extern "C" fn usdt_setup(name: *const c_char) {
    static void usdt_setup(const char *name)
    {
    struct bpf_link *link;
    int err;
    setup_libbpf();
    ctx.skel = trigger_bench__open();
    if (!ctx.skel) {
    fprintf(stderr, "failed to open skeleton\n");
    exit(1);
    }
    bpf_program__set_autoload(ctx.skel.progs.bench_trigger_usdt, true);
    err = trigger_bench__load(ctx.skel);
    if (err) {
    fprintf(stderr, "failed to load skeleton\n");
    exit(1);
    }
    link = bpf_program__attach_usdt(ctx.skel.progs.bench_trigger_usdt,
    0 /*self*/, "/proc/self/exe",
    "optimized_attach", name, core::ptr::null_mut());
    if (libbpf_get_error(link)) {
    fprintf(stderr, "failed to attach optimized_attach:%s usdt probe\n", name);
    exit(1);
    }
    ctx.skel.links.bench_trigger_usdt = link;
    }
#[no_mangle]
unsafe extern "C" fn usdt_nop_setup() {
    static void usdt_nop_setup(void)
    {
    usdt_setup("usdt_1");
    }
#[no_mangle]
unsafe extern "C" fn usdt_nop10_setup() {
    static void usdt_nop10_setup(void)
    {
    usdt_setup("usdt_2");
    }

    const struct bench bench_trig_syscall_count = {
    .name = "trig-syscall-count",
    .validate = trigger_validate,
    .setup = trigger_syscall_count_setup,
    .producer_thread = trigger_producer,
    .measure = trigger_measure,
    .report_progress = hits_drops_report_progress,
    .report_final = hits_drops_report_final,
    };
// batched (staying mostly in kernel) kprobe/fentry benchmarks

    const struct bench bench_trig_##KIND = {				\
    .name = "trig-" NAME,						\
    .setup = trigger_##KIND##_setup,				\
    .producer_thread = trigger_producer_batch,			\
    .measure = trigger_measure,					\
    .report_progress = hits_drops_report_progress,			\
    .report_final = hits_drops_report_final,			\
    .argp = &bench_trigger_batch_argp,				\
    }
    BENCH_TRIG_KERNEL(kernel_count, "kernel-count");
    BENCH_TRIG_KERNEL(kprobe, "kprobe");
    BENCH_TRIG_KERNEL(kretprobe, "kretprobe");
    BENCH_TRIG_KERNEL(kprobe_multi, "kprobe-multi");
    BENCH_TRIG_KERNEL(kretprobe_multi, "kretprobe-multi");
    BENCH_TRIG_KERNEL(fentry, "fentry");
    BENCH_TRIG_KERNEL(kprobe_multi_all, "kprobe-multi-all");
    BENCH_TRIG_KERNEL(kretprobe_multi_all, "kretprobe-multi-all");
    BENCH_TRIG_KERNEL(fexit, "fexit");
    BENCH_TRIG_KERNEL(fmodret, "fmodret");
    BENCH_TRIG_KERNEL(tp, "tp");
    BENCH_TRIG_KERNEL(rawtp, "rawtp");
// uprobe benchmarks

    const struct bench bench_trig_##KIND = {				\
    .name = "trig-" NAME,						\
    .validate = trigger_validate,					\
    .setup = KIND##_setup,						\
    .producer_thread = uprobe_producer_##PRODUCER,			\
    .measure = trigger_measure,					\
    .report_progress = hits_drops_report_progress,			\
    .report_final = hits_drops_report_final,			\
    }
    BENCH_TRIG_USERMODE(usermode_count, count, "usermode-count");
    BENCH_TRIG_USERMODE(uprobe_nop, nop, "uprobe-nop");
    BENCH_TRIG_USERMODE(uprobe_push, push, "uprobe-push");
    BENCH_TRIG_USERMODE(uprobe_ret, ret, "uprobe-ret");
    BENCH_TRIG_USERMODE(uretprobe_nop, nop, "uretprobe-nop");
    BENCH_TRIG_USERMODE(uretprobe_push, push, "uretprobe-push");
    BENCH_TRIG_USERMODE(uretprobe_ret, ret, "uretprobe-ret");
    BENCH_TRIG_USERMODE(uprobe_multi_nop, nop, "uprobe-multi-nop");
    BENCH_TRIG_USERMODE(uprobe_multi_push, push, "uprobe-multi-push");
    BENCH_TRIG_USERMODE(uprobe_multi_ret, ret, "uprobe-multi-ret");
    BENCH_TRIG_USERMODE(uretprobe_multi_nop, nop, "uretprobe-multi-nop");
    BENCH_TRIG_USERMODE(uretprobe_multi_push, push, "uretprobe-multi-push");
    BENCH_TRIG_USERMODE(uretprobe_multi_ret, ret, "uretprobe-multi-ret");

    BENCH_TRIG_USERMODE(uprobe_nop10, nop10, "uprobe-nop10");
    BENCH_TRIG_USERMODE(uretprobe_nop10, nop10, "uretprobe-nop10");
    BENCH_TRIG_USERMODE(uprobe_multi_nop10, nop10, "uprobe-multi-nop10");
    BENCH_TRIG_USERMODE(uretprobe_multi_nop10, nop10, "uretprobe-multi-nop10");
    BENCH_TRIG_USERMODE(usdt_nop, usdt_nop, "usdt-nop");
    BENCH_TRIG_USERMODE(usdt_nop10, usdt_nop10, "usdt-nop10");
