//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_pr_papr.c
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
// Copyright (C) 2011. Freescale Inc. All rights reserved.
//
// Authors:
// Alexander Graf <agraf@suse.de>
// Paul Mackerras <paulus@samba.org>
//
// Description:
//
// Hypercall handling for running PAPR guests in PR KVM on Book 3S
// processors.
//

#[no_mangle]
unsafe extern "C" fn get_pteg_addr(vcpu: *mut kvm_vcpu, pte_index: c_long) -> c_ulong {
    static unsigned long get_pteg_addr(struct kvm_vcpu *vcpu, long pte_index)
    {
    struct kvmppc_vcpu_book3s *vcpu_book3s = to_book3s(vcpu);
    unsigned long pteg_addr;
    pte_index <<= 4;
    pte_index &= ((1 << ((vcpu_book3s.sdr1 & 0x1f) + 11)) - 1) << 7 | 0x70;
    pteg_addr = vcpu_book3s.sdr1 & 0xfffffffffffc0000ULL;
    pteg_addr |= pte_index;
    return pteg_addr;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_enter(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_enter(struct kvm_vcpu *vcpu)
    {
    let mut flags: c_long = kvmppc_get_gpr(vcpu, 4);
    let mut pte_index: c_long = kvmppc_get_gpr(vcpu, 5);
    __be64 pteg[2 * 8];
    __be64 *hpte;
    unsigned long pteg_addr, i;
    long int ret;
    i = pte_index & 7;
    pte_index &= ~7UL;
    pteg_addr = get_pteg_addr(vcpu, pte_index);
    mutex_lock(&vcpu.kvm.arch.hpt_mutex);
    ret = H_FUNCTION;
    if (copy_from_user(pteg, (void __user *)pteg_addr, sizeof(pteg)))
    goto done;
    hpte = pteg;
    ret = H_PTEG_FULL;
    if (likely((flags & H_EXACT) == 0)) {
    for (i = 0; ; ++i) {
    if (i == 8)
    goto done;
    if ((be64_to_cpu(*hpte) & HPTE_V_VALID) == 0)
    break;
    hpte += 2;
    }
    } else {
    hpte += i * 2;
    if (*hpte & HPTE_V_VALID)
    goto done;
    }
    hpte[0] = cpu_to_be64(kvmppc_get_gpr(vcpu, 6));
    hpte[1] = cpu_to_be64(kvmppc_get_gpr(vcpu, 7));
    pteg_addr += i * HPTE_SIZE;
    ret = H_FUNCTION;
    if (copy_to_user((void __user *)pteg_addr, hpte, HPTE_SIZE))
    goto done;
    kvmppc_set_gpr(vcpu, 4, pte_index | i);
    ret = H_SUCCESS;
    done:
    mutex_unlock(&vcpu.kvm.arch.hpt_mutex);
    kvmppc_set_gpr(vcpu, 3, ret);
    return EMULATE_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_remove(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_remove(struct kvm_vcpu *vcpu)
    {
    let mut flags: c_ulong = kvmppc_get_gpr(vcpu, 4);
    let mut pte_index: c_ulong = kvmppc_get_gpr(vcpu, 5);
    let mut avpn: c_ulong = kvmppc_get_gpr(vcpu, 6);
    let mut v: c_ulong = 0, pteg, rb;
    unsigned long pte[2];
    long int ret;
    pteg = get_pteg_addr(vcpu, pte_index);
    mutex_lock(&vcpu.kvm.arch.hpt_mutex);
    ret = H_FUNCTION;
    if (copy_from_user(pte, (void __user *)pteg, sizeof(pte)))
    goto done;
    pte[0] = be64_to_cpu(( __be64)pte[0]);
    pte[1] = be64_to_cpu(( __be64)pte[1]);
    ret = H_NOT_FOUND;
    if ((pte[0] & HPTE_V_VALID) == 0 ||
    ((flags & H_AVPN) && (pte[0] & ~0x7fUL) != avpn) ||
    ((flags & H_ANDCOND) && (pte[0] & avpn) != 0))
    goto done;
    ret = H_FUNCTION;
    if (copy_to_user((void __user *)pteg, &v, sizeof(v)))
    goto done;
    rb = compute_tlbie_rb(pte[0], pte[1], pte_index);
    vcpu.arch.mmu.tlbie(vcpu, rb, rb & 1 ? true : false);
    ret = H_SUCCESS;
    kvmppc_set_gpr(vcpu, 4, pte[0]);
    kvmppc_set_gpr(vcpu, 5, pte[1]);
    done:
    mutex_unlock(&vcpu.kvm.arch.hpt_mutex);
    kvmppc_set_gpr(vcpu, 3, ret);
    return EMULATE_DONE;
    }
// Request defs for kvmppc_h_pr_bulk_remove()
pub const H_BULK_REMOVE_TYPE: c_uint = 0xc000000000000000ULL;
pub const H_BULK_REMOVE_REQUEST: c_uint = 0x4000000000000000ULL;
pub const H_BULK_REMOVE_RESPONSE: c_uint = 0x8000000000000000ULL;
pub const H_BULK_REMOVE_END: c_uint = 0xc000000000000000ULL;
pub const H_BULK_REMOVE_CODE: c_uint = 0x3000000000000000ULL;
pub const H_BULK_REMOVE_SUCCESS: c_uint = 0x0000000000000000ULL;
pub const H_BULK_REMOVE_NOT_FOUND: c_uint = 0x1000000000000000ULL;
pub const H_BULK_REMOVE_PARM: c_uint = 0x2000000000000000ULL;
pub const H_BULK_REMOVE_HW: c_uint = 0x3000000000000000ULL;
pub const H_BULK_REMOVE_RC: c_uint = 0x0c00000000000000ULL;
pub const H_BULK_REMOVE_FLAGS: c_uint = 0x0300000000000000ULL;
pub const H_BULK_REMOVE_ABSOLUTE: c_uint = 0x0000000000000000ULL;
pub const H_BULK_REMOVE_ANDCOND: c_uint = 0x0100000000000000ULL;
pub const H_BULK_REMOVE_AVPN: c_uint = 0x0200000000000000ULL;
pub const H_BULK_REMOVE_PTEX: c_uint = 0x00ffffffffffffffULL;
pub const H_BULK_REMOVE_MAX_BATCH: c_int = 4;
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_bulk_remove(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_bulk_remove(struct kvm_vcpu *vcpu)
    {
    int i;
    let mut paramnr: c_int = 4;
    let mut ret: c_int = H_SUCCESS;
    mutex_lock(&vcpu.kvm.arch.hpt_mutex);
    for (i = 0; i < H_BULK_REMOVE_MAX_BATCH; i++) {
    let mut tsh: c_ulong = kvmppc_get_gpr(vcpu, paramnr+(2*i));
    let mut tsl: c_ulong = kvmppc_get_gpr(vcpu, paramnr+(2*i)+1);
    unsigned long pteg, rb, flags;
    unsigned long pte[2];
    let mut v: c_ulong = 0;
    if ((tsh & H_BULK_REMOVE_TYPE) == H_BULK_REMOVE_END) {
    break; /* Exit success */
    } else if ((tsh & H_BULK_REMOVE_TYPE) !=
    H_BULK_REMOVE_REQUEST) {
    ret = H_PARAMETER;
    break; /* Exit fail */
    }
    tsh &= H_BULK_REMOVE_PTEX | H_BULK_REMOVE_FLAGS;
    tsh |= H_BULK_REMOVE_RESPONSE;
    if ((tsh & H_BULK_REMOVE_ANDCOND) &&
    (tsh & H_BULK_REMOVE_AVPN)) {
    tsh |= H_BULK_REMOVE_PARM;
    kvmppc_set_gpr(vcpu, paramnr+(2*i), tsh);
    ret = H_PARAMETER;
    break; /* Exit fail */
    }
    pteg = get_pteg_addr(vcpu, tsh & H_BULK_REMOVE_PTEX);
    if (copy_from_user(pte, (void __user *)pteg, sizeof(pte))) {
    ret = H_FUNCTION;
    break;
    }
    pte[0] = be64_to_cpu(( __be64)pte[0]);
    pte[1] = be64_to_cpu(( __be64)pte[1]);
// tsl = AVPN
    flags = (tsh & H_BULK_REMOVE_FLAGS) >> 26;
    if ((pte[0] & HPTE_V_VALID) == 0 ||
    ((flags & H_AVPN) && (pte[0] & ~0x7fUL) != tsl) ||
    ((flags & H_ANDCOND) && (pte[0] & tsl) != 0)) {
    tsh |= H_BULK_REMOVE_NOT_FOUND;
    } else {
// Splat the pteg in (userland) hpt
    if (copy_to_user((void __user *)pteg, &v, sizeof(v))) {
    ret = H_FUNCTION;
    break;
    }
    rb = compute_tlbie_rb(pte[0], pte[1],
    tsh & H_BULK_REMOVE_PTEX);
    vcpu.arch.mmu.tlbie(vcpu, rb, rb & 1 ? true : false);
    tsh |= H_BULK_REMOVE_SUCCESS;
    tsh |= (pte[1] & (HPTE_R_C | HPTE_R_R)) << 43;
    }
    kvmppc_set_gpr(vcpu, paramnr+(2*i), tsh);
    }
    mutex_unlock(&vcpu.kvm.arch.hpt_mutex);
    kvmppc_set_gpr(vcpu, 3, ret);
    return EMULATE_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_protect(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_protect(struct kvm_vcpu *vcpu)
    {
    let mut flags: c_ulong = kvmppc_get_gpr(vcpu, 4);
    let mut pte_index: c_ulong = kvmppc_get_gpr(vcpu, 5);
    let mut avpn: c_ulong = kvmppc_get_gpr(vcpu, 6);
    unsigned long rb, pteg, r, v;
    unsigned long pte[2];
    long int ret;
    pteg = get_pteg_addr(vcpu, pte_index);
    mutex_lock(&vcpu.kvm.arch.hpt_mutex);
    ret = H_FUNCTION;
    if (copy_from_user(pte, (void __user *)pteg, sizeof(pte)))
    goto done;
    pte[0] = be64_to_cpu(( __be64)pte[0]);
    pte[1] = be64_to_cpu(( __be64)pte[1]);
    ret = H_NOT_FOUND;
    if ((pte[0] & HPTE_V_VALID) == 0 ||
    ((flags & H_AVPN) && (pte[0] & ~0x7fUL) != avpn))
    goto done;
    v = pte[0];
    r = pte[1];
    r &= ~(HPTE_R_PP0 | HPTE_R_PP | HPTE_R_N | HPTE_R_KEY_HI |
    HPTE_R_KEY_LO);
    r |= (flags << 55) & HPTE_R_PP0;
    r |= (flags << 48) & HPTE_R_KEY_HI;
    r |= flags & (HPTE_R_PP | HPTE_R_N | HPTE_R_KEY_LO);
    pte[1] = r;
    rb = compute_tlbie_rb(v, r, pte_index);
    vcpu.arch.mmu.tlbie(vcpu, rb, rb & 1 ? true : false);
    pte[0] = ( u64)cpu_to_be64(pte[0]);
    pte[1] = ( u64)cpu_to_be64(pte[1]);
    ret = H_FUNCTION;
    if (copy_to_user((void __user *)pteg, pte, sizeof(pte)))
    goto done;
    ret = H_SUCCESS;
    done:
    mutex_unlock(&vcpu.kvm.arch.hpt_mutex);
    kvmppc_set_gpr(vcpu, 3, ret);
    return EMULATE_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_logical_ci_load(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_logical_ci_load(struct kvm_vcpu *vcpu)
    {
    long rc;
    rc = kvmppc_h_logical_ci_load(vcpu);
    if (rc == H_TOO_HARD)
    return EMULATE_FAIL;
    kvmppc_set_gpr(vcpu, 3, rc);
    return EMULATE_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_logical_ci_store(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_logical_ci_store(struct kvm_vcpu *vcpu)
    {
    long rc;
    rc = kvmppc_h_logical_ci_store(vcpu);
    if (rc == H_TOO_HARD)
    return EMULATE_FAIL;
    kvmppc_set_gpr(vcpu, 3, rc);
    return EMULATE_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_set_mode(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_set_mode(struct kvm_vcpu *vcpu)
    {
    let mut mflags: c_ulong = kvmppc_get_gpr(vcpu, 4);
    let mut resource: c_ulong = kvmppc_get_gpr(vcpu, 5);
    if (resource == H_SET_MODE_RESOURCE_ADDR_TRANS_MODE) {
// KVM PR does not provide AIL!=0 to guests
    if (mflags == 0)
    kvmppc_set_gpr(vcpu, 3, H_SUCCESS);
    else
    kvmppc_set_gpr(vcpu, 3, H_UNSUPPORTED_FLAG_START - 63);
    return EMULATE_DONE;
    }
    return EMULATE_FAIL;
    }

#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_put_tce(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_put_tce(struct kvm_vcpu *vcpu)
    {
    let mut liobn: c_ulong = kvmppc_get_gpr(vcpu, 4);
    let mut ioba: c_ulong = kvmppc_get_gpr(vcpu, 5);
    let mut tce: c_ulong = kvmppc_get_gpr(vcpu, 6);
    long rc;
    rc = kvmppc_h_put_tce(vcpu, liobn, ioba, tce);
    if (rc == H_TOO_HARD)
    return EMULATE_FAIL;
    kvmppc_set_gpr(vcpu, 3, rc);
    return EMULATE_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_put_tce_indirect(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_put_tce_indirect(struct kvm_vcpu *vcpu)
    {
    let mut liobn: c_ulong = kvmppc_get_gpr(vcpu, 4);
    let mut ioba: c_ulong = kvmppc_get_gpr(vcpu, 5);
    let mut tce: c_ulong = kvmppc_get_gpr(vcpu, 6);
    let mut npages: c_ulong = kvmppc_get_gpr(vcpu, 7);
    long rc;
    rc = kvmppc_h_put_tce_indirect(vcpu, liobn, ioba,
    tce, npages);
    if (rc == H_TOO_HARD)
    return EMULATE_FAIL;
    kvmppc_set_gpr(vcpu, 3, rc);
    return EMULATE_DONE;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_stuff_tce(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_stuff_tce(struct kvm_vcpu *vcpu)
    {
    let mut liobn: c_ulong = kvmppc_get_gpr(vcpu, 4);
    let mut ioba: c_ulong = kvmppc_get_gpr(vcpu, 5);
    let mut tce_value: c_ulong = kvmppc_get_gpr(vcpu, 6);
    let mut npages: c_ulong = kvmppc_get_gpr(vcpu, 7);
    long rc;
    rc = kvmppc_h_stuff_tce(vcpu, liobn, ioba, tce_value, npages);
    if (rc == H_TOO_HARD)
    return EMULATE_FAIL;
    kvmppc_set_gpr(vcpu, 3, rc);
    return EMULATE_DONE;
    }

#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_put_tce(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_put_tce(struct kvm_vcpu *vcpu)
    {
    return EMULATE_FAIL;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_put_tce_indirect(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_put_tce_indirect(struct kvm_vcpu *vcpu)
    {
    return EMULATE_FAIL;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_stuff_tce(vcpu: *mut kvm_vcpu) -> c_int {
    static int kvmppc_h_pr_stuff_tce(struct kvm_vcpu *vcpu)
    {
    return EMULATE_FAIL;
    }

#[no_mangle]
unsafe extern "C" fn kvmppc_h_pr_xics_hcall(vcpu: *mut kvm_vcpu, cmd: u32) -> c_int {
    static int kvmppc_h_pr_xics_hcall(struct kvm_vcpu *vcpu, u32 cmd)
    {
    let mut rc: c_long = kvmppc_xics_hcall(vcpu, cmd);
    kvmppc_set_gpr(vcpu, 3, rc);
    return EMULATE_DONE;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_h_pr(vcpu: *mut kvm_vcpu, cmd: c_ulong) -> c_int {
    int kvmppc_h_pr(struct kvm_vcpu *vcpu, unsigned long cmd)
    {
    int rc, idx;
    if (cmd <= MAX_HCALL_OPCODE &&
    !test_bit(cmd/4, vcpu.kvm.arch.enabled_hcalls))
    return EMULATE_FAIL;
    switch (cmd) {
    case H_ENTER:
    return kvmppc_h_pr_enter(vcpu);
    case H_REMOVE:
    return kvmppc_h_pr_remove(vcpu);
    case H_PROTECT:
    return kvmppc_h_pr_protect(vcpu);
    case H_BULK_REMOVE:
    return kvmppc_h_pr_bulk_remove(vcpu);
    case H_PUT_TCE:
    return kvmppc_h_pr_put_tce(vcpu);
    case H_PUT_TCE_INDIRECT:
    return kvmppc_h_pr_put_tce_indirect(vcpu);
    case H_STUFF_TCE:
    return kvmppc_h_pr_stuff_tce(vcpu);
    case H_CEDE:
    kvmppc_set_msr_fast(vcpu, kvmppc_get_msr(vcpu) | MSR_EE);
    kvm_vcpu_halt(vcpu);
    vcpu.stat.generic.halt_wakeup++;
    return EMULATE_DONE;
    case H_LOGICAL_CI_LOAD:
    return kvmppc_h_pr_logical_ci_load(vcpu);
    case H_LOGICAL_CI_STORE:
    return kvmppc_h_pr_logical_ci_store(vcpu);
    case H_SET_MODE:
    return kvmppc_h_pr_set_mode(vcpu);
    case H_XIRR:
    case H_CPPR:
    case H_EOI:
    case H_IPI:
    case H_IPOLL:
    case H_XIRR_X:
    if (kvmppc_xics_enabled(vcpu))
    return kvmppc_h_pr_xics_hcall(vcpu, cmd);
    break;
    case H_RTAS:
    if (list_empty(&vcpu.kvm.arch.rtas_tokens))
    break;
    idx = srcu_read_lock(&vcpu.kvm.srcu);
    rc = kvmppc_rtas_hcall(vcpu);
    srcu_read_unlock(&vcpu.kvm.srcu, idx);
    if (rc)
    break;
    kvmppc_set_gpr(vcpu, 3, 0);
    return EMULATE_DONE;
    }
    return EMULATE_FAIL;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_hcall_impl_pr(cmd: c_ulong) -> c_int {
    int kvmppc_hcall_impl_pr(unsigned long cmd)
    {
    switch (cmd) {
    case H_ENTER:
    case H_REMOVE:
    case H_PROTECT:
    case H_BULK_REMOVE:

    case H_GET_TCE:
    case H_PUT_TCE:
    case H_PUT_TCE_INDIRECT:
    case H_STUFF_TCE:

    case H_CEDE:
    case H_LOGICAL_CI_LOAD:
    case H_LOGICAL_CI_STORE:
    case H_SET_MODE:

    case H_XIRR:
    case H_CPPR:
    case H_EOI:
    case H_IPI:
    case H_IPOLL:
    case H_XIRR_X:

    return 1;
    }
    return 0;
    }
//
// List of hcall numbers to enable by default.
// For compatibility with old userspace, we enable by default
// all hcalls that were implemented before the hcall-enabling
// facility was added.  Note this list should not include H_RTAS.
//
    static unsigned int default_hcall_list[] = {
    H_ENTER,
    H_REMOVE,
    H_PROTECT,
    H_BULK_REMOVE,

    H_GET_TCE,
    H_PUT_TCE,

    H_CEDE,
    H_SET_MODE,

    H_XIRR,
    H_CPPR,
    H_EOI,
    H_IPI,
    H_IPOLL,
    H_XIRR_X,

    0
    };
#[no_mangle]
pub unsafe extern "C" fn kvmppc_pr_init_default_hcalls(kvm: *mut kvm) {
    void kvmppc_pr_init_default_hcalls(struct kvm *kvm)
    {
    int i;
    unsigned int hcall;
    for (i = 0; default_hcall_list[i]; ++i) {
    hcall = default_hcall_list[i];
    WARN_ON(!kvmppc_hcall_impl_pr(hcall));
    __set_bit(hcall / 4, kvm.arch.enabled_hcalls);
    }
    }
