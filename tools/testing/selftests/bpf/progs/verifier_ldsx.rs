//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_ldsx.c
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

    (defined(__TARGET_ARCH_riscv) && __riscv_xlen == 64) || \
    defined(__TARGET_ARCH_arm) || defined(__TARGET_ARCH_s390) || \
    defined(__TARGET_ARCH_loongarch)) && \
    __clang_major__ >= 18
    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, 1);
    } arena SEC(".maps");
    SEC("socket")
    __description("LDSX, S8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -2) -> __success __success_unpriv {
    __success __success_unpriv __retval(-2)
#[no_mangle]
pub unsafe extern "C" fn ldsx_s8() -> __naked void {
    __naked void ldsx_s8(void)
    {
    asm volatile (
    "r1 = 0x3fe;"
    "*(u64 *)(r10 - 8) = r1;"

    "r0 = *(s8 *)(r10 - 8);"

    "r0 = *(s8 *)(r10 - 1);"

    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("LDSX, S16")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -2) -> __success __success_unpriv {
    __success __success_unpriv __retval(-2)
#[no_mangle]
pub unsafe extern "C" fn ldsx_s16() -> __naked void {
    __naked void ldsx_s16(void)
    {
    asm volatile (
    "r1 = 0x3fffe;"
    "*(u64 *)(r10 - 8) = r1;"

    "r0 = *(s16 *)(r10 - 8);"

    "r0 = *(s16 *)(r10 - 2);"

    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("LDSX, S32")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success __success_unpriv {
    __success __success_unpriv __retval(-1)
#[no_mangle]
pub unsafe extern "C" fn ldsx_s32() -> __naked void {
    __naked void ldsx_s32(void)
    {
    asm volatile (
    "r1 = 0xfffffffe;"
    "*(u64 *)(r10 - 8) = r1;"

    "r0 = *(s32 *)(r10 - 8);"

    "r0 = *(s32 *)(r10 - 4);"

    "r0 >>= 1;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("LDSX, S8 range checking, privileged")
    __log_level(2) __success __retval(1)
    __msg("R1=scalar(smin=smin32=-128,smax=smax32=127)")
#[no_mangle]
pub unsafe extern "C" fn ldsx_s8_range_priv() -> __naked void {
    __naked void ldsx_s8_range_priv(void)
    {
    asm volatile (
    "call %[bpf_get_prandom_u32];"
    "*(u64 *)(r10 - 8) = r0;"

    "r1 = *(s8 *)(r10 - 8);"

    "r1 = *(s8 *)(r10 - 1);"

// r1 with s8 range
    "if r1 s> 0x7f goto l0_%=;"
    "if r1 s< -0x80 goto l0_%=;"
    "r0 = 1;"
    "l1_%=:"
    "exit;"
    "l0_%=:"
    "r0 = 2;"
    "goto l1_%=;"
    :
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("LDSX, S16 range checking")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn ldsx_s16_range() -> __naked void {
    __naked void ldsx_s16_range(void)
    {
    asm volatile (
    "call %[bpf_get_prandom_u32];"
    "*(u64 *)(r10 - 8) = r0;"

    "r1 = *(s16 *)(r10 - 8);"

    "r1 = *(s16 *)(r10 - 2);"

// r1 with s16 range
    "if r1 s> 0x7fff goto l0_%=;"
    "if r1 s< -0x8000 goto l0_%=;"
    "r0 = 1;"
    "l1_%=:"
    "exit;"
    "l0_%=:"
    "r0 = 2;"
    "goto l1_%=;"
    :
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("socket")
    __description("LDSX, S32 range checking")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn ldsx_s32_range() -> __naked void {
    __naked void ldsx_s32_range(void)
    {
    asm volatile (
    "call %[bpf_get_prandom_u32];"
    "*(u64 *)(r10 - 8) = r0;"

    "r1 = *(s32 *)(r10 - 8);"

    "r1 = *(s32 *)(r10 - 4);"

// r1 with s16 range
    "if r1 s> 0x7fffFFFF goto l0_%=;"
    "if r1 s< -0x80000000 goto l0_%=;"
    "r0 = 1;"
    "l1_%=:"
    "exit;"
    "l0_%=:"
    "r0 = 2;"
    "goto l1_%=;"
    :
    : __imm(bpf_get_prandom_u32)
    : __clobber_all);
    }
    SEC("xdp")
    __description("LDSX, xdp s32 xdp_md.data")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn ldsx_ctx_1() -> __naked void {
    __naked void ldsx_ctx_1(void)
    {
    asm volatile (
    "r2 = *(s32 *)(r1 + %[xdp_md_data]);"
    "r0 = 0;"
    "exit;"
    :
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data))
    : __clobber_all);
    }
    SEC("xdp")
    __description("LDSX, xdp s32 xdp_md.data_end")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn ldsx_ctx_2() -> __naked void {
    __naked void ldsx_ctx_2(void)
    {
    asm volatile (
    "r2 = *(s32 *)(r1 + %[xdp_md_data_end]);"
    "r0 = 0;"
    "exit;"
    :
    : __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("LDSX, xdp s32 xdp_md.data_meta")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn ldsx_ctx_3() -> __naked void {
    __naked void ldsx_ctx_3(void)
    {
    asm volatile (
    "r2 = *(s32 *)(r1 + %[xdp_md_data_meta]);"
    "r0 = 0;"
    "exit;"
    :
    : __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("tcx/ingress")
    __description("LDSX, tcx s32 __sk_buff.data")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn ldsx_ctx_4() -> __naked void {
    __naked void ldsx_ctx_4(void)
    {
    asm volatile (
    "r2 = *(s32 *)(r1 + %[sk_buff_data]);"
    "r0 = 0;"
    "exit;"
    :
    : __imm_const(sk_buff_data, offsetof(struct __sk_buff, data))
    : __clobber_all);
    }
    SEC("tcx/ingress")
    __description("LDSX, tcx s32 __sk_buff.data_end")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn ldsx_ctx_5() -> __naked void {
    __naked void ldsx_ctx_5(void)
    {
    asm volatile (
    "r2 = *(s32 *)(r1 + %[sk_buff_data_end]);"
    "r0 = 0;"
    "exit;"
    :
    : __imm_const(sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("tcx/ingress")
    __description("LDSX, tcx s32 __sk_buff.data_meta")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn ldsx_ctx_6() -> __naked void {
    __naked void ldsx_ctx_6(void)
    {
    asm volatile (
    "r2 = *(s32 *)(r1 + %[sk_buff_data_meta]);"
    "r0 = 0;"
    "exit;"
    :
    : __imm_const(sk_buff_data_meta, offsetof(struct __sk_buff, data_meta))
    : __clobber_all);
    }
    SEC("flow_dissector")
    __description("LDSX, flow_dissector s32 __sk_buff.data")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn ldsx_ctx_7() -> __naked void {
    __naked void ldsx_ctx_7(void)
    {
    asm volatile (
    "r2 = *(s32 *)(r1 + %[sk_buff_data]);"
    "r0 = 0;"
    "exit;"
    :
    : __imm_const(sk_buff_data, offsetof(struct __sk_buff, data))
    : __clobber_all);
    }
    SEC("flow_dissector")
    __description("LDSX, flow_dissector s32 __sk_buff.data_end")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
#[no_mangle]
pub unsafe extern "C" fn ldsx_ctx_8() -> __naked void {
    __naked void ldsx_ctx_8(void)
    {
    asm volatile (
    "r2 = *(s32 *)(r1 + %[sk_buff_data_end]);"
    "r0 = 0;"
    "exit;"
    :
    : __imm_const(sk_buff_data_end, offsetof(struct __sk_buff, data_end))
    : __clobber_all);
    }
    SEC("syscall")
    __description("Arena LDSX Disasm")
    __success
    __arch_x86_64
    __jited("movslq	0x10(%rax,%r12), %r14")
    __jited("movswq	0x18(%rax,%r12), %r14")
    __jited("movsbq	0x20(%rax,%r12), %r14")
    __jited("movslq	0x10(%rdi,%r12), %r15")
    __jited("movswq	0x18(%rdi,%r12), %r15")
    __jited("movsbq	0x20(%rdi,%r12), %r15")
    __arch_arm64
    __jited("add	x11, x8, x28")
    __jited("ldrsw	x21, [x11, #0x10]")
    __jited("add	x11, x8, x28")
    __jited("ldrsh	x21, [x11, #0x18]")
    __jited("add	x11, x8, x28")
    __jited("ldrsb	x21, [x11, #0x20]")
    __jited("add	x11, x0, x28")
    __jited("ldrsw	x22, [x11, #0x10]")
    __jited("add	x11, x0, x28")
    __jited("ldrsh	x22, [x11, #0x18]")
    __jited("add	x11, x0, x28")
    __jited("ldrsb	x22, [x11, #0x20]")
#[no_mangle]
pub unsafe extern "C" fn arena_ldsx_disasm(ctx: *mut c_void) -> __naked void {
    __naked void arena_ldsx_disasm(void *ctx)
    {
    asm volatile (
    "r1 = %[arena] ll;"
    "r2 = 0;"
    "r3 = 1;"
    "r4 = %[numa_no_node];"
    "r5 = 0;"
    "call %[bpf_arena_alloc_pages];"
    "r0 = addr_space_cast(r0, 0x0, 0x1);"
    "r1 = r0;"
    "r8 = *(s32 *)(r0 + 16);"
    "r8 = *(s16 *)(r0 + 24);"
    "r8 = *(s8  *)(r0 + 32);"
    "r9 = *(s32 *)(r1 + 16);"
    "r9 = *(s16 *)(r1 + 24);"
    "r9 = *(s8  *)(r1 + 32);"
    "r0 = 0;"
    "exit;"
    :: __imm(bpf_arena_alloc_pages),
    __imm_addr(arena),
    __imm_const(numa_no_node, NUMA_NO_NODE)
    :  __clobber_all
    );
    }
    SEC("syscall")
    __description("Arena LDSX Exception")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn arena_ldsx_exception(ctx: *mut c_void) -> __naked void {
    __naked void arena_ldsx_exception(void *ctx)
    {
    asm volatile (
    "r1 = %[arena] ll;"
    "r0 = 0xdeadbeef;"
    "r0 = addr_space_cast(r0, 0x0, 0x1);"
    "r1 = 0x3fe;"
    "*(u64 *)(r0 + 0) = r1;"
    "r0 = *(s8 *)(r0 + 0);"
    "exit;"
    :
    :  __imm_addr(arena)
    :  __clobber_all
    );
    }
    SEC("syscall")
    __description("Arena LDSX, S8")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success {
    __success __retval(-1)
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn arena_ldsx_s8(ctx: *mut c_void) -> __naked void {
    __naked void arena_ldsx_s8(void *ctx)
    {
    asm volatile (
    "r1 = %[arena] ll;"
    "r2 = 0;"
    "r3 = 1;"
    "r4 = %[numa_no_node];"
    "r5 = 0;"
    "call %[bpf_arena_alloc_pages];"
    "r0 = addr_space_cast(r0, 0x0, 0x1);"
    "r1 = 0x3fe;"
    "*(u64 *)(r0 + 0) = r1;"

    "r0 = *(s8 *)(r0 + 0);"

    "r0 = *(s8 *)(r0 + 7);"

    "r0 >>= 1;"
    "exit;"
    :: __imm(bpf_arena_alloc_pages),
    __imm_addr(arena),
    __imm_const(numa_no_node, NUMA_NO_NODE)
    :  __clobber_all
    );
    }
    SEC("syscall")
    __description("Arena LDSX, S16")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success {
    __success __retval(-1)
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn arena_ldsx_s16(ctx: *mut c_void) -> __naked void {
    __naked void arena_ldsx_s16(void *ctx)
    {
    asm volatile (
    "r1 = %[arena] ll;"
    "r2 = 0;"
    "r3 = 1;"
    "r4 = %[numa_no_node];"
    "r5 = 0;"
    "call %[bpf_arena_alloc_pages];"
    "r0 = addr_space_cast(r0, 0x0, 0x1);"
    "r1 = 0x3fffe;"
    "*(u64 *)(r0 + 0) = r1;"

    "r0 = *(s16 *)(r0 + 0);"

    "r0 = *(s16 *)(r0 + 6);"

    "r0 >>= 1;"
    "exit;"
    :: __imm(bpf_arena_alloc_pages),
    __imm_addr(arena),
    __imm_const(numa_no_node, NUMA_NO_NODE)
    :  __clobber_all
    );
    }
    SEC("syscall")
    __description("Arena LDSX, S32")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: -1) -> __success {
    __success __retval(-1)
    __arch_x86_64
    __arch_arm64
#[no_mangle]
pub unsafe extern "C" fn arena_ldsx_s32(ctx: *mut c_void) -> __naked void {
    __naked void arena_ldsx_s32(void *ctx)
    {
    asm volatile (
    "r1 = %[arena] ll;"
    "r2 = 0;"
    "r3 = 1;"
    "r4 = %[numa_no_node];"
    "r5 = 0;"
    "call %[bpf_arena_alloc_pages];"
    "r0 = addr_space_cast(r0, 0x0, 0x1);"
    "r1 = 0xfffffffe;"
    "*(u64 *)(r0 + 0) = r1;"

    "r0 = *(s32 *)(r0 + 0);"

    "r0 = *(s32 *)(r0 + 4);"

    "r0 >>= 1;"
    "exit;"
    :: __imm(bpf_arena_alloc_pages),
    __imm_addr(arena),
    __imm_const(numa_no_node, NUMA_NO_NODE)
    :  __clobber_all
    );
    }
// to retain debug info for BTF generation
#[no_mangle]
pub unsafe extern "C" fn kfunc_root() {
    void kfunc_root(void)
    {
    bpf_arena_alloc_pages(0, 0, 0, 0, 0);
    }

    SEC("socket")
    __description("cpuv4 is not supported by compiler or jit, use a dummy test")
    __success
#[no_mangle]
pub unsafe extern "C" fn dummy_test() -> c_int {
    int dummy_test(void)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
