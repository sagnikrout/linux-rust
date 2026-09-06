//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/kvm.c
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
// KVM paravirt_ops implementation
//
// Copyright (C) 2007, Red Hat, Inc., Ingo Molnar <mingo@redhat.com>
// Copyright IBM Corporation, 2007
// Authors: Anthony Liguori <aliguori@us.ibm.com>
//

    DEFINE_STATIC_KEY_FALSE_RO(kvm_async_pf_enabled);
    let mut kvmapf: static int = 1;
#[no_mangle]
unsafe extern "C" fn parse_no_kvmapf(arg: *mut c_char) -> int __init {
    static int __init parse_no_kvmapf(char *arg)
    {
    kvmapf = 0;
    return 0;
    }
    early_param("no-kvmapf", parse_no_kvmapf);
    let mut steal_acc: static int = 1;
#[no_mangle]
unsafe extern "C" fn parse_no_stealacc(arg: *mut c_char) -> int __init {
    static int __init parse_no_stealacc(char *arg)
    {
    steal_acc = 0;
    return 0;
    }
    early_param("no-steal-acc", parse_no_stealacc);
    static DEFINE_PER_CPU_READ_MOSTLY(bool, async_pf_enabled);
    static DEFINE_PER_CPU_DECRYPTED(struct kvm_vcpu_pv_apf_data, apf_reason) __aligned(64);
    DEFINE_PER_CPU_DECRYPTED(struct kvm_steal_time, steal_time) __aligned(64) __visible;
    let mut has_steal_clock: static int = 0;
    let mut has_guest_poll: static int = 0;
pub const KVM_TASK_SLEEP_HASHBITS: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_task_sleep_node {
    pub link: hlist_node,
    pub wq: swait_queue_head,
    pub token: u32,
    pub cpu: c_int,
    pub dummy: bool,
}

    static struct kvm_task_sleep_head {
    raw_spinlock_t lock;
    struct hlist_head list;
    } async_pf_sleepers[KVM_TASK_SLEEP_HASHSIZE];
    static struct kvm_task_sleep_node *_find_apf_task(struct kvm_task_sleep_head *b,
    u32 token)
    {
    struct hlist_node *p;
    hlist_for_each(p, &b.list) {
    struct kvm_task_sleep_node *n =
    hlist_entry(p, typeof(*n), link);
    if (n.token == token)
    return n;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn kvm_async_pf_queue_task(token: u32, n: *mut kvm_task_sleep_node) -> bool {
    static bool kvm_async_pf_queue_task(u32 token, struct kvm_task_sleep_node *n)
    {
    let mut key: u32 = hash_32(token, KVM_TASK_SLEEP_HASHBITS);
    struct kvm_task_sleep_head *b = &async_pf_sleepers[key];
    struct kvm_task_sleep_node *e;
    raw_spin_lock(&b.lock);
    e = _find_apf_task(b, token);
    if (e) {
    struct kvm_task_sleep_node *dummy = core::ptr::null_mut();
//
// The entry can either be a 'dummy' entry (which is put on the
// list when wake-up happens ahead of APF handling completion)
// or a token from another task which should not be touched.
//
    if (e.dummy) {
    hlist_del(&e.link);
    dummy = e;
    }
    raw_spin_unlock(&b.lock);
    kfree(dummy);
    return false;
    }
    n.token = token;
    n.cpu = smp_processor_id();
    n.dummy = false;
    init_swait_queue_head(&n.wq);
    hlist_add_head(&n.link, &b.list);
    raw_spin_unlock(&b.lock);
    return true;
    }
//
// kvm_async_pf_task_wait_schedule - Wait for pagefault to be handled
// @token:	Token to identify the sleep node entry
//
// Invoked from the async pagefault handling code or from the VM exit page
// fault handler. In both cases RCU is watching.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_async_pf_task_wait_schedule(token: u32) {
    void kvm_async_pf_task_wait_schedule(u32 token)
    {
    struct kvm_task_sleep_node n;
    DECLARE_SWAITQUEUE(wait);
    lockdep_assert_irqs_disabled();
    if (!kvm_async_pf_queue_task(token, &n))
    return;
    for (;;) {
    prepare_to_swait_exclusive(&n.wq, &wait, TASK_UNINTERRUPTIBLE);
    if (hlist_unhashed(&n.link))
    break;
    local_irq_enable();
    schedule();
    local_irq_disable();
    }
    finish_swait(&n.wq, &wait);
    }
    EXPORT_SYMBOL_FOR_KVM(kvm_async_pf_task_wait_schedule);
#[no_mangle]
unsafe extern "C" fn apf_task_wake_one(n: *mut kvm_task_sleep_node) {
    static void apf_task_wake_one(struct kvm_task_sleep_node *n)
    {
    hlist_del_init(&n.link);
    if (swq_has_sleeper(&n.wq))
    swake_up_one(&n.wq);
    }
#[no_mangle]
unsafe extern "C" fn apf_task_wake_all() {
    static void apf_task_wake_all(void)
    {
    int i;
    for (i = 0; i < KVM_TASK_SLEEP_HASHSIZE; i++) {
    struct kvm_task_sleep_head *b = &async_pf_sleepers[i];
    struct kvm_task_sleep_node *n;
    struct hlist_node *p, *next;
    raw_spin_lock(&b.lock);
    hlist_for_each_safe(p, next, &b.list) {
    n = hlist_entry(p, typeof(*n), link);
    if (n.cpu == smp_processor_id())
    apf_task_wake_one(n);
    }
    raw_spin_unlock(&b.lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn kvm_async_pf_task_wake(token: u32) {
    static void kvm_async_pf_task_wake(u32 token)
    {
    let mut key: u32 = hash_32(token, KVM_TASK_SLEEP_HASHBITS);
    struct kvm_task_sleep_head *b = &async_pf_sleepers[key];
    struct kvm_task_sleep_node *n, *dummy = core::ptr::null_mut();
    if (token == ~0) {
    apf_task_wake_all();
    return;
    }
    again:
    raw_spin_lock(&b.lock);
    n = _find_apf_task(b, token);
    if (!n) {
//
// Async #PF not yet handled, add a dummy entry for the token.
// Allocating the token must be down outside of the raw lock
// as the allocator is preemptible on PREEMPT_RT kernels.
//
    if (!dummy) {
    raw_spin_unlock(&b.lock);
    dummy = kzalloc_obj(*dummy, GFP_ATOMIC);
//
// Continue looping on allocation failure, eventually
// the async #PF will be handled and allocating a new
// node will be unnecessary.
//
    if (!dummy)
    cpu_relax();
//
// Recheck for async #PF completion before enqueueing
// the dummy token to avoid duplicate list entries.
//
    goto again;
    }
    dummy.token = token;
    dummy.cpu = smp_processor_id();
    dummy.dummy = true;
    init_swait_queue_head(&dummy.wq);
    hlist_add_head(&dummy.link, &b.list);
    dummy = core::ptr::null_mut();
    } else {
    apf_task_wake_one(n);
    }
    raw_spin_unlock(&b.lock);
// A dummy token might be allocated and ultimately not used.
    kfree(dummy);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_read_and_reset_apf_flags() -> noinstr u32 {
    noinstr u32 kvm_read_and_reset_apf_flags(void)
    {
    let mut flags: u32 = 0;
    if (__this_cpu_read(async_pf_enabled)) {
    flags = __this_cpu_read(apf_reason.flags);
    __this_cpu_write(apf_reason.flags, 0);
    }
    return flags;
    }
    EXPORT_SYMBOL_FOR_KVM(kvm_read_and_reset_apf_flags);
#[no_mangle]
pub unsafe extern "C" fn __kvm_handle_async_pf(regs: *mut pt_regs, token: u32) -> noinstr bool {
    noinstr bool __kvm_handle_async_pf(struct pt_regs *regs, u32 token)
    {
    let mut flags: u32 = kvm_read_and_reset_apf_flags();
    irqentry_state_t state;
    if (!flags)
    return false;
    state = irqentry_enter(regs);
    instrumentation_begin();
//
// If the host managed to inject an async #PF into an interrupt
// disabled region, then die hard as this is not going to end well
// and the host side is seriously broken.
//
    if (unlikely(!(regs.flags & X86_EFLAGS_IF)))
    panic("Host injected async #PF in interrupt disabled region\n");
    if (flags & KVM_PV_REASON_PAGE_NOT_PRESENT) {
    if (unlikely(!(user_mode(regs))))
    panic("Host injected async #PF in kernel mode\n");
// Page is swapped out by the host.
    kvm_async_pf_task_wait_schedule(token);
    } else {
    WARN_ONCE(1, "Unexpected async PF flags: %x\n", flags);
    }
    instrumentation_end();
    irqentry_exit(regs, state);
    return true;
    }
    DEFINE_IDTENTRY_SYSVEC(sysvec_kvm_asyncpf_interrupt)
    {
    struct pt_regs *old_regs = set_irq_regs(regs);
    u32 token;
    apic_eoi();
    inc_irq_stat(HYPERVISOR_CALLBACK);
    if (__this_cpu_read(async_pf_enabled)) {
    token = __this_cpu_read(apf_reason.token);
    kvm_async_pf_task_wake(token);
    __this_cpu_write(apf_reason.token, 0);
    wrmsrq(MSR_KVM_ASYNC_PF_ACK, 1);
    }
    set_irq_regs(old_regs);
    }
#[no_mangle]
unsafe extern "C" fn paravirt_ops_setup() -> void __init {
    static void __init paravirt_ops_setup(void)
    {
    pv_info.name = "KVM";
    if (kvm_para_has_feature(KVM_FEATURE_NOP_IO_DELAY))
    pv_info.io_delay = false;

    no_timer_check = 1;

    }
#[no_mangle]
unsafe extern "C" fn kvm_register_steal_time() {
    static void kvm_register_steal_time(void)
    {
    let mut cpu: c_int = smp_processor_id();
    struct kvm_steal_time *st = &per_cpu(steal_time, cpu);
    if (!has_steal_clock)
    return;
    wrmsrq(MSR_KVM_STEAL_TIME, (slow_virt_to_phys(st) | KVM_MSR_ENABLED));
    pr_debug("stealtime: cpu %d, msr %llx\n", cpu,
    (unsigned long long) slow_virt_to_phys(st));
    }
    static DEFINE_PER_CPU_DECRYPTED(unsigned long, kvm_apic_eoi) = KVM_PV_EOI_DISABLED;
#[no_mangle]
unsafe extern "C" fn kvm_guest_apic_eoi_write() -> notrace __maybe_unused void {
    static notrace __maybe_unused void kvm_guest_apic_eoi_write(void)
    {
//
// This relies on __test_and_clear_bit to modify the memory
// in a way that is atomic with respect to the local CPU.
// The hypervisor only accesses this memory from the local CPU so
// there's no need for lock or memory barriers.
// An optimization barrier is implied in apic write.
//
    if (__test_and_clear_bit(KVM_PV_EOI_BIT, this_cpu_ptr(&kvm_apic_eoi)))
    return;
    apic_native_eoi();
    }
#[no_mangle]
unsafe extern "C" fn kvm_guest_cpu_init() {
    static void kvm_guest_cpu_init(void)
    {
    if (kvm_para_has_feature(KVM_FEATURE_ASYNC_PF_INT) && kvmapf) {
    u64 pa;
    WARN_ON_ONCE(!static_branch_likely(&kvm_async_pf_enabled));
    pa = slow_virt_to_phys(this_cpu_ptr(&apf_reason));
    pa |= KVM_ASYNC_PF_ENABLED | KVM_ASYNC_PF_DELIVERY_AS_INT;
    if (kvm_para_has_feature(KVM_FEATURE_ASYNC_PF_VMEXIT))
    pa |= KVM_ASYNC_PF_DELIVERY_AS_PF_VMEXIT;
    wrmsrq(MSR_KVM_ASYNC_PF_INT, HYPERVISOR_CALLBACK_VECTOR);
    wrmsrq(MSR_KVM_ASYNC_PF_EN, pa);
    __this_cpu_write(async_pf_enabled, true);
    pr_debug("setup async PF for cpu %d\n", smp_processor_id());
    }
    if (kvm_para_has_feature(KVM_FEATURE_PV_EOI)) {
    unsigned long pa;
// Size alignment is implied but just to make it explicit.
    BUILD_BUG_ON(__alignof__(kvm_apic_eoi) < 4);
    __this_cpu_write(kvm_apic_eoi, 0);
    pa = slow_virt_to_phys(this_cpu_ptr(&kvm_apic_eoi))
    | KVM_MSR_ENABLED;
    wrmsrq(MSR_KVM_PV_EOI_EN, pa);
    }
    if (has_steal_clock)
    kvm_register_steal_time();
    }
#[no_mangle]
unsafe extern "C" fn kvm_pv_disable_apf() {
    static void kvm_pv_disable_apf(void)
    {
    if (!__this_cpu_read(async_pf_enabled))
    return;
    wrmsrq(MSR_KVM_ASYNC_PF_EN, 0);
    __this_cpu_write(async_pf_enabled, false);
    pr_debug("disable async PF for cpu %d\n", smp_processor_id());
    }
#[no_mangle]
unsafe extern "C" fn kvm_disable_steal_time() {
    static void kvm_disable_steal_time(void)
    {
    if (!has_steal_clock)
    return;
    wrmsrq(MSR_KVM_STEAL_TIME, 0);
    }
#[no_mangle]
unsafe extern "C" fn kvm_steal_clock(cpu: c_int) -> u64 {
    static u64 kvm_steal_clock(int cpu)
    {
    u64 steal;
    struct kvm_steal_time *src;
    int version;
    src = &per_cpu(steal_time, cpu);
    do {
    version = src.version;
    virt_rmb();
    steal = src.steal;
    virt_rmb();
    } while ((version & 1) || (version != src.version));
    return steal;
    }
#[no_mangle]
pub unsafe extern "C" fn __set_percpu_decrypted(ptr: *mut c_void, size: c_ulong) -> __init void {
    static inline __init void __set_percpu_decrypted(void *ptr, unsigned long size)
    {
    early_set_memory_decrypted((unsigned long) ptr, size);
    }
//
// Iterate through all possible CPUs and map the memory region pointed
// by apf_reason, steal_time and kvm_apic_eoi as decrypted at once.
//
// Note: we iterate through all possible CPUs to ensure that CPUs
// hotplugged will have their per-cpu variable already mapped as
// decrypted.
//
#[no_mangle]
unsafe extern "C" fn sev_map_percpu_data() -> void __init {
    static void __init sev_map_percpu_data(void)
    {
    int cpu;
    if (cc_vendor != CC_VENDOR_AMD ||
    !cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT))
    return;
    for_each_possible_cpu(cpu) {
    __set_percpu_decrypted(&per_cpu(apf_reason, cpu), sizeof(apf_reason));
    __set_percpu_decrypted(&per_cpu(steal_time, cpu), sizeof(steal_time));
    __set_percpu_decrypted(&per_cpu(kvm_apic_eoi, cpu), sizeof(kvm_apic_eoi));
    }
    }
#[no_mangle]
unsafe extern "C" fn kvm_guest_cpu_offline(shutdown: bool) {
    static void kvm_guest_cpu_offline(bool shutdown)
    {
    kvm_disable_steal_time();
    if (kvm_para_has_feature(KVM_FEATURE_PV_EOI))
    wrmsrq(MSR_KVM_PV_EOI_EN, 0);
    if (kvm_para_has_feature(KVM_FEATURE_MIGRATION_CONTROL))
    wrmsrq(MSR_KVM_MIGRATION_CONTROL, 0);
    kvm_pv_disable_apf();
    if (!shutdown)
    apf_task_wake_all();
    kvmclock_disable();
    }
#[no_mangle]
unsafe extern "C" fn kvm_cpu_online(cpu: c_uint) -> c_int {
    static int kvm_cpu_online(unsigned int cpu)
    {
    unsigned long flags;
    local_irq_save(flags);
    kvm_guest_cpu_init();
    local_irq_restore(flags);
    return 0;
    }

    static DEFINE_PER_CPU(cpumask_var_t, __pv_cpu_mask);
#[no_mangle]
unsafe extern "C" fn pv_tlb_flush_supported() -> bool {
    static bool pv_tlb_flush_supported(void)
    {
    return (kvm_para_has_feature(KVM_FEATURE_PV_TLB_FLUSH) &&
    !kvm_para_has_hint(KVM_HINTS_REALTIME) &&
    kvm_para_has_feature(KVM_FEATURE_STEAL_TIME) &&
    !boot_cpu_has(X86_FEATURE_MWAIT) &&
    (num_possible_cpus() != 1));
    }
#[no_mangle]
unsafe extern "C" fn pv_ipi_supported() -> bool {
    static bool pv_ipi_supported(void)
    {
    return (kvm_para_has_feature(KVM_FEATURE_PV_SEND_IPI) &&
    (num_possible_cpus() != 1));
    }
#[no_mangle]
unsafe extern "C" fn pv_sched_yield_supported() -> bool {
    static bool pv_sched_yield_supported(void)
    {
    return (kvm_para_has_feature(KVM_FEATURE_PV_SCHED_YIELD) &&
    !kvm_para_has_hint(KVM_HINTS_REALTIME) &&
    kvm_para_has_feature(KVM_FEATURE_STEAL_TIME) &&
    !boot_cpu_has(X86_FEATURE_MWAIT) &&
    (num_possible_cpus() != 1));
    }

#[no_mangle]
unsafe extern "C" fn __send_ipi_mask(mask: *const cpumask, vector: c_int) {
    static void __send_ipi_mask(const struct cpumask *mask, int vector)
    {
    unsigned long flags;
    int cpu, min = 0, max = 0;

    let mut ipi_bitmap: __uint128_t = 0;

    let mut ipi_bitmap: u64 = 0;

    u32 apic_id, icr;
    long ret;
    if (cpumask_empty(mask))
    return;
    local_irq_save(flags);
    switch (vector) {
    default:
    icr = APIC_DM_FIXED | vector;
    break;
    case NMI_VECTOR:
    icr = APIC_DM_NMI;
    break;
    }
    for_each_cpu(cpu, mask) {
    apic_id = per_cpu(x86_cpu_to_apicid, cpu);
    if (!ipi_bitmap) {
    min = max = apic_id;
    } else if (apic_id < min && max - apic_id < KVM_IPI_CLUSTER_SIZE) {
    ipi_bitmap <<= min - apic_id;
    min = apic_id;
    } else if (apic_id > min && apic_id < min + KVM_IPI_CLUSTER_SIZE) {
    max = apic_id < max ? max : apic_id;
    } else {
    ret = kvm_hypercall4(KVM_HC_SEND_IPI, (unsigned long)ipi_bitmap,
    (unsigned long)(ipi_bitmap >> BITS_PER_LONG), min, icr);
    WARN_ONCE(ret < 0, "kvm-guest: failed to send PV IPI: %ld",
    ret);
    min = max = apic_id;
    ipi_bitmap = 0;
    }
    __set_bit(apic_id - min, (unsigned long *)&ipi_bitmap);
    }
    if (ipi_bitmap) {
    ret = kvm_hypercall4(KVM_HC_SEND_IPI, (unsigned long)ipi_bitmap,
    (unsigned long)(ipi_bitmap >> BITS_PER_LONG), min, icr);
    WARN_ONCE(ret < 0, "kvm-guest: failed to send PV IPI: %ld",
    ret);
    }
    local_irq_restore(flags);
    }
#[no_mangle]
unsafe extern "C" fn kvm_send_ipi_mask(mask: *const cpumask, vector: c_int) {
    static void kvm_send_ipi_mask(const struct cpumask *mask, int vector)
    {
    __send_ipi_mask(mask, vector);
    }
#[no_mangle]
unsafe extern "C" fn kvm_send_ipi_mask_allbutself(mask: *const cpumask, vector: c_int) {
    static void kvm_send_ipi_mask_allbutself(const struct cpumask *mask, int vector)
    {
    let mut this_cpu: c_uint = smp_processor_id();
    struct cpumask *new_mask = this_cpu_cpumask_var_ptr(__pv_cpu_mask);
    const struct cpumask *local_mask;
    cpumask_copy(new_mask, mask);
    cpumask_clear_cpu(this_cpu, new_mask);
    local_mask = new_mask;
    __send_ipi_mask(local_mask, vector);
    }
#[no_mangle]
unsafe extern "C" fn setup_efi_kvm_sev_migration() -> int __init {
    static int __init setup_efi_kvm_sev_migration(void)
    {
    efi_char16_t efi_sev_live_migration_enabled[] = L"SevLiveMigrationEnabled";
    let mut efi_variable_guid: efi_guid_t = AMD_SEV_MEM_ENCRYPT_GUID;
    efi_status_t status;
    unsigned long size;
    bool enabled;
    if (!cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) ||
    !kvm_para_has_feature(KVM_FEATURE_MIGRATION_CONTROL))
    return 0;
    if (!efi_enabled(EFI_BOOT))
    return 0;
    if (!efi_enabled(EFI_RUNTIME_SERVICES)) {
    pr_info("%s : EFI runtime services are not enabled\n", __func__);
    return 0;
    }
    size = sizeof(enabled);
// Get variable contents into buffer
    status = efi.get_variable(efi_sev_live_migration_enabled,
    &efi_variable_guid, core::ptr::null_mut(), &size, &enabled);
    if (status == EFI_NOT_FOUND) {
    pr_info("%s : EFI live migration variable not found\n", __func__);
    return 0;
    }
    if (status != EFI_SUCCESS) {
    pr_info("%s : EFI variable retrieval failed\n", __func__);
    return 0;
    }
    if (enabled == 0) {
    pr_info("%s: live migration disabled in EFI\n", __func__);
    return 0;
    }
    pr_info("%s : live migration enabled in EFI\n", __func__);
    wrmsrq(MSR_KVM_MIGRATION_CONTROL, KVM_MIGRATION_READY);
    return 1;
    }
    late_initcall(setup_efi_kvm_sev_migration);
//
// Set the IPI entry points
//
#[no_mangle]
unsafe extern "C" fn kvm_setup_pv_ipi() -> __init void {
    static __init void kvm_setup_pv_ipi(void)
    {
    apic_update_callback(send_IPI_mask, kvm_send_ipi_mask);
    apic_update_callback(send_IPI_mask_allbutself, kvm_send_ipi_mask_allbutself);
    pr_info("setup PV IPIs\n");
    }
#[no_mangle]
unsafe extern "C" fn kvm_smp_send_call_func_ipi(mask: *const cpumask) {
    static void kvm_smp_send_call_func_ipi(const struct cpumask *mask)
    {
    int cpu;
    native_send_call_func_ipi(mask);
// Make sure other vCPUs get a chance to run if they need to.
    for_each_cpu(cpu, mask) {
    if (!idle_cpu(cpu) && vcpu_is_preempted(cpu)) {
    kvm_hypercall1(KVM_HC_SCHED_YIELD, per_cpu(x86_cpu_to_apicid, cpu));
    break;
    }
    }
    }
    static void kvm_flush_tlb_multi(const struct cpumask *cpumask,
    const struct flush_tlb_info *info)
    {
    u8 state;
    int cpu;
    struct kvm_steal_time *src;
    struct cpumask *flushmask;
    guard(preempt)();
    flushmask = this_cpu_cpumask_var_ptr(__pv_cpu_mask);
    cpumask_copy(flushmask, cpumask);
//
// We have to call flush only on online vCPUs. And
// queue flush_on_enter for pre-empted vCPUs
//
    for_each_cpu(cpu, flushmask) {
//
// The local vCPU is never preempted, so we do not explicitly
// skip check for local vCPU - it will never be cleared from
// flushmask.
//
    src = &per_cpu(steal_time, cpu);
    state = READ_ONCE(src.preempted);
    if ((state & KVM_VCPU_PREEMPTED)) {
    if (try_cmpxchg(&src.preempted, &state,
    state | KVM_VCPU_FLUSH_TLB))
    __cpumask_clear_cpu(cpu, flushmask);
    }
    }
    native_flush_tlb_multi(flushmask, info);
    }
#[no_mangle]
unsafe extern "C" fn kvm_alloc_cpumask() -> __init int {
    static __init int kvm_alloc_cpumask(void)
    {
    int cpu;
    if (!kvm_para_available() || nopv)
    return 0;
    if (pv_tlb_flush_supported() || pv_ipi_supported())
    for_each_possible_cpu(cpu) {
    zalloc_cpumask_var_node(per_cpu_ptr(&__pv_cpu_mask, cpu),
    GFP_KERNEL, cpu_to_node(cpu));
    }
    return 0;
    }
    arch_initcall(kvm_alloc_cpumask);
#[no_mangle]
unsafe extern "C" fn kvm_smp_prepare_boot_cpu() -> void __init {
    static void __init kvm_smp_prepare_boot_cpu(void)
    {
//
// Map the per-cpu variables as decrypted before kvm_guest_cpu_init()
// shares the guest physical address with the hypervisor.
//
    sev_map_percpu_data();
    kvm_guest_cpu_init();
    native_smp_prepare_boot_cpu();
    kvm_spinlock_init();
    }
#[no_mangle]
unsafe extern "C" fn kvm_cpu_down_prepare(cpu: c_uint) -> c_int {
    static int kvm_cpu_down_prepare(unsigned int cpu)
    {
    unsigned long flags;
    local_irq_save(flags);
    kvm_guest_cpu_offline(false);
    local_irq_restore(flags);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn kvm_suspend(data: *mut c_void) -> c_int {
    static int kvm_suspend(void *data)
    {
    let mut val: u64 = 0;
    kvm_guest_cpu_offline(false);

    if (kvm_para_has_feature(KVM_FEATURE_POLL_CONTROL))
    rdmsrq(MSR_KVM_POLL_CONTROL, val);
    has_guest_poll = !(val & 1);

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kvm_resume(data: *mut c_void) {
    static void kvm_resume(void *data)
    {
    kvm_cpu_online(raw_smp_processor_id());

    if (kvm_para_has_feature(KVM_FEATURE_POLL_CONTROL) && has_guest_poll)
    wrmsrq(MSR_KVM_POLL_CONTROL, 0);

    }
    static const struct syscore_ops kvm_syscore_ops = {
    .suspend	= kvm_suspend,
    .resume		= kvm_resume,
    };
    static struct syscore kvm_syscore = {
    .ops = &kvm_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn kvm_pv_guest_cpu_reboot(unused: *mut c_void) {
    static void kvm_pv_guest_cpu_reboot(void *unused)
    {
    kvm_guest_cpu_offline(true);
    }
    static int kvm_pv_reboot_notify(struct notifier_block *nb,
    unsigned long code, void *unused)
    {
    if (code == SYS_RESTART)
    on_each_cpu(kvm_pv_guest_cpu_reboot, core::ptr::null_mut(), 1);
    return NOTIFY_DONE;
    }
    static struct notifier_block kvm_pv_reboot_nb = {
    .notifier_call = kvm_pv_reboot_notify,
    };
//
// After a PV feature is registered, the host will keep writing to the
// registered memory location. If the guest happens to shutdown, this memory
// won't be valid. In cases like kexec, in which you install a new kernel, this
// means a random memory location will be kept being written.
//

#[no_mangle]
unsafe extern "C" fn kvm_crash_shutdown(regs: *mut pt_regs) {
    static void kvm_crash_shutdown(struct pt_regs *regs)
    {
    kvm_guest_cpu_offline(true);
    native_machine_crash_shutdown(regs);
    }

    bool __kvm_vcpu_is_preempted(long cpu);
#[no_mangle]
pub unsafe extern "C" fn __kvm_vcpu_is_preempted(cpu: c_long) -> __visible bool {
    __visible bool __kvm_vcpu_is_preempted(long cpu)
    {
    struct kvm_steal_time *src = &per_cpu(steal_time, cpu);
    return !!(src.preempted & KVM_VCPU_PREEMPTED);
    }
    PV_CALLEE_SAVE_REGS_THUNK(__kvm_vcpu_is_preempted);

    extern bool __raw_callee_save___kvm_vcpu_is_preempted(long);
//
// Hand-optimize version for x86-64 to avoid 8 64-bit register saving and
// restoring to/from the stack.
//

    "movq   __per_cpu_offset(,%rdi,8), %rax\n\t"				     \
    "cmpb   $0, " __stringify(KVM_STEAL_TIME_preempted) "+steal_time(%rax)\n\t" \
    "setne  %al\n\t"
    DEFINE_ASM_FUNC(__raw_callee_save___kvm_vcpu_is_preempted,
    PV_VCPU_PREEMPTED_ASM, .text);

#[no_mangle]
unsafe extern "C" fn kvm_guest_init() -> void __init {
    static void __init kvm_guest_init(void)
    {
    int i;
    paravirt_ops_setup();
    register_reboot_notifier(&kvm_pv_reboot_nb);
    for (i = 0; i < KVM_TASK_SLEEP_HASHSIZE; i++)
    raw_spin_lock_init(&async_pf_sleepers[i].lock);
    if (kvm_para_has_feature(KVM_FEATURE_STEAL_TIME)) {
    has_steal_clock = 1;
    static_call_update(pv_steal_clock, kvm_steal_clock);

    pv_ops_lock.vcpu_is_preempted =
    PV_CALLEE_SAVE(__kvm_vcpu_is_preempted);

    }
    if (kvm_para_has_feature(KVM_FEATURE_PV_EOI))
    apic_update_callback(eoi, kvm_guest_apic_eoi_write);
    if (kvm_para_has_feature(KVM_FEATURE_ASYNC_PF_INT) && kvmapf) {
    static_branch_enable(&kvm_async_pf_enabled);
    sysvec_install(HYPERVISOR_CALLBACK_VECTOR, sysvec_kvm_asyncpf_interrupt);
    }

    if (pv_tlb_flush_supported()) {
    pv_ops.mmu.flush_tlb_multi = kvm_flush_tlb_multi;
    pr_info("KVM setup pv remote TLB flush\n");
    }
    smp_ops.smp_prepare_boot_cpu = kvm_smp_prepare_boot_cpu;
    if (pv_sched_yield_supported()) {
    smp_ops.send_call_func_ipi = kvm_smp_send_call_func_ipi;
    pr_info("setup PV sched yield\n");
    }
    if (cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN, "x86/kvm:online",
    kvm_cpu_online, kvm_cpu_down_prepare) < 0)
    pr_err("failed to install cpu hotplug callbacks\n");

    sev_map_percpu_data();
    kvm_guest_cpu_init();

    machine_ops.crash_shutdown = kvm_crash_shutdown;

    register_syscore(&kvm_syscore);
//
// Hard lockup detection is enabled by default. Disable it, as guests
// can get false positives too easily, for example if the host is
// overcommitted.
//
    hardlockup_detector_disable();
    }
#[no_mangle]
unsafe extern "C" fn __kvm_cpuid_base() -> noinline uint32_t {
    static noinline uint32_t __kvm_cpuid_base(void)
    {
    if (boot_cpu_data.cpuid_level < 0)
    return 0;	/* So we don't blow up on old processors */
    if (boot_cpu_has(X86_FEATURE_HYPERVISOR))
    return cpuid_base_hypervisor(KVM_SIGNATURE, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_cpuid_base() -> u32 {
    static inline uint32_t kvm_cpuid_base(void)
    {
    let mut kvm_cpuid_base: static int = -1;
    if (kvm_cpuid_base == -1)
    kvm_cpuid_base = __kvm_cpuid_base();
    return kvm_cpuid_base;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_para_available() -> bool {
    bool kvm_para_available(void)
    {
    return kvm_cpuid_base() != 0;
    }
    EXPORT_SYMBOL_GPL(kvm_para_available);
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_para_features() -> c_uint {
    unsigned int kvm_arch_para_features(void)
    {
    return cpuid_eax(kvm_cpuid_base() | KVM_CPUID_FEATURES);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_para_hints() -> c_uint {
    unsigned int kvm_arch_para_hints(void)
    {
    return cpuid_edx(kvm_cpuid_base() | KVM_CPUID_FEATURES);
    }
    EXPORT_SYMBOL_GPL(kvm_arch_para_hints);
#[no_mangle]
unsafe extern "C" fn kvm_detect() -> uint32_t __init {
    static uint32_t __init kvm_detect(void)
    {
    return kvm_cpuid_base();
    }
#[no_mangle]
unsafe extern "C" fn kvm_apic_init() -> void __init {
    static void __init kvm_apic_init(void)
    {

    if (pv_ipi_supported())
    kvm_setup_pv_ipi();

    }
#[no_mangle]
unsafe extern "C" fn kvm_msi_ext_dest_id() -> bool __init {
    static bool __init kvm_msi_ext_dest_id(void)
    {
    return kvm_para_has_feature(KVM_FEATURE_MSI_EXT_DEST_ID);
    }
#[no_mangle]
unsafe extern "C" fn kvm_sev_hc_page_enc_status(pfn: c_ulong, npages: c_int, enc: bool) {
    static void kvm_sev_hc_page_enc_status(unsigned long pfn, int npages, bool enc)
    {
    kvm_sev_hypercall3(KVM_HC_MAP_GPA_RANGE, pfn << PAGE_SHIFT, npages,
    KVM_MAP_GPA_RANGE_ENC_STAT(enc) | KVM_MAP_GPA_RANGE_PAGE_SZ_4K);
    }
#[no_mangle]
unsafe extern "C" fn kvm_init_platform() -> void __init {
    static void __init kvm_init_platform(void)
    {
    let mut tolud: u64 = PFN_PHYS(e820__end_of_low_ram_pfn());
//
// Note, hardware requires variable MTRR ranges to be power-of-2 sized
// and naturally aligned.  But when forcing guest MTRR state, Linux
// doesn't program the forced ranges into hardware.  Don't bother doing
// the math to generate a technically-legal range.
//
    struct mtrr_var_range pci_hole = {
    .base_lo = tolud | X86_MEMTYPE_UC,
    .mask_lo = (u32)(~(SZ_4G - tolud - 1)) | MTRR_PHYSMASK_V,
    .mask_hi = (BIT_ULL(boot_cpu_data.x86_phys_bits) - 1) >> 32,
    };
    if (cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT) &&
    kvm_para_has_feature(KVM_FEATURE_MIGRATION_CONTROL)) {
    unsigned long nr_pages;
    int i;
    pv_ops.mmu.notify_page_enc_status_changed =
    kvm_sev_hc_page_enc_status;
//
// Reset the host's shared pages list related to kernel
// specific page encryption status settings before we load a
// new kernel by kexec. Reset the page encryption status
// during early boot instead of just before kexec to avoid SMP
// races during kvm_pv_guest_cpu_reboot().
// NOTE: We cannot reset the complete shared pages list
// here as we need to retain the UEFI/OVMF firmware
// specific settings.
//
    for (i = 0; i < e820_table.nr_entries; i++) {
    struct e820_entry *entry = &e820_table.entries[i];
    if (entry.type != E820_TYPE_RAM)
    continue;
    nr_pages = DIV_ROUND_UP(entry.size, PAGE_SIZE);
    kvm_sev_hypercall3(KVM_HC_MAP_GPA_RANGE, entry.addr,
    nr_pages,
    KVM_MAP_GPA_RANGE_ENCRYPTED | KVM_MAP_GPA_RANGE_PAGE_SZ_4K);
    }
//
// Ensure that _bss_decrypted section is marked as decrypted in the
// shared pages list.
//
    early_set_mem_enc_dec_hypercall((unsigned long)__start_bss_decrypted,
    __end_bss_decrypted - __start_bss_decrypted, 0);
//
// If not booted using EFI, enable Live migration support.
//
    if (!efi_enabled(EFI_BOOT))
    wrmsrq(MSR_KVM_MIGRATION_CONTROL,
    KVM_MIGRATION_READY);
    }
    kvmclock_init();
    x86_platform.apic_post_init = kvm_apic_init;
//
// Set WB as the default cache mode for SEV-SNP and TDX, with a single
// UC range for the legacy PCI hole, e.g. so that devices that expect
// to get UC/WC mappings don't get surprised with WB.
//
    guest_force_mtrr_state(&pci_hole, 1, MTRR_TYPE_WRBACK);
    }

#[no_mangle]
unsafe extern "C" fn kvm_sev_es_hcall_prepare(ghcb: *mut ghcb, regs: *mut pt_regs) {
    static void kvm_sev_es_hcall_prepare(struct ghcb *ghcb, struct pt_regs *regs)
    {
// RAX and CPL are already in the GHCB
    ghcb_set_rbx(ghcb, regs.bx);
    ghcb_set_rcx(ghcb, regs.cx);
    ghcb_set_rdx(ghcb, regs.dx);
    ghcb_set_rsi(ghcb, regs.si);
    }
#[no_mangle]
unsafe extern "C" fn kvm_sev_es_hcall_finish(ghcb: *mut ghcb, regs: *mut pt_regs) -> bool {
    static bool kvm_sev_es_hcall_finish(struct ghcb *ghcb, struct pt_regs *regs)
    {
// No checking of the return state needed
    return true;
    }

    const __initconst struct hypervisor_x86 x86_hyper_kvm = {
    .name				= "KVM",
    .detect				= kvm_detect,
    .type				= X86_HYPER_KVM,
    .init.guest_late_init		= kvm_guest_init,
    .init.x2apic_available		= kvm_para_available,
    .init.msi_ext_dest_id		= kvm_msi_ext_dest_id,
    .init.init_platform		= kvm_init_platform,

    .runtime.sev_es_hcall_prepare	= kvm_sev_es_hcall_prepare,
    .runtime.sev_es_hcall_finish	= kvm_sev_es_hcall_finish,

    };
#[no_mangle]
unsafe extern "C" fn activate_jump_labels() -> __init int {
    static __init int activate_jump_labels(void)
    {
    if (has_steal_clock) {
    static_key_slow_inc(&paravirt_steal_enabled);
    if (steal_acc)
    static_key_slow_inc(&paravirt_steal_rq_enabled);
    }
    return 0;
    }
    arch_initcall(activate_jump_labels);

// Kick a cpu by its apicid. Used to wake up a halted vcpu
#[no_mangle]
unsafe extern "C" fn kvm_kick_cpu(cpu: c_int) {
    static void kvm_kick_cpu(int cpu)
    {
    let mut flags: c_ulong = 0;
    u32 apicid;
    apicid = per_cpu(x86_cpu_to_apicid, cpu);
    kvm_hypercall2(KVM_HC_KICK_CPU, flags, apicid);
    }

#[no_mangle]
unsafe extern "C" fn kvm_wait(ptr: *mut u8, val: u8) {
    static void kvm_wait(u8 *ptr, u8 val)
    {
    if (in_nmi())
    return;
//
// halt until it's our turn and kicked. Note that we do safe halt
// for irq enabled case to avoid hang when lock info is overwritten
// in irq spinlock slowpath and no spurious interrupt occur to save us.
//
    if (irqs_disabled()) {
    if (READ_ONCE(*ptr) == val)
    halt();
    } else {
    local_irq_disable();
// safe_halt() will enable IRQ
    if (READ_ONCE(*ptr) == val)
    safe_halt();
    else
    local_irq_enable();
    }
    }
//
// Setup pv_lock_ops to exploit KVM_FEATURE_PV_UNHALT if present.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_spinlock_init() -> void __init {
    void __init kvm_spinlock_init(void)
    {
//
// Disable PV spinlocks and use native qspinlock when dedicated pCPUs
// are available.
//
    if (kvm_para_has_hint(KVM_HINTS_REALTIME)) {
    pr_info("PV spinlocks disabled with KVM_HINTS_REALTIME hints\n");
    goto out;
    }
    if (num_possible_cpus() == 1) {
    pr_info("PV spinlocks disabled, single CPU\n");
    goto out;
    }
    if (nopvspin) {
    pr_info("PV spinlocks disabled, forced by \"nopvspin\" parameter\n");
    goto out;
    }
//
// In case host doesn't support KVM_FEATURE_PV_UNHALT there is still an
// advantage of keeping virt_spin_lock_key enabled: virt_spin_lock() is
// preferred over native qspinlock when vCPU is preempted.
//
    if (!kvm_para_has_feature(KVM_FEATURE_PV_UNHALT)) {
    pr_info("PV spinlocks disabled, no host support\n");
    return;
    }
    pr_info("PV spinlocks enabled\n");
    __pv_init_lock_hash();
    static_call_update(queued_spin_lock_slowpath, __pv_queued_spin_lock_slowpath);
    static_call_update(queued_spin_unlock, __raw_callee_save___pv_queued_spin_unlock);
    pv_ops_lock.wait = kvm_wait;
    pv_ops_lock.kick = kvm_kick_cpu;
//
// When PV spinlock is enabled which is preferred over
// virt_spin_lock(), virt_spin_lock_key's value is meaningless.
// Just disable it anyway.
//
    out:
    static_branch_disable(&virt_spin_lock_key);
    }

#[no_mangle]
unsafe extern "C" fn kvm_disable_host_haltpoll(i: *mut c_void) {
    static void kvm_disable_host_haltpoll(void *i)
    {
    wrmsrq(MSR_KVM_POLL_CONTROL, 0);
    }
#[no_mangle]
unsafe extern "C" fn kvm_enable_host_haltpoll(i: *mut c_void) {
    static void kvm_enable_host_haltpoll(void *i)
    {
    wrmsrq(MSR_KVM_POLL_CONTROL, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_haltpoll_enable(cpu: c_uint) {
    void arch_haltpoll_enable(unsigned int cpu)
    {
    if (!kvm_para_has_feature(KVM_FEATURE_POLL_CONTROL)) {
    pr_err_once("host does not support poll control\n");
    pr_err_once("host upgrade recommended\n");
    return;
    }
// Enable guest halt poll disables host halt poll
    smp_call_function_single(cpu, kvm_disable_host_haltpoll, core::ptr::null_mut(), 1);
    }
    EXPORT_SYMBOL_GPL(arch_haltpoll_enable);
#[no_mangle]
pub unsafe extern "C" fn arch_haltpoll_disable(cpu: c_uint) {
    void arch_haltpoll_disable(unsigned int cpu)
    {
    if (!kvm_para_has_feature(KVM_FEATURE_POLL_CONTROL))
    return;
// Disable guest halt poll enables host halt poll
    smp_call_function_single(cpu, kvm_enable_host_haltpoll, core::ptr::null_mut(), 1);
    }
    EXPORT_SYMBOL_GPL(arch_haltpoll_disable);
