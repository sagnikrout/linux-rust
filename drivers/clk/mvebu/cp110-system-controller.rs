//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/cp110-system-controller.c
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
// Marvell Armada CP110 System Controller
//
// Copyright (C) 2016 Marvell
//
// Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
//
// CP110 has 6 core clocks:
//
// - PLL0		(1 Ghz)
// - PPv2 core	(1/3 PLL0)
// - x2 Core		(1/2 PLL0)
// - Core		(1/2 x2 Core)
// - SDIO		(2/5 PLL0)
//
// - NAND clock, which is either:
// - Equal to SDIO clock
// - 2/5 PLL0
//
// CP110 has 32 gateable clocks, for the various peripherals in the IP.
//

pub const CP110_PM_CLOCK_GATING_REG: c_uint = 0x220;
pub const CP110_NAND_FLASH_CLK_CTRL_REG: c_uint = 0x700;

    enum {
    CP110_CLK_TYPE_CORE,
    CP110_CLK_TYPE_GATABLE,
    };
pub const CP110_MAX_CORE_CLOCKS: c_int = 6;
pub const CP110_MAX_GATABLE_CLOCKS: c_int = 32;

    (CP110_MAX_CORE_CLOCKS + CP110_MAX_GATABLE_CLOCKS)
pub const CP110_CORE_PLL0: c_int = 0;
pub const CP110_CORE_PPV2: c_int = 1;
pub const CP110_CORE_X2CORE: c_int = 2;
pub const CP110_CORE_CORE: c_int = 3;
pub const CP110_CORE_NAND: c_int = 4;
pub const CP110_CORE_SDIO: c_int = 5;
// A number of gateable clocks need special handling
pub const CP110_GATE_AUDIO: c_int = 0;
pub const CP110_GATE_COMM_UNIT: c_int = 1;
pub const CP110_GATE_NAND: c_int = 2;
pub const CP110_GATE_PPV2: c_int = 3;
pub const CP110_GATE_SDIO: c_int = 4;
pub const CP110_GATE_MG: c_int = 5;
pub const CP110_GATE_MG_CORE: c_int = 6;
pub const CP110_GATE_XOR1: c_int = 7;
pub const CP110_GATE_XOR0: c_int = 8;
pub const CP110_GATE_GOP_DP: c_int = 9;
pub const CP110_GATE_PCIE_X1_0: c_int = 11;
pub const CP110_GATE_PCIE_X1_1: c_int = 12;
pub const CP110_GATE_PCIE_X4: c_int = 13;
pub const CP110_GATE_PCIE_XOR: c_int = 14;
pub const CP110_GATE_SATA: c_int = 15;
pub const CP110_GATE_SATA_USB: c_int = 16;
pub const CP110_GATE_MAIN: c_int = 17;
pub const CP110_GATE_SDMMC_GOP: c_int = 18;
pub const CP110_GATE_SLOW_IO: c_int = 21;
pub const CP110_GATE_USB3H0: c_int = 22;
pub const CP110_GATE_USB3H1: c_int = 23;
pub const CP110_GATE_USB3DEV: c_int = 24;
pub const CP110_GATE_EIP150: c_int = 25;
pub const CP110_GATE_EIP197: c_int = 26;
    static const char * const gate_base_names[] = {
    [CP110_GATE_AUDIO]	= "audio",
    [CP110_GATE_COMM_UNIT]	= "communit",
    [CP110_GATE_NAND]	= "nand",
    [CP110_GATE_PPV2]	= "ppv2",
    [CP110_GATE_SDIO]	= "sdio",
    [CP110_GATE_MG]		= "mg-domain",
    [CP110_GATE_MG_CORE]	= "mg-core",
    [CP110_GATE_XOR1]	= "xor1",
    [CP110_GATE_XOR0]	= "xor0",
    [CP110_GATE_GOP_DP]	= "gop-dp",
    [CP110_GATE_PCIE_X1_0]	= "pcie_x10",
    [CP110_GATE_PCIE_X1_1]	= "pcie_x11",
    [CP110_GATE_PCIE_X4]	= "pcie_x4",
    [CP110_GATE_PCIE_XOR]	= "pcie-xor",
    [CP110_GATE_SATA]	= "sata",
    [CP110_GATE_SATA_USB]	= "sata-usb",
    [CP110_GATE_MAIN]	= "main",
    [CP110_GATE_SDMMC_GOP]	= "sd-mmc-gop",
    [CP110_GATE_SLOW_IO]	= "slow-io",
    [CP110_GATE_USB3H0]	= "usb3h0",
    [CP110_GATE_USB3H1]	= "usb3h1",
    [CP110_GATE_USB3DEV]	= "usb3dev",
    [CP110_GATE_EIP150]	= "eip150",
    [CP110_GATE_EIP197]	= "eip197"
    };
