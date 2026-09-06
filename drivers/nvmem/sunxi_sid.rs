//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/sunxi_sid.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Allwinner sunXi SoCs Security ID support.
//
// Copyright (c) 2013 Oliver Schinagl <oliver@schinagl.nl>
// Copyright (C) 2014 Maxime Ripard <maxime.ripard@free-electrons.com>
//

// Registers and special values for doing register-based SID readout on H3
pub const SUN8I_SID_PRCTL: c_uint = 0x40;
pub const SUN8I_SID_RDKEY: c_uint = 0x60;
pub const SUN8I_SID_OFFSET_MASK: c_uint = 0x1FF;
pub const SUN8I_SID_OFFSET_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_sid_cfg {
    pub value_offset: u32,
    pub size: u32,
    pub need_register_readout: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_sid {
    pub base: *mut void __iomem,
    pub value_offset: u32,
}

    static int sunxi_sid_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct sunxi_sid *sid = context;
    u32 word;
// .stride = 4 so offset is guaranteed to be aligned
    __ioread32_copy(val, sid.base + sid.value_offset + offset, bytes / 4);
    val += round_down(bytes, 4);
    offset += round_down(bytes, 4);
    bytes = bytes % 4;
    if (!bytes)
    return 0;
// Handle any trailing bytes
    word = readl_relaxed(sid.base + sid.value_offset + offset);
    memcpy(val, &word, bytes);
    return 0;
    }
    static int sun8i_sid_register_readout(const struct sunxi_sid *sid,
    const unsigned int offset,
    u32 *out)
    {
    u32 reg_val;
    int ret;
// Set word, lock access, and set read command
    reg_val = (offset & SUN8I_SID_OFFSET_MASK)
    << SUN8I_SID_OFFSET_SHIFT;
    reg_val |= SUN8I_SID_OP_LOCK | SUN8I_SID_READ;
    writel(reg_val, sid.base + SUN8I_SID_PRCTL);
    ret = readl_poll_timeout(sid.base + SUN8I_SID_PRCTL, reg_val,
    !(reg_val & SUN8I_SID_READ), 100, 250000);
    if (ret)
    return ret;
    if (out)
// out = readl(sid->base + SUN8I_SID_RDKEY);
    writel(0, sid.base + SUN8I_SID_PRCTL);
    return 0;
    }
//
// On Allwinner H3, the value on the 0x200 offset of the SID controller seems
// to be not reliable at all.
// Read by the registers instead.
//
    static int sun8i_sid_read_by_reg(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct sunxi_sid *sid = context;
    u32 word;
    int ret;
// .stride = 4 so offset is guaranteed to be aligned
    while (bytes >= 4) {
    ret = sun8i_sid_register_readout(sid, offset, val);
    if (ret)
    return ret;
    val += 4;
    offset += 4;
    bytes -= 4;
    }
    if (!bytes)
    return 0;
// Handle any trailing bytes
    ret = sun8i_sid_register_readout(sid, offset, &word);
    if (ret)
    return ret;
    memcpy(val, &word, bytes);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_sid_probe(pdev: *mut platform_device) -> c_int {
    static int sunxi_sid_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct nvmem_config *nvmem_cfg;
    struct nvmem_device *nvmem;
    struct sunxi_sid *sid;
    int size;
    char *randomness;
    const struct sunxi_sid_cfg *cfg;
    sid = devm_kzalloc(dev, sizeof(*sid), GFP_KERNEL);
    if (!sid)
    return -ENOMEM;
    cfg = of_device_get_match_data(dev);
    if (!cfg)
    return -EINVAL;
    sid.value_offset = cfg.value_offset;
    sid.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(sid.base))
    return PTR_ERR(sid.base);
    size = cfg.size;
    nvmem_cfg = devm_kzalloc(dev, sizeof(*nvmem_cfg), GFP_KERNEL);
    if (!nvmem_cfg)
    return -ENOMEM;
    nvmem_cfg.dev = dev;
    nvmem_cfg.name = "sunxi-sid";
    nvmem_cfg.type = NVMEM_TYPE_OTP;
    nvmem_cfg.add_legacy_fixed_of_cells = true;
    nvmem_cfg.read_only = true;
    nvmem_cfg.size = cfg.size;
    nvmem_cfg.word_size = 1;
    nvmem_cfg.stride = 4;
    nvmem_cfg.priv = sid;
    if (cfg.need_register_readout)
    nvmem_cfg.reg_read = sun8i_sid_read_by_reg;
    else
    nvmem_cfg.reg_read = sunxi_sid_read;
    nvmem = devm_nvmem_register(dev, nvmem_cfg);
    if (IS_ERR(nvmem))
    return PTR_ERR(nvmem);
    randomness = kzalloc(size, GFP_KERNEL);
    if (!randomness)
    return -ENOMEM;
    nvmem_cfg.reg_read(sid, 0, randomness, size);
    add_device_randomness(randomness, size);
    kfree(randomness);
    platform_set_drvdata(pdev, nvmem);
    return 0;
    }
    static const struct sunxi_sid_cfg sun4i_a10_cfg = {
    .size = 0x10,
    };
    static const struct sunxi_sid_cfg sun7i_a20_cfg = {
    .size = 0x200,
    };
    static const struct sunxi_sid_cfg sun8i_h3_cfg = {
    .value_offset = 0x200,
    .size = 0x100,
    .need_register_readout = true,
    };
    static const struct sunxi_sid_cfg sun50i_a64_cfg = {
    .value_offset = 0x200,
    .size = 0x100,
    };
    static const struct sunxi_sid_cfg sun50i_h6_cfg = {
    .value_offset = 0x200,
    .size = 0x200,
    };
    static const struct of_device_id sunxi_sid_of_match[] = {
    { .compatible = "allwinner,sun4i-a10-sid", .data = &sun4i_a10_cfg },
    { .compatible = "allwinner,sun7i-a20-sid", .data = &sun7i_a20_cfg },
    { .compatible = "allwinner,sun8i-a83t-sid", .data = &sun50i_a64_cfg },
    { .compatible = "allwinner,sun8i-h3-sid", .data = &sun8i_h3_cfg },
    { .compatible = "allwinner,sun20i-d1-sid", .data = &sun50i_a64_cfg },
    { .compatible = "allwinner,sun50i-a64-sid", .data = &sun50i_a64_cfg },
    { .compatible = "allwinner,sun50i-h5-sid", .data = &sun50i_a64_cfg },
    { .compatible = "allwinner,sun50i-h6-sid", .data = &sun50i_h6_cfg },
    {/* sentinel */},
    };
    MODULE_DEVICE_TABLE(of, sunxi_sid_of_match);
    static struct platform_driver sunxi_sid_driver = {
    .probe = sunxi_sid_probe,
    .driver = {
    .name = "eeprom-sunxi-sid",
    .of_match_table = sunxi_sid_of_match,
    },
    };
    module_platform_driver(sunxi_sid_driver);
    MODULE_AUTHOR("Oliver Schinagl <oliver@schinagl.nl>");
    MODULE_DESCRIPTION("Allwinner sunxi security id driver");
    MODULE_LICENSE("GPL");
