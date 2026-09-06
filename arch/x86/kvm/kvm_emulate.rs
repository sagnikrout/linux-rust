//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/kvm_emulate.h
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
// x86_emulate.h
//
// Generic x86 (32-bit and 64-bit) instruction decoder and emulator.
//
// Copyright (c) 2005 Keir Fraser
//
// From: xen-unstable 10676:af9809f51f81a3c43f276f00c81a52ef558afda4
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_exception {
    pub vector: u8,
    pub error_code_valid: bool,
    pub error_code: u64,
    pub nested_page_fault: bool,
    pub /: *mut *mut u64 address; / cr2 or nested page fault gpa,
    pub dr6: c_ulong,
    pub payload: u64,
}

//
// This struct is used to carry enough information from the instruction
// decoder to main KVM so that a decision can be made whether the
// instruction needs to be intercepted or not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_instruction_info {
    pub /: *mut *mut u8 intercept; / which intercept,
    pub /: *mut *mut u8 rep_prefix; / rep prefix?,
    pub /: *mut *mut u8 modrm_mod; / mod part of modrm,
    pub /: *mut *mut u8 modrm_reg; / index of register used,
    pub /: *mut *mut u8 modrm_rm; / rm part of modrm,
    pub /: *mut *mut u64 src_val; / value of source operand,
    pub /: *mut *mut u64 dst_val; / value of destination operand,
    pub /: *mut *mut u8 src_bytes; / size of source operand,
    pub /: *mut *mut u8 dst_bytes; / size of destination operand,
    pub /: *mut *mut u8 src_type; / type of source operand,
    pub /: *mut *mut u8 dst_type; / type of destination operand,
    pub /: *mut *mut u8 ad_bytes; / size of src/dst address,
    pub /: *mut *mut u64 rip; / rip of the instruction,
    pub /: *mut *mut u64 next_rip; / rip following the instruction,
}

//
// x86_emulate_ops:
//
// These operations represent the instruction emulator's interface to memory.
// There are two categories of operation: those that act on ordinary memory
// regions (*_std), and those that act on memory regions known to require
// special treatment or emulation (*_emulated).
//
// The emulator assumes that an instruction accesses only one 'emulated memory'
// location, that this location is the given linear faulting address (cr2), and
// that this is one of the instruction's data operands. Instruction fetches and
// stack operations are assumed never to access emulated memory. The emulator
// automatically deduces which operand of a string-move operation is accessing
// emulated memory, and assumes that the other operand accesses normal memory.
//
// NOTES:
// 1. The emulator isn't very smart about emulated vs. standard memory.
// 'Emulated memory' access addresses should be checked for sanity.
// 'Normal memory' accesses may fault, and the caller must arrange to
// detect and handle reentrancy into the emulator via recursive faults.
// Accesses may be unaligned and may cross page boundaries.
// 2. If the access fails (cannot emulate, or a standard access faults) then
// it is up to the memop to propagate the fault to the guest VM via
// some out-of-band mechanism, unknown to the emulator. The memop signals
// failure by returning X86EMUL_PROPAGATE_FAULT to the emulator, which will
// then immediately bail.
// 3. Valid access sizes are 1, 2, 4 and 8 bytes. On x86/32 systems only
// cmpxchg8b_emulated need support 8-byte accesses.
// 4. The emulator cannot handle 64-bit mode emulation on an x86/32 system.
//
// Access completed successfully: continue emulation as normal.
pub const X86EMUL_CONTINUE: c_int = 0;
// Access is unhandleable: bail from emulation and return error to caller.
pub const X86EMUL_UNHANDLEABLE: c_int = 1;
// Terminate emulation but return success to the caller.

