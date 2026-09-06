//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_unpriv.c
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
// Converted from tools/testing/selftests/bpf/verifier/unpriv.c

    extern const int bpf_prog_active __ksym;

// struct bpf_sock_tuple tuple = {} */ \
    "r2 = 0;"			\
    "*(u32*)(r10 - 8) = r2;"	\
    "*(u64*)(r10 - 16) = r2;"	\
    "*(u64*)(r10 - 24) = r2;"	\
    "*(u64*)(r10 - 32) = r2;"	\
    "*(u64*)(r10 - 40) = r2;"	\
    "*(u64*)(r10 - 48) = r2;"	\
// sk = func(ctx, &tuple, sizeof tuple, 0, 0) */ \
    "r2 = r10;"			\
    "r2 += -48;"			\
    "r3 = %[sizeof_bpf_sock_tuple];"\
    "r4 = 0;"			\
    "r5 = 0;"			\
    "call %[" #func "];"
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, long long);
    } map_hash_8b SEC(".maps");
    void dummy_prog_42_socket(void);
    void dummy_prog_24_socket(void);
    void dummy_prog_loop1_socket(void);
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 4);
    __uint(key_size, sizeof(int));
    __array(values, void (void));
    } map_prog1_socket SEC(".maps") = {
    .values = {
    [0] = (void *)&dummy_prog_42_socket,
    [1] = (void *)&dummy_prog_loop1_socket,
    [2] = (void *)&dummy_prog_24_socket,
    },
    };
    SEC("socket")
    __auxiliary __auxiliary_unpriv
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_42_socket() -> __naked void {
    __naked void dummy_prog_42_socket(void)
    {
    asm volatile ("r0 = 42; exit;");
    }
    SEC("socket")
    __auxiliary __auxiliary_unpriv
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_24_socket() -> __naked void {
    __naked void dummy_prog_24_socket(void)
    {
    asm volatile ("r0 = 24; exit;");
    }
    SEC("socket")
    __auxiliary __auxiliary_unpriv
#[no_mangle]
pub unsafe extern "C" fn dummy_prog_loop1_socket() -> __naked void {
    __naked void dummy_prog_loop1_socket(void)
    {
    asm volatile ("			\
    r3 = 1;				\
    r2 = %[map_prog1_socket] ll;	\
    call %[bpf_tail_call];		\
    r0 = 41;			\
    exit;				\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: pseudo btf id log masks address")
    __success_unpriv
    __msg_unpriv("0: (18) r1 = 0x0")
    __not_msg_unpriv("0: (18) r1 = 0x{{[1-9a-f][0-9a-f]*}}")
    __retval_unpriv(0)
    __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn pseudo_btf_id_log_masks_address() -> __naked void {
    __naked void pseudo_btf_id_log_masks_address(void)
    {
    asm volatile ("r1 = %[bpf_prog_active] ll;"
    "r0 = 0;"
    "exit;"
    :
    : __imm_addr(bpf_prog_active)
    : __clobber_all);
    }
#[no_mangle]
unsafe extern "C" fn pseudo_func_callback(index: __u32, ctx: *mut c_void) -> c_int {
    static int pseudo_func_callback(__u32 index, void *ctx)
    {
    return 0;
    }
    SEC("socket")
    __description("unpriv: pseudo function policy diagnostic")
    __success __failure_unpriv
    __msg_unpriv("loading/calling other bpf or kernel functions")
    __not_msg_unpriv("BPF-to-BPF function call")
    __msg_unpriv("policy check failed for BPF function reference")
    __msg_unpriv("avoid BPF function references in unprivileged")
#[no_mangle]
pub unsafe extern "C" fn unpriv_pseudo_func_policy(ctx: *mut c_void) -> c_int {
    int unpriv_pseudo_func_policy(void *ctx)
    {
    bpf_loop(1, pseudo_func_callback, core::ptr::null_mut(), 0);
    return 0;
    }
    SEC("socket")
    __description("unpriv: return pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R0 leaks addr")
    __retval(POINTER_VALUE)
#[no_mangle]
pub unsafe extern "C" fn unpriv_return_pointer() -> __naked void {
    __naked void unpriv_return_pointer(void)
    {
    asm volatile ("					\
    r0 = r10;					\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: add const to pointer")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_add_const_to_pointer() -> __naked void {
    __naked void unpriv_add_const_to_pointer(void)
    {
    asm volatile ("					\
    r1 += 8;					\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: add pointer to pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R1 pointer +=) -> __failure {
    __failure __msg("R1 pointer += pointer")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn unpriv_add_pointer_to_pointer() -> __naked void {
    __naked void unpriv_add_pointer_to_pointer(void)
    {
    asm volatile ("					\
    r1 += r10;					\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: neg pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(arithmetic": "R1 pointer) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R1 pointer arithmetic")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_neg_pointer() -> __naked void {
    __naked void unpriv_neg_pointer(void)
    {
    asm volatile ("					\
    r1 = -r1;					\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: cmp pointer with const")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(comparison": "R1 pointer) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R1 pointer comparison")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_cmp_pointer_with_const() -> __naked void {
    __naked void unpriv_cmp_pointer_with_const(void)
    {
    asm volatile ("					\
    if r1 == 0 goto l0_%=;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: cmp pointer with pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(comparison": "R10 pointer) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R10 pointer comparison")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_cmp_pointer_with_pointer() -> __naked void {
    __naked void unpriv_cmp_pointer_with_pointer(void)
    {
    asm volatile ("					\
    if r1 == r10 goto l0_%=;			\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tracepoint")
    __description("unpriv: check that printk is disallowed")
    __success
#[no_mangle]
pub unsafe extern "C" fn check_that_printk_is_disallowed() -> __naked void {
    __naked void check_that_printk_is_disallowed(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r1 = r10;					\
    r1 += -8;					\
    r2 = 8;						\
    r3 = r1;					\
    call %[bpf_trace_printk];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_trace_printk)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: pass pointer to helper function")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R4 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R4 leaks addr")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn pass_pointer_to_helper_function() -> __naked void {
    __naked void pass_pointer_to_helper_function(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    r3 = r2;					\
    r4 = r2;					\
    call %[bpf_map_update_elem];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_update_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: indirectly pass pointer on stack to helper function")
    __success __failure_unpriv
    __msg_unpriv("invalid read from stack R2 off -8+0 size 8")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn on_stack_to_helper_function() -> __naked void {
    __naked void on_stack_to_helper_function(void)
    {
    asm volatile ("					\
// (u64*)(r10 - 8) = r10;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: mangle pointer on stack 1")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(spilled": "attempt to corrupt) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("attempt to corrupt spilled")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn mangle_pointer_on_stack_1() -> __naked void {
    __naked void mangle_pointer_on_stack_1(void)
    {
    asm volatile ("					\
// (u64*)(r10 - 8) = r10;				\
    r0 = 0;						\
// (u32*)(r10 - 8) = r0;				\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: mangle pointer on stack 2")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(spilled": "attempt to corrupt) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("attempt to corrupt spilled")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn mangle_pointer_on_stack_2() -> __naked void {
    __naked void mangle_pointer_on_stack_2(void)
    {
    asm volatile ("					\
// (u64*)(r10 - 8) = r10;				\
    r0 = 0;						\
// (u8*)(r10 - 1) = r0;				\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: read pointer from stack in small chunks")
#[no_mangle]
pub unsafe extern "C" fn __msg(size": "invalid) -> __failure {
    __failure __msg("invalid size")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn from_stack_in_small_chunks() -> __naked void {
    __naked void from_stack_in_small_chunks(void)
    {
    asm volatile ("					\
// (u64*)(r10 - 8) = r10;				\
    r0 = *(u32*)(r10 - 8);				\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: write pointer into ctx")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R1 leaks) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R1 leaks addr")
#[no_mangle]
pub unsafe extern "C" fn unpriv_write_pointer_into_ctx() -> __naked void {
    __naked void unpriv_write_pointer_into_ctx(void)
    {
    asm volatile ("					\
// (u64*)(r1 + 0) = r1;				\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: spill/fill of ctx")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_spill_fill_of_ctx() -> __naked void {
    __naked void unpriv_spill_fill_of_ctx(void)
    {
    asm volatile ("					\
    r6 = r10;					\
    r6 += -8;					\
// (u64*)(r6 + 0) = r1;				\
    r1 = *(u64*)(r6 + 0);				\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("tc")
    __description("unpriv: spill/fill of ctx 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn spill_fill_of_ctx_2() -> __naked void {
    __naked void spill_fill_of_ctx_2(void)
    {
    asm volatile ("					\
    r6 = r10;					\
    r6 += -8;					\
// (u64*)(r6 + 0) = r1;				\
    r1 = *(u64*)(r6 + 0);				\
    call %[bpf_get_hash_recalc];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_get_hash_recalc)
    : __clobber_all);
    }
    SEC("tc")
    __description("unpriv: spill/fill of ctx 3")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=ctx": "R1 type=fp) -> __failure {
    __failure __msg("R1 type=fp expected=ctx")
#[no_mangle]
pub unsafe extern "C" fn spill_fill_of_ctx_3() -> __naked void {
    __naked void spill_fill_of_ctx_3(void)
    {
    asm volatile ("					\
    r6 = r10;					\
    r6 += -8;					\
// (u64*)(r6 + 0) = r1;				\
// (u64*)(r6 + 0) = r10;				\
    r1 = *(u64*)(r6 + 0);				\
    call %[bpf_get_hash_recalc];			\
    exit;						\
    "	:
    : __imm(bpf_get_hash_recalc)
    : __clobber_all);
    }
    SEC("tc")
    __description("unpriv: spill/fill of ctx 4")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=ctx": "R1 type=scalar) -> __failure {
    __failure __msg("R1 type=scalar expected=ctx")
#[no_mangle]
pub unsafe extern "C" fn spill_fill_of_ctx_4() -> __naked void {
    __naked void spill_fill_of_ctx_4(void)
    {
    asm volatile ("					\
    r6 = r10;					\
    r6 += -8;					\
// (u64*)(r6 + 0) = r1;				\
    r0 = 1;						\
    lock *(u64 *)(r10 - 8) += r0;			\
    r1 = *(u64*)(r6 + 0);				\
    call %[bpf_get_hash_recalc];			\
    exit;						\
    "	:
    : __imm(bpf_get_hash_recalc)
    : __clobber_all);
    }
    SEC("tc")
    __description("unpriv: spill/fill of different pointers stx")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointers": "same insn cannot be used with different) -> __failure {
    __failure __msg("same insn cannot be used with different pointers")
#[no_mangle]
pub unsafe extern "C" fn fill_of_different_pointers_stx() -> __naked void {
    __naked void fill_of_different_pointers_stx(void)
    {
    asm volatile ("					\
    r3 = 42;					\
    r6 = r10;					\
    r6 += -8;					\
    if r1 == 0 goto l0_%=;				\
    r2 = r10;					\
    r2 += -16;					\
// (u64*)(r6 + 0) = r2;				\
    l0_%=:	if r1 != 0 goto l1_%=;				\
// (u64*)(r6 + 0) = r1;				\
    l1_%=:	r1 = *(u64*)(r6 + 0);				\
// (u32*)(r1 + %[__sk_buff_mark]) = r3;		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark))
    : __clobber_all);
    }
// Same as above, but use BPF_ST_MEM to save 42
// instead of BPF_STX_MEM.
//
    SEC("tc")
    __description("unpriv: spill/fill of different pointers st")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointers": "same insn cannot be used with different) -> __failure {
    __failure __msg("same insn cannot be used with different pointers")
#[no_mangle]
pub unsafe extern "C" fn fill_of_different_pointers_st() -> __naked void {
    __naked void fill_of_different_pointers_st(void)
    {
    asm volatile ("					\
    r6 = r10;					\
    r6 += -8;					\
    if r1 == 0 goto l0_%=;				\
    r2 = r10;					\
    r2 += -16;					\
// (u64*)(r6 + 0) = r2;				\
    l0_%=:	if r1 != 0 goto l1_%=;				\
// (u64*)(r6 + 0) = r1;				\
    l1_%=:	r1 = *(u64*)(r6 + 0);				\
    .8byte %[st_mem];				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark)),
    __imm_insn(st_mem,
    BPF_ST_MEM(BPF_W, BPF_REG_1, offsetof(struct __sk_buff, mark), 42))
    : __clobber_all);
    }
    SEC("tc")
    __description("unpriv: spill/fill of different pointers stx - ctx and sock")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=sock": "type=ctx) -> __failure {
    __failure __msg("type=ctx expected=sock")
#[no_mangle]
pub unsafe extern "C" fn pointers_stx_ctx_and_sock() -> __naked void {
    __naked void pointers_stx_ctx_and_sock(void)
    {
    asm volatile ("					\
    r8 = r1;					\
// struct bpf_sock *sock = bpf_sock_lookup(...); */\
    "	BPF_SK_LOOKUP(bpf_sk_lookup_tcp)
    "	r2 = r0;					\
// u64 foo; */					\
// void *target = &foo; */			\
    r6 = r10;					\
    r6 += -8;					\
    r1 = r8;					\
// if (skb == NULL) *target = sock; */		\
    if r1 == 0 goto l0_%=;				\
// (u64*)(r6 + 0) = r2;				\
    l0_%=:	/* else *target = skb; */			\
    if r1 != 0 goto l1_%=;				\
// (u64*)(r6 + 0) = r1;				\
    l1_%=:	/* struct __sk_buff *skb = *target; */		\
    r1 = *(u64*)(r6 + 0);				\
// skb->mark = 42; */				\
    r3 = 42;					\
// (u32*)(r1 + %[__sk_buff_mark]) = r3;		\
// if (sk) bpf_sk_release(sk) */		\
    if r1 == 0 goto l2_%=;				\
    call %[bpf_sk_release];				\
    l2_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_sk_lookup_tcp),
    __imm(bpf_sk_release),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark)),
    __imm_const(sizeof_bpf_sock_tuple, sizeof(struct bpf_sock_tuple))
    : __clobber_all);
    }
    SEC("tc")
    __description("unpriv: spill/fill of different pointers stx - leak sock")
    __failure
// .errstr = "same insn cannot be used with different pointers",
    __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn different_pointers_stx_leak_sock() -> __naked void {
    __naked void different_pointers_stx_leak_sock(void)
    {
    asm volatile ("					\
    r8 = r1;					\
// struct bpf_sock *sock = bpf_sock_lookup(...); */\
    "	BPF_SK_LOOKUP(bpf_sk_lookup_tcp)
    "	r2 = r0;					\
// u64 foo; */					\
// void *target = &foo; */			\
    r6 = r10;					\
    r6 += -8;					\
    r1 = r8;					\
// if (skb == NULL) *target = sock; */		\
    if r1 == 0 goto l0_%=;				\
// (u64*)(r6 + 0) = r2;				\
    l0_%=:	/* else *target = skb; */			\
    if r1 != 0 goto l1_%=;				\
// (u64*)(r6 + 0) = r1;				\
    l1_%=:	/* struct __sk_buff *skb = *target; */		\
    r1 = *(u64*)(r6 + 0);				\
// skb->mark = 42; */				\
    r3 = 42;					\
// (u32*)(r1 + %[__sk_buff_mark]) = r3;		\
    exit;						\
    "	:
    : __imm(bpf_sk_lookup_tcp),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark)),
    __imm_const(sizeof_bpf_sock_tuple, sizeof(struct bpf_sock_tuple))
    : __clobber_all);
    }
    SEC("tc")
    __description("unpriv: spill/fill of different pointers stx - sock and ctx (read)")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointers": "same insn cannot be used with different) -> __failure {
    __failure __msg("same insn cannot be used with different pointers")
#[no_mangle]
pub unsafe extern "C" fn stx_sock_and_ctx_read() -> __naked void {
    __naked void stx_sock_and_ctx_read(void)
    {
    asm volatile ("					\
    r8 = r1;					\
// struct bpf_sock *sock = bpf_sock_lookup(...); */\
    "	BPF_SK_LOOKUP(bpf_sk_lookup_tcp)
    "	r2 = r0;					\
// u64 foo; */					\
// void *target = &foo; */			\
    r6 = r10;					\
    r6 += -8;					\
    r1 = r8;					\
// if (skb) *target = skb */			\
    if r1 == 0 goto l0_%=;				\
// (u64*)(r6 + 0) = r1;				\
    l0_%=:	/* else *target = sock */			\
    if r1 != 0 goto l1_%=;				\
// (u64*)(r6 + 0) = r2;				\
    l1_%=:	/* struct bpf_sock *sk = *target; */		\
    r1 = *(u64*)(r6 + 0);				\
// if (sk) u32 foo = sk->mark; bpf_sk_release(sk); */\
    if r1 == 0 goto l2_%=;				\
    r3 = *(u32*)(r1 + %[bpf_sock_mark]);		\
    call %[bpf_sk_release];				\
    l2_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_sk_lookup_tcp),
    __imm(bpf_sk_release),
    __imm_const(bpf_sock_mark, offsetof(struct bpf_sock, mark)),
    __imm_const(sizeof_bpf_sock_tuple, sizeof(struct bpf_sock_tuple))
    : __clobber_all);
    }
    SEC("tc")
    __description("unpriv: spill/fill of different pointers stx - sock and ctx (write)")
    __failure
// .errstr = "same insn cannot be used with different pointers",
    __msg("cannot write into sock")
#[no_mangle]
pub unsafe extern "C" fn stx_sock_and_ctx_write() -> __naked void {
    __naked void stx_sock_and_ctx_write(void)
    {
    asm volatile ("					\
    r8 = r1;					\
// struct bpf_sock *sock = bpf_sock_lookup(...); */\
    "	BPF_SK_LOOKUP(bpf_sk_lookup_tcp)
    "	r2 = r0;					\
// u64 foo; */					\
// void *target = &foo; */			\
    r6 = r10;					\
    r6 += -8;					\
    r1 = r8;					\
// if (skb) *target = skb */			\
    if r1 == 0 goto l0_%=;				\
// (u64*)(r6 + 0) = r1;				\
    l0_%=:	/* else *target = sock */			\
    if r1 != 0 goto l1_%=;				\
// (u64*)(r6 + 0) = r2;				\
    l1_%=:	/* struct bpf_sock *sk = *target; */		\
    r1 = *(u64*)(r6 + 0);				\
// if (sk) sk->mark = 42; bpf_sk_release(sk); */\
    if r1 == 0 goto l2_%=;				\
    r3 = 42;					\
// (u32*)(r1 + %[bpf_sock_mark]) = r3;		\
    call %[bpf_sk_release];				\
    l2_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_sk_lookup_tcp),
    __imm(bpf_sk_release),
    __imm_const(bpf_sock_mark, offsetof(struct bpf_sock, mark)),
    __imm_const(sizeof_bpf_sock_tuple, sizeof(struct bpf_sock_tuple))
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: write pointer into map elem value")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R0 leaks addr")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn pointer_into_map_elem_value() -> __naked void {
    __naked void pointer_into_map_elem_value(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
// (u64*)(r0 + 0) = r0;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("alu32: mov u32 const")
    __success __success_unpriv
    __retval(0)

    __xlated_unpriv("if r0 == 0x0 goto pc+2")
    __xlated_unpriv("nospec") /* inserted to prevent `R7 invalid mem access 'scalar'` */
    __xlated_unpriv("goto pc-1") /* sanitized dead code */
    __xlated_unpriv("exit")

#[no_mangle]
pub unsafe extern "C" fn alu32_mov_u32_const() -> __naked void {
    __naked void alu32_mov_u32_const(void)
    {
    asm volatile ("					\
    w7 = 0;						\
    w7 ^= w7;					\
    w0 = w7;					\
    if r0 == 0 goto l0_%=;				\
    r0 = *(u64*)(r7 + 0);				\
    l0_%=:	exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: partial copy of pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(copy": "R10 partial) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R10 partial copy")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_partial_copy_of_pointer() -> __naked void {
    __naked void unpriv_partial_copy_of_pointer(void)
    {
    asm volatile ("					\
    w1 = w10;					\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: pass pointer to tail_call")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(helper": "R3 leaks addr into) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R3 leaks addr into helper")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn pass_pointer_to_tail_call() -> __naked void {
    __naked void pass_pointer_to_tail_call(void)
    {
    asm volatile ("					\
    r3 = r1;					\
    r2 = %[map_prog1_socket] ll;			\
    call %[bpf_tail_call];				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_tail_call),
    __imm_addr(map_prog1_socket)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: cmp map pointer with zero")
    __success __success_unpriv
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn cmp_map_pointer_with_zero() -> __naked void {
    __naked void cmp_map_pointer_with_zero(void)
    {
    asm volatile ("					\
    r1 = %[map_hash_8b] ll;				\
    if r1 == 0 goto l0_%=;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: cmp map pointer with const")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(prohibited": "R1 pointer comparison) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R1 pointer comparison prohibited")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn cmp_map_pointer_with_const() -> __naked void {
    __naked void cmp_map_pointer_with_const(void)
    {
    asm volatile ("					\
    r1 = %[map_hash_8b] ll;				\
    if r1 == 0x0000beef goto l0_%=;			\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: write into frame pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(only": "frame pointer is read) -> __failure {
    __failure __msg("frame pointer is read only")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn unpriv_write_into_frame_pointer() -> __naked void {
    __naked void unpriv_write_into_frame_pointer(void)
    {
    asm volatile ("					\
    r10 = r1;					\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: spill/fill frame pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(only": "frame pointer is read) -> __failure {
    __failure __msg("frame pointer is read only")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn unpriv_spill_fill_frame_pointer() -> __naked void {
    __naked void unpriv_spill_fill_frame_pointer(void)
    {
    asm volatile ("					\
    r6 = r10;					\
    r6 += -8;					\
// (u64*)(r6 + 0) = r10;				\
    r10 = *(u64*)(r6 + 0);				\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: cmp of frame pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(comparison": "R10 pointer) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R10 pointer comparison")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_cmp_of_frame_pointer() -> __naked void {
    __naked void unpriv_cmp_of_frame_pointer(void)
    {
    asm volatile ("					\
    if r10 == 0 goto l0_%=;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: adding of fp, reg")
    __success __failure_unpriv
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_adding_of_fp_reg() -> __naked void {
    __naked void unpriv_adding_of_fp_reg(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    r1 = 0;						\
    r1 += r10;					\
// (u64*)(r1 - 8) = r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: adding of fp, imm")
    __success __failure_unpriv
    __msg_unpriv("R1 stack pointer arithmetic goes out of range")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_adding_of_fp_imm() -> __naked void {
    __naked void unpriv_adding_of_fp_imm(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    r1 = r10;					\
    r1 += 0;					\
// (u64*)(r1 - 8) = r0;				\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: cmp of stack pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(comparison": "R2 pointer) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R2 pointer comparison")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn unpriv_cmp_of_stack_pointer() -> __naked void {
    __naked void unpriv_cmp_of_stack_pointer(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    if r2 == 0 goto l0_%=;				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: Spectre v1 path-based type confusion of scalar as stack-ptr")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)

    __xlated_unpriv("if r0 != 0x1 goto pc+2")
// This nospec prevents the exploit because it forces the mispredicted (not
// taken) `if r0 != 0x0 goto l0_%=` to resolve before using r6 as a pointer.
// This causes the CPU to realize that `r6 = r9` should have never executed. It
// ensures that r6 always contains a readable stack slot ptr when the insn after
// the nospec executes.
//
    __xlated_unpriv("nospec")
    __xlated_unpriv("r9 = *(u8 *)(r6 +0)")

#[no_mangle]
pub unsafe extern "C" fn unpriv_spec_v1_type_confusion() -> __naked void {
    __naked void unpriv_spec_v1_type_confusion(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l2_%=;				\
// r0: pointer to a map array entry */		\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
// r1, r2: prepared call args */		\
    r6 = r10;					\
    r6 += -8;					\
// r6: pointer to readable stack slot */	\
    r9 = 0xffffc900;				\
    r9 <<= 32;					\
// r9: scalar controlled by attacker */		\
    r0 = *(u64 *)(r0 + 0); /* cache miss */		\
    if r0 != 0x0 goto l0_%=;			\
    r6 = r9;					\
    l0_%=:	if r0 != 0x1 goto l1_%=;			\
    r9 = *(u8 *)(r6 + 0);				\
    l1_%=:  /* leak r9 */					\
    r9 &= 1;					\
    r9 <<= 9;					\
// (u64*)(r10 - 8) = r9;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l2_%=;				\
// leak secret into is_cached(map[0|512]): */	\
    r0 = *(u64 *)(r0 + 0);				\
    l2_%=:							\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: ldimm64 before Spectre v4 barrier")
    __success __success_unpriv
    __retval(0)

    __xlated_unpriv("r1 = 0x2020200005642020") /* should not matter */
    __xlated_unpriv("*(u64 *)(r10 -8) = r1")
    __xlated_unpriv("nospec")

#[no_mangle]
pub unsafe extern "C" fn unpriv_ldimm64_spectre_v4() -> __naked void {
    __naked void unpriv_ldimm64_spectre_v4(void)
    {
    asm volatile ("					\
    r1 = 0x2020200005642020 ll;			\
// (u64 *)(r10 -8) = r1;				\
    r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: Spectre v1 and v4 barrier")
    __success __success_unpriv
    __retval(0)

// starts with r0 == r8 == r9 == 0
    __xlated_unpriv("if r8 != 0x0 goto pc+1")
    __xlated_unpriv("goto pc+2")
    __xlated_unpriv("if r9 == 0x0 goto pc+4")
    __xlated_unpriv("r2 = r0")
// Following nospec required to prevent following dangerous `*(u64 *)(NOT_FP -64)
// = r1` iff `if r9 == 0 goto pc+4` was mispredicted because of Spectre v1. The
// test therefore ensures the Spectre-v4--induced nospec does not prevent the
// Spectre-v1--induced speculative path from being fully analyzed.
//
    __xlated_unpriv("nospec") /* Spectre v1 */
    __xlated_unpriv("*(u64 *)(r2 -64) = r1") /* could be used to leak r2 */
    __xlated_unpriv("nospec") /* Spectre v4 */

#[no_mangle]
pub unsafe extern "C" fn unpriv_spectre_v1_and_v4() -> __naked void {
    __naked void unpriv_spectre_v1_and_v4(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    r8 = r0;					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    r9 = r0;					\
    r0 = r10;					\
    r1 = 0;						\
    r2 = r10;					\
    if r8 != 0 goto l0_%=;				\
    if r9 != 0 goto l0_%=;				\
    r0 = 0;						\
    l0_%=:	if r8 != 0 goto l1_%=;				\
    goto l2_%=;					\
    l1_%=:	if r9 == 0 goto l3_%=;				\
    r2 = r0;					\
    l2_%=:	*(u64 *)(r2 -64) = r1;				\
    l3_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: Spectre v1 and v4 barrier (simple)")
    __success __success_unpriv
    __retval(0)

    __xlated_unpriv("if r8 != 0x0 goto pc+1")
    __xlated_unpriv("goto pc+2")
    __xlated_unpriv("goto pc-1") /* if r9 == 0 goto l3_%= */
    __xlated_unpriv("goto pc-1") /* r2 = r0 */
    __xlated_unpriv("nospec")
    __xlated_unpriv("*(u64 *)(r2 -64) = r1")
    __xlated_unpriv("nospec")

#[no_mangle]
pub unsafe extern "C" fn unpriv_spectre_v1_and_v4_simple() -> __naked void {
    __naked void unpriv_spectre_v1_and_v4_simple(void)
    {
    asm volatile ("					\
    r8 = 0;						\
    r8 ^= r8;					\
    r9 = 0;						\
    r9 ^= r9;					\
    r0 = r10;					\
    r1 = 0;						\
    r2 = r10;					\
    if r8 != 0 goto l0_%=;				\
    if r9 != 0 goto l0_%=;				\
    r0 = 0;						\
    l0_%=:	if r8 != 0 goto l1_%=;				\
    goto l2_%=;					\
    l1_%=:	if r9 == 0 goto l3_%=;				\
    r2 = r0;					\
    l2_%=:	*(u64 *)(r2 -64) = r1;				\
    l3_%=:	r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: ldimm64 before Spectre v1 and v4 barrier (simple)")
    __success __success_unpriv
    __retval(0)

    __xlated_unpriv("if r8 != 0x0 goto pc+1")
    __xlated_unpriv("goto pc+4")
    __xlated_unpriv("goto pc-1") /* if r9 == 0 goto l3_%= */
    __xlated_unpriv("goto pc-1") /* r2 = r0 */
    __xlated_unpriv("goto pc-1") /* r1 = 0x2020200005642020 ll */
    __xlated_unpriv("goto pc-1") /* second part of ldimm64 */
    __xlated_unpriv("nospec")
    __xlated_unpriv("*(u64 *)(r2 -64) = r1")
    __xlated_unpriv("nospec")

#[no_mangle]
pub unsafe extern "C" fn unpriv_ldimm64_spectre_v1_and_v4_simple() -> __naked void {
    __naked void unpriv_ldimm64_spectre_v1_and_v4_simple(void)
    {
    asm volatile ("					\
    r8 = 0;						\
    r8 ^= r8;					\
    r9 = 0;						\
    r9 ^= r9;					\
    r0 = r10;					\
    r1 = 0;						\
    r2 = r10;					\
    if r8 != 0 goto l0_%=;				\
    if r9 != 0 goto l0_%=;				\
    r0 = 0;						\
    l0_%=:	if r8 != 0 goto l1_%=;				\
    goto l2_%=;					\
    l1_%=:	if r9 == 0 goto l3_%=;				\
    r2 = r0;					\
    r1 = 0x2020200005642020 ll;			\
    l2_%=:	*(u64 *)(r2 -64) = r1;				\
    l3_%=:	r0 = 0;						\
    exit;						\
    "	::: __clobber_all);
    }
    SEC("socket")
    __description("unpriv: nospec after dead stack write in helper")
    __success __success_unpriv
    __retval(0)
// Dead code sanitizer rewrites the call to `goto -1`.
#[no_mangle]
pub unsafe extern "C" fn unpriv_dead_helper_stack_write_nospec_result() -> __naked void {
    __naked void unpriv_dead_helper_stack_write_nospec_result(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    if r0 != 1 goto l0_%=;				\
    r2 = 0;						\
    r3 = r10;					\
    r3 += -16;					\
    r4 = 4;						\
    r5 = 0;						\
    call %[bpf_skb_load_bytes_relative];		\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_skb_load_bytes_relative)
    : __clobber_all);
    }
    SEC("socket")
    __description("unpriv: Spectre v4 stack write slot index")
    __success __success_unpriv
    __retval(0)

    __xlated_unpriv("r0 = 0")
    __xlated_unpriv("*(u32 *)(r10 -4) = r0")
    __xlated_unpriv("nospec")
    __xlated_unpriv("*(u32 *)(r10 -8) = r0")
    __xlated_unpriv("nospec")
    __xlated_unpriv("exit")

#[no_mangle]
pub unsafe extern "C" fn stack_write_nospec_slot_index() -> __naked void {
    __naked void stack_write_nospec_slot_index(void)
    {
    asm volatile ("					\
    r0 = 0;					\
// (u32 *)(r10 - 4) = r0;			\
// (u32 *)(r10 - 8) = r0;			\
    exit;					\
    "	::: __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
