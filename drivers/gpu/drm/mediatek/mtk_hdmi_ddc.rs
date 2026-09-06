//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mediatek/mtk_hdmi_ddc.c
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
// Copyright (c) 2014 MediaTek Inc.
// Author: Jie Qiu <jie.qiu@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_hdmi_ddc {
    pub adap: i2c_adapter,
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
}

    static inline void sif_set_bit(struct mtk_hdmi_ddc *ddc, unsigned int offset,
    unsigned int val)
    {
    writel(readl(ddc.regs + offset) | val, ddc.regs + offset);
    }
    static inline void sif_clr_bit(struct mtk_hdmi_ddc *ddc, unsigned int offset,
    unsigned int val)
    {
    writel(readl(ddc.regs + offset) & ~val, ddc.regs + offset);
    }
    static inline bool sif_bit_is_set(struct mtk_hdmi_ddc *ddc, unsigned int offset,
    unsigned int val)
    {
    return (readl(ddc.regs + offset) & val) == val;
    }
    static inline void sif_write_mask(struct mtk_hdmi_ddc *ddc, unsigned int offset,
    unsigned int mask, unsigned int shift,
    unsigned int val)
    {
    unsigned int tmp;
    tmp = readl(ddc.regs + offset);
    tmp &= ~mask;
    tmp |= (val << shift) & mask;
    writel(tmp, ddc.regs + offset);
    }
    static inline unsigned int sif_read_mask(struct mtk_hdmi_ddc *ddc,
    unsigned int offset, unsigned int mask,
    unsigned int shift)
    {
    return (readl(ddc.regs + offset) & mask) >> shift;
    }
