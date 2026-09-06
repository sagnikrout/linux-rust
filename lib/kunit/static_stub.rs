//! Automatically rewritten from C to Rust
//! Source: lib/kunit/static_stub.c
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
// KUnit function redirection (static stubbing) API.
//
// Copyright (C) 2022, Google LLC.
// Author: David Gow <davidgow@google.com>
//

// Context for a static stub. This is stored in the resource data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_static_stub_ctx {
    pub real_fn_addr: *mut c_void,
    pub replacement_addr: *mut c_void,
}

#[no_mangle]
unsafe extern "C" fn __kunit_static_stub_resource_free(res: *mut kunit_resource) {
    static void __kunit_static_stub_resource_free(struct kunit_resource *res)
    {
    kfree(res.data);
    }
// Matching function for kunit_find_resource(). match_data is real_fn_addr.
    static bool __kunit_static_stub_resource_match(struct kunit *test,
    struct kunit_resource *res,
    void *match_real_fn_addr)
    {
// This pointer is only valid if res is a static stub resource.
    struct kunit_static_stub_ctx *ctx = res.data;
// Make sure the resource is a static stub resource.
    if (res.free != &__kunit_static_stub_resource_free)
    return false;
    return ctx.real_fn_addr == match_real_fn_addr;
    }
// Hook to return the address of the replacement function.
    void *__kunit_get_static_stub_address_impl(struct kunit *test, void *real_fn_addr)
    {
    struct kunit_resource *res;
    struct kunit_static_stub_ctx *ctx;
    void *replacement_addr;
    res = kunit_find_resource(test,
    __kunit_static_stub_resource_match,
    real_fn_addr);
    if (!res)
    return core::ptr::null_mut();
    ctx = res.data;
    replacement_addr = ctx.replacement_addr;
    kunit_put_resource(res);
    return replacement_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn kunit_deactivate_static_stub(test: *mut kunit, real_fn_addr: *mut c_void) {
    void kunit_deactivate_static_stub(struct kunit *test, void *real_fn_addr)
    {
    struct kunit_resource *res;
    KUNIT_ASSERT_PTR_NE_MSG(test, real_fn_addr, core::ptr::null_mut(),
    "Tried to deactivate a core::ptr::null_mut() stub.");
// Look up the existing stub for this function.
    res = kunit_find_resource(test,
    __kunit_static_stub_resource_match,
    real_fn_addr);
// Error out if the stub doesn't exist.
    KUNIT_ASSERT_PTR_NE_MSG(test, res, core::ptr::null_mut(),
    "Tried to deactivate a nonexistent stub.");
// Free the stub. We 'put' twice, as we got a reference
// from kunit_find_resource()
//
    kunit_remove_resource(test, res);
    kunit_put_resource(res);
    }
    EXPORT_SYMBOL_GPL(kunit_deactivate_static_stub);
// Helper function for kunit_activate_static_stub(). The macro does
// typechecking, so use it instead.
//
    void __kunit_activate_static_stub(struct kunit *test,
    void *real_fn_addr,
    void *replacement_addr)
    {
    struct kunit_static_stub_ctx *ctx;
    struct kunit_resource *res;
    KUNIT_ASSERT_PTR_NE_MSG(test, real_fn_addr, core::ptr::null_mut(),
    "Tried to activate a stub for function core::ptr::null_mut()");
// If the replacement address is NULL, deactivate the stub.
    if (!replacement_addr) {
    kunit_deactivate_static_stub(test, real_fn_addr);
    return;
    }
// Look up any existing stubs for this function, and replace them.
    res = kunit_find_resource(test,
    __kunit_static_stub_resource_match,
    real_fn_addr);
    if (res) {
    ctx = res.data;
    ctx.replacement_addr = replacement_addr;
// We got an extra reference from find_resource(), so put it.
    kunit_put_resource(res);
    } else {
    ctx = kmalloc_obj(*ctx);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);
    ctx.real_fn_addr = real_fn_addr;
    ctx.replacement_addr = replacement_addr;
    res = kunit_alloc_resource(test, core::ptr::null_mut(),
    &__kunit_static_stub_resource_free,
    GFP_KERNEL, ctx);
    }
    }
    EXPORT_SYMBOL_GPL(__kunit_activate_static_stub);
