//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/mmconfig-shared.c
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
// Low-level direct PCI config space access via ECAM - common code between
// i386 and x86-64.
//
// This code does:
// - known chipset handling
// - ACPI decoding and validation
//
// Per-architecture code takes care of the mappings and accesses
// themselves.
//

// Indicate if the ECAM resources have been placed into the resource table
    static bool pci_mmcfg_running_state;
    static bool pci_mmcfg_arch_init_failed;
    static DEFINE_MUTEX(pci_mmcfg_lock);

    LIST_HEAD(pci_mmcfg_list);
#[no_mangle]
unsafe extern "C" fn pci_mmconfig_remove(cfg: *mut pci_mmcfg_region) -> void __init {
    static void __init pci_mmconfig_remove(struct pci_mmcfg_region *cfg)
    {
    if (cfg.res.parent)
    release_resource(&cfg.res);
    list_del(&cfg.list);
    kfree(cfg);
    }
#[no_mangle]
unsafe extern "C" fn free_all_mmcfg() -> void __init {
    static void __init free_all_mmcfg(void)
    {
    struct pci_mmcfg_region *cfg, *tmp;
    pci_mmcfg_arch_free();
    list_for_each_entry_safe(cfg, tmp, &pci_mmcfg_list, list)
    pci_mmconfig_remove(cfg);
    }
#[no_mangle]
unsafe extern "C" fn list_add_sorted(new: *mut pci_mmcfg_region) {
    static void list_add_sorted(struct pci_mmcfg_region *new)
    {
    struct pci_mmcfg_region *cfg;
// keep list sorted by segment and starting bus number
    list_for_each_entry_rcu(cfg, &pci_mmcfg_list, list, pci_mmcfg_lock_held()) {
    if (cfg.segment > new.segment ||
    (cfg.segment == new.segment &&
    cfg.start_bus >= new.start_bus)) {
    list_add_tail_rcu(&new.list, &cfg.list);
    return;
    }
    }
    list_add_tail_rcu(&new.list, &pci_mmcfg_list);
    }
    static struct pci_mmcfg_region *pci_mmconfig_alloc(int segment, int start,
    int end, u64 addr)
    {
    struct pci_mmcfg_region *new;
    struct resource *res;
    if (addr == 0)
    return core::ptr::null_mut();
    new = kzalloc_obj(*new);
    if (!new)
    return core::ptr::null_mut();
    new.address = addr;
    new.segment = segment;
    new.start_bus = start;
    new.end_bus = end;
    res = &new.res;
    res.start = addr + PCI_MMCFG_BUS_OFFSET(start);
    res.end = addr + PCI_MMCFG_BUS_OFFSET(end + 1) - 1;
    res.flags = IORESOURCE_MEM | IORESOURCE_BUSY;
    snprintf(new.name, PCI_MMCFG_RESOURCE_NAME_LEN,
    "PCI ECAM %04x [bus %02x-%02x]", segment, start, end);
    res.name = new.name;
    return new;
    }
    struct pci_mmcfg_region *__init pci_mmconfig_add(int segment, int start,
    int end, u64 addr)
    {
    struct pci_mmcfg_region *new;
    new = pci_mmconfig_alloc(segment, start, end, addr);
    if (!new)
    return core::ptr::null_mut();
    mutex_lock(&pci_mmcfg_lock);
    list_add_sorted(new);
    mutex_unlock(&pci_mmcfg_lock);
    pr_info("ECAM %pR (base %#lx) for domain %04x [bus %02x-%02x]\n",
    &new.res, (unsigned long)addr, segment, start, end);
    return new;
    }
    struct pci_mmcfg_region *pci_mmconfig_lookup(int segment, int bus)
    {
    struct pci_mmcfg_region *cfg;
    list_for_each_entry_rcu(cfg, &pci_mmcfg_list, list, pci_mmcfg_lock_held())
    if (cfg.segment == segment &&
    cfg.start_bus <= bus && bus <= cfg.end_bus)
    return cfg;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pci_mmcfg_e7520() -> *const char __init {
    static const char *__init pci_mmcfg_e7520(void)
    {
    u32 win;
    raw_pci_ops.read(0, 0, PCI_DEVFN(0, 0), 0xce, 2, &win);
    win = win & 0xf000;
    if (win == 0x0000 || win == 0xf000)
    return core::ptr::null_mut();
    if (pci_mmconfig_add(0, 0, 255, win << 16) == core::ptr::null_mut())
    return core::ptr::null_mut();
    return "Intel Corporation E7520 Memory Controller Hub";
    }
#[no_mangle]
unsafe extern "C" fn pci_mmcfg_intel_945() -> *const char __init {
    static const char *__init pci_mmcfg_intel_945(void)
    {
    u32 pciexbar, mask = 0, len = 0;
    raw_pci_ops.read(0, 0, PCI_DEVFN(0, 0), 0x48, 4, &pciexbar);
// Enable bit
    if (!(pciexbar & 1))
    return core::ptr::null_mut();
// Size bits
    switch ((pciexbar >> 1) & 3) {
    case 0:
    mask = 0xf0000000U;
    len  = 0x10000000U;
    break;
    case 1:
    mask = 0xf8000000U;
    len  = 0x08000000U;
    break;
    case 2:
    mask = 0xfc000000U;
    len  = 0x04000000U;
    break;
    default:
    return core::ptr::null_mut();
    }
// Errata #2, things break when not aligned on a 256Mb boundary
// Can only happen in 64M/128M mode
    if ((pciexbar & mask) & 0x0fffffffU)
    return core::ptr::null_mut();
// Don't hit the APIC registers and their friends
    if ((pciexbar & mask) >= 0xf0000000U)
    return core::ptr::null_mut();
    if (pci_mmconfig_add(0, 0, (len >> 20) - 1, pciexbar & mask) == core::ptr::null_mut())
    return core::ptr::null_mut();
    return "Intel Corporation 945G/GZ/P/PL Express Memory Controller Hub";
    }
#[no_mangle]
unsafe extern "C" fn pci_mmcfg_amd_fam10h() -> *const char __init {
    static const char *__init pci_mmcfg_amd_fam10h(void)
    {
    u32 address;
    u64 base, msr;
    int i;
    let mut segnbits: unsigned = 0, busnbits, end_bus;
    if (!(pci_probe & PCI_CHECK_ENABLE_AMD_MMCONF))
    return core::ptr::null_mut();
    address = MSR_FAM10H_MMIO_CONF_BASE;
    if (rdmsrq_safe(address, &msr))
    return core::ptr::null_mut();
// ECAM is not enabled
    if (!(msr & FAM10H_MMIO_CONF_ENABLE))
    return core::ptr::null_mut();
    base = msr & (FAM10H_MMIO_CONF_BASE_MASK<<FAM10H_MMIO_CONF_BASE_SHIFT);
    busnbits = (msr >> FAM10H_MMIO_CONF_BUSRANGE_SHIFT) &
    FAM10H_MMIO_CONF_BUSRANGE_MASK;
//
// only handle bus 0 ?
// need to skip it
//
    if (!busnbits)
    return core::ptr::null_mut();
    if (busnbits > 8) {
    segnbits = busnbits - 8;
    busnbits = 8;
    }
    end_bus = (1 << busnbits) - 1;
    for (i = 0; i < (1 << segnbits); i++)
    if (pci_mmconfig_add(i, 0, end_bus,
    base + (1<<28) * i) == core::ptr::null_mut()) {
    free_all_mmcfg();
    return core::ptr::null_mut();
    }
    return "AMD Family 10h NB";
    }
    static bool __initdata mcp55_checked;
#[no_mangle]
unsafe extern "C" fn pci_mmcfg_nvidia_mcp55() -> *const char __init {
    static const char *__init pci_mmcfg_nvidia_mcp55(void)
    {
    int bus;
    let mut mcp55_mmconf_found: c_int = 0;
    let mut __initconst: static u32 extcfg_regnum = 0x90;
    let mut __initconst: static u32 extcfg_regsize = 4;
    let mut __initconst: static u32 extcfg_enable_mask = 1 << 31;
    let mut __initconst: static u32 extcfg_start_mask = 0xff << 16;
    let mut __initconst: static int extcfg_start_shift = 16;
    let mut __initconst: static u32 extcfg_size_mask = 0x3 << 28;
    let mut __initconst: static int extcfg_size_shift = 28;
    static const int extcfg_sizebus[] __initconst	= {
    0x100, 0x80, 0x40, 0x20
    };
    static const u32 extcfg_base_mask[] __initconst	= {
    0x7ff8, 0x7ffc, 0x7ffe, 0x7fff
    };
    let mut __initconst: static int extcfg_base_lshift = 25;
//
// do check if amd fam10h already took over
//
    if (!acpi_disabled || !list_empty(&pci_mmcfg_list) || mcp55_checked)
    return core::ptr::null_mut();
    mcp55_checked = true;
    for (bus = 0; bus < 256; bus++) {
    u64 base;
    u32 l, extcfg;
    u16 vendor, device;
    int start, size_index, end;
    raw_pci_ops.read(0, bus, PCI_DEVFN(0, 0), 0, 4, &l);
    vendor = l & 0xffff;
    device = (l >> 16) & 0xffff;
    if (PCI_VENDOR_ID_NVIDIA != vendor || 0x0369 != device)
    continue;
    raw_pci_ops.read(0, bus, PCI_DEVFN(0, 0), extcfg_regnum,
    extcfg_regsize, &extcfg);
    if (!(extcfg & extcfg_enable_mask))
    continue;
    size_index = (extcfg & extcfg_size_mask) >> extcfg_size_shift;
    base = extcfg & extcfg_base_mask[size_index];
// base could > 4G
    base <<= extcfg_base_lshift;
    start = (extcfg & extcfg_start_mask) >> extcfg_start_shift;
    end = start + extcfg_sizebus[size_index] - 1;
    if (pci_mmconfig_add(0, start, end, base) == core::ptr::null_mut())
    continue;
    mcp55_mmconf_found++;
    }
    if (!mcp55_mmconf_found)
    return core::ptr::null_mut();
    return "nVidia MCP55";
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_mmcfg_hostbridge_probe {
    pub bus: u32,
    pub devfn: u32,
    pub vendor: u32,
    pub device: u32,
    pub (*probe)(void): *const c_char,
}

    static const struct pci_mmcfg_hostbridge_probe pci_mmcfg_probes[] __initconst = {
    { 0, PCI_DEVFN(0, 0), PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_E7520_MCH, pci_mmcfg_e7520 },
    { 0, PCI_DEVFN(0, 0), PCI_VENDOR_ID_INTEL,
    PCI_DEVICE_ID_INTEL_82945G_HB, pci_mmcfg_intel_945 },
    { 0, PCI_DEVFN(0x18, 0), PCI_VENDOR_ID_AMD,
    0x1200, pci_mmcfg_amd_fam10h },
    { 0xff, PCI_DEVFN(0, 0), PCI_VENDOR_ID_AMD,
    0x1200, pci_mmcfg_amd_fam10h },
    { 0, PCI_DEVFN(0, 0), PCI_VENDOR_ID_NVIDIA,
    0x0369, pci_mmcfg_nvidia_mcp55 },
    };
#[no_mangle]
unsafe extern "C" fn pci_mmcfg_check_end_bus_number() -> void __init {
    static void __init pci_mmcfg_check_end_bus_number(void)
    {
    struct pci_mmcfg_region *cfg, *cfgx;
// Fixup overlaps
    list_for_each_entry(cfg, &pci_mmcfg_list, list) {
    if (cfg.end_bus < cfg.start_bus)
    cfg.end_bus = 255;
// Don't access the list head !
    if (cfg.list.next == &pci_mmcfg_list)
    break;
    cfgx = list_entry(cfg.list.next, typeof(*cfg), list);
    if (cfg.end_bus >= cfgx.start_bus)
    cfg.end_bus = cfgx.start_bus - 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn pci_mmcfg_check_hostbridge() -> int __init {
    static int __init pci_mmcfg_check_hostbridge(void)
    {
    u32 l;
    u32 bus, devfn;
    u16 vendor, device;
    int i;
    const char *name;
    if (!raw_pci_ops)
    return 0;
    free_all_mmcfg();
    for (i = 0; i < ARRAY_SIZE(pci_mmcfg_probes); i++) {
    bus =  pci_mmcfg_probes[i].bus;
    devfn = pci_mmcfg_probes[i].devfn;
    raw_pci_ops.read(0, bus, devfn, 0, 4, &l);
    vendor = l & 0xffff;
    device = (l >> 16) & 0xffff;
    name = core::ptr::null_mut();
    if (pci_mmcfg_probes[i].vendor == vendor &&
    pci_mmcfg_probes[i].device == device)
    name = pci_mmcfg_probes[i].probe();
    if (name)
    pr_info("%s with ECAM support\n", name);
    }
// some end_bus_number is crazy, fix it
    pci_mmcfg_check_end_bus_number();
    return !list_empty(&pci_mmcfg_list);
    }
#[no_mangle]
unsafe extern "C" fn check_mcfg_resource(res: *mut acpi_resource, data: *mut c_void) -> acpi_status {
    static acpi_status check_mcfg_resource(struct acpi_resource *res, void *data)
    {
    struct resource *mcfg_res = data;
    struct acpi_resource_address64 address;
    acpi_status status;
    if (res.type == ACPI_RESOURCE_TYPE_FIXED_MEMORY32) {
    struct acpi_resource_fixed_memory32 *fixmem32 =
    &res.data.fixed_memory32;
    if (!fixmem32)
    return AE_OK;
    if ((mcfg_res.start >= fixmem32.address) &&
    (mcfg_res.end < (fixmem32.address +
    fixmem32.address_length))) {
    mcfg_res.flags = 1;
    return AE_CTRL_TERMINATE;
    }
    }
    if ((res.type != ACPI_RESOURCE_TYPE_ADDRESS32) &&
    (res.type != ACPI_RESOURCE_TYPE_ADDRESS64))
    return AE_OK;
    status = acpi_resource_to_address64(res, &address);
    if (ACPI_FAILURE(status) ||
    (address.address.address_length <= 0) ||
    (address.resource_type != ACPI_MEMORY_RANGE))
    return AE_OK;
    if ((mcfg_res.start >= address.address.minimum) &&
    (mcfg_res.end < (address.address.minimum + address.address.address_length))) {
    mcfg_res.flags = 1;
    return AE_CTRL_TERMINATE;
    }
    return AE_OK;
    }
    static acpi_status find_mboard_resource(acpi_handle handle, u32 lvl,
    void *context, void **rv)
    {
    struct resource *mcfg_res = context;
    acpi_walk_resources(handle, METHOD_NAME__CRS,
    check_mcfg_resource, context);
    if (mcfg_res.flags)
    return AE_CTRL_TERMINATE;
    return AE_OK;
    }
#[no_mangle]
unsafe extern "C" fn is_acpi_reserved(start: u64, end: u64, not_used: enum e820_type) -> bool {
    static bool is_acpi_reserved(u64 start, u64 end, enum e820_type not_used)
    {
    struct resource mcfg_res;
    mcfg_res.start = start;
    mcfg_res.end = end - 1;
    mcfg_res.flags = 0;
    acpi_get_devices("PNP0C01", find_mboard_resource, &mcfg_res, core::ptr::null_mut());
    if (!mcfg_res.flags)
    acpi_get_devices("PNP0C02", find_mboard_resource, &mcfg_res,
    core::ptr::null_mut());
    return mcfg_res.flags;
    }
#[no_mangle]
unsafe extern "C" fn is_efi_mmio(res: *mut resource) -> bool {
    static bool is_efi_mmio(struct resource *res)
    {

    let mut start: u64 = res.start;
    let mut end: u64 = res.start + resource_size(res);
    efi_memory_desc_t *md;
    u64 size, mmio_start, mmio_end;
    for_each_efi_memory_desc(md) {
    if (md.type == EFI_MEMORY_MAPPED_IO) {
    size = md.num_pages << EFI_PAGE_SHIFT;
    mmio_start = md.phys_addr;
    mmio_end = mmio_start + size;
    if (mmio_start <= start && end <= mmio_end)
    return true;
    }
    }

    return false;
    }
    typedef bool (*check_reserved_t)(u64 start, u64 end, enum e820_type type);
    static bool __ref is_mmconf_reserved(check_reserved_t is_reserved,
    struct pci_mmcfg_region *cfg,
    struct device *dev, const char *method)
    {
    let mut addr: u64 = cfg.res.start;
    let mut size: u64 = resource_size(&cfg.res);
    let mut old_size: u64 = size;
    int num_buses;
    while (!is_reserved(addr, addr + size, E820_TYPE_RESERVED)) {
    size >>= 1;
    if (size < (16UL<<20))
    break;
    }
    if (size < (16UL<<20) && size != old_size)
    return false;
    if (dev)
    dev_info(dev, "ECAM %pR reserved as %s\n",
    &cfg.res, method);
    else
    pr_info("ECAM %pR reserved as %s\n", &cfg.res, method);
    if (old_size != size) {
// update end_bus
    cfg.end_bus = cfg.start_bus + ((size>>20) - 1);
    num_buses = cfg.end_bus - cfg.start_bus + 1;
    cfg.res.end = cfg.res.start +
    PCI_MMCFG_BUS_OFFSET(num_buses) - 1;
    snprintf(cfg.name, PCI_MMCFG_RESOURCE_NAME_LEN,
    "PCI ECAM %04x [bus %02x-%02x]",
    cfg.segment, cfg.start_bus, cfg.end_bus);
    if (dev)
    dev_info(dev, "ECAM %pR (base %#lx) (size reduced!)\n",
    &cfg.res, (unsigned long) cfg.address);
    else
    pr_info("ECAM %pR (base %#lx) for %04x [bus%02x-%02x] (size reduced!)\n",
    &cfg.res, (unsigned long) cfg.address,
    cfg.segment, cfg.start_bus, cfg.end_bus);
    }
    return true;
    }
    static bool __ref pci_mmcfg_reserved(struct device *dev,
    struct pci_mmcfg_region *cfg, int early)
    {
    struct resource *conflict;
    if (early) {
//
// Don't try to do this check unless configuration type 1
// is available.  How about type 2?
//
// 946f2ee5c731 ("Check that MCFG points to an e820
// reserved area") added this E820 check in 2006 to work
// around BIOS defects.
//
// Per PCI Firmware r3.3, sec 4.1.2, ECAM space must be
// reserved by a PNP0C02 resource, but it need not be
// mentioned in E820.  Before the ACPI interpreter is
// available, we can't check for PNP0C02 resources, so
// there's no reliable way to verify the region in this
// early check.  Keep it only for the old machines that
// motivated 946f2ee5c731.
//
    if (dmi_get_bios_year() < 2016 && raw_pci_ops)
    return is_mmconf_reserved(e820__mapped_all, cfg, dev,
    "E820 entry");
    return true;
    }
    if (!acpi_disabled) {
    if (is_mmconf_reserved(is_acpi_reserved, cfg, dev,
    "ACPI motherboard resource"))
    return true;
    if (dev)
    dev_info(dev, FW_INFO "ECAM %pR not reserved in ACPI motherboard resources\n",
    &cfg.res);
    else
    pr_info(FW_INFO "ECAM %pR not reserved in ACPI motherboard resources\n",
    &cfg.res);
    if (is_efi_mmio(&cfg.res)) {
    pr_info("ECAM %pR is EfiMemoryMappedIO; assuming valid\n",
    &cfg.res);
    conflict = insert_resource_conflict(&iomem_resource,
    &cfg.res);
    if (conflict)
    pr_warn("ECAM %pR conflicts with %s %pR\n",
    &cfg.res, conflict.name, conflict);
    else
    pr_info("ECAM %pR reserved to work around lack of ACPI motherboard _CRS\n",
    &cfg.res);
    return true;
    }
    }
//
// e820__mapped_all() is marked as __init.
// All entries from ACPI MCFG table have been checked at boot time.
// For MCFG information constructed from hotpluggable host bridge's
// _CBA method, just assume it's reserved.
//
    return pci_mmcfg_running_state;
    }
#[no_mangle]
unsafe extern "C" fn pci_mmcfg_reject_broken(early: c_int) -> void __init {
    static void __init pci_mmcfg_reject_broken(int early)
    {
    struct pci_mmcfg_region *cfg;
    list_for_each_entry(cfg, &pci_mmcfg_list, list) {
    if (!pci_mmcfg_reserved(core::ptr::null_mut(), cfg, early)) {
    pr_info("not using ECAM (%pR not reserved)\n",
    &cfg.res);
    free_all_mmcfg();
    return;
    }
    }
    }
    static bool __init acpi_mcfg_valid_entry(struct acpi_table_mcfg *mcfg,
    struct acpi_mcfg_allocation *cfg)
    {
    if (cfg.address < 0xFFFFFFFF)
    return true;
    if (!strncmp(mcfg.header.oem_id, "SGI", 3))
    return true;
    if ((mcfg.header.revision >= 1) && (dmi_get_bios_year() >= 2010))
    return true;
    pr_err("ECAM at %#llx for %04x [bus %02x-%02x] is above 4GB, ignored\n",
    cfg.address, cfg.pci_segment, cfg.start_bus_number,
    cfg.end_bus_number);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn pci_parse_mcfg(header: *mut acpi_table_header) -> int __init {
    static int __init pci_parse_mcfg(struct acpi_table_header *header)
    {
    struct acpi_table_mcfg *mcfg;
    struct acpi_mcfg_allocation *cfg_table, *cfg;
    unsigned long i;
    int entries;
    if (!header)
    return -EINVAL;
    mcfg = (struct acpi_table_mcfg *)header;
// how many config structures do we have
    free_all_mmcfg();
    entries = 0;
    i = header.length - sizeof(struct acpi_table_mcfg);
    while (i >= sizeof(struct acpi_mcfg_allocation)) {
    entries++;
    i -= sizeof(struct acpi_mcfg_allocation);
    }
    if (entries == 0) {
    pr_err("MCFG has no entries\n");
    return -ENODEV;
    }
    cfg_table = (struct acpi_mcfg_allocation *) &mcfg[1];
    for (i = 0; i < entries; i++) {
    cfg = &cfg_table[i];
    if (!acpi_mcfg_valid_entry(mcfg, cfg)) {
    free_all_mmcfg();
    return -ENODEV;
    }
    if (pci_mmconfig_add(cfg.pci_segment, cfg.start_bus_number,
    cfg.end_bus_number, cfg.address) == core::ptr::null_mut()) {
    pr_warn("no memory for MCFG entries\n");
    free_all_mmcfg();
    return -ENOMEM;
    }
    }
    return 0;
    }

    extern int (*arch_apei_filter_addr)(int (*func)(__u64 start, __u64 size,
    void *data), void *data);
    static int pci_mmcfg_for_each_region(int (*func)(__u64 start, __u64 size,
    void *data), void *data)
    {
    struct pci_mmcfg_region *cfg;
    int rc;
    if (list_empty(&pci_mmcfg_list))
    return 0;
    list_for_each_entry(cfg, &pci_mmcfg_list, list) {
    rc = func(cfg.res.start, resource_size(&cfg.res), data);
    if (rc)
    return rc;
    }
    return 0;
    }

// Macro flag: #define set_apei_filter()

#[no_mangle]
unsafe extern "C" fn __pci_mmcfg_init(early: c_int) -> void __init {
    static void __init __pci_mmcfg_init(int early)
    {
    pr_debug("%s(%s)\n", __func__, early ? "early" : "late");
    pci_mmcfg_reject_broken(early);
    if (list_empty(&pci_mmcfg_list))
    return;
    if (pcibios_last_bus < 0) {
    const struct pci_mmcfg_region *cfg;
    list_for_each_entry(cfg, &pci_mmcfg_list, list) {
    if (cfg.segment)
    break;
    pcibios_last_bus = cfg.end_bus;
    }
    }
    if (pci_mmcfg_arch_init())
    pci_probe = (pci_probe & ~PCI_PROBE_MASK) | PCI_PROBE_MMCONF;
    else {
    free_all_mmcfg();
    pci_mmcfg_arch_init_failed = true;
    }
    }
    static int __initdata known_bridge;
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_early_init() -> void __init {
    void __init pci_mmcfg_early_init(void)
    {
    pr_debug("%s() pci_probe %#x\n", __func__, pci_probe);
    if (pci_probe & PCI_PROBE_MMCONF) {
    if (pci_mmcfg_check_hostbridge())
    known_bridge = 1;
    else
    acpi_table_parse(ACPI_SIG_MCFG, pci_parse_mcfg);
    __pci_mmcfg_init(1);
    set_apei_filter();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn pci_mmcfg_late_init() -> void __init {
    void __init pci_mmcfg_late_init(void)
    {
    pr_debug("%s() pci_probe %#x\n", __func__, pci_probe);
// ECAM disabled
    if ((pci_probe & PCI_PROBE_MMCONF) == 0)
    return;
    if (known_bridge)
    return;
// ECAM hasn't been enabled yet, try again
    if (pci_probe & PCI_PROBE_MASK & ~PCI_PROBE_MMCONF) {
    acpi_table_parse(ACPI_SIG_MCFG, pci_parse_mcfg);
    __pci_mmcfg_init(0);
    }
    }
#[no_mangle]
unsafe extern "C" fn pci_mmcfg_late_insert_resources() -> int __init {
    static int __init pci_mmcfg_late_insert_resources(void)
    {
    struct pci_mmcfg_region *cfg;
    pci_mmcfg_running_state = true;
    pr_debug("%s() pci_probe %#x\n", __func__, pci_probe);
// If we are not using ECAM, don't insert the resources.
    if ((pci_probe & PCI_PROBE_MMCONF) == 0)
    return 1;
//
// Attempt to insert the mmcfg resources but not with the busy flag
// marked so it won't cause request errors when __request_region is
// called.
//
    list_for_each_entry(cfg, &pci_mmcfg_list, list) {
    if (!cfg.res.parent) {
    pr_debug("%s() insert %pR\n", __func__, &cfg.res);
    insert_resource(&iomem_resource, &cfg.res);
    }
    }
    return 0;
    }
//
// Perform ECAM resource insertion after PCI initialization to allow for
// misprogrammed MCFG tables that state larger sizes but actually conflict
// with other system resources.
//
    late_initcall(pci_mmcfg_late_insert_resources);
// Add ECAM information for host bridges
    int pci_mmconfig_insert(struct device *dev, u16 seg, u8 start, u8 end,
    phys_addr_t addr)
    {
    int rc;
    struct resource *tmp = core::ptr::null_mut();
    struct pci_mmcfg_region *cfg;
    dev_dbg(dev, "%s(%04x [bus %02x-%02x])\n", __func__, seg, start, end);
    if (!(pci_probe & PCI_PROBE_MMCONF) || pci_mmcfg_arch_init_failed)
    return -ENODEV;
    if (start > end)
    return -EINVAL;
    mutex_lock(&pci_mmcfg_lock);
    cfg = pci_mmconfig_lookup(seg, start);
    if (cfg) {
    if (cfg.end_bus < end)
    dev_info(dev, FW_INFO "ECAM %pR for domain %04x [bus %02x-%02x] only partially covers this bridge\n",
    &cfg.res, cfg.segment, cfg.start_bus,
    cfg.end_bus);
    mutex_unlock(&pci_mmcfg_lock);
    return -EEXIST;
    }
//
// Don't move earlier; we must return -EEXIST, not -EINVAL, if
// pci_mmconfig_lookup() finds something
//
    if (!addr) {
    mutex_unlock(&pci_mmcfg_lock);
    return -EINVAL;
    }
    rc = -EBUSY;
    cfg = pci_mmconfig_alloc(seg, start, end, addr);
    if (cfg == core::ptr::null_mut()) {
    dev_warn(dev, "fail to add ECAM (out of memory)\n");
    rc = -ENOMEM;
    } else if (!pci_mmcfg_reserved(dev, cfg, 0)) {
    dev_warn(dev, FW_BUG "ECAM %pR isn't reserved\n",
    &cfg.res);
    } else {
// Insert resource if it's not in boot stage
    if (pci_mmcfg_running_state)
    tmp = insert_resource_conflict(&iomem_resource,
    &cfg.res);
    if (tmp) {
    dev_warn(dev, "ECAM %pR conflicts with %s %pR\n",
    &cfg.res, tmp.name, tmp);
    } else if (pci_mmcfg_arch_map(cfg)) {
    dev_warn(dev, "fail to map ECAM %pR\n", &cfg.res);
    } else {
    list_add_sorted(cfg);
    dev_info(dev, "ECAM %pR (base %#lx)\n",
    &cfg.res, (unsigned long)addr);
    cfg = core::ptr::null_mut();
    rc = 0;
    }
    }
    if (cfg) {
    if (cfg.res.parent)
    release_resource(&cfg.res);
    kfree(cfg);
    }
    mutex_unlock(&pci_mmcfg_lock);
    return rc;
    }
// Delete ECAM information for host bridges
#[no_mangle]
pub unsafe extern "C" fn pci_mmconfig_delete(seg: u16, start: u8, end: u8) -> c_int {
    int pci_mmconfig_delete(u16 seg, u8 start, u8 end)
    {
    struct pci_mmcfg_region *cfg;
    mutex_lock(&pci_mmcfg_lock);
    list_for_each_entry_rcu(cfg, &pci_mmcfg_list, list)
    if (cfg.segment == seg && cfg.start_bus == start &&
    cfg.end_bus == end) {
    list_del_rcu(&cfg.list);
    synchronize_rcu();
    pci_mmcfg_arch_unmap(cfg);
    if (cfg.res.parent)
    release_resource(&cfg.res);
    mutex_unlock(&pci_mmcfg_lock);
    kfree(cfg);
    return 0;
    }
    mutex_unlock(&pci_mmcfg_lock);
    return -ENOENT;
    }
