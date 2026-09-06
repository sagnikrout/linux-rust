//! Automatically rewritten from C to Rust
//! Source: arch/x86/coco/tdx/tdx-shared.c
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


    static unsigned long try_accept_one(phys_addr_t start, unsigned long len,
    enum pg_level pg_level)
    {
    let mut accept_size: c_ulong = page_level_size(pg_level);
    let mut args: tdx_module_args = {};
    u8 page_size;
    if (!IS_ALIGNED(start, accept_size))
    return 0;
    if (len < accept_size)
    return 0;
//
// Pass the page physical address to the TDX module to accept the
// pending, private page.
//
// Bits 2:0 of RCX encode page size: 0 - 4K, 1 - 2M, 2 - 1G.
//
    switch (pg_level) {
    case PG_LEVEL_4K:
    page_size = TDX_PS_4K;
    break;
    case PG_LEVEL_2M:
    page_size = TDX_PS_2M;
    break;
    case PG_LEVEL_1G:
    page_size = TDX_PS_1G;
    break;
    default:
    return 0;
    }
    args.rcx = start | page_size;
    if (__tdcall(TDG_MEM_PAGE_ACCEPT, &args))
    return 0;
    return accept_size;
    }
#[no_mangle]
pub unsafe extern "C" fn tdx_accept_memory(start: phys_addr_t, end: phys_addr_t) -> bool {
    bool tdx_accept_memory(phys_addr_t start, phys_addr_t end)
    {
//
// For shared->private conversion, accept the page using
// TDG_MEM_PAGE_ACCEPT TDX module call.
//
    while (start < end) {
    let mut len: c_ulong = end - start;
    unsigned long accept_size;
//
// Try larger accepts first. It gives chance to VMM to keep
// 1G/2M Secure EPT entries where possible and speeds up
// process by cutting number of hypercalls (if successful).
//
    accept_size = try_accept_one(start, len, PG_LEVEL_1G);
    if (!accept_size)
    accept_size = try_accept_one(start, len, PG_LEVEL_2M);
    if (!accept_size)
    accept_size = try_accept_one(start, len, PG_LEVEL_4K);
    if (!accept_size)
    return false;
    start += accept_size;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn __tdx_hypercall(args: *mut tdx_module_args) -> noinstr u64 {
    noinstr u64 __tdx_hypercall(struct tdx_module_args *args)
    {
//
// For TDVMCALL explicitly set RCX to the bitmap of shared registers.
// The caller isn't expected to set @args->rcx anyway.
//
    args.rcx = TDVMCALL_EXPOSE_REGS_MASK;
//
// Failure of __tdcall_saved_ret() indicates a failure of the TDVMCALL
// mechanism itself and that something has gone horribly wrong with
// the TDX module.  __tdx_hypercall_failed() never returns.
//
    if (__tdcall_saved_ret(TDG_VP_VMCALL, args))
    __tdx_hypercall_failed();
// TDVMCALL leaf return code is in R10
    return args.r10;
    }
