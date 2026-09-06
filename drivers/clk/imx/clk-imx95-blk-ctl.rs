//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-imx95-blk-ctl.c
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
// Copyright 2024-2025 NXP
//

    enum {
    CLK_GATE,
    CLK_DIVIDER,
    CLK_MUX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx95_blk_ctl {
    pub dev: *mut device,
    pub lock: spinlock_t,
    pub clk_apb: *mut clk,
    pub base: *mut void __iomem,
// clock gate register
    pub clk_reg_restore: u32,
    pub pdata: *const imx95_blk_ctl_dev_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx95_blk_ctl_clk_dev_data {
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u32,
    pub reg: u32,
    pub reg_init_msk: u32,
    pub reg_init_val: u32,
    pub bit_idx: u32,
    pub bit_width: u32,
    pub clk_type: u32,
    pub flags: u32,
    pub flags2: u32,
    pub type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx95_blk_ctl_dev_data {
    pub clk_dev_data: *const imx95_blk_ctl_clk_dev_data,
    pub num_clks: u32,
    pub rpm_enabled: bool,
    pub clk_reg_offset: u32,
}

    static const struct imx95_blk_ctl_clk_dev_data vpublk_clk_dev_data[] = {
    [IMX95_CLK_VPUBLK_WAVE] = {
    .name = "vpublk_wave_vpu",
    .parent_names = (const char *[]){ "vpu", },
    .num_parents = 1,
    .reg = 8,
    .bit_idx = 0,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_VPUBLK_JPEG_ENC] = {
    .name = "vpublk_jpeg_enc",
    .parent_names = (const char *[]){ "vpujpeg", },
    .num_parents = 1,
    .reg = 8,
    .bit_idx = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_VPUBLK_JPEG_DEC] = {
    .name = "vpublk_jpeg_dec",
    .parent_names = (const char *[]){ "vpujpeg", },
    .num_parents = 1,
    .reg = 8,
    .bit_idx = 2,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    }
    };
    static const struct imx95_blk_ctl_dev_data vpublk_dev_data = {
    .num_clks = ARRAY_SIZE(vpublk_clk_dev_data),
    .clk_dev_data = vpublk_clk_dev_data,
    .rpm_enabled = true,
    .clk_reg_offset = 8,
    };
    static const struct imx95_blk_ctl_clk_dev_data camblk_clk_dev_data[] = {
    [IMX95_CLK_CAMBLK_CSI2_FOR0] = {
    .name = "camblk_csi2_for0",
    .parent_names = (const char *[]){ "camisi", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 0,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_CAMBLK_CSI2_FOR1] = {
    .name = "camblk_csi2_for1",
    .parent_names = (const char *[]){ "camisi", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_CAMBLK_ISP_AXI] = {
    .name = "camblk_isp_axi",
    .parent_names = (const char *[]){ "camaxi", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 4,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_CAMBLK_ISP_PIXEL] = {
    .name = "camblk_isp_pixel",
    .parent_names = (const char *[]){ "camisi", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 5,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_CAMBLK_ISP] = {
    .name = "camblk_isp",
    .parent_names = (const char *[]){ "camisi", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 6,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    }
    };
    static const struct imx95_blk_ctl_dev_data camblk_dev_data = {
    .num_clks = ARRAY_SIZE(camblk_clk_dev_data),
    .clk_dev_data = camblk_clk_dev_data,
    .clk_reg_offset = 0,
    };
    static const struct imx95_blk_ctl_clk_dev_data imx95_lvds_clk_dev_data[] = {
    [IMX95_CLK_DISPMIX_LVDS_PHY_DIV] = {
    .name = "ldb_phy_div",
    .parent_names = (const char *[]){ "ldbpll", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 0,
    .bit_width = 1,
    .type = CLK_DIVIDER,
    .flags2 = CLK_DIVIDER_POWER_OF_TWO,
    },
    [IMX95_CLK_DISPMIX_LVDS_CH0_GATE] = {
    .name = "lvds_ch0_gate",
    .parent_names = (const char *[]){ "ldb_phy_div", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 1,
    .bit_width = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_DISPMIX_LVDS_CH1_GATE] = {
    .name = "lvds_ch1_gate",
    .parent_names = (const char *[]){ "ldb_phy_div", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 2,
    .bit_width = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_DISPMIX_PIX_DI0_GATE] = {
    .name = "lvds_di0_gate",
    .parent_names = (const char *[]){ "ldb_pll_div7", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 3,
    .bit_width = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    [IMX95_CLK_DISPMIX_PIX_DI1_GATE] = {
    .name = "lvds_di1_gate",
    .parent_names = (const char *[]){ "ldb_pll_div7", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 4,
    .bit_width = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    };
    static const struct imx95_blk_ctl_dev_data imx95_lvds_csr_dev_data = {
    .num_clks = ARRAY_SIZE(imx95_lvds_clk_dev_data),
    .clk_dev_data = imx95_lvds_clk_dev_data,
    .clk_reg_offset = 0,
    };
    static const char * const imx95_disp_engine_parents[] = {
    "videopll1", "dsi_pll", "ldb_pll_div7"
    };
    static const struct imx95_blk_ctl_clk_dev_data imx95_dispmix_csr_clk_dev_data[] = {
    [IMX95_CLK_DISPMIX_ENG0_SEL] = {
    .name = "disp_engine0_sel",
    .parent_names = imx95_disp_engine_parents,
    .num_parents = ARRAY_SIZE(imx95_disp_engine_parents),
    .reg = 0,
    .bit_idx = 0,
    .bit_width = 2,
    .type = CLK_MUX,
    .flags = CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT,
    },
    [IMX95_CLK_DISPMIX_ENG1_SEL] = {
    .name = "disp_engine1_sel",
    .parent_names = imx95_disp_engine_parents,
    .num_parents = ARRAY_SIZE(imx95_disp_engine_parents),
    .reg = 0,
    .bit_idx = 2,
    .bit_width = 2,
    .type = CLK_MUX,
    .flags = CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT,
    }
    };
    static const struct imx95_blk_ctl_dev_data imx95_dispmix_csr_dev_data = {
    .num_clks = ARRAY_SIZE(imx95_dispmix_csr_clk_dev_data),
    .clk_dev_data = imx95_dispmix_csr_clk_dev_data,
    .clk_reg_offset = 0,
    };
    static const struct imx95_blk_ctl_clk_dev_data netxmix_clk_dev_data[] = {
    [IMX95_CLK_NETCMIX_ENETC0_RMII] = {
    .name = "enetc0_rmii_sel",
    .parent_names = (const char *[]){"ext_enetref", "enetref"},
    .num_parents = 2,
    .reg = 4,
    .bit_idx = 5,
    .bit_width = 1,
    .type = CLK_MUX,
    .flags = CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT,
    },
    [IMX95_CLK_NETCMIX_ENETC1_RMII] = {
    .name = "enetc1_rmii_sel",
    .parent_names = (const char *[]){"ext_enetref", "enetref"},
    .num_parents = 2,
    .reg = 4,
    .bit_idx = 10,
    .bit_width = 1,
    .type = CLK_MUX,
    .flags = CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT,
    },
    };
    static const struct imx95_blk_ctl_dev_data netcmix_dev_data = {
    .num_clks = ARRAY_SIZE(netxmix_clk_dev_data),
    .clk_dev_data = netxmix_clk_dev_data,
    .clk_reg_offset = 0,
    };
    static const struct imx95_blk_ctl_clk_dev_data hsio_blk_ctl_clk_dev_data[] = {
    [0] = {
    .name = "hsio_blk_ctl_clk",
    .parent_names = (const char *[]){ "func_out_en", },
    .num_parents = 1,
    .reg = 0,
    .reg_init_msk = GENMASK(10, 7),
    .reg_init_val = GENMASK(10, 7),
    .bit_idx = 6,
    .bit_width = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    },
    [1] = {
    .name = "func_out_en",
    .parent_names = (const char *[]){ "hsio_pll", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 2,
    .bit_width = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    }
    };
    static const struct imx95_blk_ctl_dev_data hsio_blk_ctl_dev_data = {
    .num_clks = ARRAY_SIZE(hsio_blk_ctl_clk_dev_data),
    .clk_dev_data = hsio_blk_ctl_clk_dev_data,
    .clk_reg_offset = 0,
    };
    static const struct imx95_blk_ctl_clk_dev_data imx94_lvds_clk_dev_data[] = {
    [IMX94_CLK_DISPMIX_LVDS_CLK_GATE] = {
    .name = "lvds_clk_gate",
    .parent_names = (const char *[]){ "ldbpll", },
    .num_parents = 1,
    .reg = 0,
    .bit_idx = 1,
    .bit_width = 1,
    .type = CLK_GATE,
    .flags = CLK_SET_RATE_PARENT,
    .flags2 = CLK_GATE_SET_TO_DISABLE,
    },
    };
    static const struct imx95_blk_ctl_dev_data imx94_lvds_csr_dev_data = {
    .num_clks = ARRAY_SIZE(imx94_lvds_clk_dev_data),
    .clk_dev_data = imx94_lvds_clk_dev_data,
    .clk_reg_offset = 0,
    .rpm_enabled = true,
    };
    static const char * const imx94_disp_engine_parents[] = {
    "disppix", "ldb_pll_div7"
    };
    static const struct imx95_blk_ctl_clk_dev_data imx94_dispmix_csr_clk_dev_data[] = {
    [IMX94_CLK_DISPMIX_CLK_SEL] = {
    .name = "disp_clk_sel",
    .parent_names = imx94_disp_engine_parents,
    .num_parents = ARRAY_SIZE(imx94_disp_engine_parents),
    .reg = 0,
    .bit_idx = 1,
    .bit_width = 1,
    .type = CLK_MUX,
    .flags = CLK_SET_RATE_NO_REPARENT | CLK_SET_RATE_PARENT,
    },
    };
    static const struct imx95_blk_ctl_dev_data imx94_dispmix_csr_dev_data = {
    .num_clks = ARRAY_SIZE(imx94_dispmix_csr_clk_dev_data),
    .clk_dev_data = imx94_dispmix_csr_clk_dev_data,
    .clk_reg_offset = 0,
    .rpm_enabled = true,
    };
#[no_mangle]
unsafe extern "C" fn imx95_bc_probe(pdev: *mut platform_device) -> c_int {
    static int imx95_bc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct imx95_blk_ctl *bc;
    struct clk_hw_onecell_data *clk_hw_data;
    struct clk_hw **hws;
    void __iomem *base;
    int i, ret;
    bc = devm_kzalloc(dev, sizeof(*bc), GFP_KERNEL);
    if (!bc)
    return -ENOMEM;
    bc.dev = dev;
    dev_set_drvdata(&pdev.dev, bc);
    spin_lock_init(&bc.lock);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    bc.base = base;
    bc.clk_apb = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(bc.clk_apb))
    return dev_err_probe(dev, PTR_ERR(bc.clk_apb), "failed to get APB clock\n");
    ret = clk_prepare_enable(bc.clk_apb);
    if (ret) {
    dev_err(dev, "failed to enable apb clock: %d\n", ret);
    return ret;
    }
    bc.pdata = of_device_get_match_data(dev);
    if (!bc.pdata)
    return devm_of_platform_populate(dev);
    clk_hw_data = devm_kzalloc(dev, struct_size(clk_hw_data, hws, bc.pdata.num_clks),
    GFP_KERNEL);
    if (!clk_hw_data)
    return -ENOMEM;
    if (bc.pdata.rpm_enabled) {
    devm_pm_runtime_enable(&pdev.dev);
    pm_runtime_resume_and_get(&pdev.dev);
    }
    clk_hw_data.num = bc.pdata.num_clks;
    hws = clk_hw_data.hws;
    for (i = 0; i < bc.pdata.num_clks; i++) {
    const struct imx95_blk_ctl_clk_dev_data *data = &bc.pdata.clk_dev_data[i];
    void __iomem *reg = base + data.reg;
    if (data.reg_init_msk)
    writel((readl(reg) & ~data.reg_init_msk) | data.reg_init_val, reg);
    if (data.type == CLK_MUX) {
    hws[i] = clk_hw_register_mux(dev, data.name, data.parent_names,
    data.num_parents, data.flags, reg,
    data.bit_idx, data.bit_width,
    data.flags2, &bc.lock);
    } else if (data.type == CLK_DIVIDER) {
    hws[i] = clk_hw_register_divider(dev, data.name, data.parent_names[0],
    data.flags, reg, data.bit_idx,
    data.bit_width, data.flags2, &bc.lock);
    } else {
    hws[i] = clk_hw_register_gate(dev, data.name, data.parent_names[0],
    data.flags, reg, data.bit_idx,
    data.flags2, &bc.lock);
    }
    if (IS_ERR(hws[i])) {
    ret = PTR_ERR(hws[i]);
    dev_err(dev, "failed to register: %s:%d\n", data.name, ret);
    goto cleanup;
    }
    }
    ret = of_clk_add_hw_provider(dev.of_node, of_clk_hw_onecell_get, clk_hw_data);
    if (ret)
    goto cleanup;
    ret = devm_of_platform_populate(dev);
    if (ret) {
    of_clk_del_provider(dev.of_node);
    goto cleanup;
    }
    if (pm_runtime_enabled(bc.dev)) {
    pm_runtime_put_sync(&pdev.dev);
    clk_disable_unprepare(bc.clk_apb);
    }
    return 0;
    cleanup:
    for (i = 0; i < bc.pdata.num_clks; i++) {
    if (IS_ERR_OR_NULL(hws[i]))
    continue;
    clk_hw_unregister(hws[i]);
    }
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn imx95_bc_runtime_suspend(dev: *mut device) -> c_int {
    static int imx95_bc_runtime_suspend(struct device *dev)
    {
    struct imx95_blk_ctl *bc = dev_get_drvdata(dev);
    bc.clk_reg_restore = readl(bc.base + bc.pdata.clk_reg_offset);
    clk_disable_unprepare(bc.clk_apb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx95_bc_runtime_resume(dev: *mut device) -> c_int {
    static int imx95_bc_runtime_resume(struct device *dev)
    {
    struct imx95_blk_ctl *bc = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(bc.clk_apb);
    if (ret)
    return ret;
    writel(bc.clk_reg_restore, bc.base + bc.pdata.clk_reg_offset);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn imx95_bc_suspend(dev: *mut device) -> c_int {
    static int imx95_bc_suspend(struct device *dev)
    {
    struct imx95_blk_ctl *bc = dev_get_drvdata(dev);
    if (pm_runtime_suspended(dev))
    return 0;
    bc.clk_reg_restore = readl(bc.base + bc.pdata.clk_reg_offset);
    clk_disable_unprepare(bc.clk_apb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx95_bc_resume(dev: *mut device) -> c_int {
    static int imx95_bc_resume(struct device *dev)
    {
    struct imx95_blk_ctl *bc = dev_get_drvdata(dev);
    int ret;
    if (pm_runtime_suspended(dev))
    return 0;
    ret = clk_prepare_enable(bc.clk_apb);
    if (ret)
    return ret;
    writel(bc.clk_reg_restore, bc.base + bc.pdata.clk_reg_offset);
    return 0;
    }

    static const struct dev_pm_ops imx95_bc_pm_ops = {
    SET_RUNTIME_PM_OPS(imx95_bc_runtime_suspend, imx95_bc_runtime_resume, core::ptr::null_mut())
    SET_SYSTEM_SLEEP_PM_OPS(imx95_bc_suspend, imx95_bc_resume)
    };
    static const struct of_device_id imx95_bc_of_match[] = {
    { .compatible = "nxp,imx94-display-csr", .data = &imx94_dispmix_csr_dev_data },
    { .compatible = "nxp,imx94-lvds-csr", .data = &imx94_lvds_csr_dev_data },
    { .compatible = "nxp,imx95-camera-csr", .data = &camblk_dev_data },
    { .compatible = "nxp,imx95-display-master-csr", },
    { .compatible = "nxp,imx95-display-csr", .data = &imx95_dispmix_csr_dev_data },
    { .compatible = "nxp,imx95-lvds-csr", .data = &imx95_lvds_csr_dev_data },
    { .compatible = "nxp,imx95-hsio-blk-ctl", .data = &hsio_blk_ctl_dev_data },
    { .compatible = "nxp,imx95-vpu-csr", .data = &vpublk_dev_data },
    { .compatible = "nxp,imx95-netcmix-blk-ctrl", .data = &netcmix_dev_data},
    { /* Sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, imx95_bc_of_match);
    static struct platform_driver imx95_bc_driver = {
    .probe = imx95_bc_probe,
    .driver = {
    .name = "imx95-blk-ctl",
    .of_match_table = imx95_bc_of_match,
    .pm = &imx95_bc_pm_ops,
    },
    };
    module_platform_driver(imx95_bc_driver);
    MODULE_DESCRIPTION("NXP i.MX95 blk ctl driver");
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_LICENSE("GPL");
