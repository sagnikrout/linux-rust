//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/bpf/bpf_helpers.h
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
//
// Note that bpf programs need to include either
// vmlinux.h (auto-generated from BTF) or linux/types.h
// in advance since bpf_helper_defs.h uses such types
// as __u64.
//

//
// Helper macro to place programs, maps, license in
// different sections in elf_bpf file. Section names
// are interpreted by libbpf depending on the context (BPF programs, BPF maps,
// extern variables, etc).
// To allow use of SEC() with externs (e.g., for extern .maps declarations),
// make sure __attribute__((unused)) doesn't trigger compilation warning.
//

//
// Pragma macros are broken on GCC
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=55578
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=90400
//

// Avoid 'linux/stddef.h' definition of '__always_inline'.

//
// Use __hidden attribute to mark a non-static BPF subprogram effectively
// static for BPF verifier's verification algorithm purposes, allowing more
// extensive and permissive BPF verification process, taking into account
// subprogram's caller context.
//

// When utilizing vmlinux.h with BPF CO-RE, user BPF programs can't include
// any system-level headers (such as stddef.h, linux/version.h, etc), and
// commonly-used macros like NULL and KERNEL_VERSION aren't available through
// vmlinux.h. This just adds unnecessary hurdles and forces users to re-define
// them on their own. So as a convenience, provide such definitions here.
//

//
// Helper macros to manipulate data structures
//
// offsetof() definition that uses __builtin_offset() might not preserve field
// offset CO-RE relocation properly, so force-redefine offsetof() using
// old-school approach which works with CO-RE correctly
//

// redefined container_of() to ensure we use the above offsetof() macro

//
// Compiler (optimization) barrier.
//

// Variable-specific compiler (optimization) barrier. It's a no-op which makes
// compiler believe that there is some black box modification of a given
// variable and thus prevents compiler from making extra assumption about its
// value and potential simplifications and optimizations on this variable.
//
// E.g., compiler might often delay or even omit 32-bit to 64-bit casting of
// a variable, making some code patterns unverifiable. Putting barrier_var()
// in place will ensure that cast is performed before the barrier_var()
// invocation, because compiler has to pessimistically assume that embedded
// asm section might perform some extra operations on that variable.
//
// This is a variable-specific variant of more global barrier().
//

//
// Helper macro to throw a compilation error if __bpf_unreachable() gets
// built into the resulting code. This works given BPF back end does not
// implement __builtin_trap(). This is useful to assert that certain paths
// of the program code are never used and hence eliminated by the compiler.
//
// For example, consider a switch statement that covers known cases used by
// the program. __bpf_unreachable() can then reside in the default case. If
// the program gets extended such that a case is not covered in the switch
// statement, then it will throw a build error due to the default case not
// being compiled out.
//

//
// Helper function to perform a tail call with a constant/immediate map slot.
//

//
// Provide a hard guarantee that LLVM won't optimize setting r2 (map
// pointer) and r3 (constant map index) from _different paths_ ending
// up at the _same_ call insn as otherwise we won't be able to use the
// jmpq/nopl retpoline-free patching by the x86-64 JIT in the kernel
// given they mismatch. See also d2e4c1e6c294 ("bpf: Constant map key
// tracking for prog array pokes") for details on verifier tracking.
//
// Note on clobber list: we need to stay in-line with BPF calling
// convention, so even if we don't end up using r0, r4, r5, we need
// to mark them as clobber so that LLVM doesn't end up using them
// before / after the call.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libbpf_pin_type {
    LIBBPF_PIN_NONE,
// PIN_BY_NAME: pin maps by name (in /sys/fs/bpf by default)
    LIBBPF_PIN_BY_NAME,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libbpf_tristate {
    TRI_NO = 0,
    TRI_YES = 1,
    TRI_MODULE = 2,
}

//
// BPF_SEQ_PRINTF to wrap bpf_seq_printf to-be-printed values
// in a structure.
//

//
// BPF_SNPRINTF wraps the bpf_snprintf helper with variadic arguments instead of
// an array of u64.
//

// Macro flag: #define BPF_PRINTK_FMT_MOD

//
// __bpf_vprintk wraps the bpf_trace_vprintk helper with variadic arguments
// instead of an array of u64.
//

// Use __bpf_printk when bpf_printk call has 3 or fewer fmt args
// Otherwise use __bpf_vprintk
//

// Helper macro to print out debug messages

// bpf_for_each(iter_type, cur_elem, args...) provides generic construct for
// using BPF open-coded iterators without having to write mundane explicit
// low-level loop logic. Instead, it provides for()-like generic construct
// that can be used pretty naturally. E.g., for some hypothetical cgroup
// iterator, you'd write:
//
// struct cgroup *cg, *parent_cg = <...>;
//
// bpf_for_each(cgroup, cg, parent_cg, CG_ITER_CHILDREN) {
// bpf_printk("Child cgroup id = %d", cg->cgroup_id);
// if (cg->cgroup_id == 123)
// break;
// }
//
// I.e., it looks almost like high-level for each loop in other languages,
// supports continue/break, and is verifiable by BPF verifier.
//
// For iterating integers, the difference between bpf_for_each(num, i, N, M)
// and bpf_for(i, N, M) is in that bpf_for() provides additional proof to
// verifier that i is in [N, M) range, and in bpf_for_each() case i is `int
// *`, not just `int`. So for integers bpf_for() is more convenient.
//
// Note: this macro relies on C99 feature of allowing to declare variables
// inside for() loop, bound to for() loop lifetime. It also utilizes GCC
// extension: __attribute__((cleanup(<func>))), supported by both GCC and
// Clang.
//

// initialize and define destructor */							\
// ___p pointer is just to call bpf_iter_##type##_new() *once* to init ___it */		\
// ___p __attribute__((unused)) = (				\
// this is a workaround for Clang bug: it currently doesn't emit BTF */			\
// for bpf_iter_##type##_destroy() when used from cleanup() attribute */		\
// iteration and termination check */							\

// bpf_for(i, start, end) implements a for()-like looping construct that sets
// provided integer variable *i* to values starting from *start* through,
// but not including, *end*. It also proves to BPF verifier that *i* belongs
// to range [start, end), so this can be used for accessing arrays without
// extra checks.
//
// Note: *start* and *end* are assumed to be expressions with no side effects
// and whose values do not change throughout bpf_for() loop execution. They do
// not have to be statically known or constant, though.
//
// Note: similarly to bpf_for_each(), it relies on C99 feature of declaring for()
// loop bound variables and cleanup attribute, supported by GCC and Clang.
//

// initialize and define destructor */							\
// ___p pointer is necessary to call bpf_iter_num_new() *once* to init ___it */		\
// ___p __attribute__((unused)) = (					\
// this is a workaround for Clang bug: it currently doesn't emit BTF */			\
// for bpf_iter_num_destroy() when used from cleanup() attribute */			\
// iteration step */								\
// termination and bounds check */						\

// bpf_repeat(N) performs N iterations without exposing iteration number
//
// Note: similarly to bpf_for_each(), it relies on C99 feature of declaring for()
// loop bound variables and cleanup attribute, supported by GCC and Clang.
//

// initialize and define destructor */							\
// ___p pointer is necessary to call bpf_iter_num_new() *once* to init ___it */		\
// ___p __attribute__((unused)) = (					\
// this is a workaround for Clang bug: it currently doesn't emit BTF */			\
// for bpf_iter_num_destroy() when used from cleanup() attribute */			\
// nothing here  */									\