#[no_mangle]
unsafe extern "C" fn gate_flags(bit_idx: u8) -> c_ulong {
    static unsigned long gate_flags(const u8 bit_idx)
    {
    switch (bit_idx) {
    case CP110_GATE_PCIE_X1_0:
    case CP110_GATE_PCIE_X1_1:
    case CP110_GATE_PCIE_X4:
//
// If a port had an active link at boot time, stopping
// the clock creates a failed state from which controller
// driver can not recover.
// Prevent stopping this clock till after a driver has taken
// ownership.
//
    return CLK_IGNORE_UNUSED;
    default:
    return 0;
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cp110_gate_clk {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub bit_idx: u8,
}

#[no_mangle]
unsafe extern "C" fn cp110_gate_enable(hw: *mut clk_hw) -> c_int {
    static int cp110_gate_enable(struct clk_hw *hw)
    {
    struct cp110_gate_clk *gate = to_cp110_gate_clk(hw);
    regmap_update_bits(gate.regmap, CP110_PM_CLOCK_GATING_REG,
    BIT(gate.bit_idx), BIT(gate.bit_idx));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cp110_gate_disable(hw: *mut clk_hw) {
    static void cp110_gate_disable(struct clk_hw *hw)
    {
    struct cp110_gate_clk *gate = to_cp110_gate_clk(hw);
    regmap_update_bits(gate.regmap, CP110_PM_CLOCK_GATING_REG,
    BIT(gate.bit_idx), 0);
    }
#[no_mangle]
unsafe extern "C" fn cp110_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    static int cp110_gate_is_enabled(struct clk_hw *hw)
    {
    struct cp110_gate_clk *gate = to_cp110_gate_clk(hw);
    u32 val;
    regmap_read(gate.regmap, CP110_PM_CLOCK_GATING_REG, &val);
    return val & BIT(gate.bit_idx);
    }
    static const struct clk_ops cp110_gate_ops = {
    .enable = cp110_gate_enable,
    .disable = cp110_gate_disable,
    .is_enabled = cp110_gate_is_enabled,
    };
    static struct clk_hw *cp110_register_gate(const char *name,
    const char *parent_name,
    struct regmap *regmap, u8 bit_idx)
    {
    struct cp110_gate_clk *gate;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    gate = kzalloc_obj(*gate);
    if (!gate)
    return ERR_PTR(-ENOMEM);
    memset(&init, 0, sizeof(init));
    init.name = name;
    init.ops = &cp110_gate_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = gate_flags(bit_idx);
    gate.regmap = regmap;
    gate.bit_idx = bit_idx;
    gate.hw.init = &init;
    hw = &gate.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(gate);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
#[no_mangle]
unsafe extern "C" fn cp110_unregister_gate(hw: *mut clk_hw) {
    static void cp110_unregister_gate(struct clk_hw *hw)
    {
    clk_hw_unregister(hw);
    kfree(to_cp110_gate_clk(hw));
    }
    static struct clk_hw *cp110_of_clk_get(struct of_phandle_args *clkspec,
    void *data)
    {
    struct clk_hw_onecell_data *clk_data = data;
    let mut type: c_uint = clkspec.args[0];
    let mut idx: c_uint = clkspec.args[1];
    if (type == CP110_CLK_TYPE_CORE) {
    if (idx >= CP110_MAX_CORE_CLOCKS)
    return ERR_PTR(-EINVAL);
    return clk_data.hws[idx];
    } else if (type == CP110_CLK_TYPE_GATABLE) {
    if (idx >= CP110_MAX_GATABLE_CLOCKS)
    return ERR_PTR(-EINVAL);
    return clk_data.hws[CP110_MAX_CORE_CLOCKS + idx];
    }
    return ERR_PTR(-EINVAL);
    }
    static int cp110_syscon_common_probe(struct platform_device *pdev,
    struct device_node *syscon_node)
    {
    struct regmap *regmap;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    const char *ppv2_name, *pll0_name, *core_name, *x2core_name, *nand_name,
// sdio_name;
    struct clk_hw_onecell_data *cp110_clk_data;
    struct clk_hw *hw, **cp110_clks;
    u32 nand_clk_ctrl;
    int i, ret;
    char *gate_name[ARRAY_SIZE(gate_base_names)];
    regmap = syscon_node_to_regmap(syscon_node);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    ret = regmap_read(regmap, CP110_NAND_FLASH_CLK_CTRL_REG,
    &nand_clk_ctrl);
    if (ret)
    return ret;
    cp110_clk_data = devm_kzalloc(dev, struct_size(cp110_clk_data, hws,
    CP110_CLK_NUM),
    GFP_KERNEL);
    if (!cp110_clk_data)
    return -ENOMEM;
    cp110_clk_data.num = CP110_CLK_NUM;
    cp110_clks = cp110_clk_data.hws;
// Register the PLL0 which is the root of the hw tree
    pll0_name = ap_cp_unique_name(dev, syscon_node, "pll0");
    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), pll0_name, core::ptr::null_mut(), 0,
    1000 * 1000 * 1000);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto fail_pll0;
    }
    cp110_clks[CP110_CORE_PLL0] = hw;
// PPv2 is PLL0/3
    ppv2_name = ap_cp_unique_name(dev, syscon_node, "ppv2-core");
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), ppv2_name, pll0_name, 0, 1, 3);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto fail_ppv2;
    }
    cp110_clks[CP110_CORE_PPV2] = hw;
// X2CORE clock is PLL0/2
    x2core_name = ap_cp_unique_name(dev, syscon_node, "x2core");
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), x2core_name, pll0_name,
    0, 1, 2);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto fail_eip;
    }
    cp110_clks[CP110_CORE_X2CORE] = hw;
