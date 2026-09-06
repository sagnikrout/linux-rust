//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_global_subprogs.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

// The compiler may be able to detect the access to uninitialized
    memory in the routines performing out of bound memory accesses and
    emit warnings about it.  This is the case of GCC. */

    int arr[1];
    int unkn_idx;
    let mut call_dead_subprog: volatile bool = false;
#[no_mangle]
pub unsafe extern "C" fn global_bad() -> __noinline long {
    __noinline long global_bad(void)
    {
    return arr[unkn_idx]; /* BOOM */
    }
#[no_mangle]
pub unsafe extern "C" fn global_good() -> __noinline long {
    __noinline long global_good(void)
    {
    return arr[0];
    }
#[no_mangle]
pub unsafe extern "C" fn global_calls_bad() -> __noinline long {
    __noinline long global_calls_bad(void)
    {
    return global_good() + global_bad() /* does BOOM indirectly */;
    }
#[no_mangle]
pub unsafe extern "C" fn global_calls_good_only() -> __noinline long {
    __noinline long global_calls_good_only(void)
    {
    return global_good();
    }
#[no_mangle]
pub unsafe extern "C" fn global_dead() -> __noinline long {
    __noinline long global_dead(void)
    {
    return arr[0] * 2;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 6) -> __success {
    __success __log_level(6)
// main prog is validated completely first
    __msg("('global_calls_good_only') is global and assumed valid.")
// eventually global_good() is transitively validated as well
    __msg("Validating global_good() func")
    __msg("('global_good') is safe for any args that match its prototype")
    __msg("subprog 0 (chained_global_func_calls_success) main insns_self 7 insns_total 7 stack")
    __msg("subprog {{[0-9]+}} (global_calls_good_only) global insns_self 2 insns_total 2 stack")

    __msg("subprog {{[0-9]+}} (global_good) global insns_self 3 insns_total 3 stack")
    __msg("processed 12 insns")

    __msg("subprog {{[0-9]+}} (global_good) global insns_self 5 insns_total 5 stack")
    __msg("processed 14 insns")

#[no_mangle]
pub unsafe extern "C" fn chained_global_func_calls_success() -> c_int {
    int chained_global_func_calls_success(void)
    {
    let mut sum: c_int = 0;
    if (call_dead_subprog)
    sum += global_dead();
    return global_calls_good_only() + sum;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
// main prog validated successfully first
    __msg("('global_calls_bad') is global and assumed valid.")
// eventually we validate global_bad() and fail
    __msg("Validating global_bad() func")
    __msg("math between map_value pointer and register") /* BOOM */
#[no_mangle]
pub unsafe extern "C" fn chained_global_func_calls_bad() -> c_int {
    int chained_global_func_calls_bad(void)
    {
    return global_calls_bad();
    }
// do out of bounds access forcing verifier to fail verification if this
// global func is called
//
#[no_mangle]
pub unsafe extern "C" fn global_unsupp(mem: *const c_int) -> __noinline int {
    __noinline int global_unsupp(const int *mem)
    {
    if (!mem)
    return 0;
    return mem[100]; /* BOOM */
    }
    let mut skip_unsupp_global: volatile bool = true;
    SEC("?raw_tp")
    __success
#[no_mangle]
pub unsafe extern "C" fn guarded_unsupp_global_called() -> c_int {
    int guarded_unsupp_global_called(void)
    {
    if (!skip_unsupp_global)
    return global_unsupp(core::ptr::null_mut());
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("Func#1 ('global_unsupp') is global and assumed valid.")
    __msg("Validating global_unsupp() func#1...")
    __msg("value is outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn unguarded_unsupp_global_called() -> c_int {
    int unguarded_unsupp_global_called(void)
    {
    let mut x: c_int = 0;
    return global_unsupp(&x);
    }
    long stack[128];
#[no_mangle]
pub unsafe extern "C" fn subprog_nullable_ptr_bad(p: *mut c_int) -> __weak int {
    __weak int subprog_nullable_ptr_bad(int *p)
    {
    return (*p) * 2; /* bad, missing null check */
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("invalid mem access 'mem_or_null'")
#[no_mangle]
pub unsafe extern "C" fn arg_tag_nullable_ptr_fail(ctx: *mut c_void) -> c_int {
    int arg_tag_nullable_ptr_fail(void *ctx)
    {
    let mut x: c_int = 42;
    return subprog_nullable_ptr_bad(&x);
    }
    typedef struct {
    int x;
    } user_struct_t;
#[no_mangle]
pub unsafe extern "C" fn subprog_user_anon_mem(t: *mut user_struct_t) -> __noinline __weak int {
    __noinline __weak int subprog_user_anon_mem(user_struct_t *t)
    {
    return t ? t.x : 0;
    }
    SEC("?tracepoint")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("Caller passes invalid args into func#1 ('subprog_user_anon_mem')")
#[no_mangle]
pub unsafe extern "C" fn anon_user_mem_invalid(ctx: *mut c_void) -> c_int {
    int anon_user_mem_invalid(void *ctx)
    {
// can't pass PTR_TO_CTX as user memory
    return subprog_user_anon_mem(ctx);
    }
    SEC("?tracepoint")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
    __msg("Func#1 ('subprog_user_anon_mem') is safe for any args that match its prototype")
#[no_mangle]
pub unsafe extern "C" fn anon_user_mem_valid(ctx: *mut c_void) -> c_int {
    int anon_user_mem_valid(void *ctx)
    {
    let mut t: user_struct_t = { .x = 42 };
    return subprog_user_anon_mem(&t);
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_user_anon_mem_huge((*p)[0x3fffffff]: *mut c_int) -> __noinline __weak int {
    __noinline __weak int subprog_user_anon_mem_huge(int (*p)[0x3fffffff])
    {
    return p ? (*p)[1] : 0;
    }
    SEC("?tracepoint")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("R1 memory size 4294967292 is too large")
#[no_mangle]
pub unsafe extern "C" fn anon_user_mem_huge_size_invalid(ctx: *mut c_void) -> c_int {
    int anon_user_mem_huge_size_invalid(void *ctx)
    {
    int (*p)[0x3fffffff];
    let mut tiny: c_int = 42;
    p = (void *)&tiny;
    return subprog_user_anon_mem_huge(p) + tiny;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_nonnull_ptr_good(__arg_nonnull: *mut *mut int p1, __arg_nonnull: *mut *mut int p2) -> __noinline __weak int {
    __noinline __weak int subprog_nonnull_ptr_good(int *p1 __arg_nonnull, int *p2 __arg_nonnull)
    {
    return (*p1) * (*p2); /* good, no need for core::ptr::null_mut() checks */
    }
    let mut x: c_int = 47;
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_nonnull_ptr_good(ctx: *mut c_void) -> c_int {
    int arg_tag_nonnull_ptr_good(void *ctx)
    {
    let mut y: c_int = 74;
    return subprog_nonnull_ptr_good(&x, &y);
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("R1 is expected to be non-core::ptr::null_mut()")
#[no_mangle]
pub unsafe extern "C" fn arg_tag_nonnull_ptr_null_bad(ctx: *mut c_void) -> c_int {
    int arg_tag_nonnull_ptr_null_bad(void *ctx)
    {
    let mut y: c_int = 74;
    return subprog_nonnull_ptr_good(core::ptr::null_mut(), &y);
    }
// this global subprog can be now called from many types of entry progs, each
// with different context type
//
#[no_mangle]
pub unsafe extern "C" fn subprog_ctx_tag(__arg_ctx: *mut *mut void ctx) -> __weak int {
    __weak int subprog_ctx_tag(void *ctx __arg_ctx)
    {
    return bpf_get_stack(ctx, stack, sizeof(stack), 0);
    }
#[no_mangle]
pub unsafe extern "C" fn raw_tp_canonical(__arg_ctx: *mut *mut bpf_raw_tracepoint_args ctx) -> __weak int {
    __weak int raw_tp_canonical(struct bpf_raw_tracepoint_args *ctx __arg_ctx)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn raw_tp_u64_array(__arg_ctx: *mut *mut u64 ctx) -> __weak int {
    __weak int raw_tp_u64_array(u64 *ctx __arg_ctx)
    {
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_raw_tp(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_raw_tp(void *ctx)
    {
    return subprog_ctx_tag(ctx) + raw_tp_canonical(ctx) + raw_tp_u64_array(ctx);
    }
    SEC("?raw_tp.w")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_raw_tp_writable(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_raw_tp_writable(void *ctx)
    {
    return subprog_ctx_tag(ctx) + raw_tp_canonical(ctx) + raw_tp_u64_array(ctx);
    }
    SEC("?tp_btf/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_raw_tp_btf(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_raw_tp_btf(void *ctx)
    {
    return subprog_ctx_tag(ctx) + raw_tp_canonical(ctx) + raw_tp_u64_array(ctx);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whatever {
#[no_mangle]
pub unsafe extern "C" fn tp_whatever(__arg_ctx: *mut *mut whatever ctx) -> __weak int {
    __weak int tp_whatever(struct whatever *ctx __arg_ctx)
    {
    pub 0: return,
    }
    SEC("?tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_tp(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_tp(void *ctx)
    {
    pub tp_whatever(ctx): return subprog_ctx_tag(ctx) +,
    }
#[no_mangle]
pub unsafe extern "C" fn kprobe_subprog_pt_regs(__arg_ctx: *mut *mut pt_regs ctx) -> __weak int {
    __weak int kprobe_subprog_pt_regs(struct pt_regs *ctx __arg_ctx)
    {
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn kprobe_subprog_typedef(__arg_ctx: *mut *mut bpf_user_pt_regs_t ctx) -> __weak int {
    __weak int kprobe_subprog_typedef(bpf_user_pt_regs_t *ctx __arg_ctx)
    {
    pub 0: return,
    }
    SEC("?kprobe")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_kprobe(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_kprobe(void *ctx)
    {
    return subprog_ctx_tag(ctx) +
    kprobe_subprog_pt_regs(ctx) +
    }
    __weak int perf_subprog_regs(

    struct user_regs_struct *ctx __arg_ctx

// user_pt_regs typedef is anonymous struct, so only `void *` works
    void *ctx __arg_ctx

    struct user_pt_regs *ctx __arg_ctx

    struct pt_regs *ctx __arg_ctx

    )
    {
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn perf_subprog_typedef(__arg_ctx: *mut *mut bpf_user_pt_regs_t ctx) -> __weak int {
    __weak int perf_subprog_typedef(bpf_user_pt_regs_t *ctx __arg_ctx)
    {
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn perf_subprog_canonical(__arg_ctx: *mut *mut bpf_perf_event_data ctx) -> __weak int {
    __weak int perf_subprog_canonical(struct bpf_perf_event_data *ctx __arg_ctx)
    {
    pub 0: return,
    }
    SEC("?perf_event")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_perf(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_perf(void *ctx)
    {
    return subprog_ctx_tag(ctx) +
    perf_subprog_regs(ctx) +
    perf_subprog_typedef(ctx) +
    }
#[no_mangle]
pub unsafe extern "C" fn iter_subprog_void(__arg_ctx: *mut *mut void ctx) -> __weak int {
    __weak int iter_subprog_void(void *ctx __arg_ctx)
    {
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn iter_subprog_typed(__arg_ctx: *mut *mut bpf_iter__task ctx) -> __weak int {
    __weak int iter_subprog_typed(struct bpf_iter__task *ctx __arg_ctx)
    {
    pub 0: return,
    }
    SEC("?iter/task")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_iter_task(ctx: *mut bpf_iter__task) -> c_int {
    int arg_tag_ctx_iter_task(struct bpf_iter__task *ctx)
    {
    pub 1: return (iter_subprog_void(ctx) + iter_subprog_typed(ctx)) &,
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_subprog_void(__arg_ctx: *mut *mut void ctx) -> __weak int {
    __weak int tracing_subprog_void(void *ctx __arg_ctx)
    {
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn tracing_subprog_u64(__arg_ctx: *mut *mut u64 ctx) -> __weak int {
    __weak int tracing_subprog_u64(u64 *ctx __arg_ctx)
    {
    pub 0: return,
    }
    pub acc: c_int,
    SEC("?fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: arg_tag_ctx_fentry) -> c_int {
    int BPF_PROG(arg_tag_ctx_fentry)
    {
    pub tracing_subprog_u64(ctx): acc += tracing_subprog_void(ctx) +,
    pub 0: return,
    }
    SEC("?fexit/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: arg_tag_ctx_fexit) -> c_int {
    int BPF_PROG(arg_tag_ctx_fexit)
    {
    pub tracing_subprog_u64(ctx): acc += tracing_subprog_void(ctx) +,
    pub 0: return,
    }
    SEC("?fmod_ret/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: arg_tag_ctx_fmod_ret) -> c_int {
    int BPF_PROG(arg_tag_ctx_fmod_ret)
    {
    pub tracing_subprog_u64(ctx): return tracing_subprog_void(ctx) +,
    }
    SEC("?lsm/bpf")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: arg_tag_ctx_lsm) -> c_int {
    int BPF_PROG(arg_tag_ctx_lsm)
    {
    pub ret: c_int,
    pub tracing_subprog_u64(ctx): ret = tracing_subprog_void(ctx) +,
    pub -1): set_if_not_errno_or_zero(ret,,
    pub ret: return,
    }
    SEC("?struct_ops/test_1")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: arg_tag_ctx_struct_ops) -> c_int {
    int BPF_PROG(arg_tag_ctx_struct_ops)
    {
    pub tracing_subprog_u64(ctx): return tracing_subprog_void(ctx) +,
    }
    SEC(".struct_ops")
    struct bpf_dummy_ops dummy_1 = {
    .test_1 = (void *)arg_tag_ctx_struct_ops,
}

    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_syscall(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_syscall(void *ctx)
    {
    return tracing_subprog_void(ctx) + tracing_subprog_u64(ctx) + tp_whatever(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn syscall_array_bpf_for(__arg_ctx: *mut *mut void ctx) -> __weak int {
    __weak int syscall_array_bpf_for(void *ctx __arg_ctx)
    {
    int *arr = ctx;
    int i;
    bpf_for(i, 0, 100)
    arr[i] *= i;
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_syscall_bpf_for(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_syscall_bpf_for(void *ctx)
    {
    return syscall_array_bpf_for(ctx);
    }
    SEC("syscall")
    __auxiliary
#[no_mangle]
pub unsafe extern "C" fn syscall_tailcall_target(ctx: *mut c_void) -> c_int {
    int syscall_tailcall_target(void *ctx)
    {
    return syscall_array_bpf_for(ctx);
    }
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __array(values, int (void *));
    } syscall_prog_array SEC(".maps") = {
    .values = {
    [0] = (void *)&syscall_tailcall_target,
    },
    };
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_syscall_tailcall(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_syscall_tailcall(void *ctx)
    {
    bpf_tail_call(ctx, &syscall_prog_array, 0);
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("dereference of modified ctx ptr R1 off=8 disallowed")
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_syscall_tailcall_fixed_off_bad(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_syscall_tailcall_fixed_off_bad(void *ctx)
    {
    char *p = ctx;
    p += 8;
    bpf_tail_call(p, &syscall_prog_array, 0);
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("variable ctx access var_off=(0x0; 0x4) disallowed")
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_syscall_tailcall_var_off_bad(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_syscall_tailcall_var_off_bad(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 4;
    p += off;
    bpf_tail_call(p, &syscall_prog_array, 0);
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("dereference of modified ctx ptr R1 off=8 disallowed")
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_syscall_fixed_off_bad(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_syscall_fixed_off_bad(void *ctx)
    {
    char *p = ctx;
    p += 8;
    return subprog_ctx_tag(p);
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __failure {
    __failure __log_level(2)
    __msg("variable ctx access var_off=(0x0; 0x4) disallowed")
#[no_mangle]
pub unsafe extern "C" fn arg_tag_ctx_syscall_var_off_bad(ctx: *mut c_void) -> c_int {
    int arg_tag_ctx_syscall_var_off_bad(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 4;
    p += off;
    return subprog_ctx_tag(p);
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_dynptr(dptr: *mut bpf_dynptr) -> __weak int {
    __weak int subprog_dynptr(struct bpf_dynptr *dptr)
    {
    long *d, t, buf[1] = {};
    d = bpf_dynptr_data(dptr, 0, sizeof(long));
    if (!d)
    return 0;
    t = *d + 1;
    d = bpf_dynptr_slice(dptr, 0, &buf, sizeof(long));
    if (!d)
    return t;
    t = *d + 2;
    return t;
    }
    SEC("?xdp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn arg_tag_dynptr(ctx: *mut xdp_md) -> c_int {
    int arg_tag_dynptr(struct xdp_md *ctx)
    {
    struct bpf_dynptr dptr;
    bpf_dynptr_from_xdp(ctx, 0, &dptr);
    return subprog_dynptr(&dptr);
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn foo() {
    void foo(void)
    {
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(!read_ok": "R0) -> __failure {
    __failure __msg("R0 !read_ok")
#[no_mangle]
pub unsafe extern "C" fn return_from_void_global(skb: *mut __sk_buff) -> c_int {
    int return_from_void_global(struct __sk_buff *skb)
    {
    foo();
    asm volatile(
    "r1 = r0;"
    :::
    );
    return 0;
    }
    char _license[] SEC("license") = "GPL";
