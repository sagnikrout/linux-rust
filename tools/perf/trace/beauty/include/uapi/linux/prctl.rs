//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/trace/beauty/include/uapi/linux/prctl.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// Values to pass as first argument to prctl()

// Get/set current->mm->dumpable
pub const PR_GET_DUMPABLE: c_int = 3;
pub const PR_SET_DUMPABLE: c_int = 4;
// Get/set unaligned access control bits (if meaningful)
pub const PR_GET_UNALIGN: c_int = 5;
pub const PR_SET_UNALIGN: c_int = 6;

// Get/set whether or not to drop capabilities on setuid() away from
// uid 0 (as per security/commoncap.c)
pub const PR_GET_KEEPCAPS: c_int = 7;
pub const PR_SET_KEEPCAPS: c_int = 8;
// Get/set floating-point emulation control bits (if meaningful)
pub const PR_GET_FPEMU: c_int = 9;
pub const PR_SET_FPEMU: c_int = 10;

// Get/set floating-point exception mode (if meaningful)
pub const PR_GET_FPEXC: c_int = 11;
pub const PR_SET_FPEXC: c_int = 12;

// Get/set whether we use statistical process timing or accurate timestamp
// based process timing
pub const PR_GET_TIMING: c_int = 13;
pub const PR_SET_TIMING: c_int = 14;

// Get/set process endian
pub const PR_GET_ENDIAN: c_int = 19;
pub const PR_SET_ENDIAN: c_int = 20;

// Get/set process seccomp mode
pub const PR_GET_SECCOMP: c_int = 21;
pub const PR_SET_SECCOMP: c_int = 22;
// Get/set the capability bounding set (as per security/commoncap.c)
pub const PR_CAPBSET_READ: c_int = 23;
pub const PR_CAPBSET_DROP: c_int = 24;
// Get/set the process' ability to use the timestamp counter instruction
pub const PR_GET_TSC: c_int = 25;
pub const PR_SET_TSC: c_int = 26;

// Get/set securebits (as per security/commoncap.c)
pub const PR_GET_SECUREBITS: c_int = 27;
pub const PR_SET_SECUREBITS: c_int = 28;
//
// Get/set the timerslack as used by poll/select/nanosleep
// A value of 0 means "use default"
//
pub const PR_SET_TIMERSLACK: c_int = 29;
pub const PR_GET_TIMERSLACK: c_int = 30;
pub const PR_TASK_PERF_EVENTS_DISABLE: c_int = 31;
pub const PR_TASK_PERF_EVENTS_ENABLE: c_int = 32;
//
// Set early/late kill mode for hwpoison memory corruption.
// This influences when the process gets killed on a memory corruption.
//
pub const PR_MCE_KILL: c_int = 33;

pub const PR_MCE_KILL_GET: c_int = 34;
//
// Tune up process memory map specifics.
//
pub const PR_SET_MM: c_int = 35;

//
// This structure provides new memory descriptor
// map which mostly modifies /proc/pid/stat[m]
// output for a task. This mostly done in a
// sake of checkpoint/restore functionality.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prctl_mm_map {
    pub /: *mut *mut __u64 start_code; / code section bounds,
    pub end_code: __u64,
    pub /: *mut *mut __u64 start_data; / data section bounds,
    pub end_data: __u64,
    pub /: *mut *mut __u64 start_brk; / heap for brk() syscall,
    pub brk: __u64,
    pub /: *mut *mut __u64 start_stack; / stack starts at,
    pub /: *mut *mut __u64 arg_start; / command line arguments bounds,
    pub arg_end: __u64,
    pub /: *mut *mut __u64 env_start; / environment variables bounds,
    pub env_end: __u64,
    pub /: *mut *mut *mut __u64 auxv; / auxiliary vector,
    pub /: *mut *mut __u32 auxv_size; / vector size,
    pub /: *mut *mut __u32 exe_fd; / /proc/$pid/exe link file,
}

//
// Set specific pid that is allowed to ptrace the current task.
// A value of 0 mean "no process".
//
pub const PR_SET_PTRACER: c_uint = 0x59616d61;

pub const PR_SET_CHILD_SUBREAPER: c_int = 36;
pub const PR_GET_CHILD_SUBREAPER: c_int = 37;
//
// If no_new_privs is set, then operations that grant new privileges (i.e.
// execve) will either fail or not grant them.  This affects suid/sgid,
// file capabilities, and LSMs.
//
// Operations that merely manipulate or drop existing privileges (setresuid,
// capset, etc.) will still work.  Drop those privileges if you want them gone.
//
// Changing LSM security domain is considered a new privilege.  So, for example,
// asking selinux for a specific new context (e.g. with runcon) will result
// in execve returning -EPERM.
//
// See Documentation/userspace-api/no_new_privs.rst for more details.
//
pub const PR_SET_NO_NEW_PRIVS: c_int = 38;
pub const PR_GET_NO_NEW_PRIVS: c_int = 39;
pub const PR_GET_TID_ADDRESS: c_int = 40;
//
// Flags for PR_SET_THP_DISABLE are only applicable when disabling. Bit 0
// is reserved, so PR_GET_THP_DISABLE can return "1 | flags", to effectively
// return "1" when no flags were specified for PR_SET_THP_DISABLE.
//
pub const PR_SET_THP_DISABLE: c_int = 41;
//
// Don't disable THPs when explicitly advised (e.g., MADV_HUGEPAGE
// VM_HUGEPAGE, MADV_COLLAPSE).
//

