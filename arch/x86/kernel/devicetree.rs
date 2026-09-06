//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/devicetree.c
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
// Architecture specific OF callbacks.
//

    __initdata u64 initial_dtb;
    char __initdata cmd_line[COMMAND_LINE_SIZE];
    int __initdata of_ioapic;
#[no_mangle]
pub unsafe extern "C" fn add_dtb(data: u64) -> void __init {
    void __init add_dtb(u64 data)
    {
    initial_dtb = data + offsetof(struct setup_data, data);
    }
//
// CE4100 ids. Will be moved to machine_device_initcall() once we have it.
//
    static struct of_device_id __initdata ce4100_ids[] = {
    { .compatible = "intel,ce4100-cp", },
    { .compatible = "isa", },
    { .compatible = "pci", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn add_bus_probe() -> int __init {
    static int __init add_bus_probe(void)
    {
    if (!of_have_populated_dt())
    return 0;
    return of_platform_bus_probe(core::ptr::null_mut(), ce4100_ids, core::ptr::null_mut());
    }
    device_initcall(add_bus_probe);

    struct device_node *pcibios_get_phb_of_node(struct pci_bus *bus)
    {
    struct device_node *np;
    for_each_node_by_type(np, "pci") {
    const void *prop;
    unsigned int bus_min;
    prop = of_get_property(np, "bus-range", core::ptr::null_mut());
    if (!prop)
    continue;
    bus_min = be32_to_cpup(prop);
    if (bus.number == bus_min)
    return np;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn x86_of_pci_irq_enable(dev: *mut pci_dev) -> c_int {
    static int x86_of_pci_irq_enable(struct pci_dev *dev)
    {
    u32 virq;
    int ret;
    u8 pin;
    ret = pci_read_config_byte(dev, PCI_INTERRUPT_PIN, &pin);
    if (ret)
    return pcibios_err_to_errno(ret);
    if (!pin)
    return 0;
    virq = of_irq_parse_and_map_pci(dev, 0, 0);
    if (virq == 0)
    return -EINVAL;
    dev.irq = virq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn x86_of_pci_irq_disable(dev: *mut pci_dev) {
    static void x86_of_pci_irq_disable(struct pci_dev *dev)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn x86_of_pci_init() {
    void x86_of_pci_init(void)
    {
    pcibios_enable_irq = x86_of_pci_irq_enable;
    pcibios_disable_irq = x86_of_pci_irq_disable;
    }

#[no_mangle]
unsafe extern "C" fn dtb_setup_hpet() -> void __init {
    static void __init dtb_setup_hpet(void)
    {

    struct device_node *dn;
    struct resource r;
    int ret;
    dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "intel,ce4100-hpet");
    if (!dn)
    return;
    ret = of_address_to_resource(dn, 0, &r);
    if (ret) {
    WARN_ON(1);
    return;
    }
    hpet_address = r.start;

    }

pub const WAKEUP_MAILBOX_SIZE: c_uint = 0x1000;
pub const WAKEUP_MAILBOX_ALIGN: c_uint = 0x1000;
// dtb_wakeup_mailbox_setup() - Parse the wakeup mailbox from the device tree
//
// Look for the presence of a wakeup mailbox in the DeviceTree. The mailbox is
// expected to follow the structure and operation described in the Multiprocessor
// Wakeup Structure of the ACPI specification.
//
#[no_mangle]
unsafe extern "C" fn dtb_wakeup_mailbox_setup() -> void __init {
    static void __init dtb_wakeup_mailbox_setup(void)
    {
    struct device_node *node;
    struct resource res;
    node = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "intel,wakeup-mailbox");
    if (!node)
    return;
    if (of_address_to_resource(node, 0, &res))
    goto done;
// The mailbox is a 4KB-aligned region.
    if (res.start & (WAKEUP_MAILBOX_ALIGN - 1))
    goto done;
// The mailbox has a size of 4KB.
    if (res.end - res.start + 1 != WAKEUP_MAILBOX_SIZE)
    goto done;
// Not supported when the mailbox is used.
    cpu_hotplug_disable_offlining();
    acpi_setup_mp_wakeup_mailbox(res.start);
    done:
    of_node_put(node);
    }

#[no_mangle]
pub unsafe extern "C" fn dtb_wakeup_mailbox_setup() -> c_int {
    static inline int dtb_wakeup_mailbox_setup(void)
    {
    return -EOPNOTSUPP;
    }

#[no_mangle]
unsafe extern "C" fn dtb_cpu_setup() -> void __init {
    static void __init dtb_cpu_setup(void)
    {
    struct device_node *dn;
    u32 apic_id;
    for_each_of_cpu_node(dn) {
    apic_id = of_get_cpu_hwid(dn, 0);
    if (apic_id == ~0U) {
    pr_warn("%pOF: missing local APIC ID\n", dn);
    continue;
    }
    topology_register_apic(apic_id, CPU_ACPIID_INVALID, true);
    set_apicid_to_node(apic_id, of_node_to_nid(dn));
    }
    }
#[no_mangle]
unsafe extern "C" fn dtb_lapic_setup() -> void __init {
    static void __init dtb_lapic_setup(void)
    {
    struct device_node *dn;
    struct resource r;
    let mut lapic_addr: c_ulong = APIC_DEFAULT_PHYS_BASE;
    int ret;
    dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "intel,ce4100-lapic");
    if (dn) {
    ret = of_address_to_resource(dn, 0, &r);
    if (WARN_ON(ret))
    return;
    lapic_addr = r.start;
    }
// Did the boot loader setup the local APIC ?
    if (!boot_cpu_has(X86_FEATURE_APIC)) {
// Try force enabling, which registers the APIC address
    if (!apic_force_enable(lapic_addr))
    return;
    } else {
    register_lapic_address(lapic_addr);
    }
    smp_found_config = 1;
    pic_mode = !of_property_read_bool(dn, "intel,virtual-wire-mode");
    pr_info("%s compatibility mode.\n", pic_mode ? "IMCR and PIC" : "Virtual Wire");
    }

    static unsigned int ioapic_id;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_ioapic_type {
    pub out_type: u32,
    pub is_level: u32,
    pub active_low: u32,
}

    static struct of_ioapic_type of_ioapic_type[] =
    {
    {
    .out_type	= IRQ_TYPE_EDGE_FALLING,
    .is_level	= 0,
    .active_low	= 1,
    },
    {
    .out_type	= IRQ_TYPE_LEVEL_HIGH,
    .is_level	= 1,
    .active_low	= 0,
    },
    {
    .out_type	= IRQ_TYPE_LEVEL_LOW,
    .is_level	= 1,
    .active_low	= 1,
    },
    {
    .out_type	= IRQ_TYPE_EDGE_RISING,
    .is_level	= 0,
    .active_low	= 0,
    },
    };
    static int dt_irqdomain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *arg)
    {
    struct irq_fwspec *fwspec = (struct irq_fwspec *)arg;
    struct of_ioapic_type *it;
    struct irq_alloc_info tmp;
    int type_index;
    if (WARN_ON(fwspec.param_count < 2))
    return -EINVAL;
    type_index = fwspec.param[1];
    if (type_index >= ARRAY_SIZE(of_ioapic_type))
    return -EINVAL;
    it = &of_ioapic_type[type_index];
    ioapic_set_alloc_attr(&tmp, NUMA_NO_NODE, it.is_level, it.active_low);
    tmp.devid = mpc_ioapic_id(mp_irqdomain_ioapic_idx(domain));
    tmp.ioapic.pin = fwspec.param[0];
    return mp_irqdomain_alloc(domain, virq, nr_irqs, &tmp);
    }
    static const struct irq_domain_ops ioapic_irq_domain_ops = {
    .alloc		= dt_irqdomain_alloc,
    .free		= mp_irqdomain_free,
    .activate	= mp_irqdomain_activate,
    .deactivate	= mp_irqdomain_deactivate,
    };
#[no_mangle]
unsafe extern "C" fn dtb_add_ioapic(dn: *mut device_node) -> void __init {
    static void __init dtb_add_ioapic(struct device_node *dn)
    {
    struct resource r;
    int ret;
    struct ioapic_domain_cfg cfg = {
    .type = IOAPIC_DOMAIN_DYNAMIC,
    .ops = &ioapic_irq_domain_ops,
    .dev = dn,
    };
    ret = of_address_to_resource(dn, 0, &r);
    if (ret) {
    pr_err("Can't obtain address from device node %pOF.\n", dn);
    return;
    }
    mp_register_ioapic(++ioapic_id, r.start, gsi_top, &cfg);
    }
#[no_mangle]
unsafe extern "C" fn dtb_ioapic_setup() -> void __init {
    static void __init dtb_ioapic_setup(void)
    {
    struct device_node *dn;
    for_each_compatible_node(dn, core::ptr::null_mut(), "intel,ce4100-ioapic")
    dtb_add_ioapic(dn);
    if (nr_ioapics) {
    of_ioapic = 1;
    return;
    }
    pr_err("Error: No information about IO-APIC in OF.\n");
    }

    static void __init dtb_ioapic_setup(void) {}

#[no_mangle]
unsafe extern "C" fn dtb_apic_setup() -> void __init {
    static void __init dtb_apic_setup(void)
    {

    dtb_lapic_setup();
    dtb_cpu_setup();

    dtb_ioapic_setup();
    }
#[no_mangle]
unsafe extern "C" fn x86_dtb_parse_smp_config() -> void __init {
    static void __init x86_dtb_parse_smp_config(void)
    {
    if (!of_have_populated_dt())
    return;
    dtb_setup_hpet();
    dtb_apic_setup();
    dtb_wakeup_mailbox_setup();
    }
#[no_mangle]
pub unsafe extern "C" fn x86_flattree_get_config() -> void __init {
    void __init x86_flattree_get_config(void)
    {

    u32 size, map_len;
    void *dt;
    if (initial_dtb) {
    map_len = max(PAGE_SIZE - (initial_dtb & ~PAGE_MASK), (u64)128);
    dt = early_memremap(initial_dtb, map_len);
    size = fdt_totalsize(dt);
    if (map_len < size) {
    early_memunmap(dt, map_len);
    dt = early_memremap(initial_dtb, size);
    map_len = size;
    }
    early_init_dt_verify(dt, __pa(dt));
    }
    unflatten_and_copy_device_tree();
    if (initial_dtb)
    early_memunmap(dt, map_len);

    if (acpi_disabled && of_have_populated_dt())
    x86_init.mpparse.parse_smp_cfg = x86_dtb_parse_smp_config;
    }
