//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/preload/bpf_preload_kern.c
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

    static struct bpf_link *maps_link, *progs_link;
    static struct iterators_bpf *skel;
#[no_mangle]
unsafe extern "C" fn free_links_and_skel() {
    static void free_links_and_skel(void)
    {
    if (!IS_ERR_OR_NULL(maps_link))
    bpf_link_put(maps_link);
    if (!IS_ERR_OR_NULL(progs_link))
    bpf_link_put(progs_link);
    iterators_bpf__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn preload(obj: *mut bpf_preload_info) -> c_int {
    static int preload(struct bpf_preload_info *obj)
    {
    strscpy(obj[0].link_name, "maps.debug", sizeof(obj[0].link_name));
    obj[0].link = maps_link;
    strscpy(obj[1].link_name, "progs.debug", sizeof(obj[1].link_name));
    obj[1].link = progs_link;
    return 0;
    }
    static struct bpf_preload_ops ops = {
    .preload = preload,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn load_skel() -> c_int {
    static int load_skel(void)
    {
    int err;
    skel = iterators_bpf__open();
    if (!skel)
    return -ENOMEM;
    err = iterators_bpf__load(skel);
    if (err)
    goto out;
    err = iterators_bpf__attach(skel);
    if (err)
    goto out;
    maps_link = bpf_link_get_from_fd(skel.links.dump_bpf_map_fd);
    if (IS_ERR(maps_link)) {
    err = PTR_ERR(maps_link);
    goto out;
    }
    progs_link = bpf_link_get_from_fd(skel.links.dump_bpf_prog_fd);
    if (IS_ERR(progs_link)) {
    err = PTR_ERR(progs_link);
    goto out;
    }
// Avoid taking over stdin/stdout/stderr of init process. Zeroing out
// makes skel_closenz() a no-op later in iterators_bpf__destroy().
//
    close_fd(skel.links.dump_bpf_map_fd);
    skel.links.dump_bpf_map_fd = 0;
    close_fd(skel.links.dump_bpf_prog_fd);
    skel.links.dump_bpf_prog_fd = 0;
    return 0;
    out:
    free_links_and_skel();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn load() -> int __init {
    static int __init load(void)
    {
    int err;
    err = load_skel();
    if (err)
    return err;
    bpf_preload_ops = &ops;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn fini() -> void __exit {
    static void __exit fini(void)
    {
    bpf_preload_ops = core::ptr::null_mut();
    free_links_and_skel();
    }
    late_initcall(load);
    module_exit(fini);
    MODULE_IMPORT_NS("BPF_INTERNAL");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Embedded BPF programs for introspection in bpffs");
