//! Automatically rewritten from C to Rust
//! Source: arch/s390/kvm/s390/sigp.c
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
// handling interprocessor communication
//
// Copyright IBM Corp. 2008, 2013
//
// Author(s): Carsten Otte <cotte@de.ibm.com>
// Christian Borntraeger <borntraeger@de.ibm.com>
// Christian Ehrhardt <ehrhardt@de.ibm.com>
//

    static int __sigp_sense(struct kvm_vcpu *vcpu, struct kvm_vcpu *dst_vcpu,
    u64 *reg)
    {
    let mut stopped: bool = kvm_s390_test_cpuflags(dst_vcpu, CPUSTAT_STOPPED);
    int rc;
    int ext_call_pending;
    ext_call_pending = kvm_s390_ext_call_pending(dst_vcpu);
    if (!stopped && !ext_call_pending)
    rc = SIGP_CC_ORDER_CODE_ACCEPTED;
    else {
// reg &= 0xffffffff00000000UL;
    if (ext_call_pending)
// reg |= SIGP_STATUS_EXT_CALL_PENDING;
    if (stopped)
// reg |= SIGP_STATUS_STOPPED;
    rc = SIGP_CC_STATUS_STORED;
    }
    VCPU_EVENT(vcpu, 4, "sensed status of cpu %x rc %x", dst_vcpu.vcpu_id,
    rc);
    return rc;
    }
    static int __inject_sigp_emergency(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu)
    {
    struct kvm_s390_irq irq = {
    .type = KVM_S390_INT_EMERGENCY,
    .u.emerg.code = vcpu.vcpu_id,
    };
    let mut rc: c_int = 0;
    rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if (!rc)
    VCPU_EVENT(vcpu, 4, "sent sigp emerg to cpu %x",
    dst_vcpu.vcpu_id);
    return rc ? rc : SIGP_CC_ORDER_CODE_ACCEPTED;
    }
