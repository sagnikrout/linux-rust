//! Automatically rewritten from C to Rust
//! Source: drivers/char/agp/efficeon-agp.c
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
// Transmeta's Efficeon AGPGART driver.
//
// Based upon a diff by Linus around November '02.
//
// Ported to the 2.6 kernel by Carlos Puchol <cpglinux@puchol.com>
// and H. Peter Anvin <hpa@transmeta.com>.
//
// NOTE-cpg-040217:
//
// - when compiled as a module, after loading the module,
// it will refuse to unload, indicating it is in use,
// when it is not.
// - no s3 (suspend to ram) testing.
// - tested on the efficeon integrated nothbridge for tens
// of iterations of starting x and glxgears.
// - tested with radeon 9000 and radeon mobility m9 cards
// - tested with c3/c4 enabled (with the mobility m9 card)
//

//
// The real differences to the generic AGP code is
// in the GART mappings - a two-level setup with the
// first level being an on-chip 64-entry table.
//
// The page array is filled through the ATTPAGE register
// (Aperture Translation Table Page Register) at 0xB8. Bits:
// 31:20: physical page address
// 11:9: Page Attribute Table Index (PATI)
// must match the PAT index for the
// mapped pages (the 2nd level page table pages
// themselves should be just regular WB-cacheable,
// so this is normally zero.)
// 8: Present
// 7:6: reserved, write as zero
// 5:0: GATT directory index: which 1st-level entry
//
// The Efficeon AGP spec requires pages to be WB-cacheable
// but to be explicitly CLFLUSH'd after any changes.
//
pub const EFFICEON_ATTPAGE: c_uint = 0xb8;

    static struct _efficeon_private {
    unsigned long l1_table[EFFICEON_L1_SIZE];
    } efficeon_private;
    static const struct gatt_mask efficeon_generic_masks[] =
    {
    {.mask = 0x00000001, .type = 0}
    };
