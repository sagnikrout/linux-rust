//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/vgic/vgic-v5.c
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
// Copyright (C) 2025, 2026 Arm Ltd.
//

//
// Not all PPIs are guaranteed to be implemented for GICv5. Deterermine which
// ones are, and generate a mask.
//
#[no_mangle]
unsafe extern "C" fn vgic_v5_get_implemented_ppis() {
    static void vgic_v5_get_implemented_ppis(void)
    {
//
// If we have KVM, we have EL2, which means that we have support for the
// EL1 and EL2 Physical & Virtual timers.
//
    __set_bit(GICV5_ARCH_PPI_CNTHP, ppi_caps.impl_ppi_mask);
    __set_bit(GICV5_ARCH_PPI_CNTV, ppi_caps.impl_ppi_mask);
    __set_bit(GICV5_ARCH_PPI_CNTHV, ppi_caps.impl_ppi_mask);
    __set_bit(GICV5_ARCH_PPI_CNTP, ppi_caps.impl_ppi_mask);
// The SW_PPI should be available
    __set_bit(GICV5_ARCH_PPI_SW_PPI, ppi_caps.impl_ppi_mask);
// The PMUIRQ is available if we have the PMU
    __assign_bit(GICV5_ARCH_PPI_PMUIRQ, ppi_caps.impl_ppi_mask, system_supports_pmuv3());
    }
//
// Probe for a vGICv5 compatible interrupt controller, returning 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_probe(info: *const gic_kvm_info) -> c_int {
    int vgic_v5_probe(const struct gic_kvm_info *info)
    {
    let mut v5_registered: bool = false;
    int ret;
    kvm_vgic_global_state.type = VGIC_V5;
    kvm_vgic_global_state.vcpu_base = 0;
    kvm_vgic_global_state.vctrl_base = core::ptr::null_mut();
    kvm_vgic_global_state.can_emulate_gicv2 = false;
    kvm_vgic_global_state.has_gicv4 = false;
    kvm_vgic_global_state.has_gicv4_1 = false;
//
// GICv5 is currently not supported in Protected mode. Skip the
// registration of GICv5 completely to make sure no guests can create a
// GICv5-based guest.
//
    if (is_protected_kvm_enabled()) {
    kvm_info("GICv5-based guests are not supported with pKVM\n");
    goto skip_v5;
    }
    kvm_vgic_global_state.max_gic_vcpus = VGIC_V5_MAX_CPUS;
    vgic_v5_get_implemented_ppis();
    ret = kvm_register_vgic_device(KVM_DEV_TYPE_ARM_VGIC_V5);
    if (ret) {
    kvm_err("Cannot register GICv5 KVM device.\n");
    goto skip_v5;
    }
    v5_registered = true;
    kvm_info("GCIE system register CPU interface\n");
    skip_v5:
// If we don't support the GICv3 compat mode we're done.
    if (!cpus_have_final_cap(ARM64_HAS_GICV5_LEGACY)) {
    if (!v5_registered)
    return -ENODEV;
    return 0;
    }
    kvm_vgic_global_state.has_gcie_v3_compat = true;
//
// The ListRegs field is 5 bits, but there is an architectural
// maximum of 16 list registers. Just ignore bit 4...
//
    kvm_vgic_global_state.nr_lr = (vgic_ich_vtr() & 0xf) + 1;
    ret = kvm_register_vgic_device(KVM_DEV_TYPE_ARM_VGIC_V3);
    if (ret) {
    kvm_err("Cannot register GICv3-legacy KVM device.\n");
    return ret;
    }
// We potentially limit the max VCPUs further than we need to here
    kvm_vgic_global_state.max_gic_vcpus = min(VGIC_V3_MAX_CPUS,
    VGIC_V5_MAX_CPUS);
    static_branch_enable(&kvm_vgic_global_state.gicv3_cpuif);
    kvm_info("GCIE legacy system register CPU interface\n");
    vgic_v3_enable_cpuif_traps();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_reset(vcpu: *mut kvm_vcpu) {
    void vgic_v5_reset(struct kvm_vcpu *vcpu)
    {
//
// We always present 16-bits of ID space to the guest, irrespective of
// the host allowing more.
//
    vcpu.arch.vgic_cpu.num_id_bits = ICC_IDR0_EL1_ID_BITS_16BITS;
//
// The GICv5 architeture only supports 5-bits of priority in the
// CPUIF (but potentially fewer in the IRS).
//
    vcpu.arch.vgic_cpu.num_pri_bits = 5;
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_init(kvm: *mut kvm) -> c_int {
    int vgic_v5_init(struct kvm *kvm)
    {
    struct kvm_vcpu *vcpu;
    unsigned long idx;
    if (vgic_initialized(kvm))
    return 0;
    kvm_for_each_vcpu(idx, vcpu, kvm) {
    if (vcpu_has_nv(vcpu)) {
    kvm_err("Nested GICv5 VMs are currently unsupported\n");
    return -EINVAL;
    }
    }
// We only allow userspace to drive the SW_PPI, if it is implemented.
    bitmap_zero(kvm.arch.vgic.gicv5_vm.userspace_ppis,
    VGIC_V5_NR_PRIVATE_IRQS);
    __set_bit(GICV5_ARCH_PPI_SW_PPI, kvm.arch.vgic.gicv5_vm.userspace_ppis);
    bitmap_and(kvm.arch.vgic.gicv5_vm.userspace_ppis,
    kvm.arch.vgic.gicv5_vm.userspace_ppis,
    ppi_caps.impl_ppi_mask, VGIC_V5_NR_PRIVATE_IRQS);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_map_resources(kvm: *mut kvm) -> c_int {
    int vgic_v5_map_resources(struct kvm *kvm)
    {
    if (!vgic_initialized(kvm))
    return -EBUSY;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_finalize_ppi_state(kvm: *mut kvm) -> c_int {
    int vgic_v5_finalize_ppi_state(struct kvm *kvm)
    {
    struct kvm_vcpu *vcpu0;
    int i;
    if (!vgic_is_v5(kvm))
    return 0;
    guard(mutex)(&kvm.arch.config_lock);
//
// If SW_PPI has been advertised, then we know we already
// initialised the whole thing, and we can return early. Yes,
// this is pretty hackish as far as state tracking goes...
//
    if (test_bit(GICV5_ARCH_PPI_SW_PPI, kvm.arch.vgic.gicv5_vm.vgic_ppi_mask))
    return 0;
// The PPI state for all VCPUs should be the same. Pick the first.
    vcpu0 = kvm_get_vcpu(kvm, 0);
    bitmap_zero(kvm.arch.vgic.gicv5_vm.vgic_ppi_mask, VGIC_V5_NR_PRIVATE_IRQS);
    bitmap_zero(kvm.arch.vgic.gicv5_vm.vgic_ppi_hmr, VGIC_V5_NR_PRIVATE_IRQS);
    for_each_set_bit(i, ppi_caps.impl_ppi_mask, VGIC_V5_NR_PRIVATE_IRQS) {
    let mut intid: u32 = vgic_v5_make_ppi(i);
    struct vgic_irq *irq;
    irq = vgic_get_vcpu_irq(vcpu0, intid);
// Expose PPIs with an owner or the SW_PPI, only
    scoped_guard(raw_spinlock_irqsave, &irq.irq_lock) {
    if (irq.owner || i == GICV5_ARCH_PPI_SW_PPI) {
    __set_bit(i, kvm.arch.vgic.gicv5_vm.vgic_ppi_mask);
    __assign_bit(i, kvm.arch.vgic.gicv5_vm.vgic_ppi_hmr,
    irq.config == VGIC_CONFIG_LEVEL);
    }
    }
    vgic_put_irq(vcpu0.kvm, irq);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vgic_v5_get_effective_priority_mask(vcpu: *mut kvm_vcpu) -> u32 {
    static u32 vgic_v5_get_effective_priority_mask(struct kvm_vcpu *vcpu)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
    u32 highest_ap, priority_mask, apr;
//
// If the guest's CPU has not opted to receive interrupts, then the
// effective running priority is the highest priority. Just return 0
// (the highest priority).
//
    if (!FIELD_GET(FEAT_GCIE_ICH_VMCR_EL2_EN, cpu_if.vgic_vmcr))
    return 0;
//
// Counting the number of trailing zeros gives the current active
// priority. Explicitly use the 32-bit version here as we have 32
// priorities. 32 then means that there are no active priorities.
//
    apr = cpu_if.vgic_apr;
    highest_ap = apr ? __builtin_ctz(apr) : 32;
//
// An interrupt is of sufficient priority if it is equal to or
// greater than the priority mask. Add 1 to the priority mask
// (i.e., lower priority) to match the APR logic before taking
// the min. This gives us the lowest priority that is masked.
//
    priority_mask = FIELD_GET(FEAT_GCIE_ICH_VMCR_EL2_VPMR, cpu_if.vgic_vmcr);
    return min(highest_ap, priority_mask + 1);
    }
//
// For GICv5, the PPIs are mostly directly managed by the hardware. We (the
// hypervisor) handle the pending, active, enable state save/restore, but
// don't need the PPIs to be queued on a per-VCPU AP list. Therefore,
// unlock, kick the vcpu and return.
//
    bool vgic_v5_ppi_queue_irq_unlock(struct kvm *kvm, struct vgic_irq *irq,
    unsigned long flags)
    __releases(&irq.irq_lock)
    {
    struct kvm_vcpu *vcpu;
    lockdep_assert_held(&irq.irq_lock);
    vcpu = irq.target_vcpu;
    raw_spin_unlock_irqrestore(&irq.irq_lock, flags);
// Directly kick the target VCPU to make sure it sees the IRQ
    kvm_make_request(KVM_REQ_IRQ_PENDING, vcpu);
    kvm_vcpu_kick(vcpu);
    return true;
    }
//
// Sets/clears the corresponding bit in the ICH_PPI_DVIR register.
//
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_set_ppi_dvi(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, dvi: bool) {
    void vgic_v5_set_ppi_dvi(struct kvm_vcpu *vcpu, struct vgic_irq *irq, bool dvi)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
    u32 ppi;
    lockdep_assert_held(&irq.irq_lock);
    ppi = vgic_v5_get_hwirq_id(irq.intid);
    assign_bit(ppi, cpu_if.vgic_ppi_dvir, dvi);
    }
    static const struct irq_ops vgic_v5_ppi_irq_ops = {
    .queue_irq_unlock = vgic_v5_ppi_queue_irq_unlock,
    .set_direct_injection = vgic_v5_set_ppi_dvi,
    };
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_set_ppi_ops(vcpu: *mut kvm_vcpu, vintid: u32) {
    void vgic_v5_set_ppi_ops(struct kvm_vcpu *vcpu, u32 vintid)
    {
    kvm_vgic_set_irq_ops(vcpu, vintid, &vgic_v5_ppi_irq_ops);
    }
//
// Sync back the PPI priorities to the vgic_irq shadow state for any interrupts
// exposed to the guest (skipping all others).
//
#[no_mangle]
unsafe extern "C" fn vgic_v5_sync_ppi_priorities(vcpu: *mut kvm_vcpu) {
    static void vgic_v5_sync_ppi_priorities(struct kvm_vcpu *vcpu)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
    u64 priorityr;
    int i;
//
// We have up to 16 PPI Priority regs, but only have a few interrupts
// that the guest is allowed to use. Limit our sync of PPI priorities to
// those actually exposed to the guest by first iterating over the mask
// of exposed PPIs.
//
    for_each_visible_v5_ppi(i, vcpu.kvm) {
    let mut intid: u32 = vgic_v5_make_ppi(i);
    struct vgic_irq *irq;
    int pri_idx, pri_reg, pri_bit;
    u8 priority;
//
// Determine which priority register and the field within it to
// extract.
//
    pri_reg = i / 8;
    pri_idx = i % 8;
    pri_bit = pri_idx * 8;
    priorityr = cpu_if.vgic_ppi_priorityr[pri_reg];
    priority = field_get(GENMASK(pri_bit + 4, pri_bit), priorityr);
    irq = vgic_get_vcpu_irq(vcpu, intid);
    scoped_guard(raw_spinlock_irqsave, &irq.irq_lock)
    irq.priority = priority;
    vgic_put_irq(vcpu.kvm, irq);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_has_pending_ppi(vcpu: *mut kvm_vcpu) -> bool {
    bool vgic_v5_has_pending_ppi(struct kvm_vcpu *vcpu)
    {
    unsigned int priority_mask;
    int i;
    priority_mask = vgic_v5_get_effective_priority_mask(vcpu);
//
// If the combined priority mask is 0, nothing can be signalled! In the
// case where the guest has disabled interrupt delivery for the vcpu
// (via ICV_CR0_EL1.EN->ICH_VMCR_EL2.EN), we calculate the priority mask
// as 0 too (the highest possible priority).
//
    if (!priority_mask)
    return false;
    for_each_visible_v5_ppi(i, vcpu.kvm) {
    let mut intid: u32 = vgic_v5_make_ppi(i);
    let mut has_pending: bool = false;
    struct vgic_irq *irq;
    irq = vgic_get_vcpu_irq(vcpu, intid);
    scoped_guard(raw_spinlock_irqsave, &irq.irq_lock)
    if (irq.enabled && irq.priority < priority_mask)
    has_pending = irq.hw ? vgic_get_phys_line_level(irq) : irq_is_pending(irq);
    vgic_put_irq(vcpu.kvm, irq);
    if (has_pending)
    return true;
    }
    return false;
    }
//
// Detect any PPIs state changes, and propagate the state with KVM's
// shadow structures.
//
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_fold_ppi_state(vcpu: *mut kvm_vcpu) {
    void vgic_v5_fold_ppi_state(struct kvm_vcpu *vcpu)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
    unsigned long *activer, *pendr;
    int i;
    activer = host_data_ptr(vgic_v5_ppi_state).activer_exit;
    pendr = host_data_ptr(vgic_v5_ppi_state).pendr;
    for_each_visible_v5_ppi(i, vcpu.kvm) {
    let mut intid: u32 = vgic_v5_make_ppi(i);
    struct vgic_irq *irq;
    irq = vgic_get_vcpu_irq(vcpu, intid);
    scoped_guard(raw_spinlock_irqsave, &irq.irq_lock) {
    irq.active = test_bit(i, activer);
// This is an OR to avoid losing incoming edges!
    if (irq.config == VGIC_CONFIG_EDGE)
    irq.pending_latch |= test_bit(i, pendr);
    }
    vgic_put_irq(vcpu.kvm, irq);
    }
//
// Re-inject the exit state as entry state next time!
//
// Note that the write of the Enable state is trapped, and hence there
// is nothing to explcitly sync back here as we already have the latest
// copy by definition.
//
    bitmap_copy(cpu_if.vgic_ppi_activer, activer, VGIC_V5_NR_PRIVATE_IRQS);
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_flush_ppi_state(vcpu: *mut kvm_vcpu) {
    void vgic_v5_flush_ppi_state(struct kvm_vcpu *vcpu)
    {
    DECLARE_BITMAP(pendr, VGIC_V5_NR_PRIVATE_IRQS);
    int i;
//
// Time to enter the guest - we first need to build the guest's
// ICC_PPI_PENDRx_EL1, however.
//
    bitmap_zero(pendr, VGIC_V5_NR_PRIVATE_IRQS);
    for_each_visible_v5_ppi(i, vcpu.kvm) {
    let mut intid: u32 = vgic_v5_make_ppi(i);
    struct vgic_irq *irq;
    irq = vgic_get_vcpu_irq(vcpu, intid);
    scoped_guard(raw_spinlock_irqsave, &irq.irq_lock) {
    __assign_bit(i, pendr, irq_is_pending(irq));
    if (irq.config == VGIC_CONFIG_EDGE)
    irq.pending_latch = false;
    }
    vgic_put_irq(vcpu.kvm, irq);
    }
//
// Copy the shadow state to the pending reg that will be written to the
// ICH_PPI_PENDRx_EL2 regs. While the guest is running we track any
// incoming changes to the pending state in the vgic_irq structures. The
// incoming changes are merged with the outgoing changes on the return
// path.
//
    bitmap_copy(host_data_ptr(vgic_v5_ppi_state).pendr, pendr,
    VGIC_V5_NR_PRIVATE_IRQS);
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_load(vcpu: *mut kvm_vcpu) {
    void vgic_v5_load(struct kvm_vcpu *vcpu)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
//
// On the WFI path, vgic_load is called a second time. The first is when
// scheduling in the vcpu thread again, and the second is when leaving
// WFI. Skip the second instance as it serves no purpose and just
// restores the same state again.
//
    if (cpu_if.gicv5_vpe.resident)
    return;
    kvm_call_hyp(__vgic_v5_restore_vmcr_apr, cpu_if);
    cpu_if.gicv5_vpe.resident = true;
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_put(vcpu: *mut kvm_vcpu) {
    void vgic_v5_put(struct kvm_vcpu *vcpu)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
//
// Do nothing if we're not resident. This can happen in the WFI path
// where we do a vgic_put in the WFI path and again later when
// descheduling the thread. We risk losing VMCR state if we sync it
// twice, so instead return early in this case.
//
    if (!cpu_if.gicv5_vpe.resident)
    return;
    kvm_call_hyp(__vgic_v5_save_apr, cpu_if);
    cpu_if.gicv5_vpe.resident = false;
// The shadow priority is only updated on entering WFI
    if (vcpu_get_flag(vcpu, IN_WFI))
    vgic_v5_sync_ppi_priorities(vcpu);
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_get_vmcr(vcpu: *mut kvm_vcpu, vmcrp: *mut vgic_vmcr) {
    void vgic_v5_get_vmcr(struct kvm_vcpu *vcpu, struct vgic_vmcr *vmcrp)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
    let mut vmcr: u64 = cpu_if.vgic_vmcr;
    vmcrp.en = FIELD_GET(FEAT_GCIE_ICH_VMCR_EL2_EN, vmcr);
    vmcrp.pmr = FIELD_GET(FEAT_GCIE_ICH_VMCR_EL2_VPMR, vmcr);
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_set_vmcr(vcpu: *mut kvm_vcpu, vmcrp: *mut vgic_vmcr) {
    void vgic_v5_set_vmcr(struct kvm_vcpu *vcpu, struct vgic_vmcr *vmcrp)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
    u64 vmcr;
    vmcr = FIELD_PREP(FEAT_GCIE_ICH_VMCR_EL2_VPMR, vmcrp.pmr) |
    FIELD_PREP(FEAT_GCIE_ICH_VMCR_EL2_EN, vmcrp.en);
    cpu_if.vgic_vmcr = vmcr;
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_restore_state(vcpu: *mut kvm_vcpu) {
    void vgic_v5_restore_state(struct kvm_vcpu *vcpu)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
    __vgic_v5_restore_state(cpu_if);
    __vgic_v5_restore_ppi_state(cpu_if);
    dsb(sy);
    }
#[no_mangle]
pub unsafe extern "C" fn vgic_v5_save_state(vcpu: *mut kvm_vcpu) {
    void vgic_v5_save_state(struct kvm_vcpu *vcpu)
    {
    struct vgic_v5_cpu_if *cpu_if = &vcpu.arch.vgic_cpu.vgic_v5;
    __vgic_v5_save_state(cpu_if);
    __vgic_v5_save_ppi_state(cpu_if);
    dsb(sy);
    }
