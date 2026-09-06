//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/fpu/types.h
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
//
// FPU data structures:
//

//
// The legacy x87 FPU state format, as saved by FSAVE and
// restored by the FRSTOR instructions:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fregs_state {
    pub /: *mut *mut u32 cwd; / FPU Control Word,
    pub /: *mut *mut u32 swd; / FPU Status Word,
    pub /: *mut *mut u32 twd; / FPU Tag Word,
    pub /: *mut *mut u32 fip; / FPU IP Offset,
    pub /: *mut *mut u32 fcs; / FPU IP Selector,
    pub /: *mut *mut u32 foo; / FPU Operand Pointer Offset,
    pub /: *mut *mut u32 fos; / FPU Operand Pointer Selector,
// 8*10 bytes for each FP-reg = 80 bytes:
    pub st_space: [u32; 20],
// Software status information [not touched by FSAVE]:
    pub status: u32,
}

//
// The legacy fx SSE/MMX FPU state format, as saved by FXSAVE and
// restored by the FXRSTOR instructions. It's similar to the FSAVE
// format, but differs in some areas, plus has extensions at
// the end for the XMM registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fxregs_state {
    pub /: *mut *mut u16 cwd; / Control Word,
    pub /: *mut *mut u16 swd; / Status Word,
    pub /: *mut *mut u16 twd; / Tag Word,
    pub /: *mut *mut u16 fop; / Last Instruction Opcode,
    pub /: *mut *mut u64 rip; / Instruction Pointer,
    pub /: *mut *mut u64 rdp; / Data Pointer,
}

// 8*16 bytes for each FP-reg = 128 bytes:
// 16*16 bytes for each XMM-reg = 256 bytes:
// Default value for fxregs_state.mxcsr:
pub const MXCSR_DEFAULT: c_uint = 0x1f80;
// Copy both mxcsr & mxcsr_flags with a single u64 memcpy:

//
// Software based FPU emulation state. This is arbitrary really,
// it matches the x87 format to make it easier to understand:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct swregs_state {
    pub cwd: u32,
    pub swd: u32,
    pub twd: u32,
    pub fip: u32,
    pub fcs: u32,
    pub foo: u32,
    pub fos: u32,
// 8*10 bytes for each FP-reg = 80 bytes:
    pub st_space: [u32; 20],
    pub ftop: u8,
    pub changed: u8,
    pub lookahead: u8,
    pub no_update: u8,
    pub rm: u8,
    pub alimit: u8,
    pub info: *mut math_emu_info,
    pub entry_eip: u32,
}

//
// List of XSAVE features Linux knows about:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xfeature {
    XFEATURE_FP,
    XFEATURE_SSE,
//
// Values above here are "legacy states".
// Those below are "extended states".
//
    XFEATURE_YMM,
    XFEATURE_BNDREGS,
    XFEATURE_BNDCSR,
    XFEATURE_OPMASK,
    XFEATURE_ZMM_Hi256,
    XFEATURE_Hi16_ZMM,
    XFEATURE_PT_UNIMPLEMENTED_SO_FAR,
    XFEATURE_PKRU,
    XFEATURE_PASID,
    XFEATURE_CET_USER,
    XFEATURE_CET_KERNEL,
    XFEATURE_RSRVD_COMP_13,
    XFEATURE_RSRVD_COMP_14,
    XFEATURE_LBR,
    XFEATURE_RSRVD_COMP_16,
    XFEATURE_XTILE_CFG,
    XFEATURE_XTILE_DATA,
    XFEATURE_APX,

    XFEATURE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_128_bit {
    pub regbytes: [u8; 128/8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_256_bit {
    pub regbytes: [u8; 256/8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_512_bit {
    pub regbytes: [u8; 512/8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_1024_byte {
    pub regbytes: [u8; 1024],
}

//
// State component 2:
//
// There are 16x 256-bit AVX registers named YMM0-YMM15.
// The low 128 bits are aliased to the 16 SSE registers (XMM0-XMM15)
// and are stored in 'struct fxregs_state::xmm_space[]' in the
// "legacy" area.
//
// The high 128 bits are stored here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ymmh_struct {
    pub hi_ymm: [reg_128_bit; 16],
    pub __packed: },
// Intel MPX support:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpx_bndreg {
    pub lower_bound: u64,
    pub upper_bound: u64,
    pub __packed: },
//
// State component 3 is used for the 4 128-bit bounds registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpx_bndreg_state {
    pub bndreg: [mpx_bndreg; 4],
    pub __packed: },
//
// State component 4 is used for the 64-bit user-mode MPX
// configuration register BNDCFGU and the 64-bit MPX status
// register BNDSTATUS.  We call the pair "BNDCSR".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpx_bndcsr {
    pub bndcfgu: u64,
    pub bndstatus: u64,
    pub __packed: },
//
// The BNDCSR state is padded out to be 64-bytes in size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpx_bndcsr_state {
    pub bndcsr: mpx_bndcsr,
    pub pad_to_64_bytes: [u8; 64],
}

// AVX-512 Components:
//
// State component 5 is used for the 8 64-bit opmask registers
// k0-k7 (opmask state).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avx_512_opmask_state {
    pub opmask_reg: [u64; 8],
    pub __packed: },
