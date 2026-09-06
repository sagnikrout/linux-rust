//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/pci-dma.c
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

    static bool disable_dac_quirk __read_mostly;
    const struct dma_map_ops *dma_ops;
    EXPORT_SYMBOL(dma_ops);

    let mut __read_mostly: int panic_on_overflow = 1;
    let mut __read_mostly: int force_iommu = 1;

    let mut __read_mostly: int panic_on_overflow = 0;
    let mut __read_mostly: int force_iommu = 0;

    let mut __read_mostly: int iommu_merge = 0;
    int no_iommu __read_mostly;
// Set this to 1 if there is a HW IOMMU in the system
    let mut __read_mostly: int iommu_detected = 0;

    bool x86_swiotlb_enable;
    static unsigned int x86_swiotlb_flags;
#[no_mangle]
unsafe extern "C" fn pci_swiotlb_detect() -> void __init {
    static void __init pci_swiotlb_detect(void)
    {
// don't initialize swiotlb if iommu=off (no_iommu=1)
    if (!no_iommu && max_possible_pfn > MAX_DMA32_PFN)
    x86_swiotlb_enable = true;
//
// Set swiotlb to 1 so that bounce buffers are allocated and used for
// devices that can't support DMA to encrypted memory.
//
    if (cc_platform_has(CC_ATTR_HOST_MEM_ENCRYPT))
    x86_swiotlb_enable = true;
//
// Guest with guest memory encryption currently perform all DMA through
// bounce buffers as the hypervisor can't access arbitrary VM memory
// that is not explicitly shared with it.
//
    if (cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT))
    x86_swiotlb_enable = true;
    }

#[no_mangle]
pub unsafe extern "C" fn pci_swiotlb_detect() -> void __init {
    static inline void __init pci_swiotlb_detect(void)
    {
    }
pub const x86_swiotlb_flags: c_int = 0;

#[no_mangle]
unsafe extern "C" fn xen_swiotlb_enabled() -> bool {
    static bool xen_swiotlb_enabled(void)
    {
    return xen_initial_domain() || x86_swiotlb_enable ||
    (IS_ENABLED(CONFIG_XEN_PCIDEV_FRONTEND) && xen_pv_pci_possible);
    }
#[no_mangle]
unsafe extern "C" fn pci_xen_swiotlb_init() -> void __init {
    static void __init pci_xen_swiotlb_init(void)
    {
    if (!xen_swiotlb_enabled())
    return;
    x86_swiotlb_enable = true;
    x86_swiotlb_flags |= SWIOTLB_ANY;
    swiotlb_init_remap(true, x86_swiotlb_flags, xen_swiotlb_fixup);
    dma_ops = &xen_swiotlb_dma_ops;
    if (IS_ENABLED(CONFIG_PCI))
    pci_request_acs();
    }

#[no_mangle]
pub unsafe extern "C" fn pci_xen_swiotlb_init() -> void __init {
    static inline void __init pci_xen_swiotlb_init(void)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn pci_iommu_alloc() -> void __init {
    void __init pci_iommu_alloc(void)
    {
    if (xen_pv_domain()) {
    pci_xen_swiotlb_init();
    return;
    }
    pci_swiotlb_detect();
    gart_iommu_hole_init();
    amd_iommu_detect();
    detect_intel_iommu();
    swiotlb_init(x86_swiotlb_enable, x86_swiotlb_flags);
    }
#[no_mangle]
unsafe extern "C" fn iommu_setup(p: *mut c_char) -> __init int {
    static __init int iommu_setup(char *p)
    {
    iommu_merge = 1;
    if (!p)
    return -EINVAL;
    while (*p) {
    if (!strncmp(p, "off", 3))
    no_iommu = 1;
// gart_parse_options has more force support
    if (!strncmp(p, "force", 5))
    force_iommu = 1;
    if (!strncmp(p, "noforce", 7)) {
    iommu_merge = 0;
    force_iommu = 0;
    }
    if (!strncmp(p, "biomerge", 8)) {
    iommu_merge = 1;
    force_iommu = 1;
    }
    if (!strncmp(p, "panic", 5))
    panic_on_overflow = 1;
    if (!strncmp(p, "nopanic", 7))
    panic_on_overflow = 0;
    if (!strncmp(p, "merge", 5)) {
    iommu_merge = 1;
    force_iommu = 1;
    }
    if (!strncmp(p, "nomerge", 7))
    iommu_merge = 0;
    if (!strncmp(p, "forcesac", 8))
    pr_warn("forcesac option ignored.\n");
    if (!strncmp(p, "allowdac", 8))
    pr_warn("allowdac option ignored.\n");
    if (!strncmp(p, "nodac", 5))
    pr_warn("nodac option ignored.\n");
    if (!strncmp(p, "usedac", 6)) {
    disable_dac_quirk = true;
    return 1;
    }

    if (!strncmp(p, "soft", 4))
    x86_swiotlb_enable = true;

    if (!strncmp(p, "pt", 2))
    iommu_set_default_passthrough(true);
    if (!strncmp(p, "nopt", 4))
    iommu_set_default_translated(true);
    gart_parse_options(p);
    p += strcspn(p, ",");
    if (*p == ',')
    ++p;
    }
    return 0;
    }
    early_param("iommu", iommu_setup);
#[no_mangle]
unsafe extern "C" fn pci_iommu_init() -> int __init {
    static int __init pci_iommu_init(void)
    {
    x86_init.iommu.iommu_init();

// An IOMMU turned us off.
    if (x86_swiotlb_enable) {
    pr_info("PCI-DMA: Using software bounce buffering for IO (SWIOTLB)\n");
    swiotlb_print_info();
    } else {
    swiotlb_exit();
    }

    return 0;
    }
// Must execute after PCI subsystem
    rootfs_initcall(pci_iommu_init);

// Many VIA bridges seem to corrupt data for DAC. Disable it here
#[no_mangle]
unsafe extern "C" fn via_no_dac_cb(pdev: *mut pci_dev, data: *mut c_void) -> c_int {
    static int via_no_dac_cb(struct pci_dev *pdev, void *data)
    {
    pdev.dev.bus_dma_limit = DMA_BIT_MASK(32);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn via_no_dac(dev: *mut pci_dev) {
    static void via_no_dac(struct pci_dev *dev)
    {
    if (!disable_dac_quirk) {
    dev_info(&dev.dev, "disabling DAC on VIA PCI bridge\n");
    pci_walk_bus(dev.subordinate, via_no_dac_cb, core::ptr::null_mut());
    }
    }
    DECLARE_PCI_FIXUP_CLASS_FINAL(PCI_VENDOR_ID_VIA, PCI_ANY_ID,
    PCI_CLASS_BRIDGE_PCI, 8, via_no_dac);
