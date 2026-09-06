//! Automatically rewritten from C to Rust
//! Source: drivers/soc/imx/soc-imx8m.c
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
// Copyright 2019 NXP.
//

pub const REV_B1: c_uint = 0x21;
pub const IMX8MQ_SW_INFO_B1: c_uint = 0x40;
pub const IMX8MQ_SW_MAGIC_B1: c_uint = 0xff0055aa;
pub const IMX_SIP_GET_SOC_INFO: c_uint = 0xc2000006;
pub const OCOTP_UID_LOW: c_uint = 0x410;
pub const OCOTP_UID_HIGH: c_uint = 0x420;
pub const IMX8MP_OCOTP_UID_OFFSET: c_uint = 0x10;
pub const IMX8MP_OCOTP_UID_HIGH: c_uint = 0xE00;
// Same as ANADIG_DIGPROG_IMX7D
pub const ANADIG_DIGPROG_IMX8MM: c_uint = 0x800;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8_soc_data {
    pub name: *mut c_char,
    pub ocotp_compatible: *const c_char,
    pub socrev): *mut *mut *mut int (soc_revision)(struct platform_device pdev, u32,
    pub socuid): *mut *mut *mut int (soc_uid)(struct platform_device pdev, u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8_soc_drvdata {
    pub ocotp_base: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn imx8mq_soc_revision_from_atf() -> u32 {
    static u32 imx8mq_soc_revision_from_atf(void)
    {
    struct arm_smccc_res res;
    arm_smccc_smc(IMX_SIP_GET_SOC_INFO, 0, 0, 0, 0, 0, 0, 0, &res);
    if (res.a0 == SMCCC_RET_NOT_SUPPORTED)
    return 0;
    else
    return res.a0 & 0xff;
    }

    static inline u32 imx8mq_soc_revision_from_atf(void) { return 0; };

#[no_mangle]
unsafe extern "C" fn imx8m_soc_uid(pdev: *mut platform_device, socuid: *mut u64) -> c_int {
    static int imx8m_soc_uid(struct platform_device *pdev, u64 *socuid)
    {
    struct imx8_soc_drvdata *drvdata = platform_get_drvdata(pdev);
    void __iomem *ocotp_base = drvdata.ocotp_base;
// socuid = readl_relaxed(ocotp_base + OCOTP_UID_HIGH);
// socuid <<= 32;
// socuid |= readl_relaxed(ocotp_base + OCOTP_UID_LOW);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mq_soc_revision(pdev: *mut platform_device, socrev: *mut u32) -> c_int {
    static int imx8mq_soc_revision(struct platform_device *pdev, u32 *socrev)
    {
    struct imx8_soc_drvdata *drvdata = platform_get_drvdata(pdev);
    void __iomem *ocotp_base = drvdata.ocotp_base;
    u32 magic;
    u32 rev;
//
// SOC revision on older imx8mq is not available in fuses so query
// the value from ATF instead.
//
    rev = imx8mq_soc_revision_from_atf();
    if (!rev) {
    magic = readl_relaxed(ocotp_base + IMX8MQ_SW_INFO_B1);
    if (magic == IMX8MQ_SW_MAGIC_B1)
    rev = REV_B1;
    }
// socrev = rev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_soc_uid(pdev: *mut platform_device, socuid: *mut u64) -> c_int {
    static int imx8mp_soc_uid(struct platform_device *pdev, u64 *socuid)
    {
    struct imx8_soc_drvdata *drvdata = platform_get_drvdata(pdev);
    void __iomem *ocotp_base = drvdata.ocotp_base;
    socuid[0] = readl_relaxed(ocotp_base + OCOTP_UID_HIGH + IMX8MP_OCOTP_UID_OFFSET);
    socuid[0] <<= 32;
    socuid[0] |= readl_relaxed(ocotp_base + OCOTP_UID_LOW + IMX8MP_OCOTP_UID_OFFSET);
    socuid[1] = readl_relaxed(ocotp_base + IMX8MP_OCOTP_UID_HIGH + 0x10);
    socuid[1] <<= 32;
    socuid[1] |= readl_relaxed(ocotp_base + IMX8MP_OCOTP_UID_HIGH);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8mm_soc_revision(pdev: *mut platform_device, socrev: *mut u32) -> c_int {
    static int imx8mm_soc_revision(struct platform_device *pdev, u32 *socrev)
    {
    struct device_node *np __free(device_node) =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx8mm-anatop");
    void __iomem *anatop_base;
    if (!np)
    return -EINVAL;
    anatop_base = of_iomap(np, 0);
    if (!anatop_base)
    return -EINVAL;
// socrev = readl_relaxed(anatop_base + ANADIG_DIGPROG_IMX8MM);
    iounmap(anatop_base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx8m_soc_prepare(pdev: *mut platform_device, ocotp_compatible: *const c_char) -> c_int {
    static int imx8m_soc_prepare(struct platform_device *pdev, const char *ocotp_compatible)
    {
    struct device_node *np __free(device_node) =
    of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), ocotp_compatible);
    struct imx8_soc_drvdata *drvdata = platform_get_drvdata(pdev);
    let mut ret: c_int = 0;
    if (!np)
    return -EINVAL;
    drvdata.ocotp_base = of_iomap(np, 0);
    if (!drvdata.ocotp_base)
    return -EINVAL;
    drvdata.clk = of_clk_get_by_name(np, core::ptr::null_mut());
    if (IS_ERR(drvdata.clk)) {
    ret = PTR_ERR(drvdata.clk);
    goto err_clk;
    }
    ret = clk_prepare_enable(drvdata.clk);
    if (ret)
    goto err_clk;
    return 0;
    err_clk:
    iounmap(drvdata.ocotp_base);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx8m_soc_unprepare(pdev: *mut platform_device) {
    static void imx8m_soc_unprepare(struct platform_device *pdev)
    {
    struct imx8_soc_drvdata *drvdata = platform_get_drvdata(pdev);
    clk_disable_unprepare(drvdata.clk);
    clk_put(drvdata.clk);
    iounmap(drvdata.ocotp_base);
    }
    static const struct imx8_soc_data imx8mq_soc_data = {
    .name = "i.MX8MQ",
    .ocotp_compatible = "fsl,imx8mq-ocotp",
    .soc_revision = imx8mq_soc_revision,
    .soc_uid = imx8m_soc_uid,
    };
    static const struct imx8_soc_data imx8mm_soc_data = {
    .name = "i.MX8MM",
    .ocotp_compatible = "fsl,imx8mm-ocotp",
    .soc_revision = imx8mm_soc_revision,
    .soc_uid = imx8m_soc_uid,
    };
    static const struct imx8_soc_data imx8mn_soc_data = {
    .name = "i.MX8MN",
    .ocotp_compatible = "fsl,imx8mm-ocotp",
    .soc_revision = imx8mm_soc_revision,
    .soc_uid = imx8m_soc_uid,
    };
    static const struct imx8_soc_data imx8mp_soc_data = {
    .name = "i.MX8MP",
    .ocotp_compatible = "fsl,imx8mm-ocotp",
    .soc_revision = imx8mm_soc_revision,
    .soc_uid = imx8mp_soc_uid,
    };
    static __maybe_unused const struct of_device_id imx8_soc_match[] = {
    { .compatible = "fsl,imx8mq", .data = &imx8mq_soc_data, },
    { .compatible = "fsl,imx8mm", .data = &imx8mm_soc_data, },
    { .compatible = "fsl,imx8mn", .data = &imx8mn_soc_data, },
    { .compatible = "fsl,imx8mp", .data = &imx8mp_soc_data, },
    { }
    };

    (soc_rev) ? \
    devm_kasprintf((dev), GFP_KERNEL, "%d.%d", ((soc_rev) >> 4) & 0xf, (soc_rev) & 0xf) : \
    "unknown"
#[no_mangle]
unsafe extern "C" fn imx8m_unregister_soc(data: *mut c_void) {
    static void imx8m_unregister_soc(void *data)
    {
    soc_device_unregister(data);
    }
#[no_mangle]
unsafe extern "C" fn imx8m_unregister_cpufreq(data: *mut c_void) {
    static void imx8m_unregister_cpufreq(void *data)
    {
    platform_device_unregister(data);
    }
#[no_mangle]
unsafe extern "C" fn imx8m_soc_probe(pdev: *mut platform_device) -> c_int {
    static int imx8m_soc_probe(struct platform_device *pdev)
    {
    struct soc_device_attribute *soc_dev_attr;
    struct platform_device *cpufreq_dev;
    const struct imx8_soc_data *data;
    struct imx8_soc_drvdata *drvdata;
    struct device *dev = &pdev.dev;
    struct soc_device *soc_dev;
    let mut soc_rev: u32 = 0;
    u64 soc_uid[2] = {0, 0};
    int ret;
    soc_dev_attr = devm_kzalloc(dev, sizeof(*soc_dev_attr), GFP_KERNEL);
    if (!soc_dev_attr)
    return -ENOMEM;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    platform_set_drvdata(pdev, drvdata);
    soc_dev_attr.family = "Freescale i.MX";
    ret = soc_attr_read_machine(soc_dev_attr);
    if (ret)
    return ret;
    data = of_machine_get_match_data(imx8_soc_match);
    if (data) {
    soc_dev_attr.soc_id = data.name;
    ret = imx8m_soc_prepare(pdev, data.ocotp_compatible);
    if (ret)
    return ret;
    if (data.soc_revision) {
    ret = data.soc_revision(pdev, &soc_rev);
    if (ret) {
    imx8m_soc_unprepare(pdev);
    return ret;
    }
    }
    if (data.soc_uid) {
    ret = data.soc_uid(pdev, soc_uid);
    if (ret) {
    imx8m_soc_unprepare(pdev);
    return ret;
    }
    }
    imx8m_soc_unprepare(pdev);
    }
    soc_dev_attr.revision = imx8_revision(dev, soc_rev);
    if (!soc_dev_attr.revision)
    return -ENOMEM;
    if (soc_uid[1])
    soc_dev_attr.serial_number = devm_kasprintf(dev, GFP_KERNEL, "%016llX%016llX",
    soc_uid[1], soc_uid[0]);
    else
    soc_dev_attr.serial_number = devm_kasprintf(dev, GFP_KERNEL, "%016llX",
    soc_uid[0]);
    if (!soc_dev_attr.serial_number)
    return -ENOMEM;
    soc_dev = soc_device_register(soc_dev_attr);
    if (IS_ERR(soc_dev))
    return PTR_ERR(soc_dev);
    ret = devm_add_action(dev, imx8m_unregister_soc, soc_dev);
    if (ret)
    return ret;
    pr_info("SoC: %s revision %s\n", soc_dev_attr.soc_id,
    soc_dev_attr.revision);
    if (IS_ENABLED(CONFIG_ARM_IMX_CPUFREQ_DT)) {
    cpufreq_dev = platform_device_register_simple("imx-cpufreq-dt", -1, core::ptr::null_mut(), 0);
    if (IS_ERR(cpufreq_dev))
    return dev_err_probe(dev, PTR_ERR(cpufreq_dev),
    "Failed to register imx-cpufreq-dev device\n");
    ret = devm_add_action(dev, imx8m_unregister_cpufreq, cpufreq_dev);
    if (ret)
    return ret;
    }
    return 0;
    }
    static struct platform_driver imx8m_soc_driver = {
    .probe = imx8m_soc_probe,
    .driver = {
    .name = "imx8m-soc",
    },
    };
#[no_mangle]
unsafe extern "C" fn imx8_soc_init() -> int __init {
    static int __init imx8_soc_init(void)
    {
    struct platform_device *pdev;
    int ret;
// No match means this is non-i.MX8M hardware, do nothing.
    if (!of_machine_device_match(imx8_soc_match))
    return 0;
    ret = platform_driver_register(&imx8m_soc_driver);
    if (ret) {
    pr_err("Failed to register imx8m-soc platform driver: %d\n", ret);
    return ret;
    }
    pdev = platform_device_register_simple("imx8m-soc", -1, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    pr_err("Failed to register imx8m-soc platform device: %ld\n", PTR_ERR(pdev));
    platform_driver_unregister(&imx8m_soc_driver);
    return PTR_ERR(pdev);
    }
    return 0;
    }
    device_initcall(imx8_soc_init);
    MODULE_DESCRIPTION("NXP i.MX8M SoC driver");
    MODULE_LICENSE("GPL");
