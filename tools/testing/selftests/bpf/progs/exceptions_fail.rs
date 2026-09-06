//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/exceptions_fail.c
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

    extern void bpf_rcu_read_lock(void) __ksym;
    extern void bpf_rcu_read_unlock(void) __ksym;
    extern void bpf_preempt_disable(void) __ksym;
    extern void bpf_preempt_enable(void) __ksym;
    extern void bpf_local_irq_save(unsigned long *) __ksym;
    extern void bpf_local_irq_restore(unsigned long *) __ksym;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub node: bpf_rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmap_elem {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 64);
    __type(key, int);
    __type(value, struct hmap_elem);
    } hmap SEC(".maps");
    private(A) struct bpf_spin_lock lock;
    private(A) struct bpf_rb_root rbtree __contains(foo, node);
    __noinline void *exception_cb_bad_ret_type1(u64 cookie)
    {
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb_bad_ret_type2(cookie: u64) -> __noinline void {
    __noinline void exception_cb_bad_ret_type2(u64 cookie)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb_bad_arg_0() -> __noinline int {
    __noinline int exception_cb_bad_arg_0(void)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb_bad_arg_2(a: c_int, b: c_int) -> __noinline int {
    __noinline int exception_cb_bad_arg_2(int a, int b)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb_ok_arg_small(a: c_int) -> __noinline int {
    __noinline int exception_cb_ok_arg_small(int a)
    {
    return 0;
    }
    SEC("?tc")
    __exception_cb(exception_cb_bad_ret_type1)
#[no_mangle]
pub unsafe extern "C" fn __msg(scalar.": "Global function exception_cb_bad_ret_type1() return value not void or) -> __failure {
    __failure __msg("Global function exception_cb_bad_ret_type1() return value not void or scalar.")
#[no_mangle]
pub unsafe extern "C" fn reject_exception_cb_type_1(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_cb_type_1(struct __sk_buff *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
    __exception_cb(exception_cb_bad_arg_0)
#[no_mangle]
pub unsafe extern "C" fn __msg(argument": "exception cb only supports single integer) -> __failure {
    __failure __msg("exception cb only supports single integer argument")
#[no_mangle]
pub unsafe extern "C" fn reject_exception_cb_type_2(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_cb_type_2(struct __sk_buff *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
    __exception_cb(exception_cb_bad_arg_2)
#[no_mangle]
pub unsafe extern "C" fn __msg(argument": "exception cb only supports single integer) -> __failure {
    __failure __msg("exception cb only supports single integer argument")
#[no_mangle]
pub unsafe extern "C" fn reject_exception_cb_type_3(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_cb_type_3(struct __sk_buff *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
    __exception_cb(exception_cb_ok_arg_small)
    __success
#[no_mangle]
pub unsafe extern "C" fn reject_exception_cb_type_4(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_cb_type_4(struct __sk_buff *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
    __exception_cb(exception_cb_bad_ret_type2)
#[no_mangle]
pub unsafe extern "C" fn __msg(void": "exception cb cannot return) -> __failure {
    __failure __msg("exception cb cannot return void")
#[no_mangle]
pub unsafe extern "C" fn reject_exception_cb_type_5(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_cb_type_5(struct __sk_buff *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    __noinline
#[no_mangle]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_cb(void *map, int *key, struct bpf_timer *timer)
    {
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(subprog": "cannot be called from callback) -> __failure {
    __failure __msg("cannot be called from callback subprog")
#[no_mangle]
pub unsafe extern "C" fn reject_async_callback_throw(ctx: *mut __sk_buff) -> c_int {
    int reject_async_callback_throw(struct __sk_buff *ctx)
    {
    struct hmap_elem *elem;
    elem = bpf_map_lookup_elem(&hmap, &(int){0});
    if (!elem)
    return 0;
    return bpf_timer_set_callback(&elem.timer, timer_cb);
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_lock(ctx: *mut __sk_buff) -> __noinline static int {
    __noinline static int subprog_lock(struct __sk_buff *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_spin_lock(&lock);
    if (ctx.len)
    bpf_throw(0);
    return ret;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "function calls are not allowed while holding a) -> __failure {
    __failure __msg("function calls are not allowed while holding a lock")
#[no_mangle]
pub unsafe extern "C" fn reject_with_lock(ctx: *mut c_void) -> c_int {
    int reject_with_lock(void *ctx)
    {
    bpf_spin_lock(&lock);
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "function calls are not allowed while holding a) -> __failure {
    __failure __msg("function calls are not allowed while holding a lock")
#[no_mangle]
pub unsafe extern "C" fn reject_subprog_with_lock(ctx: *mut c_void) -> c_int {
    int reject_subprog_with_lock(void *ctx)
    {
    return subprog_lock(ctx);
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "bpf_throw cannot be used inside bpf_rcu_read_lock-ed) -> __failure {
    __failure __msg("bpf_throw cannot be used inside bpf_rcu_read_lock-ed region")
#[no_mangle]
pub unsafe extern "C" fn reject_with_rcu_read_lock(ctx: *mut c_void) -> c_int {
    int reject_with_rcu_read_lock(void *ctx)
    {
    bpf_rcu_read_lock();
    bpf_throw(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn throwing_subprog(ctx: *mut __sk_buff) -> __noinline static int {
    __noinline static int throwing_subprog(struct __sk_buff *ctx)
    {
    if (ctx.len)
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "bpf_throw cannot be used inside bpf_rcu_read_lock-ed) -> __failure {
    __failure __msg("bpf_throw cannot be used inside bpf_rcu_read_lock-ed region")
#[no_mangle]
pub unsafe extern "C" fn reject_subprog_with_rcu_read_lock(ctx: *mut c_void) -> c_int {
    int reject_subprog_with_rcu_read_lock(void *ctx)
    {
    bpf_rcu_read_lock();
    throwing_subprog(ctx);
    bpf_rcu_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rbless(n1: *mut bpf_rb_node, n2: *const bpf_rb_node) -> bool {
    static bool rbless(struct bpf_rb_node *n1, const struct bpf_rb_node *n2)
    {
    bpf_throw(0);
    return true;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "function calls are not allowed while holding a) -> __failure {
    __failure __msg("function calls are not allowed while holding a lock")
#[no_mangle]
pub unsafe extern "C" fn reject_with_rbtree_add_throw(ctx: *mut c_void) -> c_int {
    int reject_with_rbtree_add_throw(void *ctx)
    {
    struct foo *f;
    f = bpf_obj_new(typeof(*f));
    if (!f)
    return 0;
    bpf_spin_lock(&lock);
    bpf_rbtree_add(&rbtree, &f.node, rbless);
    bpf_spin_unlock(&lock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn reject_with_reference(ctx: *mut c_void) -> c_int {
    int reject_with_reference(void *ctx)
    {
    struct foo *f;
    f = bpf_obj_new(typeof(*f));
    if (!f)
    return 0;
    bpf_throw(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn global_subprog_may_throw(ctx: *mut __sk_buff) -> __noinline int {
    __noinline int global_subprog_may_throw(struct __sk_buff *ctx)
    {
    if (ctx.len)
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn reject_global_subprog_throw_with_reference(ctx: *mut __sk_buff) -> c_int {
    int reject_global_subprog_throw_with_reference(struct __sk_buff *ctx)
    {
    struct foo *f;
    f = bpf_obj_new(typeof(*f));
    if (!f)
    return 0;
    if (ctx.protocol)
    global_subprog_may_throw(ctx);
    bpf_obj_drop(f);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_ref(ctx: *mut __sk_buff) -> __noinline static int {
    __noinline static int subprog_ref(struct __sk_buff *ctx)
    {
    struct foo *f;
    f = bpf_obj_new(typeof(*f));
    if (!f)
    return 0;
    bpf_throw(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_cb_ref(i: u32, ctx: *mut c_void) -> __noinline static int {
    __noinline static int subprog_cb_ref(u32 i, void *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn reject_with_cb_reference(ctx: *mut c_void) -> c_int {
    int reject_with_cb_reference(void *ctx)
    {
    struct foo *f;
    f = bpf_obj_new(typeof(*f));
    if (!f)
    return 0;
    bpf_loop(5, subprog_cb_ref, core::ptr::null_mut(), 0);
    bpf_obj_drop(f);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(callback": "cannot be called from) -> __failure {
    __failure __msg("cannot be called from callback")
#[no_mangle]
pub unsafe extern "C" fn reject_with_cb(ctx: *mut c_void) -> c_int {
    int reject_with_cb(void *ctx)
    {
    bpf_loop(5, subprog_cb_ref, core::ptr::null_mut(), 0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn reject_with_subprog_reference(ctx: *mut c_void) -> c_int {
    int reject_with_subprog_reference(void *ctx)
    {
    return subprog_ref(ctx) + 1;
    }
#[no_mangle]
pub unsafe extern "C" fn throwing_exception_cb(c: u64) -> __noinline int {
    __noinline int throwing_exception_cb(u64 c)
    {
    bpf_throw(0);
    return c;
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb1(c: u64) -> __noinline int {
    __noinline int exception_cb1(u64 c)
    {
    return c;
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb2(c: u64) -> __noinline int {
    __noinline int exception_cb2(u64 c)
    {
    return c;
    }
#[no_mangle]
unsafe extern "C" fn static_func(ctx: *mut __sk_buff) -> __noinline int {
    static __noinline int static_func(struct __sk_buff *ctx)
    {
    return exception_cb1(ctx.tstamp);
    }
#[no_mangle]
pub unsafe extern "C" fn global_func(ctx: *mut __sk_buff) -> __noinline int {
    __noinline int global_func(struct __sk_buff *ctx)
    {
    return exception_cb1(ctx.tstamp);
    }
    SEC("?tc")
    __exception_cb(throwing_exception_cb)
#[no_mangle]
pub unsafe extern "C" fn __msg(subprog": "cannot be called from callback) -> __failure {
    __failure __msg("cannot be called from callback subprog")
#[no_mangle]
pub unsafe extern "C" fn reject_throwing_exception_cb(ctx: *mut __sk_buff) -> c_int {
    int reject_throwing_exception_cb(struct __sk_buff *ctx)
    {
    return 0;
    }
    SEC("?tc")
    __exception_cb(exception_cb1)
#[no_mangle]
pub unsafe extern "C" fn __msg(directly": "cannot call exception cb) -> __failure {
    __failure __msg("cannot call exception cb directly")
#[no_mangle]
pub unsafe extern "C" fn reject_exception_cb_call_global_func(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_cb_call_global_func(struct __sk_buff *ctx)
    {
    return global_func(ctx);
    }
    SEC("?tc")
    __exception_cb(exception_cb1)
#[no_mangle]
pub unsafe extern "C" fn __msg(directly": "cannot call exception cb) -> __failure {
    __failure __msg("cannot call exception cb directly")
#[no_mangle]
pub unsafe extern "C" fn reject_exception_cb_call_static_func(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_cb_call_static_func(struct __sk_buff *ctx)
    {
    return static_func(ctx);
    }
    SEC("?tc")
    __exception_cb(exception_cb1)
    __exception_cb(exception_cb2)
#[no_mangle]
pub unsafe extern "C" fn __msg(subprog": "multiple exception callback tags for main) -> __failure {
    __failure __msg("multiple exception callback tags for main subprog")
#[no_mangle]
pub unsafe extern "C" fn reject_multiple_exception_cb(ctx: *mut __sk_buff) -> c_int {
    int reject_multiple_exception_cb(struct __sk_buff *ctx)
    {
    bpf_throw(0);
    return 16;
    }
#[no_mangle]
pub unsafe extern "C" fn exception_cb_bad_ret(c: u64) -> __noinline int {
    __noinline int exception_cb_bad_ret(u64 c)
    {
    return c;
    }
    SEC("?fentry/bpf_check")
    __exception_cb(exception_cb_bad_ret)
#[no_mangle]
pub unsafe extern "C" fn __msg(should": "At program exit the register R0 has unknown scalar value) -> __failure {
    __failure __msg("At program exit the register R0 has unknown scalar value should")
#[no_mangle]
pub unsafe extern "C" fn reject_set_exception_cb_bad_ret1(ctx: *mut c_void) -> c_int {
    int reject_set_exception_cb_bad_ret1(void *ctx)
    {
    return 0;
    }
    SEC("?fentry/bpf_check")
#[no_mangle]
pub unsafe extern "C" fn __msg(should": "At program exit the register R1 has smin=64 smax=64) -> __failure {
    __failure __msg("At program exit the register R1 has smin=64 smax=64 should")
#[no_mangle]
pub unsafe extern "C" fn reject_set_exception_cb_bad_ret2(ctx: *mut c_void) -> c_int {
    int reject_set_exception_cb_bad_ret2(void *ctx)
    {
    bpf_throw(64);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn loop_cb1(index: u32, ctx: *mut c_int) -> __noinline static int {
    __noinline static int loop_cb1(u32 index, int *ctx)
    {
    bpf_throw(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn loop_cb2(index: u32, ctx: *mut c_int) -> __noinline static int {
    __noinline static int loop_cb2(u32 index, int *ctx)
    {
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(callback": "cannot be called from) -> __failure {
    __failure __msg("cannot be called from callback")
#[no_mangle]
pub unsafe extern "C" fn reject_exception_throw_cb(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_throw_cb(struct __sk_buff *ctx)
    {
    bpf_loop(5, loop_cb1, core::ptr::null_mut(), 0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(callback": "cannot be called from) -> __failure {
    __failure __msg("cannot be called from callback")
#[no_mangle]
pub unsafe extern "C" fn reject_exception_throw_cb_diff(ctx: *mut __sk_buff) -> c_int {
    int reject_exception_throw_cb_diff(struct __sk_buff *ctx)
    {
    if (ctx.protocol)
    bpf_loop(5, loop_cb1, core::ptr::null_mut(), 0);
    else
    bpf_loop(5, loop_cb2, core::ptr::null_mut(), 0);
    return 0;
    }
    __weak
#[no_mangle]
pub unsafe extern "C" fn foo() {
    void foo(void)
    {
    bpf_throw(1);
    }
    SEC("?fentry/bpf_check")
#[no_mangle]
pub unsafe extern "C" fn __msg(should": "At program exit the register R1 has smin=1 smax=1) -> __failure {
    __failure __msg("At program exit the register R1 has smin=1 smax=1 should")
#[no_mangle]
pub unsafe extern "C" fn reject_out_of_range_global_throw(skb: *mut __sk_buff) -> c_int {
    int reject_out_of_range_global_throw(struct __sk_buff *skb)
    {
    foo();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn always_throws() -> __noinline static int {
    __noinline static int always_throws(void)
    {
    bpf_throw(0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn rcu_lock_then_throw() -> __noinline static int {
    __noinline static int rcu_lock_then_throw(void)
    {
    bpf_rcu_read_lock();
    bpf_throw(0);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "bpf_throw cannot be used inside bpf_rcu_read_lock-ed) -> __failure {
    __failure __msg("bpf_throw cannot be used inside bpf_rcu_read_lock-ed region")
#[no_mangle]
pub unsafe extern "C" fn reject_subprog_rcu_lock_throw(ctx: *mut c_void) -> c_int {
    int reject_subprog_rcu_lock_throw(void *ctx)
    {
    rcu_lock_then_throw();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "bpf_throw cannot be used inside bpf_preempt_disable-ed) -> __failure {
    __failure __msg("bpf_throw cannot be used inside bpf_preempt_disable-ed region")
#[no_mangle]
pub unsafe extern "C" fn reject_subprog_throw_preempt_lock(ctx: *mut c_void) -> c_int {
    int reject_subprog_throw_preempt_lock(void *ctx)
    {
    bpf_preempt_disable();
    always_throws();
    bpf_preempt_enable();
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "bpf_throw cannot be used inside bpf_local_irq_save-ed) -> __failure {
    __failure __msg("bpf_throw cannot be used inside bpf_local_irq_save-ed region")
#[no_mangle]
pub unsafe extern "C" fn reject_subprog_throw_irq_lock(ctx: *mut c_void) -> c_int {
    int reject_subprog_throw_irq_lock(void *ctx)
    {
    unsigned long flags;
    bpf_local_irq_save(&flags);
    always_throws();
    bpf_local_irq_restore(&flags);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
