//! Automatically rewritten from C to Rust
//! Source: drivers/soc/sunxi/sunxi_sram.c
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
// Allwinner SoCs SRAM Controller Driver
//
// Copyright (C) 2015 Maxime Ripard
//
// Author: Maxime Ripard <maxime.ripard@free-electrons.com>
//
// This file is licensed under the terms of the GNU General Public
// License version 2.  This program is licensed "as is" without any
// warranty of any kind, whether express or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_sram_func {
    pub func: *mut c_char,
    pub val: u8,
    pub reg_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_sram_data {
    pub name: *mut c_char,
    pub reg: u8,
    pub offset: u8,
    pub width: u8,
    pub func: *const sunxi_sram_func,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_sram_desc {
    pub data: sunxi_sram_data,
    pub claim_cnt: u8,
}

    {							\
    .func = _func,					\
    .val = _val,					\
    .reg_val = _reg_val,				\
    }

    {							\
    .name = _name,					\
    .reg = _reg,					\
    .offset = _off,					\
    .width = _width,				\
    .func = (const struct sunxi_sram_func[]){	\
    __VA_ARGS__, { } },			\
    }
    static struct sunxi_sram_desc sun4i_a10_sram_a3_a4 = {
    .data	= SUNXI_SRAM_DATA("A3-A4", 0x4, 0x4, 2,
    SUNXI_SRAM_MAP(0, 0, "cpu"),
    SUNXI_SRAM_MAP(1, 1, "emac")),
    };
    static struct sunxi_sram_desc sun4i_a10_sram_c1 = {
    .data	= SUNXI_SRAM_DATA("C1", 0x0, 0x0, 31,
    SUNXI_SRAM_MAP(0, 0, "cpu"),
    SUNXI_SRAM_MAP(0x7fffffff, 1, "ve")),
    };
    static struct sunxi_sram_desc sun4i_a10_sram_d = {
    .data	= SUNXI_SRAM_DATA("D", 0x4, 0x0, 1,
    SUNXI_SRAM_MAP(0, 0, "cpu"),
    SUNXI_SRAM_MAP(1, 1, "usb-otg")),
    };
    static struct sunxi_sram_desc sun50i_a64_sram_c = {
    .data	= SUNXI_SRAM_DATA("C", 0x4, 24, 1,
    SUNXI_SRAM_MAP(1, 0, "cpu"),
    SUNXI_SRAM_MAP(0, 1, "de2")),
    };
    static struct sunxi_sram_desc sun50i_h616_ve_sram = {
    .data	= SUNXI_SRAM_DATA("VE", 0x0, 0, 1,
    SUNXI_SRAM_MAP(1, 0, "cpu"),
    SUNXI_SRAM_MAP(0, 1, "ve")),
    };
    static const struct of_device_id sunxi_sram_dt_ids[] = {
    {
    .compatible	= "allwinner,sun4i-a10-sram-a3-a4",
    .data		= &sun4i_a10_sram_a3_a4.data,
    },
    {
    .compatible	= "allwinner,sun4i-a10-sram-c1",
    .data		= &sun4i_a10_sram_c1.data,
    },
    {
    .compatible	= "allwinner,sun4i-a10-sram-d",
    .data		= &sun4i_a10_sram_d.data,
    },
    {
    .compatible	= "allwinner,sun50i-a64-sram-c",
    .data		= &sun50i_a64_sram_c.data,
    },
    {
    .compatible	= "allwinner,sun50i-h616-ve-sram",
    .data		= &sun50i_h616_ve_sram.data,
    },
    {}
    };
    static struct device *sram_dev;
    static DEFINE_SPINLOCK(sram_lock);
    static void __iomem *base;
#[no_mangle]
unsafe extern "C" fn sunxi_sram_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int sunxi_sram_show(struct seq_file *s, void *data)
    {
    struct device_node *sram_node, *section_node;
    const struct sunxi_sram_data *sram_data;
    const struct of_device_id *match;
    const struct sunxi_sram_func *func;
    const __be32 *sram_addr_p, *section_addr_p;
    u32 val;
    seq_puts(s, "Allwinner sunXi SRAM\n");
    seq_puts(s, "--------------------\n\n");
    for_each_child_of_node(sram_dev.of_node, sram_node) {
    if (!of_device_is_compatible(sram_node, "mmio-sram"))
    continue;
    sram_addr_p = of_get_address(sram_node, 0, core::ptr::null_mut(), core::ptr::null_mut());
    seq_printf(s, "sram@%08x\n",
    be32_to_cpu(*sram_addr_p));
    for_each_child_of_node(sram_node, section_node) {
    match = of_match_node(sunxi_sram_dt_ids, section_node);
    if (!match)
    continue;
    sram_data = match.data;
    section_addr_p = of_get_address(section_node, 0,
    core::ptr::null_mut(), core::ptr::null_mut());
    seq_printf(s, "\tsection@%04x\t(%s)\n",
    be32_to_cpu(*section_addr_p),
    sram_data.name);
    val = readl(base + sram_data.reg);
    val >>= sram_data.offset;
    val &= GENMASK(sram_data.width - 1, 0);
    for (func = sram_data.func; func.func; func++) {
    seq_printf(s, "\t\t%s%c\n", func.func,
    func.reg_val == val ?
    '*' : ' ');
    }
    }
    seq_puts(s, "\n");
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(sunxi_sram);
    static inline struct sunxi_sram_desc *to_sram_desc(const struct sunxi_sram_data *data)
    {
    return container_of(data, struct sunxi_sram_desc, data);
    }
    static const struct sunxi_sram_data *sunxi_sram_get_match(struct device_node *np, u8 val,
    unsigned int *reg_value)
    {
    const struct of_device_id *match;
    const struct sunxi_sram_data *data;
    const struct sunxi_sram_func *func;
    if (!of_device_is_available(np))
    return ERR_PTR(-ENODEV);
    match = of_match_node(sunxi_sram_dt_ids, np);
    if (!match)
    return ERR_PTR(-ENODEV);
    data = match.data;
    if (!data)
    return ERR_PTR(-EINVAL);
    for (func = data.func; func.func; func++)
    if (val == func.val)
    break;
    if (!func.func)
    return ERR_PTR(-EINVAL);
    if (reg_value)
// reg_value = func->reg_val;
    return data;
    }

pub const SUNXI_SRAM_CELLS: c_int = 1;
#[no_mangle]
unsafe extern "C" fn sunxi_sram_claim_one(np: *mut device_node, arg: u8) -> c_int {
    static int sunxi_sram_claim_one(struct device_node *np, u8 arg)
    {
    const struct sunxi_sram_data *sram_data;
    struct sunxi_sram_desc *sram_desc;
    unsigned int device;
    u32 val, mask;
    sram_data = sunxi_sram_get_match(np, arg, &device);
    if (IS_ERR(sram_data))
    return PTR_ERR(sram_data);
    sram_desc = to_sram_desc(sram_data);
    spin_lock(&sram_lock);
    if (sram_desc.claim_cnt) {
    if (!WARN_ON(sram_desc.claim_cnt == U8_MAX))
    sram_desc.claim_cnt++;
    } else {
    mask = GENMASK(sram_data.offset + sram_data.width - 1,
    sram_data.offset);
    val = readl(base + sram_data.reg);
    val &= ~mask;
    writel(val | ((device << sram_data.offset) & mask),
    base + sram_data.reg);
    sram_desc.claim_cnt++;
    }
    spin_unlock(&sram_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_sram_release_one(np: *mut device_node, arg: u8) {
    static void sunxi_sram_release_one(struct device_node *np, u8 arg)
    {
    const struct sunxi_sram_data *sram_data;
    struct sunxi_sram_desc *sram_desc;
    sram_data = sunxi_sram_get_match(np, arg, core::ptr::null_mut());
    if (IS_ERR(sram_data))
    return;
    sram_desc = to_sram_desc(sram_data);
    spin_lock(&sram_lock);
    if (!WARN_ON(sram_desc.claim_cnt == 0))
    sram_desc.claim_cnt--;
    spin_unlock(&sram_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn sunxi_sram_claim(dev: *mut device) -> c_int {
    int sunxi_sram_claim(struct device *dev)
    {
    struct of_phandle_iterator it;
    int err;
    let mut count: c_int = 0;
    if (IS_ERR(base))
    return PTR_ERR(base);
    if (!base)
    return -EPROBE_DEFER;
    if (!dev || !dev.of_node)
    return -EINVAL;
    of_for_each_phandle(&it, err, dev.of_node, SUNXI_SRAM_PROP,
    core::ptr::null_mut(), SUNXI_SRAM_CELLS) {
    u32 args[SUNXI_SRAM_CELLS];
    of_phandle_iterator_args(&it, args, SUNXI_SRAM_CELLS);
    err = sunxi_sram_claim_one(it.node, args[0]);
    if (err)
    goto err;
    count++;
    }
    if (count == 0)
    return -ENOENT;
    return 0;
    err:
    while (count--) {
    struct of_phandle_args args;
    of_parse_phandle_with_fixed_args(dev.of_node, SUNXI_SRAM_PROP,
    SUNXI_SRAM_CELLS, count, &args);
    sunxi_sram_release_one(args.np, args.args[0]);
    }
    return err;
    }
    EXPORT_SYMBOL(sunxi_sram_claim);
#[no_mangle]
pub unsafe extern "C" fn sunxi_sram_release(dev: *mut device) {
    void sunxi_sram_release(struct device *dev)
    {
    struct of_phandle_iterator it;
    int err;
    if (!dev || !dev.of_node)
    return;
    of_for_each_phandle(&it, err, dev.of_node, SUNXI_SRAM_PROP,
    core::ptr::null_mut(), SUNXI_SRAM_CELLS) {
    u32 args[SUNXI_SRAM_CELLS];
    of_phandle_iterator_args(&it, args, SUNXI_SRAM_CELLS);
    sunxi_sram_release_one(it.node, args[0]);
    }
    }
    EXPORT_SYMBOL(sunxi_sram_release);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sunxi_sramc_variant {
    pub num_emac_clocks: c_int,
    pub has_ldo_ctrl: bool,
    pub has_ths_offset: bool,
}

    static const struct sunxi_sramc_variant sun4i_a10_sramc_variant = {
// Nothing special
    };
    static const struct sunxi_sramc_variant sun8i_h3_sramc_variant = {
    .num_emac_clocks = 1,
    };
    static const struct sunxi_sramc_variant sun20i_d1_sramc_variant = {
    .num_emac_clocks = 1,
    .has_ldo_ctrl = true,
    };
    static const struct sunxi_sramc_variant sun50i_a64_sramc_variant = {
    .num_emac_clocks = 1,
    };
    static const struct sunxi_sramc_variant sun50i_h616_sramc_variant = {
    .num_emac_clocks = 2,
    .has_ths_offset = true,
    };
    static const struct sunxi_sramc_variant sun55i_a523_sramc_variant = {
    .num_emac_clocks = 2,
    };
pub const SUNXI_SRAM_THS_OFFSET_REG: c_uint = 0x0;
pub const SUNXI_SRAM_EMAC_CLOCK_REG: c_uint = 0x30;
pub const SUNXI_SYS_LDO_CTRL_REG: c_uint = 0x150;
    static bool sunxi_sram_regmap_accessible_reg(struct device *dev,
    unsigned int reg)
    {
    const struct sunxi_sramc_variant *variant = dev_get_drvdata(dev);
    if (reg == SUNXI_SRAM_THS_OFFSET_REG && variant.has_ths_offset)
    return true;
    if (reg >= SUNXI_SRAM_EMAC_CLOCK_REG &&
    reg <  SUNXI_SRAM_EMAC_CLOCK_REG + variant.num_emac_clocks * 4)
    return true;
    if (reg == SUNXI_SYS_LDO_CTRL_REG && variant.has_ldo_ctrl)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sunxi_sram_lock(_lock: *mut c_void) {
    static void sunxi_sram_lock(void *_lock)
    {
    spinlock_t *lock = _lock;
    spin_lock(lock);
    }
#[no_mangle]
unsafe extern "C" fn sunxi_sram_unlock(_lock: *mut c_void) {
    static void sunxi_sram_unlock(void *_lock)
    {
    spinlock_t *lock = _lock;
    spin_unlock(lock);
    }
    static const struct regmap_config sunxi_sram_regmap_config = {
    .reg_bits       = 32,
    .val_bits       = 32,
    .reg_stride     = 4,
// last defined register
    .max_register   = SUNXI_SYS_LDO_CTRL_REG,
// other devices have no business accessing other registers
    .readable_reg	= sunxi_sram_regmap_accessible_reg,
    .writeable_reg	= sunxi_sram_regmap_accessible_reg,
    .lock		= sunxi_sram_lock,
    .unlock		= sunxi_sram_unlock,
    .lock_arg	= &sram_lock,
    };
#[no_mangle]
unsafe extern "C" fn sunxi_sram_probe(pdev: *mut platform_device) -> int __init {
    static int __init sunxi_sram_probe(struct platform_device *pdev)
    {
    const struct sunxi_sramc_variant *variant;
    struct device *dev = &pdev.dev;
    struct regmap *regmap;
    int ret;
    sram_dev = &pdev.dev;
    variant = of_device_get_match_data(&pdev.dev);
    if (!variant)
    return -EINVAL;
    dev_set_drvdata(dev, (struct sunxi_sramc_variant *)variant);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    if (variant.num_emac_clocks || variant.has_ldo_ctrl) {
    regmap = devm_regmap_init_mmio(dev, base, &sunxi_sram_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    ret = of_syscon_register_regmap(dev.of_node, regmap);
    if (ret)
    return ret;
    }
    of_platform_populate(dev.of_node, core::ptr::null_mut(), core::ptr::null_mut(), dev);
    debugfs_create_file("sram", 0444, core::ptr::null_mut(), core::ptr::null_mut(), &sunxi_sram_fops);
    return 0;
    }
    static const struct of_device_id sunxi_sram_dt_match[] = {
    {
    .compatible = "allwinner,sun4i-a10-sram-controller",
    .data = &sun4i_a10_sramc_variant,
    },
    {
    .compatible = "allwinner,sun4i-a10-system-control",
    .data = &sun4i_a10_sramc_variant,
    },
    {
    .compatible = "allwinner,sun5i-a13-system-control",
    .data = &sun4i_a10_sramc_variant,
    },
    {
    .compatible = "allwinner,sun8i-a23-system-control",
    .data = &sun4i_a10_sramc_variant,
    },
    {
    .compatible = "allwinner,sun8i-h3-system-control",
    .data = &sun8i_h3_sramc_variant,
    },
    {
    .compatible = "allwinner,sun20i-d1-system-control",
    .data = &sun20i_d1_sramc_variant,
    },
    {
    .compatible = "allwinner,sun50i-a64-sram-controller",
    .data = &sun50i_a64_sramc_variant,
    },
    {
    .compatible = "allwinner,sun50i-a64-system-control",
    .data = &sun50i_a64_sramc_variant,
    },
    {
    .compatible = "allwinner,sun50i-h5-system-control",
    .data = &sun50i_a64_sramc_variant,
    },
    {
    .compatible = "allwinner,sun50i-h616-system-control",
    .data = &sun50i_h616_sramc_variant,
    },
    {
    .compatible = "allwinner,sun55i-a523-system-control",
    .data = &sun55i_a523_sramc_variant,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, sunxi_sram_dt_match);
    static struct platform_driver sunxi_sram_driver = {
    .driver = {
    .name		= "sunxi-sram",
    .of_match_table	= sunxi_sram_dt_match,
    },
    };
    builtin_platform_driver_probe(sunxi_sram_driver, sunxi_sram_probe);
    MODULE_AUTHOR("Maxime Ripard <maxime.ripard@free-electrons.com>");
    MODULE_DESCRIPTION("Allwinner sunXi SRAM Controller Driver");
