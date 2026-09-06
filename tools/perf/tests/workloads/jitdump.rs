//! Automatically rewritten from C to Rust
//! Source: tools/perf/tests/workloads/jitdump.c
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

#[no_mangle]
pub unsafe extern "C" fn gettid() -> pid_t {
    static inline pid_t gettid(void)
    {
    return (pid_t)syscall(SYS_gettid);
    }

pub const CHK_BYTE: c_uint = 0x5a;
#[no_mangle]
pub unsafe extern "C" fn get_timestamp() -> u64 {
    static inline uint64_t get_timestamp(void)
    {

    unsigned int low, high;
    asm volatile("rdtsc" : "=a"(low), "=d"(high));
    return low | ((uint64_t)high) << 32;

    struct timespec ts;
    int ret;
    ret = clock_gettime(CLOCK_MONOTONIC, &ts);
    if (ret)
    return 0;
    return ((uint64_t)ts.tv_sec * 1000000000) + ts.tv_nsec;

    }
    static FILE *open_jitdump(void)
    {
    struct jitheader header = {
    .magic = JITHEADER_MAGIC,
    .version = JITHEADER_VERSION,
    .total_size = sizeof(header),
    .pid = getpid(),
    .timestamp = get_timestamp(),
    .flags =

    JITDUMP_FLAGS_ARCH_TIMESTAMP,

    0,

    };
    char filename[256];
    int fd;
    FILE *f;
    void *m;
    snprintf(filename, sizeof(filename), "jit-%d.dump", getpid());
// Securely open using O_CREAT | O_EXCL to prevent symlink attacks.
    fd = open(filename, O_CREAT | O_EXCL | O_RDWR, 0644);
    if (fd < 0) {
    pr_err("Failed to open jitdump '%s': %s\n", filename, strerror(errno));
    return core::ptr::null_mut();
    }
    f = fdopen(fd, "w+");
    if (!f) {
    pr_err("Failed to associate stream with fd for '%s'\n", filename);
    close(fd);
    unlink(filename);
    return core::ptr::null_mut();
    }
// Create an MMAP event for the jitdump file. That is how perf tool finds it.
    m = mmap(0, getpagesize(), PROT_READ | PROT_EXEC, MAP_PRIVATE, fileno(f), 0);
    if (m == MAP_FAILED) {
    pr_err("mmap failed: %s\n", strerror(errno));
    fclose(f);
    unlink(filename);
    return core::ptr::null_mut();
    }
    munmap(m, getpagesize());
    if (fwrite(&header, sizeof(header), 1, f) != 1) {
    pr_err("Error writing jitdump header\n");
    fclose(f);
    unlink(filename);
    return core::ptr::null_mut();
    }
    return f;
    }
#[no_mangle]
unsafe extern "C" fn write_jitdump(f: *mut FILE, addr: *mut c_void, dat: *const c_void, sz: usize, idx: *mut u64) -> c_int {
    static int write_jitdump(FILE *f, void *addr, const void *dat, size_t sz, uint64_t *idx)
    {
    const char *sym = "jit_workload";
    let mut sym_len: usize = strlen(sym) + 1;
    struct jr_code_load rec = {
    .p.id = JIT_CODE_LOAD,
    .p.total_size = sizeof(rec) + sym_len + sz,
    .p.timestamp = get_timestamp(),
    .pid = getpid(),
    .tid = gettid(),
    .vma = (unsigned long)addr,
    .code_addr = (unsigned long)addr,
    .code_size = sz,
    .code_index = ++*idx,
    };
    if (fwrite(&rec, sizeof(rec), 1, f) != 1 ||
    fwrite(sym, sym_len, 1, f) != 1 ||
    fwrite(dat, sz, 1, f) != 1)
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn close_jitdump(f: *mut FILE) {
    static void close_jitdump(FILE *f)
    {
    fclose(f);
    }
#[no_mangle]
unsafe extern "C" fn jitdump(__maybe_unused: int argc, __maybe_unused: *const *const *const char argv) -> c_int {
    static int jitdump(int argc __maybe_unused, const char **argv __maybe_unused)
    {

// Code to execute: mov CHK_BYTE, %eax ; ret
    uint8_t dat[] = { 0xb8, CHK_BYTE, 0x00, 0x00, 0x00, 0xc3 };

// Code to execute: mov w0, #CHK_BYTE ; ret
    uint8_t dat[] = {
    (CHK_BYTE << 5) & 0xff, (CHK_BYTE >> 3) & 0xff, 0x80, 0x52,
    0xc0, 0x03, 0x5f, 0xd6
    };

// Code to execute: li a0, CHK_BYTE ; ret
    uint8_t dat[] = {
    0x13, 0x05, (CHK_BYTE << 4) & 0xff, (CHK_BYTE >> 4) & 0xff,
    0x67, 0x80, 0x00, 0x00
    };

// Code to execute: li r3, CHK_BYTE ; blr
    uint32_t dat[] = { 0x38600000 | (CHK_BYTE & 0xffff), 0x4e800020 };

// Code to execute: lhi %r2, CHK_BYTE ; br %r14
    uint8_t dat[] = { 0xa7, 0x28, (CHK_BYTE >> 8) & 0xff, CHK_BYTE & 0xff, 0x07, 0xfe };

// Code to execute: mov r0, #CHK_BYTE ; bx lr
    uint8_t dat[] = {
    CHK_BYTE & 0xff, 0x00, 0xa0, 0xe3,
    0x1e, 0xff, 0x2f, 0xe1
    };

// Code to execute: addiu $v0, $zero, CHK_BYTE ; jr $ra ; nop
    uint32_t dat[] = { 0x24020000 | (CHK_BYTE & 0xffff), 0x03e00008, 0x00000000 };

// Code to execute: addi.w $a0, $zero, CHK_BYTE ; jirl $zero, $ra, 0
    uint32_t dat[] = { 0x02800004 | ((CHK_BYTE & 0xfff) << 10), 0x4c000020 };

    uint32_t dat[0];

    void *addr;
    FILE *f;
    let mut idx: u64 = 0;
    let mut ret: c_int = 1;
// Reachable fallback check for unsupported architectures right at start.
    if (sizeof(dat) == 0) {
    pr_err("JITDUMP workload not supported on this architecture\n");
    return 1;
    }
// Get a memory page to store executable code.
    addr = mmap(0, getpagesize(), PROT_READ | PROT_WRITE | PROT_EXEC,
    MAP_ANONYMOUS | MAP_PRIVATE, -1, 0);
    if (addr == MAP_FAILED) {
    pr_err("Failed to map 1 -rwx page\n");
    return 1;
    }
    f = open_jitdump();
    if (!f) {
    pr_err("Failed to open JITDUMP\n");
    munmap(addr, getpagesize());
    return 1;
    }
// Copy executable code to executable memory page.
    memcpy(addr, dat, sizeof(dat));
// Synchronize the Instruction and Data caches.
    __builtin___clear_cache(addr, (char *)addr + sizeof(dat));
// Record it in the jitdump file
    if (write_jitdump(f, addr, dat, sizeof(dat), &idx) == 0) {
    int (*fn)(void) = addr;
// Call the function.
    ret = fn() - CHK_BYTE;
    }
    close_jitdump(f);
    munmap(addr, getpagesize());
    return ret;
    }
    DEFINE_WORKLOAD(jitdump);
