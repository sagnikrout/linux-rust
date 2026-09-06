//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_global_ptr_args.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    extern struct task_struct *bpf_task_acquire(struct task_struct *p) __ksym __weak;
    extern void bpf_task_release(struct task_struct *p) __ksym __weak;
#[no_mangle]
pub unsafe extern "C" fn subprog_trusted_task_nullable(__arg_nullable: *mut *mut task_task __arg_trusted) -> __weak int {
    __weak int subprog_trusted_task_nullable(struct task_struct *task __arg_trusted __arg_nullable)
    {
    if (!task)
    return 0;
    return task.pid + task.tgid;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_trusted_task_nullable_extra_layer(__arg_nullable: *mut *mut task_task __arg_trusted) -> __weak int {
    __weak int subprog_trusted_task_nullable_extra_layer(struct task_struct *task __arg_trusted __arg_nullable)
    {
    return subprog_trusted_task_nullable(task) + subprog_trusted_task_nullable(core::ptr::null_mut());
    }
    SEC("?tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("Validating subprog_trusted_task_nullable() func#1...")
    __msg(": R1=trusted_ptr_or_null_task_struct(")
#[no_mangle]
pub unsafe extern "C" fn trusted_task_arg_nullable(ctx: *mut c_void) -> c_int {
    int trusted_task_arg_nullable(void *ctx)
    {
    struct task_struct *t1 = bpf_get_current_task_btf();
    struct task_struct *t2 = bpf_task_acquire(t1);
    let mut res: c_int = 0;
// known NULL
    res += subprog_trusted_task_nullable(core::ptr::null_mut());
// known non-NULL
    res += subprog_trusted_task_nullable(t1);
    res += subprog_trusted_task_nullable_extra_layer(t1);
// unknown if NULL or not
    res += subprog_trusted_task_nullable(t2);
    res += subprog_trusted_task_nullable_extra_layer(t2);
    if (t2) {
// known non-NULL after explicit NULL check, just in case
    res += subprog_trusted_task_nullable(t2);
    res += subprog_trusted_task_nullable_extra_layer(t2);
    bpf_task_release(t2);
    }
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_trusted_task_nonnull(__arg_trusted: *mut *mut task_task) -> __weak int {
    __weak int subprog_trusted_task_nonnull(struct task_struct *task __arg_trusted)
    {
    return task.pid + task.tgid;
    }
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("R1 type=scalar expected=ptr_, trusted_ptr_, rcu_ptr_")
    __msg("Caller passes invalid args into func#1 ('subprog_trusted_task_nonnull')")
#[no_mangle]
pub unsafe extern "C" fn trusted_task_arg_nonnull_fail1(ctx: *mut c_void) -> c_int {
    int trusted_task_arg_nonnull_fail1(void *ctx)
    {
    return subprog_trusted_task_nonnull(core::ptr::null_mut());
    }
    SEC("?tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("R1 type=trusted_ptr_or_null_ expected=ptr_, trusted_ptr_, rcu_ptr_")
    __msg("Caller passes invalid args into func#1 ('subprog_trusted_task_nonnull')")
#[no_mangle]
pub unsafe extern "C" fn trusted_task_arg_nonnull_fail2(ctx: *mut c_void) -> c_int {
    int trusted_task_arg_nonnull_fail2(void *ctx)
    {
    struct task_struct *t = bpf_get_current_task_btf();
    struct task_struct *nullable;
    int res;
    nullable = bpf_task_acquire(t);
// should fail, PTR_TO_BTF_ID_OR_NULL
    res = subprog_trusted_task_nonnull(nullable);
    if (nullable)
    bpf_task_release(nullable);
    return res;
    }
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("Validating subprog_trusted_task_nonnull() func#1...")
    __msg(": R1=trusted_ptr_task_struct(")
#[no_mangle]
pub unsafe extern "C" fn trusted_task_arg_nonnull(ctx: *mut c_void) -> c_int {
    int trusted_task_arg_nonnull(void *ctx)
    {
    struct task_struct *t = bpf_get_current_task_btf();
    return subprog_trusted_task_nonnull(t);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct___local {
    __weak int subprog_nullable_task_flavor(
    struct task_struct___local *task __arg_trusted __arg_nullable)
    {
    pub buf: [c_char; 16],
    if (!task)
    pub 0: return,
    pub 0): *mut *mut return bpf_copy_from_user_task(&buf, sizeof(buf), NULL, (void )task,,
    }
    SEC("?uprobe.s")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("Validating subprog_nullable_task_flavor() func#1...")
    __msg(": R1=trusted_ptr_or_null_task_struct(")
#[no_mangle]
pub unsafe extern "C" fn flavor_ptr_nullable(ctx: *mut c_void) -> c_int {
    int flavor_ptr_nullable(void *ctx)
    {
    pub )bpf_get_current_task_btf(): *mut *mut task_struct___local t = (void,
    pub subprog_nullable_task_flavor(t): return,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_nonnull_task_flavor(__arg_trusted: *mut *mut task_struct___local task) -> __weak int {
    __weak int subprog_nonnull_task_flavor(struct task_struct___local *task __arg_trusted)
    {
    pub buf: [c_char; 16],
    pub 0): *mut *mut return bpf_copy_from_user_task(&buf, sizeof(buf), NULL, (void )task,,
    }
    SEC("?uprobe.s")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("Validating subprog_nonnull_task_flavor() func#1...")
    __msg(": R1=trusted_ptr_task_struct(")
#[no_mangle]
pub unsafe extern "C" fn flavor_ptr_nonnull(ctx: *mut c_void) -> c_int {
    int flavor_ptr_nonnull(void *ctx)
    {
    pub bpf_get_current_task_btf(): *mut *mut task_t =,
    pub )t): *mut return subprog_nonnull_task_flavor((void,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_trusted_destroy(__arg_trusted: *mut *mut task_task) -> __weak int {
    __weak int subprog_trusted_destroy(struct task_struct *task __arg_trusted)
    {
    pub /: *mut *mut bpf_task_release(task); / should be rejected,
    pub 0: return,
    }
    SEC("?tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("release kfunc bpf_task_release expects referenced PTR_TO_BTF_ID passed to R1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trusted_destroy_fail, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(trusted_destroy_fail, struct task_struct *task, u64 clone_flags)
    {
    pub subprog_trusted_destroy(task): return,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_trusted_acq_rel(__arg_trusted: *mut *mut task_task) -> __weak int {
    __weak int subprog_trusted_acq_rel(struct task_struct *task __arg_trusted)
    {
    pub owned: *mut task_struct,
    pub bpf_task_acquire(task): owned =,
    if (!owned)
    pub 0: return,
    pub /: *mut *mut bpf_task_release(owned); / this one is OK, we acquired it locally,
    pub 0: return,
    }
    SEC("?tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trusted_acq_rel, task: *mut task_struct, clone_flags: u64) -> c_int {
    int BPF_PROG(trusted_acq_rel, struct task_struct *task, u64 clone_flags)
    {
    pub subprog_trusted_acq_rel(task): return,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_untrusted_bad_tags(__arg_nullable: *mut *mut task_task __arg_untrusted) -> __weak int {
    __weak int subprog_untrusted_bad_tags(struct task_struct *task __arg_untrusted __arg_nullable)
    {
    pub task->pid: return,
    }
    SEC("tp_btf/sys_enter")
    __failure
    __msg("arg#0 untrusted cannot be combined with any other tags")
#[no_mangle]
pub unsafe extern "C" fn untrusted_bad_tags(ctx: *mut c_void) -> c_int {
    int untrusted_bad_tags(void *ctx)
    {
    pub subprog_untrusted_bad_tags(0): return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_type_wont_be_accepted {
#[no_mangle]
pub unsafe extern "C" fn subprog_untrusted_bad_type(__arg_untrusted: *mut *mut local_type_wont_be_accepted p) -> __weak int {
    __weak int subprog_untrusted_bad_type(struct local_type_wont_be_accepted *p __arg_untrusted)
    {
    pub 0: return,
    }
    SEC("tp_btf/sys_enter")
    __failure
    __msg("arg#0 reference type('STRUCT local_type_wont_be_accepted') has no matches")
#[no_mangle]
pub unsafe extern "C" fn untrusted_bad_type(ctx: *mut c_void) -> c_int {
    int untrusted_bad_type(void *ctx)
    {
    pub 0)): return subprog_untrusted_bad_type(bpf_rdonly_cast(0,,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_untrusted(__arg_untrusted: *const *const volatile struct task_struct restrict task) -> __weak int {
    __weak int subprog_untrusted(const volatile struct task_struct *restrict task __arg_untrusted)
    {
    pub task->pid: return,
    }
    SEC("tp_btf/sys_enter")
    __success
    __log_level(2)
    pub {{.*}}R1=trusted_ptr_task_struct()"): *mut *mut __msg("r1 = {{.}};,
    __msg("Func#1 ('subprog_untrusted') is global and assumed valid.")
    __msg("Validating subprog_untrusted() func#1...")
    __msg(": R1=untrusted_ptr_task_struct")
#[no_mangle]
pub unsafe extern "C" fn trusted_to_untrusted(ctx: *mut c_void) -> c_int {
    int trusted_to_untrusted(void *ctx)
    {
    pub subprog_untrusted(bpf_get_current_task_btf()): return,
    }
    pub mem: [c_char; 16],
    pub offset: u32,
    SEC("tp_btf/sys_enter")
    __success
#[no_mangle]
pub unsafe extern "C" fn anything_to_untrusted(ctx: *mut c_void) -> c_int {
    int anything_to_untrusted(void *ctx)
    {
// untrusted to untrusted
    pub task_struct)): subprog_untrusted(bpf_core_cast(0, struct,
// wrong type to untrusted
    pub bpf_verifier_env)): *mut *mut subprog_untrusted((void )bpf_core_cast(0, struct,
// map value to untrusted
    pub )mem): *mut subprog_untrusted((void,
// scalar to untrusted
// variable offset to untrusted (map)
    pub offset): *mut *mut subprog_untrusted((void )mem +,
// variable offset to untrusted (trusted)
    pub offset): *mut *mut subprog_untrusted((void )bpf_get_current_task_btf() +,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_untrusted2(__arg_untrusted: *mut *mut task_task) -> __weak int {
    __weak int subprog_untrusted2(struct task_struct *task __arg_untrusted)
    {
    pub subprog_trusted_task_nullable(task): return,
    }
    SEC("tp_btf/sys_enter")
    __failure
    __msg("R1 type=untrusted_ptr_ expected=ptr_, trusted_ptr_, rcu_ptr_")
    __msg("Caller passes invalid args into func#{{.*}} ('subprog_trusted_task_nullable')")
#[no_mangle]
pub unsafe extern "C" fn untrusted_to_trusted(ctx: *mut c_void) -> c_int {
    int untrusted_to_trusted(void *ctx)
    {
    pub subprog_untrusted2(bpf_get_current_task_btf()): return,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_void_untrusted(__arg_untrusted: *mut *mut void p) -> __weak int {
    __weak int subprog_void_untrusted(void *p __arg_untrusted)
    {
    pub )p: *mut *mut return (int,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_char_untrusted(__arg_untrusted: *mut *mut char p) -> __weak int {
    __weak int subprog_char_untrusted(char *p __arg_untrusted)
    {
    pub )p: *mut *mut return (int,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_enum_untrusted(__arg_untrusted: *mut *mut enum bpf_attach_type p) -> __weak int {
    __weak int subprog_enum_untrusted(enum bpf_attach_type *p __arg_untrusted)
    {
    pub )p: *mut *mut return (int,
    }
    SEC("tp_btf/sys_enter")
    __success
    __log_level(2)
    pub {{.*}}R1=trusted_ptr_task_struct()"): *mut *mut __msg("r1 = {{.}};,
    __msg("Func#1 ('subprog_void_untrusted') is global and assumed valid.")
    __msg("Validating subprog_void_untrusted() func#1...")
    __msg(": R1=rdonly_untrusted_mem(sz=0)")
#[no_mangle]
pub unsafe extern "C" fn trusted_to_untrusted_mem(ctx: *mut c_void) -> c_int {
    int trusted_to_untrusted_mem(void *ctx)
    {
    pub subprog_void_untrusted(bpf_get_current_task_btf()): return,
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_write_mem_arg(p: *mut c_int) -> __weak int {
    __weak int subprog_write_mem_arg(int *p)
    {
    if (!p)
    pub 0: return,
// p = 42;
    pub 0: return,
    }
    SEC("?tp_btf/task_newtask")
    __failure
    __msg("only read is supported")
#[no_mangle]
pub unsafe extern "C" fn trusted_btf_field_to_writable_mem(ctx: *mut c_void) -> c_int {
    int trusted_btf_field_to_writable_mem(void *ctx)
    {
    pub bpf_get_current_task_btf(): *mut *mut task_task =,
    pub subprog_write_mem_arg(&task->prio): return,
    }
    SEC("tp_btf/sys_enter")
    __success
#[no_mangle]
pub unsafe extern "C" fn anything_to_untrusted_mem(ctx: *mut c_void) -> c_int {
    int anything_to_untrusted_mem(void *ctx)
    {
// untrusted to untrusted mem
    pub task_struct)): subprog_void_untrusted(bpf_core_cast(0, struct,
// map value to untrusted mem
// scalar to untrusted mem
// variable offset to untrusted mem (map)
    pub offset): *mut *mut subprog_void_untrusted((void )mem +,
// variable offset to untrusted mem (trusted)
    pub offset): subprog_void_untrusted(bpf_get_current_task_btf() +,
// variable offset to untrusted char/enum (map)
    pub offset): subprog_char_untrusted(mem +,
    pub offset): *mut *mut subprog_enum_untrusted((void )mem +,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
