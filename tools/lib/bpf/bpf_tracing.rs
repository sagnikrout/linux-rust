//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/bpf_tracing.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

// Scan the ARCH passed in from ARCH env variable (see Makefile)

// Macro flag: #define bpf_target_x86
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_s390
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_arm
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_arm64
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_mips
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_powerpc
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_sparc
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_riscv
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_arc
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_loongarch
// Macro flag: #define bpf_target_defined

// Fall back to what the compiler says

// Macro flag: #define bpf_target_x86
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_s390
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_arm
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_arm64
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_mips
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_powerpc
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_sparc
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_riscv
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_arc
// Macro flag: #define bpf_target_defined

// Macro flag: #define bpf_target_loongarch
// Macro flag: #define bpf_target_defined

//
// https://en.wikipedia.org/wiki/X86_calling_conventions#System_V_AMD64_ABI
//

//
// Syscall uses r10 for PARM4. See arch/x86/entry/entry_64.S:entry_SYSCALL_64
// comments in Linux sources. And refer to syscall(2) manpage.
//

// i386 kernel is built with -mregparm=3

// i386 syscall ABI is very different, refer to syscall(2) manpage

//
// https://github.com/IBM/s390x-abi/releases/download/v1.6/lzsabi_s390x.pdf
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs___s390 {
    pub orig_gpr2: c_ulong,
    pub __attribute__((preserve_access_index)): },
// s390 provides user_pt_regs instead of struct pt_regs to userspace

//
// https://github.com/ARM-software/abi-aa/blob/main/aapcs32/aapcs32.rst#machine-registers
//

//
// https://github.com/ARM-software/abi-aa/blob/main/aapcs64/aapcs64.rst#machine-registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs___arm64 {
    pub orig_x0: c_ulong,
    pub __attribute__((preserve_access_index)): },
// arm64 provides struct user_pt_regs instead of struct pt_regs to userspace

//
// N64 ABI is assumed right now.
// https://en.wikipedia.org/wiki/MIPS_architecture#Calling_conventions
//

//
// http://refspecs.linux-foundation.org/elf/elfspec_ppc.pdf (page 3-14,
// section "Function Calling Sequence")
//

// powerpc does not select ARCH_HAS_SYSCALL_WRAPPER.

//
// https://en.wikipedia.org/wiki/Calling_convention#SPARC
//

// Should this also be a bpf_target check for the sparc case?

//
// https://github.com/riscv-non-isa/riscv-elf-psabi-doc/blob/master/riscv-cc.adoc#risc-v-calling-conventions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_regs___riscv {
    pub orig_a0: c_ulong,
    pub __attribute__((preserve_access_index)): },
// riscv provides struct user_regs_struct instead of struct pt_regs to userspace

//
// Section "Function Calling Sequence" (page 24):
// https://raw.githubusercontent.com/wiki/foss-for-synopsys-dwc-arc-processors/toolchain/files/ARCv2_ABI.pdf
//
// arc provides struct user_regs_struct instead of struct pt_regs to userspace

// arc does not select ARCH_HAS_SYSCALL_WRAPPER.

//
// https://docs.kernel.org/loongarch/introduction.html
// https://loongson.github.io/LoongArch-Documentation/LoongArch-ELF-ABI-EN.html
//
// loongarch provides struct user_pt_regs instead of struct pt_regs to userspace

// loongarch does not select ARCH_HAS_SYSCALL_WRAPPER.

    pub pt_regs: struct,
// allow some architectures to override `struct pt_regs`

//
// Different architectures support different number of arguments passed
// through registers. i386 supports just 3, some arches support up to 8.
//

