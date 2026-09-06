//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-uniphier.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_reset_data {
    pub id: c_uint,
    pub reg: c_uint,
    pub bit: c_uint,
    pub flags: c_uint,

}

    { .id = UNIPHIER_RESET_ID_END }

    {						\
    .id = (_id),				\
    .reg = (_reg),				\
    .bit = (_bit),				\
    }

    {						\
    .id = (_id),				\
    .reg = (_reg),				\
    .bit = (_bit),				\
    .flags = UNIPHIER_RESET_ACTIVE_LOW,	\
    }
// System reset data
    static const struct uniphier_reset_data uniphier_ld4_sys_reset_data[] = {
    UNIPHIER_RESETX(2, 0x2000, 2),		/* NAND */
    UNIPHIER_RESETX(8, 0x2000, 10),		/* STDMAC (Ether, HSC, MIO) */
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_pro4_sys_reset_data[] = {
    UNIPHIER_RESETX(2, 0x2000, 2),		/* NAND */
    UNIPHIER_RESETX(6, 0x2000, 12),		/* Ether */
    UNIPHIER_RESETX(8, 0x2000, 10),		/* STDMAC (HSC, MIO, RLE) */
    UNIPHIER_RESETX(12, 0x2000, 6),		/* GIO (Ether, SATA, USB3) */
    UNIPHIER_RESETX(14, 0x2000, 17),	/* USB30 */
    UNIPHIER_RESETX(15, 0x2004, 17),	/* USB31 */
    UNIPHIER_RESETX(28, 0x2000, 18),	/* SATA0 */
    UNIPHIER_RESETX(29, 0x2004, 18),	/* SATA1 */
    UNIPHIER_RESETX(30, 0x2000, 19),	/* SATA-PHY */
    UNIPHIER_RESETX(40, 0x2000, 13),	/* AIO */
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_pro5_sys_reset_data[] = {
    UNIPHIER_RESETX(2, 0x2000, 2),		/* NAND */
    UNIPHIER_RESETX(8, 0x2000, 10),		/* STDMAC (HSC) */
    UNIPHIER_RESETX(12, 0x2000, 6),		/* GIO (PCIe, USB3) */
    UNIPHIER_RESETX(14, 0x2000, 17),	/* USB30 */
    UNIPHIER_RESETX(15, 0x2004, 17),	/* USB31 */
    UNIPHIER_RESETX(24, 0x2008, 2),		/* PCIe */
    UNIPHIER_RESETX(40, 0x2000, 13),	/* AIO */
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_pxs2_sys_reset_data[] = {
    UNIPHIER_RESETX(2, 0x2000, 2),		/* NAND */
    UNIPHIER_RESETX(6, 0x2000, 12),		/* Ether */
    UNIPHIER_RESETX(8, 0x2000, 10),		/* STDMAC (HSC, RLE) */
    UNIPHIER_RESETX(14, 0x2000, 17),	/* USB30 */
    UNIPHIER_RESETX(15, 0x2004, 17),	/* USB31 */
    UNIPHIER_RESETX(16, 0x2014, 4),		/* USB30-PHY0 */
    UNIPHIER_RESETX(17, 0x2014, 0),		/* USB30-PHY1 */
    UNIPHIER_RESETX(18, 0x2014, 2),		/* USB30-PHY2 */
    UNIPHIER_RESETX(20, 0x2014, 5),		/* USB31-PHY0 */
    UNIPHIER_RESETX(21, 0x2014, 1),		/* USB31-PHY1 */
    UNIPHIER_RESETX(28, 0x2014, 12),	/* SATA */
    UNIPHIER_RESET(30, 0x2014, 8),		/* SATA-PHY (active high) */
    UNIPHIER_RESETX(40, 0x2000, 13),	/* AIO */
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_ld11_sys_reset_data[] = {
    UNIPHIER_RESETX(2, 0x200c, 0),		/* NAND */
    UNIPHIER_RESETX(4, 0x200c, 2),		/* eMMC */
    UNIPHIER_RESETX(6, 0x200c, 6),		/* Ether */
    UNIPHIER_RESETX(8, 0x200c, 8),		/* STDMAC (HSC, MIO) */
    UNIPHIER_RESETX(9, 0x200c, 9),		/* HSC */
    UNIPHIER_RESETX(40, 0x2008, 0),		/* AIO */
    UNIPHIER_RESETX(41, 0x2008, 1),		/* EVEA */
    UNIPHIER_RESETX(42, 0x2010, 2),		/* EXIV */
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_ld20_sys_reset_data[] = {
    UNIPHIER_RESETX(2, 0x200c, 0),		/* NAND */
    UNIPHIER_RESETX(4, 0x200c, 2),		/* eMMC */
    UNIPHIER_RESETX(6, 0x200c, 6),		/* Ether */
    UNIPHIER_RESETX(8, 0x200c, 8),		/* STDMAC (HSC) */
    UNIPHIER_RESETX(9, 0x200c, 9),		/* HSC */
    UNIPHIER_RESETX(14, 0x200c, 5),		/* USB30 */
    UNIPHIER_RESETX(16, 0x200c, 12),	/* USB30-PHY0 */
    UNIPHIER_RESETX(17, 0x200c, 13),	/* USB30-PHY1 */
    UNIPHIER_RESETX(18, 0x200c, 14),	/* USB30-PHY2 */
    UNIPHIER_RESETX(19, 0x200c, 15),	/* USB30-PHY3 */
    UNIPHIER_RESETX(24, 0x200c, 4),		/* PCIe */
    UNIPHIER_RESETX(40, 0x2008, 0),		/* AIO */
    UNIPHIER_RESETX(41, 0x2008, 1),		/* EVEA */
    UNIPHIER_RESETX(42, 0x2010, 2),		/* EXIV */
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_pxs3_sys_reset_data[] = {
    UNIPHIER_RESETX(2, 0x200c, 0),		/* NAND */
    UNIPHIER_RESETX(4, 0x200c, 2),		/* eMMC */
    UNIPHIER_RESETX(6, 0x200c, 9),		/* Ether0 */
    UNIPHIER_RESETX(7, 0x200c, 10),		/* Ether1 */
    UNIPHIER_RESETX(8, 0x200c, 12),		/* STDMAC */
    UNIPHIER_RESETX(12, 0x200c, 4),		/* USB30 link */
    UNIPHIER_RESETX(13, 0x200c, 5),		/* USB31 link */
    UNIPHIER_RESETX(16, 0x200c, 16),	/* USB30-PHY0 */
    UNIPHIER_RESETX(17, 0x200c, 18),	/* USB30-PHY1 */
    UNIPHIER_RESETX(18, 0x200c, 20),	/* USB30-PHY2 */
    UNIPHIER_RESETX(20, 0x200c, 17),	/* USB31-PHY0 */
    UNIPHIER_RESETX(21, 0x200c, 19),	/* USB31-PHY1 */
    UNIPHIER_RESETX(24, 0x200c, 3),		/* PCIe */
    UNIPHIER_RESETX(28, 0x200c, 7),		/* SATA0 */
    UNIPHIER_RESETX(29, 0x200c, 8),		/* SATA1 */
    UNIPHIER_RESETX(30, 0x200c, 21),	/* SATA-PHY */
    UNIPHIER_RESETX(40, 0x2008, 0),		/* AIO */
    UNIPHIER_RESETX(42, 0x2010, 2),		/* EXIV */
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_nx1_sys_reset_data[] = {
    UNIPHIER_RESETX(4, 0x2008, 8),		/* eMMC */
    UNIPHIER_RESETX(6, 0x200c, 0),		/* Ether */
    UNIPHIER_RESETX(12, 0x200c, 16),        /* USB30 link */
    UNIPHIER_RESETX(16, 0x200c, 24),        /* USB30-PHY0 */
    UNIPHIER_RESETX(17, 0x200c, 25),        /* USB30-PHY1 */
    UNIPHIER_RESETX(18, 0x200c, 26),        /* USB30-PHY2 */
    UNIPHIER_RESETX(24, 0x200c, 8),         /* PCIe */
    UNIPHIER_RESETX(52, 0x2010, 0),         /* VOC */
    UNIPHIER_RESETX(58, 0x2010, 8),         /* HDMI-Tx */
    UNIPHIER_RESET_END,
    };
// Media I/O reset data

    UNIPHIER_RESETX((id), 0x110 + 0x200 * (ch), 0)

    UNIPHIER_RESETX((id), 0x110 + 0x200 * (ch), 26)

    UNIPHIER_RESETX((id), 0x80 + 0x200 * (ch), 0)

    UNIPHIER_RESETX((id), 0x114 + 0x200 * (ch), 0)

    UNIPHIER_RESETX((id), 0x110 + 0x200 * (ch), 24)

    UNIPHIER_RESETX((id), 0x110, 17)
    static const struct uniphier_reset_data uniphier_ld4_mio_reset_data[] = {
    UNIPHIER_MIO_RESET_SD(0, 0),
    UNIPHIER_MIO_RESET_SD(1, 1),
    UNIPHIER_MIO_RESET_SD(2, 2),
    UNIPHIER_MIO_RESET_SD_BRIDGE(3, 0),
    UNIPHIER_MIO_RESET_SD_BRIDGE(4, 1),
    UNIPHIER_MIO_RESET_SD_BRIDGE(5, 2),
    UNIPHIER_MIO_RESET_EMMC_HW_RESET(6, 1),
    UNIPHIER_MIO_RESET_DMAC(7),
    UNIPHIER_MIO_RESET_USB2(8, 0),
    UNIPHIER_MIO_RESET_USB2(9, 1),
    UNIPHIER_MIO_RESET_USB2(10, 2),
    UNIPHIER_MIO_RESET_USB2_BRIDGE(12, 0),
    UNIPHIER_MIO_RESET_USB2_BRIDGE(13, 1),
    UNIPHIER_MIO_RESET_USB2_BRIDGE(14, 2),
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_pro5_sd_reset_data[] = {
    UNIPHIER_MIO_RESET_SD(0, 0),
    UNIPHIER_MIO_RESET_SD(1, 1),
    UNIPHIER_MIO_RESET_EMMC_HW_RESET(6, 1),
    UNIPHIER_RESET_END,
    };
// Peripheral reset data

    UNIPHIER_RESETX((id), 0x114, 19 + (ch))

    UNIPHIER_RESETX((id), 0x114, 5 + (ch))

    UNIPHIER_RESETX((id), 0x114, 24 + (ch))

    UNIPHIER_RESETX((id), 0x110, 17 + (ch))

    UNIPHIER_RESETX((id), 0x114, 14)
    static const struct uniphier_reset_data uniphier_ld4_peri_reset_data[] = {
    UNIPHIER_PERI_RESET_UART(0, 0),
    UNIPHIER_PERI_RESET_UART(1, 1),
    UNIPHIER_PERI_RESET_UART(2, 2),
    UNIPHIER_PERI_RESET_UART(3, 3),
    UNIPHIER_PERI_RESET_I2C(4, 0),
    UNIPHIER_PERI_RESET_I2C(5, 1),
    UNIPHIER_PERI_RESET_I2C(6, 2),
    UNIPHIER_PERI_RESET_I2C(7, 3),
    UNIPHIER_PERI_RESET_I2C(8, 4),
    UNIPHIER_PERI_RESET_SCSSI(11, 0),
    UNIPHIER_RESET_END,
    };
    static const struct uniphier_reset_data uniphier_pro4_peri_reset_data[] = {
    UNIPHIER_PERI_RESET_UART(0, 0),
    UNIPHIER_PERI_RESET_UART(1, 1),
    UNIPHIER_PERI_RESET_UART(2, 2),
    UNIPHIER_PERI_RESET_UART(3, 3),
    UNIPHIER_PERI_RESET_FI2C(4, 0),
    UNIPHIER_PERI_RESET_FI2C(5, 1),
    UNIPHIER_PERI_RESET_FI2C(6, 2),
    UNIPHIER_PERI_RESET_FI2C(7, 3),
    UNIPHIER_PERI_RESET_FI2C(8, 4),
    UNIPHIER_PERI_RESET_FI2C(9, 5),
    UNIPHIER_PERI_RESET_FI2C(10, 6),
    UNIPHIER_PERI_RESET_SCSSI(11, 0),
    UNIPHIER_PERI_RESET_SCSSI(12, 1),
    UNIPHIER_PERI_RESET_SCSSI(13, 2),
    UNIPHIER_PERI_RESET_SCSSI(14, 3),
    UNIPHIER_PERI_RESET_MCSSI(15),
    UNIPHIER_RESET_END,
    };
// Analog signal amplifiers reset data
    static const struct uniphier_reset_data uniphier_ld11_adamv_reset_data[] = {
    UNIPHIER_RESETX(0, 0x10, 6), /* EVEA */
    UNIPHIER_RESET_END,
    };
// core implementaton
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_reset_priv {
    pub rcdev: reset_controller_dev,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub data: *const uniphier_reset_data,
}

    container_of(_rcdev, struct uniphier_reset_priv, rcdev)
    static int uniphier_reset_update(struct reset_controller_dev *rcdev,
    unsigned long id, int assert)
    {
    struct uniphier_reset_priv *priv = to_uniphier_reset_priv(rcdev);
    const struct uniphier_reset_data *p;
    for (p = priv.data; p.id != UNIPHIER_RESET_ID_END; p++) {
    unsigned int mask, val;
    if (p.id != id)
    continue;
    mask = BIT(p.bit);
    if (assert)
    val = mask;
    else
    val = ~mask;
    if (p.flags & UNIPHIER_RESET_ACTIVE_LOW)
    val = ~val;
    return regmap_write_bits(priv.regmap, p.reg, mask, val);
    }
    dev_err(priv.dev, "reset_id=%lu was not handled\n", id);
    return -EINVAL;
    }
    static int uniphier_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return uniphier_reset_update(rcdev, id, 1);
    }
    static int uniphier_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return uniphier_reset_update(rcdev, id, 0);
    }
    static int uniphier_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct uniphier_reset_priv *priv = to_uniphier_reset_priv(rcdev);
    const struct uniphier_reset_data *p;
    for (p = priv.data; p.id != UNIPHIER_RESET_ID_END; p++) {
    unsigned int val;
    int ret, asserted;
    if (p.id != id)
    continue;
    ret = regmap_read(priv.regmap, p.reg, &val);
    if (ret)
    return ret;
    asserted = !!(val & BIT(p.bit));
    if (p.flags & UNIPHIER_RESET_ACTIVE_LOW)
    asserted = !asserted;
    return asserted;
    }
    dev_err(priv.dev, "reset_id=%lu was not found\n", id);
    return -EINVAL;
    }
    static const struct reset_control_ops uniphier_reset_ops = {
    .assert = uniphier_reset_assert,
    .deassert = uniphier_reset_deassert,
    .status = uniphier_reset_status,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_reset_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_reset_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct uniphier_reset_priv *priv;
    const struct uniphier_reset_data *p, *data;
    struct regmap *regmap;
    struct device_node *parent;
    let mut nr_resets: c_uint = 0;
    data = of_device_get_match_data(dev);
    if (WARN_ON(!data))
    return -EINVAL;
    parent = of_get_parent(dev.of_node); /* parent should be syscon node */
    regmap = syscon_node_to_regmap(parent);
    of_node_put(parent);
    if (IS_ERR(regmap)) {
    dev_err(dev, "failed to get regmap (error %ld)\n",
    PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    for (p = data; p.id != UNIPHIER_RESET_ID_END; p++)
    nr_resets = max(nr_resets, p.id + 1);
    priv.rcdev.ops = &uniphier_reset_ops;
    priv.rcdev.owner = dev.driver.owner;
    priv.rcdev.of_node = dev.of_node;
    priv.rcdev.nr_resets = nr_resets;
    priv.dev = dev;
    priv.regmap = regmap;
    priv.data = data;
    return devm_reset_controller_register(&pdev.dev, &priv.rcdev);
    }
    static const struct of_device_id uniphier_reset_match[] = {
// System reset
    {
    .compatible = "socionext,uniphier-ld4-reset",
    .data = uniphier_ld4_sys_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pro4-reset",
    .data = uniphier_pro4_sys_reset_data,
    },
    {
    .compatible = "socionext,uniphier-sld8-reset",
    .data = uniphier_ld4_sys_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pro5-reset",
    .data = uniphier_pro5_sys_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-reset",
    .data = uniphier_pxs2_sys_reset_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-reset",
    .data = uniphier_ld11_sys_reset_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-reset",
    .data = uniphier_ld20_sys_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-reset",
    .data = uniphier_pxs3_sys_reset_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-reset",
    .data = uniphier_nx1_sys_reset_data,
    },
// Media I/O reset, SD reset
    {
    .compatible = "socionext,uniphier-ld4-mio-reset",
    .data = uniphier_ld4_mio_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pro4-mio-reset",
    .data = uniphier_ld4_mio_reset_data,
    },
    {
    .compatible = "socionext,uniphier-sld8-mio-reset",
    .data = uniphier_ld4_mio_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pro5-sd-reset",
    .data = uniphier_pro5_sd_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-sd-reset",
    .data = uniphier_pro5_sd_reset_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-mio-reset",
    .data = uniphier_ld4_mio_reset_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-sd-reset",
    .data = uniphier_pro5_sd_reset_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-sd-reset",
    .data = uniphier_pro5_sd_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-sd-reset",
    .data = uniphier_pro5_sd_reset_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-sd-reset",
    .data = uniphier_pro5_sd_reset_data,
    },
// Peripheral reset
    {
    .compatible = "socionext,uniphier-ld4-peri-reset",
    .data = uniphier_ld4_peri_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pro4-peri-reset",
    .data = uniphier_pro4_peri_reset_data,
    },
    {
    .compatible = "socionext,uniphier-sld8-peri-reset",
    .data = uniphier_ld4_peri_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pro5-peri-reset",
    .data = uniphier_pro4_peri_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pxs2-peri-reset",
    .data = uniphier_pro4_peri_reset_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-peri-reset",
    .data = uniphier_pro4_peri_reset_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-peri-reset",
    .data = uniphier_pro4_peri_reset_data,
    },
    {
    .compatible = "socionext,uniphier-pxs3-peri-reset",
    .data = uniphier_pro4_peri_reset_data,
    },
    {
    .compatible = "socionext,uniphier-nx1-peri-reset",
    .data = uniphier_pro4_peri_reset_data,
    },
// Analog signal amplifiers reset
    {
    .compatible = "socionext,uniphier-ld11-adamv-reset",
    .data = uniphier_ld11_adamv_reset_data,
    },
    {
    .compatible = "socionext,uniphier-ld20-adamv-reset",
    .data = uniphier_ld11_adamv_reset_data,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_reset_match);
    static struct platform_driver uniphier_reset_driver = {
    .probe = uniphier_reset_probe,
    .driver = {
    .name = "uniphier-reset",
    .of_match_table = uniphier_reset_match,
    },
    };
    module_platform_driver(uniphier_reset_driver);
    MODULE_AUTHOR("Masahiro Yamada <yamada.masahiro@socionext.com>");
    MODULE_DESCRIPTION("UniPhier Reset Controller Driver");
    MODULE_LICENSE("GPL");
