//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/vDSO/vdso_test_correctness.c
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
// ldt_gdt.c - Test cases for LDT and GDT access
// Copyright (c) 2011-2015 Andrew Lutomirski
//
// Macro flag: #define _GNU_SOURCE

    static const char *version;
    static const char **name;

pub const __NR_clock_gettime64: c_int = 403;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kernel_timespec {
    pub tv_sec: c_longlong,
    pub tv_nsec: c_longlong,
}

// max length of lines in /proc/self/maps - anything longer is skipped here
pub const MAPS_LINE_LEN: c_int = 128;
    let mut nerrs: c_int = 0;
    typedef int (*vgettime_t)(clockid_t, struct timespec *);
    vgettime_t vdso_clock_gettime;
    typedef int (*vgettime64_t)(clockid_t, struct __kernel_timespec *);
    vgettime64_t vdso_clock_gettime64;
    typedef long (*vgtod_t)(struct timeval *tv, struct timezone *tz);
    vgtod_t vdso_gettimeofday;
    typedef time_t (*vtime_t)(__kernel_time_t *tloc);
    vtime_t vdso_time;
    typedef long (*getcpu_t)(unsigned *, unsigned *, void *);
    getcpu_t vgetcpu;
    getcpu_t vdso_getcpu;
    static void *vsyscall_getcpu(void)
    {

    FILE *maps;
    char line[MAPS_LINE_LEN];
    let mut found: bool = false;
    maps = fopen("/proc/self/maps", "r");
    if (!maps) /* might still be present, but ignore it here, as we test vDSO not vsyscall */
    return core::ptr::null_mut();
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
// assume entries are OK, as we test vDSO here not vsyscall
    found = true;
    break;
    }
    fclose(maps);
    if (!found) {
    printf("Warning: failed to find vsyscall getcpu\n");
    return core::ptr::null_mut();
    }
    return (void *) (0xffffffffff600800);

    return core::ptr::null_mut();

    }
