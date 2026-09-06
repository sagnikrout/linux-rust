//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/irq.c
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
// ACPI GSI IRQ layer
//
// Copyright (C) 2015 ARM Ltd.
// Author: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
//

    enum acpi_irq_model_id acpi_irq_model;
    static acpi_gsi_domain_disp_fn acpi_get_gsi_domain_id;
    static acpi_gsi_handle_disp_fn acpi_get_gsi_handle;
    static u32 (*acpi_gsi_to_irq_fallback)(u32 gsi);
//
// acpi_gsi_to_irq() - Retrieve the linux irq number for a given GSI
// @gsi: GSI IRQ number to map
// @irq: pointer where linux IRQ number is stored
//
// irq location updated with irq value [>0 on success, 0 on failure]
//
// Returns: 0 on success
// -EINVAL on failure
//
#[no_mangle]
pub unsafe extern "C" fn acpi_gsi_to_irq(gsi: u32, irq: *mut c_uint) -> c_int {
    int acpi_gsi_to_irq(u32 gsi, unsigned int *irq)
    {
    struct irq_domain *d;
    d = irq_find_matching_fwnode(acpi_get_gsi_domain_id(gsi),
    DOMAIN_BUS_ANY);
// irq = irq_find_mapping(d, gsi);
//
// *irq == 0 means no mapping, that should be reported as a
// failure, unless there is an arch-specific fallback handler.
//
    if (!*irq && acpi_gsi_to_irq_fallback)
// irq = acpi_gsi_to_irq_fallback(gsi);
    return (*irq > 0) ? 0 : -EINVAL;
    }
    EXPORT_SYMBOL_GPL(acpi_gsi_to_irq);
//
// acpi_register_gsi() - Map a GSI to a linux IRQ number
// @dev: device for which IRQ has to be mapped
// @gsi: GSI IRQ number
// @trigger: trigger type of the GSI number to be mapped
// @polarity: polarity of the GSI to be mapped
//
// Returns: a valid linux IRQ number on success
// -EINVAL on failure
//
    int acpi_register_gsi(struct device *dev, u32 gsi, int trigger,
    int polarity)
    {
    struct irq_fwspec fwspec;
    unsigned int irq;
    fwspec.fwnode = acpi_get_gsi_domain_id(gsi);
    if (WARN_ON(!fwspec.fwnode)) {
    pr_warn("GSI: No registered irqchip, giving up\n");
    return -EINVAL;
    }
    fwspec.param[0] = gsi;
    fwspec.param[1] = acpi_dev_get_irq_type(trigger, polarity);
    fwspec.param_count = 2;
    irq = irq_create_fwspec_mapping(&fwspec);
    if (!irq)
    return -EINVAL;
    return irq;
    }
    EXPORT_SYMBOL_GPL(acpi_register_gsi);
//
// acpi_unregister_gsi() - Free a GSI<->linux IRQ number mapping
// @gsi: GSI IRQ number
//
#[no_mangle]
pub unsafe extern "C" fn acpi_unregister_gsi(gsi: u32) {
    void acpi_unregister_gsi(u32 gsi)
    {
    struct irq_domain *d;
    int irq;
    if (WARN_ON(acpi_irq_model == ACPI_IRQ_MODEL_GIC && gsi < 16))
    return;
    d = irq_find_matching_fwnode(acpi_get_gsi_domain_id(gsi),
    DOMAIN_BUS_ANY);
    irq = irq_find_mapping(d, gsi);
    irq_dispose_mapping(irq);
    }
    EXPORT_SYMBOL_GPL(acpi_unregister_gsi);
//
// acpi_get_irq_source_fwhandle() - Retrieve fwhandle from IRQ resource source.
// @source: acpi_resource_source to use for the lookup.
// @gsi: GSI IRQ number
//
// Description:
// Retrieve the fwhandle of the device referenced by the given IRQ resource
// source.
//
// Return:
// The referenced device fwhandle or NULL on failure
//
    static struct fwnode_handle *
    acpi_get_irq_source_fwhandle(const struct acpi_resource_source *source,
    u32 gsi)
    {
    struct fwnode_handle *result;
    struct acpi_device *device;
    acpi_handle handle;
    acpi_status status;
    if (!source.string_length)
    return acpi_get_gsi_domain_id(gsi);
    status = acpi_get_handle(core::ptr::null_mut(), source.string_ptr, &handle);
    if (WARN_ON(ACPI_FAILURE(status)))
    return core::ptr::null_mut();
    device = acpi_get_acpi_dev(handle);
    if (WARN_ON(!device))
    return core::ptr::null_mut();
    result = &device.fwnode;
    acpi_put_acpi_dev(device);
    return result;
    }
//
// Context for the resource walk used to lookup IRQ resources.
// Contains a return code, the lookup index, and references to the flags
// and fwspec where the result is returned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_irq_parse_one_ctx {
    pub rc: c_int,
    pub index: c_uint,
    pub res_flags: *mut c_ulong,
    pub fwspec: *mut irq_fwspec,
}

//
// acpi_irq_parse_one_match - Handle a matching IRQ resource.
// @fwnode: matching fwnode
// @hwirq: hardware IRQ number
// @triggering: triggering attributes of hwirq
// @polarity: polarity attributes of hwirq
// @shareable: shareable attributes of hwirq
// @wake_capable: wake capable attribute of hwirq
// @ctx: acpi_irq_parse_one_ctx updated by this function
//
// Description:
// Handle a matching IRQ resource by populating the given ctx with
// the information passed.
//
    static inline void acpi_irq_parse_one_match(struct fwnode_handle *fwnode,
    u32 hwirq, u8 triggering,
    u8 polarity, u8 shareable,
    u8 wake_capable,
    struct acpi_irq_parse_one_ctx *ctx)
    {
    if (!fwnode)
    return;
    ctx.rc = 0;
// ctx->res_flags = acpi_dev_irq_flags(triggering, polarity, shareable, wake_capable);
    ctx.fwspec.fwnode = fwnode;
    ctx.fwspec.param[0] = hwirq;
    ctx.fwspec.param[1] = acpi_dev_get_irq_type(triggering, polarity);
    ctx.fwspec.param_count = 2;
    }
//
// acpi_irq_parse_one_cb - Handle the given resource.
// @ares: resource to handle
// @context: context for the walk
//
// Description:
// This is called by acpi_walk_resources passing each resource returned by
// the _CRS method. We only inspect IRQ resources. Since IRQ resources
// might contain multiple interrupts we check if the index is within this
// one's interrupt array, otherwise we subtract the current resource IRQ
// count from the lookup index to prepare for the next resource.
// Once a match is found we call acpi_irq_parse_one_match to populate
// the result and end the walk by returning AE_CTRL_TERMINATE.
//
// Return:
// AE_OK if the walk should continue, AE_CTRL_TERMINATE if a matching
// IRQ resource was found.
//
    static acpi_status acpi_irq_parse_one_cb(struct acpi_resource *ares,
    void *context)
    {
    struct acpi_irq_parse_one_ctx *ctx = context;
    struct acpi_resource_irq *irq;
    struct acpi_resource_extended_irq *eirq;
    struct fwnode_handle *fwnode;
    switch (ares.type) {
    case ACPI_RESOURCE_TYPE_IRQ:
    irq = &ares.data.irq;
    if (ctx.index >= irq.interrupt_count) {
    ctx.index -= irq.interrupt_count;
    return AE_OK;
    }
    fwnode = acpi_get_gsi_domain_id(irq.interrupts[ctx.index]);
    acpi_irq_parse_one_match(fwnode, irq.interrupts[ctx.index],
    irq.triggering, irq.polarity,
    irq.shareable, irq.wake_capable, ctx);
    return AE_CTRL_TERMINATE;
    case ACPI_RESOURCE_TYPE_EXTENDED_IRQ:
    eirq = &ares.data.extended_irq;
    if (eirq.producer_consumer == ACPI_PRODUCER)
    return AE_OK;
    if (ctx.index >= eirq.interrupt_count) {
    ctx.index -= eirq.interrupt_count;
    return AE_OK;
    }
    fwnode = acpi_get_irq_source_fwhandle(&eirq.resource_source,
    eirq.interrupts[ctx.index]);
    acpi_irq_parse_one_match(fwnode, eirq.interrupts[ctx.index],
    eirq.triggering, eirq.polarity,
    eirq.shareable, eirq.wake_capable, ctx);
    return AE_CTRL_TERMINATE;
    }
    return AE_OK;
    }
//
// acpi_irq_parse_one - Resolve an interrupt for a device
// @handle: the device whose interrupt is to be resolved
// @index: index of the interrupt to resolve
// @fwspec: structure irq_fwspec filled by this function
// @flags: resource flags filled by this function
//
// Description:
// Resolves an interrupt for a device by walking its CRS resources to find
// the appropriate ACPI IRQ resource and populating the given struct irq_fwspec
// and flags.
//
// Return:
// The result stored in ctx.rc by the callback, or the default -EINVAL value
// if an error occurs.
//
    static int acpi_irq_parse_one(acpi_handle handle, unsigned int index,
    struct irq_fwspec *fwspec, unsigned long *flags)
    {
    let mut ctx: acpi_irq_parse_one_ctx = { -EINVAL, index, flags, fwspec };
    acpi_walk_resources(handle, METHOD_NAME__CRS, acpi_irq_parse_one_cb, &ctx);
    return ctx.rc;
    }
//
// acpi_irq_get - Lookup an ACPI IRQ resource and use it to initialize resource.
// @handle: ACPI device handle
// @index:  ACPI IRQ resource index to lookup
// @res:    Linux IRQ resource to initialize
//
// Description:
// Look for the ACPI IRQ resource with the given index and use it to initialize
// the given Linux IRQ resource.
//
// Return:
// 0 on success
// -EINVAL if an error occurs
// -EPROBE_DEFER if the IRQ lookup/conversion failed
//
#[no_mangle]
pub unsafe extern "C" fn acpi_irq_get(handle: acpi_handle, index: c_uint, res: *mut resource) -> c_int {
    int acpi_irq_get(acpi_handle handle, unsigned int index, struct resource *res)
    {
    struct irq_fwspec fwspec;
    struct irq_domain *domain;
    unsigned long flags;
    int rc;
    rc = acpi_irq_parse_one(handle, index, &fwspec, &flags);
    if (rc)
    return rc;
    domain = irq_find_matching_fwnode(fwspec.fwnode, DOMAIN_BUS_ANY);
    if (!domain)
    return -EPROBE_DEFER;
    rc = irq_create_fwspec_mapping(&fwspec);
    if (rc <= 0)
    return -EINVAL;
    res.start = rc;
    res.end = rc;
    res.flags = flags;
    return 0;
    }
    EXPORT_SYMBOL_GPL(acpi_irq_get);
    const struct cpumask *acpi_irq_get_affinity(acpi_handle handle,
    unsigned int index)
    {
    struct irq_fwspec_info info;
    struct irq_fwspec fwspec;
    unsigned long flags;
    if (acpi_irq_parse_one(handle, index, &fwspec, &flags))
    return core::ptr::null_mut();
    if (irq_populate_fwspec_info(&fwspec, &info))
    return core::ptr::null_mut();
    if (!(info.flags & IRQ_FWSPEC_INFO_AFFINITY_VALID))
    return core::ptr::null_mut();
    return info.affinity;
    }
//
// acpi_set_irq_model - Setup the GSI irqdomain information
// @model:	the value assigned to acpi_irq_model
// @fn:		a dispatcher function that will return the domain fwnode
// for a given GSI
// @gsi_dep_fn: a function to retrieve the acpi_handle a GSI interrupt is
// dependent on
//
    void __init acpi_set_irq_model(enum acpi_irq_model_id model,
    acpi_gsi_domain_disp_fn fn, acpi_gsi_handle_disp_fn gsi_dep_fn)
    {
    acpi_irq_model = model;
    acpi_get_gsi_domain_id = fn;
    acpi_get_gsi_handle = gsi_dep_fn;
    }
//
// acpi_get_gsi_dispatcher() - Get the GSI dispatcher function
//
// Return the dispatcher function that computes the domain fwnode for
// a given GSI.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_get_gsi_dispatcher() -> acpi_gsi_domain_disp_fn {
    acpi_gsi_domain_disp_fn acpi_get_gsi_dispatcher(void)
    {
    return acpi_get_gsi_domain_id;
    }
    EXPORT_SYMBOL_GPL(acpi_get_gsi_dispatcher);
//
// acpi_set_gsi_to_irq_fallback - Register a GSI transfer
// callback to fallback to arch specified implementation.
// @fn: arch-specific fallback handler
//
#[no_mangle]
pub unsafe extern "C" fn acpi_set_gsi_to_irq_fallback((*fn)(u32): *mut u32) -> void __init {
    void __init acpi_set_gsi_to_irq_fallback(u32 (*fn)(u32))
    {
    acpi_gsi_to_irq_fallback = fn;
    }
//
// acpi_irq_create_hierarchy - Create a hierarchical IRQ domain with the default
// GSI domain as its parent.
// @flags:      Irq domain flags associated with the domain
// @size:       Size of the domain.
// @fwnode:     Optional fwnode of the interrupt controller
// @ops:        Pointer to the interrupt domain callbacks
// @host_data:  Controller private data pointer
//
    struct irq_domain *acpi_irq_create_hierarchy(unsigned int flags,
    unsigned int size,
    struct fwnode_handle *fwnode,
    const struct irq_domain_ops *ops,
    void *host_data)
    {
    struct irq_domain *d;
// This only works for the GIC model...
    if (acpi_irq_model != ACPI_IRQ_MODEL_GIC)
    return core::ptr::null_mut();
    d = irq_find_matching_fwnode(acpi_get_gsi_domain_id(0),
    DOMAIN_BUS_ANY);
    if (!d)
    return core::ptr::null_mut();
    return irq_domain_create_hierarchy(d, flags, size, fwnode, ops,
    host_data);
    }
    EXPORT_SYMBOL_GPL(acpi_irq_create_hierarchy);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_irq_dep_ctx {
    pub rc: c_int,
    pub index: c_uint,
    pub handle: acpi_handle,
}

#[no_mangle]
unsafe extern "C" fn acpi_irq_get_parent(ares: *mut acpi_resource, context: *mut c_void) -> acpi_status {
    static acpi_status acpi_irq_get_parent(struct acpi_resource *ares, void *context)
    {
    struct acpi_irq_dep_ctx *ctx = context;
    struct acpi_resource_irq *irq;
    struct acpi_resource_extended_irq *eirq;
    switch (ares.type) {
    case ACPI_RESOURCE_TYPE_IRQ:
    irq = &ares.data.irq;
    if (ctx.index >= irq.interrupt_count) {
    ctx.index -= irq.interrupt_count;
    return AE_OK;
    }
    ctx.handle = acpi_get_gsi_handle(irq.interrupts[ctx.index]);
    ctx.rc = 0;
    return AE_CTRL_TERMINATE;
    case ACPI_RESOURCE_TYPE_EXTENDED_IRQ:
    eirq = &ares.data.extended_irq;
    if (eirq.producer_consumer == ACPI_PRODUCER)
    return AE_OK;
    if (ctx.index >= eirq.interrupt_count) {
    ctx.index -= eirq.interrupt_count;
    return AE_OK;
    }
// Support GSIs only
    if (eirq.resource_source.string_length)
    return AE_OK;
    ctx.handle = acpi_get_gsi_handle(eirq.interrupts[ctx.index]);
    ctx.rc = 0;
    return AE_CTRL_TERMINATE;
    }
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn acpi_irq_get_dep(handle: acpi_handle, index: c_uint, gsi_handle: *mut acpi_handle) -> c_int {
    static int acpi_irq_get_dep(acpi_handle handle, unsigned int index, acpi_handle *gsi_handle)
    {
    let mut ctx: acpi_irq_dep_ctx = {-EINVAL, index, core::ptr::null_mut()};
    if (!gsi_handle)
    return -EINVAL;
    acpi_walk_resources(handle, METHOD_NAME__CRS, acpi_irq_get_parent, &ctx);
// gsi_handle = ctx.handle;
    return ctx.rc;
    }
#[no_mangle]
unsafe extern "C" fn acpi_prt_entry_valid(prt_entry: *mut c_void) -> bool {
    static bool acpi_prt_entry_valid(void *prt_entry)
    {
    struct acpi_pci_routing_table *entry = prt_entry;
    return entry && entry.length > 0;
    }
    static void *acpi_prt_next_entry(void *prt_entry)
    {
    struct acpi_pci_routing_table *entry = prt_entry;
    return prt_entry + entry.length;
    }
#[no_mangle]
unsafe extern "C" fn acpi_add_prt_dep(handle: acpi_handle) -> u32 {
    static u32 acpi_add_prt_dep(acpi_handle handle)
    {
    let mut buffer: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    struct acpi_pci_routing_table *entry;
    struct acpi_handle_list dep_devices;
    acpi_handle gsi_handle;
    acpi_handle link_handle;
    acpi_status status;
    let mut count: u32 = 0;
    status = acpi_get_irq_routing_table(handle, &buffer);
    if (ACPI_FAILURE(status)) {
    acpi_handle_err(handle, "failed to get IRQ routing table\n");
    kfree(buffer.pointer);
    return 0;
    }
    entry = buffer.pointer;
    for (; acpi_prt_entry_valid(entry); entry = acpi_prt_next_entry(entry)) {
    if (entry.source[0]) {
    status = acpi_get_handle(handle, entry.source, &link_handle);
    if (ACPI_FAILURE(status))
    continue;
    dep_devices.count = 1;
    dep_devices.handles = kzalloc_objs(*dep_devices.handles,
    1);
    if (!dep_devices.handles) {
    acpi_handle_err(handle, "failed to allocate memory\n");
    continue;
    }
    dep_devices.handles[0] = link_handle;
    count += acpi_scan_add_dep(handle, &dep_devices);
    } else {
    gsi_handle = acpi_get_gsi_handle(entry.source_index);
    if (!gsi_handle)
    continue;
    dep_devices.count = 1;
    dep_devices.handles = kzalloc_objs(*dep_devices.handles,
    1);
    if (!dep_devices.handles) {
    acpi_handle_err(handle, "failed to allocate memory\n");
    continue;
    }
    dep_devices.handles[0] = gsi_handle;
    count += acpi_scan_add_dep(handle, &dep_devices);
    }
    }
    kfree(buffer.pointer);
    return count;
    }
#[no_mangle]
unsafe extern "C" fn acpi_add_irq_dep(handle: acpi_handle) -> u32 {
    static u32 acpi_add_irq_dep(acpi_handle handle)
    {
    struct acpi_handle_list dep_devices;
    acpi_handle gsi_handle;
    let mut count: u32 = 0;
    int i;
    for (i = 0; !acpi_irq_get_dep(handle, i, &gsi_handle); i++) {
    if (!gsi_handle)
    continue;
    dep_devices.count = 1;
    dep_devices.handles = kzalloc_objs(*dep_devices.handles, 1);
    if (!dep_devices.handles) {
    acpi_handle_err(handle, "failed to allocate memory\n");
    continue;
    }
    dep_devices.handles[0] = gsi_handle;
    count += acpi_scan_add_dep(handle, &dep_devices);
    }
    return count;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_irq_add_auto_dep(handle: acpi_handle) -> u32 {
    u32 acpi_irq_add_auto_dep(acpi_handle handle)
    {
    if (!acpi_get_gsi_handle)
    return 0;
    if (acpi_has_method(handle, "_PRT"))
    return acpi_add_prt_dep(handle);
    return acpi_add_irq_dep(handle);
    }
