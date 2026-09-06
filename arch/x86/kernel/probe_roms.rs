//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/probe_roms.c
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

    static struct resource system_rom_resource = {
    .name	= "System ROM",
    .start	= 0xf0000,
    .end	= 0xfffff,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    };
    static struct resource extension_rom_resource = {
    .name	= "Extension ROM",
    .start	= 0xe0000,
    .end	= 0xeffff,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    };
    static struct resource adapter_rom_resources[] = { {
    .name 	= "Adapter ROM",
    .start	= 0xc8000,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    }, {
    .name 	= "Adapter ROM",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    }, {
    .name 	= "Adapter ROM",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    }, {
    .name 	= "Adapter ROM",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    }, {
    .name 	= "Adapter ROM",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    }, {
    .name 	= "Adapter ROM",
    .start	= 0,
    .end	= 0,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    } };
    static struct resource video_rom_resource = {
    .name 	= "Video ROM",
    .start	= 0xc0000,
    .end	= 0xc7fff,
    .flags	= IORESOURCE_BUSY | IORESOURCE_READONLY | IORESOURCE_MEM
    };
// does this oprom support the given pci device, or any of the devices
// that the driver supports?
//
#[no_mangle]
unsafe extern "C" fn match_id(pdev: *mut pci_dev, vendor: c_ushort, device: c_ushort) -> bool {
    static bool match_id(struct pci_dev *pdev, unsigned short vendor, unsigned short device)
    {
    struct pci_driver *drv = to_pci_driver(pdev.dev.driver);
    const struct pci_device_id *id;
    if (pdev.vendor == vendor && pdev.device == device)
    return true;
    for (id = drv ? drv.id_table : core::ptr::null_mut(); id && id.vendor; id++)
    if (id.vendor == vendor && id.device == device)
    break;
    return id && id.vendor;
    }
    static bool probe_list(struct pci_dev *pdev, unsigned short vendor,
    const void *rom_list)
    {
    unsigned short device;
    do {
    if (get_kernel_nofault(device, rom_list) != 0)
    device = 0;
    if (device && match_id(pdev, vendor, device))
    break;
    rom_list += 2;
    } while (device);
    return !!device;
    }
    static struct resource *find_oprom(struct pci_dev *pdev)
    {
    struct resource *oprom = core::ptr::null_mut();
    int i;
    for (i = 0; i < ARRAY_SIZE(adapter_rom_resources); i++) {
    struct resource *res = &adapter_rom_resources[i];
    unsigned short offset, vendor, device, list, rev;
    const void *rom;
    if (res.end == 0)
    break;
    rom = isa_bus_to_virt(res.start);
    if (get_kernel_nofault(offset, rom + 0x18) != 0)
    continue;
    if (get_kernel_nofault(vendor, rom + offset + 0x4) != 0)
    continue;
    if (get_kernel_nofault(device, rom + offset + 0x6) != 0)
    continue;
    if (match_id(pdev, vendor, device)) {
    oprom = res;
    break;
    }
    if (get_kernel_nofault(list, rom + offset + 0x8) == 0 &&
    get_kernel_nofault(rev, rom + offset + 0xc) == 0 &&
    rev >= 3 && list &&
    probe_list(pdev, vendor, rom + offset + list)) {
    oprom = res;
    break;
    }
    }
    return oprom;
    }
    void __iomem *pci_map_biosrom(struct pci_dev *pdev)
    {
    struct resource *oprom = find_oprom(pdev);
    if (!oprom)
    return core::ptr::null_mut();
    return ioremap(oprom.start, resource_size(oprom));
    }
    EXPORT_SYMBOL(pci_map_biosrom);
#[no_mangle]
pub unsafe extern "C" fn pci_unmap_biosrom(image: *mut void __iomem) {
    void pci_unmap_biosrom(void __iomem *image)
    {
    iounmap(image);
    }
    EXPORT_SYMBOL(pci_unmap_biosrom);
#[no_mangle]
pub unsafe extern "C" fn pci_biosrom_size(pdev: *mut pci_dev) -> usize {
    size_t pci_biosrom_size(struct pci_dev *pdev)
    {
    struct resource *oprom = find_oprom(pdev);
    return oprom ? resource_size(oprom) : 0;
    }
    EXPORT_SYMBOL(pci_biosrom_size);
pub const ROMSIGNATURE: c_uint = 0xaa55;
#[no_mangle]
unsafe extern "C" fn romsignature(rom: *const c_uchar) -> int __init {
    static int __init romsignature(const unsigned char *rom)
    {
    let mut ptr: *const unsigned short  const = (const unsigned short *)rom;
    unsigned short sig;
    return get_kernel_nofault(sig, ptr) == 0 && sig == ROMSIGNATURE;
    }
#[no_mangle]
unsafe extern "C" fn romchecksum(rom: *const c_uchar, length: c_ulong) -> int __init {
    static int __init romchecksum(const unsigned char *rom, unsigned long length)
    {
    unsigned char sum, c;
    for (sum = 0; length && get_kernel_nofault(c, rom++) == 0; length--)
    sum += c;
    return !length && !sum;
    }
#[no_mangle]
pub unsafe extern "C" fn probe_roms() -> void __init {
    void __init probe_roms(void)
    {
    unsigned long start, length, upper;
    const unsigned char *rom;
    unsigned char c;
    int i;
// video rom
    upper = adapter_rom_resources[0].start;
    for (start = video_rom_resource.start; start < upper; start += 2048) {
    rom = isa_bus_to_virt(start);
    if (!romsignature(rom))
    continue;
    video_rom_resource.start = start;
    if (get_kernel_nofault(c, rom + 2) != 0)
    continue;
// 0 < length <= 0x7f * 512, historically
    length = c * 512;
// if checksum okay, trust length byte
    if (length && romchecksum(rom, length))
    video_rom_resource.end = start + length - 1;
    request_resource(&iomem_resource, &video_rom_resource);
    break;
    }
    start = (video_rom_resource.end + 1 + 2047) & ~2047UL;
    if (start < upper)
    start = upper;
// system rom
    request_resource(&iomem_resource, &system_rom_resource);
    upper = system_rom_resource.start;
// check for extension rom (ignore length byte!)
    rom = isa_bus_to_virt(extension_rom_resource.start);
    if (romsignature(rom)) {
    length = resource_size(&extension_rom_resource);
    if (romchecksum(rom, length)) {
    request_resource(&iomem_resource, &extension_rom_resource);
    upper = extension_rom_resource.start;
    }
    }
// check for adapter roms on 2k boundaries
    for (i = 0; i < ARRAY_SIZE(adapter_rom_resources) && start < upper; start += 2048) {
    rom = isa_bus_to_virt(start);
    if (!romsignature(rom))
    continue;
    if (get_kernel_nofault(c, rom + 2) != 0)
    continue;
// 0 < length <= 0x7f * 512, historically
    length = c * 512;
// but accept any length that fits if checksum okay
    if (!length || start + length > upper || !romchecksum(rom, length))
    continue;
    adapter_rom_resources[i].start = start;
    adapter_rom_resources[i].end = start + length - 1;
    request_resource(&iomem_resource, &adapter_rom_resources[i]);
    start = adapter_rom_resources[i++].end & ~2047UL;
    }
    }
