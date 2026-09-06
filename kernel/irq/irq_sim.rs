//! Automatically rewritten from C to Rust
//! Source: kernel/irq/irq_sim.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2017-2018 Bartosz Golaszewski <brgl@bgdev.pl>
// Copyright (C) 2020 Bartosz Golaszewski <bgolaszewski@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_sim_work_ctx {
    pub work: irq_work,
    pub irq_count: c_uint,
    pub pending: *mut c_ulong,
    pub domain: *mut irq_domain,
    pub ops: irq_sim_ops,
    pub user_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_sim_irq_ctx {
    pub enabled: bool,
    pub work_ctx: *mut irq_sim_work_ctx,
}

#[no_mangle]
unsafe extern "C" fn irq_sim_irqmask(data: *mut irq_data) {
    static void irq_sim_irqmask(struct irq_data *data)
    {
    struct irq_sim_irq_ctx *irq_ctx = irq_data_get_irq_chip_data(data);
    irq_ctx.enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_irqunmask(data: *mut irq_data) {
    static void irq_sim_irqunmask(struct irq_data *data)
    {
    struct irq_sim_irq_ctx *irq_ctx = irq_data_get_irq_chip_data(data);
    irq_ctx.enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int irq_sim_set_type(struct irq_data *data, unsigned int type)
    {
// We only support rising and falling edge trigger types.
    if (type & ~IRQ_TYPE_EDGE_BOTH)
    return -EINVAL;
    irqd_set_trigger_type(data, type);
    return 0;
    }
    static int irq_sim_get_irqchip_state(struct irq_data *data,
    enum irqchip_irq_state which, bool *state)
    {
    struct irq_sim_irq_ctx *irq_ctx = irq_data_get_irq_chip_data(data);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    switch (which) {
    case IRQCHIP_STATE_PENDING:
    if (irq_ctx.enabled)
// state = test_bit(hwirq, irq_ctx->work_ctx->pending);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int irq_sim_set_irqchip_state(struct irq_data *data,
    enum irqchip_irq_state which, bool state)
    {
    struct irq_sim_irq_ctx *irq_ctx = irq_data_get_irq_chip_data(data);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    switch (which) {
    case IRQCHIP_STATE_PENDING:
    if (irq_ctx.enabled) {
    assign_bit(hwirq, irq_ctx.work_ctx.pending, state);
    if (state)
    irq_work_queue(&irq_ctx.work_ctx.work);
    }
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_request_resources(data: *mut irq_data) -> c_int {
    static int irq_sim_request_resources(struct irq_data *data)
    {
    struct irq_sim_irq_ctx *irq_ctx = irq_data_get_irq_chip_data(data);
    struct irq_sim_work_ctx *work_ctx = irq_ctx.work_ctx;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    if (work_ctx.ops.irq_sim_irq_requested)
    return work_ctx.ops.irq_sim_irq_requested(work_ctx.domain,
    hwirq,
    work_ctx.user_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_release_resources(data: *mut irq_data) {
    static void irq_sim_release_resources(struct irq_data *data)
    {
    struct irq_sim_irq_ctx *irq_ctx = irq_data_get_irq_chip_data(data);
    struct irq_sim_work_ctx *work_ctx = irq_ctx.work_ctx;
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    if (work_ctx.ops.irq_sim_irq_released)
    work_ctx.ops.irq_sim_irq_released(work_ctx.domain, hwirq,
    work_ctx.user_data);
    }
    static struct irq_chip irq_sim_irqchip = {
    .name			= "irq_sim",
    .irq_mask		= irq_sim_irqmask,
    .irq_unmask		= irq_sim_irqunmask,
    .irq_set_type		= irq_sim_set_type,
    .irq_get_irqchip_state	= irq_sim_get_irqchip_state,
    .irq_set_irqchip_state	= irq_sim_set_irqchip_state,
    .irq_request_resources	= irq_sim_request_resources,
    .irq_release_resources	= irq_sim_release_resources,
    };
#[no_mangle]
unsafe extern "C" fn irq_sim_handle_irq(work: *mut irq_work) {
    static void irq_sim_handle_irq(struct irq_work *work)
    {
    struct irq_sim_work_ctx *work_ctx;
    let mut offset: c_uint = 0;
    int irqnum;
    work_ctx = container_of(work, struct irq_sim_work_ctx, work);
    while (!bitmap_empty(work_ctx.pending, work_ctx.irq_count)) {
    offset = find_next_bit(work_ctx.pending,
    work_ctx.irq_count, offset);
    clear_bit(offset, work_ctx.pending);
    irqnum = irq_find_mapping(work_ctx.domain, offset);
    handle_simple_irq(irq_to_desc(irqnum));
    }
    }
    static int irq_sim_domain_map(struct irq_domain *domain,
    unsigned int virq, irq_hw_number_t hw)
    {
    struct irq_sim_work_ctx *work_ctx = domain.host_data;
    struct irq_sim_irq_ctx *irq_ctx;
    irq_ctx = kzalloc_obj(*irq_ctx);
    if (!irq_ctx)
    return -ENOMEM;
    irq_set_chip(virq, &irq_sim_irqchip);
    irq_set_chip_data(virq, irq_ctx);
    irq_set_handler(virq, handle_simple_irq);
    irq_modify_status(virq, IRQ_NOREQUEST | IRQ_NOAUTOEN, IRQ_NOPROBE);
    irq_ctx.work_ctx = work_ctx;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_domain_unmap(domain: *mut irq_domain, virq: c_uint) {
    static void irq_sim_domain_unmap(struct irq_domain *domain, unsigned int virq)
    {
    struct irq_sim_irq_ctx *irq_ctx;
    struct irq_data *irqd;
    irqd = irq_domain_get_irq_data(domain, virq);
    irq_ctx = irq_data_get_irq_chip_data(irqd);
    irq_set_handler(virq, core::ptr::null_mut());
    irq_domain_reset_irq_data(irqd);
    kfree(irq_ctx);
    }
    static const struct irq_domain_ops irq_sim_domain_ops = {
    .map		= irq_sim_domain_map,
    .unmap		= irq_sim_domain_unmap,
    };
//
// irq_domain_create_sim - Create a new interrupt simulator irq_domain and
// allocate a range of dummy interrupts.
//
// @fwnode:     struct fwnode_handle to be associated with this domain.
// @num_irqs:   Number of interrupts to allocate.
//
// On success: return a new irq_domain object.
// On failure: a negative errno wrapped with ERR_PTR().
//
    struct irq_domain *irq_domain_create_sim(struct fwnode_handle *fwnode,
    unsigned int num_irqs)
    {
    return irq_domain_create_sim_full(fwnode, num_irqs, core::ptr::null_mut(), core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(irq_domain_create_sim);
    struct irq_domain *irq_domain_create_sim_full(struct fwnode_handle *fwnode,
    unsigned int num_irqs,
    const struct irq_sim_ops *ops,
    void *data)
    {
    struct irq_sim_work_ctx *work_ctx __free(kfree) =
    kzalloc_obj(*work_ctx);
    if (!work_ctx)
    return ERR_PTR(-ENOMEM);
    unsigned long *pending __free(bitmap) = bitmap_zalloc(num_irqs, GFP_KERNEL);
    if (!pending)
    return ERR_PTR(-ENOMEM);
    work_ctx.domain = irq_domain_create_linear(fwnode, num_irqs,
    &irq_sim_domain_ops,
    work_ctx);
    if (!work_ctx.domain)
    return ERR_PTR(-ENOMEM);
    work_ctx.irq_count = num_irqs;
    work_ctx.work = IRQ_WORK_INIT_HARD(irq_sim_handle_irq);
    work_ctx.pending = no_free_ptr(pending);
    work_ctx.user_data = data;
    if (ops)
    memcpy(&work_ctx.ops, ops, sizeof(*ops));
    return no_free_ptr(work_ctx).domain;
    }
    EXPORT_SYMBOL_GPL(irq_domain_create_sim_full);
//
// irq_domain_remove_sim - Deinitialize the interrupt simulator domain: free
// the interrupt descriptors and allocated memory.
//
// @domain:     The interrupt simulator domain to tear down.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_remove_sim(domain: *mut irq_domain) {
    void irq_domain_remove_sim(struct irq_domain *domain)
    {
    struct irq_sim_work_ctx *work_ctx = domain.host_data;
    irq_work_sync(&work_ctx.work);
    bitmap_free(work_ctx.pending);
    kfree(work_ctx);
    irq_domain_remove(domain);
    }
    EXPORT_SYMBOL_GPL(irq_domain_remove_sim);
#[no_mangle]
unsafe extern "C" fn devm_irq_domain_remove_sim(data: *mut c_void) {
    static void devm_irq_domain_remove_sim(void *data)
    {
    struct irq_domain *domain = data;
    irq_domain_remove_sim(domain);
    }
//
// devm_irq_domain_create_sim - Create a new interrupt simulator for
// a managed device.
//
// @dev:        Device to initialize the simulator object for.
// @fwnode:     struct fwnode_handle to be associated with this domain.
// @num_irqs:   Number of interrupts to allocate
//
// On success: return a new irq_domain object.
// On failure: a negative errno wrapped with ERR_PTR().
//
    struct irq_domain *devm_irq_domain_create_sim(struct device *dev,
    struct fwnode_handle *fwnode,
    unsigned int num_irqs)
    {
    return devm_irq_domain_create_sim_full(dev, fwnode, num_irqs,
    core::ptr::null_mut(), core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(devm_irq_domain_create_sim);
    struct irq_domain *
    devm_irq_domain_create_sim_full(struct device *dev,
    struct fwnode_handle *fwnode,
    unsigned int num_irqs,
    const struct irq_sim_ops *ops,
    void *data)
    {
    struct irq_domain *domain;
    int ret;
    domain = irq_domain_create_sim_full(fwnode, num_irqs, ops, data);
    if (IS_ERR(domain))
    return domain;
    ret = devm_add_action_or_reset(dev, devm_irq_domain_remove_sim, domain);
    if (ret)
    return ERR_PTR(ret);
    return domain;
    }
    EXPORT_SYMBOL_GPL(devm_irq_domain_create_sim_full);
