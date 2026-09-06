//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/utils.c
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
// Copyright 2013-2015, Michael Ellerman, IBM Corp.
//

    static char auxv[4096];
#[no_mangle]
pub unsafe extern "C" fn read_file(path: *const c_char, buf: *mut c_char, count: usize, len: *mut usize) -> c_int {
    int read_file(const char *path, char *buf, size_t count, size_t *len)
    {
    ssize_t rc;
    int fd;
    int err;
    char eof;
    fd = open(path, O_RDONLY);
    if (fd < 0)
    return -errno;
    rc = read(fd, buf, count);
    if (rc < 0) {
    err = -errno;
    goto out;
    }
    if (len)
// len = rc;
// Overflow if there are still more bytes after filling the buffer
    if (rc == count) {
    rc = read(fd, &eof, 1);
    if (rc != 0) {
    err = -EOVERFLOW;
    goto out;
    }
    }
    err = 0;
    out:
    close(fd);
    errno = -err;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn read_file_alloc(path: *const c_char, buf: *mut c_char, len: *mut usize) -> c_int {
    int read_file_alloc(const char *path, char **buf, size_t *len)
    {
    let mut read_offset: usize = 0;
    let mut buffer_len: usize = 0;
    char *buffer = core::ptr::null_mut();
    int err;
    int fd;
    fd = open(path, O_RDONLY);
    if (fd < 0)
    return -errno;
//
// We don't use stat & preallocate st_size because some non-files
// report 0 file size. Instead just dynamically grow the buffer
// as needed.
//
    while (1) {
    ssize_t rc;
    if (read_offset >= buffer_len / 2) {
    char *next_buffer;
    buffer_len = buffer_len ? buffer_len * 2 : 4096;
    next_buffer = realloc(buffer, buffer_len);
    if (!next_buffer) {
    err = -errno;
    goto out;
    }
    buffer = next_buffer;
    }
    rc = read(fd, buffer + read_offset, buffer_len - read_offset);
    if (rc < 0) {
    err = -errno;
    goto out;
    }
    if (rc == 0)
    break;
    read_offset += rc;
    }
// buf = buffer;
    if (len)
// len = read_offset;
    err = 0;
    out:
    close(fd);
    if (err)
    free(buffer);
    errno = -err;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn write_file(path: *const c_char, buf: *const c_char, count: usize) -> c_int {
    int write_file(const char *path, const char *buf, size_t count)
    {
    int fd;
    int err;
    ssize_t rc;
    fd = open(path, O_WRONLY | O_CREAT | O_TRUNC, 0644);
    if (fd < 0)
    return -errno;
    rc = write(fd, buf, count);
    if (rc < 0) {
    err = -errno;
    goto out;
    }
    if (rc != count) {
    err = -EOVERFLOW;
    goto out;
    }
    err = 0;
    out:
    close(fd);
    errno = -err;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn read_auxv(buf: *mut c_char, buf_size: isize) -> c_int {
    int read_auxv(char *buf, ssize_t buf_size)
    {
    int err;
    err = read_file("/proc/self/auxv", buf, buf_size, core::ptr::null_mut());
    if (err) {
    perror("Error reading /proc/self/auxv");
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn read_debugfs_file(subpath: *const c_char, buf: *mut c_char, count: usize) -> c_int {
    int read_debugfs_file(const char *subpath, char *buf, size_t count)
    {
    char path[PATH_MAX] = "/sys/kernel/debug/";
    strncat(path, subpath, sizeof(path) - strlen(path) - 1);
    return read_file(path, buf, count, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn write_debugfs_file(subpath: *const c_char, buf: *const c_char, count: usize) -> c_int {
    int write_debugfs_file(const char *subpath, const char *buf, size_t count)
    {
    char path[PATH_MAX] = "/sys/kernel/debug/";
    strncat(path, subpath, sizeof(path) - strlen(path) - 1);
    return write_file(path, buf, count);
    }
#[no_mangle]
unsafe extern "C" fn validate_int_parse(buffer: *const c_char, count: usize, end: *mut c_char) -> c_int {
    static int validate_int_parse(const char *buffer, size_t count, char *end)
    {
    let mut err: c_int = 0;
// Require at least one digit
    if (end == buffer) {
    err = -EINVAL;
    goto out;
    }
// Require all remaining characters be whitespace-ish
    for (; end < buffer + count; end++) {
    if (*end == '\0')
    break;
    if (*end != ' ' && *end != '\n') {
    err = -EINVAL;
    goto out;
    }
    }
    out:
    errno = -err;
    return err;
    }
    static int parse_bounded_int(const char *buffer, size_t count, intmax_t *result,
    int base, intmax_t min, intmax_t max)
    {
    int err;
    char *end;
    errno = 0;
// result = strtoimax(buffer, &end, base);
    if (errno)
    return -errno;
    err = validate_int_parse(buffer, count, end);
    if (err)
    goto out;
    if (*result < min || *result > max)
    err = -EOVERFLOW;
    out:
    errno = -err;
    return err;
    }
    static int parse_bounded_uint(const char *buffer, size_t count, uintmax_t *result,
    int base, uintmax_t max)
    {
    let mut err: c_int = 0;
    char *end;
    errno = 0;
// result = strtoumax(buffer, &end, base);
    if (errno)
    return -errno;
    err = validate_int_parse(buffer, count, end);
    if (err)
    goto out;
    if (*result > max)
    err = -EOVERFLOW;
    out:
    errno = -err;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_intmax(buffer: *const c_char, count: usize, result: *mut intmax_t, base: c_int) -> c_int {
    int parse_intmax(const char *buffer, size_t count, intmax_t *result, int base)
    {
    return parse_bounded_int(buffer, count, result, base, INTMAX_MIN, INTMAX_MAX);
    }
#[no_mangle]
pub unsafe extern "C" fn parse_uintmax(buffer: *const c_char, count: usize, result: *mut uintmax_t, base: c_int) -> c_int {
    int parse_uintmax(const char *buffer, size_t count, uintmax_t *result, int base)
    {
    return parse_bounded_uint(buffer, count, result, base, UINTMAX_MAX);
    }
#[no_mangle]
pub unsafe extern "C" fn parse_int(buffer: *const c_char, count: usize, result: *mut c_int, base: c_int) -> c_int {
    int parse_int(const char *buffer, size_t count, int *result, int base)
    {
    intmax_t parsed;
    let mut err: c_int = parse_bounded_int(buffer, count, &parsed, base, INT_MIN, INT_MAX);
// result = parsed;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_uint(buffer: *const c_char, count: usize, result: *mut c_uint, base: c_int) -> c_int {
    int parse_uint(const char *buffer, size_t count, unsigned int *result, int base)
    {
    uintmax_t parsed;
    let mut err: c_int = parse_bounded_uint(buffer, count, &parsed, base, UINT_MAX);
// result = parsed;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_long(buffer: *const c_char, count: usize, result: *mut c_long, base: c_int) -> c_int {
    int parse_long(const char *buffer, size_t count, long *result, int base)
    {
    intmax_t parsed;
    let mut err: c_int = parse_bounded_int(buffer, count, &parsed, base, LONG_MIN, LONG_MAX);
// result = parsed;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn parse_ulong(buffer: *const c_char, count: usize, result: *mut c_ulong, base: c_int) -> c_int {
    int parse_ulong(const char *buffer, size_t count, unsigned long *result, int base)
    {
    uintmax_t parsed;
    let mut err: c_int = parse_bounded_uint(buffer, count, &parsed, base, ULONG_MAX);
// result = parsed;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn read_long(path: *const c_char, result: *mut c_long, base: c_int) -> c_int {
    int read_long(const char *path, long *result, int base)
    {
    int err;
    char buffer[32] = {0};
    err = read_file(path, buffer, sizeof(buffer) - 1, core::ptr::null_mut());
    if (err)
    return err;
    return parse_long(buffer, sizeof(buffer), result, base);
    }
#[no_mangle]
pub unsafe extern "C" fn read_ulong(path: *const c_char, result: *mut c_ulong, base: c_int) -> c_int {
    int read_ulong(const char *path, unsigned long *result, int base)
    {
    int err;
    char buffer[32] = {0};
    err = read_file(path, buffer, sizeof(buffer) - 1, core::ptr::null_mut());
    if (err)
    return err;
    return parse_ulong(buffer, sizeof(buffer), result, base);
    }
#[no_mangle]
pub unsafe extern "C" fn write_long(path: *const c_char, result: c_long, base: c_int) -> c_int {
    int write_long(const char *path, long result, int base)
    {
    int err;
    int len;
    char buffer[32];
// Decimal only for now: no format specifier for signed hex values
    if (base != 10) {
    err = -EINVAL;
    goto out;
    }
    len = snprintf(buffer, sizeof(buffer), "%ld", result);
    if (len < 0 || len >= sizeof(buffer)) {
    err = -EOVERFLOW;
    goto out;
    }
    err = write_file(path, buffer, len);
    out:
    errno = -err;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn write_ulong(path: *const c_char, result: c_ulong, base: c_int) -> c_int {
    int write_ulong(const char *path, unsigned long result, int base)
    {
    int err;
    int len;
    char buffer[32];
    char *fmt;
    switch (base) {
    case 10:
    fmt = "%lu";
    break;
    case 16:
    fmt = "%lx";
    break;
    default:
    err = -EINVAL;
    goto out;
    }
    len = snprintf(buffer, sizeof(buffer), fmt, result);
    if (len < 0 || len >= sizeof(buffer)) {
    err = -errno;
    goto out;
    }
    err = write_file(path, buffer, len);
    out:
    errno = -err;
    return err;
    }
    void *find_auxv_entry(int type, char *auxv)
    {
    ElfW(auxv_t) *p;
    p = (ElfW(auxv_t) *)auxv;
    while (p.a_type != AT_NULL) {
    if (p.a_type == type)
    return p;
    p++;
    }
    return core::ptr::null_mut();
    }
    void *get_auxv_entry(int type)
    {
    ElfW(auxv_t) *p;
    if (read_auxv(auxv, sizeof(auxv)))
    return core::ptr::null_mut();
    p = find_auxv_entry(type, auxv);
    if (p)
    return (void *)p.a_un.a_val;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn pick_online_cpu() -> c_int {
    int pick_online_cpu(void)
    {
    int ncpus, cpu = -1;
    cpu_set_t *mask;
    size_t size;
    ncpus = get_nprocs_conf();
    size = CPU_ALLOC_SIZE(ncpus);
    mask = CPU_ALLOC(ncpus);
    if (!mask) {
    perror("malloc");
    return -1;
    }
    CPU_ZERO_S(size, mask);
    if (sched_getaffinity(0, size, mask)) {
    perror("sched_getaffinity");
    goto done;
    }
// We prefer a primary thread, but skip 0
    for (cpu = 8; cpu < ncpus; cpu += 8)
    if (CPU_ISSET_S(cpu, size, mask))
    goto done;
// Search for anything, but in reverse
    for (cpu = ncpus - 1; cpu >= 0; cpu--)
    if (CPU_ISSET_S(cpu, size, mask))
    goto done;
    printf("No cpus in affinity mask?!\n");
    done:
    CPU_FREE(mask);
    return cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn bind_to_cpu(cpu: c_int) -> c_int {
    int bind_to_cpu(int cpu)
    {
    cpu_set_t mask;
    int err;
    if (cpu == BIND_CPU_ANY) {
    cpu = pick_online_cpu();
    if (cpu < 0)
    return cpu;
    }
    printf("Binding to cpu %d\n", cpu);
    CPU_ZERO(&mask);
    CPU_SET(cpu, &mask);
    err = sched_setaffinity(0, sizeof(mask), &mask);
    if (err)
    return err;
    return cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn is_ppc64le() -> bool {
    bool is_ppc64le(void)
    {
    struct utsname uts;
    int rc;
    errno = 0;
    rc = uname(&uts);
    if (rc) {
    perror("uname");
    return false;
    }
    return strcmp(uts.machine, "ppc64le") == 0;
    }
#[no_mangle]
pub unsafe extern "C" fn read_sysfs_file(fpath: *mut c_char, result: *mut c_char, result_size: usize) -> c_int {
    int read_sysfs_file(char *fpath, char *result, size_t result_size)
    {
    char path[PATH_MAX] = "/sys/";
    strncat(path, fpath, PATH_MAX - strlen(path) - 1);
    return read_file(path, result, result_size, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn read_debugfs_int(debugfs_file: *const c_char, result: *mut c_int) -> c_int {
    int read_debugfs_int(const char *debugfs_file, int *result)
    {
    int err;
    char value[16] = {0};
    err = read_debugfs_file(debugfs_file, value, sizeof(value) - 1);
    if (err)
    return err;
    return parse_int(value, sizeof(value), result, 10);
    }
#[no_mangle]
pub unsafe extern "C" fn write_debugfs_int(debugfs_file: *const c_char, result: c_int) -> c_int {
    int write_debugfs_int(const char *debugfs_file, int result)
    {
    char value[16];
    snprintf(value, 16, "%d", result);
    return write_debugfs_file(debugfs_file, value, strlen(value));
    }
    static long perf_event_open(struct perf_event_attr *hw_event, pid_t pid,
    int cpu, int group_fd, unsigned long flags)
    {
    return syscall(__NR_perf_event_open, hw_event, pid, cpu,
    group_fd, flags);
    }
    static void perf_event_attr_init(struct perf_event_attr *event_attr,
    unsigned int type,
    unsigned long config)
    {
    memset(event_attr, 0, sizeof(*event_attr));
    event_attr.type = type;
    event_attr.size = sizeof(struct perf_event_attr);
    event_attr.config = config;
    event_attr.read_format = PERF_FORMAT_GROUP;
    event_attr.disabled = 1;
    event_attr.exclude_kernel = 1;
    event_attr.exclude_hv = 1;
    event_attr.exclude_guest = 1;
    }
    int perf_event_open_counter(unsigned int type,
    unsigned long config, int group_fd)
    {
    int fd;
    struct perf_event_attr event_attr;
    perf_event_attr_init(&event_attr, type, config);
    fd = perf_event_open(&event_attr, 0, -1, group_fd, 0);
    if (fd < 0)
    perror("perf_event_open() failed");
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_event_enable(fd: c_int) -> c_int {
    int perf_event_enable(int fd)
    {
    if (ioctl(fd, PERF_EVENT_IOC_ENABLE, PERF_IOC_FLAG_GROUP) == -1) {
    perror("error while enabling perf events");
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_event_disable(fd: c_int) -> c_int {
    int perf_event_disable(int fd)
    {
    if (ioctl(fd, PERF_EVENT_IOC_DISABLE, PERF_IOC_FLAG_GROUP) == -1) {
    perror("error disabling perf events");
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_event_reset(fd: c_int) -> c_int {
    int perf_event_reset(int fd)
    {
    if (ioctl(fd, PERF_EVENT_IOC_RESET, PERF_IOC_FLAG_GROUP) == -1) {
    perror("error resetting perf events");
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn using_hash_mmu(using_hash: *mut bool) -> c_int {
    int using_hash_mmu(bool *using_hash)
    {
    char line[128];
    FILE *f;
    int rc;
    f = fopen("/proc/cpuinfo", "r");
    FAIL_IF(!f);
    rc = 0;
    while (fgets(line, sizeof(line), f) != core::ptr::null_mut()) {
    if (!strcmp(line, "MMU		: Hash\n") ||
    !strcmp(line, "platform	: Cell\n") ||
    !strcmp(line, "platform	: PowerMac\n")) {
// using_hash = true;
    goto out;
    }
    if (strcmp(line, "MMU		: Radix\n") == 0) {
// using_hash = false;
    goto out;
    }
    }
    rc = -1;
    out:
    fclose(f);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn push_signal_handler(sig: c_int, (*fn)(int: *mut c_void, : *mut siginfo_t, ): *mut c_void) -> sigaction {
    struct sigaction push_signal_handler(int sig, void (*fn)(int, siginfo_t *, void *))
    {
    struct sigaction sa;
    struct sigaction old_handler;
    sa.sa_sigaction = fn;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_SIGINFO;
    FAIL_IF_EXIT_MSG(sigaction(sig, &sa, &old_handler),
    "failed to push signal handler");
    return old_handler;
    }
#[no_mangle]
pub unsafe extern "C" fn pop_signal_handler(sig: c_int, old_handler: sigaction) -> sigaction {
    struct sigaction pop_signal_handler(int sig, struct sigaction old_handler)
    {
    struct sigaction popped;
    FAIL_IF_EXIT_MSG(sigaction(sig, &old_handler, &popped),
    "failed to pop signal handler");
    return popped;
    }
