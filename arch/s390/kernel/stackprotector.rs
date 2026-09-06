//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/stackprotector.c
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

    int __bootdata_preserved(stack_protector_debug);
    unsigned long __stack_chk_guard;
    EXPORT_SYMBOL(__stack_chk_guard);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct insn_ril {
    pub 8: u8 opc1 :,
    pub 4: u8 r1 :,
    pub 4: u8 opc2 :,
    pub imm: u32,
    pub __packed: },
//
// Convert a virtual instruction address to a real instruction address. The
// decompressor needs to patch instructions within the kernel image based on
// their virtual addresses, while dynamic address translation is still
// disabled. Therefore a translation from virtual kernel image addresses to
// the corresponding physical addresses is required.
//
// After dynamic address translation is enabled and when the kernel needs to
// patch instructions such a translation is not required since the addresses
// are identical.
//
    static struct insn_ril *vaddress_to_insn(unsigned long vaddress)
    {

    pub )__kernel_pa(vaddress): *mut return (struct insn_ril,

    pub )vaddress: *mut return (struct insn_ril,

    }
#[no_mangle]
unsafe extern "C" fn insn_to_vaddress(insn: *mut insn_ril) -> c_ulong {
    static unsigned long insn_to_vaddress(struct insn_ril *insn)
    {

    pub long)__kernel_va(insn): return (unsigned,

    pub long)insn: return (unsigned,

    }

#[no_mangle]
unsafe extern "C" fn insn_ril_to_string(str: *mut c_char, insn: *mut insn_ril) {
    static void insn_ril_to_string(char *str, struct insn_ril *insn)
    {
    pub )insn: *mut *mut u8 ptr = (u8,
    pub i: c_int,
    pub i++): *mut *mut for (i = 0; i < sizeof(insn);,
    pub ptr[i]): *mut *mut hex_byte_pack(&str[2  i],,
    pub 0: *mut *mut str[2  i] =,
    }
#[no_mangle]
unsafe extern "C" fn stack_protector_dump(old: *mut insn_ril, new: *mut insn_ril) {
    static void stack_protector_dump(struct insn_ril *old, struct insn_ril *new)
    {
    pub ostr: [c_char; INSN_RIL_STRING_SIZE],
    pub nstr: [c_char; INSN_RIL_STRING_SIZE],
    pub old): insn_ril_to_string(ostr,,
    pub new): insn_ril_to_string(nstr,,
    pub nstr): DEBUGP("%016lx: %s -> %s\n", insn_to_vaddress(old), ostr,,
    }
#[no_mangle]
unsafe extern "C" fn stack_protector_verify(insn: *mut insn_ril, kernel_start: c_ulong) -> c_int {
    static int stack_protector_verify(struct insn_ril *insn, unsigned long kernel_start)
    {
    pub istr: [c_char; INSN_RIL_STRING_SIZE],
    pub offset: unsigned long vaddress,,
// larl
    if (insn.opc1 == 0xc0 && insn.opc2 == 0x0)
    pub 0: return,
// lgrl
    if (insn.opc1 == 0xc4 && insn.opc2 == 0x8)
    pub 0: return,
    pub insn): insn_ril_to_string(istr,,
    pub insn_to_vaddress(insn): vaddress =,
    if (__is_defined(__DECOMPRESSOR)) {
    pub TEXT_OFFSET: offset = (unsigned long)insn - kernel_start +,
    pub istr): EMERGP("Unexpected instruction at %016lx/%016lx: %s\n", vaddress, offset,,
    pub error\n"): PANIC("Stackprotector,
    } else {
    pub istr): EMERGP("Unexpected instruction at %016lx: %s\n", vaddress,,
    }
    pub -EINVAL: return,
    }
#[no_mangle]
pub unsafe extern "C" fn __stack_protector_apply(start: *mut c_ulong, end: *mut c_ulong, kernel_start: c_ulong) -> c_int {
    int __stack_protector_apply(unsigned long *start, unsigned long *end, unsigned long kernel_start)
    {
    pub loc: *mut unsigned long canary,,
    pub new: *mut *mut insn_ril insn,,
    pub rc: c_int,
//
// Convert LARL/LGRL instructions to LLILF so register R1 contains the
// address of the per-cpu / per-process stack canary:
//
// LARL/LGRL R1,__stack_chk_guard => LLILF R1,__lc_stack_canary
//
    pub __LC_STACK_CANARY: canary =,
    if (machine_has_relocated_lowcore())
    pub LOWCORE_ALT_ADDRESS: canary +=,
    pub {: for (loc = start; loc < end; loc++),
    pub vaddress_to_insn(*loc): *mut insn =,
    pub kernel_start): rc = stack_protector_verify(insn,,
    if (rc)
    pub rc: return,
    pub insn: *mut new =,
    pub 0xc0: new.opc1 =,
    pub 0xf: new.opc2 =,
    pub canary: new.imm =,
    if (stack_protector_debug)
    pub &new): stack_protector_dump(insn,,
    pub sizeof(*insn)): *mut s390_kernel_write(insn, &new,,
    }
    pub 0: return,
    }

#[no_mangle]
pub unsafe extern "C" fn __stack_protector_apply_early(kernel_start: c_ulong) {
    void __stack_protector_apply_early(unsigned long kernel_start)
    {
    pub end: *mut *mut unsigned long start,,
    pub )vmlinux.stack_prot_start: *mut start = (unsigned long,
    pub )vmlinux.stack_prot_end: *mut end = (unsigned long,
    pub kernel_start): __stack_protector_apply(start, end,,
    }