// Emulation during event vectoring is unhandleable.
pub const X86EMUL_UNHANDLEABLE_VECTORING: c_int = 7;
// x86-specific emulation flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_emulate_ops {
    pub ctxt): *mut *mut void (vm_bugged)(struct x86_emulate_ctxt,
//
// read_gpr: read a general purpose register (rax - r15)
//
// @reg: gpr number.
//
    pub reg): *mut *mut *mut ulong (read_gpr)(struct x86_emulate_ctxt ctxt, unsigned,
//
// write_gpr: write a general purpose register (rax - r15)
//
// @reg: gpr number.
// @val: value to write.
//
    pub val): *mut *mut *mut void (write_gpr)(struct x86_emulate_ctxt ctxt, unsigned reg, ulong,
//
// read_std: Read bytes of standard (non-emulated/special) memory.
// Used for descriptor reading.
// @addr:  [IN ] Linear address from which to read.
// @val:   [OUT] Value read from memory, zero-extended to 'u_long'.
// @bytes: [IN ] Number of bytes to read from memory.
// @system:[IN ] Whether the access is forced to be at CPL0.
//
    pub system): *mut *mut x86_exception fault, bool,
//
// write_std: Write bytes of standard (non-emulated/special) memory.
// Used for descriptor writing.
// @addr:  [IN ] Linear address to which to write.
// @val:   [OUT] Value write to memory, zero-extended to 'u_long'.
// @bytes: [IN ] Number of bytes to write to memory.
// @system:[IN ] Whether the access is forced to be at CPL0.
//
    pub system): *mut *mut x86_exception fault, bool,
//
// fetch: Read bytes of standard (non-emulated/special) memory.
// Used for instruction fetch.
// @addr:  [IN ] Linear address from which to read.
// @val:   [OUT] Value read from memory, zero-extended to 'u_long'.
// @bytes: [IN ] Number of bytes to read from memory.
//
    pub fault): *mut x86_exception,
//
// read_emulated: Read bytes from emulated/special memory area.
// @addr:  [IN ] Linear address from which to read.
// @val:   [OUT] Value read from memory, zero-extended to 'u_long'.
// @bytes: [IN ] Number of bytes to read from memory.
//
    pub fault): *mut x86_exception,
//
// write_emulated: Write bytes to emulated/special memory area.
// @addr:  [IN ] Linear address to which to write.
// @val:   [IN ] Value to write to memory (low-order bytes used as
// required).
// @bytes: [IN ] Number of bytes to write to memory.
//
    pub fault): *mut x86_exception,