//
// State component 6 is used for the upper 256 bits of the
// registers ZMM0-ZMM15. These 16 256-bit values are denoted
// ZMM0_H-ZMM15_H (ZMM_Hi256 state).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avx_512_zmm_uppers_state {
    pub zmm_upper: [reg_256_bit; 16],
    pub __packed: },
//
// State component 7 is used for the 16 512-bit registers
// ZMM16-ZMM31 (Hi16_ZMM state).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avx_512_hi16_state {
    pub hi16_zmm: [reg_512_bit; 16],
    pub __packed: },
//
// State component 9: 32-bit PKRU register.  The state is
// 8 bytes long but only 4 bytes is used currently.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkru_state {
    pub pkru: u32,
    pub pad: u32,
    pub __packed: },
//
// State component 11 is Control-flow Enforcement user states
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cet_user_state {
// user control-flow settings
    pub user_cet: u64,
// user shadow stack pointer
    pub user_ssp: u64,
}

//
// State component 12 is Control-flow Enforcement supervisor states.
// This state includes SSP pointers for privilege levels 0 through 2.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cet_supervisor_state {
    pub pl0_ssp: u64,
    pub pl1_ssp: u64,
    pub pl2_ssp: u64,
    pub __packed: },
//
// State component 15: Architectural LBR configuration state.
// The size of Arch LBR state depends on the number of LBRs (lbr_depth).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lbr_entry {
    pub from: u64,
    pub to: u64,
    pub info: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_lbr_state {
    pub lbr_ctl: u64,
    pub lbr_depth: u64,
    pub ler_from: u64,
    pub ler_to: u64,
    pub ler_info: u64,
    pub entries: [lbr_entry; ],
}

//
// State component 17: 64-byte tile configuration register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtile_cfg {
    pub tcfg: [u64; 8],
    pub __packed: },
//
// State component 18: 1KB tile data register.
// Each register represents 16 64-byte rows of the matrix
// data. But the number of registers depends on the actual
// implementation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtile_data {
    pub tmm: reg_1024_byte,
    pub __packed: },
//
// State component 19: 8B extended general purpose register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apx_state {
    pub egpr: [u64; 16],
    pub __packed: },
//
// State component 10 is supervisor state used for context-switching the
// PASID state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ia32_pasid_state {
    pub pasid: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstate_header {
    pub xfeatures: u64,
    pub xcomp_bv: u64,
    pub reserved: [u64; 6],
    pub __attribute__((packed)): },
//
// xstate_header.xcomp_bv[63] indicates that the extended_state_area
// is in compacted format.
//

//
// This is our most modern FPU state format, as saved by the XSAVE
// and restored by the XRSTOR instructions.
//
// It consists of a legacy fxregs portion, an xstate header and
// subsequent areas as defined by the xstate header.  Not all CPUs
// support all the extensions, so the size of the extended area
// can vary quite a bit between CPUs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xregs_state {
    pub i387: fxregs_state,
    pub header: xstate_header,
    pub extended_state_area: [u8; ],
