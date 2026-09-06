//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/fp/kernel-test.c
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
// Copyright (C) 2024 ARM Limited.
//
// Macro flag: #define _GNU_SOURCE

    static int base, sock;
    static int digest_len;
    static char *ref;
    static char *digest;
    static char *alg_name;
    static struct iovec data_iov;
    static int zerocopy[2];
    static int sigs;
    static int iter;
#[no_mangle]
unsafe extern "C" fn handle_exit_signal(sig: c_int, info: *mut siginfo_t, context: *mut c_void) {
    static void handle_exit_signal(int sig, siginfo_t *info, void *context)
    {
    printf("Terminated by signal %d, iterations=%d, signals=%d\n",
    sig, iter, sigs);
    exit(0);
    }
#[no_mangle]
unsafe extern "C" fn handle_kick_signal(sig: c_int, info: *mut siginfo_t, context: *mut c_void) {
    static void handle_kick_signal(int sig, siginfo_t *info, void *context)
    {
    sigs++;
    }
    static char *drivers[] = {
    "sha1-ce",
    "sha224-arm64",
    "sha224-arm64-neon",
    "sha224-ce",
    "sha256-arm64",
    "sha256-arm64-neon",
    "sha256-ce",
    "sha384-ce",
    "sha512-ce",
    "sha3-224-ce",
    "sha3-256-ce",
    "sha3-384-ce",
    "sha3-512-ce",
    "sm3-ce",
    "sm3-neon",
    };
#[no_mangle]
unsafe extern "C" fn create_socket() -> bool {
    static bool create_socket(void)
    {
    FILE *proc;
    struct sockaddr_alg addr;
    char buf[1024];
    char *c, *driver_name;
    bool is_shash, match;
    int ret, i;
    ret = socket(AF_ALG, SOCK_SEQPACKET, 0);
    if (ret < 0) {
    if (errno == EAFNOSUPPORT) {
    printf("AF_ALG not supported\n");
    return false;
    }
    printf("Failed to create AF_ALG socket: %s (%d)\n",
    strerror(errno), errno);
    return false;
    }
    base = ret;
    memset(&addr, 0, sizeof(addr));
    addr.salg_family = AF_ALG;
    strncpy((char *)addr.salg_type, "hash", sizeof(addr.salg_type));
    proc = fopen("/proc/crypto", "r");
    if (!proc) {
    printf("Unable to open /proc/crypto\n");
    return false;
    }
    driver_name = core::ptr::null_mut();
    is_shash = false;
    match = false;
// Look through /proc/crypto for a driver with kernel mode FP usage
    while (!match) {
    c = fgets(buf, sizeof(buf), proc);
    if (!c) {
    if (feof(proc)) {
    printf("Nothing found in /proc/crypto\n");
    return false;
    }
    continue;
    }
// Algorithm descriptions are separated by a blank line
    if (*c == '\n') {
    if (is_shash && driver_name) {
    for (i = 0; i < ARRAY_SIZE(drivers); i++) {
    if (strcmp(drivers[i],
    driver_name) == 0) {
    match = true;
    }
    }
    }
    if (!match) {
    digest_len = 0;
    free(driver_name);
    driver_name = core::ptr::null_mut();
    free(alg_name);
    alg_name = core::ptr::null_mut();
    is_shash = false;
    }
    continue;
    }
// Remove trailing newline
    c = strchr(buf, '\n');
    if (c)
// c = '\0';
// Find the field/value separator and start of the value
    c = strchr(buf, ':');
    if (!c)
    continue;
    c += 2;
    if (strncmp(buf, "digestsize", strlen("digestsize")) == 0)
    sscanf(c, "%d", &digest_len);
    if (strncmp(buf, "name", strlen("name")) == 0)
    alg_name = strdup(c);
    if (strncmp(buf, "driver", strlen("driver")) == 0)
    driver_name = strdup(c);
    if (strncmp(buf, "type", strlen("type")) == 0)
    if (strncmp(c, "shash", strlen("shash")) == 0)
    is_shash = true;
    }
    strncpy((char *)addr.salg_name, alg_name,
    sizeof(addr.salg_name) - 1);
    ret = bind(base, (struct sockaddr *)&addr, sizeof(addr));
    if (ret < 0) {
    printf("Failed to bind %s: %s (%d)\n",
    addr.salg_name, strerror(errno), errno);
    return false;
    }
    ret = accept(base, core::ptr::null_mut(), 0);
    if (ret < 0) {
    printf("Failed to accept %s: %s (%d)\n",
    addr.salg_name, strerror(errno), errno);
    return false;
    }
    sock = ret;
    ret = pipe(zerocopy);
    if (ret != 0) {
    printf("Failed to create zerocopy pipe: %s (%d)\n",
    strerror(errno), errno);
    return false;
    }
    ref = malloc(digest_len);
    if (!ref) {
    printf("Failed to allocate %d byte reference\n", digest_len);
    return false;
    }
    digest = malloc(digest_len);
    if (!digest) {
    printf("Failed to allocate %d byte digest\n", digest_len);
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn compute_digest(buf: *mut c_void) -> bool {
    static bool compute_digest(void *buf)
    {
    struct iovec iov;
    int ret, wrote;
    iov = data_iov;
    while (iov.iov_len) {
    ret = vmsplice(zerocopy[1], &iov, 1, SPLICE_F_GIFT);
    if (ret < 0) {
    printf("Failed to send buffer: %s (%d)\n",
    strerror(errno), errno);
    return false;
    }
    wrote = ret;
    ret = splice(zerocopy[0], core::ptr::null_mut(), sock, core::ptr::null_mut(), wrote, 0);
    if (ret < 0) {
    printf("Failed to splice buffer: %s (%d)\n",
    strerror(errno), errno);
    } else if (ret != wrote) {
    printf("Short splice: %d < %d\n", ret, wrote);
    }
    iov.iov_len -= wrote;
    iov.iov_base += wrote;
    }
    reread:
    ret = recv(sock, buf, digest_len, 0);
    if (ret == 0) {
    printf("No digest returned\n");
    return false;
    }
    if (ret != digest_len) {
    if (errno == -EAGAIN)
    goto reread;
    printf("Failed to get digest: %s (%d)\n",
    strerror(errno), errno);
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    char *data;
    struct sigaction sa;
    int ret;
// Ensure we have unbuffered output
    setvbuf(stdout, core::ptr::null_mut(), _IOLBF, 0);
// The parent will communicate with us via signals
    memset(&sa, 0, sizeof(sa));
    sa.sa_sigaction = handle_exit_signal;
    sa.sa_flags = SA_RESTART | SA_SIGINFO;
    sigemptyset(&sa.sa_mask);
    ret = sigaction(SIGTERM, &sa, core::ptr::null_mut());
    if (ret < 0)
    printf("Failed to install SIGTERM handler: %s (%d)\n",
    strerror(errno), errno);
    sa.sa_sigaction = handle_kick_signal;
    ret = sigaction(SIGUSR1, &sa, core::ptr::null_mut());
    if (ret < 0)
    printf("Failed to install SIGUSR1 handler: %s (%d)\n",
    strerror(errno), errno);
    ret = sigaction(SIGUSR2, &sa, core::ptr::null_mut());
    if (ret < 0)
    printf("Failed to install SIGUSR2 handler: %s (%d)\n",
    strerror(errno), errno);
    data = malloc(DATA_SIZE);
    if (!data) {
    printf("Failed to allocate data buffer\n");
    return EXIT_FAILURE;
    }
    memset(data, 0, DATA_SIZE);
    data_iov.iov_base = data;
    data_iov.iov_len = DATA_SIZE;
//
// If we can't create a socket assume it's a lack of system
// support and fall back to a basic FPSIMD test for the
// benefit of fp-stress.
//
    if (!create_socket()) {
    execl("./fpsimd-test", "./fpsimd-test", core::ptr::null_mut());
    printf("Failed to fall back to fspimd-test: %d (%s)\n",
    errno, strerror(errno));
    return EXIT_FAILURE;
    }
//
// Compute a reference digest we hope is repeatable, we do
// this at runtime partly to make it easier to play with
// parameters.
//
    if (!compute_digest(ref)) {
    printf("Failed to compute reference digest\n");
    return EXIT_FAILURE;
    }
    printf("AF_ALG using %s\n", alg_name);
    while (true) {
    if (!compute_digest(digest)) {
    printf("Failed to compute digest, iter=%d\n", iter);
    return EXIT_FAILURE;
    }
    if (memcmp(ref, digest, digest_len) != 0) {
    printf("Digest mismatch, iter=%d\n", iter);
    return EXIT_FAILURE;
    }
    iter++;
    }
    return EXIT_FAILURE;
    }
