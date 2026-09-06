//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_gotox.c
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

    __u64 in_user;
    __u64 ret_user;
    int pid;
//
// Skip all the tests if compiler doesn't support indirect jumps.
//
// If tests are skipped, then all functions below are compiled as
// dummy, such that the skeleton looks the same, and the userspace
// program can avoid any checks rather than if data->skip is set.
//

    __u64 skip SEC(".data") = 0;

    let mut skip: __u64 = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_ctx {
    pub x: __u64,
}

    __u64 some_var;
//
// This function adds code which will be replaced by a different
// number of instructions by the verifier. This adds additional
// stress on testing the insn_array maps corresponding to indirect jumps.
//
#[no_mangle]
unsafe extern "C" fn adjust_insns(x: __u64) -> __always_inline void {
    static __always_inline void adjust_insns(__u64 x)
    {
    some_var ^= x + bpf_jiffies64();
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn one_switch(ctx: *mut simple_ctx) -> c_int {
    int one_switch(struct simple_ctx *ctx)
    {
    switch (ctx.x) {
    case 0:
    adjust_insns(ctx.x + 1);
    ret_user = 2;
    break;
    case 1:
    adjust_insns(ctx.x + 7);
    ret_user = 3;
    break;
    case 2:
    adjust_insns(ctx.x + 9);
    ret_user = 4;
    break;
    case 3:
    adjust_insns(ctx.x + 11);
    ret_user = 5;
    break;
    case 4:
    adjust_insns(ctx.x + 17);
    ret_user = 7;
    break;
    default:
    adjust_insns(ctx.x + 177);
    ret_user = 19;
    break;
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn one_switch_non_zero_sec_off(ctx: *mut simple_ctx) -> c_int {
    int one_switch_non_zero_sec_off(struct simple_ctx *ctx)
    {
    switch (ctx.x) {
    case 0:
    adjust_insns(ctx.x + 1);
    ret_user = 2;
    break;
    case 1:
    adjust_insns(ctx.x + 7);
    ret_user = 3;
    break;
    case 2:
    adjust_insns(ctx.x + 9);
    ret_user = 4;
    break;
    case 3:
    adjust_insns(ctx.x + 11);
    ret_user = 5;
    break;
    case 4:
    adjust_insns(ctx.x + 17);
    ret_user = 7;
    break;
    default:
    adjust_insns(ctx.x + 177);
    ret_user = 19;
    break;
    }
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn simple_test_other_sec(ctx: *mut pt_regs) -> c_int {
    int simple_test_other_sec(struct pt_regs *ctx)
    {
    let mut x: __u64 = in_user;
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    switch (x) {
    case 0:
    adjust_insns(x + 1);
    ret_user = 2;
    break;
    case 1:
    adjust_insns(x + 7);
    ret_user = 3;
    break;
    case 2:
    adjust_insns(x + 9);
    ret_user = 4;
    break;
    case 3:
    adjust_insns(x + 11);
    ret_user = 5;
    break;
    case 4:
    adjust_insns(x + 17);
    ret_user = 7;
    break;
    default:
    adjust_insns(x + 177);
    ret_user = 19;
    break;
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn two_switches(ctx: *mut simple_ctx) -> c_int {
    int two_switches(struct simple_ctx *ctx)
    {
    switch (ctx.x) {
    case 0:
    adjust_insns(ctx.x + 1);
    ret_user = 2;
    break;
    case 1:
    adjust_insns(ctx.x + 7);
    ret_user = 3;
    break;
    case 2:
    adjust_insns(ctx.x + 9);
    ret_user = 4;
    break;
    case 3:
    adjust_insns(ctx.x + 11);
    ret_user = 5;
    break;
    case 4:
    adjust_insns(ctx.x + 17);
    ret_user = 7;
    break;
    default:
    adjust_insns(ctx.x + 177);
    ret_user = 19;
    break;
    }
    switch (ctx.x + !!ret_user) {
    case 1:
    adjust_insns(ctx.x + 7);
    ret_user = 103;
    break;
    case 2:
    adjust_insns(ctx.x + 9);
    ret_user = 104;
    break;
    case 3:
    adjust_insns(ctx.x + 11);
    ret_user = 107;
    break;
    case 4:
    adjust_insns(ctx.x + 11);
    ret_user = 205;
    break;
    case 5:
    adjust_insns(ctx.x + 11);
    ret_user = 115;
    break;
    default:
    adjust_insns(ctx.x + 177);
    ret_user = 1019;
    break;
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn big_jump_table(__attribute__((unused)): *mut *mut simple_ctx ctx) -> c_int {
    int big_jump_table(struct simple_ctx *ctx __attribute__((unused)))
    {
    const void *const jt[256] = {
    [0 ... 255] = &&default_label,
    [0] = &&l0,
    [11] = &&l11,
    [27] = &&l27,
    [31] = &&l31,
    };
    goto *jt[ctx.x & 0xff];
    l0:
    adjust_insns(ctx.x + 1);
    ret_user = 2;
    return 0;
    l11:
    adjust_insns(ctx.x + 7);
    ret_user = 3;
    return 0;
    l27:
    adjust_insns(ctx.x + 9);
    ret_user = 4;
    return 0;
    l31:
    adjust_insns(ctx.x + 11);
    ret_user = 5;
    return 0;
    default_label:
    adjust_insns(ctx.x + 177);
    ret_user = 19;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn one_jump_two_maps(__attribute__((unused)): *mut *mut simple_ctx ctx) -> c_int {
    int one_jump_two_maps(struct simple_ctx *ctx __attribute__((unused)))
    {
    __label__ l1, l2, l3, l4;
    void *jt1[2] = { &&l1, &&l2 };
    void *jt2[2] = { &&l3, &&l4 };
    let mut a: c_uint = ctx.x % 2;
    let mut b: c_uint = (ctx.x / 2) % 2;
    let mut ret: volatile int = 0;
    if (!(a < 2 && b < 2))
    return 19;
    if (ctx.x % 2)
    goto *jt1[a];
    else
    goto *jt2[b];
    l1: ret += 1;
    l2: ret += 3;
    l3: ret += 5;
    l4: ret += 7;
    ret_user = ret;
    return ret;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn one_map_two_jumps(__attribute__((unused)): *mut *mut simple_ctx ctx) -> c_int {
    int one_map_two_jumps(struct simple_ctx *ctx __attribute__((unused)))
    {
    __label__ l1, l2, l3;
    void *jt[3] = { &&l1, &&l2, &&l3 };
    let mut a: c_uint = (ctx.x >> 2) & 1;
    let mut b: c_uint = (ctx.x >> 3) & 1;
    let mut ret: volatile int = 0;
    if (ctx.x % 2)
    goto *jt[a];
    if (ctx.x % 3)
    goto *jt[a + b];
    l1: ret += 3;
    l2: ret += 5;
    l3: ret += 7;
    ret_user = ret;
    return ret;
    }
// Just to introduce some non-zero offsets in .text
#[no_mangle]
unsafe extern "C" fn f0(__arg_ctx: *mut *mut volatile struct simple_ctx ctx) -> __noinline int {
    static __noinline int f0(volatile struct simple_ctx *ctx __arg_ctx)
    {
    if (ctx)
    return 1;
    else
    return 13;
    }
    SEC("syscall") int f1(struct simple_ctx *ctx)
    {
    ret_user = 0;
    return f0(ctx);
    }
#[no_mangle]
unsafe extern "C" fn __static_global(x: __u64) -> __noinline int {
    static __noinline int __static_global(__u64 x)
    {
    switch (x) {
    case 0:
    adjust_insns(x + 1);
    ret_user = 2;
    break;
    case 1:
    adjust_insns(x + 7);
    ret_user = 3;
    break;
    case 2:
    adjust_insns(x + 9);
    ret_user = 4;
    break;
    case 3:
    adjust_insns(x + 11);
    ret_user = 5;
    break;
    case 4:
    adjust_insns(x + 17);
    ret_user = 7;
    break;
    default:
    adjust_insns(x + 177);
    ret_user = 19;
    break;
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn use_static_global1(ctx: *mut simple_ctx) -> c_int {
    int use_static_global1(struct simple_ctx *ctx)
    {
    ret_user = 0;
    return __static_global(ctx.x);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn use_static_global2(ctx: *mut simple_ctx) -> c_int {
    int use_static_global2(struct simple_ctx *ctx)
    {
    ret_user = 0;
    adjust_insns(ctx.x + 1);
    return __static_global(ctx.x);
    }
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn use_static_global_other_sec(ctx: *mut c_void) -> c_int {
    int use_static_global_other_sec(void *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    return __static_global(in_user);
    }
#[no_mangle]
pub unsafe extern "C" fn __nonstatic_global(x: __u64) -> __noinline int {
    __noinline int __nonstatic_global(__u64 x)
    {
    switch (x) {
    case 0:
    adjust_insns(x + 1);
    ret_user = 2;
    break;
    case 1:
    adjust_insns(x + 7);
    ret_user = 3;
    break;
    case 2:
    adjust_insns(x + 9);
    ret_user = 4;
    break;
    case 3:
    adjust_insns(x + 11);
    ret_user = 5;
    break;
    case 4:
    adjust_insns(x + 17);
    ret_user = 7;
    break;
    default:
    adjust_insns(x + 177);
    ret_user = 19;
    break;
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn use_nonstatic_global1(ctx: *mut simple_ctx) -> c_int {
    int use_nonstatic_global1(struct simple_ctx *ctx)
    {
    ret_user = 0;
    return __nonstatic_global(ctx.x);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn use_nonstatic_global2(ctx: *mut simple_ctx) -> c_int {
    int use_nonstatic_global2(struct simple_ctx *ctx)
    {
    ret_user = 0;
    adjust_insns(ctx.x + 1);
    return __nonstatic_global(ctx.x);
    }
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn use_nonstatic_global_other_sec(ctx: *mut c_void) -> c_int {
    int use_nonstatic_global_other_sec(void *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    return __nonstatic_global(in_user);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn load_with_nonzero_offset(ctx: *mut simple_ctx) -> c_int {
    int load_with_nonzero_offset(struct simple_ctx *ctx)
    {
    void *jj[] = { &&l1, &&l2, &&l3 };
//
// This makes LLVM to generate a load from the jj map with an offset:
// r1 = 0x0 ll
// r1 = *(u64 *)(r1 + 0x10)
// gotox r1
//
    if (ctx.x == 2)
    goto *jj[ctx.x];
    ret_user = 1;
    return 1;
    l1:
// never reached, but leave it here to outsmart LLVM
    ret_user = 0;
    return 0;
    l2:
// never reached, but leave it here to outsmart LLVM
    ret_user = 3;
    return 3;
    l3:
    ret_user = 5;
    return 5;
    }

    SEC("syscall") int TEST_NAME(void *ctx)		\
    {						\
    return 0;				\
    }
    SKIP_TEST(one_switch);
    SKIP_TEST(one_switch_non_zero_sec_off);
    SKIP_TEST(simple_test_other_sec);
    SKIP_TEST(two_switches);
    SKIP_TEST(big_jump_table);
    SKIP_TEST(one_jump_two_maps);
    SKIP_TEST(one_map_two_jumps);
    SKIP_TEST(use_static_global1);
    SKIP_TEST(use_static_global2);
    SKIP_TEST(use_static_global_other_sec);
    SKIP_TEST(use_nonstatic_global1);
    SKIP_TEST(use_nonstatic_global2);
    SKIP_TEST(use_nonstatic_global_other_sec);
    SKIP_TEST(load_with_nonzero_offset);

    char _license[] SEC("license") = "GPL";
