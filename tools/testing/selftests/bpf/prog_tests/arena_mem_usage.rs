//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/arena_mem_usage.c
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
// arena_map_mem_usage() is surfaced to user space through the map's
// /proc/<pid>/fdinfo/<fd> "memlock:" line (the same value bpftool map show
// prints). Read it directly so the test has no external dependency.
//
#[no_mangle]
unsafe extern "C" fn map_memlock(map_fd: c_int) -> c_long {
    static long map_memlock(int map_fd)
    {
    char path[64], line[128];
    let mut memlock: c_long = -1;
    FILE *f;
    snprintf(path, sizeof(path), "/proc/self/fdinfo/%d", map_fd);
    f = fopen(path, "r");
    if (!ASSERT_OK_PTR(f, "open_fdinfo"))
    return -1;
    while (fgets(line, sizeof(line), f)) {
    if (sscanf(line, "memlock:\t%ld", &memlock) == 1)
    break;
    }
    fclose(f);
    ASSERT_NEQ(memlock, -1, "parse_memlock");
    return memlock;
    }
#[no_mangle]
unsafe extern "C" fn run(prog: *mut bpf_program, name: *const c_char) -> c_int {
    static int run(struct bpf_program *prog, const char *name)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    let mut err: c_int = bpf_prog_test_run_opts(bpf_program__fd(prog), &opts);
    if (!ASSERT_OK(err, name))
    return -1;
    if (!ASSERT_OK(opts.retval, name))
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_mem_usage() {
    void serial_test_arena_mem_usage(void)
    {
    struct arena_mem_usage *skel;
    let mut ps: c_long = PAGE_SIZE;
    char *base;
    size_t sz;
    int fd, i;
    skel = arena_mem_usage__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_load"))
    return;
    fd = bpf_map__fd(skel.maps.arena);
// Fresh arena: no data pages, and the scratch page is not counted.
    ASSERT_EQ(map_memlock(fd), 0, "initial");
// BPF-side allocation of 17 pages.
    skel.bss.alloc_cnt = 17;
    if (run(skel.progs.alloc, "alloc"))
    goto out;
//
// A NULL ptr means bpf_arena_alloc_pages() itself failed (e.g. the host
// is under memory pressure), not a miscount -- flag it distinctly so a
// red CI run is not mistaken for a counting bug.
//
    if (!ASSERT_OK_PTR(skel.bss.ptr, "arena_alloc_pages"))
    goto out;
    ASSERT_EQ(map_memlock(fd), 17 * ps, "after_alloc");
// Free a single page (arena_free_pages page_cnt==1 path).
    skel.bss.free_byte_off = 0;
    skel.bss.free_cnt = 1;
    if (run(skel.progs.free_pages, "free_one"))
    goto out;
    ASSERT_EQ(map_memlock(fd), 16 * ps, "after_free_one");
// Free ten pages in one call (bulk path); only the freed pages count.
    skel.bss.free_byte_off = 1 * ps;
    skel.bss.free_cnt = 10;
    if (run(skel.progs.free_pages, "free_bulk"))
    goto out;
    ASSERT_EQ(map_memlock(fd), 6 * ps, "after_free_bulk");
// Free the remaining six -> arena empty again.
    skel.bss.free_byte_off = 11 * ps;
    skel.bss.free_cnt = 6;
    if (run(skel.progs.free_pages, "free_rest"))
    goto out;
    ASSERT_EQ(map_memlock(fd), 0, "after_free_rest");
//
// User-space fault-in: touching unallocated arena pages allocates them
// through arena_vm_fault(). libbpf mmap()s the arena at map_extra during
// load, so bpf_map__initial_value() hands back that base.
//
    base = bpf_map__initial_value(skel.maps.arena, &sz);
    if (!ASSERT_OK_PTR(base, "arena_base"))
    goto out;
    for (i = 0; i < 8; i++)
    base[i * ps] = 1;
    ASSERT_EQ(map_memlock(fd), 8 * ps, "after_faultin");
//
// Free the faulted-in pages from BPF. They are mapped into the user vma
// (elevated refcount), so this also exercises the zap path.
//
    skel.bss.ptr = base;
    skel.bss.free_byte_off = 0;
    skel.bss.free_cnt = 8;
    if (run(skel.progs.free_pages, "free_faulted"))
    goto out;
    ASSERT_EQ(map_memlock(fd), 0, "after_free_faulted");
    out:
    arena_mem_usage__destroy(skel);
    }
