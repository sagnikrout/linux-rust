//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/arena_mem_usage.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, 1000); /* number of pages */

    __ulong(map_extra, 0x1ull << 32); /* start of mmap() region */

    __ulong(map_extra, 0x1ull << 44); /* start of mmap() region */

    } arena SEC(".maps");
    void __arena *ptr;
    int alloc_cnt;		/* in:  pages to allocate */
    long free_byte_off;	/* in:  byte offset within ptr to start freeing */
    int free_cnt;		/* in:  pages to free */
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn alloc(ctx: *mut c_void) -> c_int {
    int alloc(void *ctx)
    {
    ptr = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), alloc_cnt, NUMA_NO_NODE, 0);
// Success/failure is checked from user space via skel->bss->ptr.
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn free_pages(ctx: *mut c_void) -> c_int {
    int free_pages(void *ctx)
    {
    if (!ptr)
    return 1;
    bpf_arena_free_pages(&arena, (char __arena *)ptr + free_byte_off, free_cnt);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
