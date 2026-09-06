//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kvm/book3s_hv_ras.c
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
// Copyright 2012 Paul Mackerras, IBM Corp. <paulus@au1.ibm.com>
//

// SRR1 bits for machine check on POWER7

pub const SRR1_MC_IFETCH_MASK: c_uint = 0x7;

// DSISR bits for machine check on POWER7
pub const DSISR_MC_DERAT_MULTI: c_uint = 0x800		/* D-ERAT multi-hit */;
pub const DSISR_MC_TLB_MULTI: c_uint = 0x400		/* D-TLB multi-hit */;
pub const DSISR_MC_SLB_PARITY: c_uint = 0x100		/* SLB parity error */;
pub const DSISR_MC_SLB_MULTI: c_uint = 0x080		/* SLB multi-hit */;
pub const DSISR_MC_SLB_PARMULTI: c_uint = 0x040		/* SLB parity + multi-hit */;
// POWER7 SLB flush and reload
#[no_mangle]
unsafe extern "C" fn reload_slb(vcpu: *mut kvm_vcpu) {
    static void reload_slb(struct kvm_vcpu *vcpu)
    {
    struct slb_shadow *slb;
    unsigned long i, n;
// First clear out SLB
    asm volatile("slbmte %0,%0; slbia" : : "r" (0));
// Do they have an SLB shadow buffer registered?
    slb = vcpu.arch.slb_shadow.pinned_addr;
    if (!slb)
    return;
// Sanity check
    n = min_t(u32, be32_to_cpu(slb.persistent), SLB_MIN_SIZE);
    if ((void *) &slb.save_area[n] > vcpu.arch.slb_shadow.pinned_end)
    return;
// Load up the SLB from that
    for (i = 0; i < n; ++i) {
    let mut rb: c_ulong = be64_to_cpu(slb.save_area[i].esid);
    let mut rs: c_ulong = be64_to_cpu(slb.save_area[i].vsid);
    rb = (rb & ~0xFFFul) | i;	/* insert entry number */
    asm volatile("slbmte %0,%1" : : "r" (rs), "r" (rb));
    }
    }
//
// On POWER7, see if we can handle a machine check that occurred inside
// the guest in real mode, without switching to the host partition.
//
#[no_mangle]
unsafe extern "C" fn kvmppc_realmode_mc_power7(vcpu: *mut kvm_vcpu) -> c_long {
    static long kvmppc_realmode_mc_power7(struct kvm_vcpu *vcpu)
    {
    let mut srr1: c_ulong = vcpu.arch.shregs.msr;
    let mut handled: c_long = 1;
    if (srr1 & SRR1_MC_LDSTERR) {
// error on load/store
    let mut dsisr: c_ulong = vcpu.arch.shregs.dsisr;
    if (dsisr & (DSISR_MC_SLB_PARMULTI | DSISR_MC_SLB_MULTI |
    DSISR_MC_SLB_PARITY | DSISR_MC_DERAT_MULTI)) {
// flush and reload SLB; flushes D-ERAT too
    reload_slb(vcpu);
    dsisr &= ~(DSISR_MC_SLB_PARMULTI | DSISR_MC_SLB_MULTI |
    DSISR_MC_SLB_PARITY | DSISR_MC_DERAT_MULTI);
    }
    if (dsisr & DSISR_MC_TLB_MULTI) {
    tlbiel_all_lpid(vcpu.kvm.arch.radix);
    dsisr &= ~DSISR_MC_TLB_MULTI;
    }
// Any other errors we don't understand?
    if (dsisr & 0xffffffffUL)
    handled = 0;
    }
    switch ((srr1 >> SRR1_MC_IFETCH_SH) & SRR1_MC_IFETCH_MASK) {
    case 0:
    break;
    case SRR1_MC_IFETCH_SLBPAR:
    case SRR1_MC_IFETCH_SLBMULTI:
    case SRR1_MC_IFETCH_SLBPARMULTI:
    reload_slb(vcpu);
    break;
    case SRR1_MC_IFETCH_TLBMULTI:
    tlbiel_all_lpid(vcpu.kvm.arch.radix);
    break;
    default:
    handled = 0;
    }
    return handled;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_realmode_machine_check(vcpu: *mut kvm_vcpu) {
    void kvmppc_realmode_machine_check(struct kvm_vcpu *vcpu)
    {
    struct machine_check_event mce_evt;
    long handled;
    if (vcpu.kvm.arch.fwnmi_enabled) {
// FWNMI guests handle their own recovery
    handled = 0;
    } else {
    handled = kvmppc_realmode_mc_power7(vcpu);
    }
//
// Now get the event and stash it in the vcpu struct so it can
// be handled by the primary thread in virtual mode.  We can't
// call machine_check_queue_event() here if we are running on
// an offline secondary thread.
//
    if (get_mce_event(&mce_evt, MCE_EVENT_RELEASE)) {
    if (handled && mce_evt.version == MCE_V1)
    mce_evt.disposition = MCE_DISPOSITION_RECOVERED;
    } else {
    memset(&mce_evt, 0, sizeof(mce_evt));
    }
    vcpu.arch.mce_evt = mce_evt;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_p9_realmode_hmi_handler(vcpu: *mut kvm_vcpu) -> c_long {
    long kvmppc_p9_realmode_hmi_handler(struct kvm_vcpu *vcpu)
    {
    struct kvmppc_vcore *vc = vcpu.arch.vcore;
    let mut ret: c_long = 0;
//
// Unapply and clear the offset first. That way, if the TB was not
// resynced then it will remain in host-offset, and if it was resynced
// then it is brought into host-offset. Then the tb offset is
// re-applied before continuing with the KVM exit.
//
// This way, we don't need to actually know whether not OPAL resynced
// the timebase or do any of the complicated dance that the P7/8
// path requires.
//
    if (vc.tb_offset_applied) {
    let mut new_tb: u64 = mftb() - vc.tb_offset_applied;
    mtspr(SPRN_TBU40, new_tb);
    if ((mftb() & 0xffffff) < (new_tb & 0xffffff)) {
    new_tb += 0x1000000;
    mtspr(SPRN_TBU40, new_tb);
    }
    vc.tb_offset_applied = 0;
    }
    local_paca.hmi_irqs++;
    if (hmi_handle_debugtrig(core::ptr::null_mut()) >= 0) {
    ret = 1;
    goto out;
    }
    if (ppc_md.hmi_exception_early)
    ppc_md.hmi_exception_early(core::ptr::null_mut());
    out:
    if (kvmppc_get_tb_offset(vcpu)) {
    let mut new_tb: u64 = mftb() + vc.tb_offset;
    mtspr(SPRN_TBU40, new_tb);
    if ((mftb() & 0xffffff) < (new_tb & 0xffffff)) {
    new_tb += 0x1000000;
    mtspr(SPRN_TBU40, new_tb);
    }
    vc.tb_offset_applied = kvmppc_get_tb_offset(vcpu);
    }
    return ret;
    }
//
// The following subcore HMI handling is all only for pre-POWER9 CPUs.
//
// Check if dynamic split is in force and return subcore size accordingly.
#[no_mangle]
pub unsafe extern "C" fn kvmppc_cur_subcore_size() -> c_int {
    static inline int kvmppc_cur_subcore_size(void)
    {
    if (local_paca.kvm_hstate.kvm_split_mode)
    return local_paca.kvm_hstate.kvm_split_mode.subcore_size;
    return threads_per_subcore;
    }
#[no_mangle]
pub unsafe extern "C" fn kvmppc_subcore_enter_guest() {
    void kvmppc_subcore_enter_guest(void)
    {
    int thread_id, subcore_id;
    thread_id = cpu_thread_in_core(local_paca.paca_index);
    subcore_id = thread_id / kvmppc_cur_subcore_size();
    local_paca.sibling_subcore_state.in_guest[subcore_id] = 1;
    }
    EXPORT_SYMBOL_GPL(kvmppc_subcore_enter_guest);
#[no_mangle]
pub unsafe extern "C" fn kvmppc_subcore_exit_guest() {
    void kvmppc_subcore_exit_guest(void)
    {
    int thread_id, subcore_id;
    thread_id = cpu_thread_in_core(local_paca.paca_index);
    subcore_id = thread_id / kvmppc_cur_subcore_size();
    local_paca.sibling_subcore_state.in_guest[subcore_id] = 0;
    }
    EXPORT_SYMBOL_GPL(kvmppc_subcore_exit_guest);
#[no_mangle]
unsafe extern "C" fn kvmppc_tb_resync_required() -> bool {
    static bool kvmppc_tb_resync_required(void)
    {
    if (test_and_set_bit(CORE_TB_RESYNC_REQ_BIT,
    &local_paca.sibling_subcore_state.flags))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn kvmppc_tb_resync_done() {
    static void kvmppc_tb_resync_done(void)
    {
    clear_bit(CORE_TB_RESYNC_REQ_BIT,
    &local_paca.sibling_subcore_state.flags);
    }
//
// kvmppc_realmode_hmi_handler() is called only by primary thread during
// guest exit path.
//
// There are multiple reasons why HMI could occur, one of them is
// Timebase (TB) error. If this HMI is due to TB error, then TB would
// have been in stopped state. The opal hmi handler Will fix it and
// restore the TB value with host timebase value. For HMI caused due
// to non-TB errors, opal hmi handler will not touch/restore TB register
// and hence there won't be any change in TB value.
//
// Since we are not sure about the cause of this HMI, we can't be sure
// about the content of TB register whether it holds guest or host timebase
// value. Hence the idea is to resync the TB on every HMI, so that we
// know about the exact state of the TB value. Resync TB call will
// restore TB to host timebase.
//
// Things to consider:
// - On TB error, HMI interrupt is reported on all the threads of the core
// that has encountered TB error irrespective of split-core mode.
// - The very first thread on the core that get chance to fix TB error
// would rsync the TB with local chipTOD value.
// - The resync TB is a core level action i.e. it will sync all the TBs
// in that core independent of split-core mode. This means if we trigger
// TB sync from a thread from one subcore, it would affect TB values of
// sibling subcores of the same core.
//
// All threads need to co-ordinate before making opal hmi handler.
// All threads will use sibling_subcore_state->in_guest[] (shared by all
// threads in the core) in paca which holds information about whether
// sibling subcores are in Guest mode or host mode. The in_guest[] array
// is of size MAX_SUBCORE_PER_CORE=4, indexed using subcore id to set/unset
// subcore status. Only primary threads from each subcore is responsible
// to set/unset its designated array element while entering/exiting the
// guset.
//
// After invoking opal hmi handler call, one of the thread (of entire core)
// will need to resync the TB. Bit 63 from subcore state bitmap flags
// (sibling_subcore_state->flags) will be used to co-ordinate between
// primary threads to decide who takes up the responsibility.
//
// This is what we do:
// - Primary thread from each subcore tries to set resync required bit[63]
// of paca->sibling_subcore_state->flags.
// - The first primary thread that is able to set the flag takes the
// responsibility of TB resync. (Let us call it as thread leader)
// - All other threads which are in host will call
// wait_for_subcore_guest_exit() and wait for in_guest[0-3] from
// paca->sibling_subcore_state to get cleared.
// - All the primary thread will clear its subcore status from subcore
// state in_guest[] array respectively.
// - Once all primary threads clear in_guest[0-3], all of them will invoke
// opal hmi handler.
// - Now all threads will wait for TB resync to complete by invoking
// wait_for_tb_resync() except the thread leader.
// - Thread leader will do a TB resync by invoking opal_resync_timebase()
// call and the it will clear the resync required bit.
// - All other threads will now come out of resync wait loop and proceed
// with individual execution.
// - On return of this function, primary thread will signal all
// secondary threads to proceed.
// - All secondary threads will eventually call opal hmi handler on
// their exit path.
//
// Returns 1 if the timebase offset should be applied, 0 if not.
//
#[no_mangle]
pub unsafe extern "C" fn kvmppc_realmode_hmi_handler() -> c_long {
    long kvmppc_realmode_hmi_handler(void)
    {
    bool resync_req;
    local_paca.hmi_irqs++;
    if (hmi_handle_debugtrig(core::ptr::null_mut()) >= 0)
    return 1;
//
// By now primary thread has already completed guest->host
// partition switch but haven't signaled secondaries yet.
// All the secondary threads on this subcore is waiting
// for primary thread to signal them to go ahead.
//
// For threads from subcore which isn't in guest, they all will
// wait until all other subcores on this core exit the guest.
//
// Now set the resync required bit. If you are the first to
// set this bit then kvmppc_tb_resync_required() function will
// return true. For rest all other subcores
// kvmppc_tb_resync_required() will return false.
//
// If resync_req == true, then this thread is responsible to
// initiate TB resync after hmi handler has completed.
// All other threads on this core will wait until this thread
// clears the resync required bit flag.
//
    resync_req = kvmppc_tb_resync_required();
// Reset the subcore status to indicate it has exited guest
    kvmppc_subcore_exit_guest();
//
// Wait for other subcores on this core to exit the guest.
// All the primary threads and threads from subcore that are
// not in guest will wait here until all subcores are out
// of guest context.
//
    wait_for_subcore_guest_exit();
//
// At this point we are sure that primary threads from each
// subcore on this core have completed guest->host partition
// switch. Now it is safe to call HMI handler.
//
    if (ppc_md.hmi_exception_early)
    ppc_md.hmi_exception_early(core::ptr::null_mut());
//
// Check if this thread is responsible to resync TB.
// All other threads will wait until this thread completes the
// TB resync.
//
    if (resync_req) {
    opal_resync_timebase();
// Reset TB resync req bit
    kvmppc_tb_resync_done();
    } else {
    wait_for_tb_resync();
    }
//
// Reset tb_offset_applied so the guest exit code won't try
// to subtract the previous timebase offset from the timebase.
//
    if (local_paca.kvm_hstate.kvm_vcore)
    local_paca.kvm_hstate.kvm_vcore.tb_offset_applied = 0;
    return 0;
    }