//
// cmpxchg_emulated: Emulate an atomic (LOCKed) CMPXCHG operation on an
// emulated/special memory area.
// @addr:  [IN ] Linear address to access.
// @old:   [IN ] Value expected to be current at @addr.
// @new:   [IN ] Value to write to @addr.
// @bytes: [IN ] Number of bytes to access using CMPXCHG.
//
    pub fault): *mut x86_exception,
    pub addr): *mut *mut *mut void (invlpg)(struct x86_emulate_ctxt ctxt, ulong,
    pub count): c_uint,
    pub count): c_uint,
    pub seg): *mut *mut *mut desc_desc, u32 base3, int,
    pub seg): *mut *mut desc_desc, u32 base3, int,
    pub seg): c_int,
    pub dt): *mut *mut *mut void (get_gdt)(struct x86_emulate_ctxt ctxt, struct desc_ptr,
    pub dt): *mut *mut *mut void (get_idt)(struct x86_emulate_ctxt ctxt, struct desc_ptr,
    pub dt): *mut *mut *mut void (set_gdt)(struct x86_emulate_ctxt ctxt, struct desc_ptr,
    pub dt): *mut *mut *mut void (set_idt)(struct x86_emulate_ctxt ctxt, struct desc_ptr,
    pub cr): *mut *mut *mut ulong (get_cr)(struct x86_emulate_ctxt ctxt, int,
    pub val): *mut *mut *mut int (set_cr)(struct x86_emulate_ctxt ctxt, int cr, ulong,
    pub ctxt): *mut *mut int (cpl)(struct x86_emulate_ctxt,
    pub ctxt): *mut *mut ulong (get_effective_dr7)(struct x86_emulate_ctxt,
    pub dr): *mut *mut *mut ulong (get_dr)(struct x86_emulate_ctxt ctxt, int,
    pub value): *mut *mut *mut int (set_dr)(struct x86_emulate_ctxt ctxt, int dr, ulong,
    pub data): *mut *mut *mut int (set_msr_with_filter)(struct x86_emulate_ctxt ctxt, u32 msr_index, u64,
    pub pdata): *mut *mut *mut int (get_msr_with_filter)(struct x86_emulate_ctxt ctxt, u32 msr_index, u64,
    pub pdata): *mut *mut *mut int (get_msr)(struct x86_emulate_ctxt ctxt, u32 msr_index, u64,
    pub pmc): *mut *mut *mut int (check_rdpmc_early)(struct x86_emulate_ctxt ctxt, u32,
    pub pdata): *mut *mut *mut int (read_pmc)(struct x86_emulate_ctxt ctxt, u32 pmc, u64,
    pub ctxt): *mut *mut void (halt)(struct x86_emulate_ctxt,
    pub ctxt): *mut *mut void (wbinvd)(struct x86_emulate_ctxt,
    pub ctxt): *mut *mut int (fix_hypercall)(struct x86_emulate_ctxt,
    pub stage): x86_intercept_stage,
    pub ctxt): *mut *mut bool (is_cpuid_allowed)(struct x86_emulate_ctxt,
    pub exact_only): *mut *mut *mut u32 ecx, u32 edx, bool,
    pub ctxt): *mut *mut bool (guest_has_movbe)(struct x86_emulate_ctxt,
    pub ctxt): *mut *mut bool (guest_has_fxsr)(struct x86_emulate_ctxt,
    pub ctxt): *mut *mut bool (guest_has_rdpid)(struct x86_emulate_ctxt,
    pub ctxt): *mut *mut bool (guest_cpuid_is_intel_compatible)(struct x86_emulate_ctxt,
    pub masked): *mut *mut *mut void (set_nmi_mask)(struct x86_emulate_ctxt ctxt, bool,
    pub ctxt): *mut *mut bool (is_smm)(struct x86_emulate_ctxt,
    pub ctxt): *mut *mut int (leave_smm)(struct x86_emulate_ctxt,
    pub ctxt): *mut *mut void (triple_fault)(struct x86_emulate_ctxt,
    pub xcr): *mut *mut *mut int (get_xcr)(struct x86_emulate_ctxt ctxt, u32 index, u64,
    pub xcr): *mut *mut *mut int (set_xcr)(struct x86_emulate_ctxt ctxt, u32 index, u64,
    pub flags): c_uint,
    pub flags): c_uint,
    pub gpa): *mut *mut *mut bool (page_address_valid)(struct x86_emulate_ctxt ctxt, gpa_t,
}

// Type, address-of, and value of an instruction's operand.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct operand {
    pub type: { OP_REG, OP_MEM, OP_MEM_STR, OP_IMM, OP_XMM, OP_YMM, OP_MM, OP_NONE },
    pub bytes: c_uint,
    pub count: c_uint,
    pub orig_val: c_ulong,
    pub orig_val64: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct segmented_address {
    pub ea: c_ulong,
    pub seg: unsigned,
    pub mem: },
    pub xmm: unsigned,
    pub mm: unsigned,
    pub addr: },
    pub val: c_ulong,
    pub val64: u64,
    pub valptr: [c_char; sizeof(avx256_t)],
    pub vec_val: sse128_t,
    pub vec_val2: avx256_t,
    pub mm_val: u64,
    pub data: *mut c_void,
    pub __aligned(32): },
}

