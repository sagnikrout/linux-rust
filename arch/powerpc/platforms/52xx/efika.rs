//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/52xx/efika.c
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


//
// Efika 5K2 platform code
// Some code really inspired from the lite5200b platform.
//
// Copyright (C) 2006 bplan GmbH
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// ------------------------------------------------------------------------
// PCI accesses thru RTAS
// ------------------------------------------------------------------------

//
// Access functions for PCI config space using RTAS calls.
//
    static int rtas_read_config(struct pci_bus *bus, unsigned int devfn, int offset,
    int len, u32 * val)
    {
    struct pci_controller *hose = pci_bus_to_host(bus);
    unsigned long addr = (offset & 0xff) | ((devfn & 0xff) << 8)
    | (((bus.number - hose.first_busno) & 0xff) << 16)
    | (hose.global_number << 24);
    let mut ret: c_int = -1;
    int rval;
    rval = rtas_call(rtas_function_token(RTAS_FN_READ_PCI_CONFIG), 2, 2, &ret, addr, len);
// val = ret;
    return rval ? PCIBIOS_DEVICE_NOT_FOUND : PCIBIOS_SUCCESSFUL;
    }
    static int rtas_write_config(struct pci_bus *bus, unsigned int devfn,
    int offset, int len, u32 val)
    {
    struct pci_controller *hose = pci_bus_to_host(bus);
    unsigned long addr = (offset & 0xff) | ((devfn & 0xff) << 8)
    | (((bus.number - hose.first_busno) & 0xff) << 16)
    | (hose.global_number << 24);
    int rval;
    rval = rtas_call(rtas_function_token(RTAS_FN_WRITE_PCI_CONFIG), 3, 1, core::ptr::null_mut(),
    addr, len, val);
    return rval ? PCIBIOS_DEVICE_NOT_FOUND : PCIBIOS_SUCCESSFUL;
    }
    static struct pci_ops rtas_pci_ops = {
    .read = rtas_read_config,
    .write = rtas_write_config,
    };
#[no_mangle]
unsafe extern "C" fn efika_pcisetup() -> void __init {
    static void __init efika_pcisetup(void)
    {
    const int *bus_range;
    int len;
    struct pci_controller *hose;
    struct device_node *root;
    struct device_node *pcictrl;
    root = of_find_node_by_path("/");
    if (root == core::ptr::null_mut()) {
    printk(KERN_WARNING EFIKA_PLATFORM_NAME
    ": Unable to find the root node\n");
    return;
    }
    for_each_child_of_node(root, pcictrl)
    if (of_node_name_eq(pcictrl, "pci"))
    break;
    of_node_put(root);
    if (pcictrl == core::ptr::null_mut()) {
    printk(KERN_WARNING EFIKA_PLATFORM_NAME
    ": Unable to find the PCI bridge node\n");
    return;
    }
    bus_range = of_get_property(pcictrl, "bus-range", &len);
    if (bus_range == core::ptr::null_mut() || len < 2 * sizeof(int)) {
    printk(KERN_WARNING EFIKA_PLATFORM_NAME
    ": Can't get bus-range for %pOF\n", pcictrl);
    goto out_put;
    }
    if (bus_range[1] == bus_range[0])
    printk(KERN_INFO EFIKA_PLATFORM_NAME ": PCI bus %d",
    bus_range[0]);
    else
    printk(KERN_INFO EFIKA_PLATFORM_NAME ": PCI buses %d..%d",
    bus_range[0], bus_range[1]);
    printk(" controlled by %pOF\n", pcictrl);
    printk("\n");
    hose = pcibios_alloc_controller(pcictrl);
    if (!hose) {
    printk(KERN_WARNING EFIKA_PLATFORM_NAME
    ": Can't allocate PCI controller structure for %pOF\n",
    pcictrl);
    goto out_put;
    }
    hose.first_busno = bus_range[0];
    hose.last_busno = bus_range[1];
    hose.ops = &rtas_pci_ops;
    pci_process_bridge_OF_ranges(hose, pcictrl, 0);
    return;
    out_put:
    of_node_put(pcictrl);
    }

#[no_mangle]
unsafe extern "C" fn efika_pcisetup() -> void __init {
    static void __init efika_pcisetup(void)
    {}

// ------------------------------------------------------------------------
// Platform setup
// ------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn efika_show_cpuinfo(m: *mut seq_file) {
    static void efika_show_cpuinfo(struct seq_file *m)
    {
    struct device_node *root;
    const char *revision;
    const char *codegendescription;
    const char *codegenvendor;
    root = of_find_node_by_path("/");
    if (!root)
    return;
    revision = of_get_property(root, "revision", core::ptr::null_mut());
    codegendescription = of_get_property(root, "CODEGEN,description", core::ptr::null_mut());
    codegenvendor = of_get_property(root, "CODEGEN,vendor", core::ptr::null_mut());
    if (codegendescription)
    seq_printf(m, "machine\t\t: %s\n", codegendescription);
    else
    seq_printf(m, "machine\t\t: Efika\n");
    if (revision)
    seq_printf(m, "revision\t: %s\n", revision);
    if (codegenvendor)
    seq_printf(m, "vendor\t\t: %s\n", codegenvendor);
    of_node_put(root);
    }

#[no_mangle]
unsafe extern "C" fn efika_suspend_prepare(mbar: *mut void __iomem) {
    static void efika_suspend_prepare(void __iomem *mbar)
    {
    u8 pin = 4;	/* GPIO_WKUP_4 (GPIO_PSC6_0 - IRDA_RX) */
    u8 level = 1;	/* wakeup on high level */
// IOW. to wake it up, short pins 1 and 3 on IRDA connector
    mpc52xx_set_wakeup_gpio(pin, level);
    }

#[no_mangle]
unsafe extern "C" fn efika_setup_arch() -> void __init {
    static void __init efika_setup_arch(void)
    {
    rtas_initialize();
// Map important registers from the internal memory map
    mpc52xx_map_common_devices();

    mpc52xx_suspend.board_suspend_prepare = efika_suspend_prepare;
    mpc52xx_pm_init();

    if (ppc_md.progress)
    ppc_md.progress("Linux/PPC " UTS_RELEASE " running on Efika ;-)\n", 0x0);
    }
#[no_mangle]
unsafe extern "C" fn efika_probe() -> int __init {
    static int __init efika_probe(void)
    {
    struct device_node *root = of_find_node_by_path("/");
    const char *model = of_get_property(root, "model", core::ptr::null_mut());
    of_node_put(root);
    if (model == core::ptr::null_mut())
    return 0;
    if (strcmp(model, "EFIKA5K2"))
    return 0;
    DMA_MODE_READ = 0x44;
    DMA_MODE_WRITE = 0x48;
    pm_power_off = rtas_power_off;
    return 1;
    }
    define_machine(efika)
    {
    .name			= EFIKA_PLATFORM_NAME,
    .probe			= efika_probe,
    .setup_arch		= efika_setup_arch,
    .discover_phbs		= efika_pcisetup,
    .init			= mpc52xx_declare_of_platform_devices,
    .show_cpuinfo		= efika_show_cpuinfo,
    .init_IRQ		= mpc52xx_init_irq,
    .get_irq		= mpc52xx_get_irq,
    .restart		= rtas_restart,
    .halt			= rtas_halt,
    .set_rtc_time		= rtas_set_rtc_time,
    .get_rtc_time		= rtas_get_rtc_time,
    .progress		= rtas_progress,
    .get_boot_time		= rtas_get_boot_time,

    .phys_mem_access_prot	= pci_phys_mem_access_prot,

    };
