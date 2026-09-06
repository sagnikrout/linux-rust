//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/paravirt_types.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_cpu_ops {
// hooks for various privileged instructions

    pub regno): *mut *mut unsigned long (get_debugreg)(int,
    pub value): *mut *mut void (set_debugreg)(int regno, unsigned long,
    pub (*read_cr0)(void): *mut c_ulong,
    pub long): *mut *mut void (write_cr0)(unsigned,
    pub long): *mut *mut void (write_cr4)(unsigned,
// Segment descriptor handling
    pub (*load_tr_desc)(void): *mut c_void,
    pub ): *const *const void (load_gdt)(struct desc_ptr,
    pub ): *const *const void (load_idt)(struct desc_ptr,
    pub entries): *const *const *const void (set_ldt)(void desc, unsigned,
    pub (*store_tr)(void): *mut c_ulong,
    pub cpu): *mut *mut *mut void (load_tls)(struct thread_struct t, unsigned int,
    pub idx): *mut *mut void (load_gs_index)(unsigned int,
    pub desc): *const c_void,
    pub size): *const *const int entrynum, void desc, int,
    pub gate): *const int entrynum, gate_desc,
    pub entries): *mut *mut *mut void (alloc_ldt)(struct desc_struct ldt, unsigned,
    pub entries): *mut *mut *mut void (free_ldt)(struct desc_struct ldt, unsigned,
    pub sp0): *mut *mut void (load_sp0)(unsigned long,

    pub (*invalidate_io_bitmap)(void): *mut c_void,
    pub (*update_io_bitmap)(void): *mut c_void,

// cpuid emulation, mostly so that caps bits can be disabled
    pub edx): *mut *mut unsigned int ecx, unsigned int,
// Unsafe MSR operations.  These will warn or panic on failure.
    pub msr): *mut *mut u64 (read_msr)(u32,
    pub val): *mut *mut void (write_msr)(u32 msr, u64,
//
// Safe MSR operations.
// Returns 0 or -EIO.
//
    pub val): *mut *mut int (read_msr_safe)(u32 msr, u64,
    pub val): *mut *mut int (write_msr_safe)(u32 msr, u64,
    pub counter): *mut *mut u64 (read_pmc)(int,
    pub prev): *mut *mut void (start_context_switch)(struct task_struct,
    pub next): *mut *mut void (end_context_switch)(struct task_struct,

    pub __no_randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_irq_ops {

//
// Get/set interrupt state.  save_fl is expected to use X86_EFLAGS_IF;
// all other bits returned from save_fl are undefined.
//
// NOTE: These functions callers expect the callee to preserve
// more registers than the standard C calling convention.
//
    pub save_fl: paravirt_callee_save,
    pub irq_disable: paravirt_callee_save,
    pub irq_enable: paravirt_callee_save,

    pub (*safe_halt)(void): *mut c_void,
    pub (*halt)(void): *mut c_void,
    pub __no_randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pv_mmu_ops {
// TLB operations
    pub (*flush_tlb_user)(void): *mut c_void,
    pub (*flush_tlb_kernel)(void): *mut c_void,
    pub addr): *mut *mut void (flush_tlb_one_user)(unsigned long,
    pub info): *const flush_tlb_info,
// Hook for intercepting the destruction of an mm_struct.
    pub mm): *mut *mut void (exit_mmap)(struct mm_struct,
    pub enc): *mut *mut void (notify_page_enc_status_changed)(unsigned long pfn, int npages, bool,

    pub read_cr2: paravirt_callee_save,
    pub long): *mut *mut void (write_cr2)(unsigned,
    pub (*read_cr3)(void): *mut c_ulong,
    pub long): *mut *mut void (write_cr3)(unsigned,
// Hook for intercepting the creation/use of an mm_struct.
    pub mm): *mut *mut void (enter_mmap)(struct mm_struct,
// Hooks for allocating and freeing a pagetable top-level
    pub mm): *mut *mut int (pgd_alloc)(struct mm_struct,
    pub pgd): *mut *mut *mut void (pgd_free)(struct mm_struct mm, pgd_t,
//
// Hooks for allocating/releasing pagetable pages when they're
// attached to a pagetable
//
    pub pfn): *mut *mut *mut void (alloc_pte)(struct mm_struct mm, unsigned long,
    pub pfn): *mut *mut *mut void (alloc_pmd)(struct mm_struct mm, unsigned long,
    pub pfn): *mut *mut *mut void (alloc_pud)(struct mm_struct mm, unsigned long,
    pub pfn): *mut *mut *mut void (alloc_p4d)(struct mm_struct mm, unsigned long,
    pub pfn): *mut *mut void (release_pte)(unsigned long,
    pub pfn): *mut *mut void (release_pmd)(unsigned long,
    pub pfn): *mut *mut void (release_pud)(unsigned long,
    pub pfn): *mut *mut void (release_p4d)(unsigned long,
// Pagetable manipulation functions
    pub pteval): *mut *mut *mut void (set_pte)(pte_t ptep, pte_t,
    pub pmdval): *mut *mut *mut void (set_pmd)(pmd_t pmdp, pmd_t,
    pub ptep): *mut pte_t,
    pub pte): *mut *mut pte_t ptep, pte_t,
    pub pte_val: paravirt_callee_save,
    pub make_pte: paravirt_callee_save,
    pub pgd_val: paravirt_callee_save,
    pub make_pgd: paravirt_callee_save,
    pub pudval): *mut *mut *mut void (set_pud)(pud_t pudp, pud_t,
    pub pmd_val: paravirt_callee_save,
    pub make_pmd: paravirt_callee_save,
    pub pud_val: paravirt_callee_save,
    pub make_pud: paravirt_callee_save,
    pub p4dval): *mut *mut *mut void (set_p4d)(p4d_t p4dp, p4d_t,
    pub p4d_val: paravirt_callee_save,
    pub make_p4d: paravirt_callee_save,
    pub pgdval): *mut *mut *mut void (set_pgd)(pgd_t pgdp, pgd_t,
    pub (*lazy_mode_flush)(void): *mut c_void,
// dom0 ops
// Sometimes the physical address is a pfn, and sometimes its
    pub flags): phys_addr_t phys, pgprot_t,

    pub __no_randomize_layout: },