// This function does the same thing as mask_memory() for this chipset...
#[no_mangle]
pub unsafe extern "C" fn efficeon_mask_memory(page: *mut page) -> c_ulong {
    static inline unsigned long efficeon_mask_memory(struct page *page)
    {
    let mut addr: c_ulong = page_to_phys(page);
    return addr | 0x00000001;
    }
    static const struct aper_size_info_lvl2 efficeon_generic_sizes[4] =
    {
    {256, 65536, 0},
    {128, 32768, 32},
    {64, 16384, 48},
    {32, 8192, 56}
    };
//
// Control interfaces are largely identical to
// the legacy Intel 440BX..
//
#[no_mangle]
unsafe extern "C" fn efficeon_fetch_size() -> c_int {
    static int efficeon_fetch_size(void)
    {
    int i;
    u16 temp;
    struct aper_size_info_lvl2 *values;
    pci_read_config_word(agp_bridge.dev, INTEL_APSIZE, &temp);
    values = A_SIZE_LVL2(agp_bridge.driver.aperture_sizes);
    for (i = 0; i < agp_bridge.driver.num_aperture_sizes; i++) {
    if (temp == values[i].size_value) {
    agp_bridge.previous_size =
    agp_bridge.current_size = (void *) (values + i);
    agp_bridge.aperture_size_idx = i;
    return values[i].size;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn efficeon_tlbflush(mem: *mut *mut agp_memory) {
    static void efficeon_tlbflush(struct agp_memory * mem)
    {
    printk(KERN_DEBUG PFX "efficeon_tlbflush()\n");
    pci_write_config_dword(agp_bridge.dev, INTEL_AGPCTRL, 0x2200);
    pci_write_config_dword(agp_bridge.dev, INTEL_AGPCTRL, 0x2280);
    }
#[no_mangle]
unsafe extern "C" fn efficeon_cleanup() {
    static void efficeon_cleanup(void)
    {
    u16 temp;
    struct aper_size_info_lvl2 *previous_size;
    printk(KERN_DEBUG PFX "efficeon_cleanup()\n");
    previous_size = A_SIZE_LVL2(agp_bridge.previous_size);
    pci_read_config_word(agp_bridge.dev, INTEL_NBXCFG, &temp);
    pci_write_config_word(agp_bridge.dev, INTEL_NBXCFG, temp & ~(1 << 9));
    pci_write_config_word(agp_bridge.dev, INTEL_APSIZE,
    previous_size.size_value);
    }
#[no_mangle]
unsafe extern "C" fn efficeon_configure() -> c_int {
    static int efficeon_configure(void)
    {
    u16 temp2;
    struct aper_size_info_lvl2 *current_size;
    printk(KERN_DEBUG PFX "efficeon_configure()\n");
    current_size = A_SIZE_LVL2(agp_bridge.current_size);
// aperture size
    pci_write_config_word(agp_bridge.dev, INTEL_APSIZE,
    current_size.size_value);
// address to map to
    agp_bridge.gart_bus_addr = pci_bus_address(agp_bridge.dev,
    AGP_APERTURE_BAR);
// agpctrl
    pci_write_config_dword(agp_bridge.dev, INTEL_AGPCTRL, 0x2280);
// paccfg/nbxcfg
    pci_read_config_word(agp_bridge.dev, INTEL_NBXCFG, &temp2);
    pci_write_config_word(agp_bridge.dev, INTEL_NBXCFG,
    (temp2 & ~(1 << 10)) | (1 << 9) | (1 << 11));
// clear any possible error conditions
    pci_write_config_byte(agp_bridge.dev, INTEL_ERRSTS + 1, 7);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn efficeon_free_gatt_table(bridge: *mut agp_bridge_data) -> c_int {
    static int efficeon_free_gatt_table(struct agp_bridge_data *bridge)
    {
    int index, freed = 0;
    for (index = 0; index < EFFICEON_L1_SIZE; index++) {
    let mut page: c_ulong = efficeon_private.l1_table[index];
    if (page) {
    efficeon_private.l1_table[index] = 0;
    free_page(page);
    freed++;
    }
    printk(KERN_DEBUG PFX "efficeon_free_gatt_table(%p, %02x, %08x)\n",
    agp_bridge.dev, EFFICEON_ATTPAGE, index);
    pci_write_config_dword(agp_bridge.dev,
    EFFICEON_ATTPAGE, index);
    }
    printk(KERN_DEBUG PFX "efficeon_free_gatt_table() freed %d pages\n", freed);
    return 0;
    }
//
// Since we don't need contiguous memory we just try
// to get the gatt table once
//

    GET_PAGE_DIR_OFF(agp_bridge.gart_bus_addr))

    GET_PAGE_DIR_IDX(addr)].remapped)
#[no_mangle]
unsafe extern "C" fn efficeon_create_gatt_table(bridge: *mut agp_bridge_data) -> c_int {
    static int efficeon_create_gatt_table(struct agp_bridge_data *bridge)
    {
    int index;
    let mut pati: c_int = EFFICEON_PATI;
    let mut present: c_int = EFFICEON_PRESENT;
    let mut clflush_chunk: c_int = ((cpuid_ebx(1) >> 8) & 0xff) << 3;
    int num_entries, l1_pages;
    num_entries = A_SIZE_LVL2(agp_bridge.current_size).num_entries;
    printk(KERN_DEBUG PFX "efficeon_create_gatt_table(%d)\n", num_entries);
// There are 2^10 PTE pages per PDE page
    BUG_ON(num_entries & 0x3ff);
    l1_pages = num_entries >> 10;
    for (index = 0 ; index < l1_pages ; index++) {
    int offset;
    unsigned long page;
    unsigned long value;
    page = efficeon_private.l1_table[index];
    BUG_ON(page);
    page = get_zeroed_page(GFP_KERNEL);
    if (!page) {
    efficeon_free_gatt_table(agp_bridge);
    return -ENOMEM;
    }
    for (offset = 0; offset < PAGE_SIZE; offset += clflush_chunk)
    clflush((char *)page+offset);
    efficeon_private.l1_table[index] = page;
    value = virt_to_phys((unsigned long *)page) | pati | present | index;
    pci_write_config_dword(agp_bridge.dev,
    EFFICEON_ATTPAGE, value);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn efficeon_insert_memory(mem: *mut *mut agp_memory, pg_start: off_t, type: c_int) -> c_int {
    static int efficeon_insert_memory(struct agp_memory * mem, off_t pg_start, int type)
    {
    int i, count = mem.page_count, num_entries;
    unsigned int *page, *last_page;
    let mut clflush_chunk: c_int = ((cpuid_ebx(1) >> 8) & 0xff) << 3;
    let mut clflush_mask: c_ulong = ~(clflush_chunk-1);
    printk(KERN_DEBUG PFX "efficeon_insert_memory(%lx, %d)\n", pg_start, count);
    num_entries = A_SIZE_LVL2(agp_bridge.current_size).num_entries;
    if ((pg_start + mem.page_count) > num_entries)
    return -EINVAL;
    if (type != 0 || mem.type != 0)
    return -EINVAL;
    if (!mem.is_flushed) {
    global_cache_flush();
    mem.is_flushed = true;
    }
    last_page = core::ptr::null_mut();
    for (i = 0; i < count; i++) {
    let mut index: c_int = pg_start + i;
    let mut insert: c_ulong = efficeon_mask_memory(mem.pages[i]);
    page = (unsigned int *) efficeon_private.l1_table[index >> 10];
    if (!page)
    continue;
    page += (index & 0x3ff);
// page = insert;
// clflush is slow, so don't clflush until we have to
    if (last_page &&
    (((unsigned long)page^(unsigned long)last_page) &
    clflush_mask))
    clflush(last_page);
    last_page = page;
    }
    if ( last_page )
    clflush(last_page);
    agp_bridge.driver.tlb_flush(mem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn efficeon_remove_memory(mem: *mut *mut agp_memory, pg_start: off_t, type: c_int) -> c_int {
    static int efficeon_remove_memory(struct agp_memory * mem, off_t pg_start, int type)
    {
    int i, count = mem.page_count, num_entries;
    printk(KERN_DEBUG PFX "efficeon_remove_memory(%lx, %d)\n", pg_start, count);
    num_entries = A_SIZE_LVL2(agp_bridge.current_size).num_entries;
    if ((pg_start + mem.page_count) > num_entries)
    return -EINVAL;
    if (type != 0 || mem.type != 0)
    return -EINVAL;
    for (i = 0; i < count; i++) {
    let mut index: c_int = pg_start + i;
    unsigned int *page = (unsigned int *) efficeon_private.l1_table[index >> 10];
    if (!page)
    continue;
    page += (index & 0x3ff);
// page = 0;
    }
    agp_bridge.driver.tlb_flush(mem);
    return 0;
    }
    static const struct agp_bridge_driver efficeon_driver = {
    .owner			= THIS_MODULE,
    .aperture_sizes		= efficeon_generic_sizes,
    .size_type		= LVL2_APER_SIZE,
    .num_aperture_sizes	= 4,
    .configure		= efficeon_configure,
    .fetch_size		= efficeon_fetch_size,
    .cleanup		= efficeon_cleanup,
    .tlb_flush		= efficeon_tlbflush,
    .mask_memory		= agp_generic_mask_memory,
    .masks			= efficeon_generic_masks,
    .agp_enable		= agp_generic_enable,
    .cache_flush		= global_cache_flush,
// Efficeon-specific GATT table setup / populate / teardown
    .create_gatt_table	= efficeon_create_gatt_table,
    .free_gatt_table	= efficeon_free_gatt_table,
    .insert_memory		= efficeon_insert_memory,
    .remove_memory		= efficeon_remove_memory,
    .cant_use_aperture	= false,	// true might be faster?
// Generic
    .alloc_by_type		= agp_generic_alloc_by_type,
    .free_by_type		= agp_generic_free_by_type,
    .agp_alloc_page		= agp_generic_alloc_page,
    .agp_alloc_pages	= agp_generic_alloc_pages,
    .agp_destroy_page	= agp_generic_destroy_page,
    .agp_destroy_pages	= agp_generic_destroy_pages,
    .agp_type_to_mask_type  = agp_generic_type_to_mask_type,
    };
    static int agp_efficeon_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct agp_bridge_data *bridge;
    u8 cap_ptr;
    struct resource *r;
    cap_ptr = pci_find_capability(pdev, PCI_CAP_ID_AGP);
    if (!cap_ptr)
    return -ENODEV;
// Probe for Efficeon controller
    if (pdev.device != PCI_DEVICE_ID_EFFICEON) {
    printk(KERN_ERR PFX "Unsupported Efficeon chipset (device id: %04x)\n",
    pdev.device);
    return -ENODEV;
    }
    printk(KERN_INFO PFX "Detected Transmeta Efficeon TM8000 series chipset\n");
    bridge = agp_alloc_bridge();
    if (!bridge)
    return -ENOMEM;
    bridge.driver = &efficeon_driver;
    bridge.dev = pdev;
    bridge.capndx = cap_ptr;
//
// If the device has not been properly setup, the following will catch
// the problem and should stop the system from crashing.
// 20030610 - hamish@zot.org
//
    if (pci_enable_device(pdev)) {
    printk(KERN_ERR PFX "Unable to Enable PCI device\n");
    agp_put_bridge(bridge);
    return -ENODEV;
    }
//
// The following fixes the case where the BIOS has "forgotten" to
// provide an address range for the GART.
// 20030610 - hamish@zot.org
//
    r = &pdev.resource[0];
    if (!r.start && r.end) {
    if (pci_assign_resource(pdev, 0)) {
    printk(KERN_ERR PFX "could not assign resource 0\n");
    agp_put_bridge(bridge);
    return -ENODEV;
    }
    }
// Fill in the mode register
    if (cap_ptr) {
    pci_read_config_dword(pdev,
    bridge.capndx+PCI_AGP_STATUS,
    &bridge.mode);
    }
    pci_set_drvdata(pdev, bridge);
    return agp_add_bridge(bridge);
    }
#[no_mangle]
unsafe extern "C" fn agp_efficeon_remove(pdev: *mut pci_dev) {
    static void agp_efficeon_remove(struct pci_dev *pdev)
    {
    struct agp_bridge_data *bridge = pci_get_drvdata(pdev);
    agp_remove_bridge(bridge);
    agp_put_bridge(bridge);
    }
#[no_mangle]
unsafe extern "C" fn agp_efficeon_resume(dev: *mut device) -> c_int {
    static int agp_efficeon_resume(struct device *dev)
    {
    printk(KERN_DEBUG PFX "agp_efficeon_resume()\n");
    return efficeon_configure();
    }
    static const struct pci_device_id agp_efficeon_pci_table[] = {
    {
    .class		= (PCI_CLASS_BRIDGE_HOST << 8),
    .class_mask	= ~0,
    .vendor		= PCI_VENDOR_ID_TRANSMETA,
    .device		= PCI_ANY_ID,
    .subvendor	= PCI_ANY_ID,
    .subdevice	= PCI_ANY_ID,
    },
    { }
    };
    static DEFINE_SIMPLE_DEV_PM_OPS(agp_efficeon_pm_ops, core::ptr::null_mut(), agp_efficeon_resume);
    MODULE_DEVICE_TABLE(pci, agp_efficeon_pci_table);
    static struct pci_driver agp_efficeon_pci_driver = {
    .name		= "agpgart-efficeon",
    .id_table	= agp_efficeon_pci_table,
    .probe		= agp_efficeon_probe,
    .remove		= agp_efficeon_remove,
    .driver.pm	= &agp_efficeon_pm_ops,
    };
#[no_mangle]
unsafe extern "C" fn agp_efficeon_init() -> int __init {
    static int __init agp_efficeon_init(void)
    {
    let mut agp_initialised: static int = 0;
    if (agp_off)
    return -EINVAL;
    if (agp_initialised == 1)
    return 0;
    agp_initialised=1;
    return pci_register_driver(&agp_efficeon_pci_driver);
    }
#[no_mangle]
unsafe extern "C" fn agp_efficeon_cleanup() -> void __exit {
    static void __exit agp_efficeon_cleanup(void)
    {
    pci_unregister_driver(&agp_efficeon_pci_driver);
    }
    module_init(agp_efficeon_init);
    module_exit(agp_efficeon_cleanup);
    MODULE_AUTHOR("Carlos Puchol <cpglinux@puchol.com>");
    MODULE_DESCRIPTION("Transmeta's Efficeon AGPGART driver");
    MODULE_LICENSE("GPL and additional rights");
