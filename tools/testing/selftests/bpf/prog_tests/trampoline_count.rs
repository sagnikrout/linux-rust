//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/trampoline_count.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inst {
    pub obj: *mut bpf_object,
    pub link: *mut bpf_link,
}

    static struct bpf_program *load_prog(char *file, char *name, struct inst *inst)
    {
    struct bpf_object *obj;
    struct bpf_program *prog;
    int err;
    obj = bpf_object__open_file(file, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(obj, "obj_open_file"))
    return core::ptr::null_mut();
    inst.obj = obj;
    err = bpf_object__load(obj);
    if (!ASSERT_OK(err, "obj_load"))
    return core::ptr::null_mut();
    prog = bpf_object__find_program_by_name(obj, name);
    if (!ASSERT_OK_PTR(prog, "obj_find_prog"))
    return core::ptr::null_mut();
    return prog;
    }
#[no_mangle]
pub unsafe extern "C" fn test_trampoline_count() {
    void test_trampoline_count(void)
    {
    char *file = "test_trampoline_count.bpf.o";
    char *const progs[] = { "fentry_test", "fmod_ret_test", "fexit_test" };
    int bpf_max_tramp_links, i;
    struct bpf_program *prog;
    struct bpf_link *link;
    struct inst *inst;
    bpf_max_tramp_links = get_bpf_max_tramp_links();
    if (!ASSERT_GE(bpf_max_tramp_links, 1, "bpf_max_tramp_links"))
    return;
    inst = calloc(bpf_max_tramp_links + 1, sizeof(*inst));
    if (!ASSERT_OK_PTR(inst, "inst"))
    return;
// attach 'allowed' trampoline programs
    for (i = 0; i < bpf_max_tramp_links; i++) {
    prog = load_prog(file, progs[i % ARRAY_SIZE(progs)], &inst[i]);
    if (!prog)
    goto cleanup;
    link = bpf_program__attach(prog);
    if (!ASSERT_OK_PTR(link, "attach_prog"))
    goto cleanup;
    inst[i].link = link;
    }
// and try 1 extra..
    prog = load_prog(file, "fmod_ret_test", &inst[i]);
    if (!prog)
    goto cleanup;
// ..that needs to fail
    link = bpf_program__attach(prog);
    if (!ASSERT_ERR_PTR(link, "attach_prog")) {
    inst[i].link = link;
    goto cleanup;
    }
// with E2BIG error
    if (!ASSERT_EQ(libbpf_get_error(link), -E2BIG, "E2BIG"))
    goto cleanup;
    if (!ASSERT_EQ(link, core::ptr::null_mut(), "ptr_is_null"))
    goto cleanup;
// and finally execute the probe
    ASSERT_OK(trigger_module_test_read(256), "trigger_module_test_read");
    cleanup:
    for (; i >= 0; i--) {
    bpf_link__destroy(inst[i].link);
    bpf_object__close(inst[i].obj);
    }
    free(inst);
    }
