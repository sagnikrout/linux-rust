//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-loongarch-avec.c
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
// Copyright (C) 2020-2024 Loongson Technologies, Inc.
//

pub const VECTORS_PER_REG: c_int = 64;
pub const IRR_VECTOR_MASK: c_uint = 0xffUL;
pub const IRR_INVALID_MASK: c_uint = 0x80000000UL;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_list {
    pub head: list_head,
}

    static struct cpumask intersect_mask;
    static DEFINE_PER_CPU(struct pending_list, pending_list);

    static DEFINE_PER_CPU(struct irq_desc * [NR_VECTORS], irq_map);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avecintc_chip {
    pub lock: raw_spinlock_t,
    pub fwnode: *mut fwnode_handle,
    pub domain: *mut irq_domain,
    pub vector_matrix: *mut irq_matrix,
    pub msi_base_addr: phys_addr_t,
}

    static struct avecintc_chip loongarch_avec;
#[no_mangle]
pub unsafe extern "C" fn avecintc_enable() {
    static inline void avecintc_enable(void)
    {

    u64 value;
    value = iocsr_read64(LOONGARCH_IOCSR_MISC_FUNC);
    value |= IOCSR_MISC_FUNC_AVEC_EN;
    iocsr_write64(value, LOONGARCH_IOCSR_MISC_FUNC);

    }
#[no_mangle]
pub unsafe extern "C" fn avecintc_ack_irq(d: *mut irq_data) {
    static inline void avecintc_ack_irq(struct irq_data *d)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn avecintc_mask_irq(d: *mut irq_data) {
    static inline void avecintc_mask_irq(struct irq_data *d)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn avecintc_unmask_irq(d: *mut irq_data) {
    static inline void avecintc_unmask_irq(struct irq_data *d)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn pending_list_init(cpu: c_int) {
    static inline void pending_list_init(int cpu)
    {
    struct pending_list *plist = per_cpu_ptr(&pending_list, cpu);
    INIT_LIST_HEAD(&plist.head);
    }
#[no_mangle]
pub unsafe extern "C" fn avecintc_sync(adata: *mut avecintc_data) {
    void avecintc_sync(struct avecintc_data *adata)
    {
    struct pending_list *plist;
    if (cpu_online(adata.prev_cpu)) {
    plist = per_cpu_ptr(&pending_list, adata.prev_cpu);
    list_add_tail(&adata.entry, &plist.head);
    adata.moving = 1;
    mp_ops.send_ipi_single(adata.prev_cpu, ACTION_CLEAR_VECTOR);
    }
    }
#[no_mangle]
unsafe extern "C" fn avecintc_set_affinity(data: *mut irq_data, dest: *const cpumask, force: bool) -> c_int {
    static int avecintc_set_affinity(struct irq_data *data, const struct cpumask *dest, bool force)
    {
    int cpu, ret, vector;
    struct avecintc_data *adata;
    scoped_guard(raw_spinlock, &loongarch_avec.lock) {
    adata = irq_data_get_irq_chip_data(data);
    if (adata.moving)
    return -EBUSY;
    if (cpu_online(adata.cpu) && cpumask_test_cpu(adata.cpu, dest))
    return IRQ_SET_MASK_OK_DONE;
    cpumask_and(&intersect_mask, dest, cpu_online_mask);
    ret = irq_matrix_alloc(loongarch_avec.vector_matrix, &intersect_mask, false, &cpu);
    if (ret < 0)
    return ret;
    vector = ret;
    adata.cpu = cpu;
    adata.vec = vector;
    per_cpu_ptr(irq_map, adata.cpu)[adata.vec] = irq_data_to_desc(data);
    if (!cpu_has_redirectint)
    avecintc_sync(adata);
    }
    irq_data_update_effective_affinity(data, cpumask_of(cpu));
    return IRQ_SET_MASK_OK;
    }
#[no_mangle]
unsafe extern "C" fn avecintc_cpu_online(cpu: c_uint) -> c_int {
    static int avecintc_cpu_online(unsigned int cpu)
    {
    if (!loongarch_avec.vector_matrix)
    return 0;
    guard(raw_spinlock)(&loongarch_avec.lock);
    avecintc_enable();
    irq_matrix_online(loongarch_avec.vector_matrix);
    pending_list_init(cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn avecintc_cpu_offline(cpu: c_uint) -> c_int {
    static int avecintc_cpu_offline(unsigned int cpu)
    {
    struct pending_list *plist = per_cpu_ptr(&pending_list, cpu);
    if (!loongarch_avec.vector_matrix)
    return 0;
    guard(raw_spinlock)(&loongarch_avec.lock);
    if (!list_empty(&plist.head))
    pr_warn("CPU#%d vector is busy\n", cpu);
    irq_matrix_offline(loongarch_avec.vector_matrix);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn complete_irq_moving() {
    void complete_irq_moving(void)
    {
    struct pending_list *plist = this_cpu_ptr(&pending_list);
    struct avecintc_data *adata, *tdata;
    int cpu, vector, bias;
    unsigned long isr;
    guard(raw_spinlock)(&loongarch_avec.lock);
    list_for_each_entry_safe(adata, tdata, &plist.head, entry) {
    cpu = adata.prev_cpu;
    vector = adata.prev_vec;
    bias = vector / VECTORS_PER_REG;
    switch (bias) {
    case 0:
    isr = csr_read(LOONGARCH_CSR_ISR0);
    break;
    case 1:
    isr = csr_read(LOONGARCH_CSR_ISR1);
    break;
    case 2:
    isr = csr_read(LOONGARCH_CSR_ISR2);
    break;
    case 3:
    isr = csr_read(LOONGARCH_CSR_ISR3);
    break;
    }
    if (isr & (1UL << (vector % VECTORS_PER_REG))) {
    mp_ops.send_ipi_single(cpu, ACTION_CLEAR_VECTOR);
    continue;
    }
    list_del(&adata.entry);
    irq_matrix_free(loongarch_avec.vector_matrix, cpu, vector, false);
    this_cpu_write(irq_map[vector], core::ptr::null_mut());
    adata.moving = 0;
    adata.prev_cpu = adata.cpu;
    adata.prev_vec = adata.vec;
    }
    }

#[no_mangle]
unsafe extern "C" fn avecintc_compose_msi_msg(d: *mut irq_data, msg: *mut msi_msg) {
    static void avecintc_compose_msi_msg(struct irq_data *d, struct msi_msg *msg)
    {
    struct avecintc_data *adata = irq_data_get_irq_chip_data(d);
    msg.address_hi = 0x0;
    msg.address_lo = (loongarch_avec.msi_base_addr |
    (adata.vec & AVEC_IRQ_MASK) << AVEC_IRQ_SHIFT) |
    ((cpu_logical_map(adata.cpu & AVEC_CPU_MASK)) << AVEC_CPU_SHIFT);
    msg.data = 0x0;
    }
    static struct irq_chip avec_irq_controller = {
    .name			= "AVECINTC",
    .irq_ack		= avecintc_ack_irq,
    .irq_mask		= avecintc_mask_irq,
    .irq_unmask		= avecintc_unmask_irq,

    .irq_set_affinity	= avecintc_set_affinity,

    .irq_compose_msi_msg	= avecintc_compose_msi_msg,
    };
#[no_mangle]
unsafe extern "C" fn avecintc_irq_dispatch(desc: *mut irq_desc) {
    static void avecintc_irq_dispatch(struct irq_desc *desc)
    {
    struct irq_chip *chip = irq_desc_get_chip(desc);
    struct irq_desc *d;
    chained_irq_enter(chip, desc);
    while (true) {
    let mut vector: c_ulong = csr_read(LOONGARCH_CSR_IRR);
    if (vector & IRR_INVALID_MASK)
    break;
    vector &= IRR_VECTOR_MASK;
    d = this_cpu_read(irq_map[vector]);
    if (d) {
    generic_handle_irq_desc(d);
    } else {
    spurious_interrupt();
    pr_warn("Unexpected IRQ occurs on CPU#%d [vector %ld]\n", smp_processor_id(), vector);
    }
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn avecintc_alloc_vector(irqd: *mut irq_data, adata: *mut avecintc_data) -> c_int {
    static int avecintc_alloc_vector(struct irq_data *irqd, struct avecintc_data *adata)
    {
    int cpu, ret;
    guard(raw_spinlock_irqsave)(&loongarch_avec.lock);
    ret = irq_matrix_alloc(loongarch_avec.vector_matrix, cpu_online_mask, false, &cpu);
    if (ret < 0)
    return ret;
    adata.prev_cpu = adata.cpu = cpu;
    adata.prev_vec = adata.vec = ret;
    per_cpu_ptr(irq_map, adata.cpu)[adata.vec] = irq_data_to_desc(irqd);
    return 0;
    }
    static int avecintc_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    for (unsigned int i = 0; i < nr_irqs; i++) {
    struct irq_data *irqd = irq_domain_get_irq_data(domain, virq + i);
    struct avecintc_data *adata = kzalloc_obj(*adata);
    int ret;
    if (!adata)
    return -ENOMEM;
    ret = avecintc_alloc_vector(irqd, adata);
    if (ret < 0) {
    kfree(adata);
    return ret;
    }
    irq_domain_set_info(domain, virq + i, virq + i, &avec_irq_controller,
    adata, handle_edge_irq, core::ptr::null_mut(), core::ptr::null_mut());
    irqd_set_single_target(irqd);
    irqd_set_affinity_on_activate(irqd);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn avecintc_free_vector(irqd: *mut irq_data, adata: *mut avecintc_data) {
    static void avecintc_free_vector(struct irq_data *irqd, struct avecintc_data *adata)
    {
    guard(raw_spinlock_irqsave)(&loongarch_avec.lock);
    per_cpu(irq_map, adata.cpu)[adata.vec] = core::ptr::null_mut();
    irq_matrix_free(loongarch_avec.vector_matrix, adata.cpu, adata.vec, false);

    if (!adata.moving)
    return;
    per_cpu(irq_map, adata.prev_cpu)[adata.prev_vec] = core::ptr::null_mut();
    irq_matrix_free(loongarch_avec.vector_matrix, adata.prev_cpu, adata.prev_vec, false);
    list_del_init(&adata.entry);

    }
    static void avecintc_domain_free(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs)
    {
    for (unsigned int i = 0; i < nr_irqs; i++) {
    struct irq_data *d = irq_domain_get_irq_data(domain, virq + i);
    if (d) {
    struct avecintc_data *adata = irq_data_get_irq_chip_data(d);
    avecintc_free_vector(d, adata);
    irq_domain_reset_irq_data(d);
    kfree(adata);
    }
    }
    }
    static const struct irq_domain_ops avecintc_domain_ops = {
    .alloc		= avecintc_domain_alloc,
    .free		= avecintc_domain_free,
    .select		= msi_lib_irq_domain_select,
    };
#[no_mangle]
unsafe extern "C" fn irq_matrix_init() -> int __init {
    static int __init irq_matrix_init(void)
    {
    loongarch_avec.vector_matrix = irq_alloc_matrix(NR_VECTORS, 0, NR_VECTORS);
    if (!loongarch_avec.vector_matrix)
    return -ENOMEM;
    for (int i = 0; i < NR_LEGACY_VECTORS; i++)
    irq_matrix_assign_system(loongarch_avec.vector_matrix, i, false);
    irq_matrix_online(loongarch_avec.vector_matrix);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn avecintc_init(parent: *mut irq_domain) -> int __init {
    static int __init avecintc_init(struct irq_domain *parent)
    {
    int ret, parent_irq;
    raw_spin_lock_init(&loongarch_avec.lock);
    loongarch_avec.fwnode = irq_domain_alloc_named_fwnode("AVECINTC");
    if (!loongarch_avec.fwnode) {
    pr_err("Unable to allocate domain handle\n");
    ret = -ENOMEM;
    goto out;
    }
    loongarch_avec.domain = irq_domain_create_tree(loongarch_avec.fwnode,
    &avecintc_domain_ops, core::ptr::null_mut());
    if (!loongarch_avec.domain) {
    pr_err("Unable to create IRQ domain\n");
    ret = -ENOMEM;
    goto out_free_handle;
    }
    parent_irq = irq_create_mapping(parent, INT_AVEC);
    if (!parent_irq) {
    pr_err("Failed to mapping hwirq\n");
    ret = -EINVAL;
    goto out_remove_domain;
    }
    ret = irq_matrix_init();
    if (ret < 0) {
    pr_err("Failed to init irq matrix\n");
    goto out_remove_domain;
    }
    irq_set_chained_handler_and_data(parent_irq, avecintc_irq_dispatch, core::ptr::null_mut());

    pending_list_init(0);
    cpuhp_setup_state_nocalls(CPUHP_AP_IRQ_AVECINTC_STARTING,
    "irqchip/loongarch/avecintc:starting",
    avecintc_cpu_online, avecintc_cpu_offline);

    avecintc_enable();
    return ret;
    out_remove_domain:
    irq_domain_remove(loongarch_avec.domain);
    out_free_handle:
    irq_domain_free_fwnode(loongarch_avec.fwnode);
    out:
    return ret;
    }
    static int __init pch_msi_parse_madt(union acpi_subtable_headers *header,
    const unsigned long end)
    {
    struct acpi_madt_msi_pic *pchmsi_entry = (struct acpi_madt_msi_pic *)header;
    loongarch_avec.msi_base_addr = pchmsi_entry.msg_address - AVEC_MSG_OFFSET;
    return pch_msi_acpi_init_avec(loongarch_avec.domain);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_cascade_irqdomain_init() -> int __init {
    static inline int __init acpi_cascade_irqdomain_init(void)
    {
    if (cpu_has_redirectint)
    return redirect_acpi_init(loongarch_avec.domain);
    return acpi_table_parse_madt(ACPI_MADT_TYPE_MSI_PIC, pch_msi_parse_madt, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn avecintc_acpi_init(parent: *mut irq_domain) -> int __init {
    int __init avecintc_acpi_init(struct irq_domain *parent)
    {
    let mut ret: c_int = avecintc_init(parent);
    if (ret < 0) {
    pr_err("Failed to init IRQ domain\n");
    return ret;
    }
    ret = acpi_cascade_irqdomain_init();
    if (ret < 0) {
    pr_err("Failed to init cascade IRQ domain\n");
    return ret;
    }
    return ret;
    }
