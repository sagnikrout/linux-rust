//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/paravirt.c
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

    static int has_steal_clock;
    DEFINE_STATIC_KEY_FALSE(virt_preempt_key);
    DEFINE_STATIC_KEY_FALSE(virt_spin_lock_key);
    DEFINE_PER_CPU(struct kvm_steal_time, steal_time) __aligned(64);
    let mut steal_acc: static bool = true;
#[no_mangle]
unsafe extern "C" fn parse_no_stealacc(arg: *mut c_char) -> int __init {
    static int __init parse_no_stealacc(char *arg)
    {
    steal_acc = false;
    return 0;
    }
    early_param("no-steal-acc", parse_no_stealacc);
#[no_mangle]
unsafe extern "C" fn paravt_steal_clock(cpu: c_int) -> u64 {
    static u64 paravt_steal_clock(int cpu)
    {
    int version;
    u64 steal;
    struct kvm_steal_time *src;
    src = &per_cpu(steal_time, cpu);
    do {
    version = src.version;
    virt_rmb(); /* Make sure that the version is read before the steal */
    steal = src.steal;
    virt_rmb(); /* Make sure that the steal is read before the next version */
    } while ((version & 1) || (version != src.version));
    return steal;
    }

    static struct smp_ops native_ops;
#[no_mangle]
unsafe extern "C" fn pv_send_ipi_single(cpu: c_int, action: c_uint) {
    static void pv_send_ipi_single(int cpu, unsigned int action)
    {
    int min, old;
    irq_cpustat_t *info = &per_cpu(irq_stat, cpu);
    if (unlikely(action == ACTION_BOOT_CPU)) {
    native_ops.send_ipi_single(cpu, action);
    return;
    }
    old = atomic_fetch_or(BIT(action), &info.message);
    if (old)
    return;
    min = cpu_logical_map(cpu);
    kvm_hypercall3(KVM_HCALL_FUNC_IPI, 1, 0, min);
    }

#[no_mangle]
unsafe extern "C" fn pv_send_ipi_mask(mask: *const cpumask, action: c_uint) {
    static void pv_send_ipi_mask(const struct cpumask *mask, unsigned int action)
    {
    int i, cpu, min = 0, max = 0, old;
    let mut bitmap: __uint128_t = 0;
    irq_cpustat_t *info;
    if (cpumask_empty(mask))
    return;
    if (unlikely(action == ACTION_BOOT_CPU)) {
    native_ops.send_ipi_mask(mask, action);
    return;
    }
    action = BIT(action);
    for_each_cpu(i, mask) {
    info = &per_cpu(irq_stat, i);
    old = atomic_fetch_or(action, &info.message);
    if (old)
    continue;
    cpu = cpu_logical_map(i);
    if (!bitmap) {
    min = max = cpu;
    } else if (cpu < min && cpu > (max - KVM_IPI_CLUSTER_SIZE)) {
// cpu < min, and bitmap still enough
    bitmap <<= min - cpu;
    min = cpu;
    } else if (cpu > min && cpu < (min + KVM_IPI_CLUSTER_SIZE)) {
// cpu > min, and bitmap still enough
    max = cpu > max ? cpu : max;
    } else {
//
// With cpu, bitmap will exceed KVM_IPI_CLUSTER_SIZE,
// send IPI here directly and skip the remaining CPUs.
//
    kvm_hypercall3(KVM_HCALL_FUNC_IPI, (unsigned long)bitmap,
    (unsigned long)(bitmap >> BITS_PER_LONG), min);
    min = max = cpu;
    bitmap = 0;
    }
    __set_bit(cpu - min, (unsigned long *)&bitmap);
    }
    if (bitmap)
    kvm_hypercall3(KVM_HCALL_FUNC_IPI, (unsigned long)bitmap,
    (unsigned long)(bitmap >> BITS_PER_LONG), min);
    }
#[no_mangle]
unsafe extern "C" fn pv_ipi_interrupt(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t pv_ipi_interrupt(int irq, void *dev)
    {
    u32 action;
    irq_cpustat_t *info;
// Clear SWI interrupt
    clear_csr_estat(1 << INT_SWI0);
    info = this_cpu_ptr(&irq_stat);
    action = atomic_xchg(&info.message, 0);
    if (action & SMP_RESCHEDULE) {
    scheduler_ipi();
    info.ipi_irqs[IPI_RESCHEDULE]++;
    }
    if (action & SMP_CALL_FUNCTION) {
    generic_smp_call_function_interrupt();
    info.ipi_irqs[IPI_CALL_FUNCTION]++;
    }
    if (action & SMP_IRQ_WORK) {
    irq_work_run();
    info.ipi_irqs[IPI_IRQ_WORK]++;
    }
    if (action & SMP_CLEAR_VECTOR) {
    complete_irq_moving();
    info.ipi_irqs[IPI_CLEAR_VECTOR]++;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pv_init_ipi() {
    static void pv_init_ipi(void)
    {
    int r, swi;
// Init native ipi irq for ACTION_BOOT_CPU
    native_ops.init_ipi();
    swi = get_percpu_irq(INT_SWI0);
    if (swi < 0)
    panic("SWI0 IRQ mapping failed\n");
    irq_set_percpu_devid(swi);
    r = request_percpu_irq(swi, pv_ipi_interrupt, "SWI0-IPI", &irq_stat);
    if (r < 0)
    panic("SWI0 IRQ request failed\n");
    }

#[no_mangle]
pub unsafe extern "C" fn kvm_para_available() -> bool {
    bool kvm_para_available(void)
    {
    int config;
    static int hypervisor_type;
    if (!cpu_has_hypervisor)
    return false;
    if (!hypervisor_type) {
    config = read_cpucfg(CPUCFG_KVM_SIG);
    if (!memcmp(&config, KVM_SIGNATURE, 4))
    hypervisor_type = HYPERVISOR_KVM;
    }
    let mut hypervisor_type: return = = HYPERVISOR_KVM;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_para_features() -> c_uint {
    unsigned int kvm_arch_para_features(void)
    {
    static unsigned int feature;
    if (!kvm_para_available())
    return 0;
    if (!feature)
    feature = read_cpucfg(CPUCFG_KVM_FEATURE);
    return feature;
    }
#[no_mangle]
pub unsafe extern "C" fn pv_ipi_init() -> int __init {
    int __init pv_ipi_init(void)
    {
    if (!kvm_para_has_feature(KVM_FEATURE_IPI))
    return 0;

    native_ops		= mp_ops;
    mp_ops.init_ipi		= pv_init_ipi;
    mp_ops.send_ipi_single	= pv_send_ipi_single;
    mp_ops.send_ipi_mask	= pv_send_ipi_mask;

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pv_enable_steal_time() -> c_int {
    static int pv_enable_steal_time(void)
    {
    let mut cpu: c_int = smp_processor_id();
    unsigned long addr;
    struct kvm_steal_time *st;
    if (!has_steal_clock)
    return -EPERM;
    st = &per_cpu(steal_time, cpu);
    addr = per_cpu_ptr_to_phys(st);
// The whole structure kvm_steal_time should be in one page
    if (PFN_DOWN(addr) != PFN_DOWN(addr + sizeof(*st))) {
    pr_warn("Illegal PV steal time addr %lx\n", addr);
    return -EFAULT;
    }
    addr |= KVM_STEAL_PHYS_VALID;
    kvm_hypercall2(KVM_HCALL_FUNC_NOTIFY, BIT(KVM_FEATURE_STEAL_TIME), addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pv_disable_steal_time() {
    static void pv_disable_steal_time(void)
    {
    if (has_steal_clock)
    kvm_hypercall2(KVM_HCALL_FUNC_NOTIFY, BIT(KVM_FEATURE_STEAL_TIME), 0);
    }

#[no_mangle]
unsafe extern "C" fn pv_time_cpu_online(cpu: c_uint) -> c_int {
    static int pv_time_cpu_online(unsigned int cpu)
    {
    unsigned long flags;
    local_irq_save(flags);
    pv_enable_steal_time();
    local_irq_restore(flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pv_time_cpu_down_prepare(cpu: c_uint) -> c_int {
    static int pv_time_cpu_down_prepare(unsigned int cpu)
    {
    unsigned long flags;
    local_irq_save(flags);
    pv_disable_steal_time();
    local_irq_restore(flags);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pv_cpu_reboot(unused: *mut c_void) {
    static void pv_cpu_reboot(void *unused)
    {
    pv_disable_steal_time();
    }
#[no_mangle]
unsafe extern "C" fn pv_reboot_notify(nb: *mut notifier_block, code: c_ulong, unused: *mut c_void) -> c_int {
    static int pv_reboot_notify(struct notifier_block *nb, unsigned long code, void *unused)
    {
    on_each_cpu(pv_cpu_reboot, core::ptr::null_mut(), 1);
    return NOTIFY_DONE;
    }
    static struct notifier_block pv_reboot_nb = {
    .notifier_call  = pv_reboot_notify,
    };
#[no_mangle]
pub unsafe extern "C" fn pv_time_init() -> int __init {
    int __init pv_time_init(void)
    {
    int r;
    if (!kvm_para_has_feature(KVM_FEATURE_STEAL_TIME))
    return 0;
    has_steal_clock = 1;
    r = pv_enable_steal_time();
    if (r < 0) {
    has_steal_clock = 0;
    return 0;
    }
    register_reboot_notifier(&pv_reboot_nb);

    r = cpuhp_setup_state_nocalls(CPUHP_AP_ONLINE_DYN,
    "loongarch/pv_time:online",
    pv_time_cpu_online, pv_time_cpu_down_prepare);
    if (r < 0) {
    has_steal_clock = 0;
    pr_err("Failed to install cpu hotplug callbacks\n");
    return r;
    }
    if (kvm_para_has_feature(KVM_FEATURE_PREEMPT))
    static_branch_enable(&virt_preempt_key);

    static_call_update(pv_steal_clock, paravt_steal_clock);
    static_key_slow_inc(&paravirt_steal_enabled);

    if (steal_acc)
    static_key_slow_inc(&paravirt_steal_rq_enabled);

    if (static_key_enabled(&virt_preempt_key))
    pr_info("Using paravirt steal-time with preempt enabled\n");
    else
    pr_info("Using paravirt steal-time with preempt disabled\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pv_spinlock_init() -> int __init {
    int __init pv_spinlock_init(void)
    {
    if (!cpu_has_hypervisor)
    return 0;
    static_branch_enable(&virt_spin_lock_key);
    return 0;
    }