//
// Similarly, syscall-specific conventions might differ between function call
// conventions within each architecture. All supported architectures pass
// either 6 or 7 syscall arguments in registers.
//
// See syscall(2) manpage for succinct table with information on each arch.
//

    pub }): *mut *mut ({ bpf_probe_read_kernel(&(ip), sizeof(ip), (void )PT_REGS_RET(ctx));,

    pub }): *mut *mut ({ bpf_probe_read_kernel(&(ip), sizeof(ip), (void )(PT_REGS_FP(ctx) + sizeof(ip)));,

//
// When invoked from a syscall handler kprobe, returns a pointer to a
// struct pt_regs containing syscall arguments and suitable for passing to
// PT_REGS_PARMn_SYSCALL() and PT_REGS_PARMn_CORE_SYSCALL().
//

// By default, assume that the arch selects ARCH_HAS_SYSCALL_WRAPPER.

//
// BPF_PROG is a convenience wrapper for generic tp_btf/fentry/fexit and
// similar kinds of BPF programs, that accept input arguments as a single
// pointer to untyped u64 array, where each u64 can actually be a typed
// pointer or integer of different size. Instead of requiring user to write
// manual casts and work with array elements by index, BPF_PROG macro
// allows user to declare a list of named and typed input arguments in the
// same syntax as for normal C function. All the casting is hidden and
// performed transparently, while user code can just assume working with
// function arguments of specified type and name.
//
// Original raw context argument is preserved as well as 'ctx' argument.
// This is useful when using BPF helpers that expect original context
// as one of the parameters (e.g., for bpf_perf_event_output()).
//

    pub \: *mut *mut name(unsigned long long ctx);,
    pub \: *mut *mut ____##name(unsigned long long ctx, ##args);,
    pub \: return ____##name(___bpf_ctx_cast(args));,

    pub \: __builtin_choose_expr(sizeof(t) == 1, ({ union { __u8 z[1]; t x; } ___t = { .z = {ctx[n]}}; ___t.x; }),,
    pub \: __builtin_choose_expr(sizeof(t) == 2, ({ union { __u16 z[1]; t x; } ___t = { .z = {ctx[n]} }; ___t.x; }),,
    pub \: __builtin_choose_expr(sizeof(t) == 4, ({ union { __u32 z[1]; t x; } ___t = { .z = {ctx[n]} }; ___t.x; }),,
    pub \: __builtin_choose_expr(sizeof(t) == 8, ({ union { __u64 z[1]; t x; } ___t = {.z = {ctx[n]} }; ___t.x; }),,
    pub \: __builtin_choose_expr(sizeof(t) == 16, ({ union { __u64 z[2]; t x; } ___t = {.z = {ctx[n], ctx[n + 1]} }; ___t.x; }),,

// Macro flag: #define ___bpf_ctx_decl0()

//
// BPF_PROG2 is an enhanced version of BPF_PROG in order to handle struct
// arguments. Since each struct argument might take one or two u64 values
// in the trampoline stack, argument type size is needed to place proper number
// of u64 values for each argument. Therefore, BPF_PROG2 has different
// syntax from BPF_PROG. For example, for the following BPF_PROG syntax:
//
// int BPF_PROG(test2, int a, int b) { ... }
//
// the corresponding BPF_PROG2 syntax is:
//
// int BPF_PROG2(test2, int, a, int, b) { ... }
//
// where type and the corresponding argument name are separated by comma.
//
// Use BPF_PROG2 macro if one of the arguments might be a struct/union larger
// than 8 bytes:
//
// int BPF_PROG2(test_struct_arg, struct bpf_testmod_struct_arg_1, a, int, b,
// int, c, int, d, struct bpf_testmod_struct_arg_2, e, int, ret)
// {
// // access a, b, c, d, e, and ret directly
// ...
// }
//

    pub \: *mut *mut name(unsigned long long ctx);,
    pub \: *mut *mut ____##name(unsigned long long ctx ___bpf_ctx_decl(args));,
    pub \: return ____##name(ctx ___bpf_ctx_arg(args));,
    pub pt_regs: struct,

//
// BPF_KPROBE serves the same purpose for kprobes as BPF_PROG for
// tp_btf/fentry/fexit BPF programs. It hides the underlying platform-specific
// low-level way of getting kprobe input arguments from struct pt_regs, and
// provides a familiar typed and named function arguments syntax and
// semantics of accessing kprobe input parameters.
//
// Original struct pt_regs* context is preserved as 'ctx' argument. This might
// be necessary when using BPF helpers like bpf_perf_event_output().
//

    pub \: *mut *mut name(struct pt_regs ctx);,
    pub \: *mut *mut ____##name(struct pt_regs ctx, ##args);,
    pub \: return ____##name(___bpf_kprobe_args(args));,

//
// BPF_KRETPROBE is similar to BPF_KPROBE, except, it only provides optional
// return value (in addition to `struct pt_regs *ctx`), but no input
// arguments, because they will be clobbered by the time probed function
// returns.
//

    pub \: *mut *mut name(struct pt_regs ctx);,
    pub \: *mut *mut ____##name(struct pt_regs ctx, ##args);,
    pub \: return ____##name(___bpf_kretprobe_args(args));,
// If kernel has CONFIG_ARCH_HAS_SYSCALL_WRAPPER, read pt_regs directly

// If kernel doesn't have CONFIG_ARCH_HAS_SYSCALL_WRAPPER, we have to BPF_CORE_READ from pt_regs

//
// BPF_KSYSCALL is a variant of BPF_KPROBE, which is intended for
// tracing syscall functions, like __x64_sys_close. It hides the underlying
// platform-specific low-level way of getting syscall input arguments from
// struct pt_regs, and provides a familiar typed and named function arguments
// syntax and semantics of accessing syscall input parameters.
//
// Original struct pt_regs * context is preserved as 'ctx' argument. This might
// be necessary when using BPF helpers like bpf_perf_event_output().
//
// At the moment BPF_KSYSCALL does not transparently handle all the calling
// convention quirks for the following syscalls:
//
// - mmap(): __ARCH_WANT_SYS_OLD_MMAP.
// - clone(): CONFIG_CLONE_BACKWARDS, CONFIG_CLONE_BACKWARDS2 and
// CONFIG_CLONE_BACKWARDS3.
// - socket-related syscalls: __ARCH_WANT_SYS_SOCKETCALL.
// - compat syscalls.
//
// This may or may not change in the future. User needs to take extra measures
// to handle such quirks explicitly, if necessary.
//
// This macro relies on BPF CO-RE support and virtual __kconfig externs.
//

    pub \: *mut *mut name(struct pt_regs ctx);,
    pub \: extern _Bool LINUX_HAS_SYSCALL_WRAPPER __kconfig;,
    pub \: *mut *mut ____##name(struct pt_regs ctx, ##args);,
    pub \: : ctx;,
    pub \: return ____##name(___bpf_syswrap_args(args));,
    pub \: return ____##name(___bpf_syscall_args(args));,

// BPF_UPROBE and BPF_URETPROBE are identical to BPF_KPROBE and BPF_KRETPROBE,
// but are named way less confusingly for SEC("uprobe") and SEC("uretprobe")
// use cases.
//

