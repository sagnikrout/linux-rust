//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/exceptions.c
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

pub const ETH_P_IP: c_uint = 0x0800;

    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 4);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } jmp_table SEC(".maps");
#[no_mangle]
unsafe extern "C" fn static_func(i: u64) -> __noinline int {
    static __noinline int static_func(u64 i)
    {
    bpf_throw(32);
    return i;
    }
#[no_mangle]
pub unsafe extern "C" fn global2static_simple(i: u64) -> __noinline int {
    __noinline int global2static_simple(u64 i)
    {
    static_func(i + 2);
    return i - 1;
    }
#[no_mangle]
pub unsafe extern "C" fn global2static(i: u64) -> __noinline int {
    __noinline int global2static(u64 i)
    {
    if (i == ETH_P_IP)
    bpf_throw(16);
    return static_func(i);
    }
#[no_mangle]
unsafe extern "C" fn static2global(i: u64) -> __noinline int {
    static __noinline int static2global(u64 i)
    {
    return global2static(i) + i;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_always_1(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_always_1(struct __sk_buff *ctx)
    {
    bpf_throw(64);
    return 0;
    }
// In this case, the global func will never be seen executing after call to
// static subprog, hence verifier will DCE the remaining instructions. Ensure we
// are resilient to that.
//
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_always_2(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_always_2(struct __sk_buff *ctx)
    {
    return global2static_simple(ctx.protocol);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_unwind_1(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_unwind_1(struct __sk_buff *ctx)
    {
    return static2global(bpf_ntohs(ctx.protocol));
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_unwind_2(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_unwind_2(struct __sk_buff *ctx)
    {
    return static2global(bpf_ntohs(ctx.protocol) - 1);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_default(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_default(struct __sk_buff *ctx)
    {
    bpf_throw(0);
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_default_value(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_default_value(struct __sk_buff *ctx)
    {
    bpf_throw(5);
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_tail_call_target(ctx: *mut __sk_buff) -> c_int {
    int exception_tail_call_target(struct __sk_buff *ctx)
    {
    bpf_throw(16);
    return 0;
    }
    static __noinline
#[no_mangle]
pub unsafe extern "C" fn exception_tail_call_subprog(ctx: *mut __sk_buff) -> c_int {
    int exception_tail_call_subprog(struct __sk_buff *ctx)
    {
    let mut ret: volatile int = 10;
    bpf_tail_call_static(ctx, &jmp_table, 0);
    return ret;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_tail_call(ctx: *mut __sk_buff) -> c_int {
    let mut ret: volatile int = 0;
    ret = exception_tail_call_subprog(ctx);
    return ret + 8;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn throw_11() {
    void throw_11(void)
    {
    bpf_throw(11);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_from_void_global(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_from_void_global(struct __sk_buff *ctx)
    {
    throw_11();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exception_ext_global(ctx: *mut __sk_buff) -> __noinline int {
    __noinline int exception_ext_global(struct __sk_buff *ctx)
    {
    let mut ret: volatile int = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exception_ext_static(ctx: *mut __sk_buff) -> __noinline int {
    static __noinline int exception_ext_static(struct __sk_buff *ctx)
    {
    return exception_ext_global(ctx);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_ext(ctx: *mut __sk_buff) -> c_int {
    int exception_ext(struct __sk_buff *ctx)
    {
    return exception_ext_static(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb_mod_global(cookie: u64) -> __noinline int {
    __noinline int exception_cb_mod_global(u64 cookie)
    {
    let mut ret: volatile int = 0;
    return ret;
    }
// Example of how the exception callback supplied during verification can still
// introduce extensions by calling to dummy global functions, and alter runtime
// behavior.
//
// Right now we don't allow freplace attachment to exception callback itself,
// but if the need arises this restriction is technically feasible to relax in
// the future.
//
#[no_mangle]
pub unsafe extern "C" fn exception_cb_mod(cookie: u64) -> __noinline int {
    __noinline int exception_cb_mod(u64 cookie)
    {
    return exception_cb_mod_global(cookie) + cookie + 10;
    }
    SEC("tc")
    __exception_cb(exception_cb_mod)
#[no_mangle]
pub unsafe extern "C" fn exception_ext_mod_cb_runtime(ctx: *mut __sk_buff) -> c_int {
    int exception_ext_mod_cb_runtime(struct __sk_buff *ctx)
    {
    bpf_throw(25);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog(ctx: *mut __sk_buff) -> __noinline static int {
    __noinline static int subprog(struct __sk_buff *ctx)
    {
    return bpf_ktime_get_ns();
    }
#[no_mangle]
pub unsafe extern "C" fn throwing_subprog(ctx: *mut __sk_buff) -> __noinline static int {
    __noinline static int throwing_subprog(struct __sk_buff *ctx)
    {
    if (ctx.tstamp)
    bpf_throw(0);
    return bpf_ktime_get_ns();
    }
#[no_mangle]
pub unsafe extern "C" fn global_subprog(ctx: *mut __sk_buff) -> __noinline int {
    __noinline int global_subprog(struct __sk_buff *ctx)
    {
    return bpf_ktime_get_ns();
    }
#[no_mangle]
pub unsafe extern "C" fn throwing_global_subprog(ctx: *mut __sk_buff) -> __noinline int {
    __noinline int throwing_global_subprog(struct __sk_buff *ctx)
    {
    if (ctx.tstamp)
    bpf_throw(0);
    return bpf_ktime_get_ns();
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_subprog(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_subprog(struct __sk_buff *ctx)
    {
    switch (ctx.protocol) {
    case 1:
    return subprog(ctx);
    case 2:
    return global_subprog(ctx);
    case 3:
    return throwing_subprog(ctx);
    case 4:
    return throwing_global_subprog(ctx);
    default:
    break;
    }
    bpf_throw(1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_nz_gfunc(c: u64) -> __noinline int {
    __noinline int assert_nz_gfunc(u64 c)
    {
    let mut cookie: volatile u64 = c;
    bpf_assert(cookie != 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_zero_gfunc(c: u64) -> __noinline int {
    __noinline int assert_zero_gfunc(u64 c)
    {
    let mut cookie: volatile u64 = c;
    bpf_assert(bpf_cmp_unlikely(cookie, ==, 0));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_neg_gfunc(c: i64) -> __noinline int {
    __noinline int assert_neg_gfunc(s64 c)
    {
    let mut cookie: volatile s64 = c;
    bpf_assert(bpf_cmp_unlikely(cookie, <, 0));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_pos_gfunc(c: i64) -> __noinline int {
    __noinline int assert_pos_gfunc(s64 c)
    {
    let mut cookie: volatile s64 = c;
    bpf_assert(bpf_cmp_unlikely(cookie, >, 0));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_negeq_gfunc(c: i64) -> __noinline int {
    __noinline int assert_negeq_gfunc(s64 c)
    {
    let mut cookie: volatile s64 = c;
    bpf_assert(bpf_cmp_unlikely(cookie, <=, -1));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_poseq_gfunc(c: i64) -> __noinline int {
    __noinline int assert_poseq_gfunc(s64 c)
    {
    let mut cookie: volatile s64 = c;
    bpf_assert(bpf_cmp_unlikely(cookie, >=, 1));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_nz_gfunc_with(c: u64) -> __noinline int {
    __noinline int assert_nz_gfunc_with(u64 c)
    {
    let mut cookie: volatile u64 = c;
    bpf_assert_with(cookie != 0, cookie + 100);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_zero_gfunc_with(c: u64) -> __noinline int {
    __noinline int assert_zero_gfunc_with(u64 c)
    {
    let mut cookie: volatile u64 = c;
    bpf_assert_with(bpf_cmp_unlikely(cookie, ==, 0), cookie + 100);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_neg_gfunc_with(c: i64) -> __noinline int {
    __noinline int assert_neg_gfunc_with(s64 c)
    {
    let mut cookie: volatile s64 = c;
    bpf_assert_with(bpf_cmp_unlikely(cookie, <, 0), cookie + 100);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_pos_gfunc_with(c: i64) -> __noinline int {
    __noinline int assert_pos_gfunc_with(s64 c)
    {
    let mut cookie: volatile s64 = c;
    bpf_assert_with(bpf_cmp_unlikely(cookie, >, 0), cookie + 100);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_negeq_gfunc_with(c: i64) -> __noinline int {
    __noinline int assert_negeq_gfunc_with(s64 c)
    {
    let mut cookie: volatile s64 = c;
    bpf_assert_with(bpf_cmp_unlikely(cookie, <=, -1), cookie + 100);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn assert_poseq_gfunc_with(c: i64) -> __noinline int {
    __noinline int assert_poseq_gfunc_with(s64 c)
    {
    let mut cookie: volatile s64 = c;
    bpf_assert_with(bpf_cmp_unlikely(cookie, >=, 1), cookie + 100);
    return 0;
    }

    SEC("tc")							\
    int exception##tag##name(struct __sk_buff *ctx)			\
    {								\
    return name(cookie) + 1;				\
    }
    check_assert(assert_nz_gfunc, 5, _);
    check_assert(assert_zero_gfunc, 0, _);
    check_assert(assert_neg_gfunc, -100, _);
    check_assert(assert_pos_gfunc, 100, _);
    check_assert(assert_negeq_gfunc, -1, _);
    check_assert(assert_poseq_gfunc, 1, _);
    check_assert(assert_nz_gfunc_with, 5, _);
    check_assert(assert_zero_gfunc_with, 0, _);
    check_assert(assert_neg_gfunc_with, -100, _);
    check_assert(assert_pos_gfunc_with, 100, _);
    check_assert(assert_negeq_gfunc_with, -1, _);
    check_assert(assert_poseq_gfunc_with, 1, _);
    check_assert(assert_nz_gfunc, 0, _bad_);
    check_assert(assert_zero_gfunc, 5, _bad_);
    check_assert(assert_neg_gfunc, 100, _bad_);
    check_assert(assert_pos_gfunc, -100, _bad_);
    check_assert(assert_negeq_gfunc, 1, _bad_);
    check_assert(assert_poseq_gfunc, -1, _bad_);
    check_assert(assert_nz_gfunc_with, 0, _bad_);
    check_assert(assert_zero_gfunc_with, 5, _bad_);
    check_assert(assert_neg_gfunc_with, 100, _bad_);
    check_assert(assert_pos_gfunc_with, -100, _bad_);
    check_assert(assert_negeq_gfunc_with, 1, _bad_);
    check_assert(assert_poseq_gfunc_with, -1, _bad_);
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_assert_range(ctx: *mut __sk_buff) -> c_int {
    int exception_assert_range(struct __sk_buff *ctx)
    {
    let mut time: u64 = bpf_ktime_get_ns();
    bpf_assert_range(time, 0, ~0ULL);
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_assert_range_with(ctx: *mut __sk_buff) -> c_int {
    int exception_assert_range_with(struct __sk_buff *ctx)
    {
    let mut time: u64 = bpf_ktime_get_ns();
    bpf_assert_range_with(time, 0, ~0ULL, 10);
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_bad_assert_range(ctx: *mut __sk_buff) -> c_int {
    int exception_bad_assert_range(struct __sk_buff *ctx)
    {
    let mut time: u64 = bpf_ktime_get_ns();
    bpf_assert_range(time, -100, 100);
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_bad_assert_range_with(ctx: *mut __sk_buff) -> c_int {
    int exception_bad_assert_range_with(struct __sk_buff *ctx)
    {
    let mut time: u64 = bpf_ktime_get_ns();
    bpf_assert_range_with(time, -1000, 1000, 10);
    return 1;
    }

    && defined(__BPF_FEATURE_STACK_ARGUMENT)
    let mut has_stack_arg: volatile bool = true;
    let mut arg1: c_long = 1, arg2 = 2, arg3 = 3, arg4 = 4, arg5 = 5;
    let mut arg6: c_long = 6, arg7 = 7, arg8 = 8, arg9 = 9, arg10 = 10;
    __noinline static long throwing_many_args(long a, long b, long c, long d,
    long e, long f, long g, long h,
    long i, long j)
    {
    bpf_throw(a + b + c + d + e + f + g + h + i + j);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb_sa(cookie: u64) -> __noinline int {
    __noinline int exception_cb_sa(u64 cookie)
    {
    return cookie + 1;
    }
    SEC("tc")
    __exception_cb(exception_cb_sa)
#[no_mangle]
pub unsafe extern "C" fn exception_throw_stack_arg(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_stack_arg(struct __sk_buff *ctx)
    {
    throwing_many_args(arg1, arg2, arg3, arg4, arg5,
    arg6, arg7, arg8, arg9, arg10);
    return 0;
    }
    __noinline static long no_throw_many_args(long a, long b, long c, long d,
    long e, long f, long g, long h,
    long i, long j)
    {
    return a + b + c + d + e + f + g + h + i + j;
    }
    SEC("tc")
    __exception_cb(exception_cb_sa)
#[no_mangle]
pub unsafe extern "C" fn exception_throw_after_stack_arg(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_after_stack_arg(struct __sk_buff *ctx)
    {
    long ret;
    ret = no_throw_many_args(arg1, arg2, arg3, arg4, arg5,
    arg6, arg7, arg8, arg9, arg10);
    if (ret > 0)
    bpf_throw(ret);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_throw_sa(val: c_long) -> __noinline static long {
    __noinline static long subprog_throw_sa(long val)
    {
    throwing_many_args(val, val + 1, val + 2, val + 3, val + 4,
    val + 5, val + 6, val + 7, val + 8, val + 9);
    return 0;
    }
    SEC("tc")
    __exception_cb(exception_cb_sa)
#[no_mangle]
pub unsafe extern "C" fn exception_throw_subprog_stack_arg(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_subprog_stack_arg(struct __sk_buff *ctx)
    {
    subprog_throw_sa(arg1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_throw_after_sa(val: c_long) -> __noinline static long {
    __noinline static long subprog_throw_after_sa(long val)
    {
    long ret;
    ret = no_throw_many_args(val, val + 1, val + 2, val + 3, val + 4,
    val + 5, val + 6, val + 7, val + 8, val + 9);
    if (ret > 0)
    bpf_throw(ret);
    return 0;
    }
    SEC("tc")
    __exception_cb(exception_cb_sa)
#[no_mangle]
pub unsafe extern "C" fn exception_throw_subprog_after_stack_arg(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_subprog_after_stack_arg(struct __sk_buff *ctx)
    {
    subprog_throw_after_sa(arg1);
    return 0;
    }

    let mut has_stack_arg: volatile bool = false;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_stack_arg(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_stack_arg(struct __sk_buff *ctx)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_after_stack_arg(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_after_stack_arg(struct __sk_buff *ctx)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_subprog_stack_arg(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_subprog_stack_arg(struct __sk_buff *ctx)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn exception_throw_subprog_after_stack_arg(ctx: *mut __sk_buff) -> c_int {
    int exception_throw_subprog_after_stack_arg(struct __sk_buff *ctx)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