// C attribute field omitted
//
// This is a union of all the possible FPU state formats
// put together, so that we can pick the right one runtime.
//
// The size of the structure is determined by the largest
// member - which is the xsave area.  The padding is there
// to ensure that statically-allocated task_structs (just
// the init_task today) have enough space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union fpregs_state {
    pub fsave: fregs_state,
    pub fxsave: fxregs_state,
    pub soft: swregs_state,
    pub xsave: xregs_state,
    pub __padding: [u8; PAGE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpstate {
// @kernel_size: The size of the kernel register image
    pub size: c_uint,
// @user_size: The size in non-compacted UABI format
    pub user_size: c_uint,
// @xfeatures:		xfeatures for which the storage is sized
    pub xfeatures: u64,
// @user_xfeatures:	xfeatures valid in UABI buffers
    pub user_xfeatures: u64,
// @xfd:		xfeatures disabled to trap userspace use.
    pub xfd: u64,
// @is_valloc:		Indicator for dynamically allocated state
    pub 1: unsigned int is_valloc :,
// @is_guest:		Indicator for guest state (KVM)
    pub 1: unsigned int is_guest :,
//
// @is_confidential:	Indicator for KVM confidential mode.
// The FPU registers are restored by the
// vmentry firmware from encrypted guest
// memory. On vmexit the FPU registers are
// saved by firmware to encrypted guest memory
// and the registers are scrubbed before
// returning to the host. So there is no
// content which is worth saving and restoring.
// The fpstate has to be there so that
// preemption and softirq FPU usage works
// without special casing.
//
    pub 1: unsigned int is_confidential :,
// @in_use:		State is in use
    pub 1: unsigned int in_use :,
// @regs: The register state union for all supported formats
    pub regs: fpregs_state,
// @regs is dynamically sized! Don't add anything after @regs!
    pub __aligned(64): },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpu_state_perm {
//
// @__state_perm:
//
// This bitmap indicates the permission for state components
// available to a thread group, including both user and supervisor
// components and software-defined bits like FPU_GUEST_PERM_LOCKED.
// The permission prctl() sets the enabled state bits in
// thread_group_leader()->thread.fpu.
//
// All run time operations use the per thread information in the
// currently active fpu.fpstate which contains the xfeature masks
// and sizes for kernel and user space.
//
// This master permission field is only to be used when
// task.fpu.fpstate based checks fail to validate whether the task
// is allowed to expand its xfeatures set which requires to
// allocate a larger sized fpstate buffer.
//
// Do not access this field directly.  Use the provided helper
// function. Unlocked access is possible for quick checks.
//
    pub __state_perm: u64,
//
// @__state_size:
//
// The size required for @__state_perm. Only valid to access
// with sighand locked.
//
    pub __state_size: c_uint,
//
// @__user_state_size:
//
// The size required for @__state_perm user part. Only valid to
// access with sighand locked.
//
    pub __user_state_size: c_uint,
}

//
// Highest level per task FPU state data structure that
// contains the FPU register state plus various FPU
// state fields:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpu {
//
// @last_cpu:
//
// Records the last CPU on which this context was loaded into
// FPU registers. (In the lazy-restore case we might be
// able to reuse FPU registers across multiple context switches
// this way, if no intermediate task used the FPU.)
//
// A value of -1 is used to indicate that the FPU state in context
// memory is newer than the FPU state in registers, and that the
// FPU state should be reloaded next time the task is run.
//
    pub last_cpu: c_uint,
//
// @avx512_timestamp:
//
// Records the timestamp of AVX512 use during last context switch.
//
    pub avx512_timestamp: c_ulong,
//
// @fpstate:
//
// Pointer to the active struct fpstate. Initialized to
// point at @__fpstate below.
//
    pub fpstate: *mut fpstate,
//
// @__task_fpstate:
//
// Pointer to an inactive struct fpstate. Initialized to NULL. Is
// used only for KVM support to swap out the regular task fpstate.
//
    pub __task_fpstate: *mut fpstate,
//
// @perm:
//
// Permission related information
//
    pub perm: fpu_state_perm,
//
// @guest_perm:
//
// Permission related information for guest pseudo FPUs
//
    pub guest_perm: fpu_state_perm,
//
// @__fpstate:
//
// Initial in-memory storage for FPU registers which are saved in
// context switch and when the kernel uses the FPU. The registers
// are restored from this storage on return to user space if they
// are not longer containing the tasks FPU register state.
//
    pub __fpstate: fpstate,
//
// WARNING: '__fpstate' is dynamically-sized.  Do not put
// anything after it here.
//
}

//
// Guest pseudo FPU container
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpu_guest {
//
// @xfeatures:			xfeature bitmap of features which are
// currently enabled for the guest vCPU.
//
    pub xfeatures: u64,
//
// @xfd_err:			Save the guest value.
//
    pub xfd_err: u64,
//
// @uabi_size:			Size required for save/restore
//
    pub uabi_size: c_uint,
//
// @fpstate:			Pointer to the allocated guest fpstate
//
    pub fpstate: *mut fpstate,
}

//
// FPU state configuration data for fpu_guest.
// Initialized at boot time. Read only after init.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_fpu_config {
//
// @size:
//
// The default size of the register state buffer in guest FPUs.
// Includes all supported features except independent managed
// features and features which have to be requested by user space
// before usage.
//
    pub size: c_uint,
//
// @features:
//
// The default supported features bitmap in guest FPUs. Does not
// include independent managed features and features which have to
// be requested by user space before usage.
//
    pub features: u64,
}

//
// FPU state configuration data. Initialized at boot time. Read only after init.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpu_state_config {
//
// @max_size:
//
// The maximum size of the register state buffer. Includes all
// supported features except independent managed features.
//
    pub max_size: c_uint,
//
// @default_size:
//
// The default size of the register state buffer. Includes all
// supported features except independent managed features,
// guest-only features and features which have to be requested by
// user space before usage.
//
    pub default_size: c_uint,
//
// @max_features:
//
// The maximum supported features bitmap. Does not include
// independent managed features.
//
    pub max_features: u64,
//
// @default_features:
//
// The default supported features bitmap. Does not include
// independent managed features, guest-only features and features
// which have to be requested by user space before usage.
//
    pub default_features: u64,
//
// @legacy_features:
//
// Features which can be reported back to user space
// even without XSAVE support, i.e. legacy features FP + SSE
//
    pub legacy_features: u64,
//
// @independent_features:
//
// Features that are supported by XSAVES, but not managed as part of
// the FPU core, such as LBR
//
    pub independent_features: u64,
}

// FPU state configuration information