pub const PR_GET_THP_DISABLE: c_int = 42;
//
// No longer implemented, but left here to ensure the numbers stay reserved:
//
pub const PR_MPX_ENABLE_MANAGEMENT: c_int = 43;
pub const PR_MPX_DISABLE_MANAGEMENT: c_int = 44;
pub const PR_SET_FP_MODE: c_int = 45;
pub const PR_GET_FP_MODE: c_int = 46;

// Control the ambient capability set
pub const PR_CAP_AMBIENT: c_int = 47;

// arm64 Scalable Vector Extension controls
// Flag values must be kept in sync with ptrace NT_ARM_SVE interface

// Bits common to PR_SVE_SET_VL and PR_SVE_GET_VL

// Per task speculation control
pub const PR_GET_SPECULATION_CTRL: c_int = 52;
pub const PR_SET_SPECULATION_CTRL: c_int = 53;
// Speculation control variants

// Return and control values for PR_SET/GET_SPECULATION_CTRL

// Reset arm64 pointer authentication keys
pub const PR_PAC_RESET_KEYS: c_int = 54;

// Tagged user address controls for arm64 and RISC-V
pub const PR_SET_TAGGED_ADDR_CTRL: c_int = 55;
pub const PR_GET_TAGGED_ADDR_CTRL: c_int = 56;

// MTE tag check fault modes

// MTE tag inclusion mask

// Unused; kept only for source compatibility

// MTE tag check store only

// RISC-V pointer masking tag length

// Control reclaim behavior when allocating memory
pub const PR_SET_IO_FLUSHER: c_int = 57;
pub const PR_GET_IO_FLUSHER: c_int = 58;
// Dispatch syscalls to a userspace handler
pub const PR_SET_SYSCALL_USER_DISPATCH: c_int = 59;

// Enable dispatch except for the specified range

// Enable dispatch for the specified range

// Legacy name for backwards compatibility

// The control values for the user space selector when dispatch is enabled

// Set/get enabled arm64 pointer authentication keys
pub const PR_PAC_SET_ENABLED_KEYS: c_int = 60;
pub const PR_PAC_GET_ENABLED_KEYS: c_int = 61;
// Request the scheduler to share a core
pub const PR_SCHED_CORE: c_int = 62;

// arm64 Scalable Matrix Extension controls
// Flag values must be in sync with SVE versions

// Bits common to PR_SME_SET_VL and PR_SME_GET_VL

// Memory deny write / execute
pub const PR_SET_MDWE: c_int = 65;

pub const PR_GET_MDWE: c_int = 66;
pub const PR_SET_VMA: c_uint = 0x53564d41;

pub const PR_GET_AUXV: c_uint = 0x41555856;
pub const PR_SET_MEMORY_MERGE: c_int = 67;
pub const PR_GET_MEMORY_MERGE: c_int = 68;
pub const PR_RISCV_V_SET_CONTROL: c_int = 69;
pub const PR_RISCV_V_GET_CONTROL: c_int = 70;

pub const PR_RISCV_SET_ICACHE_FLUSH_CTX: c_int = 71;

// PowerPC Dynamic Execution Control Register (DEXCR) controls
pub const PR_PPC_GET_DEXCR: c_int = 72;
pub const PR_PPC_SET_DEXCR: c_int = 73;
// DEXCR aspect to act on

// Action to apply / return

//
// Get the current shadow stack configuration for the current thread,
// this will be the value configured via PR_SET_SHADOW_STACK_STATUS.
//
pub const PR_GET_SHADOW_STACK_STATUS: c_int = 74;
//
// Set the current shadow stack configuration.  Enabling the shadow
// stack will cause a shadow stack to be allocated for the thread.
//
pub const PR_SET_SHADOW_STACK_STATUS: c_int = 75;

//
// Prevent further changes to the specified shadow stack
// configuration.  All bits may be locked via this call, including
// undefined bits.
//
pub const PR_LOCK_SHADOW_STACK_STATUS: c_int = 76;
//
// Controls the mode of timer_create() for CRIU restore operations.
// Enabling this allows CRIU to restore timers with explicit IDs.
//
// Don't use for normal operations as the result might be undefined.
//
pub const PR_TIMER_CREATE_RESTORE_IDS: c_int = 77;

// FUTEX hash management
pub const PR_FUTEX_HASH: c_int = 78;

// RSEQ time slice extensions
pub const PR_RSEQ_SLICE_EXTENSION: c_int = 79;

//
// Bits for RSEQ_SLICE_EXTENSION_GET/SET
// PR_RSEQ_SLICE_EXT_ENABLE:	Enable
//

//
// Get or set the control flow integrity (CFI) configuration for the
// current thread.
//
// Some per-thread control flow integrity settings are not yet
// controlled through this prctl(); see for example
// PR_{GET,SET,LOCK}_SHADOW_STACK_STATUS
//
pub const PR_GET_CFI: c_int = 80;
pub const PR_SET_CFI: c_int = 81;
//
// Forward-edge CFI variants (excluding ARM64 BTI, which has its own
// prctl()s).
//
pub const PR_CFI_BRANCH_LANDING_PADS: c_int = 0;
// Return and control values for PR_{GET,SET}_CFI

