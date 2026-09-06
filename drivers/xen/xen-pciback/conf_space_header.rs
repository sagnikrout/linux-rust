//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xen-pciback/conf_space_header.c
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
// PCI Backend - Handles the virtual fields in the configuration space headers.
//
// Author: Ryan Wilson <hap9@epoch.ncsc.mil>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_cmd_info {
    pub val: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bar_info {
    pub val: u32,
    pub len_val: u32,
    pub which: c_int,
}

// Bits guests are allowed to control in permissive mode.

    PCI_COMMAND_INVALIDATE|PCI_COMMAND_VGA_PALETTE| \
    PCI_COMMAND_WAIT|PCI_COMMAND_FAST_BACK)
    static void *command_init(struct pci_dev *dev, int offset)
    {
    struct pci_cmd_info *cmd = kmalloc_obj(*cmd);
    int err;
    if (!cmd)
    return ERR_PTR(-ENOMEM);
    err = pci_read_config_word(dev, PCI_COMMAND, &cmd.val);
    if (err) {
    kfree(cmd);
    return ERR_PTR(err);
    }
    return cmd;
    }
#[no_mangle]
unsafe extern "C" fn command_read(dev: *mut pci_dev, offset: c_int, value: *mut u16, data: *mut c_void) -> c_int {
    static int command_read(struct pci_dev *dev, int offset, u16 *value, void *data)
    {
    let mut ret: c_int = pci_read_config_word(dev, offset, value);
    const struct pci_cmd_info *cmd = data;
// value &= PCI_COMMAND_GUEST;
// value |= cmd->val & ~PCI_COMMAND_GUEST;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn command_write(dev: *mut pci_dev, offset: c_int, value: u16, data: *mut c_void) -> c_int {
    static int command_write(struct pci_dev *dev, int offset, u16 value, void *data)
    {
    struct xen_pcibk_dev_data *dev_data;
    int err;
    u16 val;
    struct pci_cmd_info *cmd = data;
    dev_data = pci_get_drvdata(dev);
    if (!pci_is_enabled(dev) && is_enable_cmd(value)) {
    dev_dbg(&dev.dev, "enable\n");
    err = pci_enable_device(dev);
    if (err)
    return err;
    if (dev_data)
    dev_data.enable_intx = 1;
    } else if (pci_is_enabled(dev) && !is_enable_cmd(value)) {
    dev_dbg(&dev.dev, "disable\n");
    pci_disable_device(dev);
    if (dev_data)
    dev_data.enable_intx = 0;
    }
    if (!dev.is_busmaster && is_master_cmd(value)) {
    dev_dbg(&dev.dev, "set bus master\n");
    pci_set_master(dev);
    } else if (dev.is_busmaster && !is_master_cmd(value)) {
    dev_dbg(&dev.dev, "clear bus master\n");
    pci_clear_master(dev);
    }
    if (!(cmd.val & PCI_COMMAND_INVALIDATE) &&
    (value & PCI_COMMAND_INVALIDATE)) {
    dev_dbg(&dev.dev, "enable memory-write-invalidate\n");
    err = pci_set_mwi(dev);
    if (err) {
    dev_warn(&dev.dev, "cannot enable memory-write-invalidate (%d)\n",
    err);
    value &= ~PCI_COMMAND_INVALIDATE;
    }
    } else if ((cmd.val & PCI_COMMAND_INVALIDATE) &&
    !(value & PCI_COMMAND_INVALIDATE)) {
    dev_dbg(&dev.dev, "disable memory-write-invalidate\n");
    pci_clear_mwi(dev);
    }
    if (dev_data && dev_data.allow_interrupt_control &&
    ((cmd.val ^ value) & PCI_COMMAND_INTX_DISABLE))
    pci_intx(dev, !(value & PCI_COMMAND_INTX_DISABLE));
    cmd.val = value;
    if (!xen_pcibk_permissive && (!dev_data || !dev_data.permissive))
    return 0;
// Only allow the guest to control certain bits.
    err = pci_read_config_word(dev, offset, &val);
    if (err || val == value)
    return err;
    value &= PCI_COMMAND_GUEST;
    value |= val & ~PCI_COMMAND_GUEST;
    return pci_write_config_word(dev, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn rom_write(dev: *mut pci_dev, offset: c_int, value: u32, data: *mut c_void) -> c_int {
    static int rom_write(struct pci_dev *dev, int offset, u32 value, void *data)
    {
    struct pci_bar_info *bar = data;
    if (unlikely(!bar)) {
    dev_warn(&dev.dev, "driver data not found\n");
    return XEN_PCI_ERR_op_failed;
    }
// A write to obtain the length must happen as a 32-bit write.
// This does not (yet) support writing individual bytes
//
    if ((value | ~PCI_ROM_ADDRESS_MASK) == ~0U)
    bar.which = 1;
    else {
    u32 tmpval;
    pci_read_config_dword(dev, offset, &tmpval);
    if (tmpval != bar.val && value == bar.val) {
// Allow restoration of bar value.
    pci_write_config_dword(dev, offset, bar.val);
    }
    bar.which = 0;
    }
// Do we need to support enabling/disabling the rom address here?
    return 0;
    }
// For the BARs, only allow writes which write ~0 or
// the correct resource information
// (Needed for when the driver probes the resource usage)
//
#[no_mangle]
unsafe extern "C" fn bar_write(dev: *mut pci_dev, offset: c_int, value: u32, data: *mut c_void) -> c_int {
    static int bar_write(struct pci_dev *dev, int offset, u32 value, void *data)
    {
    struct pci_bar_info *bar = data;
    let mut pos: c_uint = (offset - PCI_BASE_ADDRESS_0) / 4;
    const struct resource *res = dev.resource;
    u32 mask;
    if (unlikely(!bar)) {
    dev_warn(&dev.dev, "driver data not found\n");
    return XEN_PCI_ERR_op_failed;
    }
// A write to obtain the length must happen as a 32-bit write.
// This does not (yet) support writing individual bytes
//
    if (res[pos].flags & IORESOURCE_IO)
    mask = ~PCI_BASE_ADDRESS_IO_MASK;
#[no_mangle]
pub unsafe extern "C" fn if(IORESOURCE_MEM_64): pos && (res[pos - 1].flags &) -> else {
    else if (pos && (res[pos - 1].flags & IORESOURCE_MEM_64))
    mask = 0;
    else
    mask = ~PCI_BASE_ADDRESS_MEM_MASK;
    if ((value | mask) == ~0U)
    bar.which = 1;
    else {
    u32 tmpval;
    pci_read_config_dword(dev, offset, &tmpval);
    if (tmpval != bar.val && value == bar.val) {
// Allow restoration of bar value.
    pci_write_config_dword(dev, offset, bar.val);
    }
    bar.which = 0;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bar_read(dev: *mut pci_dev, offset: c_int, value: *mut *mut u32, data: *mut c_void) -> c_int {
    static int bar_read(struct pci_dev *dev, int offset, u32 * value, void *data)
    {
    struct pci_bar_info *bar = data;
    if (unlikely(!bar)) {
    dev_warn(&dev.dev, "driver data not found\n");
    return XEN_PCI_ERR_op_failed;
    }
// value = bar->which ? bar->len_val : bar->val;
    return 0;
    }
    static void *bar_init(struct pci_dev *dev, int offset)
    {
    unsigned int pos;
    const struct resource *res = dev.resource;
    struct pci_bar_info *bar = kzalloc_obj(*bar);
    if (!bar)
    return ERR_PTR(-ENOMEM);
    if (offset == PCI_ROM_ADDRESS || offset == PCI_ROM_ADDRESS1)
    pos = PCI_ROM_RESOURCE;
    else {
    pos = (offset - PCI_BASE_ADDRESS_0) / 4;
    if (pos && (res[pos - 1].flags & IORESOURCE_MEM_64)) {
//
// Use ">> 16 >> 16" instead of direct ">> 32" shift
// to avoid warnings on 32-bit architectures.
//
    bar.val = res[pos - 1].start >> 16 >> 16;
    bar.len_val = -resource_size(&res[pos - 1]) >> 16 >> 16;
    return bar;
    }
    }
    if (!res[pos].flags ||
    (res[pos].flags & (IORESOURCE_DISABLED | IORESOURCE_UNSET |
    IORESOURCE_BUSY)))
    return bar;
    bar.val = res[pos].start |
    (res[pos].flags & PCI_REGION_FLAG_MASK);
    bar.len_val = -resource_size(&res[pos]) |
    (res[pos].flags & PCI_REGION_FLAG_MASK);
    return bar;
    }
#[no_mangle]
unsafe extern "C" fn bar_reset(dev: *mut pci_dev, offset: c_int, data: *mut c_void) {
    static void bar_reset(struct pci_dev *dev, int offset, void *data)
    {
    struct pci_bar_info *bar = data;
    bar.which = 0;
    }
#[no_mangle]
unsafe extern "C" fn bar_release(dev: *mut pci_dev, offset: c_int, data: *mut c_void) {
    static void bar_release(struct pci_dev *dev, int offset, void *data)
    {
    kfree(data);
    }
    static int xen_pcibk_read_vendor(struct pci_dev *dev, int offset,
    u16 *value, void *data)
    {
// value = dev->vendor;
    return 0;
    }
    static int xen_pcibk_read_device(struct pci_dev *dev, int offset,
    u16 *value, void *data)
    {
// value = dev->device;
    return 0;
    }
    static int interrupt_read(struct pci_dev *dev, int offset, u8 * value,
    void *data)
    {
// value = (u8) dev->irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bist_write(dev: *mut pci_dev, offset: c_int, value: u8, data: *mut c_void) -> c_int {
    static int bist_write(struct pci_dev *dev, int offset, u8 value, void *data)
    {
    u8 cur_value;
    int err;
    err = pci_read_config_byte(dev, offset, &cur_value);
    if (err)
    goto out;
    if ((cur_value & ~PCI_BIST_START) == (value & ~PCI_BIST_START)
    || value == PCI_BIST_START)
    err = pci_write_config_byte(dev, offset, value);
    out:
    return err;
    }
    static const struct config_field header_common[] = {
    {
    .offset    = PCI_VENDOR_ID,
    .size      = 2,
    .u.w.read  = xen_pcibk_read_vendor,
    },
    {
    .offset    = PCI_DEVICE_ID,
    .size      = 2,
    .u.w.read  = xen_pcibk_read_device,
    },
    {
    .offset    = PCI_COMMAND,
    .size      = 2,
    .init      = command_init,
    .release   = bar_release,
    .u.w.read  = command_read,
    .u.w.write = command_write,
    },
    {
    .offset    = PCI_INTERRUPT_LINE,
    .size      = 1,
    .u.b.read  = interrupt_read,
    },
    {
    .offset    = PCI_INTERRUPT_PIN,
    .size      = 1,
    .u.b.read  = xen_pcibk_read_config_byte,
    },
    {
// Any side effects of letting driver domain control cache line?
    .offset    = PCI_CACHE_LINE_SIZE,
    .size      = 1,
    .u.b.read  = xen_pcibk_read_config_byte,
    .u.b.write = xen_pcibk_write_config_byte,
    },
    {
    .offset    = PCI_LATENCY_TIMER,
    .size      = 1,
    .u.b.read  = xen_pcibk_read_config_byte,
    },
    {
    .offset    = PCI_BIST,
    .size      = 1,
    .u.b.read  = xen_pcibk_read_config_byte,
    .u.b.write = bist_write,
    },
    {}
    };

    {						\
    .offset     = reg_offset,			\
    .size       = 4,				\
    .init       = bar_init,				\
    .reset      = bar_reset,			\
    .release    = bar_release,			\
    .u.dw.read  = bar_read,				\
    .u.dw.write = bar_write,			\
    }

    {						\
    .offset     = reg_offset,			\
    .size       = 4,				\
    .init       = bar_init,				\
    .reset      = bar_reset,			\
    .release    = bar_release,			\
    .u.dw.read  = bar_read,				\
    .u.dw.write = rom_write,			\
    }
    static const struct config_field header_0[] = {
    CFG_FIELD_BAR(PCI_BASE_ADDRESS_0),
    CFG_FIELD_BAR(PCI_BASE_ADDRESS_1),
    CFG_FIELD_BAR(PCI_BASE_ADDRESS_2),
    CFG_FIELD_BAR(PCI_BASE_ADDRESS_3),
    CFG_FIELD_BAR(PCI_BASE_ADDRESS_4),
    CFG_FIELD_BAR(PCI_BASE_ADDRESS_5),
    CFG_FIELD_ROM(PCI_ROM_ADDRESS),
    {}
    };
    static const struct config_field header_1[] = {
    CFG_FIELD_BAR(PCI_BASE_ADDRESS_0),
    CFG_FIELD_BAR(PCI_BASE_ADDRESS_1),
    CFG_FIELD_ROM(PCI_ROM_ADDRESS1),
    {}
    };
#[no_mangle]
pub unsafe extern "C" fn xen_pcibk_config_header_add_fields(dev: *mut pci_dev) -> c_int {
    int xen_pcibk_config_header_add_fields(struct pci_dev *dev)
    {
    int err;
    err = xen_pcibk_config_add_fields(dev, header_common);
    if (err)
    goto out;
    switch (dev.hdr_type) {
    case PCI_HEADER_TYPE_NORMAL:
    err = xen_pcibk_config_add_fields(dev, header_0);
    break;
    case PCI_HEADER_TYPE_BRIDGE:
    err = xen_pcibk_config_add_fields(dev, header_1);
    break;
    default:
    err = -EINVAL;
    dev_err(&dev.dev, "Unsupported header type %d!\n",
    dev.hdr_type);
    break;
    }
    out:
    return err;
    }
