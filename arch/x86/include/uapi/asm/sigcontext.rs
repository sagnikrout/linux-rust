//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/sigcontext.h
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
//
// Linux signal context definitions. The sigcontext includes a complex
// hierarchy of CPU and FPU state, available to user-space (on the stack) when
// a signal handler is executed.
//
// As over the years this ABI grew from its very simple roots towards
// supporting more and more CPU state organically, some of the details (which
// were rather clever hacks back in the days) became a bit quirky by today.
//
// The current ABI includes flexible provisions for future extensions, so we
// won't have to grow new quirks for quite some time. Promise!
//

pub const FP_XSTATE_MAGIC1: c_uint = 0x46505853U;
pub const FP_XSTATE_MAGIC2: c_uint = 0x46505845U;

//
// Bytes 464..511 in the current 512-byte layout of the FXSAVE/FXRSTOR frame
// are reserved for SW usage. On CPUs supporting XSAVE/XRSTOR, these bytes are
// used to extend the fpstate pointer in the sigcontext, which now includes the
// extended state information along with fpstate information.
//
// If sw_reserved.magic1 == FP_XSTATE_MAGIC1 then there's a
// sw_reserved.extended_size bytes large extended context area present. (The
// last 32-bit word of this extended area (at the
// fpstate+extended_size-FP_XSTATE_MAGIC2_SIZE address) is set to
// FP_XSTATE_MAGIC2 so that you can sanity check your size calculations.)
//
// This extended area typically grows with newer CPUs that have larger and
// larger XSAVE areas.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _fpx_sw_bytes {
//
// If set to FP_XSTATE_MAGIC1 then this is an xstate context.
// 0 if a legacy frame.
//
    pub magic1: __u32,
//
// Total size of the fpstate area:
//
// - if magic1 == 0 then it's sizeof(struct _fpstate)
// - if magic1 == FP_XSTATE_MAGIC1 then it's sizeof(struct _xstate)
// plus extensions (if any)
//
    pub extended_size: __u32,
//
// Feature bit mask (including FP/SSE/extended state) that is present
// in the memory layout:
//
    pub xfeatures: __u64,
//
// Actual XSAVE state size, based on the xfeatures saved in the layout.
// 'extended_size' is greater than 'xstate_size':
//
    pub xstate_size: __u32,
// For future use:
    pub padding: [__u32; 7],
}

//
// As documented in the iBCS2 standard:
//
// The first part of "struct _fpstate" is just the normal i387 hardware setup,
// the extra "status" word is used to save the coprocessor status word before
// entering the handler.
//
// The FPU state data structure has had to grow to accommodate the extended FPU
// state required by the Streaming SIMD Extensions.  There is no documented
// standard to accomplish this at the moment.
//
// 10-byte legacy floating point register:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _fpreg {
    pub significand: [__u16; 4],
    pub exponent: __u16,
}

// 16-byte floating point register:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _fpxreg {
    pub significand: [__u16; 4],
    pub exponent: __u16,
    pub padding: [__u16; 3],
}

// 16-byte XMM register:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xmmreg {
    pub element: [__u32; 4],
}

pub const X86_FXSR_MAGIC: c_uint = 0x0000;
//
// The 32-bit FPU frame:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _fpstate_32 {
// Legacy FPU environment:
    pub cw: __u32,
    pub sw: __u32,
    pub tag: __u32,
    pub ipoff: __u32,
    pub cssel: __u32,
    pub dataoff: __u32,
    pub datasel: __u32,
    pub _st: [_fpreg; 8],
    pub status: __u16,
    pub /: *mut *mut __u16 magic; / 0xffff: regular FPU data only,
// 0x0000: FXSR FPU data
// FXSR FPU environment
    pub /: *mut *mut __u32 _fxsr_env[6]; / FXSR FPU env is ignored,
    pub mxcsr: __u32,
    pub reserved: __u32,
    pub /: *mut *mut _fpxreg _fxsr_st[8]; / FXSR FPU reg data is ignored,
    pub /: *mut *mut _xmmreg _xmm[8]; / First 8 XMM registers,
    pub /: *mut *mut __u32 padding1[44]; / Second 8 XMM registers plus padding,
    pub /: *mut *mut __u32 padding[44]; / Alias name for old user-space,
}

//
// The 64-bit FPU frame. (FXSAVE format and later)
//
// Note1: If sw_reserved.magic1 == FP_XSTATE_MAGIC1 then the structure is
// larger: 'struct _xstate'. Note that 'struct _xstate' embeds
// 'struct _fpstate' so that you can always assume the _fpstate portion
// exists so that you can check the magic value.
//
// Note2: Reserved fields may someday contain valuable data. Always
// save/restore them when you change signal frames.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _fpstate_64 {
    pub cwd: __u16,
    pub swd: __u16,
