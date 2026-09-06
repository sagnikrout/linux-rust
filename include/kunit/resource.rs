//! Automatically rewritten from C Header to Rust Module
//! Source: include/kunit/resource.h
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
// KUnit resource API for test managed resources (allocations, etc.).
//
// Copyright (C) 2022, Google LLC.
// Author: Daniel Latypov <dlatypov@google.com>
//

extern "C" {
    pub fn int(: *mut *mut kunit_resource_init_t)(struct kunit_resource, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut kunit_resource_free_t)(struct kunit_resource) -> typedef;
}
//
// struct kunit_resource - represents a *test managed resource
// @data: for the user to store arbitrary data.
// @name: optional name
// @free: a user supplied function to free the resource.
//
// Represents a *test managed resource*, a resource which will automatically be
// cleaned up at the end of a test case. This cleanup is performed by the 'free'
// function. The struct kunit_resource itself is freed automatically with
// kfree() if it was allocated by KUnit (e.g., by kunit_alloc_resource()), but
// must be freed by the user otherwise.
//
// Resources are reference counted so if a resource is retrieved via
// kunit_alloc_and_get_resource() or kunit_find_resource(), we need
// to call kunit_put_resource() to reduce the resource reference count
// when finished with it.  Note that kunit_alloc_resource() does not require a
// kunit_resource_put() because it does not retrieve the resource itself.
//
// Example:
//
// .. code-block:: c
//
// struct kunit_kmalloc_params {
// size_t size;
// gfp_t gfp;
// };
//
// static int kunit_kmalloc_init(struct kunit_resource *res, void *context)
// {
// struct kunit_kmalloc_params *params = context;
// res->data = kmalloc(params->size, params->gfp);
//
// if (!res->data)
// return -ENOMEM;
//
// return 0;
// }
//
// static void kunit_kmalloc_free(struct kunit_resource *res)
// {
// kfree(res->data);
// }
//
// void *kunit_kmalloc(struct kunit *test, size_t size, gfp_t gfp)
// {
// struct kunit_kmalloc_params params;
//
// params.size = size;
// params.gfp = gfp;
//
// return kunit_alloc_resource(test, kunit_kmalloc_init,
// kunit_kmalloc_free, gfp, &params);
// }
//
// Resources can also be named, with lookup/removal done on a name
// basis also.  kunit_add_named_resource(), kunit_find_named_resource()
// and kunit_destroy_named_resource().  Resource names must be
// unique within the test instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_resource {
    pub data: *mut c_void,
    pub name: *const c_char,
    pub free: kunit_resource_free_t,
// private: internal use only.
    pub refcount: kref,
    pub node: list_head,
    pub should_kfree: bool,
}