// Core clock is X2CORE/2
    core_name = ap_cp_unique_name(dev, syscon_node, "core");
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), core_name, x2core_name,
    0, 1, 2);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto fail_core;
    }
    cp110_clks[CP110_CORE_CORE] = hw;
// NAND can be either PLL0/2.5 or core clock
    nand_name = ap_cp_unique_name(dev, syscon_node, "nand-core");
    if (nand_clk_ctrl & NF_CLOCK_SEL_400_MASK)
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), nand_name,
    pll0_name, 0, 2, 5);
    else
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), nand_name,
    core_name, 0, 1, 1);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto fail_nand;
    }
    cp110_clks[CP110_CORE_NAND] = hw;
// SDIO clock is PLL0/2.5
    sdio_name = ap_cp_unique_name(dev, syscon_node, "sdio-core");
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), sdio_name,
    pll0_name, 0, 2, 5);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto fail_sdio;
    }
    cp110_clks[CP110_CORE_SDIO] = hw;
// create the unique name for all the gate clocks
    for (i = 0; i < ARRAY_SIZE(gate_base_names); i++)
    gate_name[i] =	ap_cp_unique_name(dev, syscon_node,
    gate_base_names[i]);
    for (i = 0; i < ARRAY_SIZE(gate_base_names); i++) {
    const char *parent;
    if (gate_name[i] == core::ptr::null_mut())
    continue;
    switch (i) {
    case CP110_GATE_NAND:
    parent = nand_name;
    break;
    case CP110_GATE_MG:
    case CP110_GATE_GOP_DP:
    case CP110_GATE_PPV2:
    parent = ppv2_name;
    break;
    case CP110_GATE_SDIO:
    parent = sdio_name;
    break;
    case CP110_GATE_MAIN:
    case CP110_GATE_PCIE_XOR:
    case CP110_GATE_PCIE_X4:
    case CP110_GATE_EIP150:
    case CP110_GATE_EIP197:
    parent = x2core_name;
    break;
    default:
    parent = core_name;
    break;
    }
    hw = cp110_register_gate(gate_name[i], parent, regmap, i);
    if (IS_ERR(hw)) {
    ret = PTR_ERR(hw);
    goto fail_gate;
    }
    cp110_clks[CP110_MAX_CORE_CLOCKS + i] = hw;
    }
    ret = of_clk_add_hw_provider(np, cp110_of_clk_get, cp110_clk_data);
    if (ret)
    goto fail_clk_add;
    platform_set_drvdata(pdev, cp110_clks);
    return 0;
    fail_clk_add:
    fail_gate:
    for (i = 0; i < CP110_MAX_GATABLE_CLOCKS; i++) {
    hw = cp110_clks[CP110_MAX_CORE_CLOCKS + i];
    if (hw)
    cp110_unregister_gate(hw);
    }
    clk_hw_unregister_fixed_factor(cp110_clks[CP110_CORE_SDIO]);
    fail_sdio:
    clk_hw_unregister_fixed_factor(cp110_clks[CP110_CORE_NAND]);
    fail_nand:
    clk_hw_unregister_fixed_factor(cp110_clks[CP110_CORE_CORE]);
    fail_core:
    clk_hw_unregister_fixed_factor(cp110_clks[CP110_CORE_X2CORE]);
    fail_eip:
    clk_hw_unregister_fixed_factor(cp110_clks[CP110_CORE_PPV2]);
    fail_ppv2:
    clk_hw_unregister_fixed_rate(cp110_clks[CP110_CORE_PLL0]);
    fail_pll0:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cp110_syscon_legacy_clk_probe(pdev: *mut platform_device) -> c_int {
    static int cp110_syscon_legacy_clk_probe(struct platform_device *pdev)
    {
    dev_warn(&pdev.dev, FW_WARN "Using legacy device tree binding\n");
    dev_warn(&pdev.dev, FW_WARN "Update your device tree:\n");
    dev_warn(&pdev.dev, FW_WARN
    "This binding won't be supported in future kernels\n");
    return cp110_syscon_common_probe(pdev, pdev.dev.of_node);
    }
#[no_mangle]
unsafe extern "C" fn cp110_clk_probe(pdev: *mut platform_device) -> c_int {
    static int cp110_clk_probe(struct platform_device *pdev)
    {
    return cp110_syscon_common_probe(pdev, pdev.dev.of_node.parent);
    }
    static const struct of_device_id cp110_syscon_legacy_of_match[] = {
    { .compatible = "marvell,cp110-system-controller0", },
    { }
    };
    static struct platform_driver cp110_syscon_legacy_driver = {
    .probe = cp110_syscon_legacy_clk_probe,
    .driver		= {
    .name	= "marvell-cp110-system-controller0",
    .of_match_table = cp110_syscon_legacy_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(cp110_syscon_legacy_driver);
    static const struct of_device_id cp110_clock_of_match[] = {
    { .compatible = "marvell,cp110-clock", },
    { }
    };
    static struct platform_driver cp110_clock_driver = {
    .probe = cp110_clk_probe,
    .driver		= {
    .name	= "marvell-cp110-clock",
    .of_match_table = cp110_clock_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(cp110_clock_driver);
