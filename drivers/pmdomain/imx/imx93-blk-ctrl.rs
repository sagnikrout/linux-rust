//! Automatically rewritten from C to Rust
//! Source: drivers/pmdomain/imx/imx93-blk-ctrl.c
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
// Copyright 2022 NXP, Peng Fan <peng.fan@nxp.com>
//

pub const BLK_SFT_RSTN: c_uint = 0x0;
pub const BLK_CLK_EN: c_uint = 0x4;
pub const BLK_MAX_CLKS: c_int = 4;
pub const DOMAIN_MAX_CLKS: c_int = 4;
pub const LCDIF_QOS_REG: c_uint = 0xC;
pub const LCDIF_DEFAULT_QOS_OFF: c_int = 12;
pub const LCDIF_CFG_QOS_OFF: c_int = 8;
pub const PXP_QOS_REG: c_uint = 0x10;
pub const PXP_R_DEFAULT_QOS_OFF: c_int = 28;
pub const PXP_R_CFG_QOS_OFF: c_int = 24;
pub const PXP_W_DEFAULT_QOS_OFF: c_int = 20;
pub const PXP_W_CFG_QOS_OFF: c_int = 16;
pub const ISI_CACHE_REG: c_uint = 0x14;
pub const ISI_QOS_REG: c_uint = 0x1C;
pub const ISI_V_DEFAULT_QOS_OFF: c_int = 28;
pub const ISI_V_CFG_QOS_OFF: c_int = 24;
pub const ISI_U_DEFAULT_QOS_OFF: c_int = 20;
pub const ISI_U_CFG_QOS_OFF: c_int = 16;
pub const ISI_Y_R_DEFAULT_QOS_OFF: c_int = 12;
pub const ISI_Y_R_CFG_QOS_OFF: c_int = 8;
pub const ISI_Y_W_DEFAULT_QOS_OFF: c_int = 4;
pub const ISI_Y_W_CFG_QOS_OFF: c_int = 0;
pub const PRIO_MASK: c_uint = 0xF;

    struct imx93_blk_ctrl_domain;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_blk_ctrl {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub num_clks: c_int,
    pub clks: [clk_bulk_data; BLK_MAX_CLKS],
    pub domains: *mut imx93_blk_ctrl_domain,
    pub onecell_data: genpd_onecell_data,
}