#[no_mangle]
unsafe extern "C" fn ddcm_trigger_mode(ddc: *mut mtk_hdmi_ddc, mode: c_int) {
    static void ddcm_trigger_mode(struct mtk_hdmi_ddc *ddc, int mode)
    {
    u32 val;
    sif_write_mask(ddc, DDC_DDCMCTL1, DDCM_SIF_MODE_MASK,
    DDCM_SIF_MODE_OFFSET, mode);
    sif_set_bit(ddc, DDC_DDCMCTL1, DDCM_TRI);
    readl_poll_timeout(ddc.regs + DDC_DDCMCTL1, val,
    (val & DDCM_TRI) != DDCM_TRI, 4, 20000);
    }
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_ddc_read_msg(ddc: *mut mtk_hdmi_ddc, msg: *mut i2c_msg) -> c_int {
    static int mtk_hdmi_ddc_read_msg(struct mtk_hdmi_ddc *ddc, struct i2c_msg *msg)
    {
    struct device *dev = ddc.adap.dev.parent;
    u32 remain_count, ack_count, ack_final, read_count, temp_count;
    let mut index: u32 = 0;
    u32 ack;
    int i;
    ddcm_trigger_mode(ddc, DDCM_START);
    sif_write_mask(ddc, DDC_DDCMD0, 0xff, 0, (msg.addr << 1) | 0x01);
    sif_write_mask(ddc, DDC_DDCMCTL1, DDCM_PGLEN_MASK, DDCM_PGLEN_OFFSET,
    0x00);
    ddcm_trigger_mode(ddc, DDCM_WRITE_DATA);
    ack = sif_read_mask(ddc, DDC_DDCMCTL1, DDCM_ACK_MASK, DDCM_ACK_OFFSET);
    dev_dbg(dev, "ack = 0x%x\n", ack);
    if (ack != 0x01) {
    dev_err(dev, "i2c ack err!\n");
    return -ENXIO;
    }
    remain_count = msg.len;
    ack_count = (msg.len - 1) / 8;
    ack_final = 0;
    while (remain_count > 0) {
    if (ack_count > 0) {
    read_count = 8;
    ack_final = 0;
    ack_count--;
    } else {
    read_count = remain_count;
    ack_final = 1;
    }
    sif_write_mask(ddc, DDC_DDCMCTL1, DDCM_PGLEN_MASK,
    DDCM_PGLEN_OFFSET, read_count - 1);
    ddcm_trigger_mode(ddc, (ack_final == 1) ?
    DDCM_READ_DATA_NO_ACK :
    DDCM_READ_DATA_ACK);
    ack = sif_read_mask(ddc, DDC_DDCMCTL1, DDCM_ACK_MASK,
    DDCM_ACK_OFFSET);
    temp_count = 0;
    while (((ack & (1 << temp_count)) != 0) && (temp_count < 8))
    temp_count++;
    if (((ack_final == 1) && (temp_count != (read_count - 1))) ||
    ((ack_final == 0) && (temp_count != read_count))) {
    dev_err(dev, "Address NACK! ACK(0x%x)\n", ack);
    break;
    }
    for (i = read_count; i >= 1; i--) {
    int shift;
    int offset;
    if (i > 4) {
    offset = DDC_DDCMD1;
    shift = (i - 5) * 8;
    } else {
    offset = DDC_DDCMD0;
    shift = (i - 1) * 8;
    }
    msg.buf[index + i - 1] = sif_read_mask(ddc, offset,
    0xff << shift,
    shift);
    }
    remain_count -= read_count;
    index += read_count;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_ddc_write_msg(ddc: *mut mtk_hdmi_ddc, msg: *mut i2c_msg) -> c_int {
    static int mtk_hdmi_ddc_write_msg(struct mtk_hdmi_ddc *ddc, struct i2c_msg *msg)
    {
    struct device *dev = ddc.adap.dev.parent;
    u32 ack;
    ddcm_trigger_mode(ddc, DDCM_START);
    sif_write_mask(ddc, DDC_DDCMD0, DDCM_DATA0, 0, msg.addr << 1);
    sif_write_mask(ddc, DDC_DDCMD0, DDCM_DATA1, 8, msg.buf[0]);
    sif_write_mask(ddc, DDC_DDCMCTL1, DDCM_PGLEN_MASK, DDCM_PGLEN_OFFSET,
    0x1);
    ddcm_trigger_mode(ddc, DDCM_WRITE_DATA);
    ack = sif_read_mask(ddc, DDC_DDCMCTL1, DDCM_ACK_MASK, DDCM_ACK_OFFSET);
    dev_dbg(dev, "ack = %d\n", ack);
    if (ack != 0x03) {
    dev_err(dev, "i2c ack err!\n");
    return -EIO;
    }
    return 0;
    }
    static int mtk_hdmi_ddc_xfer(struct i2c_adapter *adapter,
    struct i2c_msg *msgs, int num)
    {
    struct mtk_hdmi_ddc *ddc = adapter.algo_data;
    struct device *dev = adapter.dev.parent;
    int ret;
    int i;
    if (!ddc) {
    dev_err(dev, "invalid arguments\n");
    return -EINVAL;
    }
    sif_set_bit(ddc, DDC_DDCMCTL0, DDCM_SCL_STRECH);
    sif_set_bit(ddc, DDC_DDCMCTL0, DDCM_SM0EN);
    sif_clr_bit(ddc, DDC_DDCMCTL0, DDCM_ODRAIN);
    if (sif_bit_is_set(ddc, DDC_DDCMCTL1, DDCM_TRI)) {
    dev_err(dev, "ddc line is busy!\n");
    return -EBUSY;
    }
    sif_write_mask(ddc, DDC_DDCMCTL0, DDCM_CLK_DIV_MASK,
    DDCM_CLK_DIV_OFFSET, SIF1_CLOK);
    for (i = 0; i < num; i++) {
    struct i2c_msg *msg = &msgs[i];
    dev_dbg(dev, "i2c msg, adr:0x%x, flags:%d, len :0x%x\n",
    msg.addr, msg.flags, msg.len);
    if (msg.flags & I2C_M_RD)
    ret = mtk_hdmi_ddc_read_msg(ddc, msg);
    else
    ret = mtk_hdmi_ddc_write_msg(ddc, msg);
    if (ret < 0)
    goto xfer_end;
    }
    ddcm_trigger_mode(ddc, DDCM_STOP);
    return i;
    xfer_end:
    ddcm_trigger_mode(ddc, DDCM_STOP);
    dev_err(dev, "ddc failed!\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_ddc_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 mtk_hdmi_ddc_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm mtk_hdmi_ddc_algorithm = {
    .master_xfer = mtk_hdmi_ddc_xfer,
    .functionality = mtk_hdmi_ddc_func,
    };
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_ddc_probe(pdev: *mut platform_device) -> c_int {
    static int mtk_hdmi_ddc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mtk_hdmi_ddc *ddc;
    struct resource *mem;
    int ret;
    ddc = devm_kzalloc(dev, sizeof(struct mtk_hdmi_ddc), GFP_KERNEL);
    if (!ddc)
    return -ENOMEM;
    ddc.clk = devm_clk_get(dev, "ddc-i2c");
    if (IS_ERR(ddc.clk))
    return dev_err_probe(dev, PTR_ERR(ddc.clk),
    "get ddc_clk failed\n");
    ddc.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mem);
    if (IS_ERR(ddc.regs))
    return PTR_ERR(ddc.regs);
    ret = clk_prepare_enable(ddc.clk);
    if (ret)
    return dev_err_probe(dev, ret, "enable ddc clk failed!\n");
    strscpy(ddc.adap.name, "mediatek-hdmi-ddc", sizeof(ddc.adap.name));
    ddc.adap.owner = THIS_MODULE;
    ddc.adap.algo = &mtk_hdmi_ddc_algorithm;
    ddc.adap.retries = 3;
    ddc.adap.dev.of_node = dev.of_node;
    ddc.adap.algo_data = ddc;
    ddc.adap.dev.parent = &pdev.dev;
    ret = i2c_add_adapter(&ddc.adap);
    if (ret < 0) {
    clk_disable_unprepare(ddc.clk);
    return dev_err_probe(dev, ret, "failed to add bus to i2c core\n");
    }
    platform_set_drvdata(pdev, ddc);
    dev_dbg(dev, "ddc.adap: %p\n", &ddc.adap);
    dev_dbg(dev, "ddc.clk: %p\n", ddc.clk);
    dev_dbg(dev, "physical adr: %pa, end: %pa\n", &mem.start,
    &mem.end);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_hdmi_ddc_remove(pdev: *mut platform_device) {
    static void mtk_hdmi_ddc_remove(struct platform_device *pdev)
    {
    struct mtk_hdmi_ddc *ddc = platform_get_drvdata(pdev);
    i2c_del_adapter(&ddc.adap);
    clk_disable_unprepare(ddc.clk);
    }
    static const struct of_device_id mtk_hdmi_ddc_match[] = {
    { .compatible = "mediatek,mt8173-hdmi-ddc", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mtk_hdmi_ddc_match);
    static struct platform_driver mtk_hdmi_ddc_driver = {
    .probe = mtk_hdmi_ddc_probe,
    .remove = mtk_hdmi_ddc_remove,
    .driver = {
    .name = "mediatek-hdmi-ddc",
    .of_match_table = mtk_hdmi_ddc_match,
    },
    };
    module_platform_driver(mtk_hdmi_ddc_driver);
    MODULE_AUTHOR("Jie Qiu <jie.qiu@mediatek.com>");
    MODULE_DESCRIPTION("MediaTek HDMI DDC Driver");
    MODULE_LICENSE("GPL v2");
