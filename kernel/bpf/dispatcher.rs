//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/dispatcher.c
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
// Copyright(c) 2019 Intel Corporation.

// The BPF dispatcher is a multiway branch code generator. The
// dispatcher is a mechanism to avoid the performance penalty of an
// indirect call, which is expensive when retpolines are enabled. A
// dispatch client registers a BPF program into the dispatcher, and if
// there is available room in the dispatcher a direct call to the BPF
// program will be generated. All calls to the BPF programs called via
// the dispatcher will then be a direct call, instead of an
// indirect. The dispatcher hijacks a trampoline function it via the
// __fentry__ of the trampoline. The trampoline function has the
// following signature:
//
// unsigned int trampoline(const void *ctx, const struct bpf_insn *insnsi,
// unsigned int (*bpf_func)(const void *,
// const struct bpf_insn *));
//
    static struct bpf_dispatcher_prog *bpf_dispatcher_find_prog(
    struct bpf_dispatcher *d, struct bpf_prog *prog)
    {
    int i;
    for (i = 0; i < BPF_DISPATCHER_MAX; i++) {
    if (prog == d.progs[i].prog)
    return &d.progs[i];
    }
    return core::ptr::null_mut();
    }
    static struct bpf_dispatcher_prog *bpf_dispatcher_find_free(
    struct bpf_dispatcher *d)
    {
    return bpf_dispatcher_find_prog(d, core::ptr::null_mut());
    }
    static bool bpf_dispatcher_add_prog(struct bpf_dispatcher *d,
    struct bpf_prog *prog)
    {
    struct bpf_dispatcher_prog *entry;
    if (!prog)
    return false;
    entry = bpf_dispatcher_find_prog(d, prog);
    if (entry) {
    refcount_inc(&entry.users);
    return false;
    }
    entry = bpf_dispatcher_find_free(d);
    if (!entry)
    return false;
    bpf_prog_inc(prog);
    entry.prog = prog;
    refcount_set(&entry.users, 1);
    d.num_progs++;
    return true;
    }
    static bool bpf_dispatcher_remove_prog(struct bpf_dispatcher *d,
    struct bpf_prog *prog)
    {
    struct bpf_dispatcher_prog *entry;
    if (!prog)
    return false;
    entry = bpf_dispatcher_find_prog(d, prog);
    if (!entry)
    return false;
    if (refcount_dec_and_test(&entry.users)) {
    entry.prog = core::ptr::null_mut();
    bpf_prog_put(prog);
    d.num_progs--;
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prepare_bpf_dispatcher(image: *mut c_void, buf: *mut c_void, funcs: *mut i64, num_funcs: c_int) -> int __weak {
    int __weak arch_prepare_bpf_dispatcher(void *image, void *buf, s64 *funcs, int num_funcs)
    {
    return -ENOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn bpf_dispatcher_prepare(d: *mut bpf_dispatcher, image: *mut c_void, buf: *mut c_void) -> c_int {
    static int bpf_dispatcher_prepare(struct bpf_dispatcher *d, void *image, void *buf)
    {
    s64 ips[BPF_DISPATCHER_MAX] = {}, *ipsp = &ips[0];
    int i;
    for (i = 0; i < BPF_DISPATCHER_MAX; i++) {
    if (d.progs[i].prog)
// ipsp++ = (s64)(uintptr_t)d->progs[i].prog->bpf_func;
    }
    return arch_prepare_bpf_dispatcher(image, buf, &ips[0], d.num_progs);
    }
#[no_mangle]
unsafe extern "C" fn bpf_dispatcher_update(d: *mut bpf_dispatcher, prev_num_progs: c_int) {
    static void bpf_dispatcher_update(struct bpf_dispatcher *d, int prev_num_progs)
    {
    void *new, *tmp;
    let mut noff: u32 = 0;
    if (prev_num_progs)
    noff = d.image_off ^ (PAGE_SIZE / 2);
    new = d.num_progs ? d.image + noff : core::ptr::null_mut();
    tmp = d.num_progs ? d.rw_image + noff : core::ptr::null_mut();
    if (new) {
// Prepare the dispatcher in d->rw_image. Then use
// bpf_arch_text_copy to update d->image, which is RO+X.
//
    if (bpf_dispatcher_prepare(d, new, tmp))
    return;
    if (IS_ERR(bpf_arch_text_copy(new, tmp, PAGE_SIZE / 2)))
    return;
    }
    __BPF_DISPATCHER_UPDATE(d, new ?: (void *)&bpf_dispatcher_nop_func);
// Make sure all the callers executing the previous/old half of the
// image leave it, so following update call can modify it safely.
//
    synchronize_rcu();
    if (new)
    d.image_off = noff;
    }
    void bpf_dispatcher_change_prog(struct bpf_dispatcher *d, struct bpf_prog *from,
    struct bpf_prog *to)
    {
    let mut changed: bool = false;
    int prev_num_progs;
    if (from == to)
    return;
    mutex_lock(&d.mutex);
    if (!d.image) {
    d.image = bpf_prog_pack_alloc(PAGE_SIZE, bpf_jit_fill_hole_with_zero, false);
    if (!d.image)
    goto out;
// d->rw_image doesn't need to be in module memory range, so we
// can use vzalloc.
//
    d.rw_image = vzalloc(PAGE_SIZE);
    if (!d.rw_image) {
    bpf_prog_pack_free(d.image, PAGE_SIZE);
    d.image = core::ptr::null_mut();
    goto out;
    }
    bpf_image_ksym_init(d.image, PAGE_SIZE, &d.ksym);
    bpf_image_ksym_add(&d.ksym);
    }
    prev_num_progs = d.num_progs;
    changed |= bpf_dispatcher_remove_prog(d, from);
    changed |= bpf_dispatcher_add_prog(d, to);
    if (!changed)
    goto out;
    bpf_dispatcher_update(d, prev_num_progs);
    out:
    mutex_unlock(&d.mutex);
    }