// This contains all the paravirt structures: we get a convenient
// number for each function using the offset which we use to indicate
// what to patch.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct paravirt_patch_template {
    pub cpu: pv_cpu_ops,
    pub irq: pv_irq_ops,
    pub mmu: pv_mmu_ops,
    pub __no_randomize_layout: },
    pub pv_ops: extern struct paravirt_patch_template,

//
// This generates an indirect call based on the operation type number.
//
// Since alternatives run after enabling CET/IBT -- the latter setting/clearing
// capabilities and the former requiring all capabilities being finalized --
// these indirect calls are subject to IBT and the paravirt stubs should have
// ENDBR on.
//
// OTOH since this is effectively a __nocfi indirect call, the paravirt stubs
// don't need to bother with CFI prefixes.
//

//
// These macros are intended to wrap calls through one of the paravirt
// ops structs, so that they can be later identified and patched at
// runtime.
//
// Normally, a call to a pv_op function is a simple indirect call:
// (pv_op_struct.operations)(args...).
//
// Unfortunately, this is a relatively slow operation for modern CPUs,
// because it cannot necessarily determine what the destination
// address is.  In this case, the address is a runtime constant, so at
// the very least we can patch the call to a simple direct call, or,
// ideally, patch an inline implementation into the callsite.  (Direct
// calls are essentially free, because the call and return addresses
// are completely predictable.)
//
// For i386, these macros rely on the standard gcc "regparm(3)" calling
// convention, in which the first three arguments are placed in %eax,
// %edx, %ecx (in that order), and the remaining arguments are placed
// on the stack.  All caller-save registers (eax,edx,ecx) are expected
// to be modified (either clobbered or used for return values).
// X86_64, on the other hand, already specifies a register-based calling
// conventions, returning at %rax, with parameters going in %rdi, %rsi,
// %rdx, and %rcx. Note that for this reason, x86_64 does not need any
// special handling for dealing with 4 arguments, unlike i386.
// However, x86_64 also has to clobber all caller saved registers, which
// unfortunately, are quite a bit (r8 - r11)
//
// Unfortunately there's no way to get gcc to generate the args setup
// for the call, and then allow the call itself to be generated by an
// inline asm.  Because of this, we must do the complete arg setup and
// return value handling from within these macros.  This is fairly
// cumbersome.
//
// There are 5 sets of PVOP_* macros for dealing with 0-4 arguments.
// It could be extended to more arguments, but there would be little
// to be gained from that.  For each number of arguments, there are
// two VCALL and CALL variants for void and non-void functions.
//
// When there is a return value, the invoker of the macro must specify
// the return type.  The macro then uses sizeof() on that type to
// determine whether it's a 32 or 64 bit value and places the return
// in the right register(s) (just %eax for 32-bit, and %edx:%eax for
// 64-bit). For x86_64 machines, it just returns in %rax regardless of
// the return value size.
//
// 64-bit arguments are passed as a pair of adjacent 32-bit arguments;
// i386 also passes 64-bit arguments as a pair of adjacent 32-bit arguments
// in low,high order
//
// Small structures are passed and returned in registers.  The macro
// calling convention can't directly deal with this, so the wrapper
// functions must do it.
//
// These PVOP_* macros are only defined within this header.  This
// means that all uses must be wrapped in inline functions.  This also
// makes sure the incoming and outgoing types are always correct.
//

    pub __ecx: unsigned long __eax = __eax, __edx = __edx, __ecx =,