// Note this is not the same as the 32-bit/x87/FSAVE twd:
    pub twd: __u16,
    pub fop: __u16,
    pub rip: __u64,
    pub rdp: __u64,
    pub mxcsr: __u32,
    pub mxcsr_mask: __u32,
    pub /: *mut *mut __u32 st_space[32]; / 8x FP registers, 16 bytes each,
    pub /: *mut *mut __u32 xmm_space[64]; / 16x XMM registers, 16 bytes each,
    pub reserved2: [__u32; 12],
    pub reserved3: [__u32; 12],
    pub /: *mut *mut _fpx_sw_bytes sw_reserved; / Potential extended state is encoded here,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _header {
    pub xfeatures: __u64,
    pub reserved1: [__u64; 2],
    pub reserved2: [__u64; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _ymmh_state {
// 16x YMM registers, 16 bytes each:
    pub ymmh_space: [__u32; 64],
}

//
// Extended state pointed to by sigcontext::fpstate.
//
// In addition to the fpstate, information encoded in _xstate::xstate_hdr
// indicates the presence of other extended state information supported
// by the CPU and kernel:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _xstate {
    pub fpstate: _fpstate,
    pub xstate_hdr: _header,
    pub ymmh: _ymmh_state,
// New processor state extensions go here:
}

//
// The 32-bit signal frame:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext_32 {
    pub __gsh: __u16 gs,,
    pub __fsh: __u16 fs,,
    pub __esh: __u16 es,,
    pub __dsh: __u16 ds,,
    pub di: __u32,
    pub si: __u32,
    pub bp: __u32,
    pub sp: __u32,
    pub bx: __u32,
    pub dx: __u32,
    pub cx: __u32,
    pub ax: __u32,
    pub trapno: __u32,
    pub err: __u32,
    pub ip: __u32,
    pub __csh: __u16 cs,,
    pub flags: __u32,
    pub sp_at_signal: __u32,
    pub __ssh: __u16 ss,,
//
// fpstate is really (struct _fpstate *) or (struct _xstate *)
// depending on the FP_XSTATE_MAGIC1 encoded in the SW reserved
// bytes of (struct _fpstate) and FP_XSTATE_MAGIC2 present at the end
// of extended memory layout. See comments at the definition of
// (struct _fpx_sw_bytes)
//
    pub /: *mut *mut __u32 fpstate; / Zero when no FPU/extended context,
    pub oldmask: __u32,
    pub cr2: __u32,
}

//
// The 64-bit signal frame:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext_64 {
    pub r8: __u64,
    pub r9: __u64,
    pub r10: __u64,
    pub r11: __u64,
    pub r12: __u64,
    pub r13: __u64,
    pub r14: __u64,
    pub r15: __u64,
    pub di: __u64,
    pub si: __u64,
    pub bp: __u64,
    pub bx: __u64,
    pub dx: __u64,
    pub ax: __u64,
    pub cx: __u64,
    pub sp: __u64,
    pub ip: __u64,
    pub flags: __u64,
    pub cs: __u16,
    pub gs: __u16,
    pub fs: __u16,
    pub ss: __u16,
    pub err: __u64,
    pub trapno: __u64,
    pub oldmask: __u64,
    pub cr2: __u64,
//
// fpstate is really (struct _fpstate *) or (struct _xstate *)
// depending on the FP_XSTATE_MAGIC1 encoded in the SW reserved
// bytes of (struct _fpstate) and FP_XSTATE_MAGIC2 present at the end
// of extended memory layout. See comments at the definition of
// (struct _fpx_sw_bytes)
//
    pub /: *mut *mut __u64 fpstate; / Zero when no FPU/extended context,
    pub reserved1: [__u64; 8],
}

//
// Create the real 'struct sigcontext' type:
//

//
// The old user-space sigcontext definition, just in case user-space still
// relies on it. The kernel definition (in asm/sigcontext.h) has unified
// field names but otherwise the same layout.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext {
    pub __gsh: __u16 gs,,
    pub __fsh: __u16 fs,,
    pub __esh: __u16 es,,
    pub __dsh: __u16 ds,,
    pub edi: __u32,
    pub esi: __u32,
    pub ebp: __u32,
    pub esp: __u32,
    pub ebx: __u32,
    pub edx: __u32,
    pub ecx: __u32,
    pub eax: __u32,
    pub trapno: __u32,
    pub err: __u32,
    pub eip: __u32,
    pub __csh: __u16 cs,,
    pub eflags: __u32,
    pub esp_at_signal: __u32,
    pub __ssh: __u16 ss,,
    pub fpstate: *mut _fpstate __user,
    pub oldmask: __u32,
    pub cr2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext {
    pub r8: __u64,
    pub r9: __u64,
    pub r10: __u64,
    pub r11: __u64,
    pub r12: __u64,
    pub r13: __u64,
    pub r14: __u64,
    pub r15: __u64,
    pub rdi: __u64,
    pub rsi: __u64,
    pub rbp: __u64,
    pub rbx: __u64,
    pub rdx: __u64,
    pub rax: __u64,
    pub rcx: __u64,
    pub rsp: __u64,
    pub rip: __u64,
    pub /: *mut *mut __u64 eflags; / RFLAGS,
    pub cs: __u16,
//
// Prior to 2.5.64 ("[PATCH] x86-64 updates for 2.5.64-bk3"),
// Linux saved and restored fs and gs in these slots.  This
// was counterproductive, as fsbase and gsbase were never
// saved, so arch_prctl was presumably unreliable.
//
// These slots should never be reused without extreme caution:
//
// - Some DOSEMU versions stash fs and gs in these slots manually,
// thus overwriting anything the kernel expects to be preserved
// in these slots.
//
// - If these slots are ever needed for any other purpose,
// there is some risk that very old 64-bit binaries could get
// confused.  I doubt that many such binaries still work,
// though, since the same patch in 2.5.64 also removed the
// 64-bit set_thread_area syscall, so it appears that there
// is no TLS API beyond modify_ldt that works in both pre-
// and post-2.5.64 kernels.
//
// If the kernel ever adds explicit fs, gs, fsbase, and gsbase
// save/restore, it will most likely need to be opt-in and use
// different context slots.
//
    pub gs: __u16,
    pub fs: __u16,
    pub /: *mut *mut __u16 ss; / If UC_SIGCONTEXT_SS,
    pub /: *mut *mut __u16 __pad0; / Alias name for old (!UC_SIGCONTEXT_SS) user-space,
}

