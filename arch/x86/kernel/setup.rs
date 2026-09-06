//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/setup.c
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
// Copyright (C) 1995  Linus Torvalds
//
// This file contains the setup_arch() code, which handles the architecture-dependent
// parts of early kernel initialization.
//

//
// max_low_pfn_mapped: highest directly mapped pfn < 4 GB
// max_pfn_mapped:     highest directly mapped pfn > 4 GB
//
// The direct mapping only covers E820_TYPE_RAM regions, so the ranges and gaps are
// represented by pfn_mapped[].
//
    unsigned long max_low_pfn_mapped;
    unsigned long max_pfn_mapped;

    RESERVE_BRK(dmi_alloc, 65536);

    let mut _brk_start: c_ulong = (unsigned long)__brk_base;
    let mut _brk_end: c_ulong = (unsigned long)__brk_base;
    struct boot_params boot_params;
//
// These are the four main kernel memory regions, we put them into
// the resource tree so that kdump tools and other debugging tools
// recover it:
//
    static struct resource rodata_resource = {
    .name	= "Kernel rodata",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM
    };
    static struct resource data_resource = {
    .name	= "Kernel data",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM
    };
    static struct resource code_resource = {
    .name	= "Kernel code",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM
    };
    static struct resource bss_resource = {
    .name	= "Kernel bss",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM
    };

// CPU data as detected by the assembly code in head_32.S
    struct cpuinfo_x86 new_cpu_data;
    struct apm_info apm_info;
    EXPORT_SYMBOL(apm_info);

    struct ist_info ist_info;
    EXPORT_SYMBOL(ist_info);

    struct ist_info ist_info;

    struct cpuinfo_x86 boot_cpu_data __read_mostly;
    EXPORT_SYMBOL(boot_cpu_data);
    SYM_PIC_ALIAS(boot_cpu_data);

    __visible unsigned long mmu_cr4_features __ro_after_init;

    let mut __ro_after_init: __visible unsigned long mmu_cr4_features = X86_CR4_PAE;

    static phys_addr_t ima_kexec_buffer_phys;
    static size_t ima_kexec_buffer_size;

// Boot loader ID and version as integers, for the benefit of proc_dointvec
    int bootloader_type, bootloader_version;
    static const struct ctl_table x86_sysctl_table[] = {
    {
    .procname       = "unknown_nmi_panic",
    .data           = &unknown_nmi_panic,
    .maxlen         = sizeof(int),
    .mode           = 0644,
    .proc_handler   = proc_dointvec,
    },
    {
    .procname	= "panic_on_unrecovered_nmi",
    .data		= &panic_on_unrecovered_nmi,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "panic_on_io_nmi",
    .data		= &panic_on_io_nmi,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "bootloader_type",
    .data		= &bootloader_type,
    .maxlen		= sizeof(int),
    .mode		= 0444,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "bootloader_version",
    .data		= &bootloader_version,
    .maxlen		= sizeof(int),
    .mode		= 0444,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "io_delay_type",
    .data		= &io_delay_type,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },

    {
    .procname	= "acpi_video_flags",
    .data		= &acpi_realmode_flags,
    .maxlen		= sizeof(unsigned long),
    .mode		= 0644,
    .proc_handler	= proc_doulongvec_minmax,
    },

    };