pub const X86_MAX_INSTRUCTION_LENGTH: c_int = 15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fetch_cache {
    pub data: [u8; X86_MAX_INSTRUCTION_LENGTH],
    pub ptr: *mut u8,
    pub end: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_cache {
    pub data: [u8; 1024],
    pub pos: c_ulong,
    pub end: c_ulong,
}

// Execution mode, passed to the emulator.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86emul_mode {
    X86EMUL_MODE_REAL,	/* Real mode.             */
    X86EMUL_MODE_VM86,	/* Virtual 8086 mode.     */
    X86EMUL_MODE_PROT16,	/* 16-bit protected mode. */
    X86EMUL_MODE_PROT32,	/* 32-bit protected mode. */
    X86EMUL_MODE_PROT64,	/* 64-bit (long) mode.    */
}

//
// fastop functions are declared as taking a never-defined fastop parameter,
// so they can't be called from C directly.
//
extern "C" {
    pub fn void(: *mut *mut fastop_t)(struct fastop) -> typedef;
}
//
// The emulator's _regs array tracks only the GPRs, i.e. excludes RIP.  RIP is
// tracked/accessed via _eip, and except for RIP relative addressing, which
// also uses _eip, RIP cannot be a register operand nor can it be an operand in
// a ModRM or SIB byte.
//

pub const NR_EMULATOR_GPRS: c_int = 16;

pub const NR_EMULATOR_GPRS: c_int = 8;

//
// Distinguish between no prefix, REX, or in the future REX2.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rex_type {
    REX_NONE,
    REX_PREFIX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_emulate_ctxt {
    pub vcpu: *mut c_void,
    pub ops: *const x86_emulate_ops,
// Register state before/after emulation.
    pub eflags: c_ulong,
    pub /: *mut *mut unsigned long eip; / eip before instruction emulation,
// Emulated execution mode, represented by an X86EMUL_MODE value.
    pub mode: x86emul_mode,
// interruptibility state, as a result of execution of STI or MOV SS
    pub interruptibility: c_int,
    pub /: *mut *mut bool perm_ok; / do not check permissions if true,
    pub /: *mut *mut bool tf; / TF value before instruction (after for syscall/sysret),
    pub have_exception: bool,
    pub exception: x86_exception,
// GPA available
    pub gpa_available: bool,
    pub gpa_val: gpa_t,
//
// decode cache
//
// current opcode length in bytes
    pub opcode_len: u8,
    pub b: u8,
    pub intercept: u8,
    pub op_prefix: bool,
    pub op_bytes: u8,
    pub ad_bytes: u8,
    pub ctxt): *mut *mut int (execute)(struct x86_emulate_ctxt,
    pub fop: fastop_t,
}

// bitmaps of registers in _regs[] that can be read
// bitmaps of registers in _regs[] that have been written
// modrm
// Here begins the usercopy section.