//
// kunit_get_resource() - Hold resource for use.  Should not need to be used
// by most users as we automatically get resources
// retrieved by kunit_find_resource*().
// @res: resource
//
// Called when refcount reaches zero via kunit_put_resource();
// should not be called directly.
//
// 'res' is valid here, as if should_kfree is set, res->free may not free
// 'res' itself, just res->data
//
// kunit_put_resource() - When caller is done with retrieved resource,
// kunit_put_resource() should be called to drop
// reference count.  The resource list maintains
// a reference count on resources, so if no users
// are utilizing a resource and it is removed from
// the resource list, it will be freed via the
// associated free function (if any).  Only
// needs to be used if we alloc_and_get() or
// find() resource.
// @res: resource
//
// __kunit_add_resource() - Internal helper to add a resource.
//
// res->should_kfree is not initialised.
// @test: The test context object.
// @init: a user-supplied function to initialize the result (if needed).  If
// none is supplied, the resource data value is simply set to @data.
// If an init function is supplied, @data is passed to it instead.
// @free: a user-supplied function to free the resource (if needed).
// @res: The resource.
// @data: value to pass to init function or set in resource data field.
//
// kunit_add_resource() - Add a *test managed resource*.
// @test: The test context object.
// @init: a user-supplied function to initialize the result (if needed).  If
// none is supplied, the resource data value is simply set to @data.
// If an init function is supplied, @data is passed to it instead.
// @free: a user-supplied function to free the resource (if needed).
// @res: The resource.
// @data: value to pass to init function or set in resource data field.
//
extern "C" {
    pub fn __kunit_add_resource(_arg: test, _arg: init, _arg: free, _arg: res, _arg: data) -> return;
}
//
// kunit_add_named_resource() - Add a named *test managed resource*.
// @test: The test context object.
// @init: a user-supplied function to initialize the resource data, if needed.
// @free: a user-supplied function to free the resource data, if needed.
// @res: The resource.
// @name: name to be set for resource.
// @data: value to pass to init function or set in resource data field.
//
extern "C" {
    pub fn __kunit_add_resource(_arg: test, _arg: init, _arg: free, _arg: res, _arg: data) -> return;
}
//
// kunit_alloc_and_get_resource() - Allocates and returns a *test managed resource*.
// @test: The test context object.
// @init: a user supplied function to initialize the resource.
// @free: a user supplied function to free the resource (if needed).
// @internal_gfp: gfp to use for internal allocations, if unsure, use GFP_KERNEL
// @context: for the user to pass in arbitrary data to the init function.
//
// Allocates a *test managed resource*, a resource which will automatically be
// cleaned up at the end of a test case. See &struct kunit_resource for an
// example.
//
// This is effectively identical to kunit_alloc_resource, but returns the
// struct kunit_resource pointer, not just the 'data' pointer. It therefore
// also increments the resource's refcount, so kunit_put_resource() should be
// called when you've finished with it.
//
// Note: KUnit needs to allocate memory for a kunit_resource object. You must
// specify an @internal_gfp that is compatible with the use context of your
// resource.
//
// bump refcount for get; kunit_resource_put() should be called
// when done.
//
// kunit_alloc_resource() - Allocates a *test managed resource*.
// @test: The test context object.
// @init: a user supplied function to initialize the resource.
// @free: a user supplied function to free the resource (if needed).
// @internal_gfp: gfp to use for internal allocations, if unsure, use GFP_KERNEL
// @context: for the user to pass in arbitrary data to the init function.
//
// Allocates a *test managed resource*, a resource which will automatically be
// cleaned up at the end of a test case. See &struct kunit_resource for an
// example.
//
// Note: KUnit needs to allocate memory for a kunit_resource object. You must
// specify an @internal_gfp that is compatible with the use context of your
// resource.
//
// kunit_resource_name_match() - Match a resource with the same name.
// @test: Test case to which the resource belongs.
// @res: The resource.
// @match_name: The name to match against.
//
// kunit_find_resource() - Find a resource using match function/data.
// @test: Test case to which the resource belongs.
// @match: match function to be applied to resources/match data.
// @match_data: data to be used in matching.
//
// kunit_find_named_resource() - Find a resource using match name.
// @test: Test case to which the resource belongs.
// @name: match name.
//
// kunit_destroy_resource() - Find a kunit_resource and destroy it.
// @test: Test case to which the resource belongs.
// @match: Match function. Returns whether a given resource matches @match_data.
// @match_data: Data passed into @match.
//
// RETURNS:
// 0 if kunit_resource is found and freed, -ENOENT if not found.
//
// kunit_remove_resource() - remove resource from resource list associated with
// test.
// @test: The test context object.
// @res: The resource to be removed.
//
// Note that the resource will not be immediately freed since it is likely
// the caller has a reference to it via alloc_and_get() or find();
// in this case a final call to kunit_put_resource() is required.
//
extern "C" {
    pub fn kunit_remove_resource(test: *mut kunit, res: *mut kunit_resource);
}
// A 'deferred action' function to be used with kunit_add_action.
extern "C" {
    pub fn void(: *mut kunit_action_t)(void) -> typedef;
}
//
// KUNIT_DEFINE_ACTION_WRAPPER() - Wrap a function for use as a deferred action.
//
// @wrapper: The name of the new wrapper function define.
// @orig: The original function to wrap.
// @arg_type: The type of the argument accepted by @orig.
//
// Defines a wrapper for a function which accepts a single, pointer-sized
// argument. This wrapper can then be passed to kunit_add_action() and
// similar. This should be used in preference to casting a function
// directly to kunit_action_t, as casting function pointers will break
// control flow integrity (CFI), leading to crashes.
//

//
// kunit_add_action() - Call a function when the test ends.
// @test: Test case to associate the action with.
// @action: The function to run on test exit
// @ctx: Data passed into @func
//
// Defer the execution of a function until the test exits, either normally or
// due to a failure.  @ctx is passed as additional context. All functions
// registered with kunit_add_action() will execute in the opposite order to that
// they were registered in.
//
// This is useful for cleaning up allocated memory and resources, as these
// functions are called even if the test aborts early due to, e.g., a failed
// assertion.
//
// See also: devm_add_action() for the devres equivalent.
//
// Returns:
// 0 on success, an error if the action could not be deferred.
//
extern "C" {
    pub fn kunit_add_action(test: *mut kunit, action: *mut kunit_action_t, ctx: *mut c_void) -> c_int;
}
//
// kunit_add_action_or_reset() - Call a function when the test ends.
// @test: Test case to associate the action with.
// @action: The function to run on test exit
// @ctx: Data passed into @func
//
// Defer the execution of a function until the test exits, either normally or
// due to a failure.  @ctx is passed as additional context. All functions
// registered with kunit_add_action() will execute in the opposite order to that
// they were registered in.
//
// This is useful for cleaning up allocated memory and resources, as these
// functions are called even if the test aborts early due to, e.g., a failed
// assertion.
//
// If the action cannot be created (e.g., due to the system being out of memory),
// then action(ctx) will be called immediately, and an error will be returned.
//
// See also: devm_add_action_or_reset() for the devres equivalent.
//
// Returns:
// 0 on success, an error if the action could not be deferred.
//
// kunit_remove_action() - Cancel a matching deferred action.
// @test: Test case the action is associated with.
// @action: The deferred function to cancel.
// @ctx: The context passed to the deferred function to trigger.
//
// Prevent an action deferred via kunit_add_action() from executing when the
// test terminates.
//
// If the function/context pair was deferred multiple times, only the most
// recent one will be cancelled.
//
// See also: devm_remove_action() for the devres equivalent.
//
// kunit_release_action() - Run a matching action call immediately.
// @test: Test case the action is associated with.
// @action: The deferred function to trigger.
// @ctx: The context passed to the deferred function to trigger.
//
// Execute a function deferred via kunit_add_action()) immediately, rather than
// when the test ends.
//
// If the function/context pair was deferred multiple times, it will only be
// executed once here. The most recent deferral will no longer execute when
// the test ends.
//
// kunit_release_action(test, func, ctx);
// is equivalent to
// func(ctx);
// kunit_remove_action(test, func, ctx);
//
// See also: devm_release_action() for the devres equivalent.
//
