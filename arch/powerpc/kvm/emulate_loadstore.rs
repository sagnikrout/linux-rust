//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/emulate_loadstore.c
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
// Copyright IBM Corp. 2007
// Copyright 2011 Freescale Semiconductor, Inc.
//
// Authors: Hollis Blanchard <hollisb@us.ibm.com>
//

#[no_mangle]
unsafe extern "C" fn kvmppc_check_fp_disabled(vcpu: *mut kvm_vcpu) -> bool {
    static bool kvmppc_check_fp_disabled(struct kvm_vcpu *vcpu)
    {
    if (!(kvmppc_get_msr(vcpu) & MSR_FP)) {
    kvmppc_core_queue_fpunavail(vcpu, kvmppc_get_msr(vcpu) & SRR1_PREFIXED);
    return true;
    }
    return false;
    }

#[no_mangle]
unsafe extern "C" fn kvmppc_check_vsx_disabled(vcpu: *mut kvm_vcpu) -> bool {
    static bool kvmppc_check_vsx_disabled(struct kvm_vcpu *vcpu)
    {
    if (!(kvmppc_get_msr(vcpu) & MSR_VSX)) {
    kvmppc_core_queue_vsx_unavail(vcpu, kvmppc_get_msr(vcpu) & SRR1_PREFIXED);
    return true;
    }
    return false;
    }

#[no_mangle]
unsafe extern "C" fn kvmppc_check_altivec_disabled(vcpu: *mut kvm_vcpu) -> bool {
    static bool kvmppc_check_altivec_disabled(struct kvm_vcpu *vcpu)
    {
    if (!(kvmppc_get_msr(vcpu) & MSR_VEC)) {
    kvmppc_core_queue_vec_unavail(vcpu, kvmppc_get_msr(vcpu) & SRR1_PREFIXED);
    return true;
    }
    return false;
    }

//
// XXX to do:
// lfiwax, lfiwzx
// vector loads and stores
//
// Instructions that trap when used on cache-inhibited mappings
// are not emulated here: multiple and string instructions,
// lq/stq, and the load-reserve/store-conditional instructions.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_emulate_loadstore(vcpu: *mut kvm_vcpu) -> c_int {
    int kvmppc_emulate_loadstore(struct kvm_vcpu *vcpu)
    {
    ppc_inst_t inst;
    let mut emulated: enum emulation_result = EMULATE_FAIL;
    struct instruction_op op;
// this default type might be overwritten by subcategories
    kvmppc_set_exit_type(vcpu, EMULATED_INST_EXITS);
    emulated = kvmppc_get_last_inst(vcpu, INST_GENERIC, &inst);
    if (emulated != EMULATE_DONE)
    return emulated;
    vcpu.arch.mmio_vsx_copy_nums = 0;
    vcpu.arch.mmio_vsx_offset = 0;
    vcpu.arch.mmio_copy_type = KVMPPC_VSX_COPY_NONE;
    vcpu.arch.mmio_sp64_extend = 0;
    vcpu.arch.mmio_sign_extend = 0;
    vcpu.arch.mmio_vmx_copy_nums = 0;
    vcpu.arch.mmio_vmx_offset = 0;
    vcpu.arch.mmio_host_swabbed = 0;
    emulated = EMULATE_FAIL;
    vcpu.arch.regs.msr = kvmppc_get_msr(vcpu);
    if (analyse_instr(&op, &vcpu.arch.regs, inst) == 0) {
    let mut type: c_int = op.type & INSTR_TYPE_MASK;
    let mut size: c_int = GETSIZE(op.type);
    vcpu.mmio_is_write = OP_IS_STORE(type);
    switch (type) {
    case LOAD:  {
    let mut instr_byte_swap: c_int = op.type & BYTEREV;
    if (op.type & SIGNEXT)
    emulated = kvmppc_handle_loads(vcpu,
    op.reg, size, !instr_byte_swap);
    else
    emulated = kvmppc_handle_load(vcpu,
    op.reg, size, !instr_byte_swap);
    if ((op.type & UPDATE) && (emulated != EMULATE_FAIL))
    kvmppc_set_gpr(vcpu, op.update_reg, vcpu.arch.vaddr_accessed);
    break;
    }

    case LOAD_FP:
    if (kvmppc_check_fp_disabled(vcpu))
    return EMULATE_DONE;
    if (op.type & FPCONV)
    vcpu.arch.mmio_sp64_extend = 1;
    if (op.type & SIGNEXT)
    emulated = kvmppc_handle_loads(vcpu,
    KVM_MMIO_REG_FPR|op.reg, size, 1);
    else
    emulated = kvmppc_handle_load(vcpu,
    KVM_MMIO_REG_FPR|op.reg, size, 1);
    if ((op.type & UPDATE) && (emulated != EMULATE_FAIL))
    kvmppc_set_gpr(vcpu, op.update_reg, vcpu.arch.vaddr_accessed);
    break;

    case LOAD_VMX:
    if (kvmppc_check_altivec_disabled(vcpu))
    return EMULATE_DONE;
// Hardware enforces alignment of VMX accesses
    vcpu.arch.vaddr_accessed &= ~((unsigned long)size - 1);
    vcpu.arch.paddr_accessed &= ~((unsigned long)size - 1);
    if (size == 16) { /* lvx */
    vcpu.arch.mmio_copy_type =
    KVMPPC_VMX_COPY_DWORD;
    } else if (size == 4) { /* lvewx  */
    vcpu.arch.mmio_copy_type =
    KVMPPC_VMX_COPY_WORD;
    } else if (size == 2) { /* lvehx  */
    vcpu.arch.mmio_copy_type =
    KVMPPC_VMX_COPY_HWORD;
    } else if (size == 1) { /* lvebx  */
    vcpu.arch.mmio_copy_type =
    KVMPPC_VMX_COPY_BYTE;
    } else
    break;
    vcpu.arch.mmio_vmx_offset =
    (vcpu.arch.vaddr_accessed & 0xf)/size;
    if (size == 16) {
    vcpu.arch.mmio_vmx_copy_nums = 2;
    emulated = kvmppc_handle_vmx_load(vcpu,
    KVM_MMIO_REG_VMX|op.reg,
    8, 1);
    } else {
    vcpu.arch.mmio_vmx_copy_nums = 1;
    emulated = kvmppc_handle_vmx_load(vcpu,
    KVM_MMIO_REG_VMX|op.reg,
    size, 1);
    }
    break;

    case LOAD_VSX: {
    int io_size_each;
    if (op.vsx_flags & VSX_CHECK_VEC) {
    if (kvmppc_check_altivec_disabled(vcpu))
    return EMULATE_DONE;
    } else {
    if (kvmppc_check_vsx_disabled(vcpu))
    return EMULATE_DONE;
    }
    if (op.vsx_flags & VSX_FPCONV)
    vcpu.arch.mmio_sp64_extend = 1;
    if (op.element_size == 8)  {
    if (op.vsx_flags & VSX_SPLAT)
    vcpu.arch.mmio_copy_type =
    KVMPPC_VSX_COPY_DWORD_LOAD_DUMP;
    else
    vcpu.arch.mmio_copy_type =
    KVMPPC_VSX_COPY_DWORD;
    } else if (op.element_size == 4) {
    if (op.vsx_flags & VSX_SPLAT)
    vcpu.arch.mmio_copy_type =
    KVMPPC_VSX_COPY_WORD_LOAD_DUMP;
    else
    vcpu.arch.mmio_copy_type =
    KVMPPC_VSX_COPY_WORD;
    } else
    break;
    if (size < op.element_size) {
// precision convert case: lxsspx, etc
    vcpu.arch.mmio_vsx_copy_nums = 1;
    io_size_each = size;
    } else { /* lxvw4x, lxvd2x, etc */
    vcpu.arch.mmio_vsx_copy_nums =
    size/op.element_size;
    io_size_each = op.element_size;
    }
    emulated = kvmppc_handle_vsx_load(vcpu,
    KVM_MMIO_REG_VSX|op.reg, io_size_each,
    1, op.type & SIGNEXT);
    break;
    }

    case STORE: {
    let mut instr_byte_swap: c_int = op.type & BYTEREV;
    emulated = kvmppc_handle_store(vcpu, kvmppc_get_gpr(vcpu, op.reg),
    size, !instr_byte_swap);
    if ((op.type & UPDATE) && (emulated != EMULATE_FAIL))
    kvmppc_set_gpr(vcpu, op.update_reg, vcpu.arch.vaddr_accessed);
    break;
    }

    case STORE_FP:
    if (kvmppc_check_fp_disabled(vcpu))
    return EMULATE_DONE;
// The FP registers need to be flushed so that
// kvmppc_handle_store() can read actual FP vals
// from vcpu->arch.
//
    if (vcpu.kvm.arch.kvm_ops.giveup_ext)
    vcpu.kvm.arch.kvm_ops.giveup_ext(vcpu,
    MSR_FP);
    if (op.type & FPCONV)
    vcpu.arch.mmio_sp64_extend = 1;
    emulated = kvmppc_handle_store(vcpu,
    kvmppc_get_fpr(vcpu, op.reg), size, 1);
    if ((op.type & UPDATE) && (emulated != EMULATE_FAIL))
    kvmppc_set_gpr(vcpu, op.update_reg, vcpu.arch.vaddr_accessed);
    break;

    case STORE_VMX:
    if (kvmppc_check_altivec_disabled(vcpu))
    return EMULATE_DONE;
// Hardware enforces alignment of VMX accesses.
    vcpu.arch.vaddr_accessed &= ~((unsigned long)size - 1);
    vcpu.arch.paddr_accessed &= ~((unsigned long)size - 1);
    if (vcpu.kvm.arch.kvm_ops.giveup_ext)
    vcpu.kvm.arch.kvm_ops.giveup_ext(vcpu,
    MSR_VEC);
    if (size == 16) { /* stvx */
    vcpu.arch.mmio_copy_type =
    KVMPPC_VMX_COPY_DWORD;
    } else if (size == 4) { /* stvewx  */
    vcpu.arch.mmio_copy_type =
    KVMPPC_VMX_COPY_WORD;
    } else if (size == 2) { /* stvehx  */
    vcpu.arch.mmio_copy_type =
    KVMPPC_VMX_COPY_HWORD;
    } else if (size == 1) { /* stvebx  */
    vcpu.arch.mmio_copy_type =
    KVMPPC_VMX_COPY_BYTE;
    } else
    break;
    vcpu.arch.mmio_vmx_offset =
    (vcpu.arch.vaddr_accessed & 0xf)/size;
    if (size == 16) {
    vcpu.arch.mmio_vmx_copy_nums = 2;
    emulated = kvmppc_handle_vmx_store(vcpu,
    op.reg, 8, 1);
    } else {
    vcpu.arch.mmio_vmx_copy_nums = 1;
    emulated = kvmppc_handle_vmx_store(vcpu,
    op.reg, size, 1);
    }
    break;

    case STORE_VSX: {
    int io_size_each;
    if (op.vsx_flags & VSX_CHECK_VEC) {
    if (kvmppc_check_altivec_disabled(vcpu))
    return EMULATE_DONE;
    } else {
    if (kvmppc_check_vsx_disabled(vcpu))
    return EMULATE_DONE;
    }
    if (vcpu.kvm.arch.kvm_ops.giveup_ext)
    vcpu.kvm.arch.kvm_ops.giveup_ext(vcpu,
    MSR_VSX);
    if (op.vsx_flags & VSX_FPCONV)
    vcpu.arch.mmio_sp64_extend = 1;
    if (op.element_size == 8)
    vcpu.arch.mmio_copy_type =
    KVMPPC_VSX_COPY_DWORD;
#[no_mangle]
pub unsafe extern "C" fn if(4: op.element_size ==) -> else {
    else if (op.element_size == 4)
    vcpu.arch.mmio_copy_type =
    KVMPPC_VSX_COPY_WORD;
    else
    break;
    if (size < op.element_size) {
// precise conversion case, like stxsspx
    vcpu.arch.mmio_vsx_copy_nums = 1;
    io_size_each = size;
    } else { /* stxvw4x, stxvd2x, etc */
    vcpu.arch.mmio_vsx_copy_nums =
    size/op.element_size;
    io_size_each = op.element_size;
    }
    emulated = kvmppc_handle_vsx_store(vcpu,
    op.reg, io_size_each, 1);
    break;
    }

    case CACHEOP:
// Do nothing. The guest is performing dcbi because
// hardware DMA is not snooped by the dcache, but
// emulated DMA either goes through the dcache as
// normal writes, or the host kernel has handled dcache
// coherence.
//
    emulated = EMULATE_DONE;
    break;
    default:
    break;
    }
    }
    trace_kvm_ppc_instr(ppc_inst_val(inst), kvmppc_get_pc(vcpu), emulated);
// Advance past emulated instruction.
    if (emulated != EMULATE_FAIL)
    kvmppc_set_pc(vcpu, kvmppc_get_pc(vcpu) + ppc_inst_len(inst));
    return emulated;
    }
