//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_ctx.c
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
// Converted from tools/testing/selftests/bpf/verifier/ctx.c

    static const char ctx_strncmp_target[] = "ctx";
    static const char ctx_snprintf_fmt[] = "";
    SEC("tc")
    __description("context stores via BPF_ATOMIC")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "BPF_ATOMIC stores into R1 ctx is not) -> __failure {
    __failure __msg("BPF_ATOMIC stores into R1 ctx is not allowed")
#[no_mangle]
pub unsafe extern "C" fn context_stores_via_bpf_atomic() -> __naked void {
    __naked void context_stores_via_bpf_atomic(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    lock *(u32 *)(r1 + %[__sk_buff_mark]) += w0;	\
    exit;						\
    "	:
    : __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
    : __clobber_all);
    }
    SEC("tc")
    __description("arithmetic ops make PTR_TO_CTX unusable")
#[no_mangle]
pub unsafe extern "C" fn __msg(ptr": "dereference of modified ctx) -> __failure {
    __failure __msg("dereference of modified ctx ptr")
#[no_mangle]
pub unsafe extern "C" fn make_ptr_to_ctx_unusable() -> __naked void {
    __naked void make_ptr_to_ctx_unusable(void)
    {
    asm volatile ("					\
    r1 += %[__imm_0];				\
    r0 = *(u32*)(r1 + %[__sk_buff_mark]);		\
    exit;						\
    "	:
    : __imm_const(__imm_0,
    offsetof(struct __sk_buff, data) - offsetof(struct __sk_buff, mark)),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
    : __clobber_all);
    }
    SEC("tc")
    __description("pass unmodified ctx pointer to helper")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unmodified_ctx_pointer_to_helper() -> __naked void {
    __naked void unmodified_ctx_pointer_to_helper(void)
    {
    asm volatile ("					\
    r2 = 0;						\
    call %[bpf_csum_update];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_csum_update)
    : __clobber_all);
    }
    SEC("tc")
    __description("pass modified ctx pointer to helper, 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "negative offset ctx ptr R1 off=-612) -> __failure {
    __failure __msg("negative offset ctx ptr R1 off=-612 disallowed")
#[no_mangle]
pub unsafe extern "C" fn ctx_pointer_to_helper_1() -> __naked void {
    __naked void ctx_pointer_to_helper_1(void)
    {
    asm volatile ("					\
    r1 += -612;					\
    r2 = 0;						\
    call %[bpf_csum_update];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_csum_update)
    : __clobber_all);
    }
    SEC("socket")
    __description("pass modified ctx pointer to helper, 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "negative offset ctx ptr R1 off=-612) -> __failure {
    __failure __msg("negative offset ctx ptr R1 off=-612 disallowed")
#[no_mangle]
pub unsafe extern "C" fn ctx_pointer_to_helper_2() -> __naked void {
    __naked void ctx_pointer_to_helper_2(void)
    {
    asm volatile ("					\
    r1 += -612;					\
    call %[bpf_get_socket_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_socket_cookie)
    : __clobber_all);
    }
    SEC("tc")
    __description("pass modified ctx pointer to helper, 3")
#[no_mangle]
pub unsafe extern "C" fn __msg(0x4)": "variable ctx access var_off=(0x0;) -> __failure {
    __failure __msg("variable ctx access var_off=(0x0; 0x4)")
#[no_mangle]
pub unsafe extern "C" fn ctx_pointer_to_helper_3() -> __naked void {
    __naked void ctx_pointer_to_helper_3(void)
    {
    asm volatile ("					\
    r3 = *(u32*)(r1 + 0);				\
    r3 &= 4;					\
    r1 += r3;					\
    r2 = 0;						\
    call %[bpf_csum_update];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_csum_update)
    : __clobber_all);
    }
    SEC("cgroup/sendmsg6")
    __description("pass ctx or null check, 1: ctx")
    __success
#[no_mangle]
pub unsafe extern "C" fn or_null_check_1_ctx() -> __naked void {
    __naked void or_null_check_1_ctx(void)
    {
    asm volatile ("					\
    call %[bpf_get_netns_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_netns_cookie)
    : __clobber_all);
    }
    SEC("cgroup/sendmsg6")
    __description("pass ctx or null check, 2: null")
    __success
#[no_mangle]
pub unsafe extern "C" fn or_null_check_2_null() -> __naked void {
    __naked void or_null_check_2_null(void)
    {
    asm volatile ("					\
    r1 = 0;						\
    call %[bpf_get_netns_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_netns_cookie)
    : __clobber_all);
    }
    SEC("cgroup/sendmsg6")
    __description("pass ctx or null check, 3: 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=ctx": "R1 type=scalar) -> __failure {
    __failure __msg("R1 type=scalar expected=ctx")
#[no_mangle]
pub unsafe extern "C" fn or_null_check_3_1() -> __naked void {
    __naked void or_null_check_3_1(void)
    {
    asm volatile ("					\
    r1 = 1;						\
    call %[bpf_get_netns_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_netns_cookie)
    : __clobber_all);
    }
    SEC("cgroup/sendmsg6")
    __description("pass ctx or null check, 4: ctx - const")
#[no_mangle]
pub unsafe extern "C" fn __msg(disallowed": "negative offset ctx ptr R1 off=-612) -> __failure {
    __failure __msg("negative offset ctx ptr R1 off=-612 disallowed")
#[no_mangle]
pub unsafe extern "C" fn null_check_4_ctx_const() -> __naked void {
    __naked void null_check_4_ctx_const(void)
    {
    asm volatile ("					\
    r1 += -612;					\
    call %[bpf_get_netns_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_netns_cookie)
    : __clobber_all);
    }
    SEC("cgroup/connect4")
    __description("pass ctx or null check, 5: null (connect)")
    __success
#[no_mangle]
pub unsafe extern "C" fn null_check_5_null_connect() -> __naked void {
    __naked void null_check_5_null_connect(void)
    {
    asm volatile ("					\
    r1 = 0;						\
    call %[bpf_get_netns_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_netns_cookie)
    : __clobber_all);
    }
    SEC("cgroup/post_bind4")
    __description("pass ctx or null check, 6: null (bind)")
    __success
#[no_mangle]
pub unsafe extern "C" fn null_check_6_null_bind() -> __naked void {
    __naked void null_check_6_null_bind(void)
    {
    asm volatile ("					\
    r1 = 0;						\
    call %[bpf_get_netns_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_netns_cookie)
    : __clobber_all);
    }
    SEC("cgroup/post_bind4")
    __description("pass ctx or null check, 7: ctx (bind)")
    __success
#[no_mangle]
pub unsafe extern "C" fn null_check_7_ctx_bind() -> __naked void {
    __naked void null_check_7_ctx_bind(void)
    {
    asm volatile ("					\
    call %[bpf_get_socket_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_socket_cookie)
    : __clobber_all);
    }
    SEC("cgroup/post_bind4")
    __description("pass ctx or null check, 8: null (bind)")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=ctx": "R1 type=scalar) -> __failure {
    __failure __msg("R1 type=scalar expected=ctx")
#[no_mangle]
pub unsafe extern "C" fn null_check_8_null_bind() -> __naked void {
    __naked void null_check_8_null_bind(void)
    {
    asm volatile ("					\
    r1 = 0;						\
    call %[bpf_get_socket_cookie];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_socket_cookie)
    : __clobber_all);
    }

    SEC(type)							\
    __description("narrow load on field " #field " of " #ctx)	\
    __failure __msg("invalid bpf_context access")			\
    __naked void invalid_narrow_load##ctx##field(void)		\
    {								\
    asm volatile ("						\
    r1 = *(u32 *)(r1 + %[off]);				\
    r0 = 0;							\
    exit;"							\
    :							\
    : __imm_const(off, offsetof(struct ctx, field) + 4)	\
    : __clobber_all);					\
    }
    narrow_load("cgroup/getsockopt", bpf_sockopt, sk);
    narrow_load("cgroup/getsockopt", bpf_sockopt, optval);
    narrow_load("cgroup/getsockopt", bpf_sockopt, optval_end);
    narrow_load("tc", __sk_buff, sk);
    narrow_load("cgroup/bind4", bpf_sock_addr, sk);
    narrow_load("sockops", bpf_sock_ops, sk);
    narrow_load("sockops", bpf_sock_ops, skb_data);
    narrow_load("sockops", bpf_sock_ops, skb_data_end);
    narrow_load("sockops", bpf_sock_ops, skb_hwtstamp);

    SEC(type)								\
    __description("unaligned access on field " #field " of " #ctx)		\
    __failure __msg("invalid bpf_context access")				\
    __naked void unaligned_ctx_access_##ctx##field(void)			\
    {									\
    asm volatile ("							\
    r1 = *(u%[size] *)(r1 + %[off]);				\
    r0 = 0;								\
    exit;"								\
    :								\
    : __imm_const(size, sizeof_field(struct ctx, field) * 8),	\
    __imm_const(off, offsetof(struct ctx, field) + 1)		\
    : __clobber_all);						\
    }
    unaligned_access("flow_dissector", __sk_buff, data);
    unaligned_access("netfilter", bpf_nf_ctx, skb);

    SEC(type)							\
    __description("access on " #ctx " padding after " #prev_field)	\
    __naked void padding_ctx_access_##ctx(void)			\
    {								\
    asm volatile ("						\
    r1 = *(u%[size] *)(r1 + %[off]);			\
    r0 = 0;							\
    exit;"							\
    :							\
    : __imm_const(size, sz * 8),				\
    __imm_const(off, offsetofend(struct ctx, prev_field))	\
    : __clobber_all);					\
    }
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    padding_access("cgroup/bind4", bpf_sock_addr, msg_src_ip6[3], 4);
    __success
    padding_access("sk_lookup", bpf_sk_lookup, remote_port, 2);
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    padding_access("tc", __sk_buff, tstamp_type, 2);
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    padding_access("cgroup/post_bind4", bpf_sock, dst_port, 2);
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    padding_access("sk_reuseport", sk_reuseport_md, hash, 4);
    SEC("?syscall")
    __description("syscall: write to ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_fixed_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_fixed_off_write(void *ctx)
    {
    char *p = ctx;
// (__u32 *)p = 0;
// (__u32 *)(p + 4) = 0;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: read ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_fixed_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_fixed_off_read(void *ctx)
    {
    char *p = ctx;
    volatile __u32 val;
    val = *(__u32 *)(p + 4);
    (void)val;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: unaligned read ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unaligned_fixed_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_unaligned_fixed_off_read(void *ctx)
    {
    char *p = ctx;
    volatile __u32 val;
    val = *(__u32 *)(p + 2);
    (void)val;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: unaligned write ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unaligned_fixed_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_unaligned_fixed_off_write(void *ctx)
    {
    char *p = ctx;
// (__u32 *)(p + 2) = 0;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: read ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_var_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_var_off_read(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    volatile __u32 val;
    off &= 0xfc;
    p += off;
    val = *(__u32 *)p;
    (void)val;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: write ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_var_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_var_off_write(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xfc;
    p += off;
// (__u32 *)p = 0;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: unaligned read ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unaligned_var_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_unaligned_var_off_read(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    volatile __u32 val;
    off &= 0xfc;
    off += 2;
    p += off;
    val = *(__u32 *)p;
    (void)val;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: unaligned write ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unaligned_var_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_unaligned_var_off_write(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xfc;
    off += 2;
    p += off;
// (__u32 *)p = 0;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: reject ctx access past U16_MAX with fixed offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "outside of the allowed memory) -> __failure {
    __failure __msg("outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_u16_max_fixed_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_u16_max_fixed_off(void *ctx)
    {
    char *p = ctx;
    volatile __u32 val;
    p += 65535;
    val = *(__u32 *)p;
    (void)val;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: reject ctx access past U16_MAX with variable offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "outside of the allowed memory) -> __failure {
    __failure __msg("outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_u16_max_var_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_u16_max_var_off(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    volatile __u32 val;
    off &= 0xffff;
    off += 1;
    p += off;
    val = *(__u32 *)p;
    (void)val;
    return 0;
    }
    SEC("?syscall")
    __description("syscall: reject negative variable offset ctx access")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "min value is) -> __failure {
    __failure __msg("min value is negative")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_neg_var_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_neg_var_off(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 4;
    p -= off;
    return *(__u32 *)p;
    }
    SEC("?syscall")
    __description("syscall: reject unbounded variable offset ctx access")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "unbounded memory) -> __failure {
    __failure __msg("unbounded memory access")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_unbounded_var_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_unbounded_var_off(void *ctx)
    {
    let mut off: __u64 = (__u32)bpf_get_prandom_u32();
    char *p = ctx;
    off <<= 2;
    p += off;
    return *(__u32 *)p;
    }
    SEC("?syscall")
    __description("syscall: helper read ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_fixed_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_fixed_off_read(void *ctx)
    {
    char *p = ctx;
    p += 4;
    return bpf_strncmp(p, 4, ctx_strncmp_target);
    }
    SEC("?syscall")
    __description("syscall: helper write ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_fixed_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_fixed_off_write(void *ctx)
    {
    char *p = ctx;
    p += 4;
    return bpf_probe_read_kernel(p, 4, 0);
    }
    SEC("?syscall")
    __description("syscall: helper unaligned read ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_unaligned_fixed_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_unaligned_fixed_off_read(void *ctx)
    {
    char *p = ctx;
    p += 2;
    return bpf_strncmp(p, 4, ctx_strncmp_target);
    }
    SEC("?syscall")
    __description("syscall: helper unaligned write ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_unaligned_fixed_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_unaligned_fixed_off_write(void *ctx)
    {
    char *p = ctx;
    p += 2;
    return bpf_probe_read_kernel(p, 4, 0);
    }
    SEC("?syscall")
    __description("syscall: helper read ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_var_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_var_off_read(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xfc;
    p += off;
    return bpf_strncmp(p, 4, ctx_strncmp_target);
    }
    SEC("?syscall")
    __description("syscall: helper write ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_var_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_var_off_write(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xfc;
    p += off;
    return bpf_probe_read_kernel(p, 4, 0);
    }
    SEC("?syscall")
    __description("syscall: helper unaligned read ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_unaligned_var_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_unaligned_var_off_read(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xfc;
    off += 2;
    p += off;
    return bpf_strncmp(p, 4, ctx_strncmp_target);
    }
    SEC("?syscall")
    __description("syscall: helper unaligned write ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_unaligned_var_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_unaligned_var_off_write(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xfc;
    off += 2;
    p += off;
    return bpf_probe_read_kernel(p, 4, 0);
    }
    SEC("?syscall")
    __description("syscall: reject helper read ctx past U16_MAX with fixed offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "outside of the allowed memory) -> __failure {
    __failure __msg("outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_u16_max_fixed_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_u16_max_fixed_off_read(void *ctx)
    {
    char *p = ctx;
    p += 65535;
    return bpf_strncmp(p, 4, ctx_strncmp_target);
    }
    SEC("?syscall")
    __description("syscall: reject helper write ctx past U16_MAX with fixed offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "outside of the allowed memory) -> __failure {
    __failure __msg("outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_u16_max_fixed_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_u16_max_fixed_off_write(void *ctx)
    {
    char *p = ctx;
    p += 65535;
    return bpf_probe_read_kernel(p, 4, 0);
    }
    SEC("?syscall")
    __description("syscall: reject helper read ctx past U16_MAX with variable offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "outside of the allowed memory) -> __failure {
    __failure __msg("outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_u16_max_var_off_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_u16_max_var_off_read(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xffff;
    off += 1;
    p += off;
    return bpf_strncmp(p, 4, ctx_strncmp_target);
    }
    SEC("?syscall")
    __description("syscall: reject helper write ctx past U16_MAX with variable offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "outside of the allowed memory) -> __failure {
    __failure __msg("outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_u16_max_var_off_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_u16_max_var_off_write(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xffff;
    off += 1;
    p += off;
    return bpf_probe_read_kernel(p, 4, 0);
    }
    SEC("?syscall")
    __description("syscall: helper read zero-sized ctx access")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_zero_sized_read(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_zero_sized_read(void *ctx)
    {
    return bpf_snprintf(0, 0, ctx_snprintf_fmt, ctx, 0);
    }
    SEC("?syscall")
    __description("syscall: helper write zero-sized ctx access")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_helper_zero_sized_write(ctx: *mut c_void) -> c_int {
    int syscall_ctx_helper_zero_sized_write(void *ctx)
    {
    return bpf_probe_read_kernel(ctx, 0, 0);
    }
    SEC("?syscall")
    __description("syscall: kfunc access ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_kfunc_fixed_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_kfunc_fixed_off(void *ctx)
    {
    char *p = ctx;
    p += 4;
    bpf_kfunc_call_test_mem_len_pass1(p, 4);
    return 0;
    }
    SEC("?syscall")
    __description("syscall: kfunc access ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_kfunc_var_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_kfunc_var_off(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xfc;
    p += off;
    bpf_kfunc_call_test_mem_len_pass1(p, 4);
    return 0;
    }
    SEC("?syscall")
    __description("syscall: kfunc unaligned access ctx with fixed offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_kfunc_unaligned_fixed_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_kfunc_unaligned_fixed_off(void *ctx)
    {
    char *p = ctx;
    p += 2;
    bpf_kfunc_call_test_mem_len_pass1(p, 4);
    return 0;
    }
    SEC("?syscall")
    __description("syscall: kfunc unaligned access ctx with variable offset")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_kfunc_unaligned_var_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_kfunc_unaligned_var_off(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xfc;
    off += 2;
    p += off;
    bpf_kfunc_call_test_mem_len_pass1(p, 4);
    return 0;
    }
    SEC("?syscall")
    __description("syscall: reject kfunc ctx access past U16_MAX with fixed offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "outside of the allowed memory) -> __failure {
    __failure __msg("outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_kfunc_u16_max_fixed_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_kfunc_u16_max_fixed_off(void *ctx)
    {
    char *p = ctx;
    p += 65535;
    bpf_kfunc_call_test_mem_len_pass1(p, 4);
    return 0;
    }
    SEC("?syscall")
    __description("syscall: reject kfunc ctx access past U16_MAX with variable offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "outside of the allowed memory) -> __failure {
    __failure __msg("outside of the allowed memory range")
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_kfunc_u16_max_var_off(ctx: *mut c_void) -> c_int {
    int syscall_ctx_kfunc_u16_max_var_off(void *ctx)
    {
    let mut off: __u64 = bpf_get_prandom_u32();
    char *p = ctx;
    off &= 0xffff;
    off += 1;
    p += off;
    bpf_kfunc_call_test_mem_len_pass1(p, 4);
    return 0;
    }
    SEC("?syscall")
    __description("syscall: kfunc access zero-sized ctx")
    __success
#[no_mangle]
pub unsafe extern "C" fn syscall_ctx_kfunc_zero_sized(ctx: *mut c_void) -> c_int {
    int syscall_ctx_kfunc_zero_sized(void *ctx)
    {
    bpf_kfunc_call_test_mem_len_pass1(ctx, 0);
    return 0;
    }
//
// For non-syscall program types without convert_ctx_access, direct ctx
// dereference is still allowed after adding a fixed offset, while variable
// and negative direct accesses reject.
//
// Passing ctx as a helper or kfunc memory argument is only permitted for
// syscall programs, so the helper and kfunc cases below validate rejection
// for non-syscall ctx pointers at fixed, variable, and zero-sized accesses.
//

    SEC("?" type)							\
    __description(type ": read ctx at fixed offset")		\
    __success							\
    int no_rewrite_##name##_fixed(void *ctx)			\
    {								\
    char *p = ctx;						\
    volatile load_t val;					\
    \
    val = *(load_t *)(p + off);				\
    (void)val;						\
    return 0;						\
    }								\
    SEC("?" type)							\
    __description(type ": reject variable offset ctx access")	\
    __failure __msg("variable ctx access var_off=")			\
    int no_rewrite_##name##_var(void *ctx)			\
    {								\
    __u64 off_var = bpf_get_prandom_u32();			\
    char *p = ctx;						\
    \
    off_var &= 4;						\
    p += off_var;						\
    return *(load_t *)p;					\
    }								\
    SEC("?" type)							\
    __description(type ": reject negative offset ctx access")	\
    __failure __msg("invalid bpf_context access")			\
    int no_rewrite_##name##_neg(void *ctx)			\
    {								\
    char *p = ctx;						\
    \
    p -= 612;						\
    return *(load_t *)p;					\
    }								\
    SEC("?" type)							\
    __description(type ": reject helper read ctx at fixed offset")	\
    __failure __msg("dereference of modified ctx ptr")		\
    int no_rewrite_##name##_helper_read_fixed(void *ctx)		\
    {								\
    char *p = ctx;						\
    \
    p += off;						\
    return bpf_strncmp(p, 4, ctx_strncmp_target);		\
    }								\
    SEC("?" type)							\
    __description(type ": reject helper write ctx at fixed offset")	\
    __failure __msg("dereference of modified ctx ptr")		\
    int no_rewrite_##name##_helper_write_fixed(void *ctx)		\
    {								\
    char *p = ctx;						\
    \
    p += off;						\
    return bpf_probe_read_kernel(p, 4, 0);			\
    }								\
    SEC("?" type)							\
    __description(type ": reject helper read ctx with variable offset") \
    __failure __msg("variable ctx access var_off=")			\
    int no_rewrite_##name##_helper_read_var(void *ctx)		\
    {								\
    __u64 off_var = bpf_get_prandom_u32();			\
    char *p = ctx;						\
    \
    off_var &= 4;						\
    p += off_var;						\
    return bpf_strncmp(p, 4, ctx_strncmp_target);		\
    }								\
    SEC("?" type)							\
    __description(type ": reject helper write ctx with variable offset") \
    __failure __msg("variable ctx access var_off=")			\
    int no_rewrite_##name##_helper_write_var(void *ctx)		\
    {								\
    __u64 off_var = bpf_get_prandom_u32();			\
    char *p = ctx;						\
    \
    off_var &= 4;						\
    p += off_var;						\
    return bpf_probe_read_kernel(p, 4, 0);			\
    }								\
    SEC("?" type)							\
    __description(type ": reject helper read zero-sized ctx access") \
    __failure __msg("R4 type=ctx expected=fp")			\
    int no_rewrite_##name##_helper_read_zero(void *ctx)		\
    {								\
    return bpf_snprintf(0, 0, ctx_snprintf_fmt, ctx, 0);	\
    }								\
    SEC("?" type)							\
    __description(type ": reject helper write zero-sized ctx access") \
    __failure __msg("R1 type=ctx expected=fp")			\
    int no_rewrite_##name##_helper_write_zero(void *ctx)		\
    {								\
    return bpf_probe_read_kernel(ctx, 0, 0);			\
    }								\
    SEC("?" type)							\
    __description(type ": reject kfunc ctx at fixed offset")	\
    __failure __msg("dereference of modified ctx ptr")		\
    int no_rewrite_##name##_kfunc_fixed(void *ctx)		\
    {								\
    char *p = ctx;						\
    \
    p += off;						\
    bpf_kfunc_call_test_mem_len_pass1(p, 4);		\
    return 0;						\
    }								\
    SEC("?" type)							\
    __description(type ": reject kfunc ctx with variable offset")	\
    __failure __msg("variable ctx access var_off=")			\
    int no_rewrite_##name##_kfunc_var(void *ctx)			\
    {								\
    __u64 off_var = bpf_get_prandom_u32();			\
    char *p = ctx;						\
    \
    off_var &= 4;						\
    p += off_var;						\
    bpf_kfunc_call_test_mem_len_pass1(p, 4);		\
    return 0;						\
    }								\
    SEC("?" type)							\
    __description(type ": reject kfunc zero-sized ctx access")	\
    __failure __msg("R1 type=ctx expected=fp")			\
    int no_rewrite_##name##_kfunc_zero(void *ctx)			\
    {								\
    bpf_kfunc_call_test_mem_len_pass1(ctx, 0);		\
    return 0;						\
    }
    no_rewrite_ctx_access("kprobe", kprobe, 8, u64);
    no_rewrite_ctx_access("tracepoint", tp, 8, u64);
    no_rewrite_ctx_access("raw_tp", raw_tp, 8, u64);
    no_rewrite_ctx_access("raw_tracepoint.w", raw_tp_w, 8, u64);
    no_rewrite_ctx_access("fentry/bpf_modify_return_test", fentry, 8, u64);
    no_rewrite_ctx_access("cgroup/dev", cgroup_dev, 4, u32);
    no_rewrite_ctx_access("netfilter", netfilter, offsetof(struct bpf_nf_ctx, skb), u64);
    char _license[] SEC("license") = "GPL";
