//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/smccc/kvm_guest.c
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

    static DECLARE_BITMAP(__kvm_arm_hyp_services, ARM_SMCCC_KVM_NUM_FUNCS) __ro_after_init = { };
#[no_mangle]
pub unsafe extern "C" fn kvm_init_hyp_services() -> void __init {
    void __init kvm_init_hyp_services(void)
    {
    let mut kvm_uuid: uuid_t = ARM_SMCCC_VENDOR_HYP_UID_KVM;
    struct arm_smccc_res res;
    u32 val[4];
    if (!arm_smccc_hypervisor_has_uuid(&kvm_uuid))
    return;
    memset(&res, 0, sizeof(res));
    arm_smccc_1_1_invoke(ARM_SMCCC_VENDOR_HYP_KVM_FEATURES_FUNC_ID, &res);
    val[0] = lower_32_bits(res.a0);
    val[1] = lower_32_bits(res.a1);
    val[2] = lower_32_bits(res.a2);
    val[3] = lower_32_bits(res.a3);
    bitmap_from_arr32(__kvm_arm_hyp_services, val, ARM_SMCCC_KVM_NUM_FUNCS);
    pr_info("hypervisor services detected (0x%08lx 0x%08lx 0x%08lx 0x%08lx)\n",
    res.a3, res.a2, res.a1, res.a0);
    kvm_arch_init_hyp_services();
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arm_hyp_service_available(func_id: u32) -> bool {
    bool kvm_arm_hyp_service_available(u32 func_id)
    {
    if (func_id >= ARM_SMCCC_KVM_NUM_FUNCS)
    return false;
    return test_bit(func_id, __kvm_arm_hyp_services);
    }
    EXPORT_SYMBOL_GPL(kvm_arm_hyp_service_available);

#[no_mangle]
pub unsafe extern "C" fn kvm_arm_target_impl_cpu_init() -> void  __init {
    void  __init kvm_arm_target_impl_cpu_init(void)
    {
    int i;
    u32 ver;
    u64 max_cpus;
    struct arm_smccc_res res;
    struct target_impl_cpu *target;
    if (!kvm_arm_hyp_service_available(ARM_SMCCC_KVM_FUNC_DISCOVER_IMPL_VER) ||
    !kvm_arm_hyp_service_available(ARM_SMCCC_KVM_FUNC_DISCOVER_IMPL_CPUS))
    return;
    arm_smccc_1_1_invoke(ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_VER_FUNC_ID,
    0, &res);
    if (res.a0 != SMCCC_RET_SUCCESS)
    return;
// Version info is in lower 32 bits and is in SMMCCC_VERSION format
    ver = lower_32_bits(res.a1);
    if (PSCI_VERSION_MAJOR(ver) != 1) {
    pr_warn("Unsupported target CPU implementation version v%d.%d\n",
    PSCI_VERSION_MAJOR(ver), PSCI_VERSION_MINOR(ver));
    return;
    }
    if (!res.a2) {
    pr_warn("No target implementation CPUs specified\n");
    return;
    }
    max_cpus = res.a2;
    target = memblock_alloc(sizeof(*target) * max_cpus,  __alignof__(*target));
    if (!target) {
    pr_warn("Not enough memory for struct target_impl_cpu\n");
    return;
    }
    for (i = 0; i < max_cpus; i++) {
    arm_smccc_1_1_invoke(ARM_SMCCC_VENDOR_HYP_KVM_DISCOVER_IMPL_CPUS_FUNC_ID,
    i, 0, 0, &res);
    if (res.a0 != SMCCC_RET_SUCCESS) {
    pr_warn("Discovering target implementation CPUs failed\n");
    goto mem_free;
    }
    target[i].midr = res.a1;
    target[i].revidr = res.a2;
    target[i].aidr = res.a3;
    }
    if (!cpu_errata_set_target_impl(max_cpus, target)) {
    pr_warn("Failed to set target implementation CPUs\n");
    goto mem_free;
    }
    pr_info("Number of target implementation CPUs is %lld\n", max_cpus);
    return;
    mem_free:
    memblock_free(target, sizeof(*target) * max_cpus);
    }
