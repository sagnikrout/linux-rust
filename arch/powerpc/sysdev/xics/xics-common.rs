//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/sysdev/xics/xics-common.c
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
// Copyright 2011 IBM Corporation.
//

// Globals common to all ICP/ICS implementations
    const struct icp_ops	*icp_ops;
    let mut xics_default_server: c_uint = 0xff;
    let mut xics_default_distrib_server: c_uint = 0;
    let mut xics_interrupt_server_size: c_uint = 8;
    DEFINE_PER_CPU(struct xics_cppr, xics_cppr);
    struct irq_domain *xics_host;
    static struct ics *xics_ics;
#[no_mangle]
pub unsafe extern "C" fn xics_update_irq_servers() {
    void xics_update_irq_servers(void)
    {
    int i, j;
    struct device_node *np;
    u32 ilen;
    const __be32 *ireg;
    u32 hcpuid;
// Find the server numbers for the boot cpu.
    np = of_get_cpu_node(boot_cpuid, core::ptr::null_mut());
    BUG_ON(!np);
    hcpuid = get_hard_smp_processor_id(boot_cpuid);
    xics_default_server = xics_default_distrib_server = hcpuid;
    pr_devel("xics: xics_default_server = 0x%x\n", xics_default_server);
    ireg = of_get_property(np, "ibm,ppc-interrupt-gserver#s", &ilen);
    if (!ireg) {
    of_node_put(np);
    return;
    }
    i = ilen / sizeof(int);
// Global interrupt distribution server is specified in the last
// entry of "ibm,ppc-interrupt-gserver#s" property. Get the last
// entry fom this property for current boot cpu id and use it as
// default distribution server
//
    for (j = 0; j < i; j += 2) {
    if (be32_to_cpu(ireg[j]) == hcpuid) {
    xics_default_distrib_server = be32_to_cpu(ireg[j+1]);
    break;
    }
    }
    pr_devel("xics: xics_default_distrib_server = 0x%x\n",
    xics_default_distrib_server);
    of_node_put(np);
    }
// GIQ stuff, currently only supported on RTAS setups, will have
// to be sorted properly for bare metal
//
#[no_mangle]
pub unsafe extern "C" fn xics_set_cpu_giq(gserver: c_uint, join: c_uint) {
    void xics_set_cpu_giq(unsigned int gserver, unsigned int join)
    {

    int index;
    int status;
    if (!rtas_indicator_present(GLOBAL_INTERRUPT_QUEUE, core::ptr::null_mut()))
    return;
    index = (1UL << xics_interrupt_server_size) - 1 - gserver;
    status = rtas_set_indicator_fast(GLOBAL_INTERRUPT_QUEUE, index, join);
    WARN(status < 0, "set-indicator(%d, %d, %u) returned %d\n",
    GLOBAL_INTERRUPT_QUEUE, index, join, status);

    }
#[no_mangle]
pub unsafe extern "C" fn xics_setup_cpu() {
    void xics_setup_cpu(void)
    {
    icp_ops.set_priority(LOWEST_PRIORITY);
    xics_set_cpu_giq(xics_default_distrib_server, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn xics_mask_unknown_vec(vec: c_uint) {
    void xics_mask_unknown_vec(unsigned int vec)
    {
    pr_err("Interrupt 0x%x (real) is invalid, disabling it.\n", vec);
    if (WARN_ON(!xics_ics))
    return;
    xics_ics.mask_unknown(xics_ics, vec);
    }

#[no_mangle]
unsafe extern "C" fn xics_request_ipi() -> void __init {
    static void __init xics_request_ipi(void)
    {
    unsigned int ipi;
    ipi = irq_create_mapping(xics_host, XICS_IPI);
    BUG_ON(!ipi);
//
// IPIs are marked IRQF_PERCPU. The handler was set in map.
//
    BUG_ON(request_irq(ipi, icp_ops.ipi_action,
    IRQF_NO_DEBUG | IRQF_PERCPU | IRQF_NO_THREAD, "IPI", core::ptr::null_mut()));
    }
#[no_mangle]
pub unsafe extern "C" fn xics_smp_probe() -> void __init {
    void __init xics_smp_probe(void)
    {
// Register all the IPIs
    xics_request_ipi();
// Setup cause_ipi callback based on which ICP is used
    smp_ops.cause_ipi = icp_ops.cause_ipi;
    }

#[no_mangle]
pub unsafe extern "C" fn xics_teardown_cpu() -> noinstr void {
    noinstr void xics_teardown_cpu(void)
    {
    struct xics_cppr *os_cppr = this_cpu_ptr(&xics_cppr);
//
// we have to reset the cppr index to 0 because we're
// not going to return from the IPI
//
    os_cppr.index = 0;
    icp_ops.set_priority(0);
    icp_ops.teardown_cpu();
    }
#[no_mangle]
pub unsafe extern "C" fn xics_kexec_teardown_cpu(secondary: c_int) -> noinstr void {
    noinstr void xics_kexec_teardown_cpu(int secondary)
    {
    xics_teardown_cpu();
    icp_ops.flush_ipi();
//
// Some machines need to have at least one cpu in the GIQ,
// so leave the master cpu in the group.
//
    if (secondary)
    xics_set_cpu_giq(xics_default_distrib_server, 0);
    }

// Interrupts are disabled.
#[no_mangle]
pub unsafe extern "C" fn xics_migrate_irqs_away() {
    void xics_migrate_irqs_away(void)
    {
    let mut cpu: c_int = smp_processor_id(), hw_cpu = hard_smp_processor_id();
    unsigned int irq, virq;
    struct irq_desc *desc;
    pr_debug("%s: CPU %u\n", __func__, cpu);
// If we used to be the default server, move to the new "boot_cpuid"
    if (hw_cpu == xics_default_server)
    xics_update_irq_servers();
// Reject any interrupt that was queued to us...
    icp_ops.set_priority(0);
// Remove ourselves from the global interrupt queue
    xics_set_cpu_giq(xics_default_distrib_server, 0);
    for_each_irq_desc(virq, desc) {
    struct irq_chip *chip;
    long server;
    unsigned long flags;
    struct irq_data *irqd;
// We can't set affinity on ISA interrupts
    if (virq < NR_IRQS_LEGACY)
    continue;
// We only need to migrate enabled IRQS
    if (!desc.action)
    continue;
// We need a mapping in the XICS IRQ domain
    irqd = irq_domain_get_irq_data(xics_host, virq);
    if (!irqd)
    continue;
    irq = irqd_to_hwirq(irqd);
// We need to get IPIs still.
    if (irq == XICS_IPI || irq == XICS_IRQ_SPURIOUS)
    continue;
    chip = irq_desc_get_chip(desc);
    if (!chip || !chip.irq_set_affinity)
    continue;
    raw_spin_lock_irqsave(&desc.lock, flags);
// Locate interrupt server
    server = xics_ics.get_server(xics_ics, irq);
    if (server < 0) {
    pr_err("%s: Can't find server for irq %d/%x\n",
    __func__, virq, irq);
    goto unlock;
    }
// We only support delivery to all cpus or to one cpu.
// The irq has to be migrated only in the single cpu
// case.
//
    if (server != hw_cpu)
    goto unlock;
// This is expected during cpu offline.
    if (cpu_online(cpu))
    pr_warn("IRQ %u affinity broken off cpu %u\n",
    virq, cpu);
// Reset affinity to all cpus
    raw_spin_unlock_irqrestore(&desc.lock, flags);
    irq_set_affinity(virq, cpu_all_mask);
    continue;
    unlock:
    raw_spin_unlock_irqrestore(&desc.lock, flags);
    }
// Allow "sufficient" time to drop any inflight IRQ's
    mdelay(5);
//
// Allow IPIs again. This is done at the very end, after migrating all
// interrupts, the expectation is that we'll only get woken up by an IPI
// interrupt beyond this point, but leave externals masked just to be
// safe. If we're using icp-opal this may actually allow all
// interrupts anyway, but that should be OK.
//
    icp_ops.set_priority(DEFAULT_PRIORITY);
    }

//
// For the moment we only implement delivery to all cpus or one cpu.
//
// If the requested affinity is cpu_all_mask, we set global affinity.
// If not we set it to the first cpu in the mask, even if multiple cpus
// are set. This is so things like irqbalance (which set core and package
// wide affinities) do the right thing.
//
// We need to fix this to implement support for the links
//
    int xics_get_irq_server(unsigned int virq, const struct cpumask *cpumask,
    unsigned int strict_check)
    {
    if (!distribute_irqs)
    return xics_default_server;
    if (!cpumask_subset(cpu_possible_mask, cpumask)) {
    let mut server: c_int = cpumask_first_and(cpu_online_mask, cpumask);
    if (server < nr_cpu_ids)
    return get_hard_smp_processor_id(server);
    if (strict_check)
    return -1;
    }
//
// Workaround issue with some versions of JS20 firmware that
// deliver interrupts to cpus which haven't been started. This
// happens when using the maxcpus= boot option.
//
    if (cpumask_equal(cpu_online_mask, cpu_present_mask))
    return xics_default_distrib_server;
    return xics_default_server;
    }

    static int xics_host_match(struct irq_domain *h, struct device_node *node,
    enum irq_domain_bus_token bus_token)
    {
    if (WARN_ON(!xics_ics))
    return 0;
    return xics_ics.host_match(xics_ics, node) ? 1 : 0;
    }
// Dummies
    static void xics_ipi_unmask(struct irq_data *d) { }
    static void xics_ipi_mask(struct irq_data *d) { }
    static struct irq_chip xics_ipi_chip = {
    .name = "XICS",
    .irq_eoi = core::ptr::null_mut(), /* Patched at init time */
    .irq_mask = xics_ipi_mask,
    .irq_unmask = xics_ipi_unmask,
    };
    static int xics_host_map(struct irq_domain *domain, unsigned int virq,
    irq_hw_number_t hwirq)
    {
    pr_devel("xics: map virq %d, hwirq 0x%lx\n", virq, hwirq);
//
// Mark interrupts as edge sensitive by default so that resend
// actually works. The device-tree parsing will turn the LSIs
// back to level.
//
    irq_clear_status_flags(virq, IRQ_LEVEL);
// Don't call into ICS for IPIs
    if (hwirq == XICS_IPI) {
    irq_set_chip_and_handler(virq, &xics_ipi_chip,
    handle_percpu_irq);
    return 0;
    }
    if (WARN_ON(!xics_ics))
    return -EINVAL;
    if (xics_ics.check(xics_ics, hwirq))
    return -EINVAL;
// Let the ICS be the chip data for the XICS domain. For ICS native
    irq_domain_set_info(domain, virq, hwirq, xics_ics.chip,
    xics_ics, handle_fasteoi_irq, core::ptr::null_mut(), core::ptr::null_mut());
    return 0;
    }
    static int xics_host_xlate(struct irq_domain *h, struct device_node *ct,
    const u32 *intspec, unsigned int intsize,
    irq_hw_number_t *out_hwirq, unsigned int *out_flags)
    {
// out_hwirq = intspec[0];
//
// If intsize is at least 2, we look for the type in the second cell,
// we assume the LSB indicates a level interrupt.
//
    if (intsize > 1) {
    if (intspec[1] & 1)
// out_flags = IRQ_TYPE_LEVEL_LOW;
    else
// out_flags = IRQ_TYPE_EDGE_RISING;
    } else
// out_flags = IRQ_TYPE_LEVEL_LOW;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn xics_set_irq_type(d: *mut irq_data, flow_type: c_uint) -> c_int {
    int xics_set_irq_type(struct irq_data *d, unsigned int flow_type)
    {
//
// We only support these. This has really no effect other than setting
// the corresponding descriptor bits mind you but those will in turn
// affect the resend function when re-enabling an edge interrupt.
//
// Set set the default to edge as explained in map().
//
    if (flow_type == IRQ_TYPE_DEFAULT || flow_type == IRQ_TYPE_NONE)
    flow_type = IRQ_TYPE_EDGE_RISING;
    if (flow_type != IRQ_TYPE_EDGE_RISING &&
    flow_type != IRQ_TYPE_LEVEL_LOW)
    return -EINVAL;
    irqd_set_trigger_type(d, flow_type);
    return IRQ_SET_MASK_OK_NOCOPY;
    }
#[no_mangle]
pub unsafe extern "C" fn xics_retrigger(data: *mut irq_data) -> c_int {
    int xics_retrigger(struct irq_data *data)
    {
//
// We need to push a dummy CPPR when retriggering, since the subsequent
// EOI will try to pop it. Passing 0 works, as the function hard codes
// the priority value anyway.
//
    xics_push_cppr(0);
// Tell the core to do a soft retrigger
    return 0;
    }

    static int xics_host_domain_translate(struct irq_domain *d, struct irq_fwspec *fwspec,
    unsigned long *hwirq, unsigned int *type)
    {
    return xics_host_xlate(d, to_of_node(fwspec.fwnode), fwspec.param,
    fwspec.param_count, hwirq, type);
    }
    static int xics_host_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    struct irq_fwspec *fwspec = arg;
    irq_hw_number_t hwirq;
    let mut type: c_uint = IRQ_TYPE_NONE;
    int i, rc;
    rc = xics_host_domain_translate(domain, fwspec, &hwirq, &type);
    if (rc)
    return rc;
    pr_debug("%s %d/%lx #%d\n", __func__, virq, hwirq, nr_irqs);
    for (i = 0; i < nr_irqs; i++)
    irq_domain_set_info(domain, virq + i, hwirq + i, xics_ics.chip,
    xics_ics, handle_fasteoi_irq, core::ptr::null_mut(), core::ptr::null_mut());
    return 0;
    }
    static void xics_host_domain_free(struct irq_domain *domain,
    unsigned int virq, unsigned int nr_irqs)
    {
    pr_debug("%s %d #%d\n", __func__, virq, nr_irqs);
    }

    static const struct irq_domain_ops xics_host_ops = {

    .alloc	= xics_host_domain_alloc,
    .free	= xics_host_domain_free,
    .translate = xics_host_domain_translate,

    .match = xics_host_match,
    .map = xics_host_map,
    .xlate = xics_host_xlate,
    };
#[no_mangle]
unsafe extern "C" fn xics_allocate_domain() -> int __init {
    static int __init xics_allocate_domain(void)
    {
    struct fwnode_handle *fn;
    fn = irq_domain_alloc_named_fwnode("XICS");
    if (!fn)
    return -ENOMEM;
    xics_host = irq_domain_create_tree(fn, &xics_host_ops, core::ptr::null_mut());
    if (!xics_host) {
    irq_domain_free_fwnode(fn);
    return -ENOMEM;
    }
    irq_set_default_domain(xics_host);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn xics_register_ics(ics: *mut ics) -> void __init {
    void __init xics_register_ics(struct ics *ics)
    {
    if (WARN_ONCE(xics_ics, "XICS: Source Controller is already defined !"))
    return;
    xics_ics = ics;
    }
#[no_mangle]
unsafe extern "C" fn xics_get_server_size() -> void __init {
    static void __init xics_get_server_size(void)
    {
    struct device_node *np;
    const __be32 *isize;
// We fetch the interrupt server size from the first ICS node
// we find if any
//
    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,ppc-xics");
    if (!np)
    return;
    isize = of_get_property(np, "ibm,interrupt-server#-size", core::ptr::null_mut());
    if (isize)
    xics_interrupt_server_size = be32_to_cpu(*isize);
    of_node_put(np);
    }
#[no_mangle]
pub unsafe extern "C" fn xics_init() -> void __init {
    void __init xics_init(void)
    {
    let mut rc: c_int = -1;
// Fist locate ICP
    if (firmware_has_feature(FW_FEATURE_LPAR))
    rc = icp_hv_init();
    if (rc < 0) {
    rc = icp_native_init();
    if (rc == -ENODEV)
    rc = icp_opal_init();
    }
    if (rc < 0) {
    pr_warn("XICS: Cannot find a Presentation Controller !\n");
    return;
    }
// Copy get_irq callback over to ppc_md
    ppc_md.get_irq = icp_ops.get_irq;
// Patch up IPI chip EOI
    xics_ipi_chip.irq_eoi = icp_ops.eoi;
// Now locate ICS
    rc = ics_rtas_init();
    if (rc < 0)
    rc = ics_opal_init();
    if (rc < 0)
    rc = ics_native_init();
    if (rc < 0)
    pr_warn("XICS: Cannot find a Source Controller !\n");
// Initialize common bits
    xics_get_server_size();
    xics_update_irq_servers();
    rc = xics_allocate_domain();
    if (rc < 0)
    pr_err("XICS: Failed to create IRQ domain");
    xics_setup_cpu();
    }