// Repeat String Operation Prefix
pub const REPE_PREFIX: c_uint = 0xf3;
pub const REPNE_PREFIX: c_uint = 0xf2;
// CPUID vendors
pub const X86EMUL_CPUID_VENDOR_AuthenticAMD_ebx: c_uint = 0x68747541;
pub const X86EMUL_CPUID_VENDOR_AuthenticAMD_ecx: c_uint = 0x444d4163;
pub const X86EMUL_CPUID_VENDOR_AuthenticAMD_edx: c_uint = 0x69746e65;
pub const X86EMUL_CPUID_VENDOR_AMDisbetterI_ebx: c_uint = 0x69444d41;
pub const X86EMUL_CPUID_VENDOR_AMDisbetterI_ecx: c_uint = 0x21726574;
pub const X86EMUL_CPUID_VENDOR_AMDisbetterI_edx: c_uint = 0x74656273;
pub const X86EMUL_CPUID_VENDOR_HygonGenuine_ebx: c_uint = 0x6f677948;
pub const X86EMUL_CPUID_VENDOR_HygonGenuine_ecx: c_uint = 0x656e6975;
pub const X86EMUL_CPUID_VENDOR_HygonGenuine_edx: c_uint = 0x6e65476e;
pub const X86EMUL_CPUID_VENDOR_GenuineIntel_ebx: c_uint = 0x756e6547;
pub const X86EMUL_CPUID_VENDOR_GenuineIntel_ecx: c_uint = 0x6c65746e;
pub const X86EMUL_CPUID_VENDOR_GenuineIntel_edx: c_uint = 0x49656e69;
pub const X86EMUL_CPUID_VENDOR_CentaurHauls_ebx: c_uint = 0x746e6543;
pub const X86EMUL_CPUID_VENDOR_CentaurHauls_ecx: c_uint = 0x736c7561;
pub const X86EMUL_CPUID_VENDOR_CentaurHauls_edx: c_uint = 0x48727561;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_intercept_stage {
    X86_ICTP_NONE = 0,   /* Allow zero-init to not match anything */
    X86_ICPT_PRE_EXCEPT,
    X86_ICPT_POST_EXCEPT,
    X86_ICPT_POST_MEMACCESS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_intercept {
    x86_intercept_none,
    x86_intercept_cr_read,
    x86_intercept_cr_write,
    x86_intercept_clts,
    x86_intercept_lmsw,
    x86_intercept_smsw,
    x86_intercept_dr_read,
    x86_intercept_dr_write,
    x86_intercept_lidt,
    x86_intercept_sidt,
    x86_intercept_lgdt,
    x86_intercept_sgdt,
    x86_intercept_lldt,
    x86_intercept_sldt,
    x86_intercept_ltr,
    x86_intercept_str,
    x86_intercept_rdtsc,
    x86_intercept_rdpmc,
    x86_intercept_pushf,
    x86_intercept_popf,
    x86_intercept_cpuid,
    x86_intercept_rsm,
    x86_intercept_iret,
    x86_intercept_intn,
    x86_intercept_invd,
    x86_intercept_pause,
    x86_intercept_hlt,
    x86_intercept_invlpg,
    x86_intercept_invlpga,
    x86_intercept_vmrun,
    x86_intercept_vmload,
    x86_intercept_vmsave,
    x86_intercept_vmmcall,
    x86_intercept_stgi,
    x86_intercept_clgi,
    x86_intercept_skinit,
    x86_intercept_rdtscp,
    x86_intercept_rdpid,
    x86_intercept_icebp,
    x86_intercept_wbinvd,
    x86_intercept_monitor,
    x86_intercept_mwait,
    x86_intercept_rdmsr,
    x86_intercept_wrmsr,
    x86_intercept_in,
    x86_intercept_ins,
    x86_intercept_out,
    x86_intercept_outs,
    x86_intercept_xsetbv,

    nr_x86_intercepts
}

extern "C" {
    pub fn x86_decode_insn(ctxt: *mut x86_emulate_ctxt, insn: *mut c_void, insn_len: c_int, emulation_type: c_int) -> c_int;
}
extern "C" {
    pub fn x86_page_table_writing_insn(ctxt: *mut x86_emulate_ctxt) -> bool;
}

pub const EMULATION_OK: c_int = 0;
pub const EMULATION_RESTART: c_int = 1;
pub const EMULATION_INTERCEPTED: c_int = 2;
extern "C" {
    pub fn init_decode_cache(ctxt: *mut x86_emulate_ctxt);
}
extern "C" {
    pub fn x86_emulate_insn(ctxt: *mut x86_emulate_ctxt, check_intercepts: bool) -> c_int;
}
extern "C" {
    pub fn emulate_int_real(ctxt: *mut x86_emulate_ctxt, irq: c_int) -> c_int;
}
extern "C" {
    pub fn emulator_invalidate_register_cache(ctxt: *mut x86_emulate_ctxt);
}
extern "C" {
    pub fn emulator_writeback_register_cache(ctxt: *mut x86_emulate_ctxt);
}
extern "C" {
    pub fn emulator_can_use_gpa(ctxt: *mut x86_emulate_ctxt) -> bool;
}
extern "C" {
    pub fn reg_write(_arg: ctxt, _arg: nr) -> return;
}
