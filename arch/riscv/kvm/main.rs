//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/main.c
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
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
//
// Authors:
// Anup Patel <anup.patel@wdc.com>
//

    static DEFINE_PER_CPU(bool, kvm_riscv_virtualization_enabled);
    DEFINE_STATIC_KEY_FALSE(kvm_riscv_vsstage_tlb_no_gpa);
#[no_mangle]
unsafe extern "C" fn kvm_riscv_setup_vendor_features() {
    static void kvm_riscv_setup_vendor_features(void)
    {
// Andes AX66: split two-stage TLBs
    if (riscv_cached_mvendorid(0) == ANDES_VENDOR_ID &&
    (riscv_cached_marchid(0) & 0xFFFF) == 0x8A66) {
    static_branch_enable(&kvm_riscv_vsstage_tlb_no_gpa);
    kvm_info("VS-stage TLB does not cache guest physical address and VMID\n");
    }
    }
    long kvm_arch_dev_ioctl(struct file *filp,
    unsigned int ioctl, unsigned long arg)
    {
    return -EINVAL;
    }
// Initialize hypervisor CSRs - called during CPU online and non-retention idle resume
#[no_mangle]
unsafe extern "C" fn kvm_riscv_csr_init() {
    static void kvm_riscv_csr_init(void)
    {
    csr_write(CSR_HEDELEG, 0);
    csr_write(CSR_HIDELEG, 0);
// VS should access only the time counter directly. Everything else should trap
    csr_write(CSR_HCOUNTEREN, 0x02);
    csr_write(CSR_HVIP, 0);
    }
// Clear hypervisor CSRs - called during CPU offline and non-retention idle entry
#[no_mangle]
unsafe extern "C" fn kvm_riscv_csr_cleanup() {
    static void kvm_riscv_csr_cleanup(void)
    {
//
// After clearing the hideleg CSR, the host kernel will receive
// spurious interrupts if hvip CSR has pending interrupts and the
// corresponding enable bits in vsie CSR are asserted. To avoid it,
// hvip CSR and vsie CSR must be cleared before clearing hideleg CSR.
//
    csr_write(CSR_VSIE, 0);
    csr_write(CSR_HVIP, 0);
    csr_write(CSR_HEDELEG, 0);
    csr_write(CSR_HIDELEG, 0);
    kvm_riscv_clear_former_vcpu();
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_enable_virtualization_cpu() -> c_int {
    int kvm_arch_enable_virtualization_cpu(void)
    {
    int rc;
    rc = kvm_riscv_nacl_enable();
    if (rc)
    return rc;
    kvm_riscv_csr_init();
    kvm_riscv_aia_enable();
    __this_cpu_write(kvm_riscv_virtualization_enabled, true);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_disable_virtualization_cpu() {
    void kvm_arch_disable_virtualization_cpu(void)
    {
    kvm_riscv_aia_disable();
    kvm_riscv_csr_cleanup();
    kvm_riscv_nacl_disable();
    __this_cpu_write(kvm_riscv_virtualization_enabled, false);
    }
#[no_mangle]
unsafe extern "C" fn kvm_riscv_cpu_pm_notifier(self: *mut notifier_block, cmd: c_ulong, v: *mut c_void) -> c_int {
    static int kvm_riscv_cpu_pm_notifier(struct notifier_block *self, unsigned long cmd, void *v)
    {
    switch (cmd) {
    case CPU_PM_EXIT:
    case CPU_PM_ENTER_FAILED:
//
// Only restore hypervisor state if KVM virtualization is
// enabled on this CPU. This prevents unintentional re-enabling
// of virtualization after it has been explicitly disabled.
//
    if (__this_cpu_read(kvm_riscv_virtualization_enabled)) {
    kvm_riscv_csr_init();
    kvm_riscv_aia_pm_exit();
    }
    return NOTIFY_OK;
    case CPU_PM_ENTER:
//
// Only save and clear hypervisor state if KVM virtualization
// is enabled on this CPU.
//
    if (__this_cpu_read(kvm_riscv_virtualization_enabled)) {
    kvm_riscv_aia_pm_enter();
    kvm_riscv_csr_cleanup();
    }
    return NOTIFY_OK;
    default:
    break;
    }
    return NOTIFY_DONE;
    }
    static struct notifier_block kvm_riscv_cpu_pm_nb = {
    .notifier_call = kvm_riscv_cpu_pm_notifier,
    };
#[no_mangle]
unsafe extern "C" fn kvm_riscv_teardown() {
    static void kvm_riscv_teardown(void)
    {
    kvm_riscv_aia_exit();
    kvm_riscv_nacl_exit();
    kvm_riscv_v_exit();
    kvm_unregister_perf_callbacks();
    }
#[no_mangle]
unsafe extern "C" fn riscv_kvm_init() -> int __init {
    static int __init riscv_kvm_init(void)
    {
    int rc;
    char slist[64];
    const char *str;
    if (!riscv_isa_extension_available(core::ptr::null_mut(), H)) {
    kvm_info("hypervisor extension not available\n");
    return -ENODEV;
    }
    if (sbi_spec_is_0_1()) {
    kvm_info("require SBI v0.2 or higher\n");
    return -ENODEV;
    }
    if (!sbi_probe_extension(SBI_EXT_RFENCE)) {
    kvm_info("require SBI RFENCE extension\n");
    return -ENODEV;
    }
    rc = kvm_riscv_nacl_init();
    if (rc && rc != -ENODEV)
    return rc;
    kvm_riscv_gstage_mode_detect();
    switch (kvm_riscv_gstage_max_pgd_levels) {
    case 2:
    str = "Sv32x4";
    break;
    case 3:
    str = "Sv39x4";
    break;
    case 4:
    str = "Sv48x4";
    break;
    case 5:
    str = "Sv57x4";
    break;
    default:
    kvm_riscv_nacl_exit();
    return -ENODEV;
    }
    kvm_riscv_gstage_vmid_detect();
    rc = kvm_riscv_aia_init();
    if (rc && rc != -ENODEV) {
    kvm_riscv_nacl_exit();
    return rc;
    }
    kvm_info("hypervisor extension available\n");
    if (kvm_riscv_nacl_available()) {
    rc = 0;
    slist[0] = '\0';
    if (kvm_riscv_nacl_sync_csr_available()) {
    if (rc)
    strcat(slist, ", ");
    strcat(slist, "sync_csr");
    rc++;
    }
    if (kvm_riscv_nacl_sync_hfence_available()) {
    if (rc)
    strcat(slist, ", ");
    strcat(slist, "sync_hfence");
    rc++;
    }
    if (kvm_riscv_nacl_sync_sret_available()) {
    if (rc)
    strcat(slist, ", ");
    strcat(slist, "sync_sret");
    rc++;
    }
    if (kvm_riscv_nacl_autoswap_csr_available()) {
    if (rc)
    strcat(slist, ", ");
    strcat(slist, "autoswap_csr");
    rc++;
    }
    kvm_info("using SBI nested acceleration with %s\n",
    (rc) ? slist : "no features");
    }
    kvm_info("highest G-stage page table mode is %s\n", str);
    kvm_info("VMID %ld bits available\n", kvm_riscv_gstage_vmid_bits());
    kvm_riscv_setup_vendor_features();
    kvm_riscv_v_init();
    kvm_register_perf_callbacks();
// Register CPU PM notifier for CPU idle non-retention states
    if (IS_ENABLED(CONFIG_CPU_PM)) {
    rc = cpu_pm_register_notifier(&kvm_riscv_cpu_pm_nb);
    if (rc) {
    kvm_err("Failed to register CPU PM notifier: %d\n", rc);
    goto err_teardown;
    }
    }
    rc = kvm_init(sizeof(struct kvm_vcpu), 0, THIS_MODULE);
    if (rc)
    goto err_unregister_cpu_pm;
    if (kvm_riscv_aia_available())
    kvm_info("AIA available with %d guest external interrupts\n",
    atomic_read(&kvm_riscv_aia_nr_hgei));
    return 0;
    err_unregister_cpu_pm:
    if (IS_ENABLED(CONFIG_CPU_PM))
    cpu_pm_unregister_notifier(&kvm_riscv_cpu_pm_nb);
    err_teardown:
    kvm_riscv_teardown();
    return rc;
    }
    module_init(riscv_kvm_init);
#[no_mangle]
unsafe extern "C" fn riscv_kvm_exit() -> void __exit {
    static void __exit riscv_kvm_exit(void)
    {
    kvm_exit();
// Unregister CPU PM notifier
    if (IS_ENABLED(CONFIG_CPU_PM))
    cpu_pm_unregister_notifier(&kvm_riscv_cpu_pm_nb);
    kvm_riscv_teardown();
    }
    module_exit(riscv_kvm_exit);
