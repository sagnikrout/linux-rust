//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/qcom-apcs-ipc-mailbox.c
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
//
// Copyright (c) 2017, Linaro Ltd
//

pub const QCOM_APCS_IPC_BITS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_apcs_ipc {
    pub mbox: mbox_controller,
    pub mbox_chans: [mbox_chan; QCOM_APCS_IPC_BITS],
    pub regmap: *mut regmap,
    pub offset: c_ulong,
    pub clk: *mut platform_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_apcs_ipc_data {
    pub offset: c_int,
    pub clk_name: *mut c_char,
}

    static const struct qcom_apcs_ipc_data ipq6018_apcs_data = {
    .offset = 8, .clk_name = "qcom,apss-ipq6018-clk"
    };
    static const struct qcom_apcs_ipc_data msm8916_apcs_data = {
    .offset = 8, .clk_name = "qcom-apcs-msm8916-clk"
    };
    static const struct qcom_apcs_ipc_data msm8994_apcs_data = {
    .offset = 8, .clk_name = core::ptr::null_mut()
    };
    static const struct qcom_apcs_ipc_data msm8996_apcs_data = {
    .offset = 16, .clk_name = "qcom-apcs-msm8996-clk"
    };
    static const struct qcom_apcs_ipc_data apps_shared_apcs_data = {
    .offset = 12, .clk_name = core::ptr::null_mut()
    };
    static const struct qcom_apcs_ipc_data sdx55_apcs_data = {
    .offset = 0x1008, .clk_name = "qcom-sdx55-acps-clk"
    };
    static const struct regmap_config apcs_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = 0x1008,
    };
#[no_mangle]
unsafe extern "C" fn qcom_apcs_ipc_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int qcom_apcs_ipc_send_data(struct mbox_chan *chan, void *data)
    {
    struct qcom_apcs_ipc *apcs = container_of(chan.mbox,
    struct qcom_apcs_ipc, mbox);
    let mut idx: c_ulong = (unsigned long)chan.con_priv;
    return regmap_write(apcs.regmap, apcs.offset, BIT(idx));
    }
    static const struct mbox_chan_ops qcom_apcs_ipc_ops = {
    .send_data = qcom_apcs_ipc_send_data,
    };
#[no_mangle]
unsafe extern "C" fn qcom_apcs_ipc_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_apcs_ipc_probe(struct platform_device *pdev)
    {
    struct qcom_apcs_ipc *apcs;
    const struct qcom_apcs_ipc_data *apcs_data;
    struct regmap *regmap;
    void __iomem *base;
    unsigned long i;
    int ret;
    apcs = devm_kzalloc(&pdev.dev, sizeof(*apcs), GFP_KERNEL);
    if (!apcs)
    return -ENOMEM;
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
    regmap = devm_regmap_init_mmio(&pdev.dev, base, &apcs_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    apcs_data = of_device_get_match_data(&pdev.dev);
    apcs.regmap = regmap;
    apcs.offset = apcs_data.offset;
// Initialize channel identifiers
    for (i = 0; i < ARRAY_SIZE(apcs.mbox_chans); i++)
    apcs.mbox_chans[i].con_priv = (void *)i;
    apcs.mbox.dev = &pdev.dev;
    apcs.mbox.ops = &qcom_apcs_ipc_ops;
    apcs.mbox.chans = apcs.mbox_chans;
    apcs.mbox.num_chans = ARRAY_SIZE(apcs.mbox_chans);
    ret = devm_mbox_controller_register(&pdev.dev, &apcs.mbox);
    if (ret) {
    dev_err(&pdev.dev, "failed to register APCS IPC controller\n");
    return ret;
    }
    if (apcs_data.clk_name) {
    struct device_node *np = of_get_child_by_name(pdev.dev.of_node,
    "clock-controller");
    struct platform_device_info pdevinfo = {
    .parent = &pdev.dev,
    .name = apcs_data.clk_name,
    .id = PLATFORM_DEVID_AUTO,
    .fwnode = of_fwnode_handle(np) ?: pdev.dev.fwnode,
    .of_node_reused = !np,
    };
    apcs.clk = platform_device_register_full(&pdevinfo);
    of_node_put(np);
    if (IS_ERR(apcs.clk))
    dev_err(&pdev.dev, "failed to register APCS clk\n");
    }
    platform_set_drvdata(pdev, apcs);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qcom_apcs_ipc_remove(pdev: *mut platform_device) {
    static void qcom_apcs_ipc_remove(struct platform_device *pdev)
    {
    struct qcom_apcs_ipc *apcs = platform_get_drvdata(pdev);
    struct platform_device *clk = apcs.clk;
    platform_device_unregister(clk);
    }
// .data is the offset of the ipc register within the global block
    static const struct of_device_id qcom_apcs_ipc_of_match[] = {
    { .compatible = "qcom,ipq6018-apcs-apps-global", .data = &ipq6018_apcs_data },
    { .compatible = "qcom,msm8916-apcs-kpss-global", .data = &msm8916_apcs_data },
    { .compatible = "qcom,msm8939-apcs-kpss-global", .data = &msm8916_apcs_data },
    { .compatible = "qcom,msm8953-apcs-kpss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,msm8994-apcs-kpss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,msm8996-apcs-hmss-global", .data = &msm8996_apcs_data },
    { .compatible = "qcom,qcm2290-apcs-hmss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,sdm845-apss-shared", .data = &apps_shared_apcs_data },
    { .compatible = "qcom,sdx55-apcs-gcc", .data = &sdx55_apcs_data },
// Do not add any more entries using existing driver data
    { .compatible = "qcom,msm8976-apcs-kpss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,msm8998-apcs-hmss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,qcs404-apcs-apps-global", .data = &msm8916_apcs_data },
    { .compatible = "qcom,sdm660-apcs-hmss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,sm4250-apcs-hmss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,sm6125-apcs-hmss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,sm6115-apcs-hmss-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,ipq5332-apcs-apps-global", .data = &ipq6018_apcs_data },
    { .compatible = "qcom,ipq5424-apcs-apps-global", .data = &msm8994_apcs_data },
    { .compatible = "qcom,ipq8074-apcs-apps-global", .data = &ipq6018_apcs_data },
    { .compatible = "qcom,sc7180-apss-shared", .data = &apps_shared_apcs_data },
    { .compatible = "qcom,sc8180x-apss-shared", .data = &apps_shared_apcs_data },
    { .compatible = "qcom,sm8150-apss-shared", .data = &apps_shared_apcs_data },
    {}
    };
    MODULE_DEVICE_TABLE(of, qcom_apcs_ipc_of_match);
    static struct platform_driver qcom_apcs_ipc_driver = {
    .probe = qcom_apcs_ipc_probe,
    .remove = qcom_apcs_ipc_remove,
    .driver = {
    .name = "qcom_apcs_ipc",
    .of_match_table = qcom_apcs_ipc_of_match,
    },
    };
#[no_mangle]
unsafe extern "C" fn qcom_apcs_ipc_init() -> int __init {
    static int __init qcom_apcs_ipc_init(void)
    {
    return platform_driver_register(&qcom_apcs_ipc_driver);
    }
    postcore_initcall(qcom_apcs_ipc_init);
#[no_mangle]
unsafe extern "C" fn qcom_apcs_ipc_exit() -> void __exit {
    static void __exit qcom_apcs_ipc_exit(void)
    {
    platform_driver_unregister(&qcom_apcs_ipc_driver);
    }
    module_exit(qcom_apcs_ipc_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Qualcomm APCS IPC driver");
