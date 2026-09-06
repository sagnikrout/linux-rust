//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/reference_tracking.c
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
pub unsafe extern "C" fn test_reference_tracking() {
    void test_reference_tracking(void)
    {
    const char *file = "test_sk_lookup_kern.bpf.o";
    const char *obj_name = "ref_track";
    DECLARE_LIBBPF_OPTS(bpf_object_open_opts, open_opts,
    .object_name = obj_name,
    .relaxed_maps = true,
    );
    struct bpf_object *obj_iter, *obj = core::ptr::null_mut();
    struct bpf_program *prog;
    let mut duration: __u32 = 0;
    let mut err: c_int = 0;
    obj_iter = bpf_object__open_file(file, &open_opts);
    if (!ASSERT_OK_PTR(obj_iter, "obj_iter_open_file"))
    return;
    if (CHECK(strcmp(bpf_object__name(obj_iter), obj_name), "obj_name",
    "wrong obj name '%s', expected '%s'\n",
    bpf_object__name(obj_iter), obj_name))
    goto cleanup;
    bpf_object__for_each_program(prog, obj_iter) {
    struct bpf_program *p;
    const char *name;
    name = bpf_program__name(prog);
    if (!test__start_subtest(name))
    continue;
    obj = bpf_object__open_file(file, &open_opts);
    if (!ASSERT_OK_PTR(obj, "obj_open_file"))
    goto cleanup;
// all programs are not loaded by default, so just set
// autoload to true for the single prog under test
//
    p = bpf_object__find_program_by_name(obj, name);
    bpf_program__set_autoload(p, true);
// Expect verifier failure if test name has 'err'
    if (strncmp(name, "err_", sizeof("err_") - 1) == 0) {
    libbpf_print_fn_t old_print_fn;
    old_print_fn = libbpf_set_print(core::ptr::null_mut());
    err = !bpf_object__load(obj);
    libbpf_set_print(old_print_fn);
    } else {
    err = bpf_object__load(obj);
    }
    ASSERT_OK(err, name);
    bpf_object__close(obj);
    obj = core::ptr::null_mut();
    }
    cleanup:
    bpf_object__close(obj);
    bpf_object__close(obj_iter);
    }
