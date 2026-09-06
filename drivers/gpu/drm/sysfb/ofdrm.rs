//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sysfb/ofdrm.c
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

pub const DRIVER_MAJOR: c_int = 1;
pub const DRIVER_MINOR: c_int = 0;
pub const PCI_VENDOR_ID_ATI_R520: c_uint = 0x7100;
pub const PCI_VENDOR_ID_ATI_R600: c_uint = 0x9400;
pub const OFDRM_GAMMA_LUT_SIZE: c_int = 256;
// Definitions used by the Avivo palette
pub const AVIVO_DC_LUT_RW_SELECT: c_uint = 0x6480;
pub const AVIVO_DC_LUT_RW_MODE: c_uint = 0x6484;
pub const AVIVO_DC_LUT_RW_INDEX: c_uint = 0x6488;
pub const AVIVO_DC_LUT_SEQ_COLOR: c_uint = 0x648c;
pub const AVIVO_DC_LUT_PWL_DATA: c_uint = 0x6490;
pub const AVIVO_DC_LUT_30_COLOR: c_uint = 0x6494;
pub const AVIVO_DC_LUT_READ_PIPE_SELECT: c_uint = 0x6498;
pub const AVIVO_DC_LUT_WRITE_EN_MASK: c_uint = 0x649c;
pub const AVIVO_DC_LUT_AUTOFILL: c_uint = 0x64a0;
pub const AVIVO_DC_LUTA_CONTROL: c_uint = 0x64c0;
pub const AVIVO_DC_LUTA_BLACK_OFFSET_BLUE: c_uint = 0x64c4;
pub const AVIVO_DC_LUTA_BLACK_OFFSET_GREEN: c_uint = 0x64c8;
pub const AVIVO_DC_LUTA_BLACK_OFFSET_RED: c_uint = 0x64cc;
pub const AVIVO_DC_LUTA_WHITE_OFFSET_BLUE: c_uint = 0x64d0;
pub const AVIVO_DC_LUTA_WHITE_OFFSET_GREEN: c_uint = 0x64d4;
pub const AVIVO_DC_LUTA_WHITE_OFFSET_RED: c_uint = 0x64d8;
pub const AVIVO_DC_LUTB_CONTROL: c_uint = 0x6cc0;
pub const AVIVO_DC_LUTB_BLACK_OFFSET_BLUE: c_uint = 0x6cc4;
pub const AVIVO_DC_LUTB_BLACK_OFFSET_GREEN: c_uint = 0x6cc8;
pub const AVIVO_DC_LUTB_BLACK_OFFSET_RED: c_uint = 0x6ccc;
pub const AVIVO_DC_LUTB_WHITE_OFFSET_BLUE: c_uint = 0x6cd0;
pub const AVIVO_DC_LUTB_WHITE_OFFSET_GREEN: c_uint = 0x6cd4;
pub const AVIVO_DC_LUTB_WHITE_OFFSET_RED: c_uint = 0x6cd8;
    enum ofdrm_model {
    OFDRM_MODEL_UNKNOWN,
    OFDRM_MODEL_MACH64, /* ATI Mach64 */
    OFDRM_MODEL_RAGE128, /* ATI Rage128 */
    OFDRM_MODEL_RAGE_M3A, /* ATI Rage Mobility M3 Head A */
    OFDRM_MODEL_RAGE_M3B, /* ATI Rage Mobility M3 Head B */
    OFDRM_MODEL_RADEON, /* ATI Radeon */
    OFDRM_MODEL_GXT2000, /* IBM GXT2000 */
    OFDRM_MODEL_AVIVO, /* ATI R5xx */
    OFDRM_MODEL_QEMU, /* QEMU VGA */
    };