// Macro flag: #define EXTRA_CLOBBERS
// Macro flag: #define VEXTRA_CLOBBERS

// [re]ax isn't an arg, but the return val

    pub __eax: __edx = __edx, __ecx = __ecx, __eax =,

//
// void functions are still allowed [re]ax for scratch.
//
// The ZERO_CALL_USED REGS feature may end up zeroing out callee-saved
// registers. Make sure we model this with the appropriate clobbers.
//

    pub \: ({ unsigned long __mask = ~0UL;,
    pub \: BUILD_BUG_ON(sizeof(rettype) > sizeof(unsigned long));,
    pub \: case 1: __mask = 0xffUL; break;,
    pub \: case 2: __mask = 0xffffUL; break;,
    pub \: case 4: __mask = 0xffffffffUL; break;,
    pub \: default: break;,
    pub \: __mask & __eax;,
//
// Use alternative patching for paravirt calls:
// - For replacing an indirect call with a direct one, use the "normal"
// ALTERNATIVE() macro with the indirect call as the initial code sequence,
// which will be replaced with the related direct call by using the
// ALT_FLAG_DIRECT_CALL special case and the "always on" feature.
// - In case the replacement is either a direct call or a short code sequence
// depending on a feature bit, the ALTERNATIVE_2() macro is being used.
// The indirect call is the initial code sequence again, while the special
// code sequence is selected with the specified feature bit. In case the
// feature is not active, the direct call is used as above via the
// ALT_FLAG_DIRECT_CALL special case and the "always on" feature.
//

    pub \: PVOP_CALL_ARGS;,

    pub \: : "memory", "cc" extra_clbr);,
    pub \: ret;,

    pub \: PVOP_CALL_ARGS;,

    pub \: : "memory", "cc" extra_clbr);,
    pub \: ret;,

// save and restore all caller-save registers, except return value

// save and restore all caller-save registers, except return value

    pub \: "push %rcx;",
    pub \: "push %rdx;",
    pub \: "push %rsi;",
    pub \: "push %rdi;",
    pub \: "push %r8;",
    pub \: "push %r9;",
    pub \: "push %r10;",
    pub %r11;": "push,

    pub \: "pop %r11;",
    pub \: "pop %r10;",
    pub \: "pop %r9;",
    pub \: "pop %r8;",
    pub \: "pop %rdi;",
    pub \: "pop %rsi;",
    pub \: "pop %rdx;",
    pub %rcx;": "pop,

//
// Generate a thunk around a function which saves all caller-save
// registers except for the return value.  This allows C functions to
// be called from assembler code where fewer than normal registers are
// available.  It may also help code generation around calls from C
// code if the common case doesn't use many registers.
//
// When a callee is wrapped in a thunk, the caller can assume that all
// arg regs and all scratch registers are preserved across the
// call. The return value in rax/eax will not be saved, even for void
// functions.
//

    pub \: extern typeof(func) __raw_callee_save_##func;,
    pub \: asm(".pushsection " section ", \"ax\";",
    pub \: ".globl " PV_THUNK_NAME(func) ";",
    pub \: ".type " PV_THUNK_NAME(func) ", @function;",
    pub \: "call " #func ";",
    pub \: ".size " PV_THUNK_NAME(func) ", .-" PV_THUNK_NAME(func) ";",

// Get a reference to a callee-save function

// Promise that "func" already uses the right calling convention