pub const DOMAIN_MAX_QOS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_blk_ctrl_qos {
    pub reg: u32,
    pub cfg_off: u32,
    pub default_prio: u32,
    pub cfg_prio: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_blk_ctrl_subdomain_link {
    pub parent: *mut generic_pm_domain,
    pub subdomain: *mut generic_pm_domain,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_blk_ctrl_domain_data {
    pub name: *const c_char,
    pub clk_names: *const *const c_char,
    pub num_clks: c_int,
    pub rst_mask: u32,
    pub clk_mask: u32,
    pub parent: u32,
    pub num_qos: c_int,
    pub qos: [imx93_blk_ctrl_qos; DOMAIN_MAX_QOS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_blk_ctrl_domain {
    pub genpd: generic_pm_domain,
    pub data: *const imx93_blk_ctrl_domain_data,
    pub clks: [clk_bulk_data; DOMAIN_MAX_CLKS],
    pub bc: *mut imx93_blk_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx93_blk_ctrl_data {
    pub domains: *const imx93_blk_ctrl_domain_data,
    pub skip_mask: u32,
    pub num_domains: c_int,
    pub clk_names: *const *const c_char,
    pub num_clks: c_int,
    pub reg_access_table: *const regmap_access_table,
}

    static inline struct imx93_blk_ctrl_domain *
    to_imx93_blk_ctrl_domain(struct generic_pm_domain *genpd)
    {
    return container_of(genpd, struct imx93_blk_ctrl_domain, genpd);
    }
#[no_mangle]
unsafe extern "C" fn imx93_blk_ctrl_set_qos(domain: *mut imx93_blk_ctrl_domain) -> c_int {
    static int imx93_blk_ctrl_set_qos(struct imx93_blk_ctrl_domain *domain)
    {
    const struct imx93_blk_ctrl_domain_data *data = domain.data;
    struct imx93_blk_ctrl *bc = domain.bc;
    const struct imx93_blk_ctrl_qos *qos;
    u32 val, mask;
    int i;
    for (i = 0; i < data.num_qos; i++) {
    qos = &data.qos[i];
    mask = PRIO_MASK << qos.cfg_off;
    mask |= PRIO_MASK << (qos.cfg_off + 4);
    val = qos.cfg_prio << qos.cfg_off;
    val |= qos.default_prio << (qos.cfg_off + 4);
    regmap_write_bits(bc.regmap, qos.reg, mask, val);
    dev_dbg(bc.dev, "data.qos[i].reg 0x%x 0x%x\n", qos.reg, val);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx93_blk_ctrl_power_on(genpd: *mut generic_pm_domain) -> c_int {
    static int imx93_blk_ctrl_power_on(struct generic_pm_domain *genpd)
    {
    struct imx93_blk_ctrl_domain *domain = to_imx93_blk_ctrl_domain(genpd);
    const struct imx93_blk_ctrl_domain_data *data = domain.data;
    struct imx93_blk_ctrl *bc = domain.bc;
    int ret;
    ret = clk_bulk_prepare_enable(bc.num_clks, bc.clks);
    if (ret) {
    dev_err(bc.dev, "failed to enable bus clocks\n");
    return ret;
    }
    ret = clk_bulk_prepare_enable(data.num_clks, domain.clks);
    if (ret) {
    clk_bulk_disable_unprepare(bc.num_clks, bc.clks);
    dev_err(bc.dev, "failed to enable clocks\n");
    return ret;
    }
    ret = pm_runtime_get_sync(bc.dev);
    if (ret < 0) {
    pm_runtime_put_noidle(bc.dev);
    dev_err(bc.dev, "failed to power up domain\n");
    goto disable_clk;
    }
// ungate clk
    regmap_clear_bits(bc.regmap, BLK_CLK_EN, data.clk_mask);
// release reset
    regmap_set_bits(bc.regmap, BLK_SFT_RSTN, data.rst_mask);
    dev_dbg(bc.dev, "pd_on: name: %s\n", genpd.name);
    return imx93_blk_ctrl_set_qos(domain);
    disable_clk:
    clk_bulk_disable_unprepare(data.num_clks, domain.clks);
    clk_bulk_disable_unprepare(bc.num_clks, bc.clks);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx93_blk_ctrl_power_off(genpd: *mut generic_pm_domain) -> c_int {
    static int imx93_blk_ctrl_power_off(struct generic_pm_domain *genpd)
    {
    struct imx93_blk_ctrl_domain *domain = to_imx93_blk_ctrl_domain(genpd);
    const struct imx93_blk_ctrl_domain_data *data = domain.data;
    struct imx93_blk_ctrl *bc = domain.bc;
    dev_dbg(bc.dev, "pd_off: name: %s\n", genpd.name);
    regmap_clear_bits(bc.regmap, BLK_SFT_RSTN, data.rst_mask);
    regmap_set_bits(bc.regmap, BLK_CLK_EN, data.clk_mask);
    pm_runtime_put(bc.dev);
    clk_bulk_disable_unprepare(data.num_clks, domain.clks);
    clk_bulk_disable_unprepare(bc.num_clks, bc.clks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx93_release_genpd_provider(data: *mut c_void) {
    static void imx93_release_genpd_provider(void *data)
    {
    struct device_node *of_node = data;
    of_genpd_del_provider(of_node);
    }
#[no_mangle]
unsafe extern "C" fn imx93_release_pm_genpd(data: *mut c_void) {
    static void imx93_release_pm_genpd(void *data)
    {
    struct generic_pm_domain *genpd = data;
    pm_genpd_remove(genpd);
    }
#[no_mangle]
unsafe extern "C" fn imx93_release_subdomain(data: *mut c_void) {
    static void imx93_release_subdomain(void *data)
    {
    struct imx93_blk_ctrl_subdomain_link *link = data;
    pm_genpd_remove_subdomain(link.parent, link.subdomain);
    }
    static struct lock_class_key blk_ctrl_genpd_lock_class;
#[no_mangle]
unsafe extern "C" fn imx93_blk_ctrl_probe(pdev: *mut platform_device) -> c_int {
    static int imx93_blk_ctrl_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct imx93_blk_ctrl_data *bc_data = of_device_get_match_data(dev);
    struct imx93_blk_ctrl *bc;
    void __iomem *base;
    int i, ret;
    struct regmap_config regmap_config = {
    .reg_bits	= 32,
    .val_bits	= 32,
    .reg_stride	= 4,
    .rd_table	= bc_data.reg_access_table,
    .wr_table	= bc_data.reg_access_table,
    .max_register   = SZ_4K,
    };
    bc = devm_kzalloc(dev, sizeof(*bc), GFP_KERNEL);
    if (!bc)
    return -ENOMEM;
    bc.dev = dev;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    bc.regmap = devm_regmap_init_mmio(dev, base, &regmap_config);
    if (IS_ERR(bc.regmap))
    return dev_err_probe(dev, PTR_ERR(bc.regmap),
    "failed to init regmap\n");
    bc.domains = devm_kcalloc(dev, bc_data.num_domains,
    sizeof(struct imx93_blk_ctrl_domain),
    GFP_KERNEL);
    if (!bc.domains)
    return -ENOMEM;
    bc.onecell_data.num_domains = bc_data.num_domains;
    bc.onecell_data.domains =
    devm_kcalloc(dev, bc_data.num_domains,
    sizeof(struct generic_pm_domain *), GFP_KERNEL);
    if (!bc.onecell_data.domains)
    return -ENOMEM;
    for (i = 0; i < bc_data.num_clks; i++)
    bc.clks[i].id = bc_data.clk_names[i];
    bc.num_clks = bc_data.num_clks;
    ret = devm_clk_bulk_get(dev, bc.num_clks, bc.clks);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get bus clock\n");
    for (i = 0; i < bc_data.num_domains; i++) {
    const struct imx93_blk_ctrl_domain_data *data = &bc_data.domains[i];
    struct imx93_blk_ctrl_domain *domain = &bc.domains[i];
    int j;
    domain.data = data;
    if (bc_data.skip_mask & BIT(i))
    continue;
    for (j = 0; j < data.num_clks; j++)
    domain.clks[j].id = data.clk_names[j];
    ret = devm_clk_bulk_get(dev, data.num_clks, domain.clks);
    if (ret)
    return dev_err_probe(dev, ret, "failed to get clock\n");
    domain.genpd.name = data.name;
    domain.genpd.power_on = imx93_blk_ctrl_power_on;
    domain.genpd.power_off = imx93_blk_ctrl_power_off;
    domain.bc = bc;
    ret = pm_genpd_init(&domain.genpd, core::ptr::null_mut(), true);
    if (ret)
    return dev_err_probe(dev, ret, "failed to init power domain\n");
    ret = devm_add_action_or_reset(dev, imx93_release_pm_genpd, &domain.genpd);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add pm_genpd release callback\n");
//
// We use runtime PM to trigger power on/off of the upstream GPC
// domain, as a strict hierarchical parent/child power domain
// setup doesn't allow us to meet the sequencing requirements.
// This means we have nested locking of genpd locks, without the
// nesting being visible at the genpd level, so we need a
// separate lock class to make lockdep aware of the fact that
// this are separate domain locks that can be nested without a
// self-deadlock.
//
    lockdep_set_class(&domain.genpd.mlock,
    &blk_ctrl_genpd_lock_class);
    bc.onecell_data.domains[i] = &domain.genpd;
    }
    for (i = 0; i < bc_data.num_domains; i++) {
    struct imx93_blk_ctrl_domain *domain = &bc.domains[i];
    const struct imx93_blk_ctrl_domain_data *data = domain.data;
    struct imx93_blk_ctrl_subdomain_link *link;
    if (bc_data.skip_mask & BIT(i) ||
    data.parent == BLK_CTRL_NO_PARENT)
    continue;
    link = devm_kzalloc(dev, sizeof(*link), GFP_KERNEL);
    if (!link)
    return -ENOMEM;
    link.parent = &bc.domains[data.parent].genpd;
    link.subdomain = &domain.genpd;
    ret = pm_genpd_add_subdomain(&bc.domains[data.parent].genpd,
    &domain.genpd);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add subdomain %s\n",
    domain.genpd.name);
    ret = devm_add_action_or_reset(dev, imx93_release_subdomain, link);
    if (ret)
    return dev_err_probe(dev, ret,
    "failed to add subdomain release callback\n");
    }
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return dev_err_probe(dev, ret, "failed to enable pm-runtime\n");
    ret = of_genpd_add_provider_onecell(dev.of_node, &bc.onecell_data);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add power domain provider\n");
    ret = devm_add_action_or_reset(dev, imx93_release_genpd_provider, dev.of_node);
    if (ret)
    return dev_err_probe(dev, ret, "failed to add genpd_provider release callback\n");
    ret = devm_of_platform_populate(dev);
    if (ret)
    return dev_err_probe(dev, ret, "failed to populate blk-ctrl sub-devices\n");
    return 0;
    }
    static const struct imx93_blk_ctrl_domain_data imx93_media_blk_ctl_domain_data[] = {
    [IMX93_MEDIABLK_PD_MIPI_DSI] = {
    .name = "mediablk-mipi-dsi",
    .clk_names = (const char *[]){ "dsi" },
    .num_clks = 1,
    .rst_mask = BIT(11),
    .clk_mask = BIT(11),
    .parent = IMX93_MEDIABLK_PD_MIPI_PHY,
    },
    [IMX93_MEDIABLK_PD_MIPI_CSI] = {
    .name = "mediablk-mipi-csi",
    .clk_names = (const char *[]){ "cam", "csi" },
    .num_clks = 2,
    .rst_mask = BIT(9) | BIT(10),
    .clk_mask = BIT(9) | BIT(10),
    .parent = IMX93_MEDIABLK_PD_MIPI_PHY,
    },
    [IMX93_MEDIABLK_PD_PXP] = {
    .name = "mediablk-pxp",
    .clk_names = (const char *[]){ "pxp" },
    .num_clks = 1,
    .rst_mask = BIT(7) | BIT(8),
    .clk_mask = BIT(7) | BIT(8),
    .parent = BLK_CTRL_NO_PARENT,
    .num_qos = 2,
    .qos = {
    {
    .reg = PXP_QOS_REG,
    .cfg_off = PXP_R_CFG_QOS_OFF,
    .default_prio = PRIO(3),
    .cfg_prio = PRIO(6),
    }, {
    .reg = PXP_QOS_REG,
    .cfg_off = PXP_W_CFG_QOS_OFF,
    .default_prio = PRIO(3),
    .cfg_prio = PRIO(6),
    }
    }
    },
    [IMX93_MEDIABLK_PD_LCDIF] = {
    .name = "mediablk-lcdif",
    .clk_names = (const char *[]){ "disp", "lcdif" },
    .num_clks = 2,
    .rst_mask = BIT(4) | BIT(5) | BIT(6),
    .clk_mask = BIT(4) | BIT(5) | BIT(6),
    .parent = BLK_CTRL_NO_PARENT,
    .num_qos = 1,
    .qos = {
    {
    .reg = LCDIF_QOS_REG,
    .cfg_off = LCDIF_CFG_QOS_OFF,
    .default_prio = PRIO(3),
    .cfg_prio = PRIO(7),
    }
    }
    },
    [IMX93_MEDIABLK_PD_ISI] = {
    .name = "mediablk-isi",
    .clk_names = (const char *[]){ "isi" },
    .num_clks = 1,
    .rst_mask = BIT(2) | BIT(3),
    .clk_mask = BIT(2) | BIT(3),
    .parent = BLK_CTRL_NO_PARENT,
    .num_qos = 4,
    .qos = {
    {
    .reg = ISI_QOS_REG,
    .cfg_off = ISI_Y_W_CFG_QOS_OFF,
    .default_prio = PRIO(3),
    .cfg_prio = PRIO(7),
    }, {
    .reg = ISI_QOS_REG,
    .cfg_off = ISI_Y_R_CFG_QOS_OFF,
    .default_prio = PRIO(3),
    .cfg_prio = PRIO(7),
    }, {
    .reg = ISI_QOS_REG,
    .cfg_off = ISI_U_CFG_QOS_OFF,
    .default_prio = PRIO(3),
    .cfg_prio = PRIO(7),
    }, {
    .reg = ISI_QOS_REG,
    .cfg_off = ISI_V_CFG_QOS_OFF,
    .default_prio = PRIO(3),
    .cfg_prio = PRIO(7),
    }
    }
    },
    [IMX93_MEDIABLK_PD_MIPI_PHY] = {
    .name = "mediablk-mipi-phy",
    .clk_names = core::ptr::null_mut(),
    .num_clks = 0,
    .rst_mask = BIT(12),
    .clk_mask = BIT(12),
    .parent = BLK_CTRL_NO_PARENT,
    },
    };
    static const struct regmap_range imx93_media_blk_ctl_yes_ranges[] = {
    regmap_reg_range(BLK_SFT_RSTN, BLK_CLK_EN),
    regmap_reg_range(LCDIF_QOS_REG, ISI_CACHE_REG),
    regmap_reg_range(ISI_QOS_REG, ISI_QOS_REG),
    };
    static const struct regmap_access_table imx93_media_blk_ctl_access_table = {
    .yes_ranges = imx93_media_blk_ctl_yes_ranges,
    .n_yes_ranges = ARRAY_SIZE(imx93_media_blk_ctl_yes_ranges),
    };
    static const char * const media_blk_clk_names[] = {
    "axi", "apb", "nic"
    };
    static const struct imx93_blk_ctrl_data imx91_media_blk_ctl_dev_data = {
    .domains = imx93_media_blk_ctl_domain_data,
    .skip_mask = BIT(IMX93_MEDIABLK_PD_MIPI_DSI) | BIT(IMX93_MEDIABLK_PD_PXP),
    .num_domains = ARRAY_SIZE(imx93_media_blk_ctl_domain_data),
    .clk_names = media_blk_clk_names,
    .num_clks = ARRAY_SIZE(media_blk_clk_names),
    .reg_access_table = &imx93_media_blk_ctl_access_table,
    };
    static const struct imx93_blk_ctrl_data imx93_media_blk_ctl_dev_data = {
    .domains = imx93_media_blk_ctl_domain_data,
    .num_domains = ARRAY_SIZE(imx93_media_blk_ctl_domain_data),
    .clk_names = media_blk_clk_names,
    .num_clks = ARRAY_SIZE(media_blk_clk_names),
    .reg_access_table = &imx93_media_blk_ctl_access_table,
    };
    static const struct of_device_id imx93_blk_ctrl_of_match[] = {
    {
    .compatible = "fsl,imx91-media-blk-ctrl",
    .data = &imx91_media_blk_ctl_dev_data
    }, {
    .compatible = "fsl,imx93-media-blk-ctrl",
    .data = &imx93_media_blk_ctl_dev_data
    }, {
// Sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, imx93_blk_ctrl_of_match);
    static struct platform_driver imx93_blk_ctrl_driver = {
    .probe = imx93_blk_ctrl_probe,
    .driver = {
    .name = "imx93-blk-ctrl",
    .of_match_table = imx93_blk_ctrl_of_match,
    },
    };
    module_platform_driver(imx93_blk_ctrl_driver);
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_DESCRIPTION("i.MX93 BLK CTRL driver");
    MODULE_LICENSE("GPL");