//
// Helpers for display nodes
//
#[no_mangle]
unsafe extern "C" fn display_get_validated_int(dev: *mut drm_device, name: *const c_char, value: u32) -> c_int {
    static int display_get_validated_int(struct drm_device *dev, const char *name, uint32_t value)
    {
    return drm_sysfb_get_validated_int(dev, name, value, INT_MAX);
    }
#[no_mangle]
unsafe extern "C" fn display_get_validated_int0(dev: *mut drm_device, name: *const c_char, value: u32) -> c_int {
    static int display_get_validated_int0(struct drm_device *dev, const char *name, uint32_t value)
    {
    return drm_sysfb_get_validated_int0(dev, name, value, INT_MAX);
    }
    static const struct drm_format_info *display_get_validated_format(struct drm_device *dev,
    u32 depth, bool big_endian)
    {
    const struct drm_format_info *info;
    u32 format;
    switch (depth) {
    case 8:
    format = drm_mode_legacy_fb_format(8, 8);
    break;
    case 15:
    case 16:
    format = drm_mode_legacy_fb_format(16, depth);
    break;
    case 32:
    format = drm_mode_legacy_fb_format(32, 24);
    break;
    default:
    drm_err(dev, "unsupported framebuffer depth %u\n", depth);
    return ERR_PTR(-EINVAL);
    }
//
// DRM formats assume little-endian byte order. Update the format
// if the scanout buffer uses big-endian ordering.
//
    if (big_endian) {
    switch (format) {
    case DRM_FORMAT_XRGB8888:
    format = DRM_FORMAT_BGRX8888;
    break;
    case DRM_FORMAT_ARGB8888:
    format = DRM_FORMAT_BGRA8888;
    break;
    case DRM_FORMAT_RGB565:
    format = DRM_FORMAT_RGB565 | DRM_FORMAT_BIG_ENDIAN;
    break;
    case DRM_FORMAT_XRGB1555:
    format = DRM_FORMAT_XRGB1555 | DRM_FORMAT_BIG_ENDIAN;
    break;
    default:
    break;
    }
    }
    info = drm_format_info(format);
    if (!info) {
    drm_err(dev, "cannot find framebuffer format for depth %u\n", depth);
    return ERR_PTR(-EINVAL);
    }
    return info;
    }
    static int display_read_u32_of(struct drm_device *dev, struct device_node *of_node,
    const char *name, u32 *value)
    {
    let mut ret: c_int = of_property_read_u32(of_node, name, value);
    if (ret)
    drm_err(dev, "cannot parse framebuffer %s: error %d\n", name, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn display_get_big_endian_of(dev: *mut drm_device, of_node: *mut device_node) -> bool {
    static bool display_get_big_endian_of(struct drm_device *dev, struct device_node *of_node)
    {
    bool big_endian;

    big_endian = !of_property_read_bool(of_node, "little-endian");

    big_endian = of_property_read_bool(of_node, "big-endian");

    return big_endian;
    }
#[no_mangle]
unsafe extern "C" fn display_get_width_of(dev: *mut drm_device, of_node: *mut device_node) -> c_int {
    static int display_get_width_of(struct drm_device *dev, struct device_node *of_node)
    {
    u32 width;
    let mut ret: c_int = display_read_u32_of(dev, of_node, "width", &width);
    if (ret)
    return ret;
    return display_get_validated_int0(dev, "width", width);
    }
#[no_mangle]
unsafe extern "C" fn display_get_height_of(dev: *mut drm_device, of_node: *mut device_node) -> c_int {
    static int display_get_height_of(struct drm_device *dev, struct device_node *of_node)
    {
    u32 height;
    let mut ret: c_int = display_read_u32_of(dev, of_node, "height", &height);
    if (ret)
    return ret;
    return display_get_validated_int0(dev, "height", height);
    }
#[no_mangle]
unsafe extern "C" fn display_get_depth_of(dev: *mut drm_device, of_node: *mut device_node) -> c_int {
    static int display_get_depth_of(struct drm_device *dev, struct device_node *of_node)
    {
    u32 depth;
    let mut ret: c_int = display_read_u32_of(dev, of_node, "depth", &depth);
    if (ret)
    return ret;
    return display_get_validated_int0(dev, "depth", depth);
    }
#[no_mangle]
unsafe extern "C" fn display_get_linebytes_of(dev: *mut drm_device, of_node: *mut device_node) -> c_int {
    static int display_get_linebytes_of(struct drm_device *dev, struct device_node *of_node)
    {
    u32 linebytes;
    let mut ret: c_int = display_read_u32_of(dev, of_node, "linebytes", &linebytes);
    if (ret)
    return ret;
    return display_get_validated_int(dev, "linebytes", linebytes);
    }
#[no_mangle]
unsafe extern "C" fn display_get_address_of(dev: *mut drm_device, of_node: *mut device_node) -> u64 {
    static u64 display_get_address_of(struct drm_device *dev, struct device_node *of_node)
    {
    u32 address;
    int ret;
//
// Not all devices provide an address property, it's not
// a bug if this fails. The driver will try to find the
// framebuffer base address from the device's memory regions.
//
    ret = of_property_read_u32(of_node, "address", &address);
    if (ret)
    return OF_BAD_ADDR;
    return address;
    }
    static const u8 *display_get_edid_of(struct drm_device *dev, struct device_node *of_node,
    u8 buf[EDID_LENGTH])
    {
    let mut ret: c_int = of_property_read_u8_array(of_node, "EDID", buf, EDID_LENGTH);
    if (ret)
    return core::ptr::null_mut();
    return buf;
    }
#[no_mangle]
unsafe extern "C" fn is_avivo(vendor: u32, device: u32) -> bool {
    static bool is_avivo(u32 vendor, u32 device)
    {
// This will match most R5xx
    return (vendor == PCI_VENDOR_ID_ATI) &&
    ((device >= PCI_VENDOR_ID_ATI_R520 && device < 0x7800) ||
    (device >= PCI_VENDOR_ID_ATI_R600));
    }
#[no_mangle]
unsafe extern "C" fn display_get_model_of(dev: *mut drm_device, of_node: *mut device_node) -> enum ofdrm_model {
    static enum ofdrm_model display_get_model_of(struct drm_device *dev, struct device_node *of_node)
    {
    let mut model: enum ofdrm_model = OFDRM_MODEL_UNKNOWN;
    if (of_node_name_prefix(of_node, "ATY,Rage128")) {
    model = OFDRM_MODEL_RAGE128;
    } else if (of_node_name_prefix(of_node, "ATY,RageM3pA") ||
    of_node_name_prefix(of_node, "ATY,RageM3p12A")) {
    model = OFDRM_MODEL_RAGE_M3A;
    } else if (of_node_name_prefix(of_node, "ATY,RageM3pB")) {
    model = OFDRM_MODEL_RAGE_M3B;
    } else if (of_node_name_prefix(of_node, "ATY,Rage6")) {
    model = OFDRM_MODEL_RADEON;
    } else if (of_node_name_prefix(of_node, "ATY,")) {
    return OFDRM_MODEL_MACH64;
    } else if (of_device_is_compatible(of_node, "pci1014,b7") ||
    of_device_is_compatible(of_node, "pci1014,21c")) {
    model = OFDRM_MODEL_GXT2000;
    } else if (of_node_name_prefix(of_node, "vga,Display-")) {
    struct device_node *of_parent;
    const __be32 *vendor_p, *device_p;
// Look for AVIVO initialized by SLOF
    of_parent = of_get_parent(of_node);
    vendor_p = of_get_property(of_parent, "vendor-id", core::ptr::null_mut());
    device_p = of_get_property(of_parent, "device-id", core::ptr::null_mut());
    if (vendor_p && device_p) {
    let mut vendor: u32 = be32_to_cpup(vendor_p);
    let mut device: u32 = be32_to_cpup(device_p);
    if (is_avivo(vendor, device))
    model = OFDRM_MODEL_AVIVO;
    }
    of_node_put(of_parent);
    } else if (of_device_is_compatible(of_node, "qemu,std-vga")) {
    model = OFDRM_MODEL_QEMU;
    }
    return model;
    }
//
// Open Firmware display device
//
    struct ofdrm_device;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ofdrm_device_funcs {
    void __iomem *(*cmap_ioremap)(struct ofdrm_device *odev,
    struct device_node *of_node,
    pub fb_bas): u64,
    void (*cmap_write)(struct ofdrm_device *odev, unsigned char index,
    pub b): unsigned char r, unsigned char g, unsigned char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ofdrm_device {
    pub sysfb: drm_sysfb_device,
    pub funcs: *const ofdrm_device_funcs,
// colormap
    pub cmap_base: *mut void __iomem,
    pub edid: [u8; EDID_LENGTH],
// modesetting
    pub formats: [u32; DRM_SYSFB_PLANE_NFORMATS(1)],
    pub primary_plane: drm_plane,
    pub crtc: drm_crtc,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
}

    static struct ofdrm_device *ofdrm_device_of_dev(struct drm_device *dev)
    {
    return container_of(to_drm_sysfb_device(dev), struct ofdrm_device, sysfb);
    }
//
// Hardware
//

    static struct pci_dev *display_get_pci_dev_of(struct drm_device *dev, struct device_node *of_node)
    {
    const __be32 *vendor_p, *device_p;
    u32 vendor, device;
    struct pci_dev *pcidev;
    vendor_p = of_get_property(of_node, "vendor-id", core::ptr::null_mut());
    if (!vendor_p)
    return ERR_PTR(-ENODEV);
    vendor = be32_to_cpup(vendor_p);
    device_p = of_get_property(of_node, "device-id", core::ptr::null_mut());
    if (!device_p)
    return ERR_PTR(-ENODEV);
    device = be32_to_cpup(device_p);
    pcidev = pci_get_device(vendor, device, core::ptr::null_mut());
    if (!pcidev)
    return ERR_PTR(-ENODEV);
    return pcidev;
    }
#[no_mangle]
unsafe extern "C" fn ofdrm_pci_release(data: *mut c_void) {
    static void ofdrm_pci_release(void *data)
    {
    struct pci_dev *pcidev = data;
    pci_disable_device(pcidev);
    pci_dev_put(pcidev);
    }
#[no_mangle]
unsafe extern "C" fn ofdrm_device_init_pci(odev: *mut ofdrm_device) -> c_int {
    static int ofdrm_device_init_pci(struct ofdrm_device *odev)
    {
    struct drm_device *dev = &odev.sysfb.dev;
    struct platform_device *pdev = to_platform_device(dev.dev);
    struct device_node *of_node = pdev.dev.of_node;
    struct pci_dev *pcidev;
    int ret;
//
// Never use pcim_ or other managed helpers on the returned PCI
// device. Otherwise, probing the native driver will fail for
// resource conflicts. PCI-device management has to be tied to
// the lifetime of the platform device until the native driver
// takes over.
//
    pcidev = display_get_pci_dev_of(dev, of_node);
    if (IS_ERR(pcidev))
    return 0; /* no PCI device found; ignore the error */
    ret = pci_enable_device(pcidev);
    if (ret) {
    drm_err(dev, "pci_enable_device(%s) failed: %d\n",
    dev_name(&pcidev.dev), ret);
    pci_dev_put(pcidev);
    return ret;
    }
    ret = devm_add_action_or_reset(&pdev.dev, ofdrm_pci_release, pcidev);
    if (ret)
    return ret;
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ofdrm_device_init_pci(odev: *mut ofdrm_device) -> c_int {
    static int ofdrm_device_init_pci(struct ofdrm_device *odev)
    {
    return 0;
    }

//
// OF display settings
//
    static struct resource *ofdrm_find_fb_resource(struct ofdrm_device *odev,
    struct resource *fb_res)
    {
    struct platform_device *pdev = to_platform_device(odev.sysfb.dev.dev);
    struct resource *res, *max_res = core::ptr::null_mut();
    u32 i;
    for (i = 0; pdev.num_resources; ++i) {
    res = platform_get_resource(pdev, IORESOURCE_MEM, i);
    if (!res)
    break; /* all resources processed */
    if (resource_size(res) < resource_size(fb_res))
    continue; /* resource too small */
    if (fb_res.start && resource_contains(res, fb_res))
    return res; /* resource contains framebuffer */
    if (!max_res || resource_size(res) > resource_size(max_res))
    max_res = res; /* store largest resource as fallback */
    }
    return max_res;
    }
//
// Colormap / Palette
//
    static void __iomem *get_cmap_address_of(struct ofdrm_device *odev, struct device_node *of_node,
    int bar_no, unsigned long offset, unsigned long size)
    {
    struct drm_device *dev = &odev.sysfb.dev;
    const __be32 *addr_p;
    u64 max_size, address;
    unsigned int flags;
    void __iomem *mem;
    addr_p = of_get_pci_address(of_node, bar_no, &max_size, &flags);
    if (!addr_p)
    addr_p = of_get_address(of_node, bar_no, &max_size, &flags);
    if (!addr_p)
    return IOMEM_ERR_PTR(-ENODEV);
    if ((flags & (IORESOURCE_IO | IORESOURCE_MEM)) == 0)
    return IOMEM_ERR_PTR(-ENODEV);
    if ((offset + size) >= max_size)
    return IOMEM_ERR_PTR(-ENODEV);
    address = of_translate_address(of_node, addr_p);
    if (address == OF_BAD_ADDR)
    return IOMEM_ERR_PTR(-ENODEV);
    mem = devm_ioremap(dev.dev, address + offset, size);
    if (!mem)
    return IOMEM_ERR_PTR(-ENOMEM);
    return mem;
    }
    static void __iomem *ofdrm_mach64_cmap_ioremap(struct ofdrm_device *odev,
    struct device_node *of_node,
    u64 fb_base)
    {
    struct drm_device *dev = &odev.sysfb.dev;
    u64 address;
    void __iomem *cmap_base;
    address = fb_base & 0xff000000ul;
    address += 0x7ff000;
    cmap_base = devm_ioremap(dev.dev, address, 0x1000);
    if (!cmap_base)
    return IOMEM_ERR_PTR(-ENOMEM);
    return cmap_base;
    }
    static void ofdrm_mach64_cmap_write(struct ofdrm_device *odev, unsigned char index,
    unsigned char r, unsigned char g, unsigned char b)
    {
    void __iomem *addr = odev.cmap_base + 0xcc0;
    void __iomem *data = odev.cmap_base + 0xcc0 + 1;
    writeb(index, addr);
    writeb(r, data);
    writeb(g, data);
    writeb(b, data);
    }
    static void __iomem *ofdrm_rage128_cmap_ioremap(struct ofdrm_device *odev,
    struct device_node *of_node,
    u64 fb_base)
    {
    return get_cmap_address_of(odev, of_node, 2, 0, 0x1fff);
    }
    static void ofdrm_rage128_cmap_write(struct ofdrm_device *odev, unsigned char index,
    unsigned char r, unsigned char g, unsigned char b)
    {
    void __iomem *addr = odev.cmap_base + 0xb0;
    void __iomem *data = odev.cmap_base + 0xb4;
    let mut color: u32 = (r << 16) | (g << 8) | b;
    writeb(index, addr);
    writel(color, data);
    }
    static void __iomem *ofdrm_rage_m3a_cmap_ioremap(struct ofdrm_device *odev,
    struct device_node *of_node,
    u64 fb_base)
    {
    return get_cmap_address_of(odev, of_node, 2, 0, 0x1fff);
    }
    static void ofdrm_rage_m3a_cmap_write(struct ofdrm_device *odev, unsigned char index,
    unsigned char r, unsigned char g, unsigned char b)
    {
    void __iomem *dac_ctl = odev.cmap_base + 0x58;
    void __iomem *addr = odev.cmap_base + 0xb0;
    void __iomem *data = odev.cmap_base + 0xb4;
    let mut color: u32 = (r << 16) | (g << 8) | b;
    u32 val;
// Clear PALETTE_ACCESS_CNTL in DAC_CNTL
    val = readl(dac_ctl);
    val &= ~0x20;
    writel(val, dac_ctl);
// Set color at palette index
    writeb(index, addr);
    writel(color, data);
    }
    static void __iomem *ofdrm_rage_m3b_cmap_ioremap(struct ofdrm_device *odev,
    struct device_node *of_node,
    u64 fb_base)
    {
    return get_cmap_address_of(odev, of_node, 2, 0, 0x1fff);
    }
    static void ofdrm_rage_m3b_cmap_write(struct ofdrm_device *odev, unsigned char index,
    unsigned char r, unsigned char g, unsigned char b)
    {
    void __iomem *dac_ctl = odev.cmap_base + 0x58;
    void __iomem *addr = odev.cmap_base + 0xb0;
    void __iomem *data = odev.cmap_base + 0xb4;
    let mut color: u32 = (r << 16) | (g << 8) | b;
    u32 val;
// Set PALETTE_ACCESS_CNTL in DAC_CNTL
    val = readl(dac_ctl);
    val |= 0x20;
    writel(val, dac_ctl);
// Set color at palette index
    writeb(index, addr);
    writel(color, data);
    }
    static void __iomem *ofdrm_radeon_cmap_ioremap(struct ofdrm_device *odev,
    struct device_node *of_node,
    u64 fb_base)
    {
    return get_cmap_address_of(odev, of_node, 1, 0, 0x1fff);
    }
    static void __iomem *ofdrm_gxt2000_cmap_ioremap(struct ofdrm_device *odev,
    struct device_node *of_node,
    u64 fb_base)
    {
    return get_cmap_address_of(odev, of_node, 0, 0x6000, 0x1000);
    }
    static void ofdrm_gxt2000_cmap_write(struct ofdrm_device *odev, unsigned char index,
    unsigned char r, unsigned char g, unsigned char b)
    {
    void __iomem *data = ((unsigned int __iomem *)odev.cmap_base) + index;
    let mut color: u32 = (r << 16) | (g << 8) | b;
    writel(color, data);
    }
    static void __iomem *ofdrm_avivo_cmap_ioremap(struct ofdrm_device *odev,
    struct device_node *of_node,
    u64 fb_base)
    {
    struct device_node *of_parent;
    void __iomem *cmap_base;
    of_parent = of_get_parent(of_node);
    cmap_base = get_cmap_address_of(odev, of_parent, 0, 0, 0x10000);
    of_node_put(of_parent);
    return cmap_base;
    }
    static void ofdrm_avivo_cmap_write(struct ofdrm_device *odev, unsigned char index,
    unsigned char r, unsigned char g, unsigned char b)
    {
    void __iomem *lutsel = odev.cmap_base + AVIVO_DC_LUT_RW_SELECT;
    void __iomem *addr = odev.cmap_base + AVIVO_DC_LUT_RW_INDEX;
    void __iomem *data = odev.cmap_base + AVIVO_DC_LUT_30_COLOR;
    let mut color: u32 = (r << 22) | (g << 12) | (b << 2);
// Write to both LUTs for now
    writel(1, lutsel);
    writeb(index, addr);
    writel(color, data);
    writel(0, lutsel);
    writeb(index, addr);
    writel(color, data);
    }
    static void __iomem *ofdrm_qemu_cmap_ioremap(struct ofdrm_device *odev,
    struct device_node *of_node,
    u64 fb_base)
    {
    static const __be32 io_of_addr[3] = {
    cpu_to_be32(0x01000000),
    cpu_to_be32(0x00),
    cpu_to_be32(0x00),
    };
    struct drm_device *dev = &odev.sysfb.dev;
    u64 address;
    void __iomem *cmap_base;
    address = of_translate_address(of_node, io_of_addr);
    if (address == OF_BAD_ADDR)
    return IOMEM_ERR_PTR(-ENODEV);
    cmap_base = devm_ioremap(dev.dev, address + 0x3c8, 2);
    if (!cmap_base)
    return IOMEM_ERR_PTR(-ENOMEM);
    return cmap_base;
    }
    static void ofdrm_qemu_cmap_write(struct ofdrm_device *odev, unsigned char index,
    unsigned char r, unsigned char g, unsigned char b)
    {
    void __iomem *addr = odev.cmap_base;
    void __iomem *data = odev.cmap_base + 1;
    writeb(index, addr);
    writeb(r, data);
    writeb(g, data);
    writeb(b, data);
    }
    static void ofdrm_set_gamma_lut(struct drm_crtc *crtc, unsigned int index,
    u16 red, u16 green, u16 blue)
    {
    struct drm_device *dev = crtc.dev;
    struct ofdrm_device *odev = ofdrm_device_of_dev(dev);
    let mut i8: u8 = index & 0xff;
    let mut r8: u8 = red >> 8;
    let mut g8: u8 = green >> 8;
    let mut b8: u8 = blue >> 8;
    if (drm_WARN_ON_ONCE(dev, index != i8))
    return; /* driver bug */
    odev.funcs.cmap_write(odev, i8, r8, g8, b8);
    }
    static void ofdrm_device_fill_gamma(struct ofdrm_device *odev,
    const struct drm_format_info *format)
    {
    struct drm_device *dev = &odev.sysfb.dev;
    struct drm_crtc *crtc = &odev.crtc;
    switch (format.format) {
    case DRM_FORMAT_RGB565:
    case DRM_FORMAT_RGB565 | DRM_FORMAT_BIG_ENDIAN:
    drm_crtc_fill_gamma_565(crtc, ofdrm_set_gamma_lut);
    break;
    case DRM_FORMAT_XRGB8888:
    case DRM_FORMAT_BGRX8888:
    drm_crtc_fill_gamma_888(crtc, ofdrm_set_gamma_lut);
    break;
    default:
    drm_warn_once(dev, "Unsupported format %p4cc for gamma correction\n",
    &format.format);
    break;
    }
    }
    static void ofdrm_device_load_gamma(struct ofdrm_device *odev,
    const struct drm_format_info *format,
    struct drm_color_lut *lut)
    {
    struct drm_device *dev = &odev.sysfb.dev;
    struct drm_crtc *crtc = &odev.crtc;
    switch (format.format) {
    case DRM_FORMAT_RGB565:
    case DRM_FORMAT_RGB565 | DRM_FORMAT_BIG_ENDIAN:
    drm_crtc_load_gamma_565_from_888(crtc, lut, ofdrm_set_gamma_lut);
    break;
    case DRM_FORMAT_XRGB8888:
    case DRM_FORMAT_BGRX8888:
    drm_crtc_load_gamma_888(crtc, lut, ofdrm_set_gamma_lut);
    break;
    default:
    drm_warn_once(dev, "Unsupported format %p4cc for gamma correction\n",
    &format.format);
    break;
    }
    }
//
// Modesetting
//
    static const u64 ofdrm_primary_plane_format_modifiers[] = {
    DRM_SYSFB_PLANE_FORMAT_MODIFIERS,
    };
    static const struct drm_plane_helper_funcs ofdrm_primary_plane_helper_funcs = {
    DRM_SYSFB_PLANE_HELPER_FUNCS,
    };
    static const struct drm_plane_funcs ofdrm_primary_plane_funcs = {
    DRM_SYSFB_PLANE_FUNCS,
    .destroy = drm_plane_cleanup,
    };
#[no_mangle]
unsafe extern "C" fn ofdrm_crtc_helper_atomic_flush(crtc: *mut drm_crtc, state: *mut drm_atomic_commit) {
    static void ofdrm_crtc_helper_atomic_flush(struct drm_crtc *crtc, struct drm_atomic_commit *state)
    {
    struct ofdrm_device *odev = ofdrm_device_of_dev(crtc.dev);
    struct drm_crtc_state *crtc_state = drm_atomic_get_new_crtc_state(state, crtc);
    struct drm_sysfb_crtc_state *sysfb_crtc_state = to_drm_sysfb_crtc_state(crtc_state);
    if (crtc_state.enable && crtc_state.color_mgmt_changed) {
    const struct drm_format_info *format = sysfb_crtc_state.format;
    if (crtc_state.gamma_lut)
    ofdrm_device_load_gamma(odev, format, crtc_state.gamma_lut.data);
    else
    ofdrm_device_fill_gamma(odev, format);
    }
    }
    static const struct drm_crtc_helper_funcs ofdrm_crtc_helper_funcs = {
    DRM_SYSFB_CRTC_HELPER_FUNCS,
    .atomic_flush = ofdrm_crtc_helper_atomic_flush,
    };
    static const struct drm_crtc_funcs ofdrm_crtc_funcs = {
    DRM_SYSFB_CRTC_FUNCS,
    .destroy = drm_crtc_cleanup,
    };
    static const struct drm_encoder_funcs ofdrm_encoder_funcs = {
    .destroy = drm_encoder_cleanup,
    };
    static const struct drm_connector_helper_funcs ofdrm_connector_helper_funcs = {
    DRM_SYSFB_CONNECTOR_HELPER_FUNCS,
    };
    static const struct drm_connector_funcs ofdrm_connector_funcs = {
    DRM_SYSFB_CONNECTOR_FUNCS,
    .destroy = drm_connector_cleanup,
    };
    static const struct drm_mode_config_funcs ofdrm_mode_config_funcs = {
    DRM_SYSFB_MODE_CONFIG_FUNCS,
    };
//
// Init / Cleanup
//
    static const struct ofdrm_device_funcs ofdrm_unknown_device_funcs = {
    };
    static const struct ofdrm_device_funcs ofdrm_mach64_device_funcs = {
    .cmap_ioremap = ofdrm_mach64_cmap_ioremap,
    .cmap_write = ofdrm_mach64_cmap_write,
    };
    static const struct ofdrm_device_funcs ofdrm_rage128_device_funcs = {
    .cmap_ioremap = ofdrm_rage128_cmap_ioremap,
    .cmap_write = ofdrm_rage128_cmap_write,
    };
    static const struct ofdrm_device_funcs ofdrm_rage_m3a_device_funcs = {
    .cmap_ioremap = ofdrm_rage_m3a_cmap_ioremap,
    .cmap_write = ofdrm_rage_m3a_cmap_write,
    };
    static const struct ofdrm_device_funcs ofdrm_rage_m3b_device_funcs = {
    .cmap_ioremap = ofdrm_rage_m3b_cmap_ioremap,
    .cmap_write = ofdrm_rage_m3b_cmap_write,
    };
    static const struct ofdrm_device_funcs ofdrm_radeon_device_funcs = {
    .cmap_ioremap = ofdrm_radeon_cmap_ioremap,
    .cmap_write = ofdrm_rage128_cmap_write, /* same as Rage128 */
    };
    static const struct ofdrm_device_funcs ofdrm_gxt2000_device_funcs = {
    .cmap_ioremap = ofdrm_gxt2000_cmap_ioremap,
    .cmap_write = ofdrm_gxt2000_cmap_write,
    };
    static const struct ofdrm_device_funcs ofdrm_avivo_device_funcs = {
    .cmap_ioremap = ofdrm_avivo_cmap_ioremap,
    .cmap_write = ofdrm_avivo_cmap_write,
    };
    static const struct ofdrm_device_funcs ofdrm_qemu_device_funcs = {
    .cmap_ioremap = ofdrm_qemu_cmap_ioremap,
    .cmap_write = ofdrm_qemu_cmap_write,
    };
    static struct ofdrm_device *ofdrm_device_create(struct drm_driver *drv,
    struct platform_device *pdev)
    {
    struct device_node *of_node = pdev.dev.of_node;
    struct ofdrm_device *odev;
    struct drm_sysfb_device *sysfb;
    struct drm_device *dev;
    enum ofdrm_model model;
    bool big_endian;
    int width, height, depth, linebytes;
    const struct drm_format_info *format;
    u64 address;
    const u8 *edid;
    resource_size_t fb_size, fb_base, fb_pgbase, fb_pgsize;
    struct resource *res, *mem;
    void __iomem *screen_base;
    struct drm_plane *primary_plane;
    struct drm_crtc *crtc;
    struct drm_encoder *encoder;
    struct drm_connector *connector;
    unsigned long max_width, max_height;
    size_t nformats;
    int ret;
    odev = devm_drm_dev_alloc(&pdev.dev, drv, struct ofdrm_device, sysfb.dev);
    if (IS_ERR(odev))
    return ERR_CAST(odev);
    sysfb = &odev.sysfb;
    dev = &sysfb.dev;
    platform_set_drvdata(pdev, dev);
    ret = ofdrm_device_init_pci(odev);
    if (ret)
    return ERR_PTR(ret);
//
// OF display-node settings
//
    model = display_get_model_of(dev, of_node);
    drm_dbg(dev, "detected model %d\n", model);
    switch (model) {
    case OFDRM_MODEL_UNKNOWN:
    odev.funcs = &ofdrm_unknown_device_funcs;
    break;
    case OFDRM_MODEL_MACH64:
    odev.funcs = &ofdrm_mach64_device_funcs;
    break;
    case OFDRM_MODEL_RAGE128:
    odev.funcs = &ofdrm_rage128_device_funcs;
    break;
    case OFDRM_MODEL_RAGE_M3A:
    odev.funcs = &ofdrm_rage_m3a_device_funcs;
    break;
    case OFDRM_MODEL_RAGE_M3B:
    odev.funcs = &ofdrm_rage_m3b_device_funcs;
    break;
    case OFDRM_MODEL_RADEON:
    odev.funcs = &ofdrm_radeon_device_funcs;
    break;
    case OFDRM_MODEL_GXT2000:
    odev.funcs = &ofdrm_gxt2000_device_funcs;
    break;
    case OFDRM_MODEL_AVIVO:
    odev.funcs = &ofdrm_avivo_device_funcs;
    break;
    case OFDRM_MODEL_QEMU:
    odev.funcs = &ofdrm_qemu_device_funcs;
    break;
    }
    big_endian = display_get_big_endian_of(dev, of_node);
    width = display_get_width_of(dev, of_node);
    if (width < 0)
    return ERR_PTR(width);
    height = display_get_height_of(dev, of_node);
    if (height < 0)
    return ERR_PTR(height);
    depth = display_get_depth_of(dev, of_node);
    if (depth < 0)
    return ERR_PTR(depth);
    linebytes = display_get_linebytes_of(dev, of_node);
    if (linebytes < 0)
    return ERR_PTR(linebytes);
    format = display_get_validated_format(dev, depth, big_endian);
    if (IS_ERR(format))
    return ERR_CAST(format);
    if (!linebytes) {
    linebytes = drm_format_info_min_pitch(format, 0, width);
    if (drm_WARN_ON(dev, !linebytes))
    return ERR_PTR(-EINVAL);
    }
    if (check_mul_overflow(linebytes, height, &fb_size)) {
    drm_err(dev, "framebuffer size exceeds maximum\n");
    return ERR_PTR(-EINVAL);
    }
//
// Try to figure out the address of the framebuffer. Unfortunately, Open
// Firmware doesn't provide a standard way to do so. All we can do is a
// dodgy heuristic that happens to work in practice.
//
// On most machines, the "address" property contains what we need, though
// not on Matrox cards found in IBM machines. What appears to give good
// results is to go through the PCI ranges and pick one that encloses the
// "address" property. If none match, we pick the largest.
//
    address = display_get_address_of(dev, of_node);
    if (address != OF_BAD_ADDR) {
    let mut fb_res: resource = DEFINE_RES_MEM(address, fb_size);
    res = ofdrm_find_fb_resource(odev, &fb_res);
    if (!res)
    return ERR_PTR(-EINVAL);
    if (resource_contains(res, &fb_res))
    fb_base = address;
    else
    fb_base = res.start;
    } else {
    let mut fb_res: resource = DEFINE_RES_MEM(0u, fb_size);
    res = ofdrm_find_fb_resource(odev, &fb_res);
    if (!res)
    return ERR_PTR(-EINVAL);
    fb_base = res.start;
    }
//
// I/O resources
//
    fb_pgbase = round_down(fb_base, PAGE_SIZE);
    fb_pgsize = fb_base - fb_pgbase + round_up(fb_size, PAGE_SIZE);
    ret = devm_aperture_acquire_for_platform_device(pdev, fb_pgbase, fb_pgsize);
    if (ret) {
    drm_err(dev, "could not acquire memory range %pr: error %d\n", &res, ret);
    return ERR_PTR(ret);
    }
    mem = devm_request_mem_region(&pdev.dev, fb_pgbase, fb_pgsize, drv.name);
    if (!mem) {
    drm_warn(dev, "could not acquire memory region %pr\n", &res);
    return ERR_PTR(-ENOMEM);
    }
    screen_base = devm_ioremap(&pdev.dev, mem.start, resource_size(mem));
    if (!screen_base)
    return ERR_PTR(-ENOMEM);
    if (odev.funcs.cmap_ioremap) {
    void __iomem *cmap_base = odev.funcs.cmap_ioremap(odev, of_node, fb_base);
    if (IS_ERR(cmap_base)) {
// Don't fail; continue without colormap
    drm_warn(dev, "could not find colormap: error %ld\n", PTR_ERR(cmap_base));
    } else {
    odev.cmap_base = cmap_base;
    }
    }
// EDID is optional
    edid = display_get_edid_of(dev, of_node, odev.edid);
//
// Firmware framebuffer
//
    iosys_map_set_vaddr_iomem(&sysfb.fb_addr, screen_base);
    sysfb.fb_mode = drm_sysfb_mode(width, height, 0, 0);
    sysfb.fb_format = format;
    sysfb.fb_pitch = linebytes;
    if (odev.cmap_base)
    sysfb.fb_gamma_lut_size = OFDRM_GAMMA_LUT_SIZE;
    sysfb.edid = edid;
    drm_dbg(dev, "display mode={" DRM_MODE_FMT "}\n", DRM_MODE_ARG(&sysfb.fb_mode));
    drm_dbg(dev, "framebuffer format=%p4cc, size=%dx%d, linebytes=%d byte\n",
    &format.format, width, height, linebytes);
//
// Mode-setting pipeline
//
    ret = drmm_mode_config_init(dev);
    if (ret)
    return ERR_PTR(ret);
    max_width = max_t(unsigned long, width, DRM_SHADOW_PLANE_MAX_WIDTH);
    max_height = max_t(unsigned long, height, DRM_SHADOW_PLANE_MAX_HEIGHT);
    dev.mode_config.min_width = width;
    dev.mode_config.max_width = max_width;
    dev.mode_config.min_height = height;
    dev.mode_config.max_height = max_height;
    dev.mode_config.funcs = &ofdrm_mode_config_funcs;
    dev.mode_config.preferred_depth = format.depth;
    dev.mode_config.quirk_addfb_prefer_host_byte_order = true;
// Primary plane
    nformats = drm_sysfb_build_fourcc_list(dev, &format.format, 1,
    odev.formats, ARRAY_SIZE(odev.formats));
    primary_plane = &odev.primary_plane;
    ret = drm_universal_plane_init(dev, primary_plane, 0, &ofdrm_primary_plane_funcs,
    odev.formats, nformats,
    ofdrm_primary_plane_format_modifiers,
    DRM_PLANE_TYPE_PRIMARY, core::ptr::null_mut());
    if (ret)
    return ERR_PTR(ret);
    drm_plane_helper_add(primary_plane, &ofdrm_primary_plane_helper_funcs);
    drm_plane_enable_fb_damage_clips(primary_plane);
// CRTC
    crtc = &odev.crtc;
    ret = drm_crtc_init_with_planes(dev, crtc, primary_plane, core::ptr::null_mut(),
    &ofdrm_crtc_funcs, core::ptr::null_mut());
    if (ret)
    return ERR_PTR(ret);
    drm_crtc_helper_add(crtc, &ofdrm_crtc_helper_funcs);
    if (sysfb.fb_gamma_lut_size) {
    ret = drm_mode_crtc_set_gamma_size(crtc, sysfb.fb_gamma_lut_size);
    if (!ret)
    drm_crtc_enable_color_mgmt(crtc, 0, false, sysfb.fb_gamma_lut_size);
    }
// Encoder
    encoder = &odev.encoder;
    ret = drm_encoder_init(dev, encoder, &ofdrm_encoder_funcs, DRM_MODE_ENCODER_NONE, core::ptr::null_mut());
    if (ret)
    return ERR_PTR(ret);
    encoder.possible_crtcs = drm_crtc_mask(crtc);
// Connector
    connector = &odev.connector;
    ret = drm_connector_init(dev, connector, &ofdrm_connector_funcs,
    DRM_MODE_CONNECTOR_Unknown);
    if (ret)
    return ERR_PTR(ret);
    drm_connector_helper_add(connector, &ofdrm_connector_helper_funcs);
    drm_connector_set_panel_orientation_with_quirk(connector,
    DRM_MODE_PANEL_ORIENTATION_UNKNOWN,
    width, height);
    if (edid)
    drm_connector_attach_edid_property(connector);
    ret = drm_connector_attach_encoder(connector, encoder);
    if (ret)
    return ERR_PTR(ret);
    drm_mode_config_reset(dev);
    return odev;
    }
//
// DRM driver
//
    DEFINE_DRM_GEM_FOPS(ofdrm_fops);
    static struct drm_driver ofdrm_driver = {
    DRM_GEM_SHMEM_DRIVER_OPS,
    DRM_FBDEV_SHMEM_DRIVER_OPS,
    .name			= DRIVER_NAME,
    .desc			= DRIVER_DESC,
    .major			= DRIVER_MAJOR,
    .minor			= DRIVER_MINOR,
    .driver_features	= DRIVER_ATOMIC | DRIVER_GEM | DRIVER_MODESET,
    .fops			= &ofdrm_fops,
    };
//
// Platform driver
//
#[no_mangle]
unsafe extern "C" fn ofdrm_pm_suspend(dev: *mut device) -> c_int {
    static int ofdrm_pm_suspend(struct device *dev)
    {
    struct drm_device *drm = dev_get_drvdata(dev);
    return drm_mode_config_helper_suspend(drm);
    }
#[no_mangle]
unsafe extern "C" fn ofdrm_pm_resume(dev: *mut device) -> c_int {
    static int ofdrm_pm_resume(struct device *dev)
    {
    struct drm_device *drm = dev_get_drvdata(dev);
    return drm_mode_config_helper_resume(drm);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ofdrm_pm_ops, ofdrm_pm_suspend, ofdrm_pm_resume);
#[no_mangle]
unsafe extern "C" fn ofdrm_probe(pdev: *mut platform_device) -> c_int {
    static int ofdrm_probe(struct platform_device *pdev)
    {
    struct ofdrm_device *odev;
    struct drm_sysfb_device *sysfb;
    struct drm_device *dev;
    int ret;
    odev = ofdrm_device_create(&ofdrm_driver, pdev);
    if (IS_ERR(odev))
    return PTR_ERR(odev);
    sysfb = &odev.sysfb;
    dev = &sysfb.dev;
    ret = drm_dev_register(dev, 0);
    if (ret)
    return ret;
    drm_client_setup(dev, sysfb.fb_format);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ofdrm_remove(pdev: *mut platform_device) {
    static void ofdrm_remove(struct platform_device *pdev)
    {
    struct drm_device *dev = platform_get_drvdata(pdev);
    drm_dev_unplug(dev);
    }
    static const struct of_device_id ofdrm_of_match_display[] = {
    { .compatible = "display", },
    { },
    };
    MODULE_DEVICE_TABLE(of, ofdrm_of_match_display);
    static struct platform_driver ofdrm_platform_driver = {
    .driver = {
    .name = "of-display",
    .of_match_table = ofdrm_of_match_display,
    .pm = pm_sleep_ptr(&ofdrm_pm_ops),
    },
    .probe = ofdrm_probe,
    .remove = ofdrm_remove,
    };
    module_platform_driver(ofdrm_platform_driver);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
