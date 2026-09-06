//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/kprobes/core.c
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
// Kernel Probes (KProbes)
//
// Copyright (C) IBM Corporation, 2002, 2004
//
// 2002-Oct	Created by Vamsi Krishna S <vamsi_krishna@in.ibm.com> Kernel
// Probes initial implementation ( includes contributions from
// Rusty Russell).
// 2004-July	Suparna Bhattacharya <suparna@in.ibm.com> added jumper probes
// interface to access function arguments.
// 2004-Oct	Jim Keniston <jkenisto@us.ibm.com> and Prasanna S Panchamukhi
// <prasanna@in.ibm.com> adapted for x86_64 from i386.
// 2005-Mar	Roland McGrath <roland@redhat.com>
// Fixed to handle %rip-relative addressing mode correctly.
// 2005-May	Hien Nguyen <hien@us.ibm.com>, Jim Keniston
// <jkenisto@us.ibm.com> and Prasanna S Panchamukhi
// <prasanna@in.ibm.com> added function-return probes.
// 2005-May	Rusty Lynch <rusty.lynch@intel.com>
// Added function return probes functionality
// 2006-Feb	Masami Hiramatsu <hiramatu@sdl.hitachi.co.jp> added
// kprobe-booster and kretprobe-booster for i386.
// 2007-Dec	Masami Hiramatsu <mhiramat@redhat.com> added kprobe-booster
// and kretprobe-booster for x86-64
// 2007-Dec	Masami Hiramatsu <mhiramat@redhat.com>, Arjan van de Ven
// <arjan@infradead.org> and Jim Keniston <jkenisto@us.ibm.com>
// unified x86 kprobes code.
//

    DEFINE_PER_CPU(struct kprobe *, current_kprobe) = core::ptr::null_mut();
    DEFINE_PER_CPU(struct kprobe_ctlblk, kprobe_ctlblk);

    (((b0##UL << 0x0)|(b1##UL << 0x1)|(b2##UL << 0x2)|(b3##UL << 0x3) |   \
    (b4##UL << 0x4)|(b5##UL << 0x5)|(b6##UL << 0x6)|(b7##UL << 0x7) |   \
    (b8##UL << 0x8)|(b9##UL << 0x9)|(ba##UL << 0xa)|(bb##UL << 0xb) |   \
    (bc##UL << 0xc)|(bd##UL << 0xd)|(be##UL << 0xe)|(bf##UL << 0xf))    \
    << (row % 32))
//
// Undefined/reserved opcodes, conditional jump, Opcode Extension
// Groups, and some special opcodes can not boost.
// This is non-const and volatile to keep gcc from statically
// optimizing it out, as variable_test_bit makes gcc think only
// *(unsigned long*) is used.
//
    static volatile u32 twobyte_is_boostable[256 / 32] = {
// 0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
// ----------------------------------------------
    W(0x00, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0) | /* 00 */
    W(0x10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1) , /* 10 */
    W(0x20, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0) | /* 20 */
    W(0x30, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0) , /* 30 */
    W(0x40, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1) | /* 40 */
    W(0x50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0) , /* 50 */
    W(0x60, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1) | /* 60 */
    W(0x70, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1) , /* 70 */
    W(0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0) | /* 80 */
    W(0x90, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1) , /* 90 */
    W(0xa0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1) | /* a0 */
    W(0xb0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1) , /* b0 */
    W(0xc0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1) | /* c0 */
    W(0xd0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1) , /* d0 */
    W(0xe0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1) | /* e0 */
    W(0xf0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1, 0)   /* f0 */
// -----------------------------------------------
// 0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
    };

    struct kretprobe_blackpoint kretprobe_blacklist[] = {
    {"__switch_to", }, /* This function switches only current task, but
    doesn't switch kernel stack.*/
    {core::ptr::null_mut(), core::ptr::null_mut()}	/* Terminator */
    };
    let mut kretprobe_blacklist_size: c_int = ARRAY_SIZE(kretprobe_blacklist);
    static nokprobe_inline void
    __synthesize_relative_insn(void *dest, void *from, void *to, u8 op)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __arch_relative_insn {
    pub op: u8,
    pub raddr: i32,
    pub insn: *mut } __packed,
    pub )dest: *mut insn = (struct __arch_relative_insn,
    pub 5)): insn->raddr = (s32)((long)(to) - ((long)(from) +,
    pub op: insn->op =,
    }
// Insert a jump instruction at address 'from', which jumps to address 'to'.
#[no_mangle]
pub unsafe extern "C" fn synthesize_reljump(dest: *mut c_void, from: *mut c_void, to: *mut c_void) {
    void synthesize_reljump(void *dest, void *from, void *to)
    {
    pub JMP32_INSN_OPCODE): __synthesize_relative_insn(dest, from, to,,
    }
// Insert a call instruction at address 'from', which calls address 'to'.
#[no_mangle]
pub unsafe extern "C" fn synthesize_relcall(dest: *mut c_void, from: *mut c_void, to: *mut c_void) {
    void synthesize_relcall(void *dest, void *from, void *to)
    {
    pub CALL_INSN_OPCODE): __synthesize_relative_insn(dest, from, to,,
    }
//
// Returns non-zero if INSN is boostable.
// RIP relative instructions are adjusted at copying time in 64 bits mode
//
#[no_mangle]
pub unsafe extern "C" fn can_boost(insn: *mut insn, addr: *mut c_void) -> bool {
    bool can_boost(struct insn *insn, void *addr)
    {
    pub opcode: kprobe_opcode_t,
    pub prefix: insn_byte_t,
    if (search_exception_tables((unsigned long)addr))
    pub /: *mut *mut return false; / Page fault may occur on this address.,
// 2nd-byte opcode
    if (insn.opcode.nbytes == 2)
    return test_bit(insn.opcode.bytes[1],
    pub )twobyte_is_boostable): *mut (unsigned long,
    if (insn.opcode.nbytes != 1)
    pub false: return,
    for_each_insn_prefix(insn, prefix) {
    pub attr: insn_attr_t,
    pub inat_get_opcode_attribute(prefix): attr =,
// Can't boost Address-size override prefix and CS override prefix
    if (prefix == 0x2e || inat_is_address_size_prefix(attr))
    pub false: return,
    }
    pub insn->opcode.bytes[0]: opcode =,
    switch (opcode) {
    case 0x62:		/* bound */
    case 0x70 ... 0x7f:	/* Conditional jumps */
    case 0x9a:		/* Call far */
    case 0xcc ... 0xce:	/* software exceptions */
    case 0xd6:		/* (UD) */
    case 0xd8 ... 0xdf:	/* ESC */
    case 0xe0 ... 0xe3:	/* LOOP*, JCXZ */
    case 0xe8 ... 0xe9:	/* near Call, JMP */
    case 0xeb:		/* Short JMP */
    case 0xf0 ... 0xf4:	/* LOCK/REP, HLT */
// ... are not boostable
    pub false: return,
    case 0xc0 ... 0xc1:	/* Grp2 */
    case 0xd0 ... 0xd3:	/* Grp2 */
//
// AMD uses nnn == 110 as SHL/SAL, but Intel makes it reserved.
//
    pub 0b110: return X86_MODRM_REG(insn->modrm.bytes[0]) !=,
    case 0xf6 ... 0xf7:	/* Grp3 */
// AMD uses nnn == 001 as TEST, but Intel makes it reserved.
    pub 0b001: return X86_MODRM_REG(insn->modrm.bytes[0]) !=,
    case 0xfe:		/* Grp4 */
// Only INC and DEC are boostable
    return X86_MODRM_REG(insn.modrm.bytes[0]) == 0b000 ||
    pub 0b001: X86_MODRM_REG(insn->modrm.bytes[0]) ==,
    case 0xff:		/* Grp5 */
// Only INC, DEC, and indirect JMP are boostable
    return X86_MODRM_REG(insn.modrm.bytes[0]) == 0b000 ||
    X86_MODRM_REG(insn.modrm.bytes[0]) == 0b001 ||
    pub 0b100: X86_MODRM_REG(insn->modrm.bytes[0]) ==,
    default:
    pub true: return,
    }
    }
    static unsigned long
    __recover_probed_insn(kprobe_opcode_t *buf, unsigned long addr)
    {
    pub kp: *mut kprobe,
    pub faddr: bool,
    pub )addr): *mut kp = get_kprobe((void,
    pub addr: faddr = ftrace_location(addr) ==,
//
// Use the current code if it is not modified by Kprobe
// and it cannot be modified by ftrace.
//
    if (!kp && !faddr)
    pub addr: return,
//
// Basically, kp->ainsn.insn has an original instruction.
// However, RIP-relative instruction can not do single-stepping
// at different place, __copy_instruction() tweaks the displacement of
// that instruction. In that case, we can't recover the instruction
// from the kp->ainsn.insn.
//
// On the other hand, in case on normal Kprobe, kp->opcode has a copy
// of the first byte of the probed instruction, which is overwritten
// by int3. And the instruction at kp->addr is not modified by kprobes
// except for the first byte, we can recover the original instruction
// from it and kp->opcode.
//
// In case of Kprobes using ftrace, we do not have a copy of
// the original instruction. In fact, the ftrace location might
// be modified at anytime and even could be in an inconsistent state.
// Fortunately, we know that the original code is the ideal 5-byte
// long NOP.
//
    if (copy_from_kernel_nofault(buf, (void *)addr,
#[no_mangle]
pub unsafe extern "C" fn sizeof(_arg: kprobe_opcode_t))) -> *mut MAX_INSN_SIZE {
    MAX_INSN_SIZE * sizeof(kprobe_opcode_t)))
    pub 0UL: return,
    if (faddr)
    pub 5): memcpy(buf, x86_nops[5],,
    else
    pub kp->opcode: buf[0] =,
    pub long)buf: return (unsigned,
    }
//
// Recover the probed instruction at addr for further analysis.
// Caller must lock kprobes by kprobe_mutex, or disable preemption
// for preventing to release referencing kprobes.
// Returns zero if the instruction can not get recovered (or access failed).
//
#[no_mangle]
pub unsafe extern "C" fn recover_probed_instruction(buf: *mut kprobe_opcode_t, addr: c_ulong) -> c_ulong {
    unsigned long recover_probed_instruction(kprobe_opcode_t *buf, unsigned long addr)
    {
    pub __addr: c_ulong,
    pub addr): __addr = __recover_optprobed_insn(buf,,
    if (__addr != addr)
    pub __addr: return,
    pub addr): return __recover_probed_insn(buf,,
    }
// Check if insn is INT or UD
#[no_mangle]
pub unsafe extern "C" fn is_exception_insn(insn: *mut insn) -> bool {
    static inline bool is_exception_insn(struct insn *insn)
    {
// UD uses 0f escape
    if (insn.opcode.bytes[0] == 0x0f) {
// UD0 / UD1 / UD2
    return insn.opcode.bytes[1] == 0xff ||
    insn.opcode.bytes[1] == 0xb9 ||
    pub 0x0b: insn->opcode.bytes[1] ==,
    }
// INT3 / INT n / INTO / INT1
    return insn.opcode.bytes[0] == 0xcc ||
    insn.opcode.bytes[0] == 0xcd ||
    insn.opcode.bytes[0] == 0xce ||
    pub 0xf1: insn->opcode.bytes[0] ==,
    }
//
// Check if paddr is at an instruction boundary and that instruction can
// be probed
//
#[no_mangle]
unsafe extern "C" fn can_probe(paddr: c_ulong) -> bool {
    static bool can_probe(unsigned long paddr)
    {
    pub 0: unsigned long addr, __addr, offset =,
    pub insn: insn,
    pub buf: [kprobe_opcode_t; MAX_INSN_SIZE],
    if (!kallsyms_lookup_size_offset(paddr, core::ptr::null_mut(), &offset))
    pub false: return,
// Decode instructions
    pub offset: addr = paddr -,
    while (addr < paddr) {
//
// Check if the instruction has been modified by another
// kprobe, in which case we replace the breakpoint by the
// original instruction in our buffer.
// Also, jump optimization will change the breakpoint to
// relative-jump. Since the relative-jump itself is
// normally used, we just go through if there is no kprobe.
//
    pub addr): __addr = recover_probed_instruction(buf,,
    if (!__addr)
    pub false: return,
    if (insn_decode_kernel(&insn, (void *)__addr) < 0)
    pub false: return,

//
// If there is a dynamically installed kgdb sw breakpoint,
// this function should not be probed.
//
    if (insn.opcode.bytes[0] == INT3_INSN_OPCODE &&
    kgdb_has_hit_break(addr))
    pub false: return,

    pub insn.length: addr +=,
    }
// Check if paddr is at an instruction boundary
    if (addr != paddr)
    pub false: return,
    pub addr): __addr = recover_probed_instruction(buf,,
    if (!__addr)
    pub false: return,
    if (insn_decode_kernel(&insn, (void *)__addr) < 0)
    pub false: return,
// INT and UD are special and should not be kprobed
    if (is_exception_insn(&insn))
    pub false: return,
    if (IS_ENABLED(CONFIG_CFI)) {
//
// The compiler generates the following instruction sequence
// for indirect call checks and cfi.c decodes this;
//
// movl    -<id>, %r10d       ; 6 bytes
// addl    -4(%reg), %r10d    ; 4 bytes
// je      .Ltmp1             ; 2 bytes
// ud2                        ; <- regs->ip
// .Ltmp1:
//
// Also, these movl and addl are used for showing expected
// type. So those must not be touched.
//
    if (insn.opcode.value == 0xBA)
    pub 12: offset =,
#[no_mangle]
pub unsafe extern "C" fn if(0x3: insn.opcode.value ==) -> else {
    else if (insn.opcode.value == 0x3)
    pub 6: offset =,
    else
    pub out: goto,
// This movl/addl is used for decoding CFI.
    if (is_cfi_trap(addr + offset))
    pub false: return,
    }
    out:
    pub true: return,
    }
// If x86 supports IBT (ENDBR) it must be skipped.
    kprobe_opcode_t *arch_adjust_kprobe_addr(unsigned long addr, unsigned long offset,
    bool *on_func_entry)
    {
    if (is_endbr((u32 *)addr)) {
// on_func_entry = !offset || offset == 4;
    if (*on_func_entry)
    pub 4: offset =,
    } else {
// on_func_entry = !offset;
    }
    pub offset): *mut *mut return (kprobe_opcode_t )(addr +,
    }
//
// Copy an instruction with recovering modified instruction by kprobes
// and adjust the displacement if the instruction uses the %rip-relative
// addressing mode. Note that since @real will be the final place of copied
// instruction, displacement must be adjust by @real, not @dest.
// This returns the length of copied instruction, or 0 if it has an error.
//
#[no_mangle]
pub unsafe extern "C" fn __copy_instruction(dest: *mut u8, src: *mut u8, real: *mut u8, insn: *mut insn) -> c_int {
    int __copy_instruction(u8 *dest, u8 *src, u8 *real, struct insn *insn)
    {
    pub buf: [kprobe_opcode_t; MAX_INSN_SIZE],
    pub long)src): unsigned long recovered_insn = recover_probed_instruction(buf, (unsigned,
    pub ret: c_int,
    if (!recovered_insn || !insn)
    pub 0: return,
// This can access kernel text if given address is not recovered
    if (copy_from_kernel_nofault(dest, (void *)recovered_insn,
    MAX_INSN_SIZE))
    pub 0: return,
    pub dest): ret = insn_decode_kernel(insn,,
    if (ret < 0)
    pub 0: return,
// We can not probe force emulate prefixed instruction
    if (insn_has_emulate_prefix(insn))
    pub 0: return,
// Another subsystem puts a breakpoint, failed to recover
    if (insn.opcode.bytes[0] == INT3_INSN_OPCODE)
    pub 0: return,
// We should not singlestep on the exception masking instructions
    if (insn_masking_exception(insn))
    pub 0: return,

// Only x86_64 has RIP relative instructions
    if (insn_rip_relative(insn)) {
    pub newdisp: i64,
    pub disp: *mut u8,
//
// The copied instruction uses the %rip-relative addressing
// mode.  Adjust the displacement for the difference between
// the original location of this instruction and the location
// of the copy that will actually be run.  The tricky bit here
// is making sure that the sign extension happens correctly in
// this calculation, since we need a signed 32-bit result to
// be sign-extended to 64 bits when it's added to the %rip
// value and yield the same 64-bit result that the sign-
// extension of the original signed 32-bit displacement would
// have given.
//
    newdisp = (u8 *) src + (s64) insn.displacement.value
    pub real: *mut *mut - (u8 ),
    if ((s64) (s32) newdisp != newdisp) {
    pub newdisp): pr_err("Kprobes error: new displacement does not fit into s32 (%llx)\n",,
    pub 0: return,
    }
    pub insn_offset_displacement(insn): *mut *mut disp = (u8 ) dest +,
// (s32 *) disp = (s32) newdisp;
    }

    pub insn->length: return,
    }
// Prepare reljump or int3 right after instruction
    static int prepare_singlestep(kprobe_opcode_t *buf, struct kprobe *p,
    struct insn *insn)
    {
    pub insn->length: int len =,
    if (!IS_ENABLED(CONFIG_PREEMPTION) &&
    !p.post_handler && can_boost(insn, p.addr) &&
    MAX_INSN_SIZE - len >= JMP32_INSN_SIZE) {
//
// These instructions can be executed directly if it
// jumps back to correct address.
//
    synthesize_reljump(buf + len, p.ainsn.insn + len,
    pub insn->length): p->addr +,
    pub JMP32_INSN_SIZE: len +=,
    pub 1: p->ainsn.boostable =,
    } else {
// Otherwise, put an int3 for trapping singlestep
    if (MAX_INSN_SIZE - len < INT3_INSN_SIZE)
    pub -ENOSPC: return,
    pub INT3_INSN_OPCODE: buf[len] =,
    pub INT3_INSN_SIZE: len +=,
    }
    pub len: return,
    }
// Kprobe x86 instruction emulation - only regs->ip or IF flag modifiers
#[no_mangle]
unsafe extern "C" fn kprobe_emulate_ifmodifiers(p: *mut kprobe, regs: *mut pt_regs) {
    static void kprobe_emulate_ifmodifiers(struct kprobe *p, struct pt_regs *regs)
    {
    switch (p.ainsn.opcode) {
    case 0xfa:	/* cli */
    pub ~(X86_EFLAGS_IF): regs->flags &=,
    case 0xfb:	/* sti */
    pub X86_EFLAGS_IF: regs->flags |=,
    case 0x9c:	/* pushf */
    pub regs->flags): int3_emulate_push(regs,,
    case 0x9d:	/* popf */
    pub int3_emulate_pop(regs): regs->flags =,
    }
    pub p->ainsn.size: regs->ip = regs->ip - INT3_INSN_SIZE +,
    }
#[no_mangle]
unsafe extern "C" fn kprobe_emulate_ret(p: *mut kprobe, regs: *mut pt_regs) {
    static void kprobe_emulate_ret(struct kprobe *p, struct pt_regs *regs)
    {
    }
#[no_mangle]
unsafe extern "C" fn kprobe_emulate_call(p: *mut kprobe, regs: *mut pt_regs) {
    static void kprobe_emulate_call(struct kprobe *p, struct pt_regs *regs)
    {
    pub p->ainsn.size: unsigned long func = regs->ip - INT3_INSN_SIZE +,
    pub p->ainsn.rel32: func +=,
    pub func): int3_emulate_call(regs,,
    }
#[no_mangle]
unsafe extern "C" fn kprobe_emulate_jmp(p: *mut kprobe, regs: *mut pt_regs) {
    static void kprobe_emulate_jmp(struct kprobe *p, struct pt_regs *regs)
    {
    pub p->ainsn.size: unsigned long ip = regs->ip - INT3_INSN_SIZE +,
    pub p->ainsn.rel32: ip +=,
    pub ip): int3_emulate_jmp(regs,,
    }
#[no_mangle]
unsafe extern "C" fn kprobe_emulate_jcc(p: *mut kprobe, regs: *mut pt_regs) {
    static void kprobe_emulate_jcc(struct kprobe *p, struct pt_regs *regs)
    {
    pub p->ainsn.size: unsigned long ip = regs->ip - INT3_INSN_SIZE +,
    pub p->ainsn.rel32): int3_emulate_jcc(regs, p->ainsn.jcc.type, ip,,
    }
#[no_mangle]
unsafe extern "C" fn kprobe_emulate_loop(p: *mut kprobe, regs: *mut pt_regs) {
    static void kprobe_emulate_loop(struct kprobe *p, struct pt_regs *regs)
    {
    pub p->ainsn.size: unsigned long ip = regs->ip - INT3_INSN_SIZE +,
    pub match: bool,
    if (p.ainsn.loop.type != 3) {	/* LOOP* */
    if (p.ainsn.loop.asize == 32)
    pub 0: *mut *mut *mut match = (((u32 )&regs->cx)--) !=,

#[no_mangle]
pub unsafe extern "C" fn if(64: p->ainsn.loop.asize ==) -> else {
    else if (p.ainsn.loop.asize == 64)
    pub 0: *mut *mut *mut match = (((u64 )&regs->cx)--) !=,

    else
    pub 0: *mut *mut *mut match = (((u16 )&regs->cx)--) !=,
    } else {			/* JCXZ */
    if (p.ainsn.loop.asize == 32)
    pub 0: *mut *mut *mut match = (u32 )(&regs->cx) ==,

#[no_mangle]
pub unsafe extern "C" fn if(64: p->ainsn.loop.asize ==) -> else {
    else if (p.ainsn.loop.asize == 64)
    pub 0: *mut *mut *mut match = (u64 )(&regs->cx) ==,

    else
    pub 0: *mut *mut *mut match = (u16 )(&regs->cx) ==,
    }
    if (p.ainsn.loop.type == 0)	/* LOOPNE */
    pub X86_EFLAGS_ZF): match = match && !(regs->flags &,
    else if (p.ainsn.loop.type == 1)	/* LOOPE */
    pub X86_EFLAGS_ZF): match = match && (regs->flags &,
    if (match)
    pub p->ainsn.rel32: ip +=,
    pub ip): int3_emulate_jmp(regs,,
    }
    static const int addrmode_regoffs[] = {
    offsetof(struct pt_regs, ax),
    offsetof(struct pt_regs, cx),
    offsetof(struct pt_regs, dx),
    offsetof(struct pt_regs, bx),
    offsetof(struct pt_regs, sp),
    offsetof(struct pt_regs, bp),
    offsetof(struct pt_regs, si),
    offsetof(struct pt_regs, di),

    offsetof(struct pt_regs, r8),
    offsetof(struct pt_regs, r9),
    offsetof(struct pt_regs, r10),
    offsetof(struct pt_regs, r11),
    offsetof(struct pt_regs, r12),
    offsetof(struct pt_regs, r13),
    offsetof(struct pt_regs, r14),
    offsetof(struct pt_regs, r15),

}

#[no_mangle]
unsafe extern "C" fn kprobe_emulate_call_indirect(p: *mut kprobe, regs: *mut pt_regs) {
    static void kprobe_emulate_call_indirect(struct kprobe *p, struct pt_regs *regs)
    {
    let mut offs: c_ulong = addrmode_regoffs[p.ainsn.indirect.reg];
    int3_emulate_push(regs, regs.ip - INT3_INSN_SIZE + p.ainsn.size);
    int3_emulate_jmp(regs, regs_get_register(regs, offs));
    }
    NOKPROBE_SYMBOL(kprobe_emulate_call_indirect);
#[no_mangle]
unsafe extern "C" fn kprobe_emulate_jmp_indirect(p: *mut kprobe, regs: *mut pt_regs) {
    static void kprobe_emulate_jmp_indirect(struct kprobe *p, struct pt_regs *regs)
    {
    let mut offs: c_ulong = addrmode_regoffs[p.ainsn.indirect.reg];
    int3_emulate_jmp(regs, regs_get_register(regs, offs));
    }
    NOKPROBE_SYMBOL(kprobe_emulate_jmp_indirect);
#[no_mangle]
unsafe extern "C" fn prepare_emulation(p: *mut kprobe, insn: *mut insn) -> c_int {
    static int prepare_emulation(struct kprobe *p, struct insn *insn)
    {
    let mut opcode: insn_byte_t = insn.opcode.bytes[0];
    switch (opcode) {
    case 0xfa:		/* cli */
    case 0xfb:		/* sti */
    case 0x9c:		/* pushfl */
    case 0x9d:		/* popf/popfd */
//
// IF modifiers must be emulated since it will enable interrupt while
// int3 single stepping.
//
    p.ainsn.emulate_op = kprobe_emulate_ifmodifiers;
    p.ainsn.opcode = opcode;
    break;
    case 0xc2:	/* ret/lret */
    case 0xc3:
    case 0xca:
    case 0xcb:
    p.ainsn.emulate_op = kprobe_emulate_ret;
    break;
    case 0x9a:	/* far call absolute -- segment is not supported */
    case 0xea:	/* far jmp absolute -- segment is not supported */
    case 0xcc:	/* int3 */
    case 0xcf:	/* iret -- in-kernel IRET is not supported */
    return -EOPNOTSUPP;
    break;
    case 0xe8:	/* near call relative */
    p.ainsn.emulate_op = kprobe_emulate_call;
    if (insn.immediate.nbytes == 2)
    p.ainsn.rel32 = *(s16 *)&insn.immediate.value;
    else
    p.ainsn.rel32 = *(s32 *)&insn.immediate.value;
    break;
    case 0xeb:	/* short jump relative */
    case 0xe9:	/* near jump relative */
    p.ainsn.emulate_op = kprobe_emulate_jmp;
    if (insn.immediate.nbytes == 1)
    p.ainsn.rel32 = *(s8 *)&insn.immediate.value;
#[no_mangle]
pub unsafe extern "C" fn if(2: insn->immediate.nbytes ==) -> else {
    else if (insn.immediate.nbytes == 2)
    p.ainsn.rel32 = *(s16 *)&insn.immediate.value;
    else
    p.ainsn.rel32 = *(s32 *)&insn.immediate.value;
    break;
    case 0x70 ... 0x7f:
// 1 byte conditional jump
    p.ainsn.emulate_op = kprobe_emulate_jcc;
    p.ainsn.jcc.type = opcode & 0xf;
    p.ainsn.rel32 = insn.immediate.value;
    break;
    case 0x0f:
    opcode = insn.opcode.bytes[1];
    if ((opcode & 0xf0) == 0x80) {
// 2 bytes Conditional Jump
    p.ainsn.emulate_op = kprobe_emulate_jcc;
    p.ainsn.jcc.type = opcode & 0xf;
    if (insn.immediate.nbytes == 2)
    p.ainsn.rel32 = *(s16 *)&insn.immediate.value;
    else
    p.ainsn.rel32 = *(s32 *)&insn.immediate.value;
    } else if (opcode == 0x01 &&
    X86_MODRM_REG(insn.modrm.bytes[0]) == 0 &&
    X86_MODRM_MOD(insn.modrm.bytes[0]) == 3) {
// VM extensions - not supported
    return -EOPNOTSUPP;
    }
    break;
    case 0xe0:	/* Loop NZ */
    case 0xe1:	/* Loop */
    case 0xe2:	/* Loop */
    case 0xe3:	/* J*CXZ */
    p.ainsn.emulate_op = kprobe_emulate_loop;
    p.ainsn.loop.type = opcode & 0x3;
    p.ainsn.loop.asize = insn.addr_bytes * 8;
    p.ainsn.rel32 = *(s8 *)&insn.immediate.value;
    break;
    case 0xff:
//
// Since the 0xff is an extended group opcode, the instruction
// is determined by the MOD/RM byte.
//
    opcode = insn.modrm.bytes[0];
    switch (X86_MODRM_REG(opcode)) {
    case 0b010:	/* FF /2, call near, absolute indirect */
    p.ainsn.emulate_op = kprobe_emulate_call_indirect;
    break;
    case 0b100:	/* FF /4, jmp near, absolute indirect */
    p.ainsn.emulate_op = kprobe_emulate_jmp_indirect;
    break;
    case 0b011:	/* FF /3, call far, absolute indirect */
    case 0b101:	/* FF /5, jmp far, absolute indirect */
    return -EOPNOTSUPP;
    }
    if (!p.ainsn.emulate_op)
    break;
    if (insn.addr_bytes != sizeof(unsigned long))
    return -EOPNOTSUPP;	/* Don't support different size */
    if (X86_MODRM_MOD(opcode) != 3)
    return -EOPNOTSUPP;	/* TODO: support memory addressing */
    p.ainsn.indirect.reg = X86_MODRM_RM(opcode);

    if (X86_REX_B(insn.rex_prefix.value))
    p.ainsn.indirect.reg += 8;

    break;
    default:
    break;
    }
    p.ainsn.size = insn.length;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn arch_copy_kprobe(p: *mut kprobe) -> c_int {
    static int arch_copy_kprobe(struct kprobe *p)
    {
    struct insn insn;
    kprobe_opcode_t buf[MAX_INSN_SIZE];
    int ret, len;
// Copy an instruction with recovering if other optprobe modifies it.
    len = __copy_instruction(buf, p.addr, p.ainsn.insn, &insn);
    if (!len)
    return -EINVAL;
// Analyze the opcode and setup emulate functions
    ret = prepare_emulation(p, &insn);
    if (ret < 0)
    return ret;
// Add int3 for single-step or booster jmp
    len = prepare_singlestep(buf, p, &insn);
    if (len < 0)
    return len;
// Also, displacement change doesn't affect the first byte
    p.opcode = buf[0];
    p.ainsn.tp_len = len;
    perf_event_text_poke(p.ainsn.insn, core::ptr::null_mut(), 0, buf, len);
// OK, write back the instruction(s) into ROX insn buffer
    text_poke(p.ainsn.insn, buf, len);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_prepare_kprobe(p: *mut kprobe) -> c_int {
    int arch_prepare_kprobe(struct kprobe *p)
    {
    int ret;
    if (!can_probe((unsigned long)p.addr))
    return -EILSEQ;
    memset(&p.ainsn, 0, sizeof(p.ainsn));
// insn: must be on special executable page on x86.
    p.ainsn.insn = get_insn_slot();
    if (!p.ainsn.insn)
    return -ENOMEM;
    ret = arch_copy_kprobe(p);
    if (ret) {
    free_insn_slot(p.ainsn.insn, 0);
    p.ainsn.insn = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_arm_kprobe(p: *mut kprobe) {
    void arch_arm_kprobe(struct kprobe *p)
    {
    let mut int3: u8 = INT3_INSN_OPCODE;
    text_poke(p.addr, &int3, 1);
    smp_text_poke_sync_each_cpu();
    perf_event_text_poke(p.addr, &p.opcode, 1, &int3, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_disarm_kprobe(p: *mut kprobe) {
    void arch_disarm_kprobe(struct kprobe *p)
    {
    let mut int3: u8 = INT3_INSN_OPCODE;
    perf_event_text_poke(p.addr, &int3, 1, &p.opcode, 1);
    text_poke(p.addr, &p.opcode, 1);
    smp_text_poke_sync_each_cpu();
    }
#[no_mangle]
pub unsafe extern "C" fn arch_remove_kprobe(p: *mut kprobe) {
    void arch_remove_kprobe(struct kprobe *p)
    {
    if (p.ainsn.insn) {
// Record the perf event before freeing the slot
    perf_event_text_poke(p.ainsn.insn, p.ainsn.insn,
    p.ainsn.tp_len, core::ptr::null_mut(), 0);
    free_insn_slot(p.ainsn.insn, p.ainsn.boostable);
    p.ainsn.insn = core::ptr::null_mut();
    }
    }
    static nokprobe_inline void
    save_previous_kprobe(struct kprobe_ctlblk *kcb)
    {
    kcb.prev_kprobe.kp = kprobe_running();
    kcb.prev_kprobe.status = kcb.kprobe_status;
    kcb.prev_kprobe.old_flags = kcb.kprobe_old_flags;
    kcb.prev_kprobe.saved_flags = kcb.kprobe_saved_flags;
    }
    static nokprobe_inline void
    restore_previous_kprobe(struct kprobe_ctlblk *kcb)
    {
    __this_cpu_write(current_kprobe, kcb.prev_kprobe.kp);
    kcb.kprobe_status = kcb.prev_kprobe.status;
    kcb.kprobe_old_flags = kcb.prev_kprobe.old_flags;
    kcb.kprobe_saved_flags = kcb.prev_kprobe.saved_flags;
    }
    static nokprobe_inline void
    set_current_kprobe(struct kprobe *p, struct pt_regs *regs,
    struct kprobe_ctlblk *kcb)
    {
    __this_cpu_write(current_kprobe, p);
    kcb.kprobe_saved_flags = kcb.kprobe_old_flags
    = (regs.flags & X86_EFLAGS_IF);
    }
    static void kprobe_post_process(struct kprobe *cur, struct pt_regs *regs,
    struct kprobe_ctlblk *kcb)
    {
// Restore back the original saved kprobes variables and continue.
    if (kcb.kprobe_status == KPROBE_REENTER) {
// This will restore both kcb and current_kprobe
    restore_previous_kprobe(kcb);
    } else {
//
// Always update the kcb status because
// reset_curent_kprobe() doesn't update kcb.
//
    kcb.kprobe_status = KPROBE_HIT_SSDONE;
    if (cur.post_handler)
    cur.post_handler(cur, regs, 0);
    reset_current_kprobe();
    }
    }
    NOKPROBE_SYMBOL(kprobe_post_process);
    static void setup_singlestep(struct kprobe *p, struct pt_regs *regs,
    struct kprobe_ctlblk *kcb, int reenter)
    {
    if (setup_detour_execution(p, regs, reenter))
    return;

    if (p.ainsn.boostable) {
// Boost up -- we can execute copied instructions directly
    if (!reenter)
    reset_current_kprobe();
//
// Reentering boosted probe doesn't reset current_kprobe,
// nor set current_kprobe, because it doesn't use single
// stepping.
//
    regs.ip = (unsigned long)p.ainsn.insn;
    return;
    }

    if (reenter) {
    save_previous_kprobe(kcb);
    set_current_kprobe(p, regs, kcb);
    kcb.kprobe_status = KPROBE_REENTER;
    } else
    kcb.kprobe_status = KPROBE_HIT_SS;
    if (p.ainsn.emulate_op) {
    p.ainsn.emulate_op(p, regs);
    kprobe_post_process(p, regs, kcb);
    return;
    }
// Disable interrupt, and set ip register on trampoline
    regs.flags &= ~X86_EFLAGS_IF;
    regs.ip = (unsigned long)p.ainsn.insn;
    }
    NOKPROBE_SYMBOL(setup_singlestep);
//
// Called after single-stepping.  p->addr is the address of the
// instruction whose first byte has been replaced by the "int3"
// instruction.  To avoid the SMP problems that can occur when we
// temporarily put back the original opcode to single-step, we
// single-stepped a copy of the instruction.  The address of this
// copy is p->ainsn.insn. We also doesn't use trap, but "int3" again
// right after the copied instruction.
// Different from the trap single-step, "int3" single-step can not
// handle the instruction which changes the ip register, e.g. jmp,
// call, conditional jmp, and the instructions which changes the IF
// flags because interrupt must be disabled around the single-stepping.
// Such instructions are software emulated, but others are single-stepped
// using "int3".
//
// When the 2nd "int3" handled, the regs->ip and regs->flags needs to
// be adjusted, so that we can resume execution on correct code.
//
    static void resume_singlestep(struct kprobe *p, struct pt_regs *regs,
    struct kprobe_ctlblk *kcb)
    {
    let mut copy_ip: c_ulong = (unsigned long)p.ainsn.insn;
    let mut orig_ip: c_ulong = (unsigned long)p.addr;
// Restore saved interrupt flag and ip register
    regs.flags |= kcb.kprobe_saved_flags;
// Note that regs->ip is executed int3 so must be a step back
    regs.ip += (orig_ip - copy_ip) - INT3_INSN_SIZE;
    }
    NOKPROBE_SYMBOL(resume_singlestep);
//
// We have reentered the kprobe_handler(), since another probe was hit while
// within the handler. We save the original kprobes variables and just single
// step on the instruction of the new probe without calling any user handlers.
//
    static int reenter_kprobe(struct kprobe *p, struct pt_regs *regs,
    struct kprobe_ctlblk *kcb)
    {
    switch (kcb.kprobe_status) {
    case KPROBE_HIT_SSDONE:
    case KPROBE_HIT_ACTIVE:
    case KPROBE_HIT_SS:
    kprobes_inc_nmissed_count(p);
    setup_singlestep(p, regs, kcb, 1);
    break;
    case KPROBE_REENTER:
// A probe has been hit in the codepath leading up to, or just
// after, single-stepping of a probed instruction. This entire
// codepath should strictly reside in .kprobes.text section.
// Raise a BUG or we'll continue in an endless reentering loop
// and eventually a stack overflow.
//
    pr_err("Unrecoverable kprobe detected.\n");
    dump_kprobe(p);
    BUG();
    default:
// impossible cases
    WARN_ON(1);
    return 0;
    }
    return 1;
    }
    NOKPROBE_SYMBOL(reenter_kprobe);
#[no_mangle]
unsafe extern "C" fn kprobe_is_ss(kcb: *mut kprobe_ctlblk) -> nokprobe_inline int {
    static nokprobe_inline int kprobe_is_ss(struct kprobe_ctlblk *kcb)
    {
    return (kcb.kprobe_status == KPROBE_HIT_SS ||
    kcb.kprobe_status == KPROBE_REENTER);
    }
//
// Interrupts are disabled on entry as trap3 is an interrupt gate and they
// remain disabled throughout this function.
//
#[no_mangle]
pub unsafe extern "C" fn kprobe_int3_handler(regs: *mut pt_regs) -> c_int {
    int kprobe_int3_handler(struct pt_regs *regs)
    {
    kprobe_opcode_t *addr;
    struct kprobe *p;
    struct kprobe_ctlblk *kcb;
    if (user_mode(regs))
    return 0;
    addr = (kprobe_opcode_t *)(regs.ip - sizeof(kprobe_opcode_t));
//
// We don't want to be preempted for the entire duration of kprobe
// processing. Since int3 and debug trap disables irqs and we clear
// IF while singlestepping, it must be no preemptible.
//
    kcb = get_kprobe_ctlblk();
    p = get_kprobe(addr);
    if (p) {
    if (kprobe_running()) {
    if (reenter_kprobe(p, regs, kcb))
    return 1;
    } else {
    set_current_kprobe(p, regs, kcb);
    kcb.kprobe_status = KPROBE_HIT_ACTIVE;
//
// If we have no pre-handler or it returned 0, we
// continue with normal processing.  If we have a
// pre-handler and it returned non-zero, that means
// user handler setup registers to exit to another
// instruction, we must skip the single stepping.
//
    if (!p.pre_handler || !p.pre_handler(p, regs))
    setup_singlestep(p, regs, kcb, 0);
    else
    reset_current_kprobe();
    return 1;
    }
    } else if (kprobe_is_ss(kcb)) {
    p = kprobe_running();
    if ((unsigned long)p.ainsn.insn < regs.ip &&
    (unsigned long)p.ainsn.insn + MAX_INSN_SIZE > regs.ip) {
// Most provably this is the second int3 for singlestep
    resume_singlestep(p, regs, kcb);
    kprobe_post_process(p, regs, kcb);
    return 1;
    }
    } /* else: not a kprobe fault; let the kernel handle it */
    return 0;
    }
    NOKPROBE_SYMBOL(kprobe_int3_handler);
#[no_mangle]
pub unsafe extern "C" fn kprobe_fault_handler(regs: *mut pt_regs, trapnr: c_int) -> c_int {
    int kprobe_fault_handler(struct pt_regs *regs, int trapnr)
    {
    struct kprobe *cur = kprobe_running();
    struct kprobe_ctlblk *kcb = get_kprobe_ctlblk();
    if (unlikely(regs.ip == (unsigned long)cur.ainsn.insn)) {
// This must happen on single-stepping
    WARN_ON(kcb.kprobe_status != KPROBE_HIT_SS &&
    kcb.kprobe_status != KPROBE_REENTER);
//
// We are here because the instruction being single
// stepped caused a page fault. We reset the current
// kprobe and the ip points back to the probe address
// and allow the page fault handler to continue as a
// normal page fault.
//
    regs.ip = (unsigned long)cur.addr;
//
// If the IF flag was set before the kprobe hit,
// don't touch it:
//
    regs.flags |= kcb.kprobe_old_flags;
    if (kcb.kprobe_status == KPROBE_REENTER)
    restore_previous_kprobe(kcb);
    else
    reset_current_kprobe();
    }
    return 0;
    }
    NOKPROBE_SYMBOL(kprobe_fault_handler);
#[no_mangle]
pub unsafe extern "C" fn arch_populate_kprobe_blacklist() -> int __init {
    int __init arch_populate_kprobe_blacklist(void)
    {
    return kprobe_add_area_blacklist((unsigned long)__entry_text_start,
    (unsigned long)__entry_text_end);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_init_kprobes() -> int __init {
    int __init arch_init_kprobes(void)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_trampoline_kprobe(p: *mut kprobe) -> c_int {
    int arch_trampoline_kprobe(struct kprobe *p)
    {
    return 0;
    }
