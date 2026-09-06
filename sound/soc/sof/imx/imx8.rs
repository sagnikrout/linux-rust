//! Automatically rewritten from C to Rust
//! Source: sound/soc/sof/imx/imx8.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright 2019-2025 NXP
//
// Author: Daniel Baluta <daniel.baluta@nxp.com>
//
// Hardware interface for audio DSP on i.MX8

// imx8/imx8x macros
pub const RESET_VECTOR_VADDR: c_uint = 0x596f8000;
// imx8m macros
pub const IMX8M_DAP_DEBUG: c_uint = 0x28800000;

// imx8ulp macros
pub const FSL_SIP_HIFI_XRDC: c_uint = 0xc200000e;
pub const SYSCTRL0: c_uint = 0x8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8m_chip_data {
    pub dap: *mut void __iomem,
    pub regmap: *mut regmap,
    pub run_stall: *mut reset_control,
}

#[no_mangle]
unsafe extern "C" fn imx8_shutdown(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8_shutdown(struct snd_sof_dev *sdev)
    {
//
// Force the DSP to stall. After the firmware image is loaded,
// the stall will be removed during run() by a matching
// imx_sc_pm_cpu_start() call.
//
    imx_sc_pm_cpu_start(get_chip_pdata(sdev), IMX_SC_R_DSP, false,
    RESET_VECTOR_VADDR);
    return 0;
    }
//
// DSP control.
//
#[no_mangle]
unsafe extern "C" fn imx8x_run(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8x_run(struct snd_sof_dev *sdev)
    {
    int ret;
    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP,
    IMX_SC_C_OFS_SEL, 1);
    if (ret < 0) {
    dev_err(sdev.dev, "Error system address offset source select\n");
    return ret;
    }
    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP,
    IMX_SC_C_OFS_AUDIO, 0x80);
    if (ret < 0) {
    dev_err(sdev.dev, "Error system address offset of AUDIO\n");
    return ret;
    }
    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP,
    IMX_SC_C_OFS_PERIPH, 0x5A);
    if (ret < 0) {
    dev_err(sdev.dev, "Error system address offset of PERIPH %d\n",
    ret);
    return ret;
    }
    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP,
    IMX_SC_C_OFS_IRQ, 0x51);
    if (ret < 0) {
    dev_err(sdev.dev, "Error system address offset of IRQ\n");
    return ret;
    }
    imx_sc_pm_cpu_start(get_chip_pdata(sdev), IMX_SC_R_DSP, true,
    RESET_VECTOR_VADDR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8_run(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8_run(struct snd_sof_dev *sdev)
    {
    int ret;
    ret = imx_sc_misc_set_control(get_chip_pdata(sdev), IMX_SC_R_DSP,
    IMX_SC_C_OFS_SEL, 0);
    if (ret < 0) {
    dev_err(sdev.dev, "Error system address offset source select\n");
    return ret;
    }
    imx_sc_pm_cpu_start(get_chip_pdata(sdev), IMX_SC_R_DSP, true,
    RESET_VECTOR_VADDR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8_probe(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8_probe(struct snd_sof_dev *sdev)
    {
    struct imx_sc_ipc *sc_ipc_handle;
    struct imx_common_data *common;
    int ret;
    common = sdev.pdata.hw_pdata;
    ret = imx_scu_get_handle(&sc_ipc_handle);
    if (ret < 0)
    return dev_err_probe(sdev.dev, ret,
    "failed to fetch SC IPC handle\n");
    common.chip_pdata = sc_ipc_handle;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8m_reset(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8m_reset(struct snd_sof_dev *sdev)
    {
    struct imx8m_chip_data *chip;
    u32 pwrctl;
    chip = get_chip_pdata(sdev);
// put DSP into reset and stall
    pwrctl = readl(chip.dap + IMX8M_DAP_PWRCTL);
    pwrctl |= IMX8M_PWRCTL_CORERESET;
    writel(pwrctl, chip.dap + IMX8M_DAP_PWRCTL);
// keep reset asserted for 10 cycles
    usleep_range(1, 2);
    reset_control_assert(chip.run_stall);
// take the DSP out of reset and keep stalled for FW loading
    pwrctl = readl(chip.dap + IMX8M_DAP_PWRCTL);
    pwrctl &= ~IMX8M_PWRCTL_CORERESET;
    writel(pwrctl, chip.dap + IMX8M_DAP_PWRCTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8m_run(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8m_run(struct snd_sof_dev *sdev)
    {
    struct imx8m_chip_data *chip = get_chip_pdata(sdev);
    return reset_control_deassert(chip.run_stall);
    }
#[no_mangle]
unsafe extern "C" fn imx8m_probe(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8m_probe(struct snd_sof_dev *sdev)
    {
    struct imx_common_data *common;
    struct imx8m_chip_data *chip;
    common = sdev.pdata.hw_pdata;
    chip = devm_kzalloc(sdev.dev, sizeof(*chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.dap = devm_ioremap(sdev.dev, IMX8M_DAP_DEBUG, IMX8M_DAP_DEBUG_SIZE);
    if (!chip.dap)
    return dev_err_probe(sdev.dev, -ENODEV,
    "failed to ioremap DAP\n");
    chip.run_stall = devm_reset_control_get_exclusive(sdev.dev, "runstall");
    if (IS_ERR(chip.run_stall))
    return dev_err_probe(sdev.dev, PTR_ERR(chip.run_stall),
    "failed to get dsp runstall reset control\n");
    common.chip_pdata = chip;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8ulp_run(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8ulp_run(struct snd_sof_dev *sdev)
    {
    struct regmap *regmap = get_chip_pdata(sdev);
// Controls the HiFi4 DSP Reset: 1 in reset, 0 out of reset
    regmap_update_bits(regmap, SYSCTRL0, RESET_BIT, 0);
// Reset HiFi4 DSP Debug logic: 1 debug reset, 0  out of reset
    regmap_update_bits(regmap, SYSCTRL0, DEBUG_LOGIC_BIT, 0);
// Stall HIFI4 DSP Execution: 1 stall, 0 run
    regmap_update_bits(regmap, SYSCTRL0, EXECUTE_BIT, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8ulp_reset(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8ulp_reset(struct snd_sof_dev *sdev)
    {
    struct arm_smccc_res smc_res;
    struct regmap *regmap;
    regmap = get_chip_pdata(sdev);
// HiFi4 Platform Clock Enable: 1 enabled, 0 disabled
    regmap_update_bits(regmap, SYSCTRL0, PLAT_CLK_BIT, PLAT_CLK_BIT);
// HiFi4 PBCLK clock enable: 1 enabled, 0 disabled
    regmap_update_bits(regmap, SYSCTRL0, PB_CLK_BIT, PB_CLK_BIT);
// HiFi4 Clock Enable: 1 enabled, 0 disabled
    regmap_update_bits(regmap, SYSCTRL0, HIFI4_CLK_BIT, HIFI4_CLK_BIT);
    regmap_update_bits(regmap, SYSCTRL0, RESET_BIT, RESET_BIT);
    usleep_range(1, 2);
// Stall HIFI4 DSP Execution: 1 stall, 0 not stall
    regmap_update_bits(regmap, SYSCTRL0, EXECUTE_BIT, EXECUTE_BIT);
    usleep_range(1, 2);
    arm_smccc_smc(FSL_SIP_HIFI_XRDC, 0, 0, 0, 0, 0, 0, 0, &smc_res);
    return smc_res.a0;
    }
#[no_mangle]
unsafe extern "C" fn imx8ulp_probe(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8ulp_probe(struct snd_sof_dev *sdev)
    {
    struct imx_common_data *common;
    struct regmap *regmap;
    common = sdev.pdata.hw_pdata;
    regmap = syscon_regmap_lookup_by_phandle(sdev.dev.of_node, "fsl,dsp-ctrl");
    if (IS_ERR(regmap))
    return dev_err_probe(sdev.dev, PTR_ERR(regmap),
    "failed to fetch dsp ctrl regmap\n");
    common.chip_pdata = regmap;
    return 0;
    }
    static struct snd_soc_dai_driver imx8_dai[] = {
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("esai0", 1, 8),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai1", 1, 32),
    };
    static struct snd_soc_dai_driver imx8m_dai[] = {
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai1", 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai2", 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai3", 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai5", 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai6", 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai7", 1, 32),
    IMX_SOF_DAI_DRV_ENTRY("micfil", 0, 0, 1, 8),
    };
    static struct snd_soc_dai_driver imx8ulp_dai[] = {
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai5", 1, 32),
    IMX_SOF_DAI_DRV_ENTRY_BIDIR("sai6", 1, 32),
    };
    static struct snd_sof_dsp_ops sof_imx8_ops;
#[no_mangle]
unsafe extern "C" fn imx8_ops_init(sdev: *mut snd_sof_dev) -> c_int {
    static int imx8_ops_init(struct snd_sof_dev *sdev)
    {
// first copy from template
    memcpy(&sof_imx8_ops, &sof_imx_ops, sizeof(sof_imx_ops));
// then set common imx8 ops
    sof_imx8_ops.dbg_dump = imx8_dump;
    sof_imx8_ops.dsp_arch_ops = &sof_xtensa_arch_ops;
    sof_imx8_ops.debugfs_add_region_item =
    snd_sof_debugfs_add_region_item_iomem;
// ... and finally set DAI driver
    sof_imx8_ops.drv = get_chip_info(sdev).drv;
    sof_imx8_ops.num_drv = get_chip_info(sdev).num_drv;
    return 0;
    }
    static const struct imx_chip_ops imx8_chip_ops = {
    .probe = imx8_probe,
    .core_kick = imx8_run,
    .core_shutdown = imx8_shutdown,
    };
    static const struct imx_chip_ops imx8x_chip_ops = {
    .probe = imx8_probe,
    .core_kick = imx8x_run,
    .core_shutdown = imx8_shutdown,
    };
    static const struct imx_chip_ops imx8m_chip_ops = {
    .probe = imx8m_probe,
    .core_kick = imx8m_run,
    .core_reset = imx8m_reset,
    };
    static const struct imx_chip_ops imx8ulp_chip_ops = {
    .probe = imx8ulp_probe,
    .core_kick = imx8ulp_run,
    .core_reset = imx8ulp_reset,
    };
    static struct imx_memory_info imx8_memory_regions[] = {
    { .name = "iram", .reserved = false },
    { .name = "sram", .reserved = true },
    { }
    };
    static struct imx_memory_info imx8m_memory_regions[] = {
    { .name = "iram", .reserved = false },
    { .name = "sram", .reserved = true },
    { }
    };
    static struct imx_memory_info imx8ulp_memory_regions[] = {
    { .name = "iram", .reserved = false },
    { .name = "sram", .reserved = true },
    { }
    };
    static const struct imx_chip_info imx8_chip_info = {
    .ipc_info = {
    .has_panic_code = true,
    .boot_mbox_offset = 0x800000,
    .window_offset = 0x800000,
    },
    .memory = imx8_memory_regions,
    .drv = imx8_dai,
    .num_drv = ARRAY_SIZE(imx8_dai),
    .ops = &imx8_chip_ops,
    };
    static const struct imx_chip_info imx8x_chip_info = {
    .ipc_info = {
    .has_panic_code = true,
    .boot_mbox_offset = 0x800000,
    .window_offset = 0x800000,
    },
    .memory = imx8_memory_regions,
    .drv = imx8_dai,
    .num_drv = ARRAY_SIZE(imx8_dai),
    .ops = &imx8x_chip_ops,
    };
    static const struct imx_chip_info imx8m_chip_info = {
    .ipc_info = {
    .has_panic_code = true,
    .boot_mbox_offset = 0x800000,
    .window_offset = 0x800000,
    },
    .memory = imx8m_memory_regions,
    .drv = imx8m_dai,
    .num_drv = ARRAY_SIZE(imx8m_dai),
    .ops = &imx8m_chip_ops,
    };
    static const struct imx_chip_info imx8ulp_chip_info = {
    .ipc_info = {
    .has_panic_code = true,
    .boot_mbox_offset = 0x800000,
    .window_offset = 0x800000,
    },
    .has_dma_reserved = true,
    .memory = imx8ulp_memory_regions,
    .drv = imx8ulp_dai,
    .num_drv = ARRAY_SIZE(imx8ulp_dai),
    .ops = &imx8ulp_chip_ops,
    };
    static struct snd_sof_of_mach sof_imx8_machs[] = {
    {
    .compatible = "fsl,imx8qxp-mek",
    .sof_tplg_filename = "sof-imx8-wm8960.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {
    .compatible = "fsl,imx8qxp-mek-wcpu",
    .sof_tplg_filename = "sof-imx8-wm8962.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {
    .compatible = "fsl,imx8qm-mek",
    .sof_tplg_filename = "sof-imx8-wm8960.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {
    .compatible = "fsl,imx8qm-mek-revd",
    .sof_tplg_filename = "sof-imx8-wm8962.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {
    .compatible = "fsl,imx8qxp-mek-bb",
    .sof_tplg_filename = "sof-imx8-cs42888.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {
    .compatible = "fsl,imx8qm-mek-bb",
    .sof_tplg_filename = "sof-imx8-cs42888.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {
    .compatible = "fsl,imx8mp-evk",
    .sof_tplg_filename = "sof-imx8mp-wm8960.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {
    .compatible = "fsl,imx8mp-evk-revb4",
    .sof_tplg_filename = "sof-imx8mp-wm8962.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {
    .compatible = "fsl,imx8ulp-evk",
    .sof_tplg_filename = "sof-imx8ulp-btsco.tplg",
    .drv_name = "asoc-audio-graph-card2",
    },
    {}
    };
    IMX_SOF_DEV_DESC(imx8, sof_imx8_machs, &imx8_chip_info, &sof_imx8_ops, imx8_ops_init);
    IMX_SOF_DEV_DESC(imx8x, sof_imx8_machs, &imx8x_chip_info, &sof_imx8_ops, imx8_ops_init);
    IMX_SOF_DEV_DESC(imx8m, sof_imx8_machs, &imx8m_chip_info, &sof_imx8_ops, imx8_ops_init);
    IMX_SOF_DEV_DESC(imx8ulp, sof_imx8_machs, &imx8ulp_chip_info, &sof_imx8_ops, imx8_ops_init);
    static const struct of_device_id sof_of_imx8_ids[] = {
    {
    .compatible = "fsl,imx8qxp-dsp",
    .data = &IMX_SOF_DEV_DESC_NAME(imx8x),
    },
    {
    .compatible = "fsl,imx8qm-dsp",
    .data = &IMX_SOF_DEV_DESC_NAME(imx8),
    },
    {
    .compatible = "fsl,imx8mp-dsp",
    .data = &IMX_SOF_DEV_DESC_NAME(imx8m),
    },
    {
    .compatible = "fsl,imx8ulp-dsp",
    .data = &IMX_SOF_DEV_DESC_NAME(imx8ulp),
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, sof_of_imx8_ids);
// DT driver definition
    static struct platform_driver snd_sof_of_imx8_driver = {
    .probe = sof_of_probe,
    .remove = sof_of_remove,
    .driver = {
    .name = "sof-audio-of-imx8",
    .pm = pm_ptr(&sof_of_pm),
    .of_match_table = sof_of_imx8_ids,
    },
    };
    module_platform_driver(snd_sof_of_imx8_driver);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("SOF support for IMX8 platforms");
    MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA");
