//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/vDSO/vdso_test_getrandom.c
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
// Copyright (C) 2022-2024 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
//

    do {								\
    (vsp).tv_sec = (tsp).tv_sec - (usp).tv_sec;		\
    (vsp).tv_nsec = (tsp).tv_nsec - (usp).tv_nsec;	\
    if ((vsp).tv_nsec < 0) {				\
    (vsp).tv_sec--;				\
    (vsp).tv_nsec += 1000000000L;			\
    }							\
    } while (0)

    do { if (!(condition)) ksft_exit_fail_msg("Assertion failed: %s\n", #condition); } while (0)
    static struct {
    pthread_mutex_t lock;
    void **states;
    size_t len, cap;
    ssize_t(*fn)(void *, size_t, unsigned long, void *, size_t);
    struct vgetrandom_opaque_params params;
    } vgrnd = {
    .lock = PTHREAD_MUTEX_INITIALIZER
    };
    static void *vgetrandom_get_state(void)
    {
    void *state = core::ptr::null_mut();
    pthread_mutex_lock(&vgrnd.lock);
    if (!vgrnd.len) {
    let mut page_size: usize = getpagesize();
    size_t new_cap;
    size_t alloc_size, num = sysconf(_SC_NPROCESSORS_ONLN); /* Just a decent heuristic. */
    size_t state_size_aligned, cache_line_size = sysconf(_SC_LEVEL1_DCACHE_LINESIZE) ?: 1;
    void *new_block, *new_states;
    state_size_aligned = (vgrnd.params.size_of_opaque_state + cache_line_size - 1) & (~(cache_line_size - 1));
    alloc_size = (num * state_size_aligned + page_size - 1) & (~(page_size - 1));
    num = (page_size / state_size_aligned) * (alloc_size / page_size);
    new_block = mmap(0, alloc_size, vgrnd.params.mmap_prot, vgrnd.params.mmap_flags, -1, 0);
    if (new_block == MAP_FAILED)
    goto out;
    new_cap = vgrnd.cap + num;
    new_states = reallocarray(vgrnd.states, new_cap, sizeof(*vgrnd.states));
    if (!new_states)
    goto unmap;
    vgrnd.cap = new_cap;
    vgrnd.states = new_states;
    for (size_t i = 0; i < num; ++i) {
    if (((uintptr_t)new_block & (page_size - 1)) + vgrnd.params.size_of_opaque_state > page_size)
    new_block = (void *)(((uintptr_t)new_block + page_size - 1) & (~(page_size - 1)));
    vgrnd.states[i] = new_block;
    new_block += state_size_aligned;
    }
    vgrnd.len = num;
    goto success;
    unmap:
    munmap(new_block, alloc_size);
    goto out;
    }
    success:
    state = vgrnd.states[--vgrnd.len];
    out:
    pthread_mutex_unlock(&vgrnd.lock);
    return state;
    }
    __attribute__((unused)) /* Example for libc implementors */
#[no_mangle]
unsafe extern "C" fn vgetrandom_put_state(state: *mut c_void) {
    static void vgetrandom_put_state(void *state)
    {
    if (!state)
    return;
    pthread_mutex_lock(&vgrnd.lock);
    vgrnd.states[vgrnd.len++] = state;
    pthread_mutex_unlock(&vgrnd.lock);
    }
#[no_mangle]
unsafe extern "C" fn vgetrandom_init() {
    static void vgetrandom_init(void)
    {
    const char *version = versions[VDSO_VERSION];
    const char *name = names[VDSO_NAMES][6];
    let mut sysinfo_ehdr: c_ulong = getauxval(AT_SYSINFO_EHDR);
    ssize_t ret;
    if (!sysinfo_ehdr)
    ksft_exit_skip("AT_SYSINFO_EHDR is not present\n");
    vdso_init_from_sysinfo_ehdr(sysinfo_ehdr);
    vgrnd.fn = (__typeof__(vgrnd.fn))vdso_sym(version, name);
    if (!vgrnd.fn)
    ksft_exit_skip("%s@%s symbol is missing from vDSO\n", name, version);
    ret = VDSO_CALL(vgrnd.fn, 5, core::ptr::null_mut(), 0, 0, &vgrnd.params, ~0UL);
    if (ret == -ENOSYS)
    ksft_exit_skip("CPU does not have runtime support\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ret) -> else {
    else if (ret)
    ksft_exit_fail_msg("Failed to fetch vgetrandom params: %zd\n", ret);
    }
#[no_mangle]
unsafe extern "C" fn vgetrandom(buf: *mut c_void, len: usize, flags: c_ulong) -> isize {
    static ssize_t vgetrandom(void *buf, size_t len, unsigned long flags)
    {
    static __thread void *state;
    if (!state) {
    state = vgetrandom_get_state();
    ksft_assert(state);
    }
    return VDSO_CALL(vgrnd.fn, 5, buf, len, flags, state, vgrnd.params.size_of_opaque_state);
    }
    enum { TRIALS = 25000000, THREADS = 256 };
    static void *test_vdso_getrandom(void *ctx)
    {
    for (size_t i = 0; i < TRIALS; ++i) {
    unsigned int val;
    let mut ret: isize = vgetrandom(&val, sizeof(val), 0);
    ksft_assert(ret == sizeof(val));
    }
    return core::ptr::null_mut();
    }
    static void *test_libc_getrandom(void *ctx)
    {
    for (size_t i = 0; i < TRIALS; ++i) {
    unsigned int val;
    let mut ret: isize = getrandom(&val, sizeof(val), 0);
    ksft_assert(ret == sizeof(val));
    }
    return core::ptr::null_mut();
    }
    static void *test_syscall_getrandom(void *ctx)
    {
    for (size_t i = 0; i < TRIALS; ++i) {
    unsigned int val;
    let mut ret: isize = syscall(__NR_getrandom, &val, sizeof(val), 0);
    ksft_assert(ret == sizeof(val));
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn bench_single() {
    static void bench_single(void)
    {
    struct timespec start, end, diff;
    clock_gettime(CLOCK_MONOTONIC, &start);
    test_vdso_getrandom(core::ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &end);
    timespecsub(&end, &start, &diff);
    printf("   vdso: %u times in %lu.%09lu seconds\n", TRIALS, diff.tv_sec, diff.tv_nsec);
    clock_gettime(CLOCK_MONOTONIC, &start);
    test_libc_getrandom(core::ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &end);
    timespecsub(&end, &start, &diff);
    printf("   libc: %u times in %lu.%09lu seconds\n", TRIALS, diff.tv_sec, diff.tv_nsec);
    clock_gettime(CLOCK_MONOTONIC, &start);
    test_syscall_getrandom(core::ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &end);
    timespecsub(&end, &start, &diff);
    printf("syscall: %u times in %lu.%09lu seconds\n", TRIALS, diff.tv_sec, diff.tv_nsec);
    }
#[no_mangle]
unsafe extern "C" fn bench_multi() {
    static void bench_multi(void)
    {
    struct timespec start, end, diff;
    pthread_t threads[THREADS];
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (size_t i = 0; i < THREADS; ++i)
    ksft_assert(pthread_create(&threads[i], core::ptr::null_mut(), test_vdso_getrandom, core::ptr::null_mut()) == 0);
    for (size_t i = 0; i < THREADS; ++i)
    pthread_join(threads[i], core::ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &end);
    timespecsub(&end, &start, &diff);
    printf("   vdso: %u x %u times in %lu.%09lu seconds\n", TRIALS, THREADS, diff.tv_sec, diff.tv_nsec);
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (size_t i = 0; i < THREADS; ++i)
    ksft_assert(pthread_create(&threads[i], core::ptr::null_mut(), test_libc_getrandom, core::ptr::null_mut()) == 0);
    for (size_t i = 0; i < THREADS; ++i)
    pthread_join(threads[i], core::ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &end);
    timespecsub(&end, &start, &diff);
    printf("   libc: %u x %u times in %lu.%09lu seconds\n", TRIALS, THREADS, diff.tv_sec, diff.tv_nsec);
    clock_gettime(CLOCK_MONOTONIC, &start);
    for (size_t i = 0; i < THREADS; ++i)
    ksft_assert(pthread_create(&threads[i], core::ptr::null_mut(), test_syscall_getrandom, core::ptr::null_mut()) == 0);
    for (size_t i = 0; i < THREADS; ++i)
    pthread_join(threads[i], core::ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &end);
    timespecsub(&end, &start, &diff);
    printf("   syscall: %u x %u times in %lu.%09lu seconds\n", TRIALS, THREADS, diff.tv_sec, diff.tv_nsec);
    }
#[no_mangle]
unsafe extern "C" fn fill() {
    static void fill(void)
    {
    uint8_t weird_size[323929];
    for (;;)
    vgetrandom(weird_size, sizeof(weird_size), 0);
    }
#[no_mangle]
unsafe extern "C" fn kselftest() {
    static void kselftest(void)
    {
    uint8_t weird_size[1263];
    pid_t child;
    ksft_print_header();
    vgetrandom_init();
    ksft_set_plan(2);
    for (size_t i = 0; i < 1000; ++i) {
    let mut ret: isize = vgetrandom(weird_size, sizeof(weird_size), 0);
    ksft_assert(ret == sizeof(weird_size));
    }
    ksft_test_result_pass("getrandom: PASS\n");
    unshare(CLONE_NEWUSER);
    ksft_assert(unshare(CLONE_NEWTIME) == 0);
    child = fork();
    ksft_assert(child >= 0);
    if (!child) {
    vgetrandom_init();
    child = getpid();
    ksft_assert(ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut(), core::ptr::null_mut()) == 0);
    ksft_assert(kill(child, SIGSTOP) == 0);
    ksft_assert(vgetrandom(weird_size, sizeof(weird_size), 0) == sizeof(weird_size));
    _exit(0);
    }
    for (;;) {
    let mut info: ptrace_syscall_info = { 0 };
    int status;
    ksft_assert(waitpid(child, &status, 0) >= 0);
    if (WIFEXITED(status)) {
    ksft_assert(WEXITSTATUS(status) == 0);
    break;
    }
    ksft_assert(WIFSTOPPED(status));
    if (WSTOPSIG(status) == SIGSTOP)
    ksft_assert(ptrace(PTRACE_SETOPTIONS, child, 0, PTRACE_O_TRACESYSGOOD) == 0);
#[no_mangle]
pub unsafe extern "C" fn if(0x80): WSTOPSIG(status) == (SIGTRAP |) -> else {
    ksft_assert(ptrace(PTRACE_GET_SYSCALL_INFO, child, sizeof(info), &info) > 0);
    if (info.op == PTRACE_SYSCALL_INFO_ENTRY && info.entry.nr == __NR_getrandom &&
    info.entry.args[0] == (uintptr_t)weird_size && info.entry.args[1] == sizeof(weird_size))
    ksft_exit_fail_msg("vgetrandom passed buffer to syscall getrandom unexpectedly\n");
    }
    ksft_assert(ptrace(PTRACE_SYSCALL, child, 0, 0) == 0);
    }
    ksft_test_result_pass("getrandom timens: PASS\n");
    ksft_exit_pass();
    }
#[no_mangle]
unsafe extern "C" fn usage(argv0: *const c_char) {
    static void usage(const char *argv0)
    {
    fprintf(stderr, "Usage: %s [bench-single|bench-multi|fill]\n", argv0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    if (argc == 1) {
    kselftest();
    return 0;
    }
    if (argc != 2) {
    usage(argv[0]);
    return 1;
    }
    vgetrandom_init();
    if (!strcmp(argv[1], "bench-single"))
    bench_single();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "bench-multi")) -> else {
    else if (!strcmp(argv[1], "bench-multi"))
    bench_multi();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "fill")) -> else {
    else if (!strcmp(argv[1], "fill"))
    fill();
    else {
    usage(argv[0]);
    return 1;
    }
    return 0;
    }
