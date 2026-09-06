//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/jailhouse.c
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
// Jailhouse paravirt_ops implementation
//
// Copyright (c) Siemens AG, 2015-2017
//
// Authors:
// Jan Kiszka <jan.kiszka@siemens.com>
//

    static struct jailhouse_setup_data setup_data;

    static unsigned int precalibrated_tsc_khz;
#[no_mangle]
unsafe extern "C" fn jailhouse_setup_irq(irq: c_uint) {
    static void jailhouse_setup_irq(unsigned int irq)
    {
    struct mpc_intsrc mp_irq = {
    .type		= MP_INTSRC,
    .irqtype	= mp_INT,
    .irqflag	= MP_IRQPOL_ACTIVE_HIGH | MP_IRQTRIG_EDGE,
    .srcbusirq	= irq,
    .dstirq		= irq,
    };
    mp_save_irq(&mp_irq);
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_cpuid_base() -> u32 {
    static uint32_t jailhouse_cpuid_base(void)
    {
    if (boot_cpu_data.cpuid_level < 0 ||
    !boot_cpu_has(X86_FEATURE_HYPERVISOR))
    return 0;
    return cpuid_base_hypervisor("Jailhouse\0\0\0", 0);
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_detect() -> uint32_t __init {
    static uint32_t __init jailhouse_detect(void)
    {
    return jailhouse_cpuid_base();
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_get_wallclock(now: *mut timespec64) {
    static void jailhouse_get_wallclock(struct timespec64 *now)
    {
    memset(now, 0, sizeof(*now));
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_timer_init() -> void __init {
    static void __init jailhouse_timer_init(void)
    {
    lapic_timer_period = setup_data.v1.apic_khz * (1000 / HZ);
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_get_tsc() -> c_ulong {
    static unsigned long jailhouse_get_tsc(void)
    {
    return precalibrated_tsc_khz;
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_x2apic_init() -> void __init {
    static void __init jailhouse_x2apic_init(void)
    {

    if (!x2apic_enabled())
    return;
//
// We do not have access to IR inside Jailhouse non-root cells.  So
// we have to run in physical mode.
//
    x2apic_phys = 1;
//
// This will trigger the switch to apic_x2apic_phys.  Empty OEM IDs
// ensure that only this APIC driver picks up the call.
//
    default_acpi_madt_oem_check("", "");

    }
#[no_mangle]
unsafe extern "C" fn jailhouse_parse_smp_config() -> void __init {
    static void __init jailhouse_parse_smp_config(void)
    {
    struct ioapic_domain_cfg ioapic_cfg = {
    .type = IOAPIC_DOMAIN_STRICT,
    .ops = &mp_ioapic_irqdomain_ops,
    };
    unsigned int cpu;
    jailhouse_x2apic_init();
    register_lapic_address(0xfee00000);
    for (cpu = 0; cpu < setup_data.v1.num_cpus; cpu++)
    topology_register_apic(setup_data.v1.cpu_ids[cpu], CPU_ACPIID_INVALID, true);
    smp_found_config = 1;
    if (setup_data.v1.standard_ioapic) {
    mp_register_ioapic(0, 0xfec00000, gsi_top, &ioapic_cfg);
    if (IS_ENABLED(CONFIG_SERIAL_8250) &&
    setup_data.hdr.version < 2) {
// Register 1:1 mapping for legacy UART IRQs 3 and 4
    jailhouse_setup_irq(3);
    jailhouse_setup_irq(4);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_no_restart() {
    static void jailhouse_no_restart(void)
    {
    pr_notice("Jailhouse: Restart not supported, halting\n");
    machine_halt();
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_pci_arch_init() -> int __init {
    static int __init jailhouse_pci_arch_init(void)
    {
    pci_direct_init(1);
//
// There are no bridges on the virtual PCI root bus under Jailhouse,
// thus no other way to discover all devices than a full scan.
// Respect any overrides via the command line, though.
//
    if (pcibios_last_bus < 0)
    pcibios_last_bus = 0xff;

    if (setup_data.v1.pci_mmconfig_base) {
    pci_mmconfig_add(0, 0, pcibios_last_bus,
    setup_data.v1.pci_mmconfig_base);
    pci_mmcfg_arch_init();
    }

    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn jailhouse_uart_enabled(uart_nr: c_uint) -> bool {
    static inline bool jailhouse_uart_enabled(unsigned int uart_nr)
    {
    return setup_data.v2.flags & BIT(uart_nr);
    }
    static void jailhouse_serial_fixup(int port, struct uart_port *up,
    u32 *capabilities)
    {
    static const u16 pcuart_base[] = {0x3f8, 0x2f8, 0x3e8, 0x2e8};
    unsigned int n;
    for (n = 0; n < ARRAY_SIZE(pcuart_base); n++) {
    if (pcuart_base[n] != up.iobase)
    continue;
    if (jailhouse_uart_enabled(n)) {
    pr_info("Enabling UART%u (port 0x%lx)\n", n,
    up.iobase);
    jailhouse_setup_irq(up.irq);
    } else {
// Deactivate UART if access isn't allowed
    up.iobase = 0;
    }
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_serial_workaround() -> void __init {
    static void __init jailhouse_serial_workaround(void)
    {
//
// There are flags inside setup_data that indicate availability of
// platform UARTs since setup data version 2.
//
// In case of version 1, we don't know which UARTs belong Linux. In
// this case, unconditionally register 1:1 mapping for legacy UART IRQs
// 3 and 4.
//
    if (setup_data.hdr.version > 1)
    serial8250_set_isa_configurator(jailhouse_serial_fixup);
    }

#[no_mangle]
pub unsafe extern "C" fn jailhouse_serial_workaround() {
    static inline void jailhouse_serial_workaround(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn jailhouse_init_platform() -> void __init {
    static void __init jailhouse_init_platform(void)
    {
    let mut pa_data: u64 = boot_params.hdr.setup_data;
    unsigned long setup_data_len;
    struct setup_data header;
    void *mapping;
    x86_init.irqs.pre_vector_init		= x86_init_noop;
    x86_init.timers.timer_init		= jailhouse_timer_init;
    x86_init.mpparse.find_mptable		= x86_init_noop;
    x86_init.mpparse.early_parse_smp_cfg	= x86_init_noop;
    x86_init.mpparse.parse_smp_cfg		= jailhouse_parse_smp_config;
    x86_init.pci.arch_init			= jailhouse_pci_arch_init;
    x86_platform.calibrate_cpu		= jailhouse_get_tsc;
    x86_platform.calibrate_tsc		= jailhouse_get_tsc;
    x86_platform.get_wallclock		= jailhouse_get_wallclock;
    x86_platform.legacy.rtc			= 0;
    x86_platform.legacy.warm_reset		= 0;
    x86_platform.legacy.i8042		= X86_LEGACY_I8042_PLATFORM_ABSENT;
    legacy_pic				= &null_legacy_pic;
    machine_ops.emergency_restart		= jailhouse_no_restart;
    while (pa_data) {
    mapping = early_memremap(pa_data, sizeof(header));
    memcpy(&header, mapping, sizeof(header));
    early_memunmap(mapping, sizeof(header));
    if (header.type == SETUP_JAILHOUSE)
    break;
    pa_data = header.next;
    }
    if (!pa_data)
    panic("Jailhouse: No valid setup data found");
// setup data must at least contain the header
    if (header.len < sizeof(setup_data.hdr))
    goto unsupported;
    pa_data += offsetof(struct setup_data, data);
    setup_data_len = min_t(unsigned long, sizeof(setup_data),
    (unsigned long)header.len);
    mapping = early_memremap(pa_data, setup_data_len);
    memcpy(&setup_data, mapping, setup_data_len);
    early_memunmap(mapping, setup_data_len);
    if (setup_data.hdr.version == 0 ||
    setup_data.hdr.compatible_version !=
    JAILHOUSE_SETUP_REQUIRED_VERSION ||
    (setup_data.hdr.version == 1 && header.len < SETUP_DATA_V1_LEN) ||
    (setup_data.hdr.version >= 2 && header.len < SETUP_DATA_V2_LEN))
    goto unsupported;
    pmtmr_ioport = setup_data.v1.pm_timer_address;
    pr_debug("Jailhouse: PM-Timer IO Port: %#x\n", pmtmr_ioport);
    precalibrated_tsc_khz = setup_data.v1.tsc_khz;
    setup_force_cpu_cap(X86_FEATURE_TSC_KNOWN_FREQ);
    pci_probe = 0;
//
// Avoid that the kernel complains about missing ACPI tables - there
// are none in a non-root cell.
//
    disable_acpi();
    jailhouse_serial_workaround();
    return;
    unsupported:
    panic("Jailhouse: Unsupported setup data structure");
    }
#[no_mangle]
pub unsafe extern "C" fn jailhouse_paravirt() -> bool {
    bool jailhouse_paravirt(void)
    {
    return jailhouse_cpuid_base() != 0;
    }
#[no_mangle]
unsafe extern "C" fn jailhouse_x2apic_available() -> bool __init {
    static bool __init jailhouse_x2apic_available(void)
    {
//
// The x2APIC is only available if the root cell enabled it. Jailhouse
// does not support switching between xAPIC and x2APIC.
//
    return x2apic_enabled();
    }
    const struct hypervisor_x86 x86_hyper_jailhouse __refconst = {
    .name			= "Jailhouse",
    .detect			= jailhouse_detect,
    .init.init_platform	= jailhouse_init_platform,
    .init.x2apic_available	= jailhouse_x2apic_available,
    .ignore_nopv		= true,
    };