#[no_mangle]
unsafe extern "C" fn fill_function_pointers() {
    static void fill_function_pointers(void)
    {
    let mut sysinfo_ehdr: c_ulong = getauxval(AT_SYSINFO_EHDR);
    if (!sysinfo_ehdr) {
    printf("[WARN]\tfailed to find vDSO\n");
    return;
    }
    vdso_init_from_sysinfo_ehdr(sysinfo_ehdr);
    vdso_getcpu = (getcpu_t)vdso_sym(version, name[4]);
    if (!vdso_getcpu)
    printf("Warning: failed to find getcpu in vDSO\n");
    vgetcpu = (getcpu_t) vsyscall_getcpu();
    vdso_clock_gettime = (vgettime_t)vdso_sym(version, name[1]);
    if (!vdso_clock_gettime)
    printf("Warning: failed to find clock_gettime in vDSO\n");

    vdso_clock_gettime64 = (vgettime64_t)vdso_sym(version, name[5]);
    if (!vdso_clock_gettime64)
    printf("Warning: failed to find clock_gettime64 in vDSO\n");

    vdso_gettimeofday = (vgtod_t)vdso_sym(version, name[0]);
    if (!vdso_gettimeofday)
    printf("Warning: failed to find gettimeofday in vDSO\n");
    vdso_time = (vtime_t)vdso_sym(version, name[2]);
    if (!vdso_time)
    printf("Warning: failed to find time in vDSO\n");
    }
    static long sys_getcpu(unsigned * cpu, unsigned * node,
    void* cache)
    {
    return syscall(__NR_getcpu, cpu, node, cache);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_clock_gettime(id: clockid_t, ts: *mut timespec) -> c_int {
    static inline int sys_clock_gettime(clockid_t id, struct timespec *ts)
    {
    return syscall(__NR_clock_gettime, id, ts);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_clock_gettime64(id: clockid_t, ts: *mut __kernel_timespec) -> c_int {
    static inline int sys_clock_gettime64(clockid_t id, struct __kernel_timespec *ts)
    {
    return syscall(__NR_clock_gettime64, id, ts);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_gettimeofday(tv: *mut timeval, tz: *mut timezone) -> c_int {
    static inline int sys_gettimeofday(struct timeval *tv, struct timezone *tz)
    {
    return syscall(__NR_gettimeofday, tv, tz);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_time(tloc: *mut __kernel_old_time_t) -> __kernel_old_time_t {
    static inline __kernel_old_time_t sys_time(__kernel_old_time_t *tloc)
    {

    return syscall(__NR_time, tloc);

    errno = ENOSYS;
    return -1;

    }
#[no_mangle]
unsafe extern "C" fn test_getcpu() {
    static void test_getcpu(void)
    {
    printf("[RUN]\tTesting getcpu...\n");
    for (int cpu = 0; ; cpu++) {
    cpu_set_t cpuset;
    CPU_ZERO(&cpuset);
    CPU_SET(cpu, &cpuset);
    if (sched_setaffinity(0, sizeof(cpuset), &cpuset) != 0)
    return;
    unsigned cpu_sys, cpu_vdso, cpu_vsys,
    node_sys, node_vdso, node_vsys;
    long ret_sys, ret_vdso = 1, ret_vsys = 1;
    unsigned node;
    ret_sys = sys_getcpu(&cpu_sys, &node_sys, 0);
    if (vdso_getcpu)
    ret_vdso = VDSO_CALL(vdso_getcpu, 3, &cpu_vdso, &node_vdso, 0);
    if (vgetcpu)
    ret_vsys = vgetcpu(&cpu_vsys, &node_vsys, 0);
    if (!ret_sys)
    node = node_sys;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !ret_vdso) -> else {
    else if (!ret_vdso)
    node = node_vdso;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !ret_vsys) -> else {
    else if (!ret_vsys)
    node = node_vsys;
    let mut ok: bool = true;
    if (!ret_sys && (cpu_sys != cpu || node_sys != node))
    ok = false;
    if (!ret_vdso && (cpu_vdso != cpu || node_vdso != node))
    ok = false;
    if (!ret_vsys && (cpu_vsys != cpu || node_vsys != node))
    ok = false;
    printf("[%s]\tCPU %u:", ok ? "OK" : "FAIL", cpu);
    if (!ret_sys)
    printf(" syscall: cpu %u, node %u", cpu_sys, node_sys);
    if (!ret_vdso)
    printf(" vdso: cpu %u, node %u", cpu_vdso, node_vdso);
    if (!ret_vsys)
    printf(" vsyscall: cpu %u, node %u", cpu_vsys,
    node_vsys);
    printf("\n");
    if (!ok)
    nerrs++;
    }
    }
#[no_mangle]
unsafe extern "C" fn ts_leq(a: *const timespec, b: *const timespec) -> bool {
    static bool ts_leq(const struct timespec *a, const struct timespec *b)
    {
    if (a.tv_sec != b.tv_sec)
    return a.tv_sec < b.tv_sec;
    else
    return a.tv_nsec <= b.tv_nsec;
    }
    static bool ts64_leq(const struct __kernel_timespec *a,
    const struct __kernel_timespec *b)
    {
    if (a.tv_sec != b.tv_sec)
    return a.tv_sec < b.tv_sec;
    else
    return a.tv_nsec <= b.tv_nsec;
    }
#[no_mangle]
unsafe extern "C" fn tv_leq(a: *const timeval, b: *const timeval) -> bool {
    static bool tv_leq(const struct timeval *a, const struct timeval *b)
    {
    if (a.tv_sec != b.tv_sec)
    return a.tv_sec < b.tv_sec;
    else
    return a.tv_usec <= b.tv_usec;
    }
    static char const * const clocknames[] = {
    [0] = "CLOCK_REALTIME",
    [1] = "CLOCK_MONOTONIC",
    [2] = "CLOCK_PROCESS_CPUTIME_ID",
    [3] = "CLOCK_THREAD_CPUTIME_ID",
    [4] = "CLOCK_MONOTONIC_RAW",
    [5] = "CLOCK_REALTIME_COARSE",
    [6] = "CLOCK_MONOTONIC_COARSE",
    [7] = "CLOCK_BOOTTIME",
    [8] = "CLOCK_REALTIME_ALARM",
    [9] = "CLOCK_BOOTTIME_ALARM",
    [10] = "CLOCK_SGI_CYCLE",
    [11] = "CLOCK_TAI",
    };
#[no_mangle]
unsafe extern "C" fn test_one_clock_gettime(clock: c_int, name: *const c_char) {
    static void test_one_clock_gettime(int clock, const char *name)
    {
    struct timespec start, vdso, end;
    int vdso_ret, end_ret;
    printf("[RUN]\tTesting clock_gettime for clock %s (%d)...\n", name, clock);
    if (sys_clock_gettime(clock, &start) < 0) {
    if (errno == EINVAL) {
    vdso_ret = VDSO_CALL(vdso_clock_gettime, 2, clock, &vdso);
    if (vdso_ret == -EINVAL) {
    printf("[OK]\tNo such clock.\n");
    } else {
    printf("[FAIL]\tNo such clock, but __vdso_clock_gettime returned %d\n", vdso_ret);
    nerrs++;
    }
    } else {
    printf("[WARN]\t clock_gettime(%d) syscall returned error %d\n", clock, errno);
    }
    return;
    }
    vdso_ret = VDSO_CALL(vdso_clock_gettime, 2, clock, &vdso);
    end_ret = sys_clock_gettime(clock, &end);
    if (vdso_ret != 0 || end_ret != 0) {
    printf("[FAIL]\tvDSO returned %d, syscall errno=%d\n",
    vdso_ret, errno);
    nerrs++;
    return;
    }
    printf("\t%llu.%09ld %llu.%09ld %llu.%09ld\n",
    (unsigned long long)start.tv_sec, start.tv_nsec,
    (unsigned long long)vdso.tv_sec, vdso.tv_nsec,
    (unsigned long long)end.tv_sec, end.tv_nsec);
    if (!ts_leq(&start, &vdso) || !ts_leq(&vdso, &end)) {
    printf("[FAIL]\tTimes are out of sequence\n");
    nerrs++;
    return;
    }
    printf("[OK]\tTest Passed.\n");
    }
#[no_mangle]
unsafe extern "C" fn test_clock_gettime() {
    static void test_clock_gettime(void)
    {
    if (!vdso_clock_gettime) {
    printf("[SKIP]\tNo vDSO, so skipping clock_gettime() tests\n");
    return;
    }
    for (int clock = 0; clock < ARRAY_SIZE(clocknames); clock++)
    test_one_clock_gettime(clock, clocknames[clock]);
// Also test some invalid clock ids
    test_one_clock_gettime(-1, "invalid");
    test_one_clock_gettime(INT_MIN, "invalid");
    test_one_clock_gettime(INT_MAX, "invalid");
    }
#[no_mangle]
unsafe extern "C" fn test_one_clock_gettime64(clock: c_int, name: *const c_char) {
    static void test_one_clock_gettime64(int clock, const char *name)
    {
    struct __kernel_timespec start, vdso, end;
    int vdso_ret, end_ret;
    printf("[RUN]\tTesting clock_gettime64 for clock %s (%d)...\n", name, clock);
    if (sys_clock_gettime64(clock, &start) < 0) {
    if (errno == EINVAL) {
    vdso_ret = VDSO_CALL(vdso_clock_gettime64, 2, clock, &vdso);
    if (vdso_ret == -EINVAL) {
    printf("[OK]\tNo such clock.\n");
    } else {
    printf("[FAIL]\tNo such clock, but __vdso_clock_gettime64 returned %d\n", vdso_ret);
    nerrs++;
    }
    } else {
    printf("[WARN]\t clock_gettime64(%d) syscall returned error %d\n", clock, errno);
    }
    return;
    }
    vdso_ret = VDSO_CALL(vdso_clock_gettime64, 2, clock, &vdso);
    end_ret = sys_clock_gettime64(clock, &end);
    if (vdso_ret != 0 || end_ret != 0) {
    printf("[FAIL]\tvDSO returned %d, syscall errno=%d\n",
    vdso_ret, errno);
    nerrs++;
    return;
    }
    printf("\t%llu.%09lld %llu.%09lld %llu.%09lld\n",
    (unsigned long long)start.tv_sec, start.tv_nsec,
    (unsigned long long)vdso.tv_sec, vdso.tv_nsec,
    (unsigned long long)end.tv_sec, end.tv_nsec);
    if (!ts64_leq(&start, &vdso) || !ts64_leq(&vdso, &end)) {
    printf("[FAIL]\tTimes are out of sequence\n");
    nerrs++;
    return;
    }
    printf("[OK]\tTest Passed.\n");
    }
#[no_mangle]
unsafe extern "C" fn test_clock_gettime64() {
    static void test_clock_gettime64(void)
    {
    if (!vdso_clock_gettime64) {
    printf("[SKIP]\tNo vDSO, so skipping clock_gettime64() tests\n");
    return;
    }
    for (int clock = 0; clock < ARRAY_SIZE(clocknames); clock++)
    test_one_clock_gettime64(clock, clocknames[clock]);
// Also test some invalid clock ids
    test_one_clock_gettime64(-1, "invalid");
    test_one_clock_gettime64(INT_MIN, "invalid");
    test_one_clock_gettime64(INT_MAX, "invalid");
    }
#[no_mangle]
unsafe extern "C" fn test_gettimeofday() {
    static void test_gettimeofday(void)
    {
    struct timeval start, vdso, end;
    struct timezone sys_tz, vdso_tz;
    int vdso_ret, end_ret;
    if (!vdso_gettimeofday)
    return;
    printf("[RUN]\tTesting gettimeofday...\n");
    if (sys_gettimeofday(&start, &sys_tz) < 0) {
    printf("[FAIL]\tsys_gettimeofday failed (%d)\n", errno);
    nerrs++;
    return;
    }
    vdso_ret = VDSO_CALL(vdso_gettimeofday, 2, &vdso, &vdso_tz);
    end_ret = sys_gettimeofday(&end, core::ptr::null_mut());
    if (vdso_ret != 0 || end_ret != 0) {
    printf("[FAIL]\tvDSO returned %d, syscall errno=%d\n",
    vdso_ret, errno);
    nerrs++;
    return;
    }
    printf("\t%llu.%06lld %llu.%06lld %llu.%06lld\n",
    (unsigned long long)start.tv_sec, (long long)start.tv_usec,
    (unsigned long long)vdso.tv_sec, (long long)vdso.tv_usec,
    (unsigned long long)end.tv_sec, (long long)end.tv_usec);
    if (!tv_leq(&start, &vdso) || !tv_leq(&vdso, &end)) {
    printf("[FAIL]\tTimes are out of sequence\n");
    nerrs++;
    }
    if (sys_tz.tz_minuteswest == vdso_tz.tz_minuteswest &&
    sys_tz.tz_dsttime == vdso_tz.tz_dsttime) {
    printf("[OK]\ttimezones match: minuteswest=%d, dsttime=%d\n",
    sys_tz.tz_minuteswest, sys_tz.tz_dsttime);
    } else {
    printf("[FAIL]\ttimezones do not match\n");
    nerrs++;
    }
// And make sure that passing NULL for tz doesn't crash.
    VDSO_CALL(vdso_gettimeofday, 2, &vdso, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn test_time() {
    static void test_time(void)
    {
    __kernel_old_time_t start, end, vdso_ret, vdso_param;
    if (!vdso_time)
    return;
    printf("[RUN]\tTesting time...\n");
    if (sys_time(&start) < 0) {
    if (errno == -ENOSYS) {
    printf("[SKIP]\tNo time() support\n");
    } else {
    printf("[FAIL]\tsys_time failed (%d)\n", errno);
    nerrs++;
    }
    return;
    }
    vdso_ret = VDSO_CALL(vdso_time, 1, &vdso_param);
    end = sys_time(core::ptr::null_mut());
    if (vdso_ret < 0 || end < 0) {
    printf("[FAIL]\tvDSO returned %d, syscall errno=%d\n",
    (int)vdso_ret, errno);
    nerrs++;
    return;
    }
    printf("\t%lld %lld %lld\n",
    (long long)start,
    (long long)vdso_ret,
    (long long)end);
    if (vdso_ret != vdso_param) {
    printf("[FAIL]\tinconsistent return values: %lld %lld\n",
    (long long)vdso_ret, (long long)vdso_param);
    nerrs++;
    return;
    }
    if (!(start <= vdso_ret) || !(vdso_ret <= end)) {
    printf("[FAIL]\tTimes are out of sequence\n");
    nerrs++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    version = versions[VDSO_VERSION];
    name = (const char **)&names[VDSO_NAMES];
    fill_function_pointers();
    test_clock_gettime();
    test_clock_gettime64();
    test_gettimeofday();
    test_time();
//
// Test getcpu() last so that, if something goes wrong setting affinity,
// we still run the other tests.
//
    test_getcpu();
    return nerrs ? 1 : 0;
    }
