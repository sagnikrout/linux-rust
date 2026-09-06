//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/mmconf-fam10h_64.c
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
// AMD Family 10h mmconfig enablement
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_hostbridge_probe {
    pub bus: u32,
    pub slot: u32,
    pub vendor: u32,
    pub device: u32,
}

    static u64 fam10h_pci_mmconf_base;
    static struct pci_hostbridge_probe pci_probes[] = {
    { 0, 0x18, PCI_VENDOR_ID_AMD, 0x1200 },
    { 0xff, 0, PCI_VENDOR_ID_AMD, 0x1200 },
    };
#[no_mangle]
unsafe extern "C" fn cmp_range(x1: *const c_void, x2: *const c_void) -> c_int {
    static int cmp_range(const void *x1, const void *x2)
    {
    const struct range *r1 = x1;
    const struct range *r2 = x2;
    int start1, start2;
    start1 = r1.start >> 32;
    start2 = r2.start >> 32;
    return start1 - start2;
    }

// need to avoid (0xfd<<32), (0xfe<<32), and (0xff<<32), ht used space

#[no_mangle]
unsafe extern "C" fn get_fam10h_pci_mmconf_base() {
    static void get_fam10h_pci_mmconf_base(void)
    {
    int i;
    unsigned bus;
    unsigned slot;
    int found;
    u64 val;
    u32 address;
    u64 tom2;
    let mut base: u64 = FAM10H_PCI_MMCONF_BASE;
    int hi_mmio_num;
    struct range range[8];
// only try to get setting from BSP
    if (fam10h_pci_mmconf_base)
    return;
    if (!early_pci_allowed())
    return;
    found = 0;
    for (i = 0; i < ARRAY_SIZE(pci_probes); i++) {
    u32 id;
    u16 device;
    u16 vendor;
    bus = pci_probes[i].bus;
    slot = pci_probes[i].slot;
    id = read_pci_config(bus, slot, 0, PCI_VENDOR_ID);
    vendor = id & 0xffff;
    device = (id>>16) & 0xffff;
    if (pci_probes[i].vendor == vendor &&
    pci_probes[i].device == device) {
    found = 1;
    break;
    }
    }
    if (!found)
    return;
// SYS_CFG
    address = MSR_AMD64_SYSCFG;
    rdmsrq(address, val);
// TOP_MEM2 is not enabled?
    if (!(val & (1<<21))) {
    tom2 = 1ULL << 32;
    } else {
// TOP_MEM2
    address = MSR_K8_TOP_MEM2;
    rdmsrq(address, val);
    tom2 = max(val & 0xffffff800000ULL, 1ULL << 32);
    }
    if (base <= tom2)
    base = (tom2 + 2 * MMCONF_UNIT - 1) & MMCONF_MASK;
//
// need to check if the range is in the high mmio range that is
// above 4G
//
    hi_mmio_num = 0;
    for (i = 0; i < 8; i++) {
    u32 reg;
    u64 start;
    u64 end;
    reg = read_pci_config(bus, slot, 1, 0x80 + (i << 3));
    if (!(reg & 3))
    continue;
    start = (u64)(reg & 0xffffff00) << 8; /* 39:16 on 31:8*/
    reg = read_pci_config(bus, slot, 1, 0x84 + (i << 3));
    end = ((u64)(reg & 0xffffff00) << 8) | 0xffff; /* 39:16 on 31:8*/
    if (end < tom2)
    continue;
    range[hi_mmio_num].start = start;
    range[hi_mmio_num].end = end;
    hi_mmio_num++;
    }
    if (!hi_mmio_num)
    goto out;
// sort the range
    sort(range, hi_mmio_num, sizeof(struct range), cmp_range, core::ptr::null_mut());
    if (range[hi_mmio_num - 1].end < base)
    goto out;
    if (range[0].start > base + MMCONF_SIZE)
    goto out;
// need to find one window
    base = (range[0].start & MMCONF_MASK) - MMCONF_UNIT;
    if ((base > tom2) && BASE_VALID(base))
    goto out;
    base = (range[hi_mmio_num - 1].end + MMCONF_UNIT) & MMCONF_MASK;
    if (BASE_VALID(base))
    goto out;
// need to find window between ranges
    for (i = 1; i < hi_mmio_num; i++) {
    base = (range[i - 1].end + MMCONF_UNIT) & MMCONF_MASK;
    val = range[i].start & MMCONF_MASK;
    if (val >= base + MMCONF_SIZE && BASE_VALID(base))
    goto out;
    }
    return;
    out:
    fam10h_pci_mmconf_base = base;
    }
#[no_mangle]
pub unsafe extern "C" fn fam10h_check_enable_mmcfg() {
    void fam10h_check_enable_mmcfg(void)
    {
    u64 val;
    u32 address;
    if (!(pci_probe & PCI_CHECK_ENABLE_AMD_MMCONF))
    return;
    address = MSR_FAM10H_MMIO_CONF_BASE;
    rdmsrq(address, val);
// try to make sure that AP's setting is identical to BSP setting
    if (val & FAM10H_MMIO_CONF_ENABLE) {
    unsigned busnbits;
    busnbits = (val >> FAM10H_MMIO_CONF_BUSRANGE_SHIFT) &
    FAM10H_MMIO_CONF_BUSRANGE_MASK;
// only trust the one handle 256 buses, if acpi=off
    if (!acpi_pci_disabled || busnbits >= 8) {
    let mut base: u64 = val & MMCONF_MASK;
    if (!fam10h_pci_mmconf_base) {
    fam10h_pci_mmconf_base = base;
    return;
    } else if (fam10h_pci_mmconf_base ==  base)
    return;
    }
    }
//
// if it is not enabled, try to enable it and assume only one segment
// with 256 buses
//
    get_fam10h_pci_mmconf_base();
    if (!fam10h_pci_mmconf_base) {
    pci_probe &= ~PCI_CHECK_ENABLE_AMD_MMCONF;
    return;
    }
    printk(KERN_INFO "Enable MMCONFIG on AMD Family 10h\n");
    val &= ~((FAM10H_MMIO_CONF_BASE_MASK<<FAM10H_MMIO_CONF_BASE_SHIFT) |
    (FAM10H_MMIO_CONF_BUSRANGE_MASK<<FAM10H_MMIO_CONF_BUSRANGE_SHIFT));
    val |= fam10h_pci_mmconf_base | (8 << FAM10H_MMIO_CONF_BUSRANGE_SHIFT) |
    FAM10H_MMIO_CONF_ENABLE;
    wrmsrq(address, val);
    }
#[no_mangle]
unsafe extern "C" fn set_check_enable_amd_mmconf(d: *const dmi_system_id) -> int __init {
    static int __init set_check_enable_amd_mmconf(const struct dmi_system_id *d)
    {
    pci_probe |= PCI_CHECK_ENABLE_AMD_MMCONF;
    return 0;
    }
    static const struct dmi_system_id __initconst mmconf_dmi_table[] = {
    {
    .callback = set_check_enable_amd_mmconf,
    .ident = "Sun Microsystems Machine",
    .matches = {
    DMI_MATCH(DMI_SYS_VENDOR, "Sun Microsystems"),
    },
    },
    {}
    };
// Called from a non __init function, but only on the BSP.
#[no_mangle]
pub unsafe extern "C" fn check_enable_amd_mmconf_dmi() -> void __ref {
    void __ref check_enable_amd_mmconf_dmi(void)
    {
    dmi_check_system(mmconf_dmi_table);
    }
