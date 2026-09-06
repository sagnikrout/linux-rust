//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/paca.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// This control block defines the PACA which defines the processor
// specific data for each logical processor on the system.
// There are some pointers defined that are utilized by PLIC.
//
// C 2001 PPC 64 Team, IBM Corp
//

extern "C" {
    pub fn asm(_arg: "r13") -> *mut register struct paca_struct local_paca;
}

//
// Add standard checks that preemption cannot occur when using get_paca():
// otherwise the paca_struct it points to may be the wrong one just after.
//

//
// Defines the layout of the paca.
//
// This structure is not directly accessed by firmware or the service
// processor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct paca_struct {

//
// Because hw_cpu_id, unlike other paca fields, is accessed
// routinely from other CPUs (from the IRQ code), we stick to
// read-only (after boot) fields in the first cacheline to
// avoid cacheline bouncing.
//
    pub /: *mut *mut *mut lppaca lppaca_ptr; / Pointer to LpPaca for PLIC,

//
// MAGIC: the spinlock functions in arch/powerpc/lib/locks.c
// load lock_token and paca_index with a single lwz
// instruction.  They must travel together and be properly
// aligned.
//

    pub /: *mut *mut u16 lock_token; / Constant 0x8000, used in locks,
    pub /: *mut *mut u16 paca_index; / Logical processor number,

    pub /: *mut *mut u16 paca_index; / Logical processor number,
    pub /: *mut *mut u16 lock_token; / Constant 0x8000, used in locks,

    pub /: *mut *mut u64 kernel_toc; / Kernel TOC address,

    pub /: *mut *mut u64 kernelbase; / Base address of kernel,
    pub /: *mut *mut u64 kernel_msr; / MSR while running in kernel,
    pub /: *mut *mut *mut void emergency_sp; / pointer to emergency stack,
    pub /: *mut *mut u64 data_offset; / per cpu data offset,
    pub /: *mut *mut s16 hw_cpu_id; / Physical processor number,
    pub /: *mut *mut u8 cpu_start; / At startup, processor spins until,
// this becomes non-zero.
    pub /: *mut *mut u8 kexec_state; / set when kexec down has irqs off,

    pub slb_shadow_ptr: *mut slb_shadow,

    pub dispatch_log: *mut dtl_entry,
    pub dispatch_log_end: *mut dtl_entry,

    pub /: *mut *mut u64 dscr_default; / per-CPU default DSCR,

//
// Now, starting in cacheline 2, the exception save areas
//
// used for most interrupts/exceptions
    pub __attribute__((aligned(0x80))): u64 exgen[EX_SIZE],

// SLB related definitions
    pub vmalloc_sllp: u16,
    pub slb_cache_ptr: u8,
    pub /: *mut *mut u8 stab_rr; / stab/slb round-robin counter,

    pub in_kernel_slb_handler: u8,

    pub /: *mut *mut u32 slb_used_bitmap; / Bitmaps for first 32 SLB entries.,
    pub slb_kern_bitmap: u32,
    pub slb_cache: [u32; SLB_CACHE_ENTRIES],
    pub __aligned(0x40): u64 exgen[8],
// Keep pgd in the same cacheline as the start of extlb
    pub /: *mut *mut *mut pgd_t pgd __aligned(0x40); / Current PGD,
    pub /: *mut *mut *mut pgd_t kernel_pgd; / Kernel PGD,
// Shared by all threads of a core -- points to tcd of first thread
    pub tcd_ptr: *mut tlb_core_data,
//
// We can have up to 3 levels of reentrancy in the TLB miss handler,
// in each of four exception levels (normal, crit, mcheck, debug).
//
    pub sizeof(u64)]: u64 extlb[12][EX_TLB_SIZE /,
    pub /: *mut *mut u64 exmc[8]; / used for machine checks,
    pub /: *mut *mut u64 excrit[8]; / used for crit interrupts,
    pub /: *mut *mut u64 exdbg[8]; / used for debug interrupts,
// Kernel stack pointers for use by special exceptions
    pub mc_kstack: *mut c_void,
    pub crit_kstack: *mut c_void,
    pub dbg_kstack: *mut c_void,
    pub tcd: tlb_core_data,

    pub BITS_PER_BYTE]: unsigned char mm_ctx_low_slices_psize[BITS_PER_LONG /,
    pub mm_ctx_high_slices_psize: [c_uchar; SLICE_ARRAY_SIZE],
//
// then miscellaneous read-write fields
//
    pub /: *mut *mut *mut task___current; / Pointer to current,
    pub /: *mut *mut u64 kstack; / Saved Kernel stack addr,
    pub /: *mut *mut u64 saved_r1; / r1 save for RTAS calls or PM or EE=0,
    pub /: *mut *mut u64 saved_msr; / MSR saved here by enter_rtas,
    pub /: *mut *mut u64 exit_save_r1; / Syscall/interrupt R1 save,

    pub /: *mut *mut u16 trap_save; / Used when bad stack is encountered,

    pub /: *mut *mut u8 hsrr_valid; / HSRRs set for HRFID,
    pub /: *mut *mut u8 srr_valid; / SRRs set for RFID,

    pub /: *mut *mut u8 irq_soft_mask; / mask for irq soft masking,
    pub /: *mut *mut u8 irq_happened; / irq happened while soft-disabled,
    pub /: *mut *mut u8 irq_work_pending; / IRQ_WORK interrupt while soft-disable,

    pub /: *mut *mut u8 pmcregs_in_use; / pseries puts this in lppaca,

    pub /: *mut *mut u64 sprg_vdso; / Saved user-visible sprg,

    pub /: *mut *mut u64 tm_scratch; / TM scratch area for reclaim,

// PowerNV idle fields
// PNV_CORE_IDLE_* bits, all siblings work on thread 0 paca
    pub /: *mut *mut unsigned long idle_lock; / A value of 1 means acquired,
    pub idle_state: c_ulong,
// P7/P8 specific fields
// PNV_THREAD_RUNNING/NAP/SLEEP
    pub thread_idle_state: u8,
// Mask to denote subcore sibling threads
    pub subcore_sibling_mask: u8,
}

// P9 specific fields

// The PSSCR value that the kernel requested before going to stop
// Flag to request this thread not to stop

// Non-maskable exceptions that are not performance critical
// Exclusive stacks for system reset and machine check exception.
//
// Flag to check whether we are in machine check early handler
// and already using emergency stack.
//

// Stuff for accurate time accounting

// We use this to store guest state in

//
// Bitmap for sibling subcore status. See kvm/book3s_hv_ras.c for
// more details
//

//
// rfi fallback flush must be in its own cacheline to prevent
// other paca data leaking into the L1d
//

// Capture SLB related old contents in MCE handler.

extern "C" {
    pub fn copy_mm_to_paca(mm: *mut mm_struct);
}
extern "C" {
    pub fn initialise_paca(new_paca: *mut paca_struct, cpu: c_int);
}
extern "C" {
    pub fn setup_paca(new_paca: *mut paca_struct);
}
extern "C" {
    pub fn allocate_paca_ptrs();
}
extern "C" {
    pub fn allocate_paca(cpu: c_int);
}
extern "C" {
    pub fn free_unused_pacas();
}