#[no_mangle]
unsafe extern "C" fn __sigp_emergency(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu) -> c_int {
    static int __sigp_emergency(struct kvm_vcpu *vcpu, struct kvm_vcpu *dst_vcpu)
    {
    return __inject_sigp_emergency(vcpu, dst_vcpu);
    }
    static int __sigp_conditional_emergency(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu,
    u16 asn, u64 *reg)
    {
    let mut psw_int_mask: u64 = PSW_MASK_IO | PSW_MASK_EXT;
    u16 p_asn, s_asn;
    psw_t *psw;
    bool idle;
    idle = is_vcpu_idle(vcpu);
    psw = &dst_vcpu.arch.sie_block.gpsw;
    p_asn = dst_vcpu.arch.sie_block.gcr[4] & 0xffff;  /* Primary ASN */
    s_asn = dst_vcpu.arch.sie_block.gcr[3] & 0xffff;  /* Secondary ASN */
// Inject the emergency signal?
    if (!is_vcpu_stopped(vcpu)
    || (psw.mask & psw_int_mask) != psw_int_mask
    || (idle && psw.addr != 0)
    || (!idle && (asn == p_asn || asn == s_asn))) {
    return __inject_sigp_emergency(vcpu, dst_vcpu);
    } else {
// reg &= 0xffffffff00000000UL;
// reg |= SIGP_STATUS_INCORRECT_STATE;
    return SIGP_CC_STATUS_STORED;
    }
    }
    static int __sigp_external_call(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu, u64 *reg)
    {
    struct kvm_s390_irq irq = {
    .type = KVM_S390_INT_EXTERNAL_CALL,
    .u.extcall.code = vcpu.vcpu_id,
    };
    int rc;
    rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if (rc == -EBUSY) {
// reg &= 0xffffffff00000000UL;
// reg |= SIGP_STATUS_EXT_CALL_PENDING;
    return SIGP_CC_STATUS_STORED;
    } else if (rc == 0) {
    VCPU_EVENT(vcpu, 4, "sent sigp ext call to cpu %x",
    dst_vcpu.vcpu_id);
    }
    return rc ? rc : SIGP_CC_ORDER_CODE_ACCEPTED;
    }
#[no_mangle]
unsafe extern "C" fn __sigp_stop(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu) -> c_int {
    static int __sigp_stop(struct kvm_vcpu *vcpu, struct kvm_vcpu *dst_vcpu)
    {
    struct kvm_s390_irq irq = {
    .type = KVM_S390_SIGP_STOP,
    };
    int rc;
    rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if (rc == -EBUSY)
    rc = SIGP_CC_BUSY;
#[no_mangle]
pub unsafe extern "C" fn if(0: rc ==) -> else {
    else if (rc == 0)
    VCPU_EVENT(vcpu, 4, "sent sigp stop to cpu %x",
    dst_vcpu.vcpu_id);
    return rc;
    }
    static int __sigp_stop_and_store_status(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu, u64 *reg)
    {
    struct kvm_s390_irq irq = {
    .type = KVM_S390_SIGP_STOP,
    .u.stop.flags = KVM_S390_STOP_FLAG_STORE_STATUS,
    };
    int rc;
    rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if (rc == -EBUSY)
    rc = SIGP_CC_BUSY;
#[no_mangle]
pub unsafe extern "C" fn if(0: rc ==) -> else {
    else if (rc == 0)
    VCPU_EVENT(vcpu, 4, "sent sigp stop and store status to cpu %x",
    dst_vcpu.vcpu_id);
    return rc;
    }
    static int __sigp_set_arch(struct kvm_vcpu *vcpu, u32 parameter,
    u64 *status_reg)
    {
// status_reg &= 0xffffffff00000000UL;
// Reject set arch order, with czam we're always in z/Arch mode.
// status_reg |= SIGP_STATUS_INVALID_PARAMETER;
    return SIGP_CC_STATUS_STORED;
    }
    static int __sigp_set_prefix(struct kvm_vcpu *vcpu, struct kvm_vcpu *dst_vcpu,
    u32 address, u64 *reg)
    {
    struct kvm_s390_irq irq = {
    .type = KVM_S390_SIGP_SET_PREFIX,
    .u.prefix.address = address & 0x7fffe000u,
    };
    int rc;
//
// Make sure the new value is valid memory. We only need to check the
// first page, since address is 8k aligned and memory pieces are always
// at least 1MB aligned and have at least a size of 1MB.
//
    if (!kvm_is_gpa_in_memslot(vcpu.kvm, irq.u.prefix.address)) {
// reg &= 0xffffffff00000000UL;
// reg |= SIGP_STATUS_INVALID_PARAMETER;
    return SIGP_CC_STATUS_STORED;
    }
    rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if (rc == -EBUSY) {
// reg &= 0xffffffff00000000UL;
// reg |= SIGP_STATUS_INCORRECT_STATE;
    return SIGP_CC_STATUS_STORED;
    }
    return rc;
    }
    static int __sigp_store_status_at_addr(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu,
    u32 addr, u64 *reg)
    {
    int rc;
    if (!kvm_s390_test_cpuflags(dst_vcpu, CPUSTAT_STOPPED)) {
// reg &= 0xffffffff00000000UL;
// reg |= SIGP_STATUS_INCORRECT_STATE;
    return SIGP_CC_STATUS_STORED;
    }
    addr &= 0x7ffffe00;
    rc = kvm_s390_store_status_unloaded(dst_vcpu, addr);
    if (rc == -EFAULT) {
// reg &= 0xffffffff00000000UL;
// reg |= SIGP_STATUS_INVALID_PARAMETER;
    rc = SIGP_CC_STATUS_STORED;
    }
    return rc;
    }
    static int __sigp_sense_running(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu, u64 *reg)
    {
    int rc;
    if (!test_kvm_facility(vcpu.kvm, 9)) {
// reg &= 0xffffffff00000000UL;
// reg |= SIGP_STATUS_INVALID_ORDER;
    return SIGP_CC_STATUS_STORED;
    }
    if (kvm_s390_test_cpuflags(dst_vcpu, CPUSTAT_RUNNING)) {
// running
    rc = SIGP_CC_ORDER_CODE_ACCEPTED;
    } else {
// not running
// reg &= 0xffffffff00000000UL;
// reg |= SIGP_STATUS_NOT_RUNNING;
    rc = SIGP_CC_STATUS_STORED;
    }
    VCPU_EVENT(vcpu, 4, "sensed running status of cpu %x rc %x",
    dst_vcpu.vcpu_id, rc);
    return rc;
    }
    static int __prepare_sigp_re_start(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu, u8 order_code)
    {
    struct kvm_s390_local_interrupt *li = &dst_vcpu.arch.local_int;
// handle (RE)START in user space
    let mut rc: c_int = -EOPNOTSUPP;
// make sure we don't race with STOP irq injection
    spin_lock(&li.lock);
    if (kvm_s390_is_stop_irq_pending(dst_vcpu))
    rc = SIGP_CC_BUSY;
    spin_unlock(&li.lock);
    return rc;
    }
    static int __prepare_sigp_cpu_reset(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu, u8 order_code)
    {
// handle (INITIAL) CPU RESET in user space
    return -EOPNOTSUPP;
    }
    static int __prepare_sigp_unknown(struct kvm_vcpu *vcpu,
    struct kvm_vcpu *dst_vcpu)
    {
// handle unknown orders in user space
    return -EOPNOTSUPP;
    }
    static int handle_sigp_dst(struct kvm_vcpu *vcpu, u8 order_code,
    u16 cpu_addr, u32 parameter, u64 *status_reg)
    {
    int rc;
    struct kvm_vcpu *dst_vcpu = kvm_get_vcpu_by_id(vcpu.kvm, cpu_addr);
    if (!dst_vcpu)
    return SIGP_CC_NOT_OPERATIONAL;
//
// SIGP RESTART, SIGP STOP, and SIGP STOP AND STORE STATUS orders
// are processed asynchronously. Until the affected VCPU finishes
// its work and calls back into KVM to clear the (RESTART or STOP)
// interrupt, we need to return any new non-reset orders "busy".
//
// This is important because a single VCPU could issue:
// 1) SIGP STOP $DESTINATION
// 2) SIGP SENSE $DESTINATION
//
// If the SIGP SENSE would not be rejected as "busy", it could
// return an incorrect answer as to whether the VCPU is STOPPED
// or OPERATING.
//
    if (order_code != SIGP_INITIAL_CPU_RESET &&
    order_code != SIGP_CPU_RESET) {
//
// Lockless check. Both SIGP STOP and SIGP (RE)START
// properly synchronize everything while processing
// their orders, while the guest cannot observe a
// difference when issuing other orders from two
// different VCPUs.
//
    if (kvm_s390_is_stop_irq_pending(dst_vcpu) ||
    kvm_s390_is_restart_irq_pending(dst_vcpu))
    return SIGP_CC_BUSY;
    }
    switch (order_code) {
    case SIGP_SENSE:
    vcpu.stat.instruction_sigp_sense++;
    rc = __sigp_sense(vcpu, dst_vcpu, status_reg);
    break;
    case SIGP_EXTERNAL_CALL:
    vcpu.stat.instruction_sigp_external_call++;
    rc = __sigp_external_call(vcpu, dst_vcpu, status_reg);
    break;
    case SIGP_EMERGENCY_SIGNAL:
    vcpu.stat.instruction_sigp_emergency++;
    rc = __sigp_emergency(vcpu, dst_vcpu);
    break;
    case SIGP_STOP:
    vcpu.stat.instruction_sigp_stop++;
    rc = __sigp_stop(vcpu, dst_vcpu);
    break;
    case SIGP_STOP_AND_STORE_STATUS:
    vcpu.stat.instruction_sigp_stop_store_status++;
    rc = __sigp_stop_and_store_status(vcpu, dst_vcpu, status_reg);
    break;
    case SIGP_STORE_STATUS_AT_ADDRESS:
    vcpu.stat.instruction_sigp_store_status++;
    rc = __sigp_store_status_at_addr(vcpu, dst_vcpu, parameter,
    status_reg);
    break;
    case SIGP_SET_PREFIX:
    vcpu.stat.instruction_sigp_prefix++;
    rc = __sigp_set_prefix(vcpu, dst_vcpu, parameter, status_reg);
    break;
    case SIGP_COND_EMERGENCY_SIGNAL:
    vcpu.stat.instruction_sigp_cond_emergency++;
    rc = __sigp_conditional_emergency(vcpu, dst_vcpu, parameter,
    status_reg);
    break;
    case SIGP_SENSE_RUNNING:
    vcpu.stat.instruction_sigp_sense_running++;
    rc = __sigp_sense_running(vcpu, dst_vcpu, status_reg);
    break;
    case SIGP_START:
    vcpu.stat.instruction_sigp_start++;
    rc = __prepare_sigp_re_start(vcpu, dst_vcpu, order_code);
    break;
    case SIGP_RESTART:
    vcpu.stat.instruction_sigp_restart++;
    rc = __prepare_sigp_re_start(vcpu, dst_vcpu, order_code);
    break;
    case SIGP_INITIAL_CPU_RESET:
    vcpu.stat.instruction_sigp_init_cpu_reset++;
    rc = __prepare_sigp_cpu_reset(vcpu, dst_vcpu, order_code);
    break;
    case SIGP_CPU_RESET:
    vcpu.stat.instruction_sigp_cpu_reset++;
    rc = __prepare_sigp_cpu_reset(vcpu, dst_vcpu, order_code);
    break;
    default:
    vcpu.stat.instruction_sigp_unknown++;
    rc = __prepare_sigp_unknown(vcpu, dst_vcpu);
    }
    if (rc == -EOPNOTSUPP)
    VCPU_EVENT(vcpu, 4,
    "sigp order %u . cpu %x: handled in user space",
    order_code, dst_vcpu.vcpu_id);
    return rc;
    }
    static int handle_sigp_order_in_user_space(struct kvm_vcpu *vcpu, u8 order_code,
    u16 cpu_addr)
    {
    if (!vcpu.kvm.arch.user_sigp)
    return 0;
    switch (order_code) {
    case SIGP_SENSE:
    case SIGP_EXTERNAL_CALL:
    case SIGP_EMERGENCY_SIGNAL:
    case SIGP_COND_EMERGENCY_SIGNAL:
    case SIGP_SENSE_RUNNING:
    return 0;
// update counters as we're directly dropping to user space
    case SIGP_STOP:
    vcpu.stat.instruction_sigp_stop++;
    break;
    case SIGP_STOP_AND_STORE_STATUS:
    vcpu.stat.instruction_sigp_stop_store_status++;
    break;
    case SIGP_STORE_STATUS_AT_ADDRESS:
    vcpu.stat.instruction_sigp_store_status++;
    break;
    case SIGP_STORE_ADDITIONAL_STATUS:
    vcpu.stat.instruction_sigp_store_adtl_status++;
    break;
    case SIGP_SET_PREFIX:
    vcpu.stat.instruction_sigp_prefix++;
    break;
    case SIGP_START:
    vcpu.stat.instruction_sigp_start++;
    break;
    case SIGP_RESTART:
    vcpu.stat.instruction_sigp_restart++;
    break;
    case SIGP_INITIAL_CPU_RESET:
    vcpu.stat.instruction_sigp_init_cpu_reset++;
    break;
    case SIGP_CPU_RESET:
    vcpu.stat.instruction_sigp_cpu_reset++;
    break;
    default:
    vcpu.stat.instruction_sigp_unknown++;
    }
    VCPU_EVENT(vcpu, 3, "SIGP: order %u for CPU %d handled in userspace",
    order_code, cpu_addr);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_s390_handle_sigp(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_s390_handle_sigp(struct kvm_vcpu *vcpu)
    {
    let mut r1: c_int = (vcpu.arch.sie_block.ipa & 0x00f0) >> 4;
    let mut r3: c_int = vcpu.arch.sie_block.ipa & 0x000f;
    u32 parameter;
    let mut cpu_addr: u16 = vcpu.run.s.regs.gprs[r3];
    u8 order_code;
    int rc;
// sigp in userspace can exit
    if (vcpu.arch.sie_block.gpsw.mask & PSW_MASK_PSTATE)
    return kvm_s390_inject_program_int(vcpu, PGM_PRIVILEGED_OP);
    order_code = kvm_s390_get_base_disp_rs(vcpu, core::ptr::null_mut());
    if (handle_sigp_order_in_user_space(vcpu, order_code, cpu_addr))
    return -EOPNOTSUPP;
    if (r1 % 2)
    parameter = vcpu.run.s.regs.gprs[r1];
    else
    parameter = vcpu.run.s.regs.gprs[r1 + 1];
    trace_kvm_s390_handle_sigp(vcpu, order_code, cpu_addr, parameter);
    switch (order_code) {
    case SIGP_SET_ARCHITECTURE:
    vcpu.stat.instruction_sigp_arch++;
    rc = __sigp_set_arch(vcpu, parameter,
    &vcpu.run.s.regs.gprs[r1]);
    break;
    default:
    rc = handle_sigp_dst(vcpu, order_code, cpu_addr,
    parameter,
    &vcpu.run.s.regs.gprs[r1]);
    }
    if (rc < 0)
    return rc;
    kvm_s390_set_psw_cc(vcpu, rc);
    return 0;
    }
//
// Handle SIGP partial execution interception.
//
// This interception will occur at the source cpu when a source cpu sends an
// external call to a target cpu and the target cpu has the WAIT bit set in
// its cpuflags. Interception will occur after the interrupt indicator bits at
// the target cpu have been set. All error cases will lead to instruction
// interception, therefore nothing is to be checked or prepared.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_s390_handle_sigp_pei(vcpu: *mut kvm_vcpu) -> c_int {
    int kvm_s390_handle_sigp_pei(struct kvm_vcpu *vcpu)
    {
    let mut r3: c_int = vcpu.arch.sie_block.ipa & 0x000f;
    let mut cpu_addr: u16 = vcpu.run.s.regs.gprs[r3];
    struct kvm_vcpu *dest_vcpu;
    let mut order_code: u8 = kvm_s390_get_base_disp_rs(vcpu, core::ptr::null_mut());
    if (order_code == SIGP_EXTERNAL_CALL) {
    trace_kvm_s390_handle_sigp_pei(vcpu, order_code, cpu_addr);
    dest_vcpu = kvm_get_vcpu_by_id(vcpu.kvm, cpu_addr);
    BUG_ON(dest_vcpu == core::ptr::null_mut());
    kvm_s390_vcpu_wakeup(dest_vcpu);
    kvm_s390_set_psw_cc(vcpu, SIGP_CC_ORDER_CODE_ACCEPTED);
    return 0;
    }
    return -EOPNOTSUPP;
    }
