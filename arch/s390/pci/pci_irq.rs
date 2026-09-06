//! Automatically rewritten from C to Rust
//! Source: arch/s390/pci/pci_irq.c
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

    static enum {FLOATING, DIRECTED} irq_delivery;
//
// summary bit vector
// FLOATING - summary bit per function
// DIRECTED - summary bit per cpu (only used in fallback path)
//
    static struct airq_iv *zpci_sbv;
//
// interrupt bit vectors
// FLOATING - interrupt bit vector per function
// DIRECTED - interrupt bit vector per cpu
//
    static struct airq_iv **zpci_ibv;
// Modify PCI: Register floating adapter interruptions
#[no_mangle]
unsafe extern "C" fn zpci_set_airq(zdev: *mut zpci_dev) -> c_int {
    static int zpci_set_airq(struct zpci_dev *zdev)
    {
    let mut req: u64 = ZPCI_CREATE_REQ(zdev.fh, 0, ZPCI_MOD_FC_REG_INT);
    let mut fib: zpci_fib = {0};
    u8 status;
    fib.fmt0.isc = PCI_ISC;
    fib.fmt0.sum = 1;	/* enable summary notifications */
    fib.fmt0.noi = airq_iv_end(zdev.aibv);
    fib.fmt0.aibv = virt_to_phys(zdev.aibv.vector);
    fib.fmt0.aibvo = 0;	/* each zdev has its own interrupt vector */
    fib.fmt0.aisb = virt_to_phys(zpci_sbv.vector) + (zdev.aisb / 64) * 8;
    fib.fmt0.aisbo = zdev.aisb & 63;
    fib.gd = zdev.gisa;
    return zpci_mod_fc(req, &fib, &status) ? -EIO : 0;
    }
// Modify PCI: Unregister floating adapter interruptions
#[no_mangle]
unsafe extern "C" fn zpci_clear_airq(zdev: *mut zpci_dev) -> c_int {
    static int zpci_clear_airq(struct zpci_dev *zdev)
    {
    let mut req: u64 = ZPCI_CREATE_REQ(zdev.fh, 0, ZPCI_MOD_FC_DEREG_INT);
    let mut fib: zpci_fib = {0};
    u8 cc, status;
    fib.gd = zdev.gisa;
    cc = zpci_mod_fc(req, &fib, &status);
    if (cc == 3 || (cc == 1 && status == 24))
// Function already gone or IRQs already deregistered.
    cc = 0;
    return cc ? -EIO : 0;
    }
// Modify PCI: Register CPU directed interruptions
#[no_mangle]
unsafe extern "C" fn zpci_set_directed_irq(zdev: *mut zpci_dev) -> c_int {
    static int zpci_set_directed_irq(struct zpci_dev *zdev)
    {
    let mut req: u64 = ZPCI_CREATE_REQ(zdev.fh, 0, ZPCI_MOD_FC_REG_INT_D);
    let mut fib: zpci_fib = {0};
    u8 status;
    fib.fmt = 1;
    fib.fmt1.noi = zdev.msi_nr_irqs;
    fib.fmt1.dibvo = zdev.msi_first_bit;
    fib.gd = zdev.gisa;
    return zpci_mod_fc(req, &fib, &status) ? -EIO : 0;
    }
// Modify PCI: Unregister CPU directed interruptions
#[no_mangle]
unsafe extern "C" fn zpci_clear_directed_irq(zdev: *mut zpci_dev) -> c_int {
    static int zpci_clear_directed_irq(struct zpci_dev *zdev)
    {
    let mut req: u64 = ZPCI_CREATE_REQ(zdev.fh, 0, ZPCI_MOD_FC_DEREG_INT_D);
    let mut fib: zpci_fib = {0};
    u8 cc, status;
    fib.fmt = 1;
    fib.gd = zdev.gisa;
    cc = zpci_mod_fc(req, &fib, &status);
    if (cc == 3 || (cc == 1 && status == 24))
// Function already gone or IRQs already deregistered.
    cc = 0;
    return cc ? -EIO : 0;
    }
// Register adapter interruptions
#[no_mangle]
pub unsafe extern "C" fn zpci_set_irq(zdev: *mut zpci_dev) -> c_int {
    int zpci_set_irq(struct zpci_dev *zdev)
    {
    int rc;
    if (irq_delivery == DIRECTED)
    rc = zpci_set_directed_irq(zdev);
    else
    rc = zpci_set_airq(zdev);
    return rc;
    }
// Clear adapter interruptions
#[no_mangle]
unsafe extern "C" fn zpci_clear_irq(zdev: *mut zpci_dev) -> c_int {
    static int zpci_clear_irq(struct zpci_dev *zdev)
    {
    int rc;
    if (irq_delivery == DIRECTED)
    rc = zpci_clear_directed_irq(zdev);
    else
    rc = zpci_clear_airq(zdev);
    return rc;
    }
    static int zpci_set_irq_affinity(struct irq_data *data, const struct cpumask *dest,
    bool force)
    {
    irq_data_update_affinity(data, dest);
    return IRQ_SET_MASK_OK;
    }
//
// Encode the hwirq number for the parent domain. The encoding must be unique
// for each IRQ of each device in the parent domain, so it uses the devfn to
// identify the device and the msi_index to identify the IRQ within that device.
//
#[no_mangle]
pub unsafe extern "C" fn zpci_encode_hwirq(devfn: u8, msi_index: u16) -> u32 {
    static inline u32 zpci_encode_hwirq(u8 devfn, u16 msi_index)
    {
    return (devfn << 16) | msi_index;
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_decode_hwirq_msi_index(hwirq: irq_hw_number_t) -> u16 {
    static inline u16 zpci_decode_hwirq_msi_index(irq_hw_number_t hwirq)
    {
    return hwirq & 0xffff;
    }
#[no_mangle]
unsafe extern "C" fn zpci_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) {
    static void zpci_compose_msi_msg(struct irq_data *data, struct msi_msg *msg)
    {
    struct msi_desc *desc = irq_data_get_msi_desc(data);
    struct zpci_dev *zdev = to_zpci_dev(desc.dev);
    if (irq_delivery == DIRECTED) {
    let mut cpu: c_int = cpumask_first(irq_data_get_affinity_mask(data));
    msg.address_lo = zdev.msi_addr & 0xff0000ff;
    msg.address_lo |= (smp_cpu_get_cpu_address(cpu) << 8);
    } else {
    msg.address_lo = zdev.msi_addr & 0xffffffff;
    }
    msg.address_hi = zdev.msi_addr >> 32;
    msg.data = zpci_decode_hwirq_msi_index(data.hwirq);
    }
    static struct irq_chip zpci_irq_chip = {
    .name = "PCI-MSI",
    .irq_compose_msi_msg = zpci_compose_msi_msg,
    };
#[no_mangle]
unsafe extern "C" fn zpci_handle_cpu_local_irq(rescan: bool) {
    static void zpci_handle_cpu_local_irq(bool rescan)
    {
    struct airq_iv *dibv = zpci_ibv[smp_processor_id()];
    let mut iib: union zpci_sic_iib = {{0}};
    struct irq_domain *msi_domain;
    irq_hw_number_t hwirq;
    unsigned long bit;
    let mut irqs_on: c_int = 0;
    for (bit = 0;;) {
// Scan the directed IRQ bit vector
    bit = airq_iv_scan(dibv, bit, airq_iv_end(dibv));
    if (bit == -1UL) {
    if (!rescan || irqs_on++)
// End of second scan with interrupts on.
    break;
// First scan complete, re-enable interrupts.
    if (zpci_set_irq_ctrl(SIC_IRQ_MODE_D_SINGLE, PCI_ISC, &iib))
    break;
    bit = 0;
    continue;
    }
    inc_irq_stat(IRQIO_MSI);
    hwirq = airq_iv_get_data(dibv, bit);
    msi_domain = (struct irq_domain *)airq_iv_get_ptr(dibv, bit);
    generic_handle_domain_irq(msi_domain, hwirq);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_irq_data {
    pub csd: call_single_data_t,
    pub scheduled: core::sync::atomic::AtomicI32,
}

    static DEFINE_PER_CPU_SHARED_ALIGNED(struct cpu_irq_data, irq_data);
#[no_mangle]
unsafe extern "C" fn zpci_handle_remote_irq(data: *mut c_void) {
    static void zpci_handle_remote_irq(void *data)
    {
    atomic_t *scheduled = data;
    do {
    zpci_handle_cpu_local_irq(false);
    } while (atomic_dec_return(scheduled));
    }
#[no_mangle]
unsafe extern "C" fn zpci_handle_fallback_irq() {
    static void zpci_handle_fallback_irq(void)
    {
    struct cpu_irq_data *cpu_data;
    let mut iib: union zpci_sic_iib = {{0}};
    unsigned long cpu;
    let mut irqs_on: c_int = 0;
    for (cpu = 0;;) {
    cpu = airq_iv_scan(zpci_sbv, cpu, airq_iv_end(zpci_sbv));
    if (cpu == -1UL) {
    if (irqs_on++)
// End of second scan with interrupts on.
    break;
// First scan complete, re-enable interrupts.
    if (zpci_set_irq_ctrl(SIC_IRQ_MODE_SINGLE, PCI_ISC, &iib))
    break;
    cpu = 0;
    continue;
    }
    cpu_data = &per_cpu(irq_data, cpu);
    if (atomic_inc_return(&cpu_data.scheduled) > 1)
    continue;
    INIT_CSD(&cpu_data.csd, zpci_handle_remote_irq, &cpu_data.scheduled);
    smp_call_function_single_async(cpu, &cpu_data.csd);
    }
    }
    static void zpci_directed_irq_handler(struct airq_struct *airq,
    struct tpi_info *tpi_info)
    {
    let mut floating: bool = !tpi_info.directed_irq;
    if (floating) {
    inc_irq_stat(IRQIO_PCF);
    zpci_handle_fallback_irq();
    } else {
    inc_irq_stat(IRQIO_PCD);
    zpci_handle_cpu_local_irq(true);
    }
    }
    static void zpci_floating_irq_handler(struct airq_struct *airq,
    struct tpi_info *tpi_info)
    {
    let mut iib: union zpci_sic_iib = {{0}};
    struct irq_domain *msi_domain;
    irq_hw_number_t hwirq;
    unsigned long si, ai;
    struct airq_iv *aibv;
    let mut irqs_on: c_int = 0;
    inc_irq_stat(IRQIO_PCF);
    for (si = 0;;) {
// Scan adapter summary indicator bit vector
    si = airq_iv_scan(zpci_sbv, si, airq_iv_end(zpci_sbv));
    if (si == -1UL) {
    if (irqs_on++)
// End of second scan with interrupts on.
    break;
// First scan complete, re-enable interrupts.
    if (zpci_set_irq_ctrl(SIC_IRQ_MODE_SINGLE, PCI_ISC, &iib))
    break;
    si = 0;
    continue;
    }
// Scan the adapter interrupt vector for this device.
    aibv = zpci_ibv[si];
    for (ai = 0;;) {
    ai = airq_iv_scan(aibv, ai, airq_iv_end(aibv));
    if (ai == -1UL)
    break;
    inc_irq_stat(IRQIO_MSI);
    airq_iv_lock(aibv, ai);
    hwirq = airq_iv_get_data(aibv, ai);
    msi_domain = (struct irq_domain *)airq_iv_get_ptr(aibv, ai);
    generic_handle_domain_irq(msi_domain, hwirq);
    airq_iv_unlock(aibv, ai);
    }
    }
    }
    static int __alloc_airq(struct zpci_dev *zdev, int msi_vecs,
    unsigned long *bit)
    {
    if (irq_delivery == DIRECTED) {
// Allocate cpu vector bits
// bit = airq_iv_alloc(zpci_ibv[0], msi_vecs);
    if (*bit == -1UL)
    return -EIO;
    } else {
// Allocate adapter summary indicator bit
// bit = airq_iv_alloc_bit(zpci_sbv);
    if (*bit == -1UL)
    return -EIO;
    zdev.aisb = *bit;
// Create adapter interrupt vector
    zdev.aibv = airq_iv_create(msi_vecs,
    AIRQ_IV_PTR | AIRQ_IV_DATA | AIRQ_IV_BITLOCK,
    core::ptr::null_mut());
    if (!zdev.aibv)
    return -ENOMEM;
// Wire up shortcut pointer
    zpci_ibv[*bit] = zdev.aibv;
// Each function has its own interrupt vector
// bit = 0;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_restore_msi_irqs(pdev: *mut pci_dev) -> bool {
    bool arch_restore_msi_irqs(struct pci_dev *pdev)
    {
    struct zpci_dev *zdev = to_zpci(pdev);
    zpci_set_irq(zdev);
    return true;
    }
    static struct airq_struct zpci_airq = {
    .handler = zpci_floating_irq_handler,
    .isc = PCI_ISC,
    };
#[no_mangle]
unsafe extern "C" fn zpci_msi_teardown_directed(zdev: *mut zpci_dev) {
    static void zpci_msi_teardown_directed(struct zpci_dev *zdev)
    {
    airq_iv_free(zpci_ibv[0], zdev.msi_first_bit, zdev.max_msi);
    zdev.msi_first_bit = -1U;
    zdev.msi_nr_irqs = 0;
    }
#[no_mangle]
unsafe extern "C" fn zpci_msi_teardown_floating(zdev: *mut zpci_dev) {
    static void zpci_msi_teardown_floating(struct zpci_dev *zdev)
    {
    airq_iv_release(zdev.aibv);
    zdev.aibv = core::ptr::null_mut();
    airq_iv_free_bit(zpci_sbv, zdev.aisb);
    zdev.aisb = -1UL;
    zdev.msi_first_bit = -1U;
    zdev.msi_nr_irqs = 0;
    }
#[no_mangle]
unsafe extern "C" fn zpci_msi_teardown(domain: *mut irq_domain, arg: *mut msi_alloc_info_t) {
    static void zpci_msi_teardown(struct irq_domain *domain, msi_alloc_info_t *arg)
    {
    struct zpci_dev *zdev = to_zpci_dev(domain.dev);
    zpci_clear_irq(zdev);
    if (irq_delivery == DIRECTED)
    zpci_msi_teardown_directed(zdev);
    else
    zpci_msi_teardown_floating(zdev);
    }
    static int zpci_msi_prepare(struct irq_domain *domain,
    struct device *dev, int nvec,
    msi_alloc_info_t *info)
    {
    struct zpci_dev *zdev = to_zpci_dev(dev);
    struct pci_dev *pdev = to_pci_dev(dev);
    unsigned long bit;
    int msi_vecs, rc;
    msi_vecs = min_t(unsigned int, nvec, zdev.max_msi);
    if (msi_vecs < nvec) {
    pr_info("%s requested %d IRQs, allocate system limit of %d\n",
    pci_name(pdev), nvec, zdev.max_msi);
    }
    rc = __alloc_airq(zdev, msi_vecs, &bit);
    if (rc) {
    pr_err("Allocating adapter IRQs for %s failed\n", pci_name(pdev));
    return rc;
    }
    zdev.msi_first_bit = bit;
    zdev.msi_nr_irqs = msi_vecs;
    rc = zpci_set_irq(zdev);
    if (rc) {
    pr_err("Registering adapter IRQs for %s failed\n",
    pci_name(pdev));
    if (irq_delivery == DIRECTED)
    zpci_msi_teardown_directed(zdev);
    else
    zpci_msi_teardown_floating(zdev);
    return rc;
    }
    return 0;
    }
    static int zpci_msi_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *args)
    {
    struct msi_desc *desc = ((msi_alloc_info_t *)args).desc;
    struct zpci_dev *zdev = to_zpci_dev(desc.dev);
    struct zpci_bus *zbus = zdev.zbus;
    unsigned int cpu, hwirq;
    unsigned long bit;
    int i;
    bit = zdev.msi_first_bit + desc.msi_index;
    hwirq = zpci_encode_hwirq(zdev.devfn, desc.msi_index);
    if (desc.msi_index + nr_irqs > zdev.max_msi)
    return -EINVAL;
    for (i = 0; i < nr_irqs; i++) {
    irq_domain_set_info(domain, virq + i, hwirq + i,
    &zpci_irq_chip, zdev,
    handle_percpu_irq, core::ptr::null_mut(), core::ptr::null_mut());
    if (irq_delivery == DIRECTED) {
    for_each_possible_cpu(cpu) {
    airq_iv_set_ptr(zpci_ibv[cpu], bit + i,
    (unsigned long)zbus.msi_parent_domain);
    airq_iv_set_data(zpci_ibv[cpu], bit + i, hwirq + i);
    }
    } else {
    airq_iv_set_ptr(zdev.aibv, bit + i,
    (unsigned long)zbus.msi_parent_domain);
    airq_iv_set_data(zdev.aibv, bit + i, hwirq + i);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zpci_msi_clear_airq(d: *mut irq_data, i: c_int) {
    static void zpci_msi_clear_airq(struct irq_data *d, int i)
    {
    struct msi_desc *desc = irq_data_get_msi_desc(d);
    struct zpci_dev *zdev = to_zpci_dev(desc.dev);
    unsigned long bit;
    unsigned int cpu;
    u16 msi_index;
    msi_index = zpci_decode_hwirq_msi_index(d.hwirq);
    bit = zdev.msi_first_bit + msi_index;
    if (irq_delivery == DIRECTED) {
    for_each_possible_cpu(cpu) {
    airq_iv_set_ptr(zpci_ibv[cpu], bit + i, 0);
    airq_iv_set_data(zpci_ibv[cpu], bit + i, 0);
    }
    } else {
    airq_iv_set_ptr(zdev.aibv, bit + i, 0);
    airq_iv_set_data(zdev.aibv, bit + i, 0);
    }
    }
    static void zpci_msi_domain_free(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs)
    {
    struct irq_data *d;
    int i;
    for (i = 0; i < nr_irqs; i++) {
    d = irq_domain_get_irq_data(domain, virq + i);
    zpci_msi_clear_airq(d, i);
    irq_domain_reset_irq_data(d);
    }
    }
    static const struct irq_domain_ops zpci_msi_domain_ops = {
    .alloc = zpci_msi_domain_alloc,
    .free  = zpci_msi_domain_free,
    };
    static bool zpci_init_dev_msi_info(struct device *dev, struct irq_domain *domain,
    struct irq_domain *real_parent,
    struct msi_domain_info *info)
    {
    if (!msi_lib_init_dev_msi_info(dev, domain, real_parent, info))
    return false;
    info.ops.msi_prepare = zpci_msi_prepare;
    info.ops.msi_teardown = zpci_msi_teardown;
    return true;
    }
    static struct msi_parent_ops zpci_msi_parent_ops = {
    .supported_flags   = MSI_GENERIC_FLAGS_MASK	|
    MSI_FLAG_PCI_MSIX		|
    MSI_FLAG_MULTI_PCI_MSI,
    .required_flags	   = MSI_FLAG_USE_DEF_DOM_OPS  |
    MSI_FLAG_USE_DEF_CHIP_OPS,
    .init_dev_msi_info = zpci_init_dev_msi_info,
    };
#[no_mangle]
pub unsafe extern "C" fn zpci_create_parent_msi_domain(zbus: *mut zpci_bus) -> c_int {
    int zpci_create_parent_msi_domain(struct zpci_bus *zbus)
    {
    char fwnode_name[18];
    snprintf(fwnode_name, sizeof(fwnode_name), "ZPCI_MSI_DOM_%04x", zbus.domain_nr);
    struct irq_domain_info info = {
    .fwnode		= irq_domain_alloc_named_fwnode(fwnode_name),
    .ops		= &zpci_msi_domain_ops,
    };
    if (!info.fwnode) {
    pr_err("Failed to allocate fwnode for MSI IRQ domain\n");
    return -ENOMEM;
    }
    if (irq_delivery == FLOATING)
    zpci_msi_parent_ops.required_flags |= MSI_FLAG_NO_AFFINITY;
    zbus.msi_parent_domain = msi_create_parent_irq_domain(&info, &zpci_msi_parent_ops);
    if (!zbus.msi_parent_domain) {
    irq_domain_free_fwnode(info.fwnode);
    pr_err("Failed to create MSI IRQ domain\n");
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_remove_parent_msi_domain(zbus: *mut zpci_bus) {
    void zpci_remove_parent_msi_domain(struct zpci_bus *zbus)
    {
    struct fwnode_handle *fn;
    fn = zbus.msi_parent_domain.fwnode;
    irq_domain_remove(zbus.msi_parent_domain);
    irq_domain_free_fwnode(fn);
    }
#[no_mangle]
unsafe extern "C" fn cpu_enable_directed_irq(unused: *mut c_void) -> void __init {
    static void __init cpu_enable_directed_irq(void *unused)
    {
    let mut iib: union zpci_sic_iib = {{0}};
    let mut ziib: union zpci_sic_iib = {{0}};
    iib.cdiib.dibv_addr = virt_to_phys(zpci_ibv[smp_processor_id()].vector);
    zpci_set_irq_ctrl(SIC_IRQ_MODE_SET_CPU, 0, &iib);
    zpci_set_irq_ctrl(SIC_IRQ_MODE_D_SINGLE, PCI_ISC, &ziib);
    }
#[no_mangle]
unsafe extern "C" fn zpci_directed_irq_init() -> int __init {
    static int __init zpci_directed_irq_init(void)
    {
    let mut iib: union zpci_sic_iib = {{0}};
    unsigned int cpu;
    zpci_sbv = airq_iv_create(num_possible_cpus(), 0, core::ptr::null_mut());
    if (!zpci_sbv)
    return -ENOMEM;
    iib.diib.isc = PCI_ISC;
    iib.diib.nr_cpus = num_possible_cpus();
    iib.diib.disb_addr = virt_to_phys(zpci_sbv.vector);
    zpci_set_irq_ctrl(SIC_IRQ_MODE_DIRECT, 0, &iib);
    zpci_ibv = kzalloc_objs(*zpci_ibv, num_possible_cpus());
    if (!zpci_ibv)
    return -ENOMEM;
    for_each_possible_cpu(cpu) {
//
// Per CPU IRQ vectors look the same but bit-allocation
// is only done on the first vector.
//
    zpci_ibv[cpu] = airq_iv_create(cache_line_size() * BITS_PER_BYTE,
    AIRQ_IV_PTR |
    AIRQ_IV_DATA |
    AIRQ_IV_CACHELINE |
    (!cpu ? AIRQ_IV_ALLOC : 0), core::ptr::null_mut());
    if (!zpci_ibv[cpu])
    return -ENOMEM;
    }
    on_each_cpu(cpu_enable_directed_irq, core::ptr::null_mut(), 1);
    zpci_irq_chip.irq_set_affinity = zpci_set_irq_affinity;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zpci_floating_irq_init() -> int __init {
    static int __init zpci_floating_irq_init(void)
    {
    zpci_ibv = kzalloc_objs(*zpci_ibv, ZPCI_NR_DEVICES);
    if (!zpci_ibv)
    return -ENOMEM;
    zpci_sbv = airq_iv_create(ZPCI_NR_DEVICES, AIRQ_IV_ALLOC, core::ptr::null_mut());
    if (!zpci_sbv)
    goto out_free;
    return 0;
    out_free:
    kfree(zpci_ibv);
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_irq_init() -> int __init {
    int __init zpci_irq_init(void)
    {
    let mut iib: union zpci_sic_iib = {{0}};
    int rc;
    irq_delivery = sclp.has_dirq ? DIRECTED : FLOATING;
    if (s390_pci_force_floating)
    irq_delivery = FLOATING;
    if (irq_delivery == DIRECTED)
    zpci_airq.handler = zpci_directed_irq_handler;
    rc = register_adapter_interrupt(&zpci_airq);
    if (rc)
    goto out;
// Set summary to 1 to be called every time for the ISC.
// zpci_airq.lsi_ptr = 1;
    switch (irq_delivery) {
    case FLOATING:
    rc = zpci_floating_irq_init();
    break;
    case DIRECTED:
    rc = zpci_directed_irq_init();
    break;
    }
    if (rc)
    goto out_airq;
//
// Enable floating IRQs (with suppression after one IRQ). When using
// directed IRQs this enables the fallback path.
//
    zpci_set_irq_ctrl(SIC_IRQ_MODE_SINGLE, PCI_ISC, &iib);
    return 0;
    out_airq:
    unregister_adapter_interrupt(&zpci_airq);
    out:
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_irq_exit() -> void __init {
    void __init zpci_irq_exit(void)
    {
    unsigned int cpu;
    if (irq_delivery == DIRECTED) {
    for_each_possible_cpu(cpu) {
    airq_iv_release(zpci_ibv[cpu]);
    }
    }
    kfree(zpci_ibv);
    if (zpci_sbv)
    airq_iv_release(zpci_sbv);
    unregister_adapter_interrupt(&zpci_airq);
    }
