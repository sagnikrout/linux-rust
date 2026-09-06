//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/test_vsyscall.c
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
// Macro flag: #define _GNU_SOURCE

pub const TOTAL_TESTS: c_int = 13;

pub const TOTAL_TESTS: c_int = 8;

// max length of lines in /proc/self/maps - anything longer is skipped here
pub const MAPS_LINE_LEN: c_int = 128;
// vsyscalls and vDSO
    let mut vsyscall_map_r: bool = false, vsyscall_map_x = false;
    typedef long (*gtod_t)(struct timeval *tv, struct timezone *tz);
    let mut vgtod: gtod_t = (gtod_t)VSYS(0xffffffffff600000);
    gtod_t vdso_gtod;
    typedef int (*vgettime_t)(clockid_t, struct timespec *);
    vgettime_t vdso_gettime;
    typedef long (*time_func_t)(time_t *t);
    let mut vtime: time_func_t = (time_func_t)VSYS(0xffffffffff600400);
    time_func_t vdso_time;
    typedef long (*getcpu_t)(unsigned *, unsigned *, void *);
    let mut vgetcpu: getcpu_t = (getcpu_t)VSYS(0xffffffffff600800);
    getcpu_t vdso_getcpu;
#[no_mangle]
unsafe extern "C" fn init_vdso() {
    static void init_vdso(void)
    {
    void *vdso = dlopen("linux-vdso.so.1", RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);
    if (!vdso)
    vdso = dlopen("linux-gate.so.1", RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);
    if (!vdso) {
    ksft_print_msg("[WARN] failed to find vDSO\n");
    return;
    }
    vdso_gtod = (gtod_t)dlsym(vdso, "__vdso_gettimeofday");
    if (!vdso_gtod)
    ksft_print_msg("[WARN] failed to find gettimeofday in vDSO\n");
    vdso_gettime = (vgettime_t)dlsym(vdso, "__vdso_clock_gettime");
    if (!vdso_gettime)
    ksft_print_msg("[WARN] failed to find clock_gettime in vDSO\n");
    vdso_time = (time_func_t)dlsym(vdso, "__vdso_time");
    if (!vdso_time)
    ksft_print_msg("[WARN] failed to find time in vDSO\n");
    vdso_getcpu = (getcpu_t)dlsym(vdso, "__vdso_getcpu");
    if (!vdso_getcpu)
    ksft_print_msg("[WARN] failed to find getcpu in vDSO\n");
    }
// syscalls
#[no_mangle]
pub unsafe extern "C" fn sys_gtod(tv: *mut timeval, tz: *mut timezone) -> c_long {
    static inline long sys_gtod(struct timeval *tv, struct timezone *tz)
    {
    return syscall(SYS_gettimeofday, tv, tz);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_time(t: *mut time_t) -> c_long {
    static inline long sys_time(time_t *t)
    {
    return syscall(SYS_time, t);
    }
    static inline long sys_getcpu(unsigned * cpu, unsigned * node,
    void* cache)
    {
    return syscall(SYS_getcpu, cpu, node, cache);
    }
#[no_mangle]
unsafe extern "C" fn tv_diff(a: *const timeval, b: *const timeval) -> double {
    static double tv_diff(const struct timeval *a, const struct timeval *b)
    {
    return (double)(a.tv_sec - b.tv_sec) +
    (double)((int)a.tv_usec - (int)b.tv_usec) * 1e-6;
    }
    static void check_gtod(const struct timeval *tv_sys1,
    const struct timeval *tv_sys2,
    const struct timezone *tz_sys,
    const char *which,
    const struct timeval *tv_other,
    const struct timezone *tz_other)
    {
    double d1, d2;
    if (tz_other && (tz_sys.tz_minuteswest != tz_other.tz_minuteswest ||
    tz_sys.tz_dsttime != tz_other.tz_dsttime))
    ksft_print_msg("%s tz mismatch\n", which);
    d1 = tv_diff(tv_other, tv_sys1);
    d2 = tv_diff(tv_sys2, tv_other);
    ksft_print_msg("%s time offsets: %lf %lf\n", which, d1, d2);
    ksft_test_result(!(d1 < 0 || d2 < 0), "%s gettimeofday()'s timeval\n", which);
    }
#[no_mangle]
unsafe extern "C" fn test_gtod() {
    static void test_gtod(void)
    {
    struct timeval tv_sys1, tv_sys2, tv_vdso, tv_vsys;
    struct timezone tz_sys, tz_vdso, tz_vsys;
    let mut ret_vdso: c_long = -1;
    let mut ret_vsys: c_long = -1;
    ksft_print_msg("test gettimeofday()\n");
    if (sys_gtod(&tv_sys1, &tz_sys) != 0)
    ksft_exit_fail_msg("syscall gettimeofday: %s\n", strerror(errno));
    if (vdso_gtod)
    ret_vdso = vdso_gtod(&tv_vdso, &tz_vdso);
    if (vsyscall_map_x)
    ret_vsys = vgtod(&tv_vsys, &tz_vsys);
    if (sys_gtod(&tv_sys2, &tz_sys) != 0)
    ksft_exit_fail_msg("syscall gettimeofday: %s\n", strerror(errno));
    if (vdso_gtod) {
    if (ret_vdso == 0)
    check_gtod(&tv_sys1, &tv_sys2, &tz_sys, "vDSO", &tv_vdso, &tz_vdso);
    else
    ksft_test_result_fail("vDSO gettimeofday() failed: %ld\n", ret_vdso);
    } else {
    ksft_test_result_skip("vdso_gtod isn't set\n");
    }
    if (vsyscall_map_x) {
    if (ret_vsys == 0)
    check_gtod(&tv_sys1, &tv_sys2, &tz_sys, "vsyscall", &tv_vsys, &tz_vsys);
    else
    ksft_test_result_fail("vsys gettimeofday() failed: %ld\n", ret_vsys);
    } else {
    ksft_test_result_skip("vsyscall_map_x isn't set\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn test_time() {
    static void test_time(void)
    {
    long t_sys1, t_sys2, t_vdso = 0, t_vsys = 0;
    let mut t2_sys1: c_long = -1, t2_sys2 = -1, t2_vdso = -1, t2_vsys = -1;
    ksft_print_msg("test time()\n");
    t_sys1 = sys_time(&t2_sys1);
    if (vdso_time)
    t_vdso = vdso_time(&t2_vdso);
    if (vsyscall_map_x)
    t_vsys = vtime(&t2_vsys);
    t_sys2 = sys_time(&t2_sys2);
    if (t_sys1 < 0 || t_sys1 != t2_sys1 || t_sys2 < 0 || t_sys2 != t2_sys2) {
    ksft_print_msg("syscall failed (ret1:%ld output1:%ld ret2:%ld output2:%ld)\n",
    t_sys1, t2_sys1, t_sys2, t2_sys2);
    ksft_test_result_skip("vdso_time\n");
    ksft_test_result_skip("vdso_time\n");
    return;
    }
    if (vdso_time) {
    if (t_vdso < 0 || t_vdso != t2_vdso)
    ksft_test_result_fail("vDSO failed (ret:%ld output:%ld)\n",
    t_vdso, t2_vdso);
#[no_mangle]
pub unsafe extern "C" fn if(t_sys2: t_vdso < t_sys1 || t_vdso >) -> else {
    else if (t_vdso < t_sys1 || t_vdso > t_sys2)
    ksft_test_result_fail("vDSO returned the wrong time (%ld %ld %ld)\n",
    t_sys1, t_vdso, t_sys2);
    else
    ksft_test_result_pass("vDSO time() is okay\n");
    } else {
    ksft_test_result_skip("vdso_time isn't set\n");
    }
    if (vsyscall_map_x) {
    if (t_vsys < 0 || t_vsys != t2_vsys)
    ksft_test_result_fail("vsyscall failed (ret:%ld output:%ld)\n",
    t_vsys, t2_vsys);
#[no_mangle]
pub unsafe extern "C" fn if(t_sys2: t_vsys < t_sys1 || t_vsys >) -> else {
    else if (t_vsys < t_sys1 || t_vsys > t_sys2)
    ksft_test_result_fail("vsyscall returned the wrong time (%ld %ld %ld)\n",
    t_sys1, t_vsys, t_sys2);
    else
    ksft_test_result_pass("vsyscall time() is okay\n");
    } else {
    ksft_test_result_skip("vsyscall_map_x isn't set\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn test_getcpu(cpu: c_int) {
    static void test_getcpu(int cpu)
    {
    unsigned int cpu_sys, cpu_vdso, cpu_vsys, node_sys, node_vdso, node_vsys;
    long ret_sys, ret_vdso = -1, ret_vsys = -1;
    let mut node: c_uint = 0;
    let mut have_node: bool = false;
    cpu_set_t cpuset;
    ksft_print_msg("getcpu() on CPU %d\n", cpu);
    CPU_ZERO(&cpuset);
    CPU_SET(cpu, &cpuset);
    if (sched_setaffinity(0, sizeof(cpuset), &cpuset) != 0) {
    ksft_print_msg("failed to force CPU %d\n", cpu);
    ksft_test_result_skip("vdso_getcpu\n");
    ksft_test_result_skip("vsyscall_map_x\n");
    return;
    }
    ret_sys = sys_getcpu(&cpu_sys, &node_sys, 0);
    if (vdso_getcpu)
    ret_vdso = vdso_getcpu(&cpu_vdso, &node_vdso, 0);
    if (vsyscall_map_x)
    ret_vsys = vgetcpu(&cpu_vsys, &node_vsys, 0);
    if (ret_sys == 0) {
    if (cpu_sys != cpu)
    ksft_print_msg("syscall reported CPU %u but should be %d\n",
    cpu_sys, cpu);
    have_node = true;
    node = node_sys;
    }
    if (vdso_getcpu) {
    if (ret_vdso) {
    ksft_test_result_fail("vDSO getcpu() failed\n");
    } else {
    if (!have_node) {
    have_node = true;
    node = node_vdso;
    }
    if (cpu_vdso != cpu || node_vdso != node) {
    if (cpu_vdso != cpu)
    ksft_print_msg("vDSO reported CPU %u but should be %d\n",
    cpu_vdso, cpu);
    if (node_vdso != node)
    ksft_print_msg("vDSO reported node %u but should be %u\n",
    node_vdso, node);
    ksft_test_result_fail("Wrong values\n");
    } else {
    ksft_test_result_pass("vDSO reported correct CPU and node\n");
    }
    }
    } else {
    ksft_test_result_skip("vdso_getcpu isn't set\n");
    }
    if (vsyscall_map_x) {
    if (ret_vsys) {
    ksft_test_result_fail("vsyscall getcpu() failed\n");
    } else {
    if (!have_node) {
    have_node = true;
    node = node_vsys;
    }
    if (cpu_vsys != cpu || node_vsys != node) {
    if (cpu_vsys != cpu)
    ksft_print_msg("vsyscall reported CPU %u but should be %d\n",
    cpu_vsys, cpu);
    if (node_vsys != node)
    ksft_print_msg("vsyscall reported node %u but should be %u\n",
    node_vsys, node);
    ksft_test_result_fail("Wrong values\n");
    } else {
    ksft_test_result_pass("vsyscall reported correct CPU and node\n");
    }
    }
    } else {
    ksft_test_result_skip("vsyscall_map_x isn't set\n");
    }
    }

    static jmp_buf jmpbuf;
    static volatile unsigned long segv_err, segv_trapno;
#[no_mangle]
unsafe extern "C" fn sigsegv(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigsegv(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t *)ctx_void;
    segv_trapno = ctx.uc_mcontext.gregs[REG_TRAPNO];
    segv_err =  ctx.uc_mcontext.gregs[REG_ERR];
    siglongjmp(jmpbuf, 1);
    }
#[no_mangle]
unsafe extern "C" fn test_vsys_r() {
    static void test_vsys_r(void)
    {
    ksft_print_msg("Checking read access to the vsyscall page\n");
    bool can_read;
    if (sigsetjmp(jmpbuf, 1) == 0) {
// (volatile int *)0xffffffffff600000;
    can_read = true;
    } else {
    can_read = false;
    }
    if (can_read && !vsyscall_map_r)
    ksft_test_result_fail("We have read access, but we shouldn't\n");
#[no_mangle]
pub unsafe extern "C" fn if(vsyscall_map_r: !can_read &&) -> else {
    else if (!can_read && vsyscall_map_r)
    ksft_test_result_fail("We don't have read access, but we should\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: can_read) -> else {
    else if (can_read)
    ksft_test_result_pass("We have read access\n");
    else
    ksft_test_result_pass("We do not have read access (trap=%ld, error=0x%lx)\n",
    segv_trapno, segv_err);
    }
#[no_mangle]
unsafe extern "C" fn test_vsys_x() {
    static void test_vsys_x(void)
    {
    if (vsyscall_map_x) {
// We already tested this adequately.
    ksft_test_result_pass("vsyscall_map_x is true\n");
    return;
    }
    ksft_print_msg("Make sure that vsyscalls really cause a fault\n");
    bool can_exec;
    if (sigsetjmp(jmpbuf, 1) == 0) {
    vgtod(core::ptr::null_mut(), core::ptr::null_mut());
    can_exec = true;
    } else {
    can_exec = false;
    }
    if (can_exec)
    ksft_test_result_fail("Executing the vsyscall did not fault\n");
// #GP or #PF (with X86_PF_INSTR)
#[no_mangle]
pub unsafe extern "C" fn if(4))): (segv_trapno == 13) || ((segv_trapno == 14) && (segv_err & (1 <<) -> else {
    else if ((segv_trapno == 13) || ((segv_trapno == 14) && (segv_err & (1 << 4))))
    ksft_test_result_pass("Executing the vsyscall page failed (trap=%ld, error=0x%lx)\n",
    segv_trapno, segv_err);
    else
    ksft_test_result_fail("Execution failed with the wrong error (trap=%ld, error=0x%lx)\n",
    segv_trapno, segv_err);
    }
//
// Debuggers expect ptrace() to be able to peek at the vsyscall page.
// Use process_vm_readv() as a proxy for ptrace() to test this.  We
// want it to work in the vsyscall=emulate case and to fail in the
// vsyscall=xonly case.
//
// It's worth noting that this ABI is a bit nutty.  write(2) can't
// read from the vsyscall page on any kernel version or mode.  The
// fact that ptrace() ever worked was a nice courtesy of old kernels,
// but the code to support it is fairly gross.
//
#[no_mangle]
unsafe extern "C" fn test_process_vm_readv() {
    static void test_process_vm_readv(void)
    {
    char buf[4096];
    struct iovec local, remote;
    int ret;
    ksft_print_msg("process_vm_readv() from vsyscall page\n");
    local.iov_base = buf;
    local.iov_len = 4096;
    remote.iov_base = (void *)0xffffffffff600000;
    remote.iov_len = 4096;
    ret = process_vm_readv(getpid(), &local, 1, &remote, 1, 0);
    if (ret != 4096) {
//
// We expect process_vm_readv() to work if and only if the
// vsyscall page is readable.
//
    ksft_test_result(!vsyscall_map_r,
    "process_vm_readv() failed (ret = %d, errno = %d)\n", ret, errno);
    return;
    }
    if (vsyscall_map_r)
    ksft_test_result(!memcmp(buf, remote.iov_base, sizeof(buf)), "Read data\n");
    else
    ksft_test_result_fail("process_rm_readv() succeeded, but it should have failed in this configuration\n");
    }
#[no_mangle]
unsafe extern "C" fn init_vsys() {
    static void init_vsys(void)
    {
    let mut nerrs: c_int = 0;
    FILE *maps;
    char line[MAPS_LINE_LEN];
    let mut found: bool = false;
    maps = fopen("/proc/self/maps", "r");
    if (!maps) {
    ksft_test_result_skip("Could not open /proc/self/maps -- assuming vsyscall is r-x\n");
    vsyscall_map_r = true;
    return;
    }
    while (fgets(line, MAPS_LINE_LEN, maps)) {
    char r, x;
    void *start, *end;
    char name[MAPS_LINE_LEN];
// sscanf() is safe here as strlen(name) >= strlen(line)
    if (sscanf(line, "%p-%p %c-%cp %*x %*x:%*x %*u %s",
    &start, &end, &r, &x, name) != 5)
    continue;
    if (strcmp(name, "[vsyscall]"))
    continue;
    ksft_print_msg("vsyscall map: %s", line);
    if (start != (void *)0xffffffffff600000 ||
    end != (void *)0xffffffffff601000) {
    ksft_print_msg("address range is nonsense\n");
    nerrs++;
    }
    ksft_print_msg("vsyscall permissions are %c-%c\n", r, x);
    vsyscall_map_r = (r == 'r');
    vsyscall_map_x = (x == 'x');
    found = true;
    break;
    }
    fclose(maps);
    if (!found) {
    ksft_print_msg("no vsyscall map in /proc/self/maps\n");
    vsyscall_map_r = false;
    vsyscall_map_x = false;
    }
    ksft_test_result(!nerrs, "vsyscall map\n");
    }
    static volatile sig_atomic_t num_vsyscall_traps;
#[no_mangle]
unsafe extern "C" fn sigtrap(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigtrap(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t *)ctx_void;
    let mut ip: c_ulong = ctx.uc_mcontext.gregs[REG_RIP];
    if (((ip ^ 0xffffffffff600000UL) & ~0xfffUL) == 0)
    num_vsyscall_traps++;
    }
#[no_mangle]
unsafe extern "C" fn test_emulation() {
    static void test_emulation(void)
    {
    time_t tmp;
    bool is_native;
    if (!vsyscall_map_x) {
    ksft_test_result_skip("vsyscall_map_x isn't set\n");
    return;
    }
    ksft_print_msg("checking that vsyscalls are emulated\n");
    sethandler(SIGTRAP, sigtrap, 0);
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    vtime(&tmp);
    set_eflags(get_eflags() & ~X86_EFLAGS_TF);
//
// If vsyscalls are emulated, we expect a single trap in the
// vsyscall page -- the call instruction will trap with RIP
// pointing to the entry point before emulation takes over.
// In native mode, we expect two traps, since whatever code
// the vsyscall page contains will be more than just a ret
// instruction.
//
    is_native = (num_vsyscall_traps > 1);
    ksft_test_result(!is_native, "vsyscalls are %s (%d instructions in vsyscall page)\n",
    (is_native ? "native" : "emulated"), (int)num_vsyscall_traps);
    }

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut total_tests: c_int = TOTAL_TESTS;
    ksft_print_header();
    ksft_set_plan(total_tests);
    init_vdso();

    init_vsys();

    test_gtod();
    test_time();
    test_getcpu(0);
    test_getcpu(1);

    sethandler(SIGSEGV, sigsegv, 0);
    test_vsys_r();
    test_vsys_x();
    test_process_vm_readv();
    test_emulation();

    ksft_finished();
    }
