//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/exec/binfmt_loader_payload.c
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
// Payload for the binfmt_misc 'L' (loader substitution) selftest. It is
// executed as the MAIN image - a fully native exec - with the registered
// interpreter substituted for its PT_INTERP, and asserts the native
// identity from the inside. Exits 0 when every surface checks out.
//
// Modes, selected by the orchestrator via the environment:
// - default:                full assertions, path-based ones included
// - BINFMT_TEST_MEMFD=1:    executed from an inaccessible memfd, skip
// the path-based assertions
// - BINFMT_TEST_STATIC=1:   static build; the override was dropped, so
// expect no interpreter at all
//
// Macro flag: #define _GNU_SOURCE

// Start of our own mapped image, courtesy of the linker.
    extern const char __ehdr_start[];
// An image is never this large; used to bracket "within our image".

    static int failed;
#[no_mangle]
unsafe extern "C" fn check(cond: c_int, what: *const c_char) {
    static void check(int cond, const char *what)
    {
    if (cond)
    return;
    fprintf(stderr, "[payload] FAILED: %s (errno %d)\n", what, errno);
    failed = 1;
    }
// Return whether /proc/self/maps names a path starting with @prefix.
#[no_mangle]
unsafe extern "C" fn maps_has_prefix(prefix: *const c_char) -> c_int {
    static int maps_has_prefix(const char *prefix)
    {
    char *line = core::ptr::null_mut();
    let mut len: usize = 0;
    let mut found: c_int = 0;
    FILE *f;
    f = fopen("/proc/self/maps", "r");
    if (!f)
    return -1;
    while (getline(&line, &len, f) > 0) {
    char *path = strchr(line, '/');
    if (path && !strncmp(path, prefix, strlen(prefix))) {
    found = 1;
    break;
    }
    }
    free(line);
    fclose(f);
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    const char *binary = getenv("BINFMT_TEST_BINARY");
    const char *interp = getenv("BINFMT_TEST_INTERP");
    let mut memfd_mode: c_int = getenv("BINFMT_TEST_MEMFD") != core::ptr::null_mut();
    let mut static_mode: c_int = getenv("BINFMT_TEST_STATIC") != core::ptr::null_mut();
    let mut self: c_ulong = (unsigned long)__ehdr_start;
    let mut base: c_ulong = getauxval(AT_BASE);
    let mut phdr: c_ulong = getauxval(AT_PHDR);
    let mut entry: c_ulong = getauxval(AT_ENTRY);
    unsigned long start_code, end_code;
// The argument vector is exactly what the caller built.
    check(argc == 3 && !strcmp(argv[0], PAYLOAD_ARGV0) &&
    !strcmp(argv[1], PAYLOAD_ARG1) && !strcmp(argv[2], PAYLOAD_ARG2),
    "argv was rewritten");
// Native from birth: no execfd, no dispatch marker.
    check(getauxval(AT_EXECFD) == 0, "AT_EXECFD present");
    check(getauxval(AT_FLAGS) == 0, "AT_FLAGS not native");
    if (static_mode) {
// The override was dropped: no interpreter was loaded.
    check(base == 0, "AT_BASE set for a static payload");
    } else {
// A loader is mapped in the interpreter slot, not our image.
    check(base != 0, "AT_BASE missing");
    check(base < self || base >= self + IMAGE_SPAN,
    "AT_BASE inside our own image");
    }
// We occupy the main-image slot.
    check(phdr >= self && phdr < self + IMAGE_SPAN,
    "AT_PHDR outside our image");
    check(entry >= self && entry < self + IMAGE_SPAN,
    "AT_ENTRY outside our image");
// The code statistics markers describe our image, natively placed.
    if (stat_codes(getpid(), &start_code, &end_code) == 0) {
    check(start_code >= self && start_code < end_code &&
    end_code < self + IMAGE_SPAN,
    "stat start_code/end_code not our image");
    check(entry >= start_code && entry < end_code,
    "AT_ENTRY outside [start_code, end_code)");
    } else {
    check(0, "cannot parse /proc/self/stat");
    }
    if (!memfd_mode && binary) {
    const char *execfn = (const char *)getauxval(AT_EXECFN);
    const char *base_name = strrchr(binary, '/');
    base_name = base_name ? base_name + 1 : binary;
// exe link, AT_EXECFN and comm all follow the binary.
    check(exe_is(binary), "/proc/self/exe");
    check(execfn && !strcmp(execfn, binary), "AT_EXECFN");
    check(comm_is(base_name), "comm");
// The running binary is write-denied, natively.
    check(write_denied(binary), "no ETXTBSY on the binary");
    }
    if (interp) {
    let mut found: c_int = maps_has_prefix(interp);
    if (static_mode)
// Nothing was substituted, nothing may be mapped.
    check(found == 0, "loader mapped for a static payload");
    else
// The substituted loader shows under its real path.
    check(found == 1, "loader path not in /proc/self/maps");
    }
    if (failed)
    return 1;
    printf("[payload] native identity checks out\n");
    return 0;
    }
