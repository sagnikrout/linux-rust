//! Automatically rewritten from C to Rust
//! Source: arch/x86/kvm/emulate.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// emulate.c
//
// Generic x86 (32-bit and 64-bit) instruction decoder and emulator.
//
// Copyright (c) 2005 Keir Fraser
//
// Linux coding style, mod r/m decoder, segment base fixes, real-mode
// privileged instructions:
//
// Copyright (C) 2006 Qumranet
// Copyright 2010 Red Hat, Inc. and/or its affiliates.
//
// Avi Kivity <avi@qumranet.com>
// Yaniv Kamay <yaniv@qumranet.com>
//
// From: xen-unstable 10676:af9809f51f81a3c43f276f00c81a52ef558afda4
//

//
// Operand types
//

//
// Opcode effective-address decode tables.
// Note that we only emulate instructions that have at least one memory
// operand (excluding implicit stack references). We assume that stack
// references and instruction fetches will never occur in special memory
// areas that require emulation. So, for example, 'mov <imm>,<reg>' need
// not be handled.
//
// Operand sizes: 8-bit operands or specified/overridden size.

// free: 37-39

// free: 43-44

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opcode {
    pub flags: u64,
    pub intercept: u8,
    pub pad: [u8; 7],
    union {
    pub ctxt): *mut *mut int (execute)(struct x86_emulate_ctxt,
    pub group: *const opcode,
    pub gdual: *const group_dual,
    pub gprefix: *const gprefix,
    pub esc: *const escape,
    pub idual: *const instr_dual,
    pub mdual: *const mode_dual,
    pub u: },
    pub ctxt): *mut *mut int (check_perm)(struct x86_emulate_ctxt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_dual {
    pub mod012: [opcode; 8],
    pub mod3: [opcode; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gprefix {
    pub pfx_no: opcode,
    pub pfx_66: opcode,
    pub pfx_f2: opcode,
    pub pfx_f3: opcode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct escape {
    pub op: [opcode; 8],
    pub high: [opcode; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct instr_dual {
    pub mod012: opcode,
    pub mod3: opcode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mode_dual {
    pub mode32: opcode,
    pub mode64: opcode,
}

pub const EFLG_RESERVED_ZEROS_MASK: c_uint = 0xffc0802a;
    enum x86_transfer_type {
    X86_TRANSFER_NONE,
    X86_TRANSFER_CALL_JMP,
    X86_TRANSFER_RET,
    X86_TRANSFER_TASK_SWITCH,
    };
    enum rex_bits {
    REX_B = 1,
    REX_X = 2,
    REX_R = 4,
    REX_W = 8,
    };
#[no_mangle]
unsafe extern "C" fn writeback_registers(ctxt: *mut x86_emulate_ctxt) {
    static void writeback_registers(struct x86_emulate_ctxt *ctxt)
    {
    let mut dirty: c_ulong = ctxt.regs_dirty;
    unsigned reg;
    for_each_set_bit(reg, &dirty, NR_EMULATOR_GPRS)
    ctxt.ops.write_gpr(ctxt, reg, ctxt._regs[reg]);
    }
#[no_mangle]
unsafe extern "C" fn invalidate_registers(ctxt: *mut x86_emulate_ctxt) {
    static void invalidate_registers(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.regs_dirty = 0;
    ctxt.regs_valid = 0;
    }
//
// These EFLAGS bits are restored from saved value during emulation, and
// any changes are written back to the saved value after emulation.
//

    X86_EFLAGS_PF|X86_EFLAGS_CF)

// Macro flag: #define ON64(x...)

    static int em_##op(struct x86_emulate_ctxt *ctxt) \
    { \
    unsigned long flags = (ctxt.eflags & EFLAGS_MASK) | X86_EFLAGS_IF; \
    int bytes = 1, ok = 1; \
    if (!(ctxt.d & ByteOp)) \
    bytes = ctxt.dst.bytes; \
    switch (bytes) {

    asm("push %[flags]; popf \n\t" \
    "10: " str \
    "pushf; pop %[flags] \n\t" \
    "11: \n\t" \
    : "+a" (ctxt.dst.val), \
    "+d" (ctxt.src.val), \
    [flags] "+D" (flags), \
    "+S" (ok) \
    : "c" (ctxt.src2.val))

    __EM_ASM(#op " %%" #dst " \n\t")

    __EM_ASM(#op " %%" #dst " \n\t" \
    _ASM_EXTABLE_TYPE_REG(10b, 11f, EX_TYPE_ZERO_REG, %%esi))

    __EM_ASM(#op " %%" #src ", %%" #dst " \n\t")

    __EM_ASM(#op " %%" #src2 ", %%" #src ", %%" #dst " \n\t")

    } \
    ctxt.eflags = (ctxt.eflags & ~EFLAGS_MASK) | (flags & EFLAGS_MASK); \
    return !ok ? emulate_de(ctxt) : X86EMUL_CONTINUE; \
    }
// 1-operand, using "a" (dst)

    EM_ASM_START(op) \
    case 1: __EM_ASM_1(op##b, al); break; \
    case 2: __EM_ASM_1(op##w, ax); break; \
    case 4: __EM_ASM_1(op##l, eax); break; \
    ON64(case 8: __EM_ASM_1(op##q, rax); break;) \
    EM_ASM_END
// 1-operand, using "c" (src2)

    EM_ASM_START(name) \
    case 1: __EM_ASM_1(op##b, cl); break; \
    case 2: __EM_ASM_1(op##w, cx); break; \
    case 4: __EM_ASM_1(op##l, ecx); break; \
    ON64(case 8: __EM_ASM_1(op##q, rcx); break;) \
    EM_ASM_END
// 1-operand, using "c" (src2) with exception

    EM_ASM_START(name) \
    case 1: __EM_ASM_1_EX(op##b, cl); break; \
    case 2: __EM_ASM_1_EX(op##w, cx); break; \
    case 4: __EM_ASM_1_EX(op##l, ecx); break; \
    ON64(case 8: __EM_ASM_1_EX(op##q, rcx); break;) \
    EM_ASM_END
// 2-operand, using "a" (dst), "d" (src)

    EM_ASM_START(op) \
    case 1: __EM_ASM_2(op##b, al, dl); break; \
    case 2: __EM_ASM_2(op##w, ax, dx); break; \
    case 4: __EM_ASM_2(op##l, eax, edx); break; \
    ON64(case 8: __EM_ASM_2(op##q, rax, rdx); break;) \
    EM_ASM_END
// 2-operand, reversed

    EM_ASM_START(name) \
    case 1: __EM_ASM_2(op##b, dl, al); break; \
    case 2: __EM_ASM_2(op##w, dx, ax); break; \
    case 4: __EM_ASM_2(op##l, edx, eax); break; \
    ON64(case 8: __EM_ASM_2(op##q, rdx, rax); break;) \
    EM_ASM_END
// 2-operand, word only (no byte op)

    EM_ASM_START(op) \
    case 1: break; \
    case 2: __EM_ASM_2(op##w, ax, dx); break; \
    case 4: __EM_ASM_2(op##l, eax, edx); break; \
    ON64(case 8: __EM_ASM_2(op##q, rax, rdx); break;) \
    EM_ASM_END
// 2-operand, using "a" (dst) and CL (src2)

    EM_ASM_START(op) \
    case 1: __EM_ASM_2(op##b, al, cl); break; \
    case 2: __EM_ASM_2(op##w, ax, cl); break; \
    case 4: __EM_ASM_2(op##l, eax, cl); break; \
    ON64(case 8: __EM_ASM_2(op##q, rax, cl); break;) \
    EM_ASM_END
// 3-operand, using "a" (dst), "d" (src) and CL (src2)

    EM_ASM_START(op) \
    case 1: break; \
    case 2: __EM_ASM_3(op##w, ax, dx, cl); break; \
    case 4: __EM_ASM_3(op##l, eax, edx, cl); break; \
    ON64(case 8: __EM_ASM_3(op##q, rax, rdx, cl); break;) \
    EM_ASM_END
#[no_mangle]
unsafe extern "C" fn em_salc(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_salc(struct x86_emulate_ctxt *ctxt)
    {
//
// Set AL 0xFF if CF is set, or 0x00 when clear.
//
    ctxt.dst.val = 0xFF * !!(ctxt.eflags & X86_EFLAGS_CF);
    return X86EMUL_CONTINUE;
    }
//
// XXX: inoutclob user must know where the argument is being expanded.
// Using asm goto would allow us to remove _fault.
//

    ({ \
    int _fault = 0; \
    \
    asm volatile("1:" insn "\n" \
    "2:\n" \
    _ASM_EXTABLE_TYPE_REG(1b, 2b, EX_TYPE_ONE_REG, %[_fault]) \
    : [_fault] "+r"(_fault) inoutclob ); \
    \
    _fault ? X86EMUL_UNHANDLEABLE : X86EMUL_CONTINUE; \
    })
    static int emulator_check_intercept(struct x86_emulate_ctxt *ctxt,
    enum x86_intercept intercept,
    enum x86_intercept_stage stage)
    {
    struct x86_instruction_info info = {
    .intercept  = intercept,
    .rep_prefix = ctxt.rep_prefix,
    .modrm_mod  = ctxt.modrm_mod,
    .modrm_reg  = ctxt.modrm_reg,
    .modrm_rm   = ctxt.modrm_rm,
    .src_val    = ctxt.src.val64,
    .dst_val    = ctxt.dst.val64,
    .src_bytes  = ctxt.src.bytes,
    .dst_bytes  = ctxt.dst.bytes,
    .src_type   = ctxt.src.type,
    .dst_type   = ctxt.dst.type,
    .ad_bytes   = ctxt.ad_bytes,
    .rip	    = ctxt.eip,
    .next_rip   = ctxt._eip,
    };
    return ctxt.ops.intercept(ctxt, &info, stage);
    }
#[no_mangle]
unsafe extern "C" fn assign_masked(dest: *mut c_ulong, src: c_ulong, mask: c_ulong) {
    static void assign_masked(ulong *dest, ulong src, ulong mask)
    {
// dest = (*dest & ~mask) | (src & mask);
    }
#[no_mangle]
pub unsafe extern "C" fn ad_mask(ctxt: *mut x86_emulate_ctxt) -> c_ulong {
    static inline unsigned long ad_mask(struct x86_emulate_ctxt *ctxt)
    {
    return (1UL << (ctxt.ad_bytes << 3)) - 1;
    }
#[no_mangle]
unsafe extern "C" fn stack_mask(ctxt: *mut x86_emulate_ctxt) -> c_ulong {
    static ulong stack_mask(struct x86_emulate_ctxt *ctxt)
    {
    u16 sel;
    struct desc_struct ss;
    if (ctxt.mode == X86EMUL_MODE_PROT64)
    return ~0UL;
    ctxt.ops.get_segment(ctxt, &sel, &ss, core::ptr::null_mut(), VCPU_SREG_SS);
    return ~0U >> ((ss.d ^ 1) * 16);  /* d=0: 0xffff; d=1: 0xffffffff */
    }
#[no_mangle]
unsafe extern "C" fn stack_size(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int stack_size(struct x86_emulate_ctxt *ctxt)
    {
    return (__fls(stack_mask(ctxt)) + 1) >> 3;
    }
// Access/update address held in a register, based on addressing mode.
    static inline unsigned long
    address_mask(struct x86_emulate_ctxt *ctxt, unsigned long reg)
    {
    if (ctxt.ad_bytes == sizeof(unsigned long))
    return reg;
    else
    return reg & ad_mask(ctxt);
    }
    static inline unsigned long
    register_address(struct x86_emulate_ctxt *ctxt, int reg)
    {
    return address_mask(ctxt, reg_read(ctxt, reg));
    }
#[no_mangle]
unsafe extern "C" fn masked_increment(reg: *mut c_ulong, mask: c_ulong, inc: c_int) {
    static void masked_increment(ulong *reg, ulong mask, int inc)
    {
    assign_masked(reg, *reg + inc, mask);
    }
    static inline void
    register_address_increment(struct x86_emulate_ctxt *ctxt, int reg, int inc)
    {
    ulong *preg = reg_rmw(ctxt, reg);
    insn_assign_reg(preg, *preg + inc, ctxt.ad_bytes);
    }
#[no_mangle]
unsafe extern "C" fn rsp_increment(ctxt: *mut x86_emulate_ctxt, inc: c_int) {
    static void rsp_increment(struct x86_emulate_ctxt *ctxt, int inc)
    {
    masked_increment(reg_rmw(ctxt, VCPU_REGS_RSP), stack_mask(ctxt), inc);
    }
#[no_mangle]
unsafe extern "C" fn desc_limit_scaled(desc: *mut desc_struct) -> u32 {
    static u32 desc_limit_scaled(struct desc_struct *desc)
    {
    let mut limit: u32 = get_desc_limit(desc);
    return desc.g ? (limit << 12) | 0xfff : limit;
    }
#[no_mangle]
unsafe extern "C" fn seg_base(ctxt: *mut x86_emulate_ctxt, seg: c_int) -> c_ulong {
    static unsigned long seg_base(struct x86_emulate_ctxt *ctxt, int seg)
    {
    if (ctxt.mode == X86EMUL_MODE_PROT64 && seg < VCPU_SREG_FS)
    return 0;
    return ctxt.ops.get_cached_segment_base(ctxt, seg);
    }
    static int emulate_exception(struct x86_emulate_ctxt *ctxt, int vec,
    u32 error, bool valid)
    {
    if (KVM_EMULATOR_BUG_ON(vec > 0x1f, ctxt))
    return X86EMUL_UNHANDLEABLE;
    ctxt.exception.vector = vec;
    ctxt.exception.error_code = error;
    ctxt.exception.error_code_valid = valid;
    return X86EMUL_PROPAGATE_FAULT;
    }
#[no_mangle]
unsafe extern "C" fn emulate_db(ctxt: *mut x86_emulate_ctxt, dr6: c_ulong) -> c_int {
    static int emulate_db(struct x86_emulate_ctxt *ctxt, unsigned long dr6)
    {
    ctxt.exception.dr6 = dr6;
    return emulate_exception(ctxt, DB_VECTOR, 0, false);
    }
#[no_mangle]
unsafe extern "C" fn emulate_gp(ctxt: *mut x86_emulate_ctxt, err: c_int) -> c_int {
    static int emulate_gp(struct x86_emulate_ctxt *ctxt, int err)
    {
    return emulate_exception(ctxt, GP_VECTOR, err, true);
    }
#[no_mangle]
unsafe extern "C" fn emulate_ss(ctxt: *mut x86_emulate_ctxt, err: c_int) -> c_int {
    static int emulate_ss(struct x86_emulate_ctxt *ctxt, int err)
    {
    return emulate_exception(ctxt, SS_VECTOR, err, true);
    }
#[no_mangle]
unsafe extern "C" fn emulate_ud(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int emulate_ud(struct x86_emulate_ctxt *ctxt)
    {
    return emulate_exception(ctxt, UD_VECTOR, 0, false);
    }
#[no_mangle]
unsafe extern "C" fn emulate_ts(ctxt: *mut x86_emulate_ctxt, err: c_int) -> c_int {
    static int emulate_ts(struct x86_emulate_ctxt *ctxt, int err)
    {
    return emulate_exception(ctxt, TS_VECTOR, err, true);
    }
#[no_mangle]
unsafe extern "C" fn emulate_de(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int emulate_de(struct x86_emulate_ctxt *ctxt)
    {
    return emulate_exception(ctxt, DE_VECTOR, 0, false);
    }
#[no_mangle]
unsafe extern "C" fn emulate_nm(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int emulate_nm(struct x86_emulate_ctxt *ctxt)
    {
    return emulate_exception(ctxt, NM_VECTOR, 0, false);
    }
#[no_mangle]
unsafe extern "C" fn get_segment_selector(ctxt: *mut x86_emulate_ctxt, seg: unsigned) -> u16 {
    static u16 get_segment_selector(struct x86_emulate_ctxt *ctxt, unsigned seg)
    {
    u16 selector;
    struct desc_struct desc;
    ctxt.ops.get_segment(ctxt, &selector, &desc, core::ptr::null_mut(), seg);
    return selector;
    }
    static void set_segment_selector(struct x86_emulate_ctxt *ctxt, u16 selector,
    unsigned seg)
    {
    u16 dummy;
    u32 base3;
    struct desc_struct desc;
    ctxt.ops.get_segment(ctxt, &dummy, &desc, &base3, seg);
    ctxt.ops.set_segment(ctxt, selector, &desc, base3, seg);
    }
#[no_mangle]
pub unsafe extern "C" fn ctxt_virt_addr_bits(ctxt: *mut x86_emulate_ctxt) -> u8 {
    static inline u8 ctxt_virt_addr_bits(struct x86_emulate_ctxt *ctxt)
    {
    return (ctxt.ops.get_cr(ctxt, 4) & X86_CR4_LA57) ? 57 : 48;
    }
    static inline bool emul_is_noncanonical_address(u64 la,
    struct x86_emulate_ctxt *ctxt,
    unsigned int flags)
    {
    return !ctxt.ops.is_canonical_addr(ctxt, la, flags);
    }
//
// x86 defines three classes of vector instructions: explicitly
// aligned, explicitly unaligned, and the rest, which change behaviour
// depending on whether they're AVX encoded or not.
//
// Also included is CMPXCHG16B which is not a vector instruction, yet it is
// subject to the same check.  FXSAVE and FXRSTOR are checked here too as their
// 512 bytes of data must be aligned to a 16 byte boundary.
//
#[no_mangle]
unsafe extern "C" fn insn_alignment(ctxt: *mut x86_emulate_ctxt, size: unsigned) -> unsigned {
    static unsigned insn_alignment(struct x86_emulate_ctxt *ctxt, unsigned size)
    {
    let mut alignment: u64 = ctxt.d & AlignMask;
    if (likely(size < 16))
    return 1;
    switch (alignment) {
    case Unaligned:
    return 1;
    case Aligned16:
    return 16;
    case Aligned:
    default:
    return size;
    }
    }
    static __always_inline int __linearize(struct x86_emulate_ctxt *ctxt,
    struct segmented_address addr,
    unsigned *max_size, unsigned size,
    enum x86emul_mode mode, ulong *linear,
    unsigned int flags)
    {
    struct desc_struct desc;
    bool usable;
    ulong la;
    u32 lim;
    u16 sel;
    u8  va_bits;
    la = seg_base(ctxt, addr.seg) + addr.ea;
// max_size = 0;
    switch (mode) {
    case X86EMUL_MODE_PROT64:
// linear = la = ctxt->ops->get_untagged_addr(ctxt, la, flags);
    va_bits = ctxt_virt_addr_bits(ctxt);
    if (!__is_canonical_address(la, va_bits))
    goto bad;
// max_size = min_t(u64, ~0u, (1ull << va_bits) - la);
    if (size > *max_size)
    goto bad;
    break;
    default:
// linear = la = (u32)la;
    usable = ctxt.ops.get_segment(ctxt, &sel, &desc, core::ptr::null_mut(),
    addr.seg);
    if (!usable)
    goto bad;
// code segment in protected mode or read-only data segment
    if ((((ctxt.mode != X86EMUL_MODE_REAL) && (desc.type & 8)) || !(desc.type & 2)) &&
    (flags & X86EMUL_F_WRITE))
    goto bad;
// unreadable code segment
    if (!(flags & X86EMUL_F_FETCH) && (desc.type & 8) && !(desc.type & 2))
    goto bad;
    lim = desc_limit_scaled(&desc);
    if (!(desc.type & 8) && (desc.type & 4)) {
// expand-down segment
    if (addr.ea <= lim)
    goto bad;
    lim = desc.d ? 0xffffffff : 0xffff;
    }
    if (addr.ea > lim)
    goto bad;
    if (lim == 0xffffffff)
// max_size = ~0u;
    else {
// max_size = (u64)lim + 1 - addr.ea;
    if (size > *max_size)
    goto bad;
    }
    break;
    }
    if (la & (insn_alignment(ctxt, size) - 1))
    return emulate_gp(ctxt, 0);
    return X86EMUL_CONTINUE;
    bad:
    if (addr.seg == VCPU_SREG_SS)
    return emulate_ss(ctxt, 0);
    else
    return emulate_gp(ctxt, 0);
    }
    static int linearize(struct x86_emulate_ctxt *ctxt,
    struct segmented_address addr,
    unsigned size, bool write,
    ulong *linear)
    {
    unsigned max_size;
    return __linearize(ctxt, addr, &max_size, size, ctxt.mode, linear,
    write ? X86EMUL_F_WRITE : 0);
    }
#[no_mangle]
pub unsafe extern "C" fn assign_eip(ctxt: *mut x86_emulate_ctxt, dst: c_ulong) -> c_int {
    static inline int assign_eip(struct x86_emulate_ctxt *ctxt, ulong dst)
    {
    ulong linear;
    int rc;
    unsigned max_size;
    struct segmented_address addr = { .seg = VCPU_SREG_CS,
    .ea = dst };
    if (ctxt.op_bytes != sizeof(unsigned long))
    addr.ea = dst & ((1UL << (ctxt.op_bytes << 3)) - 1);
    rc = __linearize(ctxt, addr, &max_size, 1, ctxt.mode, &linear,
    X86EMUL_F_FETCH);
    if (rc == X86EMUL_CONTINUE)
    ctxt._eip = addr.ea;
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn emulator_recalc_and_set_mode(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static inline int emulator_recalc_and_set_mode(struct x86_emulate_ctxt *ctxt)
    {
    u64 efer;
    struct desc_struct cs;
    u16 selector;
    u32 base3;
    ctxt.ops.get_msr(ctxt, MSR_EFER, &efer);
    if (!(ctxt.ops.get_cr(ctxt, 0) & X86_CR0_PE)) {
// Real mode. cpu must not have long mode active
    if (efer & EFER_LMA)
    return X86EMUL_UNHANDLEABLE;
    ctxt.mode = X86EMUL_MODE_REAL;
    return X86EMUL_CONTINUE;
    }
    if (ctxt.eflags & X86_EFLAGS_VM) {
// Protected/VM86 mode. cpu must not have long mode active
    if (efer & EFER_LMA)
    return X86EMUL_UNHANDLEABLE;
    ctxt.mode = X86EMUL_MODE_VM86;
    return X86EMUL_CONTINUE;
    }
    if (!ctxt.ops.get_segment(ctxt, &selector, &cs, &base3, VCPU_SREG_CS))
    return X86EMUL_UNHANDLEABLE;
    if (efer & EFER_LMA) {
    if (cs.l) {
// Proper long mode
    ctxt.mode = X86EMUL_MODE_PROT64;
    } else if (cs.d) {
// 32 bit compatibility mode
    ctxt.mode = X86EMUL_MODE_PROT32;
    } else {
    ctxt.mode = X86EMUL_MODE_PROT16;
    }
    } else {
// Legacy 32 bit / 16 bit mode
    ctxt.mode = cs.d ? X86EMUL_MODE_PROT32 : X86EMUL_MODE_PROT16;
    }
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
pub unsafe extern "C" fn assign_eip_near(ctxt: *mut x86_emulate_ctxt, dst: c_ulong) -> c_int {
    static inline int assign_eip_near(struct x86_emulate_ctxt *ctxt, ulong dst)
    {
    return assign_eip(ctxt, dst);
    }
#[no_mangle]
unsafe extern "C" fn assign_eip_far(ctxt: *mut x86_emulate_ctxt, dst: c_ulong) -> c_int {
    static int assign_eip_far(struct x86_emulate_ctxt *ctxt, ulong dst)
    {
    let mut rc: c_int = emulator_recalc_and_set_mode(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return assign_eip(ctxt, dst);
    }
#[no_mangle]
pub unsafe extern "C" fn jmp_rel(ctxt: *mut x86_emulate_ctxt, rel: c_int) -> c_int {
    static inline int jmp_rel(struct x86_emulate_ctxt *ctxt, int rel)
    {
    return assign_eip_near(ctxt, ctxt._eip + rel);
    }
    static int linear_read_system(struct x86_emulate_ctxt *ctxt, ulong linear,
    void *data, unsigned size)
    {
    return ctxt.ops.read_std(ctxt, linear, data, size, &ctxt.exception, true);
    }
    static int linear_write_system(struct x86_emulate_ctxt *ctxt,
    ulong linear, void *data,
    unsigned int size)
    {
    return ctxt.ops.write_std(ctxt, linear, data, size, &ctxt.exception, true);
    }
    static int segmented_read_std(struct x86_emulate_ctxt *ctxt,
    struct segmented_address addr,
    void *data,
    unsigned size)
    {
    int rc;
    ulong linear;
    rc = linearize(ctxt, addr, size, false, &linear);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return ctxt.ops.read_std(ctxt, linear, data, size, &ctxt.exception, false);
    }
    static int segmented_write_std(struct x86_emulate_ctxt *ctxt,
    struct segmented_address addr,
    void *data,
    unsigned int size)
    {
    int rc;
    ulong linear;
    rc = linearize(ctxt, addr, size, true, &linear);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return ctxt.ops.write_std(ctxt, linear, data, size, &ctxt.exception, false);
    }
//
// Prefetch the remaining bytes of the instruction without crossing page
// boundary if they are not in fetch_cache yet.
//
#[no_mangle]
unsafe extern "C" fn __do_insn_fetch_bytes(ctxt: *mut x86_emulate_ctxt, op_size: c_int) -> c_int {
    static int __do_insn_fetch_bytes(struct x86_emulate_ctxt *ctxt, int op_size)
    {
    int rc;
    unsigned size, max_size;
    unsigned long linear;
    let mut cur_size: c_int = ctxt.fetch.end - ctxt.fetch.data;
    struct segmented_address addr = { .seg = VCPU_SREG_CS,
    .ea = ctxt.eip + cur_size };
//
// We do not know exactly how many bytes will be needed, and
// __linearize is expensive, so fetch as much as possible.  We
// just have to avoid going beyond the 15 byte limit, the end
// of the segment, or the end of the page.
//
// __linearize is called with size 0 so that it does not do any
// boundary check itself.  Instead, we use max_size to check
// against op_size.
//
    rc = __linearize(ctxt, addr, &max_size, 0, ctxt.mode, &linear,
    X86EMUL_F_FETCH);
    if (unlikely(rc != X86EMUL_CONTINUE))
    return rc;
    size = min_t(unsigned, 15UL ^ cur_size, max_size);
    size = min_t(unsigned, size, PAGE_SIZE - offset_in_page(linear));
//
// One instruction can only straddle two pages,
// and one has been loaded at the beginning of
// x86_decode_insn.  So, if not enough bytes
// still, we must have hit the 15-byte boundary.
//
    if (unlikely(size < op_size))
    return emulate_gp(ctxt, 0);
    rc = ctxt.ops.fetch(ctxt, linear, ctxt.fetch.end,
    size, &ctxt.exception);
    if (unlikely(rc != X86EMUL_CONTINUE))
    return rc;
    ctxt.fetch.end += size;
    return X86EMUL_CONTINUE;
    }
    static __always_inline int do_insn_fetch_bytes(struct x86_emulate_ctxt *ctxt,
    unsigned size)
    {
    let mut done_size: unsigned = ctxt.fetch.end - ctxt.fetch.ptr;
    if (unlikely(done_size < size))
    return __do_insn_fetch_bytes(ctxt, size - done_size);
    else
    return X86EMUL_CONTINUE;
    }
// Fetch next part of the instruction being emulated.

    ({	_type _x;							\
    \
    rc = do_insn_fetch_bytes(_ctxt, sizeof(_type));			\
    if (rc != X86EMUL_CONTINUE)					\
    goto done;						\
    ctxt._eip += sizeof(_type);					\
    memcpy(&_x, ctxt.fetch.ptr, sizeof(_type));			\
    ctxt.fetch.ptr += sizeof(_type);				\
    _x;								\
    })

    ({									\
    rc = do_insn_fetch_bytes(_ctxt, _size);				\
    if (rc != X86EMUL_CONTINUE)					\
    goto done;						\
    ctxt._eip += (_size);						\
    memcpy(_arr, ctxt.fetch.ptr, _size);				\
    ctxt.fetch.ptr += (_size);					\
    })
//
// Given the 'reg' portion of a ModRM byte, and a register block, return a
// pointer into the block that addresses the relevant register.
// @highbyte_regs specifies whether to decode AH,CH,DH,BH.
//
    static void *decode_register(struct x86_emulate_ctxt *ctxt, u8 modrm_reg,
    int byteop)
    {
    void *p;
    let mut highbyte_regs: c_int = (ctxt.rex_prefix == REX_NONE) && byteop;
    if (highbyte_regs && modrm_reg >= 4 && modrm_reg < 8)
    p = (unsigned char *)reg_rmw(ctxt, modrm_reg & 3) + 1;
    else
    p = reg_rmw(ctxt, modrm_reg);
    return p;
    }
    static int read_descriptor(struct x86_emulate_ctxt *ctxt,
    struct segmented_address addr,
    u16 *size, unsigned long *address, int op_bytes)
    {
    int rc;
    if (op_bytes == 2)
    op_bytes = 3;
// address = 0;
    rc = segmented_read_std(ctxt, addr, size, 2);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    addr.ea += 2;
    rc = segmented_read_std(ctxt, addr, address, op_bytes);
    return rc;
    }
    EM_ASM_2(add);
    EM_ASM_2(or);
    EM_ASM_2(adc);
    EM_ASM_2(sbb);
    EM_ASM_2(and);
    EM_ASM_2(sub);
    EM_ASM_2(xor);
    EM_ASM_2(cmp);
    EM_ASM_2(test);
    EM_ASM_2(xadd);
    EM_ASM_1SRC2(mul, mul_ex);
    EM_ASM_1SRC2(imul, imul_ex);
    EM_ASM_1SRC2EX(div, div_ex);
    EM_ASM_1SRC2EX(idiv, idiv_ex);
    EM_ASM_3WCL(shld);
    EM_ASM_3WCL(shrd);
    EM_ASM_2W(imul);
    EM_ASM_1(not);
    EM_ASM_1(neg);
    EM_ASM_1(inc);
    EM_ASM_1(dec);
    EM_ASM_2CL(rol);
    EM_ASM_2CL(ror);
    EM_ASM_2CL(rcl);
    EM_ASM_2CL(rcr);
    EM_ASM_2CL(shl);
    EM_ASM_2CL(shr);
    EM_ASM_2CL(sar);
    EM_ASM_2W(bsf);
    EM_ASM_2W(bsr);
    EM_ASM_2W(bt);
    EM_ASM_2W(bts);
    EM_ASM_2W(btr);
    EM_ASM_2W(btc);
    EM_ASM_2R(cmp, cmp_r);
#[no_mangle]
unsafe extern "C" fn em_bsf_c(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_bsf_c(struct x86_emulate_ctxt *ctxt)
    {
// If src is zero, do not writeback, but update flags
    if (ctxt.src.val == 0)
    ctxt.dst.type = OP_NONE;
    return em_bsf(ctxt);
    }
#[no_mangle]
unsafe extern "C" fn em_bsr_c(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_bsr_c(struct x86_emulate_ctxt *ctxt)
    {
// If src is zero, do not writeback, but update flags
    if (ctxt.src.val == 0)
    ctxt.dst.type = OP_NONE;
    return em_bsr(ctxt);
    }
#[no_mangle]
unsafe extern "C" fn test_cc(condition: c_uint, flags: c_ulong) -> __always_inline u8 {
    static __always_inline u8 test_cc(unsigned int condition, unsigned long flags)
    {
    return __emulate_cc(flags, condition & 0xf);
    }
#[no_mangle]
unsafe extern "C" fn fetch_register_operand(op: *mut operand) {
    static void fetch_register_operand(struct operand *op)
    {
    switch (op.bytes) {
    case 1:
    op.val = *(u8 *)op.addr.reg;
    break;
    case 2:
    op.val = *(u16 *)op.addr.reg;
    break;
    case 4:
    op.val = *(u32 *)op.addr.reg;
    break;
    case 8:
    op.val = *(u64 *)op.addr.reg;
    break;
    }
    op.orig_val = op.val;
    }
#[no_mangle]
unsafe extern "C" fn em_fninit(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_fninit(struct x86_emulate_ctxt *ctxt)
    {
    if (ctxt.ops.get_cr(ctxt, 0) & (X86_CR0_TS | X86_CR0_EM))
    return emulate_nm(ctxt);
    kvm_fpu_get();
    asm volatile("fninit");
    kvm_fpu_put();
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_fnstcw(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_fnstcw(struct x86_emulate_ctxt *ctxt)
    {
    u16 fcw;
    if (ctxt.ops.get_cr(ctxt, 0) & (X86_CR0_TS | X86_CR0_EM))
    return emulate_nm(ctxt);
    kvm_fpu_get();
    asm volatile("fnstcw %0": "+m"(fcw));
    kvm_fpu_put();
    ctxt.dst.val = fcw;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_fnstsw(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_fnstsw(struct x86_emulate_ctxt *ctxt)
    {
    u16 fsw;
    if (ctxt.ops.get_cr(ctxt, 0) & (X86_CR0_TS | X86_CR0_EM))
    return emulate_nm(ctxt);
    kvm_fpu_get();
    asm volatile("fnstsw %0": "+m"(fsw));
    kvm_fpu_put();
    ctxt.dst.val = fsw;
    return X86EMUL_CONTINUE;
    }
    static void __decode_register_operand(struct x86_emulate_ctxt *ctxt,
    struct operand *op, int reg)
    {
    if ((ctxt.d & Avx) && ctxt.op_bytes == 32) {
    op.type = OP_YMM;
    op.bytes = 32;
    op.addr.xmm = reg;
    kvm_read_avx_reg(reg, &op.vec_val2);
    return;
    }
    if (ctxt.d & (Avx|Sse)) {
    op.type = OP_XMM;
    op.bytes = 16;
    op.addr.xmm = reg;
    kvm_read_sse_reg(reg, &op.vec_val);
    return;
    }
    if (ctxt.d & Mmx) {
    reg &= 7;
    op.type = OP_MM;
    op.bytes = 8;
    op.addr.mm = reg;
    return;
    }
    op.type = OP_REG;
    op.bytes = (ctxt.d & ByteOp) ? 1 : ctxt.op_bytes;
    op.addr.reg = decode_register(ctxt, reg, ctxt.d & ByteOp);
    fetch_register_operand(op);
    }
    static void decode_register_operand(struct x86_emulate_ctxt *ctxt,
    struct operand *op)
    {
    unsigned int reg;
    if (ctxt.d & ModRM)
    reg = ctxt.modrm_reg;
    else
    reg = (ctxt.b & 7) | (ctxt.rex_bits & REX_B ? 8 : 0);
    __decode_register_operand(ctxt, op, reg);
    }
#[no_mangle]
unsafe extern "C" fn adjust_modrm_seg(ctxt: *mut x86_emulate_ctxt, base_reg: c_int) {
    static void adjust_modrm_seg(struct x86_emulate_ctxt *ctxt, int base_reg)
    {
    if (base_reg == VCPU_REGS_RSP || base_reg == VCPU_REGS_RBP)
    ctxt.modrm_seg = VCPU_SREG_SS;
    }
    static int decode_modrm(struct x86_emulate_ctxt *ctxt,
    struct operand *op)
    {
    u8 sib;
    int index_reg, base_reg, scale;
    let mut rc: c_int = X86EMUL_CONTINUE;
    let mut modrm_ea: c_ulong = 0;
    ctxt.modrm_reg = (ctxt.rex_bits & REX_R ? 8 : 0);
    index_reg = (ctxt.rex_bits & REX_X ? 8 : 0);
    base_reg = (ctxt.rex_bits & REX_B ? 8 : 0);
    ctxt.modrm_mod = (ctxt.modrm & 0xc0) >> 6;
    ctxt.modrm_reg |= (ctxt.modrm & 0x38) >> 3;
    ctxt.modrm_rm = base_reg | (ctxt.modrm & 0x07);
    ctxt.modrm_seg = VCPU_SREG_DS;
    if (ctxt.modrm_mod == 3 || (ctxt.d & NoMod)) {
    __decode_register_operand(ctxt, op, ctxt.modrm_rm);
    return rc;
    }
    op.type = OP_MEM;
    if (ctxt.ad_bytes == 2) {
    let mut bx: unsigned = reg_read(ctxt, VCPU_REGS_RBX);
    let mut bp: unsigned = reg_read(ctxt, VCPU_REGS_RBP);
    let mut si: unsigned = reg_read(ctxt, VCPU_REGS_RSI);
    let mut di: unsigned = reg_read(ctxt, VCPU_REGS_RDI);
// 16-bit ModR/M decode.
    switch (ctxt.modrm_mod) {
    case 0:
    if (ctxt.modrm_rm == 6)
    modrm_ea += insn_fetch(u16, ctxt);
    break;
    case 1:
    modrm_ea += insn_fetch(s8, ctxt);
    break;
    case 2:
    modrm_ea += insn_fetch(u16, ctxt);
    break;
    }
    switch (ctxt.modrm_rm) {
    case 0:
    modrm_ea += bx + si;
    break;
    case 1:
    modrm_ea += bx + di;
    break;
    case 2:
    modrm_ea += bp + si;
    break;
    case 3:
    modrm_ea += bp + di;
    break;
    case 4:
    modrm_ea += si;
    break;
    case 5:
    modrm_ea += di;
    break;
    case 6:
    if (ctxt.modrm_mod != 0)
    modrm_ea += bp;
    break;
    case 7:
    modrm_ea += bx;
    break;
    }
    if (ctxt.modrm_rm == 2 || ctxt.modrm_rm == 3 ||
    (ctxt.modrm_rm == 6 && ctxt.modrm_mod != 0))
    ctxt.modrm_seg = VCPU_SREG_SS;
    modrm_ea = (u16)modrm_ea;
    } else {
// 32/64-bit ModR/M decode.
    if ((ctxt.modrm_rm & 7) == 4) {
    sib = insn_fetch(u8, ctxt);
    index_reg |= (sib >> 3) & 7;
    base_reg |= sib & 7;
    scale = sib >> 6;
    if ((base_reg & 7) == 5 && ctxt.modrm_mod == 0)
    modrm_ea += insn_fetch(s32, ctxt);
    else {
    modrm_ea += reg_read(ctxt, base_reg);
    adjust_modrm_seg(ctxt, base_reg);
// Increment ESP on POP [ESP]
    if ((ctxt.d & IncSP) &&
    base_reg == VCPU_REGS_RSP)
    modrm_ea += ctxt.op_bytes;
    }
    if (index_reg != 4)
    modrm_ea += reg_read(ctxt, index_reg) << scale;
    } else if ((ctxt.modrm_rm & 7) == 5 && ctxt.modrm_mod == 0) {
    modrm_ea += insn_fetch(s32, ctxt);
    if (ctxt.mode == X86EMUL_MODE_PROT64)
    ctxt.rip_relative = 1;
    } else {
    base_reg = ctxt.modrm_rm;
    modrm_ea += reg_read(ctxt, base_reg);
    adjust_modrm_seg(ctxt, base_reg);
    }
    switch (ctxt.modrm_mod) {
    case 1:
    modrm_ea += insn_fetch(s8, ctxt);
    break;
    case 2:
    modrm_ea += insn_fetch(s32, ctxt);
    break;
    }
    }
    op.addr.mem.ea = modrm_ea;
    if (ctxt.ad_bytes != 8)
    ctxt.memop.addr.mem.ea = (u32)ctxt.memop.addr.mem.ea;
    done:
    return rc;
    }
    static int decode_abs(struct x86_emulate_ctxt *ctxt,
    struct operand *op)
    {
    let mut rc: c_int = X86EMUL_CONTINUE;
    op.type = OP_MEM;
    switch (ctxt.ad_bytes) {
    case 2:
    op.addr.mem.ea = insn_fetch(u16, ctxt);
    break;
    case 4:
    op.addr.mem.ea = insn_fetch(u32, ctxt);
    break;
    case 8:
    op.addr.mem.ea = insn_fetch(u64, ctxt);
    break;
    }
    done:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn fetch_bit_operand(ctxt: *mut x86_emulate_ctxt) {
    static void fetch_bit_operand(struct x86_emulate_ctxt *ctxt)
    {
    let mut sv: c_long = 0, mask;
    if (ctxt.dst.type == OP_MEM && ctxt.src.type == OP_REG) {
    mask = ~((long)ctxt.dst.bytes * 8 - 1);
    if (ctxt.src.bytes == 2)
    sv = (s16)ctxt.src.val & (s16)mask;
#[no_mangle]
pub unsafe extern "C" fn if(4: ctxt->src.bytes ==) -> else {
    else if (ctxt.src.bytes == 4)
    sv = (s32)ctxt.src.val & (s32)mask;
    else
    sv = (s64)ctxt.src.val & (s64)mask;
    ctxt.dst.addr.mem.ea = address_mask(ctxt,
    ctxt.dst.addr.mem.ea + (sv >> 3));
    }
// only subword offset
    ctxt.src.val &= (ctxt.dst.bytes << 3) - 1;
    }
    static int read_emulated(struct x86_emulate_ctxt *ctxt,
    unsigned long addr, void *dest, unsigned size)
    {
    int rc;
    struct read_cache *mc = &ctxt.mem_read;
//
// If the read gets a cache hit, simply copy the value from the cache.
// A "hit" here means that there is unused data in the cache, i.e. when
// re-emulating an instruction to complete a userspace exit, KVM relies
// on "no decode" to ensure the instruction is re-emulated in the same
// sequence, so that multiple reads are fulfilled in the correct order.
//
    if (mc.pos < mc.end)
    goto read_cached;
    if (KVM_EMULATOR_BUG_ON((mc.end + size) >= sizeof(mc.data), ctxt))
    return X86EMUL_UNHANDLEABLE;
//
// Route all reads to the cache.  This allows @dest to be an on-stack
// variable without triggering use-after-free if KVM needs to exit to
// userspace to handle an MMIO read (the MMIO fragment will point at
// the current location in the cache).
//
    rc = ctxt.ops.read_emulated(ctxt, addr, mc.data + mc.end, size,
    &ctxt.exception);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    mc.end += size;
    read_cached:
    memcpy(dest, mc.data + mc.pos, size);
    mc.pos += size;
    return X86EMUL_CONTINUE;
    }
    static int segmented_read(struct x86_emulate_ctxt *ctxt,
    struct segmented_address addr,
    void *data,
    unsigned size)
    {
    int rc;
    ulong linear;
    rc = linearize(ctxt, addr, size, false, &linear);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return read_emulated(ctxt, linear, data, size);
    }
    static int segmented_write(struct x86_emulate_ctxt *ctxt,
    struct segmented_address addr,
    const void *data,
    unsigned size)
    {
    int rc;
    ulong linear;
    rc = linearize(ctxt, addr, size, true, &linear);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return ctxt.ops.write_emulated(ctxt, linear, data, size,
    &ctxt.exception);
    }
    static int segmented_cmpxchg(struct x86_emulate_ctxt *ctxt,
    struct segmented_address addr,
    const void *orig_data, const void *data,
    unsigned size)
    {
    int rc;
    ulong linear;
    rc = linearize(ctxt, addr, size, true, &linear);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return ctxt.ops.cmpxchg_emulated(ctxt, linear, orig_data, data,
    size, &ctxt.exception);
    }
    static int pio_in_emulated(struct x86_emulate_ctxt *ctxt,
    unsigned int size, unsigned short port,
    void *dest)
    {
    struct read_cache *rc = &ctxt.io_read;
    if (rc.pos == rc.end) { /* refill pio read ahead */
    unsigned int in_page, n;
    unsigned int count = ctxt.rep_prefix ?
    address_mask(ctxt, reg_read(ctxt, VCPU_REGS_RCX)) : 1;
    in_page = (ctxt.eflags & X86_EFLAGS_DF) ?
    offset_in_page(reg_read(ctxt, VCPU_REGS_RDI)) :
    PAGE_SIZE - offset_in_page(reg_read(ctxt, VCPU_REGS_RDI));
    n = min3(in_page, (unsigned int)sizeof(rc.data) / size, count);
    if (n == 0)
    n = 1;
    rc.pos = rc.end = 0;
    if (!ctxt.ops.pio_in_emulated(ctxt, size, port, rc.data, n))
    return 0;
    rc.end = n * size;
    }
    if (ctxt.rep_prefix && (ctxt.d & String) &&
    !(ctxt.eflags & X86_EFLAGS_DF)) {
    ctxt.dst.data = rc.data + rc.pos;
    ctxt.dst.type = OP_MEM_STR;
    ctxt.dst.count = (rc.end - rc.pos) / size;
    rc.pos = rc.end;
    } else {
    memcpy(dest, rc.data + rc.pos, size);
    rc.pos += size;
    }
    return 1;
    }
    static int read_interrupt_descriptor(struct x86_emulate_ctxt *ctxt,
    u16 index, struct desc_struct *desc)
    {
    struct desc_ptr dt;
    ulong addr;
    ctxt.ops.get_idt(ctxt, &dt);
    if (dt.size < index * 8 + 7)
    return emulate_gp(ctxt, index << 3 | 0x2);
    addr = dt.address + index * 8;
    return linear_read_system(ctxt, addr, desc, sizeof(*desc));
    }
    static void get_descriptor_table_ptr(struct x86_emulate_ctxt *ctxt,
    u16 selector, struct desc_ptr *dt)
    {
    const struct x86_emulate_ops *ops = ctxt.ops;
    let mut base3: u32 = 0;
    if (selector & 1 << 2) {
    struct desc_struct desc;
    u16 sel;
    memset(dt, 0, sizeof(*dt));
    if (!ops.get_segment(ctxt, &sel, &desc, &base3,
    VCPU_SREG_LDTR))
    return;
    dt.size = desc_limit_scaled(&desc); /* what if limit > 65535? */
    dt.address = get_desc_base(&desc) | ((u64)base3 << 32);
    } else
    ops.get_gdt(ctxt, dt);
    }
    static int get_descriptor_ptr(struct x86_emulate_ctxt *ctxt,
    u16 selector, ulong *desc_addr_p)
    {
    struct desc_ptr dt;
    let mut index: u16 = selector >> 3;
    ulong addr;
    get_descriptor_table_ptr(ctxt, selector, &dt);
    if (dt.size < index * 8 + 7)
    return emulate_gp(ctxt, selector & 0xfffc);
    addr = dt.address + index * 8;

    if (addr >> 32 != 0) {
    let mut efer: u64 = 0;
    ctxt.ops.get_msr(ctxt, MSR_EFER, &efer);
    if (!(efer & EFER_LMA))
    addr &= (u32)-1;
    }

// desc_addr_p = addr;
    return X86EMUL_CONTINUE;
    }
// allowed just for 8 bytes segments
    static int read_segment_descriptor(struct x86_emulate_ctxt *ctxt,
    u16 selector, struct desc_struct *desc,
    ulong *desc_addr_p)
    {
    int rc;
    rc = get_descriptor_ptr(ctxt, selector, desc_addr_p);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return linear_read_system(ctxt, *desc_addr_p, desc, sizeof(*desc));
    }
// allowed just for 8 bytes segments
    static int write_segment_descriptor(struct x86_emulate_ctxt *ctxt,
    u16 selector, struct desc_struct *desc)
    {
    int rc;
    ulong addr;
    rc = get_descriptor_ptr(ctxt, selector, &addr);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return linear_write_system(ctxt, addr, desc, sizeof(*desc));
    }
#[no_mangle]
unsafe extern "C" fn emulator_is_ssp_invalid(ctxt: *mut x86_emulate_ctxt, cpl: u8) -> bool {
    static bool emulator_is_ssp_invalid(struct x86_emulate_ctxt *ctxt, u8 cpl)
    {
    let mut MSR_IA32_X_CET: u32 = cpl == 3 ? MSR_IA32_U_CET : MSR_IA32_S_CET;
    let mut efer: u64 = 0, cet = 0, ssp = 0;
    if (!(ctxt.ops.get_cr(ctxt, 4) & X86_CR4_CET))
    return false;
    if (ctxt.ops.get_msr(ctxt, MSR_EFER, &efer))
    return true;
// SSP is guaranteed to be valid if the vCPU was already in 32-bit mode.
    if (!(efer & EFER_LMA))
    return false;
    if (ctxt.ops.get_msr(ctxt, MSR_IA32_X_CET, &cet))
    return true;
    if (!(cet & CET_SHSTK_EN))
    return false;
    if (ctxt.ops.get_msr(ctxt, MSR_KVM_INTERNAL_GUEST_SSP, &ssp))
    return true;
//
// On transfer from 64-bit mode to compatibility mode, SSP[63:32] must
// be 0, i.e. SSP must be a 32-bit value outside of 64-bit mode.
//
    return ssp >> 32;
    }
    static int __load_segment_descriptor(struct x86_emulate_ctxt *ctxt,
    u16 selector, int seg, u8 cpl,
    enum x86_transfer_type transfer,
    struct desc_struct *desc)
    {
    struct desc_struct seg_desc, old_desc;
    u8 dpl, rpl;
    let mut err_vec: unsigned = GP_VECTOR;
    let mut err_code: u32 = 0;
    bool null_selector = !(selector & ~0x3); /* 0000-0003 are null */
    ulong desc_addr;
    int ret;
    u16 dummy;
    let mut base3: u32 = 0;
    memset(&seg_desc, 0, sizeof(seg_desc));
    if (ctxt.mode == X86EMUL_MODE_REAL) {
// set real mode segment descriptor (keep limit etc. for
// unreal mode)
    ctxt.ops.get_segment(ctxt, &dummy, &seg_desc, core::ptr::null_mut(), seg);
    set_desc_base(&seg_desc, selector << 4);
    goto load;
    } else if (seg <= VCPU_SREG_GS && ctxt.mode == X86EMUL_MODE_VM86) {
// VM86 needs a clean new segment descriptor
    set_desc_base(&seg_desc, selector << 4);
    set_desc_limit(&seg_desc, 0xffff);
    seg_desc.type = 3;
    seg_desc.p = 1;
    seg_desc.s = 1;
    seg_desc.dpl = 3;
    goto load;
    }
    rpl = selector & 3;
// TR should be in GDT only
    if (seg == VCPU_SREG_TR && (selector & (1 << 2)))
    goto exception;
// NULL selector is not valid for TR, CS and (except for long mode) SS
    if (null_selector) {
    if (seg == VCPU_SREG_CS || seg == VCPU_SREG_TR)
    goto exception;
    if (seg == VCPU_SREG_SS) {
    if (ctxt.mode != X86EMUL_MODE_PROT64 || rpl != cpl)
    goto exception;
//
// ctxt->ops->set_segment expects the CPL to be in
// SS.DPL, so fake an expand-up 32-bit data segment.
//
    seg_desc.type = 3;
    seg_desc.p = 1;
    seg_desc.s = 1;
    seg_desc.dpl = cpl;
    seg_desc.d = 1;
    seg_desc.g = 1;
    }
// Skip all following checks
    goto load;
    }
    ret = read_segment_descriptor(ctxt, selector, &seg_desc, &desc_addr);
    if (ret != X86EMUL_CONTINUE)
    return ret;
    err_code = selector & 0xfffc;
    err_vec = (transfer == X86_TRANSFER_TASK_SWITCH) ? TS_VECTOR :
    GP_VECTOR;
// can't load system descriptor into segment selector
    if (seg <= VCPU_SREG_GS && !seg_desc.s) {
    if (transfer == X86_TRANSFER_CALL_JMP)
    return X86EMUL_UNHANDLEABLE;
    goto exception;
    }
    dpl = seg_desc.dpl;
    switch (seg) {
    case VCPU_SREG_SS:
//
// segment is not a writable data segment or segment
// selector's RPL != CPL or DPL != CPL
//
    if (rpl != cpl || (seg_desc.type & 0xa) != 0x2 || dpl != cpl)
    goto exception;
    break;
    case VCPU_SREG_CS:
//
// KVM uses "none" when loading CS as part of emulating Real
// Mode exceptions and IRET (handled above).  In all other
// cases, loading CS without a control transfer is a KVM bug.
//
    if (WARN_ON_ONCE(transfer == X86_TRANSFER_NONE))
    goto exception;
    if (!(seg_desc.type & 8))
    goto exception;
    if (transfer == X86_TRANSFER_RET) {
// RET can never return to an inner privilege level.
    if (rpl < cpl)
    goto exception;
// Outer-privilege level return is not implemented
    if (rpl > cpl)
    return X86EMUL_UNHANDLEABLE;
    }
    if (transfer == X86_TRANSFER_RET || transfer == X86_TRANSFER_TASK_SWITCH) {
    if (seg_desc.type & 4) {
// conforming
    if (dpl > rpl)
    goto exception;
    } else {
// nonconforming
    if (dpl != rpl)
    goto exception;
    }
    } else { /* X86_TRANSFER_CALL_JMP */
    if (seg_desc.type & 4) {
// conforming
    if (dpl > cpl)
    goto exception;
    } else {
// nonconforming
    if (rpl > cpl || dpl != cpl)
    goto exception;
    }
    }
// in long-mode d/b must be clear if l is set
    if (seg_desc.d && seg_desc.l) {
    let mut efer: u64 = 0;
    ctxt.ops.get_msr(ctxt, MSR_EFER, &efer);
    if (efer & EFER_LMA)
    goto exception;
    }
    if (!seg_desc.l && emulator_is_ssp_invalid(ctxt, cpl)) {
    err_code = 0;
    goto exception;
    }
// CS(RPL) <- CPL
    selector = (selector & 0xfffc) | cpl;
    break;
    case VCPU_SREG_TR:
    if (seg_desc.s || (seg_desc.type != 1 && seg_desc.type != 9))
    goto exception;
    break;
    case VCPU_SREG_LDTR:
    if (seg_desc.s || seg_desc.type != 2)
    goto exception;
    break;
    default: /*  DS, ES, FS, or GS */
//
// segment is not a data or readable code segment or
// ((segment is a data or nonconforming code segment)
// and ((RPL > DPL) or (CPL > DPL)))
//
    if ((seg_desc.type & 0xa) == 0x8 ||
    (((seg_desc.type & 0xc) != 0xc) &&
    (rpl > dpl || cpl > dpl)))
    goto exception;
    break;
    }
    if (!seg_desc.p) {
    err_vec = (seg == VCPU_SREG_SS) ? SS_VECTOR : NP_VECTOR;
    goto exception;
    }
    if (seg_desc.s) {
// mark segment as accessed
    if (!(seg_desc.type & 1)) {
    seg_desc.type |= 1;
    ret = write_segment_descriptor(ctxt, selector,
    &seg_desc);
    if (ret != X86EMUL_CONTINUE)
    return ret;
    }
    } else if (ctxt.mode == X86EMUL_MODE_PROT64) {
    ret = linear_read_system(ctxt, desc_addr+8, &base3, sizeof(base3));
    if (ret != X86EMUL_CONTINUE)
    return ret;
    if (emul_is_noncanonical_address(get_desc_base(&seg_desc) |
    ((u64)base3 << 32), ctxt,
    X86EMUL_F_DT_LOAD))
    return emulate_gp(ctxt, err_code);
    }
    if (seg == VCPU_SREG_TR) {
    old_desc = seg_desc;
    seg_desc.type |= 2; /* busy */
    ret = ctxt.ops.cmpxchg_emulated(ctxt, desc_addr, &old_desc, &seg_desc,
    sizeof(seg_desc), &ctxt.exception);
    if (ret != X86EMUL_CONTINUE)
    return ret;
    }
    load:
    ctxt.ops.set_segment(ctxt, selector, &seg_desc, base3, seg);
    if (desc)
// desc = seg_desc;
    return X86EMUL_CONTINUE;
    exception:
    return emulate_exception(ctxt, err_vec, err_code, true);
    }
    static int load_segment_descriptor(struct x86_emulate_ctxt *ctxt,
    u16 selector, int seg)
    {
    let mut cpl: u8 = ctxt.ops.cpl(ctxt);
//
// None of MOV, POP and LSS can load a NULL selector in CPL=3, but
// they can load it at CPL<3 (Intel's manual says only LSS can,
// but it's wrong).
//
// However, the Intel manual says that putting IST=1/DPL=3 in
// an interrupt gate will result in SS=3 (the AMD manual instead
// says it doesn't), so allow SS=3 in __load_segment_descriptor
// and only forbid it here.
//
    if (seg == VCPU_SREG_SS && selector == 3 &&
    ctxt.mode == X86EMUL_MODE_PROT64)
    return emulate_exception(ctxt, GP_VECTOR, 0, true);
    return __load_segment_descriptor(ctxt, selector, seg, cpl,
    X86_TRANSFER_NONE, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn write_register_operand(op: *mut operand) {
    static void write_register_operand(struct operand *op)
    {
    return insn_assign_reg(op.addr.reg, op.val, op.bytes);
    }
#[no_mangle]
unsafe extern "C" fn writeback(ctxt: *mut x86_emulate_ctxt, op: *mut operand) -> c_int {
    static int writeback(struct x86_emulate_ctxt *ctxt, struct operand *op)
    {
    switch (op.type) {
    case OP_REG:
    write_register_operand(op);
    break;
    case OP_MEM:
    if (ctxt.lock_prefix)
    return segmented_cmpxchg(ctxt,
    op.addr.mem,
    &op.orig_val,
    &op.val,
    op.bytes);
    else
    return segmented_write(ctxt,
    op.addr.mem,
    &op.val,
    op.bytes);
    case OP_MEM_STR:
    return segmented_write(ctxt,
    op.addr.mem,
    op.data,
    op.bytes * op.count);
    case OP_XMM:
    if (!(ctxt.d & Avx)) {
    kvm_write_sse_reg(op.addr.xmm, &op.vec_val);
    break;
    }
// full YMM write but with high bytes cleared
    memset(op.valptr + 16, 0, 16);
    fallthrough;
    case OP_YMM:
    kvm_write_avx_reg(op.addr.xmm, &op.vec_val2);
    break;
    case OP_MM:
    kvm_write_mmx_reg(op.addr.mm, &op.mm_val);
    break;
    case OP_NONE:
// no writeback
    break;
    default:
    break;
    }
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn emulate_push(ctxt: *mut x86_emulate_ctxt, data: *const c_void, len: c_int) -> c_int {
    static int emulate_push(struct x86_emulate_ctxt *ctxt, const void *data, int len)
    {
    struct segmented_address addr;
    rsp_increment(ctxt, -len);
    addr.ea = reg_read(ctxt, VCPU_REGS_RSP) & stack_mask(ctxt);
    addr.seg = VCPU_SREG_SS;
    return segmented_write(ctxt, addr, data, len);
    }
#[no_mangle]
unsafe extern "C" fn em_push(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_push(struct x86_emulate_ctxt *ctxt)
    {
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return emulate_push(ctxt, &ctxt.src.val, ctxt.op_bytes);
    }
    static int emulate_pop(struct x86_emulate_ctxt *ctxt,
    void *dest, int len)
    {
    int rc;
    struct segmented_address addr;
    addr.ea = reg_read(ctxt, VCPU_REGS_RSP) & stack_mask(ctxt);
    addr.seg = VCPU_SREG_SS;
    rc = segmented_read(ctxt, addr, dest, len);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rsp_increment(ctxt, len);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_pop(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_pop(struct x86_emulate_ctxt *ctxt)
    {
    return emulate_pop(ctxt, &ctxt.dst.val, ctxt.op_bytes);
    }
    static int emulate_popf(struct x86_emulate_ctxt *ctxt,
    void *dest, int len)
    {
    int rc;
    let mut val: c_ulong = 0;
    unsigned long change_mask;
    let mut iopl: c_int = (ctxt.eflags & X86_EFLAGS_IOPL) >> X86_EFLAGS_IOPL_BIT;
    let mut cpl: c_int = ctxt.ops.cpl(ctxt);
    rc = emulate_pop(ctxt, &val, len);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    change_mask = X86_EFLAGS_CF | X86_EFLAGS_PF | X86_EFLAGS_AF |
    X86_EFLAGS_ZF | X86_EFLAGS_SF | X86_EFLAGS_OF |
    X86_EFLAGS_TF | X86_EFLAGS_DF | X86_EFLAGS_NT |
    X86_EFLAGS_AC | X86_EFLAGS_ID;
    switch(ctxt.mode) {
    case X86EMUL_MODE_PROT64:
    case X86EMUL_MODE_PROT32:
    case X86EMUL_MODE_PROT16:
    if (cpl == 0)
    change_mask |= X86_EFLAGS_IOPL;
    if (cpl <= iopl)
    change_mask |= X86_EFLAGS_IF;
    break;
    case X86EMUL_MODE_VM86:
    if (iopl < 3)
    return emulate_gp(ctxt, 0);
    change_mask |= X86_EFLAGS_IF;
    break;
    default: /* real mode */
    change_mask |= (X86_EFLAGS_IOPL | X86_EFLAGS_IF);
    break;
    }
// (unsigned long *)dest =
    (ctxt.eflags & ~change_mask) | (val & change_mask);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_popf(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_popf(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.dst.type = OP_REG;
    ctxt.dst.addr.reg = &ctxt.eflags;
    ctxt.dst.bytes = ctxt.op_bytes;
    return emulate_popf(ctxt, &ctxt.dst.val, ctxt.op_bytes);
    }
#[no_mangle]
unsafe extern "C" fn em_enter(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_enter(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    let mut frame_size: unsigned = ctxt.src.val;
    let mut nesting_level: unsigned = ctxt.src2.val & 31;
    ulong rbp;
    if (nesting_level)
    return X86EMUL_UNHANDLEABLE;
    rbp = reg_read(ctxt, VCPU_REGS_RBP);
    rc = emulate_push(ctxt, &rbp, stack_size(ctxt));
    if (rc != X86EMUL_CONTINUE)
    return rc;
    assign_masked(reg_rmw(ctxt, VCPU_REGS_RBP), reg_read(ctxt, VCPU_REGS_RSP),
    stack_mask(ctxt));
    assign_masked(reg_rmw(ctxt, VCPU_REGS_RSP),
    reg_read(ctxt, VCPU_REGS_RSP) - frame_size,
    stack_mask(ctxt));
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_leave(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_leave(struct x86_emulate_ctxt *ctxt)
    {
    assign_masked(reg_rmw(ctxt, VCPU_REGS_RSP), reg_read(ctxt, VCPU_REGS_RBP),
    stack_mask(ctxt));
    return emulate_pop(ctxt, reg_rmw(ctxt, VCPU_REGS_RBP), ctxt.op_bytes);
    }
#[no_mangle]
unsafe extern "C" fn em_push_sreg(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_push_sreg(struct x86_emulate_ctxt *ctxt)
    {
    let mut seg: c_int = ctxt.src2.val;
    ctxt.src.val = get_segment_selector(ctxt, seg);
    if (ctxt.op_bytes == 4) {
    rsp_increment(ctxt, -2);
    ctxt.op_bytes = 2;
    }
    return em_push(ctxt);
    }
#[no_mangle]
unsafe extern "C" fn em_pop_sreg(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_pop_sreg(struct x86_emulate_ctxt *ctxt)
    {
    let mut seg: c_int = ctxt.src2.val;
    let mut selector: c_ulong = 0;
    int rc;
    rc = emulate_pop(ctxt, &selector, 2);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    if (seg == VCPU_SREG_SS)
    ctxt.interruptibility = KVM_X86_SHADOW_INT_MOV_SS;
    if (ctxt.op_bytes > 2)
    rsp_increment(ctxt, ctxt.op_bytes - 2);
    rc = load_segment_descriptor(ctxt, (u16)selector, seg);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_pusha(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_pusha(struct x86_emulate_ctxt *ctxt)
    {
    let mut old_esp: c_ulong = reg_read(ctxt, VCPU_REGS_RSP);
    let mut rc: c_int = X86EMUL_CONTINUE;
    let mut reg: c_int = VCPU_REGS_RAX;
    while (reg <= VCPU_REGS_RDI) {
    (reg == VCPU_REGS_RSP) ?
    (ctxt.src.val = old_esp) : (ctxt.src.val = reg_read(ctxt, reg));
    rc = em_push(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    ++reg;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_pushf(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_pushf(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.src.val = (unsigned long)ctxt.eflags & ~X86_EFLAGS_VM;
    return em_push(ctxt);
    }
#[no_mangle]
unsafe extern "C" fn em_popa(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_popa(struct x86_emulate_ctxt *ctxt)
    {
    let mut rc: c_int = X86EMUL_CONTINUE;
    let mut reg: c_int = VCPU_REGS_RDI;
    let mut val: u32 = 0;
    while (reg >= VCPU_REGS_RAX) {
    if (reg == VCPU_REGS_RSP) {
    rsp_increment(ctxt, ctxt.op_bytes);
    --reg;
    }
    rc = emulate_pop(ctxt, &val, ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    break;
    insn_assign_reg(reg_rmw(ctxt, reg), val, ctxt.op_bytes);
    --reg;
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn __emulate_int_real(ctxt: *mut x86_emulate_ctxt, irq: c_int) -> c_int {
    static int __emulate_int_real(struct x86_emulate_ctxt *ctxt, int irq)
    {
    const struct x86_emulate_ops *ops = ctxt.ops;
    int rc;
    struct desc_ptr dt;
    gva_t cs_addr;
    gva_t eip_addr;
    u16 cs, eip;
// TODO: Add limit checks
    ctxt.src.val = ctxt.eflags;
    rc = em_push(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    ctxt.eflags &= ~(X86_EFLAGS_IF | X86_EFLAGS_TF | X86_EFLAGS_AC);
    ctxt.src.val = get_segment_selector(ctxt, VCPU_SREG_CS);
    rc = em_push(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    ctxt.src.val = ctxt._eip;
    rc = em_push(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    ops.get_idt(ctxt, &dt);
    eip_addr = dt.address + (irq << 2);
    cs_addr = dt.address + (irq << 2) + 2;
    rc = linear_read_system(ctxt, cs_addr, &cs, 2);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = linear_read_system(ctxt, eip_addr, &eip, 2);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = load_segment_descriptor(ctxt, cs, VCPU_SREG_CS);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    ctxt._eip = eip;
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn emulate_int_real(ctxt: *mut x86_emulate_ctxt, irq: c_int) -> c_int {
    int emulate_int_real(struct x86_emulate_ctxt *ctxt, int irq)
    {
    int rc;
    invalidate_registers(ctxt);
    rc = __emulate_int_real(ctxt, irq);
    if (rc == X86EMUL_CONTINUE)
    writeback_registers(ctxt);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn emulate_int(ctxt: *mut x86_emulate_ctxt, irq: c_int) -> c_int {
    static int emulate_int(struct x86_emulate_ctxt *ctxt, int irq)
    {
    switch(ctxt.mode) {
    case X86EMUL_MODE_REAL:
    return __emulate_int_real(ctxt, irq);
    case X86EMUL_MODE_VM86:
    case X86EMUL_MODE_PROT16:
    case X86EMUL_MODE_PROT32:
    case X86EMUL_MODE_PROT64:
    default:
// Protected mode interrupts unimplemented yet
    return X86EMUL_UNHANDLEABLE;
    }
    }
#[no_mangle]
unsafe extern "C" fn emulate_iret_real(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int emulate_iret_real(struct x86_emulate_ctxt *ctxt)
    {
    let mut rc: c_int = X86EMUL_CONTINUE;
    let mut temp_eip: c_ulong = 0;
    let mut temp_eflags: c_ulong = 0;
    let mut cs: c_ulong = 0;
    unsigned long mask = X86_EFLAGS_CF | X86_EFLAGS_PF | X86_EFLAGS_AF |
    X86_EFLAGS_ZF | X86_EFLAGS_SF | X86_EFLAGS_TF |
    X86_EFLAGS_IF | X86_EFLAGS_DF | X86_EFLAGS_OF |
    X86_EFLAGS_IOPL | X86_EFLAGS_NT | X86_EFLAGS_RF |
    X86_EFLAGS_AC | X86_EFLAGS_ID |
    X86_EFLAGS_FIXED;
    unsigned long vm86_mask = X86_EFLAGS_VM | X86_EFLAGS_VIF |
    X86_EFLAGS_VIP;
// TODO: Add stack limit check
    rc = emulate_pop(ctxt, &temp_eip, ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    if (temp_eip & ~0xffff)
    return emulate_gp(ctxt, 0);
    rc = emulate_pop(ctxt, &cs, ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = emulate_pop(ctxt, &temp_eflags, ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = load_segment_descriptor(ctxt, (u16)cs, VCPU_SREG_CS);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    ctxt._eip = temp_eip;
    if (ctxt.op_bytes == 4)
    ctxt.eflags = ((temp_eflags & mask) | (ctxt.eflags & vm86_mask));
#[no_mangle]
pub unsafe extern "C" fn if(2: ctxt->op_bytes ==) -> else {
    ctxt.eflags &= ~0xffff;
    ctxt.eflags |= temp_eflags;
    }
    ctxt.eflags &= ~EFLG_RESERVED_ZEROS_MASK; /* Clear reserved zeros */
    ctxt.eflags |= X86_EFLAGS_FIXED;
    ctxt.ops.set_nmi_mask(ctxt, false);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_iret(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_iret(struct x86_emulate_ctxt *ctxt)
    {
    switch(ctxt.mode) {
    case X86EMUL_MODE_REAL:
    return emulate_iret_real(ctxt);
    case X86EMUL_MODE_VM86:
    case X86EMUL_MODE_PROT16:
    case X86EMUL_MODE_PROT32:
    case X86EMUL_MODE_PROT64:
    default:
// iret from protected mode unimplemented yet
    return X86EMUL_UNHANDLEABLE;
    }
    }
#[no_mangle]
unsafe extern "C" fn em_jmp_far(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_jmp_far(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    unsigned short sel;
    struct desc_struct new_desc;
    let mut cpl: u8 = ctxt.ops.cpl(ctxt);
    memcpy(&sel, ctxt.src.valptr + ctxt.op_bytes, 2);
    rc = __load_segment_descriptor(ctxt, sel, VCPU_SREG_CS, cpl,
    X86_TRANSFER_CALL_JMP,
    &new_desc);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = assign_eip_far(ctxt, ctxt.src.val);
// Error handling is not implemented.
    if (rc != X86EMUL_CONTINUE)
    return X86EMUL_UNHANDLEABLE;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_jmp_abs(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_jmp_abs(struct x86_emulate_ctxt *ctxt)
    {
    return assign_eip_near(ctxt, ctxt.src.val);
    }
#[no_mangle]
unsafe extern "C" fn em_call_near_abs(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_call_near_abs(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    long int old_eip;
    old_eip = ctxt._eip;
    rc = assign_eip_near(ctxt, ctxt.src.val);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    ctxt.src.val = old_eip;
    rc = em_push(ctxt);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_cmpxchg8b(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_cmpxchg8b(struct x86_emulate_ctxt *ctxt)
    {
    let mut old: u64 = ctxt.dst.orig_val64;
    if (ctxt.dst.bytes == 16)
    return X86EMUL_UNHANDLEABLE;
    if (((u32) (old >> 0) != (u32) reg_read(ctxt, VCPU_REGS_RAX)) ||
    ((u32) (old >> 32) != (u32) reg_read(ctxt, VCPU_REGS_RDX))) {
// reg_write(ctxt, VCPU_REGS_RAX) = (u32) (old >> 0);
// reg_write(ctxt, VCPU_REGS_RDX) = (u32) (old >> 32);
    ctxt.eflags &= ~X86_EFLAGS_ZF;
    } else {
    ctxt.dst.val64 = ((u64)reg_read(ctxt, VCPU_REGS_RCX) << 32) |
    (u32) reg_read(ctxt, VCPU_REGS_RBX);
    ctxt.eflags |= X86_EFLAGS_ZF;
    }
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_ret(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_ret(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    let mut eip: c_ulong = 0;
    rc = emulate_pop(ctxt, &eip, ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return assign_eip_near(ctxt, eip);
    }
#[no_mangle]
unsafe extern "C" fn em_ret_far(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_ret_far(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    let mut eip: c_ulong = 0;
    let mut cs: c_ulong = 0;
    let mut cpl: c_int = ctxt.ops.cpl(ctxt);
    struct desc_struct new_desc;
    rc = emulate_pop(ctxt, &eip, ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = emulate_pop(ctxt, &cs, ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = __load_segment_descriptor(ctxt, (u16)cs, VCPU_SREG_CS, cpl,
    X86_TRANSFER_RET,
    &new_desc);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = assign_eip_far(ctxt, eip);
// Error handling is not implemented.
    if (rc != X86EMUL_CONTINUE)
    return X86EMUL_UNHANDLEABLE;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_ret_far_imm(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_ret_far_imm(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    rc = em_ret_far(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rsp_increment(ctxt, ctxt.src.val);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_cmpxchg(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_cmpxchg(struct x86_emulate_ctxt *ctxt)
    {
// Save real source value, then compare EAX against destination.
    ctxt.dst.orig_val = ctxt.dst.val;
    ctxt.dst.val = reg_read(ctxt, VCPU_REGS_RAX);
    ctxt.src.orig_val = ctxt.src.val;
    ctxt.src.val = ctxt.dst.orig_val;
    em_cmp(ctxt);
    if (ctxt.eflags & X86_EFLAGS_ZF) {
// Success: write back to memory; no update of EAX
    ctxt.src.type = OP_NONE;
    ctxt.dst.val = ctxt.src.orig_val;
    } else {
// Failure: write the value we saw to EAX.
    ctxt.src.type = OP_REG;
    ctxt.src.addr.reg = reg_rmw(ctxt, VCPU_REGS_RAX);
    ctxt.src.val = ctxt.dst.orig_val;
// Create write-cycle to dest by writing the same value
    ctxt.dst.val = ctxt.dst.orig_val;
    }
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_lseg(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_lseg(struct x86_emulate_ctxt *ctxt)
    {
    let mut seg: c_int = ctxt.src2.val;
    unsigned short sel;
    int rc;
    memcpy(&sel, ctxt.src.valptr + ctxt.op_bytes, 2);
    rc = load_segment_descriptor(ctxt, sel, seg);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    ctxt.dst.val = ctxt.src.val;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_rsm(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_rsm(struct x86_emulate_ctxt *ctxt)
    {
    if (!ctxt.ops.is_smm(ctxt))
    return emulate_ud(ctxt);
    if (ctxt.ops.leave_smm(ctxt))
    ctxt.ops.triple_fault(ctxt);
    return emulator_recalc_and_set_mode(ctxt);
    }
    static void
    setup_syscalls_segments(struct desc_struct *cs, struct desc_struct *ss)
    {
    cs.l = 0;		/* will be adjusted later */
    set_desc_base(cs, 0);	/* flat segment */
    cs.g = 1;		/* 4kb granularity */
    set_desc_limit(cs, 0xfffff);	/* 4GB limit */
    cs.type = 0x0b;	/* Read, Execute, Accessed */
    cs.s = 1;
    cs.dpl = 0;		/* will be adjusted later */
    cs.p = 1;
    cs.d = 1;
    cs.avl = 0;
    set_desc_base(ss, 0);	/* flat segment */
    set_desc_limit(ss, 0xfffff);	/* 4GB limit */
    ss.g = 1;		/* 4kb granularity */
    ss.s = 1;
    ss.type = 0x03;	/* Read/Write, Accessed */
    ss.d = 1;		/* 32bit stack segment */
    ss.dpl = 0;
    ss.p = 1;
    ss.l = 0;
    ss.avl = 0;
    }
#[no_mangle]
unsafe extern "C" fn em_syscall(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_syscall(struct x86_emulate_ctxt *ctxt)
    {
    const struct x86_emulate_ops *ops = ctxt.ops;
    struct desc_struct cs, ss;
    u64 msr_data;
    u16 cs_sel, ss_sel;
    let mut efer: u64 = 0;
// syscall is not available in real mode
    if (ctxt.mode == X86EMUL_MODE_REAL ||
    ctxt.mode == X86EMUL_MODE_VM86)
    return emulate_ud(ctxt);
//
// Intel compatible CPUs only support SYSCALL in 64-bit mode, whereas
// AMD allows SYSCALL in any flavor of protected mode.  Note, it's
// infeasible to emulate Intel behavior when running on AMD hardware,
// as SYSCALL won't fault in the "wrong" mode, i.e. there is no #UD
// for KVM to trap-and-emulate, unlike emulating AMD on Intel.
//
    if (ctxt.mode != X86EMUL_MODE_PROT64 &&
    ctxt.ops.guest_cpuid_is_intel_compatible(ctxt))
    return emulate_ud(ctxt);
    ops.get_msr(ctxt, MSR_EFER, &efer);
    if (!(efer & EFER_SCE))
    return emulate_ud(ctxt);
    setup_syscalls_segments(&cs, &ss);
    ops.get_msr(ctxt, MSR_STAR, &msr_data);
    msr_data >>= 32;
    cs_sel = (u16)(msr_data & 0xfffc);
    ss_sel = (u16)(msr_data + 8);
    if (efer & EFER_LMA) {
    cs.d = 0;
    cs.l = 1;
    }
    ops.set_segment(ctxt, cs_sel, &cs, 0, VCPU_SREG_CS);
    ops.set_segment(ctxt, ss_sel, &ss, 0, VCPU_SREG_SS);
// reg_write(ctxt, VCPU_REGS_RCX) = ctxt->_eip;
    if (efer & EFER_LMA) {

// reg_write(ctxt, VCPU_REGS_R11) = ctxt->eflags;
    ops.get_msr(ctxt,
    ctxt.mode == X86EMUL_MODE_PROT64 ?
    MSR_LSTAR : MSR_CSTAR, &msr_data);
    ctxt._eip = msr_data;
    ops.get_msr(ctxt, MSR_SYSCALL_MASK, &msr_data);
    ctxt.eflags &= ~msr_data;
    ctxt.eflags |= X86_EFLAGS_FIXED;

    } else {
// legacy mode
    ops.get_msr(ctxt, MSR_STAR, &msr_data);
    ctxt._eip = (u32)msr_data;
    ctxt.eflags &= ~(X86_EFLAGS_VM | X86_EFLAGS_IF);
    }
    ctxt.tf = (ctxt.eflags & X86_EFLAGS_TF) != 0;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_sysenter(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_sysenter(struct x86_emulate_ctxt *ctxt)
    {
    const struct x86_emulate_ops *ops = ctxt.ops;
    struct desc_struct cs, ss;
    u64 msr_data;
    u16 cs_sel, ss_sel;
    let mut efer: u64 = 0;
    ops.get_msr(ctxt, MSR_EFER, &efer);
// inject #GP if in real mode
    if (ctxt.mode == X86EMUL_MODE_REAL)
    return emulate_gp(ctxt, 0);
//
// Intel's architecture allows SYSENTER in compatibility mode, but AMD
// does not.  Note, AMD does allow SYSENTER in legacy protected mode.
//
    if ((ctxt.mode != X86EMUL_MODE_PROT64) && (efer & EFER_LMA) &&
    !ctxt.ops.guest_cpuid_is_intel_compatible(ctxt))
    return emulate_ud(ctxt);
// sysenter/sysexit have not been tested in 64bit mode.
    if (ctxt.mode == X86EMUL_MODE_PROT64)
    return X86EMUL_UNHANDLEABLE;
    ops.get_msr(ctxt, MSR_IA32_SYSENTER_CS, &msr_data);
    if ((msr_data & 0xfffc) == 0x0)
    return emulate_gp(ctxt, 0);
    setup_syscalls_segments(&cs, &ss);
    ctxt.eflags &= ~(X86_EFLAGS_VM | X86_EFLAGS_IF);
    cs_sel = (u16)msr_data & ~SEGMENT_RPL_MASK;
    ss_sel = cs_sel + 8;
    if (efer & EFER_LMA) {
    cs.d = 0;
    cs.l = 1;
    }
    ops.set_segment(ctxt, cs_sel, &cs, 0, VCPU_SREG_CS);
    ops.set_segment(ctxt, ss_sel, &ss, 0, VCPU_SREG_SS);
    ops.get_msr(ctxt, MSR_IA32_SYSENTER_EIP, &msr_data);
    ctxt._eip = (efer & EFER_LMA) ? msr_data : (u32)msr_data;
    ops.get_msr(ctxt, MSR_IA32_SYSENTER_ESP, &msr_data);
// reg_write(ctxt, VCPU_REGS_RSP) = (efer & EFER_LMA) ? msr_data :
    (u32)msr_data;
    if (efer & EFER_LMA)
    ctxt.mode = X86EMUL_MODE_PROT64;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_sysexit(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_sysexit(struct x86_emulate_ctxt *ctxt)
    {
    const struct x86_emulate_ops *ops = ctxt.ops;
    struct desc_struct cs, ss;
    u64 msr_data, rcx, rdx;
    int usermode;
    let mut cs_sel: u16 = 0, ss_sel = 0;
// inject #GP if in real mode or Virtual 8086 mode
    if (ctxt.mode == X86EMUL_MODE_REAL ||
    ctxt.mode == X86EMUL_MODE_VM86)
    return emulate_gp(ctxt, 0);
    setup_syscalls_segments(&cs, &ss);
    if (ctxt.rex_bits & REX_W)
    usermode = X86EMUL_MODE_PROT64;
    else
    usermode = X86EMUL_MODE_PROT32;
    rcx = reg_read(ctxt, VCPU_REGS_RCX);
    rdx = reg_read(ctxt, VCPU_REGS_RDX);
    cs.dpl = 3;
    ss.dpl = 3;
    ops.get_msr(ctxt, MSR_IA32_SYSENTER_CS, &msr_data);
    switch (usermode) {
    case X86EMUL_MODE_PROT32:
    cs_sel = (u16)(msr_data + 16);
    if ((msr_data & 0xfffc) == 0x0)
    return emulate_gp(ctxt, 0);
    ss_sel = (u16)(msr_data + 24);
    rcx = (u32)rcx;
    rdx = (u32)rdx;
    break;
    case X86EMUL_MODE_PROT64:
    cs_sel = (u16)(msr_data + 32);
    if (msr_data == 0x0)
    return emulate_gp(ctxt, 0);
    ss_sel = cs_sel + 8;
    cs.d = 0;
    cs.l = 1;
    if (emul_is_noncanonical_address(rcx, ctxt, 0) ||
    emul_is_noncanonical_address(rdx, ctxt, 0))
    return emulate_gp(ctxt, 0);
    break;
    }
    cs_sel |= SEGMENT_RPL_MASK;
    ss_sel |= SEGMENT_RPL_MASK;
    ops.set_segment(ctxt, cs_sel, &cs, 0, VCPU_SREG_CS);
    ops.set_segment(ctxt, ss_sel, &ss, 0, VCPU_SREG_SS);
    ctxt._eip = rdx;
    ctxt.mode = usermode;
// reg_write(ctxt, VCPU_REGS_RSP) = rcx;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn emulator_bad_iopl(ctxt: *mut x86_emulate_ctxt) -> bool {
    static bool emulator_bad_iopl(struct x86_emulate_ctxt *ctxt)
    {
    int iopl;
    if (ctxt.mode == X86EMUL_MODE_REAL)
    return false;
    if (ctxt.mode == X86EMUL_MODE_VM86)
    return true;
    iopl = (ctxt.eflags & X86_EFLAGS_IOPL) >> X86_EFLAGS_IOPL_BIT;
    return ctxt.ops.cpl(ctxt) > iopl;
    }

    static bool emulator_io_port_access_allowed(struct x86_emulate_ctxt *ctxt,
    u16 port, u16 len)
    {
    const struct x86_emulate_ops *ops = ctxt.ops;
    struct desc_struct tr_seg;
    u32 base3;
    int r;
    u16 tr, io_bitmap_ptr, perm, bit_idx = port & 0x7;
    let mut mask: unsigned = (1 << len) - 1;
    unsigned long base;
//
// VMware allows access to these ports even if denied
// by TSS I/O permission bitmap. Mimic behavior.
//
    if (enable_vmware_backdoor &&
    ((port == VMWARE_PORT_VMPORT) || (port == VMWARE_PORT_VMRPC)))
    return true;
    ops.get_segment(ctxt, &tr, &tr_seg, &base3, VCPU_SREG_TR);
    if (!tr_seg.p)
    return false;
    if (desc_limit_scaled(&tr_seg) < 103)
    return false;
    base = get_desc_base(&tr_seg);

    base |= ((u64)base3) << 32;

    r = ops.read_std(ctxt, base + 102, &io_bitmap_ptr, 2, core::ptr::null_mut(), true);
    if (r != X86EMUL_CONTINUE)
    return false;
    if (io_bitmap_ptr + port/8 > desc_limit_scaled(&tr_seg))
    return false;
    r = ops.read_std(ctxt, base + io_bitmap_ptr + port/8, &perm, 2, core::ptr::null_mut(), true);
    if (r != X86EMUL_CONTINUE)
    return false;
    if ((perm >> bit_idx) & mask)
    return false;
    return true;
    }
    static bool emulator_io_permitted(struct x86_emulate_ctxt *ctxt,
    u16 port, u16 len)
    {
    if (ctxt.perm_ok)
    return true;
    if (emulator_bad_iopl(ctxt))
    if (!emulator_io_port_access_allowed(ctxt, port, len))
    return false;
    ctxt.perm_ok = true;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn string_registers_quirk(ctxt: *mut x86_emulate_ctxt) {
    static void string_registers_quirk(struct x86_emulate_ctxt *ctxt)
    {
//
// Intel CPUs mask the counter and pointers in quite strange
// manner when ECX is zero due to REP-string optimizations.
//

    u32 eax, ebx, ecx, edx;
    if (ctxt.ad_bytes != 4)
    return;
    eax = ecx = 0;
    ctxt.ops.get_cpuid(ctxt, &eax, &ebx, &ecx, &edx, true);
    if (!is_guest_vendor_intel(ebx, ecx, edx))
    return;
// reg_write(ctxt, VCPU_REGS_RCX) = 0;
    switch (ctxt.b) {
    case 0xa4:	/* movsb */
    case 0xa5:	/* movsd/w */
// reg_rmw(ctxt, VCPU_REGS_RSI) &= (u32)-1;
    fallthrough;
    case 0xaa:	/* stosb */
    case 0xab:	/* stosd/w */
// reg_rmw(ctxt, VCPU_REGS_RDI) &= (u32)-1;
    }

    }
    static void save_state_to_tss16(struct x86_emulate_ctxt *ctxt,
    struct tss_segment_16 *tss)
    {
    tss.ip = ctxt._eip;
    tss.flag = ctxt.eflags;
    tss.ax = reg_read(ctxt, VCPU_REGS_RAX);
    tss.cx = reg_read(ctxt, VCPU_REGS_RCX);
    tss.dx = reg_read(ctxt, VCPU_REGS_RDX);
    tss.bx = reg_read(ctxt, VCPU_REGS_RBX);
    tss.sp = reg_read(ctxt, VCPU_REGS_RSP);
    tss.bp = reg_read(ctxt, VCPU_REGS_RBP);
    tss.si = reg_read(ctxt, VCPU_REGS_RSI);
    tss.di = reg_read(ctxt, VCPU_REGS_RDI);
    tss.es = get_segment_selector(ctxt, VCPU_SREG_ES);
    tss.cs = get_segment_selector(ctxt, VCPU_SREG_CS);
    tss.ss = get_segment_selector(ctxt, VCPU_SREG_SS);
    tss.ds = get_segment_selector(ctxt, VCPU_SREG_DS);
    tss.ldt = get_segment_selector(ctxt, VCPU_SREG_LDTR);
    }
    static int load_state_from_tss16(struct x86_emulate_ctxt *ctxt,
    struct tss_segment_16 *tss)
    {
    int ret;
    u8 cpl;
    ctxt._eip = tss.ip;
    ctxt.eflags = tss.flag | 2;
// reg_write(ctxt, VCPU_REGS_RAX) = tss->ax;
// reg_write(ctxt, VCPU_REGS_RCX) = tss->cx;
// reg_write(ctxt, VCPU_REGS_RDX) = tss->dx;
// reg_write(ctxt, VCPU_REGS_RBX) = tss->bx;
// reg_write(ctxt, VCPU_REGS_RSP) = tss->sp;
// reg_write(ctxt, VCPU_REGS_RBP) = tss->bp;
// reg_write(ctxt, VCPU_REGS_RSI) = tss->si;
// reg_write(ctxt, VCPU_REGS_RDI) = tss->di;
//
// SDM says that segment selectors are loaded before segment
// descriptors
//
    set_segment_selector(ctxt, tss.ldt, VCPU_SREG_LDTR);
    set_segment_selector(ctxt, tss.es, VCPU_SREG_ES);
    set_segment_selector(ctxt, tss.cs, VCPU_SREG_CS);
    set_segment_selector(ctxt, tss.ss, VCPU_SREG_SS);
    set_segment_selector(ctxt, tss.ds, VCPU_SREG_DS);
    cpl = tss.cs & 3;
//
// Now load segment descriptors. If fault happens at this stage
// it is handled in a context of new task
//
    ret = __load_segment_descriptor(ctxt, tss.ldt, VCPU_SREG_LDTR, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.es, VCPU_SREG_ES, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.cs, VCPU_SREG_CS, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.ss, VCPU_SREG_SS, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.ds, VCPU_SREG_DS, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    return X86EMUL_CONTINUE;
    }
    static int task_switch_16(struct x86_emulate_ctxt *ctxt, u16 old_tss_sel,
    ulong old_tss_base, struct desc_struct *new_desc)
    {
    struct tss_segment_16 tss_seg;
    int ret;
    let mut new_tss_base: u32 = get_desc_base(new_desc);
    ret = linear_read_system(ctxt, old_tss_base, &tss_seg, sizeof(tss_seg));
    if (ret != X86EMUL_CONTINUE)
    return ret;
    save_state_to_tss16(ctxt, &tss_seg);
    ret = linear_write_system(ctxt, old_tss_base, &tss_seg, sizeof(tss_seg));
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = linear_read_system(ctxt, new_tss_base, &tss_seg, sizeof(tss_seg));
    if (ret != X86EMUL_CONTINUE)
    return ret;
    if (old_tss_sel != 0xffff) {
    tss_seg.prev_task_link = old_tss_sel;
    ret = linear_write_system(ctxt, new_tss_base,
    &tss_seg.prev_task_link,
    sizeof(tss_seg.prev_task_link));
    if (ret != X86EMUL_CONTINUE)
    return ret;
    }
    return load_state_from_tss16(ctxt, &tss_seg);
    }
    static void save_state_to_tss32(struct x86_emulate_ctxt *ctxt,
    struct tss_segment_32 *tss)
    {
// CR3 and ldt selector are not saved intentionally
    tss.eip = ctxt._eip;
    tss.eflags = ctxt.eflags;
    tss.eax = reg_read(ctxt, VCPU_REGS_RAX);
    tss.ecx = reg_read(ctxt, VCPU_REGS_RCX);
    tss.edx = reg_read(ctxt, VCPU_REGS_RDX);
    tss.ebx = reg_read(ctxt, VCPU_REGS_RBX);
    tss.esp = reg_read(ctxt, VCPU_REGS_RSP);
    tss.ebp = reg_read(ctxt, VCPU_REGS_RBP);
    tss.esi = reg_read(ctxt, VCPU_REGS_RSI);
    tss.edi = reg_read(ctxt, VCPU_REGS_RDI);
    tss.es = get_segment_selector(ctxt, VCPU_SREG_ES);
    tss.cs = get_segment_selector(ctxt, VCPU_SREG_CS);
    tss.ss = get_segment_selector(ctxt, VCPU_SREG_SS);
    tss.ds = get_segment_selector(ctxt, VCPU_SREG_DS);
    tss.fs = get_segment_selector(ctxt, VCPU_SREG_FS);
    tss.gs = get_segment_selector(ctxt, VCPU_SREG_GS);
    }
    static int load_state_from_tss32(struct x86_emulate_ctxt *ctxt,
    struct tss_segment_32 *tss)
    {
    int ret;
    u8 cpl;
    if (ctxt.ops.set_cr(ctxt, 3, tss.cr3))
    return emulate_gp(ctxt, 0);
    ctxt._eip = tss.eip;
    ctxt.eflags = tss.eflags | 2;
// General purpose registers
// reg_write(ctxt, VCPU_REGS_RAX) = tss->eax;
// reg_write(ctxt, VCPU_REGS_RCX) = tss->ecx;
// reg_write(ctxt, VCPU_REGS_RDX) = tss->edx;
// reg_write(ctxt, VCPU_REGS_RBX) = tss->ebx;
// reg_write(ctxt, VCPU_REGS_RSP) = tss->esp;
// reg_write(ctxt, VCPU_REGS_RBP) = tss->ebp;
// reg_write(ctxt, VCPU_REGS_RSI) = tss->esi;
// reg_write(ctxt, VCPU_REGS_RDI) = tss->edi;
//
// SDM says that segment selectors are loaded before segment
// descriptors.  This is important because CPL checks will
// use CS.RPL.
//
    set_segment_selector(ctxt, tss.ldt_selector, VCPU_SREG_LDTR);
    set_segment_selector(ctxt, tss.es, VCPU_SREG_ES);
    set_segment_selector(ctxt, tss.cs, VCPU_SREG_CS);
    set_segment_selector(ctxt, tss.ss, VCPU_SREG_SS);
    set_segment_selector(ctxt, tss.ds, VCPU_SREG_DS);
    set_segment_selector(ctxt, tss.fs, VCPU_SREG_FS);
    set_segment_selector(ctxt, tss.gs, VCPU_SREG_GS);
//
// If we're switching between Protected Mode and VM86, we need to make
// sure to update the mode before loading the segment descriptors so
// that the selectors are interpreted correctly.
//
    if (ctxt.eflags & X86_EFLAGS_VM) {
    ctxt.mode = X86EMUL_MODE_VM86;
    cpl = 3;
    } else {
    ctxt.mode = X86EMUL_MODE_PROT32;
    cpl = tss.cs & 3;
    }
//
// Now load segment descriptors. If fault happens at this stage
// it is handled in a context of new task
//
    ret = __load_segment_descriptor(ctxt, tss.ldt_selector, VCPU_SREG_LDTR,
    cpl, X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.es, VCPU_SREG_ES, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.cs, VCPU_SREG_CS, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.ss, VCPU_SREG_SS, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.ds, VCPU_SREG_DS, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.fs, VCPU_SREG_FS, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = __load_segment_descriptor(ctxt, tss.gs, VCPU_SREG_GS, cpl,
    X86_TRANSFER_TASK_SWITCH, core::ptr::null_mut());
    return ret;
    }
    static int task_switch_32(struct x86_emulate_ctxt *ctxt, u16 old_tss_sel,
    ulong old_tss_base, struct desc_struct *new_desc)
    {
    struct tss_segment_32 tss_seg;
    int ret;
    let mut new_tss_base: u32 = get_desc_base(new_desc);
    let mut eip_offset: u32 = offsetof(struct tss_segment_32, eip);
    let mut ldt_sel_offset: u32 = offsetof(struct tss_segment_32, ldt_selector);
    ret = linear_read_system(ctxt, old_tss_base, &tss_seg, sizeof(tss_seg));
    if (ret != X86EMUL_CONTINUE)
    return ret;
    save_state_to_tss32(ctxt, &tss_seg);
// Only GP registers and segment selectors are saved
    ret = linear_write_system(ctxt, old_tss_base + eip_offset, &tss_seg.eip,
    ldt_sel_offset - eip_offset);
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = linear_read_system(ctxt, new_tss_base, &tss_seg, sizeof(tss_seg));
    if (ret != X86EMUL_CONTINUE)
    return ret;
    if (old_tss_sel != 0xffff) {
    tss_seg.prev_task_link = old_tss_sel;
    ret = linear_write_system(ctxt, new_tss_base,
    &tss_seg.prev_task_link,
    sizeof(tss_seg.prev_task_link));
    if (ret != X86EMUL_CONTINUE)
    return ret;
    }
    return load_state_from_tss32(ctxt, &tss_seg);
    }
    static int emulator_do_task_switch(struct x86_emulate_ctxt *ctxt,
    u16 tss_selector, int idt_index, int reason,
    bool has_error_code, u32 error_code)
    {
    const struct x86_emulate_ops *ops = ctxt.ops;
    struct desc_struct curr_tss_desc, next_tss_desc;
    int ret;
    let mut old_tss_sel: u16 = get_segment_selector(ctxt, VCPU_SREG_TR);
    ulong old_tss_base =
    ops.get_cached_segment_base(ctxt, VCPU_SREG_TR);
    u32 desc_limit;
    ulong desc_addr, dr7;
// FIXME: old_tss_base == ~0 ?
    ret = read_segment_descriptor(ctxt, tss_selector, &next_tss_desc, &desc_addr);
    if (ret != X86EMUL_CONTINUE)
    return ret;
    ret = read_segment_descriptor(ctxt, old_tss_sel, &curr_tss_desc, &desc_addr);
    if (ret != X86EMUL_CONTINUE)
    return ret;
// FIXME: check that next_tss_desc is tss
//
// Check privileges. The three cases are task switch caused by...
//
// 1. jmp/call/int to task gate: Check against DPL of the task gate
// 2. Exception/IRQ/iret: No check is performed
// 3. jmp/call to TSS/task-gate: No check is performed since the
// hardware checks it before exiting.
//
    if (reason == TASK_SWITCH_GATE) {
    if (idt_index != -1) {
// Software interrupts
    struct desc_struct task_gate_desc;
    int dpl;
    ret = read_interrupt_descriptor(ctxt, idt_index,
    &task_gate_desc);
    if (ret != X86EMUL_CONTINUE)
    return ret;
    dpl = task_gate_desc.dpl;
    if ((tss_selector & 3) > dpl || ops.cpl(ctxt) > dpl)
    return emulate_gp(ctxt, (idt_index << 3) | 0x2);
    }
    }
    desc_limit = desc_limit_scaled(&next_tss_desc);
    if (!next_tss_desc.p ||
    ((desc_limit < 0x67 && (next_tss_desc.type & 8)) ||
    desc_limit < 0x2b)) {
    return emulate_ts(ctxt, tss_selector & 0xfffc);
    }
    if (reason == TASK_SWITCH_IRET || reason == TASK_SWITCH_JMP) {
    curr_tss_desc.type &= ~(1 << 1); /* clear busy flag */
    write_segment_descriptor(ctxt, old_tss_sel, &curr_tss_desc);
    }
    if (reason == TASK_SWITCH_IRET)
    ctxt.eflags = ctxt.eflags & ~X86_EFLAGS_NT;
// set back link to prev task only if NT bit is set in eflags
    note that old_tss_sel is not used after this point */
    if (reason != TASK_SWITCH_CALL && reason != TASK_SWITCH_GATE)
    old_tss_sel = 0xffff;
    if (next_tss_desc.type & 8)
    ret = task_switch_32(ctxt, old_tss_sel, old_tss_base, &next_tss_desc);
    else
    ret = task_switch_16(ctxt, old_tss_sel,
    old_tss_base, &next_tss_desc);
    if (ret != X86EMUL_CONTINUE)
    return ret;
    if (reason == TASK_SWITCH_CALL || reason == TASK_SWITCH_GATE)
    ctxt.eflags = ctxt.eflags | X86_EFLAGS_NT;
    if (reason != TASK_SWITCH_IRET) {
    next_tss_desc.type |= (1 << 1); /* set busy flag */
    write_segment_descriptor(ctxt, tss_selector, &next_tss_desc);
    }
    ops.set_cr(ctxt, 0,  ops.get_cr(ctxt, 0) | X86_CR0_TS);
    ops.set_segment(ctxt, tss_selector, &next_tss_desc, 0, VCPU_SREG_TR);
    if (has_error_code) {
    ctxt.op_bytes = ctxt.ad_bytes = (next_tss_desc.type & 8) ? 4 : 2;
    ctxt.lock_prefix = 0;
    ctxt.src.val = (unsigned long) error_code;
    ret = em_push(ctxt);
    }
    dr7 = ops.get_dr(ctxt, 7);
    ops.set_dr(ctxt, 7, dr7 & ~(DR_LOCAL_ENABLE_MASK | DR_LOCAL_SLOWDOWN));
    return ret;
    }
    int emulator_task_switch(struct x86_emulate_ctxt *ctxt,
    u16 tss_selector, int idt_index, int reason,
    bool has_error_code, u32 error_code)
    {
    int rc;
    invalidate_registers(ctxt);
    ctxt._eip = ctxt.eip;
    ctxt.dst.type = OP_NONE;
    rc = emulator_do_task_switch(ctxt, tss_selector, idt_index, reason,
    has_error_code, error_code);
    if (rc == X86EMUL_CONTINUE) {
    ctxt.eip = ctxt._eip;
    writeback_registers(ctxt);
    }
    return (rc == X86EMUL_UNHANDLEABLE) ? EMULATION_FAILED : EMULATION_OK;
    }
    static void string_addr_inc(struct x86_emulate_ctxt *ctxt, int reg,
    struct operand *op)
    {
    let mut df: c_int = (ctxt.eflags & X86_EFLAGS_DF) ? -op.count : op.count;
    register_address_increment(ctxt, reg, df * op.bytes);
    op.addr.mem.ea = register_address(ctxt, reg);
    }
#[no_mangle]
unsafe extern "C" fn em_das(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_das(struct x86_emulate_ctxt *ctxt)
    {
    u8 al, old_al;
    bool af, cf, old_cf;
    cf = ctxt.eflags & X86_EFLAGS_CF;
    al = ctxt.dst.val;
    old_al = al;
    old_cf = cf;
    cf = false;
    af = ctxt.eflags & X86_EFLAGS_AF;
    if ((al & 0x0f) > 9 || af) {
    al -= 6;
    cf = old_cf | (al >= 250);
    af = true;
    } else {
    af = false;
    }
    if (old_al > 0x99 || old_cf) {
    al -= 0x60;
    cf = true;
    }
    ctxt.dst.val = al;
// Set PF, ZF, SF
    ctxt.src.type = OP_IMM;
    ctxt.src.val = 0;
    ctxt.src.bytes = 1;
    em_or(ctxt);
    ctxt.eflags &= ~(X86_EFLAGS_AF | X86_EFLAGS_CF);
    if (cf)
    ctxt.eflags |= X86_EFLAGS_CF;
    if (af)
    ctxt.eflags |= X86_EFLAGS_AF;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_aam(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_aam(struct x86_emulate_ctxt *ctxt)
    {
    u8 al, ah;
    if (ctxt.src.val == 0)
    return emulate_de(ctxt);
    al = ctxt.dst.val & 0xff;
    ah = al / ctxt.src.val;
    al %= ctxt.src.val;
    ctxt.dst.val = (ctxt.dst.val & 0xffff0000) | al | (ah << 8);
// Set PF, ZF, SF
    ctxt.src.type = OP_IMM;
    ctxt.src.val = 0;
    ctxt.src.bytes = 1;
    em_or(ctxt);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_aad(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_aad(struct x86_emulate_ctxt *ctxt)
    {
    let mut al: u8 = ctxt.dst.val & 0xff;
    let mut ah: u8 = (ctxt.dst.val >> 8) & 0xff;
    al = (al + (ah * ctxt.src.val)) & 0xff;
    ctxt.dst.val = (ctxt.dst.val & 0xffff0000) | al;
// Set PF, ZF, SF
    ctxt.src.type = OP_IMM;
    ctxt.src.val = 0;
    ctxt.src.bytes = 1;
    em_or(ctxt);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_call(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_call(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    let mut rel: c_long = ctxt.src.val;
    ctxt.src.val = (unsigned long)ctxt._eip;
    rc = jmp_rel(ctxt, rel);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return em_push(ctxt);
    }
#[no_mangle]
unsafe extern "C" fn em_call_far(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_call_far(struct x86_emulate_ctxt *ctxt)
    {
    u16 sel, old_cs;
    ulong old_eip;
    int rc;
    struct desc_struct old_desc, new_desc;
    const struct x86_emulate_ops *ops = ctxt.ops;
    let mut cpl: c_int = ctxt.ops.cpl(ctxt);
    let mut prev_mode: enum x86emul_mode = ctxt.mode;
    old_eip = ctxt._eip;
    ops.get_segment(ctxt, &old_cs, &old_desc, core::ptr::null_mut(), VCPU_SREG_CS);
    memcpy(&sel, ctxt.src.valptr + ctxt.op_bytes, 2);
    rc = __load_segment_descriptor(ctxt, sel, VCPU_SREG_CS, cpl,
    X86_TRANSFER_CALL_JMP, &new_desc);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = assign_eip_far(ctxt, ctxt.src.val);
    if (rc != X86EMUL_CONTINUE)
    goto fail;
    ctxt.src.val = old_cs;
    rc = em_push(ctxt);
    if (rc != X86EMUL_CONTINUE)
    goto fail;
    ctxt.src.val = old_eip;
    rc = em_push(ctxt);
// If we failed, we tainted the memory, but the very least we should
    restore cs */
    if (rc != X86EMUL_CONTINUE) {
    pr_warn_once("faulting far call emulation tainted memory\n");
    goto fail;
    }
    return rc;
    fail:
    ops.set_segment(ctxt, old_cs, &old_desc, 0, VCPU_SREG_CS);
    ctxt.mode = prev_mode;
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_ret_near_imm(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_ret_near_imm(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    let mut eip: c_ulong = 0;
    rc = emulate_pop(ctxt, &eip, ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rc = assign_eip_near(ctxt, eip);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    rsp_increment(ctxt, ctxt.src.val);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_xchg(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_xchg(struct x86_emulate_ctxt *ctxt)
    {
// Write back the register source.
    ctxt.src.val = ctxt.dst.val;
    write_register_operand(&ctxt.src);
// Write back the memory destination with implicit LOCK prefix.
    ctxt.dst.val = ctxt.src.orig_val;
    ctxt.lock_prefix = 1;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_imul_3op(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_imul_3op(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.dst.val = ctxt.src2.val;
    return em_imul(ctxt);
    }
#[no_mangle]
unsafe extern "C" fn em_cwd(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_cwd(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.dst.type = OP_REG;
    ctxt.dst.bytes = ctxt.src.bytes;
    ctxt.dst.addr.reg = reg_rmw(ctxt, VCPU_REGS_RDX);
    ctxt.dst.val = ~((ctxt.src.val >> (ctxt.src.bytes * 8 - 1)) - 1);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_rdpid(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_rdpid(struct x86_emulate_ctxt *ctxt)
    {
    let mut tsc_aux: u64 = 0;
    if (!ctxt.ops.guest_has_rdpid(ctxt))
    return emulate_ud(ctxt);
    ctxt.ops.get_msr(ctxt, MSR_TSC_AUX, &tsc_aux);
    ctxt.dst.val = tsc_aux;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_rdtsc(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_rdtsc(struct x86_emulate_ctxt *ctxt)
    {
    let mut tsc: u64 = 0;
    ctxt.ops.get_msr(ctxt, MSR_IA32_TSC, &tsc);
// reg_write(ctxt, VCPU_REGS_RAX) = (u32)tsc;
// reg_write(ctxt, VCPU_REGS_RDX) = tsc >> 32;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_rdpmc(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_rdpmc(struct x86_emulate_ctxt *ctxt)
    {
    u64 pmc;
    if (ctxt.ops.read_pmc(ctxt, reg_read(ctxt, VCPU_REGS_RCX), &pmc))
    return emulate_gp(ctxt, 0);
// reg_write(ctxt, VCPU_REGS_RAX) = (u32)pmc;
// reg_write(ctxt, VCPU_REGS_RDX) = pmc >> 32;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_mov(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_mov(struct x86_emulate_ctxt *ctxt)
    {
    memcpy(ctxt.dst.valptr, ctxt.src.valptr, sizeof(ctxt.src.valptr));
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_movbe(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_movbe(struct x86_emulate_ctxt *ctxt)
    {
    u16 tmp;
    if (!ctxt.ops.guest_has_movbe(ctxt))
    return emulate_ud(ctxt);
    switch (ctxt.op_bytes) {
    case 2:
//
// From MOVBE definition: "...When the operand size is 16 bits,
// the upper word of the destination register remains unchanged
// ..."
//
// Both casting ->valptr and ->val to u16 breaks strict aliasing
// rules so we have to do the operation almost per hand.
//
    tmp = (u16)ctxt.src.val;
    ctxt.dst.val &= ~0xffffUL;
    ctxt.dst.val |= (unsigned long)swab16(tmp);
    break;
    case 4:
    ctxt.dst.val = swab32((u32)ctxt.src.val);
    break;
    case 8:
    ctxt.dst.val = swab64(ctxt.src.val);
    break;
    default:
    BUG();
    }
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_cr_write(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_cr_write(struct x86_emulate_ctxt *ctxt)
    {
    let mut cr_num: c_int = ctxt.modrm_reg;
    int r;
    if (ctxt.ops.set_cr(ctxt, cr_num, ctxt.src.val))
    return emulate_gp(ctxt, 0);
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    if (cr_num == 0) {
//
// CR0 write might have updated CR0.PE and/or CR0.PG
// which can affect the cpu's execution mode.
//
    r = emulator_recalc_and_set_mode(ctxt);
    if (r != X86EMUL_CONTINUE)
    return r;
    }
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_dr_write(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_dr_write(struct x86_emulate_ctxt *ctxt)
    {
    unsigned long val;
    if (ctxt.mode == X86EMUL_MODE_PROT64)
    val = ctxt.src.val & ~0ULL;
    else
    val = ctxt.src.val & ~0U;
//
// A #GP due to an illegal value should be impossible at this point, as
// such #GPs have priority over MOV DR intercepts on SVM, i.e. KVM must
// manually check the value *before* emulating the write.
//
    if (WARN_ON_ONCE(ctxt.ops.set_dr(ctxt, ctxt.modrm_reg, val)))
    return emulate_gp(ctxt, 0);
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_wrmsr(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_wrmsr(struct x86_emulate_ctxt *ctxt)
    {
    let mut msr_index: u64 = reg_read(ctxt, VCPU_REGS_RCX);
    u64 msr_data;
    int r;
    msr_data = (u32)reg_read(ctxt, VCPU_REGS_RAX)
    | ((u64)reg_read(ctxt, VCPU_REGS_RDX) << 32);
    r = ctxt.ops.set_msr_with_filter(ctxt, msr_index, msr_data);
    if (r == X86EMUL_PROPAGATE_FAULT)
    return emulate_gp(ctxt, 0);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn em_rdmsr(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_rdmsr(struct x86_emulate_ctxt *ctxt)
    {
    let mut msr_index: u64 = reg_read(ctxt, VCPU_REGS_RCX);
    u64 msr_data;
    int r;
    r = ctxt.ops.get_msr_with_filter(ctxt, msr_index, &msr_data);
    if (r == X86EMUL_PROPAGATE_FAULT)
    return emulate_gp(ctxt, 0);
    if (r == X86EMUL_CONTINUE) {
// reg_write(ctxt, VCPU_REGS_RAX) = (u32)msr_data;
// reg_write(ctxt, VCPU_REGS_RDX) = msr_data >> 32;
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn em_store_sreg(ctxt: *mut x86_emulate_ctxt, segment: c_int) -> c_int {
    static int em_store_sreg(struct x86_emulate_ctxt *ctxt, int segment)
    {
    if (segment > VCPU_SREG_GS &&
    (ctxt.ops.get_cr(ctxt, 4) & X86_CR4_UMIP) &&
    ctxt.ops.cpl(ctxt) > 0)
    return emulate_gp(ctxt, 0);
    ctxt.dst.val = get_segment_selector(ctxt, segment);
    if (ctxt.dst.bytes == 4 && ctxt.dst.type == OP_MEM)
    ctxt.dst.bytes = 2;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_mov_rm_sreg(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_mov_rm_sreg(struct x86_emulate_ctxt *ctxt)
    {
    if (ctxt.modrm_reg > VCPU_SREG_GS)
    return emulate_ud(ctxt);
    return em_store_sreg(ctxt, ctxt.modrm_reg);
    }
#[no_mangle]
unsafe extern "C" fn em_mov_sreg_rm(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_mov_sreg_rm(struct x86_emulate_ctxt *ctxt)
    {
    let mut sel: u16 = ctxt.src.val;
    if (ctxt.modrm_reg == VCPU_SREG_CS || ctxt.modrm_reg > VCPU_SREG_GS)
    return emulate_ud(ctxt);
    if (ctxt.modrm_reg == VCPU_SREG_SS)
    ctxt.interruptibility = KVM_X86_SHADOW_INT_MOV_SS;
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return load_segment_descriptor(ctxt, sel, ctxt.modrm_reg);
    }
#[no_mangle]
unsafe extern "C" fn em_sldt(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_sldt(struct x86_emulate_ctxt *ctxt)
    {
    return em_store_sreg(ctxt, VCPU_SREG_LDTR);
    }
#[no_mangle]
unsafe extern "C" fn em_lldt(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_lldt(struct x86_emulate_ctxt *ctxt)
    {
    let mut sel: u16 = ctxt.src.val;
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return load_segment_descriptor(ctxt, sel, VCPU_SREG_LDTR);
    }
#[no_mangle]
unsafe extern "C" fn em_str(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_str(struct x86_emulate_ctxt *ctxt)
    {
    return em_store_sreg(ctxt, VCPU_SREG_TR);
    }
#[no_mangle]
unsafe extern "C" fn em_ltr(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_ltr(struct x86_emulate_ctxt *ctxt)
    {
    let mut sel: u16 = ctxt.src.val;
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return load_segment_descriptor(ctxt, sel, VCPU_SREG_TR);
    }
#[no_mangle]
unsafe extern "C" fn em_invlpg(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_invlpg(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    ulong linear;
    unsigned int max_size;
    rc = __linearize(ctxt, ctxt.src.addr.mem, &max_size, 1, ctxt.mode,
    &linear, X86EMUL_F_INVLPG);
    if (rc == X86EMUL_CONTINUE)
    ctxt.ops.invlpg(ctxt, linear);
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_clts(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_clts(struct x86_emulate_ctxt *ctxt)
    {
    ulong cr0;
    cr0 = ctxt.ops.get_cr(ctxt, 0);
    cr0 &= ~X86_CR0_TS;
    ctxt.ops.set_cr(ctxt, 0, cr0);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_hypercall(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_hypercall(struct x86_emulate_ctxt *ctxt)
    {
    let mut rc: c_int = ctxt.ops.fix_hypercall(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
// Let the processor re-execute the fixed hypercall
    ctxt._eip = ctxt.eip;
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return X86EMUL_CONTINUE;
    }
    static int emulate_store_desc_ptr(struct x86_emulate_ctxt *ctxt,
    void (*get)(struct x86_emulate_ctxt *ctxt,
    struct desc_ptr *ptr))
    {
    struct desc_ptr desc_ptr;
    if ((ctxt.ops.get_cr(ctxt, 4) & X86_CR4_UMIP) &&
    ctxt.ops.cpl(ctxt) > 0)
    return emulate_gp(ctxt, 0);
    if (ctxt.mode == X86EMUL_MODE_PROT64)
    ctxt.op_bytes = 8;
    get(ctxt, &desc_ptr);
    if (ctxt.op_bytes == 2) {
    ctxt.op_bytes = 4;
    desc_ptr.address &= 0x00ffffff;
    }
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return segmented_write_std(ctxt, ctxt.dst.addr.mem,
    &desc_ptr, 2 + ctxt.op_bytes);
    }
#[no_mangle]
unsafe extern "C" fn em_sgdt(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_sgdt(struct x86_emulate_ctxt *ctxt)
    {
    return emulate_store_desc_ptr(ctxt, ctxt.ops.get_gdt);
    }
#[no_mangle]
unsafe extern "C" fn em_sidt(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_sidt(struct x86_emulate_ctxt *ctxt)
    {
    return emulate_store_desc_ptr(ctxt, ctxt.ops.get_idt);
    }
#[no_mangle]
unsafe extern "C" fn em_lgdt_lidt(ctxt: *mut x86_emulate_ctxt, lgdt: bool) -> c_int {
    static int em_lgdt_lidt(struct x86_emulate_ctxt *ctxt, bool lgdt)
    {
    struct desc_ptr desc_ptr;
    int rc;
    if (ctxt.mode == X86EMUL_MODE_PROT64)
    ctxt.op_bytes = 8;
    rc = read_descriptor(ctxt, ctxt.src.addr.mem,
    &desc_ptr.size, &desc_ptr.address,
    ctxt.op_bytes);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    if (ctxt.mode == X86EMUL_MODE_PROT64 &&
    emul_is_noncanonical_address(desc_ptr.address, ctxt,
    X86EMUL_F_DT_LOAD))
    return emulate_gp(ctxt, 0);
    if (lgdt)
    ctxt.ops.set_gdt(ctxt, &desc_ptr);
    else
    ctxt.ops.set_idt(ctxt, &desc_ptr);
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_lgdt(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_lgdt(struct x86_emulate_ctxt *ctxt)
    {
    return em_lgdt_lidt(ctxt, true);
    }
#[no_mangle]
unsafe extern "C" fn em_lidt(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_lidt(struct x86_emulate_ctxt *ctxt)
    {
    return em_lgdt_lidt(ctxt, false);
    }
#[no_mangle]
unsafe extern "C" fn em_smsw(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_smsw(struct x86_emulate_ctxt *ctxt)
    {
    if ((ctxt.ops.get_cr(ctxt, 4) & X86_CR4_UMIP) &&
    ctxt.ops.cpl(ctxt) > 0)
    return emulate_gp(ctxt, 0);
    if (ctxt.dst.type == OP_MEM)
    ctxt.dst.bytes = 2;
    ctxt.dst.val = ctxt.ops.get_cr(ctxt, 0);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_lmsw(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_lmsw(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.ops.set_cr(ctxt, 0, (ctxt.ops.get_cr(ctxt, 0) & ~0x0eul)
    | (ctxt.src.val & 0x0f));
    ctxt.dst.type = OP_NONE;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_loop(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_loop(struct x86_emulate_ctxt *ctxt)
    {
    let mut rc: c_int = X86EMUL_CONTINUE;
    register_address_increment(ctxt, VCPU_REGS_RCX, -1);
    if ((address_mask(ctxt, reg_read(ctxt, VCPU_REGS_RCX)) != 0) &&
    (ctxt.b == 0xe2 || test_cc(ctxt.b ^ 0x5, ctxt.eflags)))
    rc = jmp_rel(ctxt, ctxt.src.val);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_jcxz(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_jcxz(struct x86_emulate_ctxt *ctxt)
    {
    let mut rc: c_int = X86EMUL_CONTINUE;
    if (address_mask(ctxt, reg_read(ctxt, VCPU_REGS_RCX)) == 0)
    rc = jmp_rel(ctxt, ctxt.src.val);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_in(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_in(struct x86_emulate_ctxt *ctxt)
    {
    if (!pio_in_emulated(ctxt, ctxt.dst.bytes, ctxt.src.val,
    &ctxt.dst.val))
    return X86EMUL_IO_NEEDED;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_out(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_out(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.ops.pio_out_emulated(ctxt, ctxt.src.bytes, ctxt.dst.val,
    &ctxt.src.val, 1);
// Disable writeback.
    ctxt.dst.type = OP_NONE;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_cli(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_cli(struct x86_emulate_ctxt *ctxt)
    {
    if (emulator_bad_iopl(ctxt))
    return emulate_gp(ctxt, 0);
    ctxt.eflags &= ~X86_EFLAGS_IF;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_sti(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_sti(struct x86_emulate_ctxt *ctxt)
    {
    if (emulator_bad_iopl(ctxt))
    return emulate_gp(ctxt, 0);
    ctxt.interruptibility = KVM_X86_SHADOW_INT_STI;
    ctxt.eflags |= X86_EFLAGS_IF;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_cpuid(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_cpuid(struct x86_emulate_ctxt *ctxt)
    {
    u32 eax, ebx, ecx, edx;
    if (!ctxt.ops.is_cpuid_allowed(ctxt))
    return emulate_gp(ctxt, 0);
    eax = reg_read(ctxt, VCPU_REGS_RAX);
    ecx = reg_read(ctxt, VCPU_REGS_RCX);
    ctxt.ops.get_cpuid(ctxt, &eax, &ebx, &ecx, &edx, false);
// reg_write(ctxt, VCPU_REGS_RAX) = eax;
// reg_write(ctxt, VCPU_REGS_RBX) = ebx;
// reg_write(ctxt, VCPU_REGS_RCX) = ecx;
// reg_write(ctxt, VCPU_REGS_RDX) = edx;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_sahf(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_sahf(struct x86_emulate_ctxt *ctxt)
    {
    u32 flags;
    flags = X86_EFLAGS_CF | X86_EFLAGS_PF | X86_EFLAGS_AF | X86_EFLAGS_ZF |
    X86_EFLAGS_SF;
    flags &= *reg_rmw(ctxt, VCPU_REGS_RAX) >> 8;
    ctxt.eflags &= ~0xffUL;
    ctxt.eflags |= flags | X86_EFLAGS_FIXED;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_lahf(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_lahf(struct x86_emulate_ctxt *ctxt)
    {
// reg_rmw(ctxt, VCPU_REGS_RAX) &= ~0xff00UL;
// reg_rmw(ctxt, VCPU_REGS_RAX) |= (ctxt->eflags & 0xff) << 8;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_bswap(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_bswap(struct x86_emulate_ctxt *ctxt)
    {
    switch (ctxt.op_bytes) {

    case 8:
    asm("bswap %0" : "+r"(ctxt.dst.val));
    break;

    default:
    asm("bswap %0" : "+r"(*(u32 *)&ctxt.dst.val));
    break;
    }
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_clflush(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_clflush(struct x86_emulate_ctxt *ctxt)
    {
// emulating clflush regardless of cpuid
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_clflushopt(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_clflushopt(struct x86_emulate_ctxt *ctxt)
    {
// emulating clflushopt regardless of cpuid
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn em_movsxd(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_movsxd(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.dst.val = (s32) ctxt.src.val;
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn check_fxsr(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_fxsr(struct x86_emulate_ctxt *ctxt)
    {
    if (!ctxt.ops.guest_has_fxsr(ctxt))
    return emulate_ud(ctxt);
    if (ctxt.ops.get_cr(ctxt, 0) & (X86_CR0_TS | X86_CR0_EM))
    return emulate_nm(ctxt);
//
// Don't emulate a case that should never be hit, instead of working
// around a lack of fxsave64/fxrstor64 on old compilers.
//
    if (ctxt.mode >= X86EMUL_MODE_PROT64)
    return X86EMUL_UNHANDLEABLE;
    return X86EMUL_CONTINUE;
    }
//
// Hardware doesn't save and restore XMM 0-7 without CR4.OSFXSR, but does save
// and restore MXCSR.
//
#[no_mangle]
unsafe extern "C" fn __fxstate_size(nregs: c_int) -> usize {
    static size_t __fxstate_size(int nregs)
    {
    return offsetof(struct fxregs_state, xmm_space[0]) + nregs * 16;
    }
#[no_mangle]
pub unsafe extern "C" fn fxstate_size(ctxt: *mut x86_emulate_ctxt) -> usize {
    static inline size_t fxstate_size(struct x86_emulate_ctxt *ctxt)
    {
    bool cr4_osfxsr;
    if (ctxt.mode == X86EMUL_MODE_PROT64)
    return __fxstate_size(16);
    cr4_osfxsr = ctxt.ops.get_cr(ctxt, 4) & X86_CR4_OSFXSR;
    return __fxstate_size(cr4_osfxsr ? 8 : 0);
    }
//
// FXSAVE and FXRSTOR have 4 different formats depending on execution mode,
// 1) 16 bit mode
// 2) 32 bit mode
// - like (1), but FIP and FDP (foo) are only 16 bit.  At least Intel CPUs
// preserve whole 32 bit values, though, so (1) and (2) are the same wrt.
// save and restore
// 3) 64-bit mode with REX.W prefix
// - like (2), but XMM 8-15 are being saved and restored
// 4) 64-bit mode without REX.W prefix
// - like (3), but FIP and FDP are 64 bit
//
// Emulation uses (3) for (1) and (2) and preserves XMM 8-15 to reach the
// desired result.  (4) is not emulated.
//
// Note: Guest and host CPUID.(EAX=07H,ECX=0H):EBX[bit 13] (deprecate FPU CS
// and FPU DS) should match.
//
#[no_mangle]
unsafe extern "C" fn em_fxsave(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_fxsave(struct x86_emulate_ctxt *ctxt)
    {
    let mut fx_state: fxregs_state = {};
    int rc;
    rc = check_fxsr(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    kvm_fpu_get();
    rc = asm_safe("fxsave %[fx]", , [fx] "+m"(fx_state));
    kvm_fpu_put();
    if (rc != X86EMUL_CONTINUE)
    return rc;
    return segmented_write_std(ctxt, ctxt.memop.addr.mem, &fx_state,
    fxstate_size(ctxt));
    }
//
// FXRSTOR might restore XMM registers not provided by the guest. Fill
// in the host registers (via FXSAVE) instead, so they won't be modified.
// (preemption has to stay disabled until FXRSTOR).
//
// Use noinline to keep the stack for other functions called by callers small.
//
    static noinline int fxregs_fixup(struct fxregs_state *fx_state,
    const size_t used_size)
    {
    let mut fx_tmp: fxregs_state = {};
    int rc;
    rc = asm_safe("fxsave %[fx]", , [fx] "+m"(fx_tmp));
    memcpy((void *)fx_state + used_size, (void *)&fx_tmp + used_size,
    __fxstate_size(16) - used_size);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_fxrstor(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_fxrstor(struct x86_emulate_ctxt *ctxt)
    {
    struct fxregs_state fx_state;
    int rc;
    size_t size;
    rc = check_fxsr(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    size = fxstate_size(ctxt);
    rc = segmented_read_std(ctxt, ctxt.memop.addr.mem, &fx_state, size);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    kvm_fpu_get();
    if (size < __fxstate_size(16)) {
    rc = fxregs_fixup(&fx_state, size);
    if (rc != X86EMUL_CONTINUE)
    goto out;
    }
    if (fx_state.mxcsr >> 16) {
    rc = emulate_gp(ctxt, 0);
    goto out;
    }
    if (rc == X86EMUL_CONTINUE)
    rc = asm_safe("fxrstor %[fx]", : [fx] "m"(fx_state));
    out:
    kvm_fpu_put();
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn em_xsetbv(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int em_xsetbv(struct x86_emulate_ctxt *ctxt)
    {
    u32 eax, ecx, edx;
    if (!(ctxt.ops.get_cr(ctxt, 4) & X86_CR4_OSXSAVE))
    return emulate_ud(ctxt);
    eax = reg_read(ctxt, VCPU_REGS_RAX);
    edx = reg_read(ctxt, VCPU_REGS_RDX);
    ecx = reg_read(ctxt, VCPU_REGS_RCX);
    if (ctxt.ops.set_xcr(ctxt, ecx, ((u64)edx << 32) | eax))
    return emulate_gp(ctxt, 0);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn valid_cr(nr: c_int) -> bool {
    static bool valid_cr(int nr)
    {
    switch (nr) {
    case 0:
    case 2 ... 4:
    case 8:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn check_cr_access(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_cr_access(struct x86_emulate_ctxt *ctxt)
    {
    if (!valid_cr(ctxt.modrm_reg))
    return emulate_ud(ctxt);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn check_dr_read(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_dr_read(struct x86_emulate_ctxt *ctxt)
    {
    let mut is_intel: bool = ctxt.ops.guest_cpuid_is_intel_compatible(ctxt);
    let mut dr: c_int = ctxt.modrm_reg;
    if (dr > 7)
    return emulate_ud(ctxt);
    if ((dr == 4 || dr == 5) && (ctxt.ops.get_cr(ctxt, 4) & X86_CR4_DE))
    return emulate_ud(ctxt);
// Intel CPUs prioritize the DR7.GD=1 #DB over the CPL>0 #GP.
    if (!is_intel && ctxt.ops.cpl(ctxt))
    return emulate_gp(ctxt, 0);
    if (ctxt.ops.get_effective_dr7(ctxt) & DR7_GD)
    return emulate_db(ctxt, DR6_BD);
    if (is_intel && ctxt.ops.cpl(ctxt))
    return emulate_gp(ctxt, 0);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn check_dr_write(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_dr_write(struct x86_emulate_ctxt *ctxt)
    {
    let mut new_val: u64 = ctxt.src.val64;
    int rc;
    rc = check_dr_read(ctxt);
    if (rc != X86EMUL_CONTINUE)
    return rc;
    switch (ctxt.modrm_reg) {
    case 4:
    case 6:
    if (!kvm_dr6_valid(new_val))
    return emulate_gp(ctxt, 0);
    break;
    case 5:
    case 7:
    if (!kvm_dr7_valid(new_val))
    return emulate_gp(ctxt, 0);
    break;
    default:
    break;
    }
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn check_svme(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_svme(struct x86_emulate_ctxt *ctxt)
    {
    let mut efer: u64 = 0;
    ctxt.ops.get_msr(ctxt, MSR_EFER, &efer);
    if (!(efer & EFER_SVME))
    return emulate_ud(ctxt);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn check_svme_pa(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_svme_pa(struct x86_emulate_ctxt *ctxt)
    {
    let mut rax: u64 = reg_read(ctxt, VCPU_REGS_RAX);
    if (!ctxt.ops.page_address_valid(ctxt, rax))
    return emulate_gp(ctxt, 0);
    return check_svme(ctxt);
    }
#[no_mangle]
unsafe extern "C" fn check_rdtsc(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_rdtsc(struct x86_emulate_ctxt *ctxt)
    {
    let mut cr4: u64 = ctxt.ops.get_cr(ctxt, 4);
    if (cr4 & X86_CR4_TSD && ctxt.ops.cpl(ctxt))
    return emulate_gp(ctxt, 0);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn check_rdpmc(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_rdpmc(struct x86_emulate_ctxt *ctxt)
    {
    let mut cr4: u64 = ctxt.ops.get_cr(ctxt, 4);
    let mut rcx: u64 = reg_read(ctxt, VCPU_REGS_RCX);
//
// VMware allows access to these Pseduo-PMCs even when read via RDPMC
// in Ring3 when CR4.PCE=0.
//
    if (enable_vmware_backdoor && is_vmware_backdoor_pmc(rcx))
    return X86EMUL_CONTINUE;
//
// If CR4.PCE is set, the SDM requires CPL=0 or CR0.PE=0.  The CR0.PE
// check however is unnecessary because CPL is always 0 outside
// protected mode.
//
    if ((!(cr4 & X86_CR4_PCE) && ctxt.ops.cpl(ctxt)) ||
    ctxt.ops.check_rdpmc_early(ctxt, rcx))
    return emulate_gp(ctxt, 0);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn check_perm_in(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_perm_in(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.dst.bytes = min(ctxt.dst.bytes, 4u);
    if (!emulator_io_permitted(ctxt, ctxt.src.val, ctxt.dst.bytes))
    return emulate_gp(ctxt, 0);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn check_perm_out(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int check_perm_out(struct x86_emulate_ctxt *ctxt)
    {
    ctxt.src.bytes = min(ctxt.src.bytes, 4u);
    if (!emulator_io_permitted(ctxt, ctxt.dst.val, ctxt.src.bytes))
    return emulate_gp(ctxt, 0);
    return X86EMUL_CONTINUE;
    }

    .intercept = x86_intercept_##_i, .check_perm = (_p) }

    { .flags = (_f)|Intercept, .u.execute = (_e), .intercept = x86_intercept_##_i }

    { .flags = (_f)|Intercept|CheckPerm, .u.execute = (_e), \
    .intercept = x86_intercept_##_i, .check_perm = (_p) }

    IIP((_f) | ByteOp, _e, _i, _p), IIP(_f, _e, _i, _p)

    I2bv(((_f) | DstReg | SrcMem | ModRM) & ~Lock, _e),	\
    I2bv(((_f) & ~Lock) | DstAcc | SrcImm, _e)
    let mut ud: static struct opcode = I(SrcNone, emulate_ud);
    static const struct opcode group7_rm0[] = {
    N,
    I(SrcNone | Priv | EmulateOnUD,	em_hypercall),
    N, N, N, N, N, N,
    };
    static const struct opcode group7_rm1[] = {
    DI(SrcNone | Priv, monitor),
    DI(SrcNone | Priv, mwait),
    N, N, N, N, N, N,
    };
    static const struct opcode group7_rm2[] = {
    N,
    II(ImplicitOps | Priv,			em_xsetbv,	xsetbv),
    N, N, N, N, N, N,
    };
    static const struct opcode group7_rm3[] = {
    DIP(SrcNone | Prot | Priv,		vmrun,		check_svme_pa),
    II(SrcNone  | Prot | EmulateOnUD,	em_hypercall,	vmmcall),
    DIP(SrcNone | Prot | Priv,		vmload,		check_svme_pa),
    DIP(SrcNone | Prot | Priv,		vmsave,		check_svme_pa),
    DIP(SrcNone | Prot | Priv,		stgi,		check_svme),
    DIP(SrcNone | Prot | Priv,		clgi,		check_svme),
    DIP(SrcNone | Prot | Priv,		skinit,		check_svme),
    DIP(SrcNone | Prot | Priv,		invlpga,	check_svme),
    };
    static const struct opcode group7_rm7[] = {
    N,
    DIP(SrcNone, rdtscp, check_rdtsc),
    N, N, N, N, N, N,
    };
    static const struct opcode group1[] = {
    I(Lock, em_add),
    I(Lock | PageTable, em_or),
    I(Lock, em_adc),
    I(Lock, em_sbb),
    I(Lock | PageTable, em_and),
    I(Lock, em_sub),
    I(Lock, em_xor),
    I(NoWrite, em_cmp),
    };
    static const struct opcode group1A[] = {
    I(DstMem | SrcNone | Mov | Stack | IncSP | TwoMemOp, em_pop), N, N, N, N, N, N, N,
    };
    static const struct opcode group2[] = {
    I(DstMem | ModRM, em_rol),
    I(DstMem | ModRM, em_ror),
    I(DstMem | ModRM, em_rcl),
    I(DstMem | ModRM, em_rcr),
    I(DstMem | ModRM, em_shl),
    I(DstMem | ModRM, em_shr),
    I(DstMem | ModRM, em_shl),
    I(DstMem | ModRM, em_sar),
    };
    static const struct opcode group3[] = {
    I(DstMem | SrcImm | NoWrite, em_test),
    I(DstMem | SrcImm | NoWrite, em_test),
    I(DstMem | SrcNone | Lock, em_not),
    I(DstMem | SrcNone | Lock, em_neg),
    I(DstXacc | Src2Mem, em_mul_ex),
    I(DstXacc | Src2Mem, em_imul_ex),
    I(DstXacc | Src2Mem, em_div_ex),
    I(DstXacc | Src2Mem, em_idiv_ex),
    };
    static const struct opcode group4[] = {
    I(ByteOp | DstMem | SrcNone | Lock, em_inc),
    I(ByteOp | DstMem | SrcNone | Lock, em_dec),
    N, N, N, N, N, N,
    };
    static const struct opcode group5[] = {
    I(DstMem | SrcNone | Lock,		em_inc),
    I(DstMem | SrcNone | Lock,		em_dec),
    I(SrcMem | NearBranch | IsBranch | ShadowStack, em_call_near_abs),
    I(SrcMemFAddr | ImplicitOps | IsBranch | ShadowStack, em_call_far),
    I(SrcMem | NearBranch | IsBranch,       em_jmp_abs),
    I(SrcMemFAddr | ImplicitOps | IsBranch, em_jmp_far),
    I(SrcMem | Stack | TwoMemOp,		em_push), D(Undefined),
    };
    static const struct opcode group6[] = {
    II(Prot | DstMem,	   em_sldt, sldt),
    II(Prot | DstMem,	   em_str, str),
    II(Prot | Priv | SrcMem16, em_lldt, lldt),
    II(Prot | Priv | SrcMem16, em_ltr, ltr),
    N, N, N, N,
    };
    static const struct group_dual group7 = { {
    II(Mov | DstMem,			em_sgdt, sgdt),
    II(Mov | DstMem,			em_sidt, sidt),
    II(SrcMem | Priv,			em_lgdt, lgdt),
    II(SrcMem | Priv,			em_lidt, lidt),
    II(SrcNone | DstMem | Mov,		em_smsw, smsw), N,
    II(SrcMem16 | Mov | Priv,		em_lmsw, lmsw),
    II(SrcMem | ByteOp | Priv | NoAccess,	em_invlpg, invlpg),
    }, {
    EXT(0, group7_rm0),
    EXT(0, group7_rm1),
    EXT(0, group7_rm2),
    EXT(0, group7_rm3),
    II(SrcNone | DstMem | Mov,		em_smsw, smsw), N,
    II(SrcMem16 | Mov | Priv,		em_lmsw, lmsw),
    EXT(0, group7_rm7),
    } };
    static const struct opcode group8[] = {
    N, N, N, N,
    I(DstMem | SrcImmByte | NoWrite,		em_bt),
    I(DstMem | SrcImmByte | Lock | PageTable,	em_bts),
    I(DstMem | SrcImmByte | Lock,			em_btr),
    I(DstMem | SrcImmByte | Lock | PageTable,	em_btc),
    };
//
// The "memory" destination is actually always a register, since we come
// from the register case of group9.
//
    static const struct gprefix pfx_0f_c7_7 = {
    N, N, N, II(DstMem | ModRM | Op3264 | EmulateOnUD, em_rdpid, rdpid),
    };
    static const struct group_dual group9 = { {
    N, I(DstMem64 | Lock | PageTable, em_cmpxchg8b), N, N, N, N, N, N,
    }, {
    N, N, N, N, N, N, N,
    GP(0, &pfx_0f_c7_7),
    } };
    static const struct opcode group11[] = {
    I(DstMem | SrcImm | Mov | PageTable, em_mov),
    X7(D(Undefined)),
    };
    static const struct gprefix pfx_0f_ae_7 = {
    I(SrcMem | ByteOp, em_clflush), I(SrcMem | ByteOp, em_clflushopt), N, N,
    };
    static const struct group_dual group15 = { {
    I(ModRM | Aligned16, em_fxsave),
    I(ModRM | Aligned16, em_fxrstor),
    N, N, N, N, N, GP(0, &pfx_0f_ae_7),
    }, {
    N, N, N, N, N, N, N, N,
    } };
    static const struct gprefix pfx_0f_6f_0f_7f = {
    I(Mmx, em_mov), I(Sse | Avx | Aligned, em_mov), N, I(Sse | Avx | Unaligned, em_mov),
    };
    static const struct instr_dual instr_dual_0f_2b = {
    I(0, em_mov), N
    };
    static const struct gprefix pfx_0f_2b = {
    ID(0, &instr_dual_0f_2b), ID(0, &instr_dual_0f_2b), N, N,
    };
    static const struct gprefix pfx_0f_10_0f_11 = {
    I(Unaligned, em_mov), I(Unaligned, em_mov), N, N,
    };
    static const struct gprefix pfx_0f_28_0f_29 = {
    I(Aligned, em_mov), I(Aligned, em_mov), N, N,
    };
    static const struct gprefix pfx_0f_e7_0f_38_2a = {
    N, I(Sse | Avx, em_mov), N, N,
    };
    static const struct escape escape_d9 = { {
    N, N, N, N, N, N, N, I(DstMem16 | Mov, em_fnstcw),
    }, {
// 0xC0 - 0xC7
    N, N, N, N, N, N, N, N,
// 0xC8 - 0xCF
    N, N, N, N, N, N, N, N,
// 0xD0 - 0xC7
    N, N, N, N, N, N, N, N,
// 0xD8 - 0xDF
    N, N, N, N, N, N, N, N,
// 0xE0 - 0xE7
    N, N, N, N, N, N, N, N,
// 0xE8 - 0xEF
    N, N, N, N, N, N, N, N,
// 0xF0 - 0xF7
    N, N, N, N, N, N, N, N,
// 0xF8 - 0xFF
    N, N, N, N, N, N, N, N,
    } };
    static const struct escape escape_db = { {
    N, N, N, N, N, N, N, N,
    }, {
// 0xC0 - 0xC7
    N, N, N, N, N, N, N, N,
// 0xC8 - 0xCF
    N, N, N, N, N, N, N, N,
// 0xD0 - 0xC7
    N, N, N, N, N, N, N, N,
// 0xD8 - 0xDF
    N, N, N, N, N, N, N, N,
// 0xE0 - 0xE7
    N, N, N, I(ImplicitOps, em_fninit), N, N, N, N,
// 0xE8 - 0xEF
    N, N, N, N, N, N, N, N,
// 0xF0 - 0xF7
    N, N, N, N, N, N, N, N,
// 0xF8 - 0xFF
    N, N, N, N, N, N, N, N,
    } };
    static const struct escape escape_dd = { {
    N, N, N, N, N, N, N, I(DstMem16 | Mov, em_fnstsw),
    }, {
// 0xC0 - 0xC7
    N, N, N, N, N, N, N, N,
// 0xC8 - 0xCF
    N, N, N, N, N, N, N, N,
// 0xD0 - 0xC7
    N, N, N, N, N, N, N, N,
// 0xD8 - 0xDF
    N, N, N, N, N, N, N, N,
// 0xE0 - 0xE7
    N, N, N, N, N, N, N, N,
// 0xE8 - 0xEF
    N, N, N, N, N, N, N, N,
// 0xF0 - 0xF7
    N, N, N, N, N, N, N, N,
// 0xF8 - 0xFF
    N, N, N, N, N, N, N, N,
    } };
    static const struct instr_dual instr_dual_0f_c3 = {
    I(DstMem | SrcReg | ModRM | No16 | Mov, em_mov), N
    };
    static const struct mode_dual mode_dual_63 = {
    N, I(DstReg | SrcMem32 | ModRM | Mov, em_movsxd)
    };
    static const struct instr_dual instr_dual_8d = {
    D(DstReg | SrcMem | ModRM | NoAccess), N
    };
    static const struct opcode opcode_table[256] = {
// 0x00 - 0x07
    I6ALU(Lock, em_add),
    I(ImplicitOps | Stack | No64 | Src2ES, em_push_sreg),
    I(ImplicitOps | Stack | No64 | Src2ES, em_pop_sreg),
// 0x08 - 0x0F
    I6ALU(Lock | PageTable, em_or),
    I(ImplicitOps | Stack | No64 | Src2CS, em_push_sreg),
    N,
// 0x10 - 0x17
    I6ALU(Lock, em_adc),
    I(ImplicitOps | Stack | No64 | Src2SS, em_push_sreg),
    I(ImplicitOps | Stack | No64 | Src2SS, em_pop_sreg),
// 0x18 - 0x1F
    I6ALU(Lock, em_sbb),
    I(ImplicitOps | Stack | No64 | Src2DS, em_push_sreg),
    I(ImplicitOps | Stack | No64 | Src2DS, em_pop_sreg),
// 0x20 - 0x27
    I6ALU(Lock | PageTable, em_and), N, N,
// 0x28 - 0x2F
    I6ALU(Lock, em_sub), N, I(ByteOp | DstAcc | No64, em_das),
// 0x30 - 0x37
    I6ALU(Lock, em_xor), N, N,
// 0x38 - 0x3F
    I6ALU(NoWrite, em_cmp), N, N,
// 0x40 - 0x4F
    X8(I(DstReg, em_inc)), X8(I(DstReg, em_dec)),
// 0x50 - 0x57
    X8(I(SrcReg | Stack, em_push)),
// 0x58 - 0x5F
    X8(I(DstReg | Stack, em_pop)),
// 0x60 - 0x67
    I(ImplicitOps | Stack | No64, em_pusha),
    I(ImplicitOps | Stack | No64, em_popa),
    N, MD(ModRM, &mode_dual_63),
    N, N, N, N,
// 0x68 - 0x6F
    I(SrcImm | Mov | Stack, em_push),
    I(DstReg | SrcMem | ModRM | Src2Imm, em_imul_3op),
    I(SrcImmByte | Mov | Stack, em_push),
    I(DstReg | SrcMem | ModRM | Src2ImmByte, em_imul_3op),
    I2bvIP(DstDI | SrcDX | Mov | String | Unaligned, em_in, ins, check_perm_in), /* insb, insw/insd */
    I2bvIP(SrcSI | DstDX | String, em_out, outs, check_perm_out), /* outsb, outsw/outsd */
// 0x70 - 0x7F
    X16(D(SrcImmByte | NearBranch | IsBranch)),
// 0x80 - 0x87
    G(ByteOp | DstMem | SrcImm, group1),
    G(DstMem | SrcImm, group1),
    G(ByteOp | DstMem | SrcImm | No64, group1),
    G(DstMem | SrcImmByte, group1),
    I2bv(DstMem | SrcReg | ModRM | NoWrite, em_test),
    I2bv(DstMem | SrcReg | ModRM | Lock | PageTable, em_xchg),
// 0x88 - 0x8F
    I2bv(DstMem | SrcReg | ModRM | Mov | PageTable, em_mov),
    I2bv(DstReg | SrcMem | ModRM | Mov, em_mov),
    I(DstMem | SrcNone | ModRM | Mov | PageTable, em_mov_rm_sreg),
    ID(0, &instr_dual_8d),
    I(ImplicitOps | SrcMem16 | ModRM, em_mov_sreg_rm),
    G(0, group1A),
// 0x90 - 0x97
    DI(SrcAcc | DstReg, pause), X7(D(SrcAcc | DstReg)),
// 0x98 - 0x9F
    D(DstAcc | SrcNone), I(ImplicitOps | SrcAcc, em_cwd),
    I(SrcImmFAddr | No64 | IsBranch | ShadowStack, em_call_far), N,
    II(ImplicitOps | Stack, em_pushf, pushf),
    II(ImplicitOps | Stack, em_popf, popf),
    I(ImplicitOps, em_sahf), I(ImplicitOps, em_lahf),
// 0xA0 - 0xA7
    I2bv(DstAcc | SrcMem | Mov | MemAbs, em_mov),
    I2bv(DstMem | SrcAcc | Mov | MemAbs | PageTable, em_mov),
    I2bv(SrcSI | DstDI | Mov | String | TwoMemOp, em_mov),
    I2bv(SrcSI | DstDI | String | NoWrite | TwoMemOp, em_cmp_r),
// 0xA8 - 0xAF
    I2bv(DstAcc | SrcImm | NoWrite, em_test),
    I2bv(SrcAcc | DstDI | Mov | String, em_mov),
    I2bv(SrcSI | DstAcc | Mov | String, em_mov),
    I2bv(SrcAcc | DstDI | String | NoWrite, em_cmp_r),
// 0xB0 - 0xB7
    X8(I(ByteOp | DstReg | SrcImm | Mov, em_mov)),
// 0xB8 - 0xBF
    X8(I(DstReg | SrcImm64 | Mov, em_mov)),
// 0xC0 - 0xC7
    G(ByteOp | Src2ImmByte, group2), G(Src2ImmByte, group2),
    I(ImplicitOps | NearBranch | SrcImmU16 | IsBranch | ShadowStack, em_ret_near_imm),
    I(ImplicitOps | NearBranch | IsBranch | ShadowStack, em_ret),
    I(DstReg | SrcMemFAddr | ModRM | No64 | Src2ES, em_lseg),
    I(DstReg | SrcMemFAddr | ModRM | No64 | Src2DS, em_lseg),
    G(ByteOp, group11), G(0, group11),
// 0xC8 - 0xCF
    I(Stack | SrcImmU16 | Src2ImmByte, em_enter),
    I(Stack, em_leave),
    I(ImplicitOps | SrcImmU16 | IsBranch | ShadowStack, em_ret_far_imm),
    I(ImplicitOps | IsBranch | ShadowStack, em_ret_far),
    D(ImplicitOps | IsBranch), DI(SrcImmByte | IsBranch | ShadowStack, intn),
    D(ImplicitOps | No64 | IsBranch),
    II(ImplicitOps | IsBranch | ShadowStack, em_iret, iret),
// 0xD0 - 0xD7
    G(Src2One | ByteOp, group2), G(Src2One, group2),
    G(Src2CL | ByteOp, group2), G(Src2CL, group2),
    I(DstAcc | SrcImmUByte | No64, em_aam),
    I(DstAcc | SrcImmUByte | No64, em_aad),
    I(DstAcc | ByteOp | No64, em_salc),
    I(DstAcc | SrcXLat | ByteOp, em_mov),
// 0xD8 - 0xDF
    N, E(0, &escape_d9), N, E(0, &escape_db), N, E(0, &escape_dd), N, N,
// 0xE0 - 0xE7
    X3(I(SrcImmByte | NearBranch | IsBranch, em_loop)),
    I(SrcImmByte | NearBranch | IsBranch, em_jcxz),
    I2bvIP(SrcImmUByte | DstAcc, em_in,  in,  check_perm_in),
    I2bvIP(SrcAcc | DstImmUByte, em_out, out, check_perm_out),
// 0xE8 - 0xEF
    I(SrcImm | NearBranch | IsBranch | ShadowStack, em_call),
    D(SrcImm | ImplicitOps | NearBranch | IsBranch),
    I(SrcImmFAddr | No64 | IsBranch, em_jmp_far),
    D(SrcImmByte | ImplicitOps | NearBranch | IsBranch),
    I2bvIP(SrcDX | DstAcc, em_in,  in,  check_perm_in),
    I2bvIP(SrcAcc | DstDX, em_out, out, check_perm_out),
// 0xF0 - 0xF7
    N, DI(ImplicitOps, icebp), N, N,
    DI(ImplicitOps | Priv, hlt), D(ImplicitOps),
    G(ByteOp, group3), G(0, group3),
// 0xF8 - 0xFF
    D(ImplicitOps), D(ImplicitOps),
    I(ImplicitOps, em_cli), I(ImplicitOps, em_sti),
    D(ImplicitOps), D(ImplicitOps), G(0, group4), G(0, group5),
    };
    static const struct opcode twobyte_table[256] = {
// 0x00 - 0x0F
    G(0, group6), GD(0, &group7), N, N,
    N, I(ImplicitOps | EmulateOnUD | IsBranch | ShadowStack, em_syscall),
    II(ImplicitOps | Priv, em_clts, clts), N,
    DI(ImplicitOps | Priv, invd), DI(ImplicitOps | Priv, wbinvd), N, N,
    N, D(ImplicitOps | ModRM | SrcMem | NoAccess), N, N,
// 0x10 - 0x1F
    GP(ModRM | DstReg | SrcMem | Mov | Sse | Avx, &pfx_0f_10_0f_11),
    GP(ModRM | DstMem | SrcReg | Mov | Sse | Avx, &pfx_0f_10_0f_11),
    N, N, N, N, N, N,
    D(ImplicitOps | ModRM | SrcMem | NoAccess), /* 4 * prefetch + 4 * reserved NOP */
    D(ImplicitOps | ModRM | SrcMem | NoAccess), N, N,
    D(ImplicitOps | ModRM | SrcMem | NoAccess), /* 8 * reserved NOP */
    D(ImplicitOps | ModRM | SrcMem | NoAccess), /* 8 * reserved NOP */
    D(ImplicitOps | ModRM | SrcMem | NoAccess), /* 8 * reserved NOP */
    D(ImplicitOps | ModRM | SrcMem | NoAccess), /* NOP + 7 * reserved NOP */
// 0x20 - 0x2F
    DIP(ModRM | DstMem | Priv | Op3264 | NoMod, cr_read, check_cr_access),
    DIP(ModRM | DstMem | Op3264 | NoMod, dr_read, check_dr_read),
    IIP(ModRM | SrcMem | Priv | Op3264 | NoMod, em_cr_write, cr_write,
    check_cr_access),
    IIP(ModRM | SrcMem | Op3264 | NoMod, em_dr_write, dr_write, check_dr_write),
    N, N, N, N,
    GP(ModRM | DstReg | SrcMem | Mov | Sse | Avx, &pfx_0f_28_0f_29),
    GP(ModRM | DstMem | SrcReg | Mov | Sse | Avx, &pfx_0f_28_0f_29),
    N, GP(ModRM | DstMem | SrcReg | Mov | Sse | Avx, &pfx_0f_2b),
    N, N, N, N,
// 0x30 - 0x3F
    II(ImplicitOps | Priv, em_wrmsr, wrmsr),
    IIP(ImplicitOps, em_rdtsc, rdtsc, check_rdtsc),
    II(ImplicitOps | Priv, em_rdmsr, rdmsr),
    IIP(ImplicitOps, em_rdpmc, rdpmc, check_rdpmc),
    I(ImplicitOps | EmulateOnUD | IsBranch | ShadowStack, em_sysenter),
    I(ImplicitOps | Priv | EmulateOnUD | IsBranch | ShadowStack, em_sysexit),
    N, N,
    N, N, N, N, N, N, N, N,
// 0x40 - 0x4F
    X16(D(DstReg | SrcMem | ModRM)),
// 0x50 - 0x5F
    N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N,
// 0x60 - 0x6F
    N, N, N, N,
    N, N, N, N,
    N, N, N, N,
    N, N, N, GP(SrcMem | DstReg | ModRM | Mov, &pfx_0f_6f_0f_7f),
// 0x70 - 0x7F
    N, N, N, N,
    N, N, N, N,
    N, N, N, N,
    N, N, N, GP(SrcReg | DstMem | ModRM | Mov, &pfx_0f_6f_0f_7f),
// 0x80 - 0x8F
    X16(D(SrcImm | NearBranch | IsBranch)),
// 0x90 - 0x9F
    X16(D(ByteOp | DstMem | SrcNone | ModRM| Mov)),
// 0xA0 - 0xA7
    I(Stack | Src2FS, em_push_sreg), I(Stack | Src2FS, em_pop_sreg),
    II(ImplicitOps, em_cpuid, cpuid),
    I(DstMem | SrcReg | ModRM | BitOp | NoWrite, em_bt),
    I(DstMem | SrcReg | Src2ImmByte | ModRM, em_shld),
    I(DstMem | SrcReg | Src2CL | ModRM, em_shld), N, N,
// 0xA8 - 0xAF
    I(Stack | Src2GS, em_push_sreg), I(Stack | Src2GS, em_pop_sreg),
    II(EmulateOnUD | ImplicitOps, em_rsm, rsm),
    I(DstMem | SrcReg | ModRM | BitOp | Lock | PageTable, em_bts),
    I(DstMem | SrcReg | Src2ImmByte | ModRM, em_shrd),
    I(DstMem | SrcReg | Src2CL | ModRM, em_shrd),
    GD(0, &group15), I(DstReg | SrcMem | ModRM, em_imul),
// 0xB0 - 0xB7
    I2bv(DstMem | SrcReg | ModRM | Lock | PageTable | SrcWrite, em_cmpxchg),
    I(DstReg | SrcMemFAddr | ModRM | Src2SS, em_lseg),
    I(DstMem | SrcReg | ModRM | BitOp | Lock, em_btr),
    I(DstReg | SrcMemFAddr | ModRM | Src2FS, em_lseg),
    I(DstReg | SrcMemFAddr | ModRM | Src2GS, em_lseg),
    D(DstReg | SrcMem8 | ModRM | Mov), D(DstReg | SrcMem16 | ModRM | Mov),
// 0xB8 - 0xBF
    N, N,
    G(BitOp, group8),
    I(DstMem | SrcReg | ModRM | BitOp | Lock | PageTable, em_btc),
    I(DstReg | SrcMem | ModRM, em_bsf_c),
    I(DstReg | SrcMem | ModRM, em_bsr_c),
    D(DstReg | SrcMem8 | ModRM | Mov), D(DstReg | SrcMem16 | ModRM | Mov),
// 0xC0 - 0xC7
    I2bv(DstMem | SrcReg | ModRM | SrcWrite | Lock, em_xadd),
    N, ID(0, &instr_dual_0f_c3),
    N, N, N, GD(0, &group9),
// 0xC8 - 0xCF
    X8(I(DstReg, em_bswap)),
// 0xD0 - 0xDF
    N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N,
// 0xE0 - 0xEF
    N, N, N, N, N, N, N, GP(SrcReg | DstMem | ModRM | Mov, &pfx_0f_e7_0f_38_2a),
    N, N, N, N, N, N, N, N,
// 0xF0 - 0xFF
    N, N, N, N, N, N, N, N, N, N, N, N, N, N, N, N
    };
    static const struct instr_dual instr_dual_0f_38_f0 = {
    I(DstReg | SrcMem | Mov, em_movbe), N
    };
    static const struct instr_dual instr_dual_0f_38_f1 = {
    I(DstMem | SrcReg | Mov, em_movbe), N
    };
    static const struct gprefix three_byte_0f_38_f0 = {
    ID(0, &instr_dual_0f_38_f0), ID(0, &instr_dual_0f_38_f0), N, N
    };
    static const struct gprefix three_byte_0f_38_f1 = {
    ID(0, &instr_dual_0f_38_f1), ID(0, &instr_dual_0f_38_f1), N, N
    };
//
// Insns below are selected by the prefix which indexed by the third opcode
// byte.
//
    static const struct opcode opcode_map_0f_38[256] = {
// 0x00 - 0x1f
    X16(N), X16(N),
// 0x20 - 0x2f
    X8(N),
    X2(N), GP(SrcMem | DstReg | ModRM | Mov | Aligned, &pfx_0f_e7_0f_38_2a), N, N, N, N, N,
// 0x30 - 0x7f
    X16(N), X16(N), X16(N), X16(N), X16(N),
// 0x80 - 0xef
    X16(N), X16(N), X16(N), X16(N), X16(N), X16(N), X16(N),
// 0xf0 - 0xf1
    GP(EmulateOnUD | ModRM, &three_byte_0f_38_f0),
    GP(EmulateOnUD | ModRM, &three_byte_0f_38_f1),
// 0xf2 - 0xff
    N, N, X4(N), X8(N)
    };

#[no_mangle]
unsafe extern "C" fn is_shstk_instruction(ctxt: *mut x86_emulate_ctxt) -> bool {
    static bool is_shstk_instruction(struct x86_emulate_ctxt *ctxt)
    {
    return ctxt.d & ShadowStack;
    }
#[no_mangle]
unsafe extern "C" fn is_ibt_instruction(ctxt: *mut x86_emulate_ctxt) -> bool {
    static bool is_ibt_instruction(struct x86_emulate_ctxt *ctxt)
    {
    let mut flags: u64 = ctxt.d;
    if (!(flags & IsBranch))
    return false;
//
// All far JMPs and CALLs (including SYSCALL, SYSENTER, and INTn) are
// indirect and thus affect IBT state.  All far RETs (including SYSEXIT
// and IRET) are protected via Shadow Stacks and thus don't affect IBT
// state.  IRET #GPs when returning to virtual-8086 and IBT or SHSTK is
// enabled, but that should be handled by IRET emulation (in the very
// unlikely scenario that KVM adds support for fully emulating IRET).
//
    if (!(flags & NearBranch))
    return ctxt.execute != em_iret &&
    ctxt.execute != em_ret_far &&
    ctxt.execute != em_ret_far_imm &&
    ctxt.execute != em_sysexit;
    switch (flags & SrcMask) {
    case SrcReg:
    case SrcMem:
    case SrcMem16:
    case SrcMem32:
    return true;
    case SrcMemFAddr:
    case SrcImmFAddr:
// Far branches should be handled above.
    WARN_ON_ONCE(1);
    return true;
    case SrcNone:
    case SrcImm:
    case SrcImmByte:
//
// Note, ImmU16 is used only for the stack adjustment operand on ENTER
// and RET instructions.  ENTER isn't a branch and RET FAR is handled
// by the NearBranch check above.  RET itself isn't an indirect branch.
//
    case SrcImmU16:
    return false;
    default:
    WARN_ONCE(1, "Unexpected Src operand '%llx' on branch",
    flags & SrcMask);
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn imm_size(ctxt: *mut x86_emulate_ctxt) -> unsigned {
    static unsigned imm_size(struct x86_emulate_ctxt *ctxt)
    {
    unsigned size;
    size = (ctxt.d & ByteOp) ? 1 : ctxt.op_bytes;
    if (size == 8)
    size = 4;
    return size;
    }
    static int decode_imm(struct x86_emulate_ctxt *ctxt, struct operand *op,
    unsigned size, bool sign_extension)
    {
    let mut rc: c_int = X86EMUL_CONTINUE;
    op.type = OP_IMM;
    op.bytes = size;
    op.addr.mem.ea = ctxt._eip;
// NB. Immediates are sign-extended as necessary.
    switch (op.bytes) {
    case 1:
    op.val = insn_fetch(s8, ctxt);
    break;
    case 2:
    op.val = insn_fetch(s16, ctxt);
    break;
    case 4:
    op.val = insn_fetch(s32, ctxt);
    break;
    case 8:
    op.val = insn_fetch(s64, ctxt);
    break;
    }
    if (!sign_extension) {
    switch (op.bytes) {
    case 1:
    op.val &= 0xff;
    break;
    case 2:
    op.val &= 0xffff;
    break;
    case 4:
    op.val &= 0xffffffff;
    break;
    }
    }
    done:
    return rc;
    }
    static int decode_operand(struct x86_emulate_ctxt *ctxt, struct operand *op,
    unsigned d)
    {
    let mut rc: c_int = X86EMUL_CONTINUE;
    switch (d) {
    case OpReg:
    decode_register_operand(ctxt, op);
    break;
    case OpImmUByte:
    rc = decode_imm(ctxt, op, 1, false);
    break;
    case OpMem:
    ctxt.memop.bytes = (ctxt.d & ByteOp) ? 1 : ctxt.op_bytes;
    mem_common:
// op = ctxt->memop;
    ctxt.memopp = op;
    if (ctxt.d & BitOp)
    fetch_bit_operand(ctxt);
    op.orig_val = op.val;
    break;
    case OpMem64:
    ctxt.memop.bytes = (ctxt.op_bytes == 8) ? 16 : 8;
    goto mem_common;
    case OpAcc:
    op.type = OP_REG;
    op.bytes = (ctxt.d & ByteOp) ? 1 : ctxt.op_bytes;
    op.addr.reg = reg_rmw(ctxt, VCPU_REGS_RAX);
    fetch_register_operand(op);
    break;
    case OpAccLo:
    op.type = OP_REG;
    op.bytes = (ctxt.d & ByteOp) ? 2 : ctxt.op_bytes;
    op.addr.reg = reg_rmw(ctxt, VCPU_REGS_RAX);
    fetch_register_operand(op);
    break;
    case OpAccHi:
    if (ctxt.d & ByteOp) {
    op.type = OP_NONE;
    break;
    }
    op.type = OP_REG;
    op.bytes = ctxt.op_bytes;
    op.addr.reg = reg_rmw(ctxt, VCPU_REGS_RDX);
    fetch_register_operand(op);
    break;
    case OpDI:
    op.type = OP_MEM;
    op.bytes = (ctxt.d & ByteOp) ? 1 : ctxt.op_bytes;
    op.addr.mem.ea =
    register_address(ctxt, VCPU_REGS_RDI);
    op.addr.mem.seg = VCPU_SREG_ES;
    op.val = 0;
    op.count = 1;
    break;
    case OpDX:
    op.type = OP_REG;
    op.bytes = 2;
    op.addr.reg = reg_rmw(ctxt, VCPU_REGS_RDX);
    fetch_register_operand(op);
    break;
    case OpCL:
    op.type = OP_IMM;
    op.bytes = 1;
    op.val = reg_read(ctxt, VCPU_REGS_RCX) & 0xff;
    break;
    case OpImmByte:
    rc = decode_imm(ctxt, op, 1, true);
    break;
    case OpOne:
    op.type = OP_IMM;
    op.bytes = 1;
    op.val = 1;
    break;
    case OpImm:
    rc = decode_imm(ctxt, op, imm_size(ctxt), true);
    break;
    case OpImm64:
    rc = decode_imm(ctxt, op, ctxt.op_bytes, true);
    break;
    case OpMem8:
    ctxt.memop.bytes = 1;
    if (ctxt.memop.type == OP_REG) {
    ctxt.memop.addr.reg = decode_register(ctxt,
    ctxt.modrm_rm, true);
    fetch_register_operand(&ctxt.memop);
    }
    goto mem_common;
    case OpMem16:
    ctxt.memop.bytes = 2;
    goto mem_common;
    case OpMem32:
    ctxt.memop.bytes = 4;
    goto mem_common;
    case OpImmU16:
    rc = decode_imm(ctxt, op, 2, false);
    break;
    case OpImmU:
    rc = decode_imm(ctxt, op, imm_size(ctxt), false);
    break;
    case OpSI:
    op.type = OP_MEM;
    op.bytes = (ctxt.d & ByteOp) ? 1 : ctxt.op_bytes;
    op.addr.mem.ea =
    register_address(ctxt, VCPU_REGS_RSI);
    op.addr.mem.seg = ctxt.seg_override;
    op.val = 0;
    op.count = 1;
    break;
    case OpXLat:
    op.type = OP_MEM;
    op.bytes = (ctxt.d & ByteOp) ? 1 : ctxt.op_bytes;
    op.addr.mem.ea =
    address_mask(ctxt,
    reg_read(ctxt, VCPU_REGS_RBX) +
    (reg_read(ctxt, VCPU_REGS_RAX) & 0xff));
    op.addr.mem.seg = ctxt.seg_override;
    op.val = 0;
    break;
    case OpImmFAddr:
    op.type = OP_IMM;
    op.addr.mem.ea = ctxt._eip;
    op.bytes = ctxt.op_bytes + 2;
    insn_fetch_arr(op.valptr, op.bytes, ctxt);
    break;
    case OpMemFAddr:
    ctxt.memop.bytes = ctxt.op_bytes + 2;
    goto mem_common;
    case OpES:
    op.type = OP_IMM;
    op.val = VCPU_SREG_ES;
    break;
    case OpCS:
    op.type = OP_IMM;
    op.val = VCPU_SREG_CS;
    break;
    case OpSS:
    op.type = OP_IMM;
    op.val = VCPU_SREG_SS;
    break;
    case OpDS:
    op.type = OP_IMM;
    op.val = VCPU_SREG_DS;
    break;
    case OpFS:
    op.type = OP_IMM;
    op.val = VCPU_SREG_FS;
    break;
    case OpGS:
    op.type = OP_IMM;
    op.val = VCPU_SREG_GS;
    break;
    case OpImplicit:
// Special instructions do their own operand decoding.
    default:
    op.type = OP_NONE; /* Disable writeback. */
    break;
    }
    done:
    return rc;
    }
    static int x86_decode_avx(struct x86_emulate_ctxt *ctxt,
    u8 vex_1st, u8 vex_2nd, struct opcode *opcode)
    {
    u8 vex_3rd, map, pp, l, v;
    let mut rc: c_int = X86EMUL_CONTINUE;
    if (ctxt.rep_prefix || ctxt.op_prefix || ctxt.rex_prefix)
    goto ud;
    if (vex_1st == 0xc5) {
// Expand RVVVVlpp to VEX3 format
    vex_3rd = vex_2nd & ~0x80;         /* VVVVlpp from VEX2, w=0 */
    vex_2nd = (vex_2nd & 0x80) | 0x61; /* R from VEX2, X=1 B=1 mmmmm=00001 */
    } else {
    vex_3rd = insn_fetch(u8, ctxt);
    }
// vex_2nd = RXBmmmmm, vex_3rd = wVVVVlpp.  Fix polarity
    vex_2nd ^= 0xE0; /* binary 11100000 */
    vex_3rd ^= 0x78; /* binary 01111000 */
    ctxt.rex_prefix = REX_PREFIX;
    ctxt.rex_bits = (vex_2nd & 0xE0) >> 5; /* RXB */
    ctxt.rex_bits |= (vex_3rd & 0x80) >> 4; /* w */
    if (ctxt.rex_bits && ctxt.mode != X86EMUL_MODE_PROT64)
    goto ud;
    map = vex_2nd & 0x1f;
    v = (vex_3rd >> 3) & 0xf;
    l = vex_3rd & 0x4;
    pp = vex_3rd & 0x3;
    ctxt.b = insn_fetch(u8, ctxt);
    switch (map) {
    case 1:
    ctxt.opcode_len = 2;
// opcode = twobyte_table[ctxt->b];
    break;
    case 2:
    ctxt.opcode_len = 3;
// opcode = opcode_map_0f_38[ctxt->b];
    break;
    case 3:
// no 0f 3a instructions are supported yet
    return X86EMUL_UNHANDLEABLE;
    default:
    goto ud;
    }
//
// No three operand instructions are supported yet; those that
// *are* marked with the Avx flag reserve the VVVV flag.
//
    if (v)
    goto ud;
    if (l)
    ctxt.op_bytes = 32;
    else
    ctxt.op_bytes = 16;
    switch (pp) {
    case 0: break;
    case 1: ctxt.op_prefix = true; break;
    case 2: ctxt.rep_prefix = 0xf3; break;
    case 3: ctxt.rep_prefix = 0xf2; break;
    }
    done:
    return rc;
    ud:
// opcode = ud;
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn x86_decode_insn(ctxt: *mut x86_emulate_ctxt, insn: *mut c_void, insn_len: c_int, emulation_type: c_int) -> c_int {
    int x86_decode_insn(struct x86_emulate_ctxt *ctxt, void *insn, int insn_len, int emulation_type)
    {
    let mut rc: c_int = X86EMUL_CONTINUE;
    let mut mode: c_int = ctxt.mode;
    int def_op_bytes, def_ad_bytes, goffset, simd_prefix;
    let mut vex_prefix: bool = false;
    let mut has_seg_override: bool = false;
    struct opcode opcode;
    u16 dummy;
    struct desc_struct desc;
    ctxt.memop.type = OP_NONE;
    ctxt.memopp = core::ptr::null_mut();
    ctxt._eip = ctxt.eip;
    ctxt.fetch.ptr = ctxt.fetch.data;
    ctxt.fetch.end = ctxt.fetch.data + insn_len;
    ctxt.opcode_len = 1;
    ctxt.intercept = x86_intercept_none;
    if (insn_len > 0)
    memcpy(ctxt.fetch.data, insn, insn_len);
    else {
    rc = __do_insn_fetch_bytes(ctxt, 1);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    }
    switch (mode) {
    case X86EMUL_MODE_REAL:
    case X86EMUL_MODE_VM86:
    def_op_bytes = def_ad_bytes = 2;
    ctxt.ops.get_segment(ctxt, &dummy, &desc, core::ptr::null_mut(), VCPU_SREG_CS);
    if (desc.d)
    def_op_bytes = def_ad_bytes = 4;
    break;
    case X86EMUL_MODE_PROT16:
    def_op_bytes = def_ad_bytes = 2;
    break;
    case X86EMUL_MODE_PROT32:
    def_op_bytes = def_ad_bytes = 4;
    break;

    case X86EMUL_MODE_PROT64:
    def_op_bytes = 4;
    def_ad_bytes = 8;
    break;

    default:
    return EMULATION_FAILED;
    }
    ctxt.op_bytes = def_op_bytes;
    ctxt.ad_bytes = def_ad_bytes;
// Legacy prefixes.
    for (;;) {
    switch (ctxt.b = insn_fetch(u8, ctxt)) {
    case 0x66:	/* operand-size override */
    ctxt.op_prefix = true;
// switch between 2/4 bytes
    ctxt.op_bytes = def_op_bytes ^ 6;
    break;
    case 0x67:	/* address-size override */
    if (mode == X86EMUL_MODE_PROT64)
// switch between 4/8 bytes
    ctxt.ad_bytes = def_ad_bytes ^ 12;
    else
// switch between 2/4 bytes
    ctxt.ad_bytes = def_ad_bytes ^ 6;
    break;
    case 0x26:	/* ES override */
    has_seg_override = true;
    ctxt.seg_override = VCPU_SREG_ES;
    break;
    case 0x2e:	/* CS override */
    has_seg_override = true;
    ctxt.seg_override = VCPU_SREG_CS;
    break;
    case 0x36:	/* SS override */
    has_seg_override = true;
    ctxt.seg_override = VCPU_SREG_SS;
    break;
    case 0x3e:	/* DS override */
    has_seg_override = true;
    ctxt.seg_override = VCPU_SREG_DS;
    break;
    case 0x64:	/* FS override */
    has_seg_override = true;
    ctxt.seg_override = VCPU_SREG_FS;
    break;
    case 0x65:	/* GS override */
    has_seg_override = true;
    ctxt.seg_override = VCPU_SREG_GS;
    break;
    case 0x40 ... 0x4f: /* REX */
    if (mode != X86EMUL_MODE_PROT64)
    goto done_prefixes;
    ctxt.rex_prefix = REX_PREFIX;
    ctxt.rex_bits   = ctxt.b & 0xf;
    continue;
    case 0xf0:	/* LOCK */
    ctxt.lock_prefix = 1;
    break;
    case 0xf2:	/* REPNE/REPNZ */
    case 0xf3:	/* REP/REPE/REPZ */
    ctxt.rep_prefix = ctxt.b;
    break;
    default:
    goto done_prefixes;
    }
// Any legacy prefix after a REX prefix nullifies its effect.
    ctxt.rex_prefix = REX_NONE;
    ctxt.rex_bits = 0;
    }
    done_prefixes:
// REX prefix.
    if (ctxt.rex_bits & REX_W)
    ctxt.op_bytes = 8;
// Opcode byte(s).
    if (ctxt.b == 0xc4 || ctxt.b == 0xc5) {
// VEX or LDS/LES
    let mut vex_2nd: u8 = insn_fetch(u8, ctxt);
    if (mode != X86EMUL_MODE_PROT64 && (vex_2nd & 0xc0) != 0xc0) {
    opcode = opcode_table[ctxt.b];
    ctxt.modrm = vex_2nd;
// the Mod/RM byte has been fetched already!
    goto done_modrm;
    }
    vex_prefix = true;
    rc = x86_decode_avx(ctxt, ctxt.b, vex_2nd, &opcode);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    } else if (ctxt.b == 0x0f) {
// Two- or three-byte opcode
    ctxt.opcode_len = 2;
    ctxt.b = insn_fetch(u8, ctxt);
    opcode = twobyte_table[ctxt.b];
// 0F_38 opcode map
    if (ctxt.b == 0x38) {
    ctxt.opcode_len = 3;
    ctxt.b = insn_fetch(u8, ctxt);
    opcode = opcode_map_0f_38[ctxt.b];
    }
    } else {
// Opcode byte(s).
    opcode = opcode_table[ctxt.b];
    }
    if (opcode.flags & ModRM)
    ctxt.modrm = insn_fetch(u8, ctxt);
    done_modrm:
    ctxt.d = opcode.flags;
    while (ctxt.d & GroupMask) {
    switch (ctxt.d & GroupMask) {
    case Group:
    goffset = (ctxt.modrm >> 3) & 7;
    opcode = opcode.u.group[goffset];
    break;
    case GroupDual:
    goffset = (ctxt.modrm >> 3) & 7;
    if ((ctxt.modrm >> 6) == 3)
    opcode = opcode.u.gdual.mod3[goffset];
    else
    opcode = opcode.u.gdual.mod012[goffset];
    break;
    case RMExt:
    goffset = ctxt.modrm & 7;
    opcode = opcode.u.group[goffset];
    break;
    case Prefix:
    if (ctxt.rep_prefix && ctxt.op_prefix)
    return EMULATION_FAILED;
    simd_prefix = ctxt.op_prefix ? 0x66 : ctxt.rep_prefix;
    switch (simd_prefix) {
    case 0x00: opcode = opcode.u.gprefix.pfx_no; break;
    case 0x66: opcode = opcode.u.gprefix.pfx_66; break;
    case 0xf2: opcode = opcode.u.gprefix.pfx_f2; break;
    case 0xf3: opcode = opcode.u.gprefix.pfx_f3; break;
    }
    break;
    case Escape:
    if (ctxt.modrm > 0xbf) {
    let mut size: usize = ARRAY_SIZE(opcode.u.esc.high);
    u32 index = array_index_nospec(
    ctxt.modrm - 0xc0, size);
    opcode = opcode.u.esc.high[index];
    } else {
    opcode = opcode.u.esc.op[(ctxt.modrm >> 3) & 7];
    }
    break;
    case InstrDual:
    if ((ctxt.modrm >> 6) == 3)
    opcode = opcode.u.idual.mod3;
    else
    opcode = opcode.u.idual.mod012;
    break;
    case ModeDual:
    if (ctxt.mode == X86EMUL_MODE_PROT64)
    opcode = opcode.u.mdual.mode64;
    else
    opcode = opcode.u.mdual.mode32;
    break;
    default:
    return EMULATION_FAILED;
    }
    ctxt.d &= ~(u64)GroupMask;
    ctxt.d |= opcode.flags;
    }
    ctxt.is_branch = opcode.flags & IsBranch;
// Unrecognised?
    if (ctxt.d == 0)
    return EMULATION_FAILED;
    if (unlikely(vex_prefix)) {
//
// Only specifically marked instructions support VEX.  Since many
// instructions support it but are not annotated, return not implemented
// rather than #UD.
//
    if (!(ctxt.d & Avx))
    return EMULATION_FAILED;
    if (!(ctxt.d & AlignMask))
    ctxt.d |= Unaligned;
    }
    ctxt.execute = opcode.u.execute;
//
// Reject emulation if KVM might need to emulate shadow stack updates
// and/or indirect branch tracking enforcement, which the emulator
// doesn't support.
//
    if ((is_ibt_instruction(ctxt) || is_shstk_instruction(ctxt)) &&
    ctxt.ops.get_cr(ctxt, 4) & X86_CR4_CET) {
    let mut u_cet: u64 = 0, s_cet = 0;
//
// Check both User and Supervisor on far transfers as inter-
// privilege level transfers are impacted by CET at the target
// privilege level, and that is not known at this time.  The
// expectation is that the guest will not require emulation of
// any CET-affected instructions at any privilege level.
//
    if (!(ctxt.d & NearBranch))
    u_cet = s_cet = CET_SHSTK_EN | CET_ENDBR_EN;
#[no_mangle]
pub unsafe extern "C" fn if(3: ctxt->ops->cpl(ctxt) ==) -> else {
    else if (ctxt.ops.cpl(ctxt) == 3)
    u_cet = CET_SHSTK_EN | CET_ENDBR_EN;
    else
    s_cet = CET_SHSTK_EN | CET_ENDBR_EN;
    if ((u_cet && ctxt.ops.get_msr(ctxt, MSR_IA32_U_CET, &u_cet)) ||
    (s_cet && ctxt.ops.get_msr(ctxt, MSR_IA32_S_CET, &s_cet)))
    return EMULATION_FAILED;
    if ((u_cet | s_cet) & CET_SHSTK_EN && is_shstk_instruction(ctxt))
    return EMULATION_FAILED;
    if ((u_cet | s_cet) & CET_ENDBR_EN && is_ibt_instruction(ctxt))
    return EMULATION_FAILED;
    }
    if (unlikely(emulation_type & EMULTYPE_TRAP_UD) &&
    likely(!(ctxt.d & EmulateOnUD)))
    return EMULATION_FAILED;
    if (unlikely(ctxt.d &
    (NotImpl|Stack|Op3264|Sse|Mmx|Intercept|CheckPerm|NearBranch|
    No16))) {
//
// These are copied unconditionally here, and checked unconditionally
// in x86_emulate_insn.
//
    ctxt.check_perm = opcode.check_perm;
    ctxt.intercept = opcode.intercept;
    if (ctxt.d & NotImpl)
    return EMULATION_FAILED;
    if (mode == X86EMUL_MODE_PROT64) {
    if (ctxt.op_bytes == 4 && (ctxt.d & Stack))
    ctxt.op_bytes = 8;
#[no_mangle]
pub unsafe extern "C" fn if(NearBranch: ctxt->d &) -> else {
    else if (ctxt.d & NearBranch)
    ctxt.op_bytes = 8;
    }
    if (ctxt.d & Op3264) {
    if (mode == X86EMUL_MODE_PROT64)
    ctxt.op_bytes = 8;
    else
    ctxt.op_bytes = 4;
    }
    if ((ctxt.d & No16) && ctxt.op_bytes == 2)
    ctxt.op_bytes = 4;
    if (vex_prefix)
    ;
#[no_mangle]
pub unsafe extern "C" fn if(Sse: ctxt->d &) -> else {
    else if (ctxt.d & Sse)
    ctxt.op_bytes = 16, ctxt.d &= ~Avx;
#[no_mangle]
pub unsafe extern "C" fn if(Mmx: ctxt->d &) -> else {
    else if (ctxt.d & Mmx)
    ctxt.op_bytes = 8;
    }
// ModRM and SIB bytes.
    if (ctxt.d & ModRM) {
    rc = decode_modrm(ctxt, &ctxt.memop);
    if (!has_seg_override) {
    has_seg_override = true;
    ctxt.seg_override = ctxt.modrm_seg;
    }
    } else if (ctxt.d & MemAbs)
    rc = decode_abs(ctxt, &ctxt.memop);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    if (!has_seg_override)
    ctxt.seg_override = VCPU_SREG_DS;
    ctxt.memop.addr.mem.seg = ctxt.seg_override;
//
// Decode and fetch the source operand: register, memory
// or immediate.
//
    rc = decode_operand(ctxt, &ctxt.src, (ctxt.d >> SrcShift) & OpMask);
    if (rc != X86EMUL_CONTINUE)
    goto done;
//
// Decode and fetch the second source operand: register, memory
// or immediate.
//
    rc = decode_operand(ctxt, &ctxt.src2, (ctxt.d >> Src2Shift) & OpMask);
    if (rc != X86EMUL_CONTINUE)
    goto done;
// Decode and fetch the destination operand: register or memory.
    rc = decode_operand(ctxt, &ctxt.dst, (ctxt.d >> DstShift) & OpMask);
    if (ctxt.rip_relative && likely(ctxt.memopp))
    ctxt.memopp.addr.mem.ea = address_mask(ctxt,
    ctxt.memopp.addr.mem.ea + ctxt._eip);
    done:
    if (rc == X86EMUL_PROPAGATE_FAULT)
    ctxt.have_exception = true;
    return (rc != X86EMUL_CONTINUE) ? EMULATION_FAILED : EMULATION_OK;
    }
#[no_mangle]
pub unsafe extern "C" fn x86_page_table_writing_insn(ctxt: *mut x86_emulate_ctxt) -> bool {
    bool x86_page_table_writing_insn(struct x86_emulate_ctxt *ctxt)
    {
    return ctxt.d & PageTable;
    }
#[no_mangle]
unsafe extern "C" fn string_insn_completed(ctxt: *mut x86_emulate_ctxt) -> bool {
    static bool string_insn_completed(struct x86_emulate_ctxt *ctxt)
    {
// The second termination condition only applies for REPE
// and REPNE. Test if the repeat string operation prefix is
// REPE/REPZ or REPNE/REPNZ and if it's the case it tests the
// corresponding termination condition according to:
// - if REPE/REPZ and ZF = 0 then done
// - if REPNE/REPNZ and ZF = 1 then done
//
    if (((ctxt.b == 0xa6) || (ctxt.b == 0xa7) ||
    (ctxt.b == 0xae) || (ctxt.b == 0xaf))
    && (((ctxt.rep_prefix == REPE_PREFIX) &&
    ((ctxt.eflags & X86_EFLAGS_ZF) == 0))
    || ((ctxt.rep_prefix == REPNE_PREFIX) &&
    ((ctxt.eflags & X86_EFLAGS_ZF) == X86_EFLAGS_ZF))))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn flush_pending_x87_faults(ctxt: *mut x86_emulate_ctxt) -> c_int {
    static int flush_pending_x87_faults(struct x86_emulate_ctxt *ctxt)
    {
    int rc;
    kvm_fpu_get();
    rc = asm_safe("fwait");
    kvm_fpu_put();
    if (unlikely(rc != X86EMUL_CONTINUE))
    return emulate_exception(ctxt, MF_VECTOR, 0, false);
    return X86EMUL_CONTINUE;
    }
#[no_mangle]
unsafe extern "C" fn fetch_possible_mmx_operand(op: *mut operand) {
    static void fetch_possible_mmx_operand(struct operand *op)
    {
    if (op.type == OP_MM)
    kvm_read_mmx_reg(op.addr.mm, &op.mm_val);
    }
#[no_mangle]
pub unsafe extern "C" fn init_decode_cache(ctxt: *mut x86_emulate_ctxt) {
    void init_decode_cache(struct x86_emulate_ctxt *ctxt)
    {
// Clear fields that are set conditionally but read without a guard.
    ctxt.rip_relative = false;
    ctxt.rex_prefix = REX_NONE;
    ctxt.rex_bits = 0;
    ctxt.lock_prefix = 0;
    ctxt.op_prefix = false;
    ctxt.rep_prefix = 0;
    ctxt.regs_valid = 0;
    ctxt.regs_dirty = 0;
    ctxt.io_read.pos = 0;
    ctxt.io_read.end = 0;
    ctxt.mem_read.end = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn x86_emulate_insn(ctxt: *mut x86_emulate_ctxt, check_intercepts: bool) -> c_int {
    int x86_emulate_insn(struct x86_emulate_ctxt *ctxt, bool check_intercepts)
    {
    const struct x86_emulate_ops *ops = ctxt.ops;
    let mut rc: c_int = X86EMUL_CONTINUE;
    let mut saved_dst_type: c_int = ctxt.dst.type;
    ctxt.mem_read.pos = 0;
// LOCK prefix is allowed only with some instructions
    if (ctxt.lock_prefix && (!(ctxt.d & Lock) || ctxt.dst.type != OP_MEM)) {
    rc = emulate_ud(ctxt);
    goto done;
    }
    if ((ctxt.d & SrcMask) == SrcMemFAddr && ctxt.src.type != OP_MEM) {
    rc = emulate_ud(ctxt);
    goto done;
    }
    if (unlikely(ctxt.d &
    (No64|Undefined|Avx|Sse|Mmx|Intercept|CheckPerm|Priv|Prot|String))) {
    if ((ctxt.mode == X86EMUL_MODE_PROT64 && (ctxt.d & No64)) ||
    (ctxt.d & Undefined)) {
    rc = emulate_ud(ctxt);
    goto done;
    }
    if ((ctxt.d & (Avx|Sse|Mmx)) && ((ops.get_cr(ctxt, 0) & X86_CR0_EM))) {
    rc = emulate_ud(ctxt);
    goto done;
    }
    if (ctxt.d & Avx) {
    let mut xcr: u64 = 0;
    if (!(ops.get_cr(ctxt, 4) & X86_CR4_OSXSAVE)
    || ops.get_xcr(ctxt, 0, &xcr)
    || !(xcr & XFEATURE_MASK_YMM)) {
    rc = emulate_ud(ctxt);
    goto done;
    }
    } else if (ctxt.d & Sse) {
    if (!(ops.get_cr(ctxt, 4) & X86_CR4_OSFXSR)) {
    rc = emulate_ud(ctxt);
    goto done;
    }
    }
    if ((ctxt.d & (Avx|Sse|Mmx)) && (ops.get_cr(ctxt, 0) & X86_CR0_TS)) {
    rc = emulate_nm(ctxt);
    goto done;
    }
    if (ctxt.d & Mmx) {
    rc = flush_pending_x87_faults(ctxt);
    if (rc != X86EMUL_CONTINUE)
    goto done;
//
// Now that we know the fpu is exception safe, we can fetch
// operands from it.
//
    fetch_possible_mmx_operand(&ctxt.src);
    fetch_possible_mmx_operand(&ctxt.src2);
    if (!(ctxt.d & Mov))
    fetch_possible_mmx_operand(&ctxt.dst);
    }
    if (unlikely(check_intercepts) && ctxt.intercept) {
    rc = emulator_check_intercept(ctxt, ctxt.intercept,
    X86_ICPT_PRE_EXCEPT);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    }
// Instruction can only be executed in protected mode
    if ((ctxt.d & Prot) && ctxt.mode < X86EMUL_MODE_PROT16) {
    rc = emulate_ud(ctxt);
    goto done;
    }
// Privileged instruction can be executed only in CPL=0
    if ((ctxt.d & Priv) && ops.cpl(ctxt)) {
    if (ctxt.d & PrivUD)
    rc = emulate_ud(ctxt);
    else
    rc = emulate_gp(ctxt, 0);
    goto done;
    }
// Do instruction specific permission checks
    if (ctxt.d & CheckPerm) {
    rc = ctxt.check_perm(ctxt);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    }
    if (unlikely(check_intercepts) && (ctxt.d & Intercept)) {
    rc = emulator_check_intercept(ctxt, ctxt.intercept,
    X86_ICPT_POST_EXCEPT);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    }
    if (ctxt.rep_prefix && (ctxt.d & String)) {
// All REP prefixes have the same first termination condition
    if (address_mask(ctxt, reg_read(ctxt, VCPU_REGS_RCX)) == 0) {
    string_registers_quirk(ctxt);
    ctxt.eip = ctxt._eip;
    ctxt.eflags &= ~X86_EFLAGS_RF;
    goto done;
    }
    }
    }
    if ((ctxt.src.type == OP_MEM) && !(ctxt.d & NoAccess)) {
    rc = segmented_read(ctxt, ctxt.src.addr.mem,
    ctxt.src.valptr, ctxt.src.bytes);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    ctxt.src.orig_val64 = ctxt.src.val64;
    }
    if (ctxt.src2.type == OP_MEM) {
    rc = segmented_read(ctxt, ctxt.src2.addr.mem,
    &ctxt.src2.val, ctxt.src2.bytes);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    }
    if ((ctxt.d & DstMask) == ImplicitOps)
    goto special_insn;
    if ((ctxt.dst.type == OP_MEM) && !(ctxt.d & Mov)) {
// optimisation - avoid slow emulated read if Mov
    rc = segmented_read(ctxt, ctxt.dst.addr.mem,
    &ctxt.dst.val, ctxt.dst.bytes);
    if (rc != X86EMUL_CONTINUE) {
    if (!(ctxt.d & NoWrite) &&
    rc == X86EMUL_PROPAGATE_FAULT &&
    ctxt.exception.vector == PF_VECTOR)
    ctxt.exception.error_code |= PFERR_WRITE_MASK;
    goto done;
    }
    }
// Copy full 64-bit value for CMPXCHG8B.
    ctxt.dst.orig_val64 = ctxt.dst.val64;
    special_insn:
    if (unlikely(check_intercepts) && (ctxt.d & Intercept)) {
    rc = emulator_check_intercept(ctxt, ctxt.intercept,
    X86_ICPT_POST_MEMACCESS);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    }
    if (ctxt.rep_prefix && (ctxt.d & String))
    ctxt.eflags |= X86_EFLAGS_RF;
    else
    ctxt.eflags &= ~X86_EFLAGS_RF;
    if (ctxt.execute) {
    rc = ctxt.execute(ctxt);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    goto writeback;
    }
    if (ctxt.opcode_len == 2)
    goto twobyte_insn;
#[no_mangle]
pub unsafe extern "C" fn if(3: ctxt->opcode_len ==) -> else {
    else if (ctxt.opcode_len == 3)
    goto threebyte_insn;
    switch (ctxt.b) {
    case 0x70 ... 0x7f: /* jcc (short) */
    if (test_cc(ctxt.b, ctxt.eflags))
    rc = jmp_rel(ctxt, ctxt.src.val);
    break;
    case 0x8d: /* lea r16/r32, m */
    ctxt.dst.val = ctxt.src.addr.mem.ea;
    break;
    case 0x90 ... 0x97: /* nop / xchg reg, rax */
    if (ctxt.dst.addr.reg == reg_rmw(ctxt, VCPU_REGS_RAX))
    ctxt.dst.type = OP_NONE;
    else
    rc = em_xchg(ctxt);
    break;
    case 0x98: /* cbw/cwde/cdqe */
    switch (ctxt.op_bytes) {
    case 2: ctxt.dst.val = (s8)ctxt.dst.val; break;
    case 4: ctxt.dst.val = (s16)ctxt.dst.val; break;
    case 8: ctxt.dst.val = (s32)ctxt.dst.val; break;
    }
    break;
    case 0xcc:		/* int3 */
    rc = emulate_int(ctxt, 3);
    break;
    case 0xcd:		/* int n */
    rc = emulate_int(ctxt, ctxt.src.val);
    break;
    case 0xce:		/* into */
    if (ctxt.eflags & X86_EFLAGS_OF)
    rc = emulate_int(ctxt, 4);
    break;
    case 0xe9: /* jmp rel */
    case 0xeb: /* jmp rel short */
    rc = jmp_rel(ctxt, ctxt.src.val);
    ctxt.dst.type = OP_NONE; /* Disable writeback. */
    break;
    case 0xf4:              /* hlt */
    ctxt.ops.halt(ctxt);
    break;
    case 0xf5:	/* cmc */
// complement carry flag from eflags reg
    ctxt.eflags ^= X86_EFLAGS_CF;
    break;
    case 0xf8: /* clc */
    ctxt.eflags &= ~X86_EFLAGS_CF;
    break;
    case 0xf9: /* stc */
    ctxt.eflags |= X86_EFLAGS_CF;
    break;
    case 0xfc: /* cld */
    ctxt.eflags &= ~X86_EFLAGS_DF;
    break;
    case 0xfd: /* std */
    ctxt.eflags |= X86_EFLAGS_DF;
    break;
    default:
    goto cannot_emulate;
    }
    if (rc != X86EMUL_CONTINUE)
    goto done;
    writeback:
    if (ctxt.d & SrcWrite) {
    BUG_ON(ctxt.src.type == OP_MEM || ctxt.src.type == OP_MEM_STR);
    rc = writeback(ctxt, &ctxt.src);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    }
    if (!(ctxt.d & NoWrite)) {
    rc = writeback(ctxt, &ctxt.dst);
    if (rc != X86EMUL_CONTINUE)
    goto done;
    }
//
// restore dst type in case the decoding will be reused
// (happens for string instruction )
//
    ctxt.dst.type = saved_dst_type;
    if ((ctxt.d & SrcMask) == SrcSI)
    string_addr_inc(ctxt, VCPU_REGS_RSI, &ctxt.src);
    if ((ctxt.d & DstMask) == DstDI)
    string_addr_inc(ctxt, VCPU_REGS_RDI, &ctxt.dst);
    if (ctxt.rep_prefix && (ctxt.d & String)) {
    unsigned int count;
    struct read_cache *r = &ctxt.io_read;
    if ((ctxt.d & SrcMask) == SrcSI)
    count = ctxt.src.count;
    else
    count = ctxt.dst.count;
    register_address_increment(ctxt, VCPU_REGS_RCX, -count);
    if (!string_insn_completed(ctxt)) {
//
// Re-enter guest when pio read ahead buffer is empty
// or, if it is not used, after each 1024 iteration.
//
    if ((r.end != 0 || reg_read(ctxt, VCPU_REGS_RCX) & 0x3ff) &&
    (r.end == 0 || r.end != r.pos)) {
//
// Reset read cache. Usually happens before
// decode, but since instruction is restarted
// we have to do it here.
//
    ctxt.mem_read.end = 0;
    writeback_registers(ctxt);
    return EMULATION_RESTART;
    }
    goto done; /* skip rip writeback */
    }
    ctxt.eflags &= ~X86_EFLAGS_RF;
    }
    ctxt.eip = ctxt._eip;
    if (ctxt.mode != X86EMUL_MODE_PROT64)
    ctxt.eip = (u32)ctxt._eip;
    done:
    if (rc == X86EMUL_PROPAGATE_FAULT) {
    if (KVM_EMULATOR_BUG_ON(ctxt.exception.vector > 0x1f, ctxt))
    return EMULATION_FAILED;
    ctxt.have_exception = true;
    }
    if (rc == X86EMUL_INTERCEPTED)
    return EMULATION_INTERCEPTED;
    if (rc == X86EMUL_CONTINUE)
    writeback_registers(ctxt);
    return (rc == X86EMUL_UNHANDLEABLE) ? EMULATION_FAILED : EMULATION_OK;
    twobyte_insn:
    switch (ctxt.b) {
    case 0x09:		/* wbinvd */
    (ctxt.ops.wbinvd)(ctxt);
    break;
    case 0x08:		/* invd */
    case 0x0d:		/* GrpP (prefetch) */
    case 0x18:		/* Grp16 (prefetch/nop) */
    case 0x1f:		/* nop */
    break;
    case 0x20: /* mov cr, reg */
    ctxt.dst.val = ops.get_cr(ctxt, ctxt.modrm_reg);
    break;
    case 0x21: /* mov from dr to reg */
    ctxt.dst.val = ops.get_dr(ctxt, ctxt.modrm_reg);
    break;
    case 0x40 ... 0x4f:	/* cmov */
    if (test_cc(ctxt.b, ctxt.eflags))
    ctxt.dst.val = ctxt.src.val;
#[no_mangle]
pub unsafe extern "C" fn if(4: ctxt->op_bytes !=) -> else {
    else if (ctxt.op_bytes != 4)
    ctxt.dst.type = OP_NONE; /* no writeback */
    break;
    case 0x80 ... 0x8f: /* jnz rel, etc*/
    if (test_cc(ctxt.b, ctxt.eflags))
    rc = jmp_rel(ctxt, ctxt.src.val);
    break;
    case 0x90 ... 0x9f:     /* setcc r/m8 */
    ctxt.dst.val = test_cc(ctxt.b, ctxt.eflags);
    break;
    case 0xb6 ... 0xb7:	/* movzx */
    ctxt.dst.bytes = ctxt.op_bytes;
    ctxt.dst.val = (ctxt.src.bytes == 1) ? (u8) ctxt.src.val
    : (u16) ctxt.src.val;
    break;
    case 0xbe ... 0xbf:	/* movsx */
    ctxt.dst.bytes = ctxt.op_bytes;
    ctxt.dst.val = (ctxt.src.bytes == 1) ? (s8) ctxt.src.val :
    (s16) ctxt.src.val;
    break;
    default:
    goto cannot_emulate;
    }
    threebyte_insn:
    if (rc != X86EMUL_CONTINUE)
    goto done;
    goto writeback;
    cannot_emulate:
    return EMULATION_FAILED;
    }
#[no_mangle]
pub unsafe extern "C" fn emulator_invalidate_register_cache(ctxt: *mut x86_emulate_ctxt) {
    void emulator_invalidate_register_cache(struct x86_emulate_ctxt *ctxt)
    {
    invalidate_registers(ctxt);
    }
#[no_mangle]
pub unsafe extern "C" fn emulator_writeback_register_cache(ctxt: *mut x86_emulate_ctxt) {
    void emulator_writeback_register_cache(struct x86_emulate_ctxt *ctxt)
    {
    writeback_registers(ctxt);
    }
#[no_mangle]
pub unsafe extern "C" fn emulator_can_use_gpa(ctxt: *mut x86_emulate_ctxt) -> bool {
    bool emulator_can_use_gpa(struct x86_emulate_ctxt *ctxt)
    {
    if (ctxt.rep_prefix && (ctxt.d & String))
    return false;
    if (ctxt.d & TwoMemOp)
    return false;
    return true;
    }
