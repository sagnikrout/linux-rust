//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/clone_attach_btf_id.c
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
// Copyright (c) 2025 Meta

//
// Test that bpf_program__clone() respects caller-provided attach_btf_id
// override via bpf_prog_load_opts.
//
// The BPF program has SEC("fentry/bpf_fentry_test1"). Clone it twice
// from the same prepared object: first with no opts (callback resolves
// attach_btf_id from sec_name), then with attach_btf_id overridden to
// bpf_fentry_test2. Verify each loaded program's attach_btf_id via
// bpf_prog_get_info_by_fd().
//
#[no_mangle]
unsafe extern "C" fn get_prog_attach_btf_id(prog_fd: c_int) -> c_int {
    static int get_prog_attach_btf_id(int prog_fd)
    {
    let mut info: bpf_prog_info = {};
    let mut info_len: __u32 = sizeof(info);
    int err;
    err = bpf_prog_get_info_by_fd(prog_fd, &info, &info_len);
    if (err)
    return err;
    return info.attach_btf_id;
    }
#[no_mangle]
pub unsafe extern "C" fn test_clone_attach_btf_id() {
    void test_clone_attach_btf_id(void)
    {
    struct clone_attach_btf_id *skel;
    let mut fd1: c_int = -1, fd2 = -1, err;
    int btf_id_test1, btf_id_test2;
    btf_id_test1 = libbpf_find_vmlinux_btf_id("bpf_fentry_test1", BPF_TRACE_FENTRY);
    if (!ASSERT_GT(btf_id_test1, 0, "find_btf_id_test1"))
    return;
    btf_id_test2 = libbpf_find_vmlinux_btf_id("bpf_fentry_test2", BPF_TRACE_FENTRY);
    if (!ASSERT_GT(btf_id_test2, 0, "find_btf_id_test2"))
    return;
    skel = clone_attach_btf_id__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    err = bpf_object__prepare(skel.obj);
    if (!ASSERT_OK(err, "obj_prepare"))
    goto out;
// Clone with no opts — callback resolves BTF from sec_name
    fd1 = bpf_program__clone(skel.progs.fentry_handler, core::ptr::null_mut());
    if (!ASSERT_GE(fd1, 0, "clone_default"))
    goto out;
    ASSERT_EQ(get_prog_attach_btf_id(fd1), btf_id_test1,
    "attach_btf_id_default");
//
// Clone with attach_btf_id override pointing to a different
// function. The BPF program never accesses arguments, so the
// load succeeds regardless of signature mismatch.
//
    LIBBPF_OPTS(bpf_prog_load_opts, opts,
    .attach_btf_id = btf_id_test2,
    );
    fd2 = bpf_program__clone(skel.progs.fentry_handler, &opts);
    if (!ASSERT_GE(fd2, 0, "clone_override"))
    goto out;
    ASSERT_EQ(get_prog_attach_btf_id(fd2), btf_id_test2,
    "attach_btf_id_override");
    out:
    if (fd1 >= 0)
    close(fd1);
    if (fd2 >= 0)
    close(fd2);
    clone_attach_btf_id__destroy(skel);
    }