#[no_mangle]
unsafe extern "C" fn init_x86_sysctl() -> int __init {
    static int __init init_x86_sysctl(void)
    {
    register_sysctl_init("kernel", x86_sysctl_table);
    return 0;
    }
    arch_initcall(init_x86_sysctl);
//
// Setup options
//
    struct sysfb_display_info sysfb_primary_display;
    EXPORT_SYMBOL(sysfb_primary_display);
    extern int root_mountflags;
    unsigned long saved_video_mode;
pub const RAMDISK_IMAGE_START_MASK: c_uint = 0x07FF;
pub const RAMDISK_PROMPT_FLAG: c_uint = 0x8000;
pub const RAMDISK_LOAD_FLAG: c_uint = 0x4000;
    static char __initdata command_line[COMMAND_LINE_SIZE];

    char builtin_cmdline[COMMAND_LINE_SIZE] = CONFIG_CMDLINE;
    bool builtin_cmdline_added __ro_after_init;

    struct edd edd;

    EXPORT_SYMBOL(edd);

//
// copy_edd() - Copy the BIOS EDD information
// from boot_params into a safe place.
//
#[no_mangle]
pub unsafe extern "C" fn copy_edd() -> void __init {
    static inline void __init copy_edd(void)
    {
    memcpy(edd.mbr_signature, boot_params.edd_mbr_sig_buffer,
    sizeof(edd.mbr_signature));
    memcpy(edd.edd_info, boot_params.eddbuf, sizeof(edd.edd_info));
    edd.mbr_signature_nr = boot_params.edd_mbr_sig_buf_entries;
    edd.edd_info_nr = boot_params.eddbuf_entries;
    }

#[no_mangle]
pub unsafe extern "C" fn copy_edd() -> void __init {
    static inline void __init copy_edd(void)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn extend_brk(size: usize, align: usize) -> *mut void  __init {
    void * __init extend_brk(size_t size, size_t align)
    {
    let mut mask: usize = align - 1;
    void *ret;
    BUG_ON(_brk_start == 0);
    BUG_ON(align & mask);
    _brk_end = (_brk_end + mask) & ~mask;
    BUG_ON((char *)(_brk_end + size) > __brk_limit);
    ret = (void *)_brk_end;
    _brk_end += size;
    memset(ret, 0, size);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn cleanup_highmap() -> void __init {
    static void __init cleanup_highmap(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn reserve_brk() -> void __init {
    static void __init reserve_brk(void)
    {
    if (_brk_end > _brk_start)
    memblock_reserve_kern(__pa_symbol(_brk_start),
    _brk_end - _brk_start);
// Mark brk area as locked down and no longer taking any
    new allocations */
    _brk_start = 0;
    }

#[no_mangle]
unsafe extern "C" fn get_ramdisk_image() -> u64 __init {
    static u64 __init get_ramdisk_image(void)
    {
    let mut ramdisk_image: u64 = boot_params.hdr.ramdisk_image;
    ramdisk_image |= (u64)boot_params.ext_ramdisk_image << 32;
    if (ramdisk_image == 0)
    ramdisk_image = phys_initrd_start;
    return ramdisk_image;
    }
#[no_mangle]
unsafe extern "C" fn get_ramdisk_size() -> u64 __init {
    static u64 __init get_ramdisk_size(void)
    {
    let mut ramdisk_size: u64 = boot_params.hdr.ramdisk_size;
    ramdisk_size |= (u64)boot_params.ext_ramdisk_size << 32;
    if (ramdisk_size == 0)
    ramdisk_size = phys_initrd_size;
    return ramdisk_size;
    }
#[no_mangle]
unsafe extern "C" fn relocate_initrd() -> void __init {
    static void __init relocate_initrd(void)
    {
// Assume only end is not page aligned
    let mut ramdisk_image: u64 = get_ramdisk_image();
    let mut ramdisk_size: u64 = get_ramdisk_size();
    let mut area_size: u64 = PAGE_ALIGN(ramdisk_size);
    let mut ret: c_int = 0;
// We need to move the initrd down into directly mapped mem
    u64 relocated_ramdisk = memblock_phys_alloc_range(area_size, PAGE_SIZE, 0,
    PFN_PHYS(max_pfn_mapped));
    if (!relocated_ramdisk)
    panic("Cannot find place for new RAMDISK of size %lld\n",
    ramdisk_size);
    initrd_start = relocated_ramdisk + PAGE_OFFSET;
    initrd_end   = initrd_start + ramdisk_size;
    printk(KERN_INFO "Allocated new RAMDISK: [mem %#010llx-%#010llx]\n",
    relocated_ramdisk, relocated_ramdisk + ramdisk_size - 1);
    ret = copy_from_early_mem((void *)initrd_start, ramdisk_image, ramdisk_size);
    if (ret)
    panic("Copy RAMDISK failed\n");
    printk(KERN_INFO "Move RAMDISK from [mem %#010llx-%#010llx] to"
    " [mem %#010llx-%#010llx]\n",
    ramdisk_image, ramdisk_image + ramdisk_size - 1,
    relocated_ramdisk, relocated_ramdisk + ramdisk_size - 1);
    }
#[no_mangle]
unsafe extern "C" fn early_reserve_initrd() -> void __init {
    static void __init early_reserve_initrd(void)
    {
// Assume only end is not page aligned
    let mut ramdisk_image: u64 = get_ramdisk_image();
    let mut ramdisk_size: u64 = get_ramdisk_size();
    let mut ramdisk_end: u64 = PAGE_ALIGN(ramdisk_image + ramdisk_size);
    if (!boot_params.hdr.type_of_loader ||
    !ramdisk_image || !ramdisk_size)
    return;		/* No initrd provided by bootloader */
    memblock_reserve_kern(ramdisk_image, ramdisk_end - ramdisk_image);
    }
#[no_mangle]
unsafe extern "C" fn reserve_initrd() -> void __init {
    static void __init reserve_initrd(void)
    {
// Assume only end is not page aligned
    let mut ramdisk_image: u64 = get_ramdisk_image();
    let mut ramdisk_size: u64 = get_ramdisk_size();
    let mut ramdisk_end: u64 = PAGE_ALIGN(ramdisk_image + ramdisk_size);
    if (!boot_params.hdr.type_of_loader ||
    !ramdisk_image || !ramdisk_size)
    return;		/* No initrd provided by bootloader */
    initrd_start = 0;
    printk(KERN_INFO "RAMDISK: [mem %#010llx-%#010llx]\n", ramdisk_image,
    ramdisk_end - 1);
    if (pfn_range_is_mapped(PFN_DOWN(ramdisk_image),
    PFN_DOWN(ramdisk_end))) {
// All are mapped, easy case
    initrd_start = ramdisk_image + PAGE_OFFSET;
    initrd_end = initrd_start + ramdisk_size;
    return;
    }
    relocate_initrd();
    memblock_phys_free(ramdisk_image, ramdisk_end - ramdisk_image);
    }

#[no_mangle]
unsafe extern "C" fn early_reserve_initrd() -> void __init {
    static void __init early_reserve_initrd(void)
    {
    }
#[no_mangle]
unsafe extern "C" fn reserve_initrd() -> void __init {
    static void __init reserve_initrd(void)
    {
    }

#[no_mangle]
unsafe extern "C" fn add_early_ima_buffer(phys_addr: u64) -> void __init {
    static void __init add_early_ima_buffer(u64 phys_addr)
    {

    struct ima_setup_data *data;
    data = early_memremap(phys_addr + sizeof(struct setup_data), sizeof(*data));
    if (!data) {
    pr_warn("setup: failed to memremap ima_setup_data entry\n");
    return;
    }
    if (data.size) {
    memblock_reserve_kern(data.addr, data.size);
    ima_kexec_buffer_phys = data.addr;
    ima_kexec_buffer_size = data.size;
    }
    early_memunmap(data, sizeof(*data));

    pr_warn("Passed IMA kexec data, but CONFIG_IMA not set. Ignoring.\n");

    }

#[no_mangle]
pub unsafe extern "C" fn ima_free_kexec_buffer() -> int __init {
    int __init ima_free_kexec_buffer(void)
    {
    if (!ima_kexec_buffer_size)
    return -ENOENT;
    memblock_phys_free(ima_kexec_buffer_phys,
    ima_kexec_buffer_size);
    ima_kexec_buffer_phys = 0;
    ima_kexec_buffer_size = 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ima_get_kexec_buffer(addr: *mut c_void, size: *mut usize) -> int __init {
    int __init ima_get_kexec_buffer(void **addr, size_t *size)
    {
    int ret;
    if (!ima_kexec_buffer_size)
    return -ENOENT;
    ret = ima_validate_range(ima_kexec_buffer_phys, ima_kexec_buffer_size);
    if (ret)
    return ret;
// addr = __va(ima_kexec_buffer_phys);
// size = ima_kexec_buffer_size;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn add_kho(phys_addr: u64, data_len: u32) -> void __init {
    static void __init add_kho(u64 phys_addr, u32 data_len)
    {
    struct kho_data *kho;
    let mut addr: u64 = phys_addr + sizeof(struct setup_data);
    let mut size: u64 = data_len - sizeof(struct setup_data);
    if (!IS_ENABLED(CONFIG_KEXEC_HANDOVER)) {
    pr_warn("Passed KHO data, but CONFIG_KEXEC_HANDOVER not set. Ignoring.\n");
    return;
    }
    kho = early_memremap(addr, size);
    if (!kho) {
    pr_warn("setup: failed to memremap kho data (0x%llx, 0x%llx)\n",
    addr, size);
    return;
    }
    kho_populate(kho.fdt_addr, kho.fdt_size, kho.scratch_addr, kho.scratch_size);
    early_memunmap(kho, size);
    }
#[no_mangle]
unsafe extern "C" fn parse_setup_data() -> void __init {
    static void __init parse_setup_data(void)
    {
    struct setup_data *data;
    u64 pa_data, pa_next;
    pa_data = boot_params.hdr.setup_data;
    while (pa_data) {
    u32 data_len, data_type;
    data = early_memremap(pa_data, sizeof(*data));
    data_len = data.len + sizeof(struct setup_data);
    data_type = data.type;
    pa_next = data.next;
    early_memunmap(data, sizeof(*data));
    switch (data_type) {
    case SETUP_E820_EXT:
    e820__memory_setup_extended(pa_data, data_len);
    break;
    case SETUP_DTB:
    add_dtb(pa_data);
    break;
    case SETUP_EFI:
    parse_efi_setup(pa_data, data_len);
    break;
    case SETUP_IMA:
    add_early_ima_buffer(pa_data);
    break;
    case SETUP_KEXEC_KHO:
    add_kho(pa_data, data_len);
    break;
    case SETUP_RNG_SEED:
    data = early_memremap(pa_data, data_len);
    add_bootloader_randomness(data.data, data.len);
// Zero seed for forward secrecy.
    memzero_explicit(data.data, data.len);
// Zero length in case we find ourselves back here by accident.
    memzero_explicit(&data.len, sizeof(data.len));
    early_memunmap(data, data_len);
    break;
    default:
    break;
    }
    pa_data = pa_next;
    }
    }
//
// Translate the fields of 'struct boot_param' into global variables
// representing these parameters.
//
#[no_mangle]
unsafe extern "C" fn parse_boot_params() -> void __init {
    static void __init parse_boot_params(void)
    {
    ROOT_DEV = old_decode_dev(boot_params.hdr.root_dev);
    sysfb_primary_display.screen = boot_params.screen_info;

    sysfb_primary_display.edid = boot_params.edid_info;

    apm_info.bios = boot_params.apm_bios_info;
    ist_info = boot_params.ist_info;

    saved_video_mode = boot_params.hdr.vid_mode;
    bootloader_type = boot_params.hdr.type_of_loader;
    if ((bootloader_type >> 4) == 0xe) {
    bootloader_type &= 0xf;
    bootloader_type |= (boot_params.hdr.ext_loader_type+0x10) << 4;
    }
    bootloader_version  = bootloader_type & 0xf;
    bootloader_version |= boot_params.hdr.ext_loader_ver << 4;

    rd_image_start = boot_params.hdr.ram_size & RAMDISK_IMAGE_START_MASK;

    if (!strncmp((char *)&boot_params.efi_info.efi_loader_signature,
    EFI32_LOADER_SIGNATURE, 4)) {
    set_bit(EFI_BOOT, &efi.flags);
    } else if (!strncmp((char *)&boot_params.efi_info.efi_loader_signature,
    EFI64_LOADER_SIGNATURE, 4)) {
    set_bit(EFI_BOOT, &efi.flags);
    set_bit(EFI_64BIT, &efi.flags);
    }

    if (!boot_params.hdr.root_flags)
    root_mountflags &= ~MS_RDONLY;
    }
#[no_mangle]
unsafe extern "C" fn memblock_x86_reserve_range_setup_data() -> void __init {
    static void __init memblock_x86_reserve_range_setup_data(void)
    {
    struct setup_indirect *indirect;
    struct setup_data *data;
    u64 pa_data, pa_next;
    u32 len;
    pa_data = boot_params.hdr.setup_data;
    while (pa_data) {
    data = early_memremap(pa_data, sizeof(*data));
    if (!data) {
    pr_warn("setup: failed to memremap setup_data entry\n");
    return;
    }
    len = sizeof(*data);
    pa_next = data.next;
    memblock_reserve_kern(pa_data, sizeof(*data) + data.len);
    if (data.type == SETUP_INDIRECT) {
    len += data.len;
    early_memunmap(data, sizeof(*data));
    data = early_memremap(pa_data, len);
    if (!data) {
    pr_warn("setup: failed to memremap indirect setup_data\n");
    return;
    }
    indirect = (struct setup_indirect *)data.data;
    if (indirect.type != SETUP_INDIRECT)
    memblock_reserve_kern(indirect.addr, indirect.len);
    }
    pa_data = pa_next;
    early_memunmap(data, len);
    }
    }
#[no_mangle]
unsafe extern "C" fn arch_reserve_crashkernel() -> void __init {
    static void __init arch_reserve_crashkernel(void)
    {
    unsigned long long crash_base, crash_size, low_size = 0, cma_size = 0;
    let mut high: bool = false;
    int ret;
    if (!IS_ENABLED(CONFIG_CRASH_RESERVE))
    return;
    ret = parse_crashkernel(boot_command_line, memblock_phys_mem_size(),
    &crash_size, &crash_base,
    &low_size, &cma_size, &high);
    if (ret)
    return;
    if (xen_pv_domain()) {
    pr_info("Ignoring crashkernel for a Xen PV domain\n");
    return;
    }
    reserve_crashkernel_generic(crash_size, crash_base, low_size, high);
    reserve_crashkernel_cma(cma_size);
    }
    static struct resource standard_io_resources[] = {
    { .name = "dma1", .start = 0x00, .end = 0x1f,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "pic1", .start = 0x20, .end = 0x21,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "timer0", .start = 0x40, .end = 0x43,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "timer1", .start = 0x50, .end = 0x53,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "keyboard", .start = 0x60, .end = 0x60,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "keyboard", .start = 0x64, .end = 0x64,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "dma page reg", .start = 0x80, .end = 0x8f,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "pic2", .start = 0xa0, .end = 0xa1,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "dma2", .start = 0xc0, .end = 0xdf,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO },
    { .name = "fpu", .start = 0xf0, .end = 0xff,
    .flags = IORESOURCE_BUSY | IORESOURCE_IO }
    };
#[no_mangle]
pub unsafe extern "C" fn reserve_standard_io_resources() -> void __init {
    void __init reserve_standard_io_resources(void)
    {
    int i;
// request I/O space for devices used on all i[345]86 PCs
    for (i = 0; i < ARRAY_SIZE(standard_io_resources); i++)
    request_resource(&ioport_resource, &standard_io_resources[i]);
    }
#[no_mangle]
unsafe extern "C" fn setup_kernel_resources() -> void __init {
    static void __init setup_kernel_resources(void)
    {
    code_resource.start = __pa_symbol(_text);
    code_resource.end = __pa_symbol(_etext)-1;
    rodata_resource.start = __pa_symbol(__start_rodata);
    rodata_resource.end = __pa_symbol(__end_rodata)-1;
    data_resource.start = __pa_symbol(_sdata);
    data_resource.end = __pa_symbol(_edata)-1;
    bss_resource.start = __pa_symbol(__bss_start);
    bss_resource.end = __pa_symbol(__bss_stop)-1;
    insert_resource(&iomem_resource, &code_resource);
    insert_resource(&iomem_resource, &rodata_resource);
    insert_resource(&iomem_resource, &data_resource);
    insert_resource(&iomem_resource, &bss_resource);
    }
#[no_mangle]
unsafe extern "C" fn snb_gfx_workaround_needed() -> bool __init {
    static bool __init snb_gfx_workaround_needed(void)
    {

    int i;
    u16 vendor, devid;
    static const __initconst u16 snb_ids[] = {
    0x0102,
    0x0112,
    0x0122,
    0x0106,
    0x0116,
    0x0126,
    0x010a,
    };
// Assume no if something weird is going on with PCI
    if (!early_pci_allowed())
    return false;
    vendor = read_pci_config_16(0, 2, 0, PCI_VENDOR_ID);
    if (vendor != 0x8086)
    return false;
    devid = read_pci_config_16(0, 2, 0, PCI_DEVICE_ID);
    for (i = 0; i < ARRAY_SIZE(snb_ids); i++)
    if (devid == snb_ids[i])
    return true;

    return false;
    }
//
// Sandy Bridge graphics has trouble with certain ranges, exclude
// them from allocation.
//
#[no_mangle]
unsafe extern "C" fn trim_snb_memory() -> void __init {
    static void __init trim_snb_memory(void)
    {
    static const __initconst unsigned long bad_pages[] = {
    0x20050000,
    0x20110000,
    0x20130000,
    0x20138000,
    0x40004000,
    };
    int i;
    if (!snb_gfx_workaround_needed())
    return;
    printk(KERN_DEBUG "reserving inaccessible SNB gfx pages\n");
//
// SandyBridge integrated graphics devices have a bug that prevents
// them from accessing certain memory ranges, namely anything below
// 1M and in the pages listed in bad_pages[] above.
//
// To avoid these pages being ever accessed by SNB gfx devices reserve
// bad_pages that have not already been reserved at boot time.
// All memory below the 1 MB mark is anyway reserved later during
// setup_arch(), so there is no need to reserve it here.
//
    for (i = 0; i < ARRAY_SIZE(bad_pages); i++) {
    if (memblock_reserve(bad_pages[i], PAGE_SIZE))
    printk(KERN_WARNING "failed to reserve 0x%08lx\n",
    bad_pages[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn trim_bios_range() -> void __init {
    static void __init trim_bios_range(void)
    {
//
// A special case is the first 4Kb of memory;
// This is a BIOS owned area, not kernel ram, but generally
// not listed as such in the E820 table.
//
// This typically reserves additional memory (64KiB by default)
// since some BIOSes are known to corrupt low memory.  See the
// Kconfig help text for X86_RESERVE_LOW.
//
    e820__range_update(0, PAGE_SIZE, E820_TYPE_RAM, E820_TYPE_RESERVED);
//
// special case: Some BIOSes report the PC BIOS
// area (640Kb -> 1Mb) as RAM even though it is not.
// take them out.
//
    e820__range_remove(BIOS_BEGIN, BIOS_END - BIOS_BEGIN, E820_TYPE_RAM);
    e820__update_table(e820_table);
    }
// called before trim_bios_range() to spare extra sanitize
#[no_mangle]
unsafe extern "C" fn e820_add_kernel_range() -> void __init {
    static void __init e820_add_kernel_range(void)
    {
    let mut start: u64 = __pa_symbol(_text);
    let mut size: u64 = __pa_symbol(_end) - start;
//
// Complain if .text .data and .bss are not marked as E820_TYPE_RAM and
// attempt to fix it by adding the range. We may have a confused BIOS,
// or the user may have used memmap=exactmap or memmap=xxM$yyM to
// exclude kernel range. If we really are running on top non-RAM,
// we will crash later anyways.
//
    if (e820__mapped_all(start, start + size, E820_TYPE_RAM))
    return;
    pr_warn(".text .data .bss are not marked as E820_TYPE_RAM!\n");
    e820__range_remove(start, size, 0);
    e820__range_add(start, size, E820_TYPE_RAM);
    }
#[no_mangle]
unsafe extern "C" fn early_reserve_memory() -> void __init {
    static void __init early_reserve_memory(void)
    {
//
// Reserve the memory occupied by the kernel between _text and
// __end_of_kernel_reserve symbols. Any kernel sections after the
// __end_of_kernel_reserve symbol must be explicitly reserved with a
// separate memblock_reserve() or they will be discarded.
//
    memblock_reserve_kern(__pa_symbol(_text),
    (unsigned long)__end_of_kernel_reserve - (unsigned long)_text);
//
// The first 4Kb of memory is a BIOS owned area, but generally it is
// not listed as such in the E820 table.
//
// Reserve the first 64K of memory since some BIOSes are known to
// corrupt low memory. After the real mode trampoline is allocated the
// rest of the memory below 640k is reserved.
//
// In addition, make sure page 0 is always reserved because on
// systems with L1TF its contents can be leaked to user processes.
//
    memblock_reserve(0, SZ_64K);
    early_reserve_initrd();
    memblock_x86_reserve_range_setup_data();
    reserve_bios_regions();
    trim_snb_memory();
    }
//
// Dump out kernel offset information on panic.
//
    static int
    dump_kernel_offset(struct notifier_block *self, unsigned long v, void *p)
    {
    if (kaslr_enabled()) {
    pr_emerg("Kernel Offset: 0x%lx from 0x%lx (relocation range: 0x%lx-0x%lx)\n",
    kaslr_offset(),
    __START_KERNEL,
    __START_KERNEL_map,
    MODULES_VADDR-1);
    } else {
    pr_emerg("Kernel Offset: disabled\n");
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn x86_configure_nx() {
    void x86_configure_nx(void)
    {
    if (boot_cpu_has(X86_FEATURE_NX))
    __supported_pte_mask |= _PAGE_NX;
    else
    __supported_pte_mask &= ~_PAGE_NX;
    }
#[no_mangle]
unsafe extern "C" fn x86_report_nx() -> void __init {
    static void __init x86_report_nx(void)
    {
    if (!boot_cpu_has(X86_FEATURE_NX)) {
    printk(KERN_NOTICE "Notice: NX (Execute Disable) protection "
    "missing in CPU!\n");
    } else {

    printk(KERN_INFO "NX (Execute Disable) protection: active\n");

// 32bit non-PAE kernel, NX cannot be used
    printk(KERN_NOTICE "Notice: NX (Execute Disable) protection "
    "cannot be enabled: non-PAE kernel!\n");

    }
    }
//
// Determine if we were loaded by an EFI loader.  If so, then we have also been
// passed the efi memmap, systab, etc., so we should use these data structures
// for initialization.  Note, the efi init code path is determined by the
// global efi_enabled. This allows the same kernel image to be used on existing
// systems (with a traditional BIOS) as well as on EFI systems.
//
// setup_arch - architecture-specific boot-time initializations
//
// Note: On x86_64, fixmaps are ready for use even before this is called.
//
#[no_mangle]
pub unsafe extern "C" fn setup_arch(cmdline_p: *mut c_char) -> void __init {
    void __init setup_arch(char **cmdline_p)
    {

    memcpy(&boot_cpu_data, &new_cpu_data, sizeof(new_cpu_data));
//
// copy kernel address range established so far and switch
// to the proper swapper page table
//
    clone_pgd_range(swapper_pg_dir     + KERNEL_PGD_BOUNDARY,
    initial_page_table + KERNEL_PGD_BOUNDARY,
    KERNEL_PGD_PTRS);
    load_cr3(swapper_pg_dir);
//
// Note: Quark X1000 CPUs advertise PGE incorrectly and require
// a cr3 based tlb flush, so the following __flush_tlb_all()
// will not flush anything because the CPU quirk which clears
// X86_FEATURE_PGE has not been invoked yet. Though due to the
// load_cr3() above the TLB has been flushed already. The
// quirk is invoked before subsequent calls to __flush_tlb_all()
// so proper operation is guaranteed.
//
    __flush_tlb_all();

    printk(KERN_INFO "Command line: %s\n", boot_command_line);
    boot_cpu_data.x86_phys_bits = MAX_PHYSMEM_BITS;

    strscpy(boot_command_line, builtin_cmdline, COMMAND_LINE_SIZE);

    if (builtin_cmdline[0]) {
// append boot loader cmdline to builtin
    strlcat(builtin_cmdline, " ", COMMAND_LINE_SIZE);
    strlcat(builtin_cmdline, boot_command_line, COMMAND_LINE_SIZE);
    strscpy(boot_command_line, builtin_cmdline, COMMAND_LINE_SIZE);
    }

    builtin_cmdline_added = true;

//
// Prepend the build-time-rendered embedded "kernel" keys here so
// parse_early_param() below sees them, using the same opt-in as the
// runtime parser, plus the build-time CONFIG_BOOT_CONFIG_FORCE.
//
    if (bootconfig_cmdline_requested(boot_command_line, core::ptr::null_mut()) ||
    IS_ENABLED(CONFIG_BOOT_CONFIG_FORCE))
    xbc_prepend_embedded_cmdline(boot_command_line,
    COMMAND_LINE_SIZE);

    strscpy(command_line, boot_command_line, COMMAND_LINE_SIZE);
// cmdline_p = command_line;
//
// If we have OLPC OFW, we might end up relocating the fixmap due to
// reserve_top(), so do this before touching the ioremap area.
//
    olpc_ofw_detect();
    idt_setup_early_traps();
    early_cpu_init();
    jump_label_init();
    static_call_init();
    early_ioremap_init();
    setup_olpc_ofw_pgd();
    parse_boot_params();
    x86_init.oem.arch_setup();
//
// Do some memory reservations *before* memory is added to memblock, so
// memblock allocations won't overwrite it.
//
// After this point, everything still needed from the boot loader or
// firmware or kernel text should be early reserved or marked not RAM in
// e820. All other memory is free game.
//
// This call needs to happen before e820__memory_setup() which calls the
// xen_memory_setup() on Xen dom0 which relies on the fact that those
// early reservations have happened already.
//
    early_reserve_memory();
    iomem_resource.end = (1ULL << boot_cpu_data.x86_phys_bits) - 1;
    e820__memory_setup();
    parse_setup_data();
    copy_edd();
    setup_initial_init_mm(_text, _etext, _edata, (void *)_brk_end);
//
// x86_configure_nx() is called before parse_early_param() to detect
// whether hardware doesn't support NX (so that the early EHCI debug
// console setup can safely call set_fixmap()).
//
    x86_configure_nx();
    parse_early_param();
    if (efi_enabled(EFI_BOOT))
    efi_memblock_x86_reserve_range();
    x86_report_nx();
    apic_setup_apic_calls();
    if (acpi_mps_check()) {

    apic_is_disabled = true;

    setup_clear_cpu_cap(X86_FEATURE_APIC);
    }
    e820__finish_early_params();
    if (efi_enabled(EFI_BOOT))
    efi_init();
    reserve_ibft_region();
    x86_init.resources.dmi_setup();
//
// VMware detection requires dmi to be available, so this
// needs to be done after dmi_setup(), for the boot CPU.
// For some guest types (Xen PV, SEV-SNP, TDX) it is required to be
// called before cache_bp_init() for setting up MTRR state.
//
    init_hypervisor_platform();
    tsc_early_init();
    x86_init.resources.probe_roms();
//
// Add resources for kernel text and data to the iomem_resource.
// Do it after parse_early_param, so it can be debugged.
//
    setup_kernel_resources();
    e820_add_kernel_range();
    trim_bios_range();

    if (ppro_with_ram_bug()) {
    pr_info("Applying PPro RAM bug workaround: punching 256 kB hole at 1.75 GB physical.\n");
    e820__range_update(0x70000000ULL, SZ_256K, E820_TYPE_RAM, E820_TYPE_RESERVED);
    e820__update_table(e820_table);
    }

    early_gart_iommu_check();

//
// partially used pages are not usable - thus
// we are rounding upwards:
//
    max_pfn = e820__end_of_ram_pfn();
// update e820 for memory not covered by WB MTRRs
    cache_bp_init();
    if (mtrr_trim_uncached_memory(max_pfn))
    max_pfn = e820__end_of_ram_pfn();
    max_possible_pfn = max_pfn;
//
// Define random base addresses for memory sections after max_pfn is
// defined and before each memory section base is used.
//
    kernel_randomize_memory();

// max_low_pfn get updated here
    find_low_pfn_range();

    check_x2apic();
// How many end-of-memory variables you have, grandma!
// need this before calling reserve_initrd
    if (max_pfn > (1UL<<(32 - PAGE_SHIFT)))
    max_low_pfn = e820__end_of_low_ram_pfn();
    else
    max_low_pfn = max_pfn;

// Find and reserve MPTABLE area
    x86_init.mpparse.find_mptable();
    early_alloc_pgt_buf();
//
// Need to conclude brk, before e820__memblock_setup()
// it could use memblock_find_in_range, could overlap with
// brk area.
//
    reserve_brk();
    cleanup_highmap();
    e820__memblock_setup();
//
// Needs to run after memblock setup because it needs the physical
// memory size.
//
    mem_encrypt_setup_arch();
    cc_random_init();
    efi_find_mirror();
    efi_esrt_init();
    efi_mokvar_table_init();
//
// The EFI specification says that boot service code won't be
// called after ExitBootServices(). This is, in fact, a lie.
//
    efi_reserve_boot_services();
// preallocate 4k for mptable mpc
    e820__memblock_alloc_reserved_mpc_new();

    setup_bios_corruption_check();

    printk(KERN_DEBUG "initial memory mapped: [mem 0x00000000-%#010lx]\n",
    (max_pfn_mapped<<PAGE_SHIFT) - 1);

//
// Find free memory for the real mode trampoline and place it there. If
// there is not enough free memory under 1M, on EFI-enabled systems
// there will be additional attempt to reclaim the memory for the real
// mode trampoline at efi_free_boot_services().
//
// Unconditionally reserve the entire first 1M of RAM because BIOSes
// are known to corrupt low memory and several hundred kilobytes are not
// worth complex detection what memory gets clobbered. Windows does the
// same thing for very similar reasons.
//
// Moreover, on machines with SandyBridge graphics or in setups that use
// crashkernel the entire 1M is reserved anyway.
//
// Note the host kernel TDX also requires the first 1MB being reserved.
//
    x86_platform.realmode_reserve();
    init_mem_mapping();
//
// init_mem_mapping() relies on the early IDT page fault handling.
// Now either enable FRED or install the real page fault handler
// for 64-bit in the IDT.
//
    cpu_init_replace_early_idt();
//
// Update mmu_cr4_features (and, indirectly, trampoline_cr4_features)
// with the current CR4 value.  This may not be necessary, but
// auditing all the early-boot CR4 manipulation would be needed to
// rule it out.
//
// Mask off features that don't work outside long mode (just
// PCIDE for now).
//
    mmu_cr4_features = __read_cr4() & ~X86_CR4_PCIDE;
    memblock_set_current_limit(get_max_mapped());
//
// NOTE: On x86-32, only from this point on, fixmaps are ready for use.
//

    if (init_ohci1394_dma_early)
    init_ohci1394_dma_on_all_controllers();

// Allocate bigger log buffer
    setup_log_buf(1);
    if (efi_enabled(EFI_BOOT)) {
    switch (boot_params.secure_boot) {
    case efi_secureboot_mode_disabled:
    pr_info("Secure boot disabled\n");
    break;
    case efi_secureboot_mode_enabled:
    pr_info("Secure boot enabled\n");
    break;
    default:
    pr_info("Secure boot could not be determined\n");
    break;
    }
    }
    reserve_initrd();
    acpi_table_upgrade();
// Look for ACPI tables and reserve memory occupied by them.
    acpi_boot_table_init();
    vsmp_init();
    io_delay_init();
    early_platform_quirks();
// Some platforms need the APIC registered for NUMA configuration
    early_acpi_boot_init();
    x86_init.mpparse.early_parse_smp_cfg();
    x86_flattree_get_config();
    initmem_init();
    dma_contiguous_reserve(max_pfn_mapped << PAGE_SHIFT);
//
// Reserve memory for crash kernel after SRAT is parsed so that it
// won't consume hotpluggable memory.
//
    arch_reserve_crashkernel();
    if (!early_xdbc_setup_hardware())
    early_xdbc_register_console();
    x86_init.paging.pagetable_init();
    kasan_init();
//
// Sync back kernel address range.
//
// FIXME: Can the later sync in setup_cpu_entry_areas() replace
// this call?
//
    sync_initial_page_table();
    tboot_probe();
    map_vsyscall();
    x86_32_probe_apic();
    early_quirks();
    topology_apply_cmdline_limits_early();
//
// Parse SMP configuration. Try ACPI first and then the platform
// specific parser.
//
    acpi_boot_init();
    x86_init.mpparse.parse_smp_cfg();
// Last opportunity to detect and map the local APIC
    init_apic_mappings();
    topology_init_possible_cpus();
    init_cpu_to_node();
    init_gi_nodes();
    io_apic_init_mappings();
    x86_init.hyper.guest_late_init();
    e820__reserve_resources();
    e820__register_nosave_regions(max_pfn);
    x86_init.resources.reserve_resources();
    e820__setup_pci_gap();

    if (!efi_enabled(EFI_BOOT) || (efi_mem_type(0xa0000) != EFI_CONVENTIONAL_MEMORY))
    vgacon_register_screen(&sysfb_primary_display.screen);

    x86_init.oem.banner();
    x86_init.timers.wallclock_init();
//
// This needs to run before setup_local_APIC() which soft-disables the
// local APIC temporarily and that masks the thermal LVT interrupt,
// leading to softlockups on machines which have configured SMI
// interrupt delivery.
//
    therm_lvt_init();
    mcheck_init();
    register_refined_jiffies(PIT_TICK_RATE);

    if (efi_enabled(EFI_BOOT))
    efi_apply_memmap_quirks();

    unwind_init();
    }

    static struct resource video_ram_resource = {
    .name	= "Video RAM area",
    .start	= 0xa0000,
    .end	= 0xbffff,
    .flags	= IORESOURCE_BUSY | IORESOURCE_MEM
    };
#[no_mangle]
pub unsafe extern "C" fn i386_reserve_resources() -> void __init {
    void __init i386_reserve_resources(void)
    {
    request_resource(&iomem_resource, &video_ram_resource);
    reserve_standard_io_resources();
    }

    static struct notifier_block kernel_offset_notifier = {
    .notifier_call = dump_kernel_offset
    };
#[no_mangle]
unsafe extern "C" fn register_kernel_offset_dumper() -> int __init {
    static int __init register_kernel_offset_dumper(void)
    {
    atomic_notifier_chain_register(&panic_notifier_list,
    &kernel_offset_notifier);
    return 0;
    }
    __initcall(register_kernel_offset_dumper);

#[no_mangle]
pub unsafe extern "C" fn arch_cpu_is_hotpluggable(cpu: c_int) -> bool {
    bool arch_cpu_is_hotpluggable(int cpu)
    {
    return cpu > 0;
    }
