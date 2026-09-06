//! Automatically rewritten from C to Rust
//! Source: drivers/misc/lkdtm/cfi.c
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
// This is for all the tests relating directly to Control Flow Integrity.
//

    static int called_count;
// Function taking one argument, without a return value.
#[no_mangle]
unsafe extern "C" fn lkdtm_increment_void(counter: *mut c_int) -> noinline void {
    static noinline void lkdtm_increment_void(int *counter)
    {
    (*counter)++;
    }
// Function taking one argument, returning int.
#[no_mangle]
unsafe extern "C" fn lkdtm_increment_int(counter: *mut c_int) -> noinline int {
    static noinline int lkdtm_increment_int(int *counter)
    {
    (*counter)++;
    return *counter;
    }
// Don't allow the compiler to inline the calls.
#[no_mangle]
unsafe extern "C" fn lkdtm_indirect_call(): *mut *mut void (func)(int) -> noinline void {
    static noinline void lkdtm_indirect_call(void (*func)(int *))
    {
    func(&called_count);
    }
//
// This tries to call an indirect function with a mismatched prototype.
//
#[no_mangle]
unsafe extern "C" fn lkdtm_CFI_FORWARD_PROTO() {
    static void lkdtm_CFI_FORWARD_PROTO(void)
    {
//
// Matches lkdtm_increment_void()'s prototype, but not
// lkdtm_increment_int()'s prototype.
//
    pr_info("Calling matched prototype ...\n");
    lkdtm_indirect_call(lkdtm_increment_void);
    pr_info("Calling mismatched prototype ...\n");
    lkdtm_indirect_call((void *)lkdtm_increment_int);
    pr_err("FAIL: survived mismatched prototype function call!\n");
    pr_expected_config(CONFIG_CFI);
    }
//
// This can stay local to LKDTM, as there should not be a production reason
// to disable PAC && SCS.
//

    (( __typeof__(addr))((uintptr_t)(addr) | PAGE_OFFSET))

// https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-cc.adoc#frame-pointer-convention

pub const FRAME_RA_OFFSET: c_int = 1;

// The ultimate ROP gadget.
    static noinline __no_ret_protection
#[no_mangle]
pub unsafe extern "C" fn set_return_addr_unchecked(expected: *mut c_ulong, addr: *mut c_ulong) {
    void set_return_addr_unchecked(unsigned long *expected, unsigned long *addr)
    {
// Use of volatile is to make sure final write isn't seen as a dead store.
    unsigned long * volatile *ret_addr =
    (unsigned long **)__builtin_frame_address(0) + FRAME_RA_OFFSET;
// Make sure we've found the right place on the stack before writing it.
    if (no_pac_addr(*ret_addr) == expected)
// ret_addr = (addr);
    else
// Check architecture, stack layout, or compiler behavior...
    pr_warn("Eek: return address mismatch! %px != %px\n",
// ret_addr, addr);
    }
    static noinline
#[no_mangle]
pub unsafe extern "C" fn set_return_addr(expected: *mut c_ulong, addr: *mut c_ulong) {
    void set_return_addr(unsigned long *expected, unsigned long *addr)
    {
// Use of volatile is to make sure final write isn't seen as a dead store.
    unsigned long * volatile *ret_addr =
    (unsigned long **)__builtin_frame_address(0) + FRAME_RA_OFFSET;
// Make sure we've found the right place on the stack before writing it.
    if (no_pac_addr(*ret_addr) == expected)
// ret_addr = (addr);
    else
// Check architecture, stack layout, or compiler behavior...
    pr_warn("Eek: return address mismatch! %px != %px\n",
// ret_addr, addr);
    }
    static volatile int force_check;
#[no_mangle]
unsafe extern "C" fn lkdtm_CFI_BACKWARD() {
    static void lkdtm_CFI_BACKWARD(void)
    {
// Use calculated gotos to keep labels addressable.
    void *labels[] = { core::ptr::null_mut(), &&normal, &&redirected, &&check_normal, &&check_redirected };
    pr_info("Attempting unchecked stack return address redirection ...\n");
// Always false
    if (force_check) {
//
// Prepare to call with NULLs to avoid parameters being treated as
// constants in -02.
//
    set_return_addr_unchecked(core::ptr::null_mut(), core::ptr::null_mut());
    set_return_addr(core::ptr::null_mut(), core::ptr::null_mut());
    if (force_check)
    goto *labels[1];
    if (force_check)
    goto *labels[2];
    if (force_check)
    goto *labels[3];
    if (force_check)
    goto *labels[4];
    return;
    }
//
// Use fallthrough switch case to keep basic block ordering between
// set_return_addr*() and the label after it.
//
    switch (force_check) {
    case 0:
    set_return_addr_unchecked(&&normal, &&redirected);
    fallthrough;
    case 1:
    normal:
// Always true
    if (!force_check) {
    pr_err("FAIL: stack return address manipulation failed!\n");
// If we can't redirect "normally", we can't test mitigations.
    return;
    }
    break;
    default:
    redirected:
    pr_info("ok: redirected stack return address.\n");
    break;
    }
    pr_info("Attempting checked stack return address redirection ...\n");
    switch (force_check) {
    case 0:
    set_return_addr(&&check_normal, &&check_redirected);
    fallthrough;
    case 1:
    check_normal:
// Always true
    if (!force_check) {
    pr_info("ok: control flow unchanged.\n");
    return;
    }
    check_redirected:
    pr_err("FAIL: stack return address was redirected!\n");
    break;
    }
    if (IS_ENABLED(CONFIG_ARM64_PTR_AUTH_KERNEL)) {
    pr_expected_config(CONFIG_ARM64_PTR_AUTH_KERNEL);
    return;
    }
    if (IS_ENABLED(CONFIG_SHADOW_CALL_STACK)) {
    pr_expected_config(CONFIG_SHADOW_CALL_STACK);
    return;
    }
    pr_warn("This is probably expected, since this %s was built *without* %s=y nor %s=y\n",
    lkdtm_kernel_info,
    "CONFIG_ARM64_PTR_AUTH_KERNEL", "CONFIG_SHADOW_CALL_STACK");
    }
    static struct crashtype crashtypes[] = {
    CRASHTYPE(CFI_FORWARD_PROTO),
    CRASHTYPE(CFI_BACKWARD),
    };
    struct crashtype_category cfi_crashtypes = {
    .crashtypes = crashtypes,
    .len	    = ARRAY_SIZE(crashtypes),
    };
