//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/riscv/hwprobe/cbo.c
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
// Copyright (c) 2023 Ventana Micro Systems Inc.
//
// Run with 'taskset -c <cpu-list> cbo' to only execute hwprobe on a
// subset of cpus, as well as only executing the tests on those cpus.
//
// Macro flag: #define _GNU_SOURCE

    le32_bswap(0 << 25 | (uint32_t)(fn) << 20 | 10 << 15 | 6 << 12 | 0 << 7 | 19)
    static char mem[4096] __aligned(4096) = { [0 ... 4095] = 0xa5 };
    static bool got_fault;
#[no_mangle]
unsafe extern "C" fn fault_handler(sig: c_int, info: *mut siginfo_t, context: *mut c_void) {
    static void fault_handler(int sig, siginfo_t *info, void *context)
    {
    unsigned long *regs = (unsigned long *)&((ucontext_t *)context).uc_mcontext;
    let mut insn: u32 = *(uint32_t *)regs[0];
    if (sig == SIGILL)
    assert(insn == MK_CBO(regs[11]));
    if (sig == SIGSEGV || sig == SIGBUS)
    assert(insn == MK_PREFETCH(regs[11]));
    got_fault = true;
    regs[0] += 4;
    }

    ({										\
    asm volatile(								\
    "mv	a0, %0\n"							\
    "li	a1, %1\n"							\
    ".4byte	%2\n"								\
    : : "r" (base), "i" (fn), "i" (MK_CBO(fn)) : "a0", "a1", "memory");	\
    })

    ({										\
    asm volatile(								\
    "mv	a0, %0\n"							\
    "li	a1, %1\n"							\
    ".4byte	%2\n"								\
    : : "r" (base), "i" (fn), "i" (MK_PREFETCH(fn)) : "a0", "a1");		\
    })
    static void cbo_inval(char *base) { cbo_insn(base, 0); }
    static void cbo_clean(char *base) { cbo_insn(base, 1); }
    static void cbo_flush(char *base) { cbo_insn(base, 2); }
    static void cbo_zero(char *base)  { cbo_insn(base, 4); }
    static void prefetch_i(char *base) { prefetch_insn(base, 0); }
    static void prefetch_r(char *base) { prefetch_insn(base, 1); }
    static void prefetch_w(char *base) { prefetch_insn(base, 3); }
#[no_mangle]
unsafe extern "C" fn test_no_cbo_inval(arg: *mut c_void) {
    static void test_no_cbo_inval(void *arg)
    {
    ksft_print_msg("Testing cbo.inval instruction remain privileged\n");
    got_fault = false;
    cbo_inval(&mem[0]);
    ksft_test_result(got_fault, "No cbo.inval\n");
    }
#[no_mangle]
unsafe extern "C" fn test_no_zicbom(arg: *mut c_void) {
    static void test_no_zicbom(void *arg)
    {
    ksft_print_msg("Testing Zicbom instructions remain privileged\n");
    got_fault = false;
    cbo_clean(&mem[0]);
    ksft_test_result(got_fault, "No cbo.clean\n");
    got_fault = false;
    cbo_flush(&mem[0]);
    ksft_test_result(got_fault, "No cbo.flush\n");
    }
#[no_mangle]
unsafe extern "C" fn test_no_zicboz(arg: *mut c_void) {
    static void test_no_zicboz(void *arg)
    {
    ksft_print_msg("No Zicboz, testing cbo.zero remains privileged\n");
    got_fault = false;
    cbo_zero(&mem[0]);
    ksft_test_result(got_fault, "No cbo.zero\n");
    }
#[no_mangle]
unsafe extern "C" fn is_power_of_2(n: __u64) -> bool {
    static bool is_power_of_2(__u64 n)
    {
    return n != 0 && (n & (n - 1)) == 0;
    }
#[no_mangle]
unsafe extern "C" fn test_zicbop(arg: *mut c_void) {
    static void test_zicbop(void *arg)
    {
    struct riscv_hwprobe pair = {
    .key = RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE,
    };
    struct sigaction act = {
    .sa_sigaction = &fault_handler,
    .sa_flags = SA_SIGINFO
    };
    struct sigaction dfl = {
    .sa_handler = SIG_DFL
    };
    cpu_set_t *cpus = (cpu_set_t *)arg;
    __u64 block_size;
    long rc;
    rc = sigaction(SIGSEGV, &act, core::ptr::null_mut());
    assert(rc == 0);
    rc = sigaction(SIGBUS, &act, core::ptr::null_mut());
    assert(rc == 0);
    rc = riscv_hwprobe(&pair, 1, sizeof(cpu_set_t), (unsigned long *)cpus, 0);
    block_size = pair.value;
    ksft_test_result(rc == 0 && pair.key == RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE &&
    is_power_of_2(block_size), "Zicbop block size\n");
    ksft_print_msg("Zicbop block size: %llu\n", block_size);
    got_fault = false;
    prefetch_i(&mem[0]);
    prefetch_r(&mem[0]);
    prefetch_w(&mem[0]);
    ksft_test_result(!got_fault, "Zicbop prefetch.* on valid address\n");
    got_fault = false;
    prefetch_i(core::ptr::null_mut());
    prefetch_r(core::ptr::null_mut());
    prefetch_w(core::ptr::null_mut());
    ksft_test_result(!got_fault, "Zicbop prefetch.* on core::ptr::null_mut()\n");
    rc = sigaction(SIGBUS, &dfl, core::ptr::null_mut());
    assert(rc == 0);
    rc = sigaction(SIGSEGV, &dfl, core::ptr::null_mut());
    assert(rc == 0);
    }
#[no_mangle]
unsafe extern "C" fn test_zicbom(arg: *mut c_void) {
    static void test_zicbom(void *arg)
    {
    struct riscv_hwprobe pair = {
    .key = RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE,
    };
    cpu_set_t *cpus = (cpu_set_t *)arg;
    __u64 block_size;
    long rc;
    rc = riscv_hwprobe(&pair, 1, sizeof(cpu_set_t), (unsigned long *)cpus, 0);
    block_size = pair.value;
    ksft_test_result(rc == 0 && pair.key == RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE &&
    is_power_of_2(block_size), "Zicbom block size\n");
    ksft_print_msg("Zicbom block size: %llu\n", block_size);
    got_fault = false;
    cbo_clean(&mem[block_size]);
    ksft_test_result(!got_fault, "cbo.clean\n");
    got_fault = false;
    cbo_flush(&mem[block_size]);
    ksft_test_result(!got_fault, "cbo.flush\n");
    }
#[no_mangle]
unsafe extern "C" fn test_zicboz(arg: *mut c_void) {
    static void test_zicboz(void *arg)
    {
    struct riscv_hwprobe pair = {
    .key = RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE,
    };
    cpu_set_t *cpus = (cpu_set_t *)arg;
    __u64 block_size;
    int i, j;
    long rc;
    rc = riscv_hwprobe(&pair, 1, sizeof(cpu_set_t), (unsigned long *)cpus, 0);
    block_size = pair.value;
    ksft_test_result(rc == 0 && pair.key == RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE &&
    is_power_of_2(block_size), "Zicboz block size\n");
    ksft_print_msg("Zicboz block size: %llu\n", block_size);
    got_fault = false;
    cbo_zero(&mem[block_size]);
    ksft_test_result(!got_fault, "cbo.zero\n");
    if (got_fault || !is_power_of_2(block_size)) {
    ksft_test_result_skip("cbo.zero check\n");
    return;
    }
    assert(block_size <= 1024);
    for (i = 0; i < 4096 / block_size; ++i) {
    if (i % 2)
    cbo_zero(&mem[i * block_size]);
    }
    for (i = 0; i < 4096 / block_size; ++i) {
    let mut expected: c_char = i % 2 ? 0x0 : 0xa5;
    for (j = 0; j < block_size; ++j) {
    if (mem[i * block_size + j] != expected) {
    ksft_test_result_fail("cbo.zero check\n");
    ksft_print_msg("cbo.zero check: mem[%llu] != 0x%x\n",
    i * block_size + j, expected);
    return;
    }
    }
    }
    ksft_test_result_pass("cbo.zero check\n");
    }
#[no_mangle]
unsafe extern "C" fn check_no_zicbo_cpus(cpus: *mut cpu_set_t, cbo: __u64) {
    static void check_no_zicbo_cpus(cpu_set_t *cpus, __u64 cbo)
    {
    struct riscv_hwprobe pair = {
    .key = RISCV_HWPROBE_KEY_IMA_EXT_0,
    };
    cpu_set_t one_cpu;
    let mut i: c_int = 0, c = 0;
    long rc;
    char *cbostr;
    while (i++ < CPU_COUNT(cpus)) {
    while (!CPU_ISSET(c, cpus))
    ++c;
    CPU_ZERO(&one_cpu);
    CPU_SET(c, &one_cpu);
    rc = riscv_hwprobe(&pair, 1, sizeof(cpu_set_t), (unsigned long *)&one_cpu, 0);
    assert(rc == 0 && pair.key == RISCV_HWPROBE_KEY_IMA_EXT_0);
    switch (cbo) {
    case RISCV_HWPROBE_EXT_ZICBOZ:
    cbostr = "Zicboz";
    break;
    case RISCV_HWPROBE_EXT_ZICBOM:
    cbostr = "Zicbom";
    break;
    case RISCV_HWPROBE_EXT_ZICBOP:
    cbostr = "Zicbop";
    break;
    default:
    ksft_exit_fail_msg("Internal error: invalid cbo %llu\n", cbo);
    }
    if (pair.value & cbo)
    ksft_exit_fail_msg("%s is only present on a subset of harts.\n"
    "Use taskset to select a set of harts where %s\n"
    "presence (present or not) is consistent for each hart\n",
    cbostr, cbostr);
    ++c;
    }
    }
    enum {
    TEST_ZICBOZ,
    TEST_NO_ZICBOZ,
    TEST_ZICBOM,
    TEST_NO_ZICBOM,
    TEST_NO_CBO_INVAL,
    TEST_ZICBOP,
    };
    static struct test_info {
    bool enabled;
    unsigned int nr_tests;
    void (*test_fn)(void *arg);
    } tests[] = {
    [TEST_ZICBOZ]		= { .nr_tests = 3, test_zicboz },
    [TEST_NO_ZICBOZ]	= { .nr_tests = 1, test_no_zicboz },
    [TEST_ZICBOM]		= { .nr_tests = 3, test_zicbom },
    [TEST_NO_ZICBOM]	= { .nr_tests = 2, test_no_zicbom },
    [TEST_NO_CBO_INVAL]	= { .nr_tests = 1, test_no_cbo_inval },
    [TEST_ZICBOP]		= { .nr_tests = 3, test_zicbop },
    };
    static const struct option long_opts[] = {
    {"zicbom-raises-sigill", no_argument, 0, 'm'},
    {"zicboz-raises-sigill", no_argument, 0, 'z'},
    {0, 0, 0, 0}
    };
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct sigaction act = {
    .sa_sigaction = &fault_handler,
    .sa_flags = SA_SIGINFO,
    };
    struct riscv_hwprobe pair;
    let mut plan: c_uint = 0;
    cpu_set_t cpus;
    long rc;
    int i, opt, long_index;
    long_index = 0;
    while ((opt = getopt_long(argc, argv, "mz", long_opts, &long_index)) != -1) {
    switch (opt) {
    case 'm':
    tests[TEST_NO_ZICBOM].enabled = true;
    tests[TEST_NO_CBO_INVAL].enabled = true;
    rc = sigaction(SIGILL, &act, core::ptr::null_mut());
    assert(rc == 0);
    break;
    case 'z':
    tests[TEST_NO_ZICBOZ].enabled = true;
    tests[TEST_NO_CBO_INVAL].enabled = true;
    rc = sigaction(SIGILL, &act, core::ptr::null_mut());
    assert(rc == 0);
    break;
    case '?':
    fprintf(stderr,
    "Usage: %s [--zicbom-raises-sigill|-m] [--zicboz-raises-sigill|-z]\n",
    argv[0]);
    exit(1);
    default:
    break;
    }
    }
    rc = sched_getaffinity(0, sizeof(cpu_set_t), &cpus);
    assert(rc == 0);
    ksft_print_header();
    pair.key = RISCV_HWPROBE_KEY_IMA_EXT_0;
    rc = riscv_hwprobe(&pair, 1, sizeof(cpu_set_t), (unsigned long *)&cpus, 0);
    if (rc < 0)
    ksft_exit_fail_msg("hwprobe() failed with %ld\n", rc);
    assert(rc == 0 && pair.key == RISCV_HWPROBE_KEY_IMA_EXT_0);
    if (pair.value & RISCV_HWPROBE_EXT_ZICBOZ) {
    tests[TEST_ZICBOZ].enabled = true;
    tests[TEST_NO_ZICBOZ].enabled = false;
    } else {
    check_no_zicbo_cpus(&cpus, RISCV_HWPROBE_EXT_ZICBOZ);
    }
    if (pair.value & RISCV_HWPROBE_EXT_ZICBOM) {
    tests[TEST_ZICBOM].enabled = true;
    tests[TEST_NO_ZICBOM].enabled = false;
    } else {
    check_no_zicbo_cpus(&cpus, RISCV_HWPROBE_EXT_ZICBOM);
    }
    if (pair.value & RISCV_HWPROBE_EXT_ZICBOP)
    tests[TEST_ZICBOP].enabled = true;
    else
    check_no_zicbo_cpus(&cpus, RISCV_HWPROBE_EXT_ZICBOP);
    for (i = 0; i < ARRAY_SIZE(tests); ++i)
    plan += tests[i].enabled ? tests[i].nr_tests : 0;
    if (plan == 0)
    ksft_print_msg("No tests enabled.\n");
    else
    ksft_set_plan(plan);
    for (i = 0; i < ARRAY_SIZE(tests); ++i) {
    if (tests[i].enabled)
    tests[i].test_fn(&cpus);
    }
    ksft_finished();
    }
