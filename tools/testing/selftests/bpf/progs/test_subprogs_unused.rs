//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_subprogs_unused.c
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


    const char LICENSE[] SEC("license") = "GPL";
    __attribute__((unused)) __noinline int unused1(int x)
    {
    return x + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn __attribute__(x: (unused)) __noinline int unused2(int) -> static {
    static __attribute__((unused)) __noinline int unused2(int x)
    {
    return x + 2;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn main_prog(ctx: *mut c_void) -> c_int {
    int main_prog(void *ctx)
    {
    return 0;
    }
