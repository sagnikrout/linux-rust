//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-ingenic.c
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
// SPI bus driver for the Ingenic SoCs
// Copyright (c) 2017-2021 Artur Rojek <contact@artur-rojek.eu>
// Copyright (c) 2017-2021 Paul Cercueil <paul@crapouillou.net>
// Copyright (c) 2022 周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>
//

pub const REG_SSIDR: c_uint = 0x0;
pub const REG_SSICR0: c_uint = 0x4;
pub const REG_SSICR1: c_uint = 0x8;
pub const REG_SSISR: c_uint = 0xc;
pub const REG_SSIGR: c_uint = 0x18;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jz_soc_info {
    pub bits_per_word_mask: u32,
    pub flen_field: reg_field,
    pub has_trendian: bool,
    pub max_speed_hz: c_uint,
    pub max_native_cs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_spi {
    pub soc_info: *const jz_soc_info,
    pub clk: *mut clk,
    pub mem_res: *mut resource,
    pub map: *mut regmap,
    pub flen_field: *mut regmap_field,
}

    static int spi_ingenic_wait(struct ingenic_spi *priv,
    unsigned long mask,
    bool condition)
    {
    unsigned int val;
    return regmap_read_poll_timeout(priv.map, REG_SSISR, val,
    !!(val & mask) == condition,
    100, 10000);
    }
#[no_mangle]
unsafe extern "C" fn spi_ingenic_set_cs(spi: *mut spi_device, disable: bool) {
    static void spi_ingenic_set_cs(struct spi_device *spi, bool disable)
    {
    struct ingenic_spi *priv = spi_controller_get_devdata(spi.controller);
    if (disable) {
    regmap_clear_bits(priv.map, REG_SSICR1, REG_SSICR1_UNFIN);
    regmap_clear_bits(priv.map, REG_SSISR,
    REG_SSISR_UNDR | REG_SSISR_OVER);
    spi_ingenic_wait(priv, REG_SSISR_END, true);
    } else {
    regmap_set_bits(priv.map, REG_SSICR1, REG_SSICR1_UNFIN);
    }
    regmap_set_bits(priv.map, REG_SSICR0,
    REG_SSICR0_RFLUSH | REG_SSICR0_TFLUSH);
    }
    static void spi_ingenic_prepare_transfer(struct ingenic_spi *priv,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    let mut clk_hz: c_ulong = clk_get_rate(priv.clk);
    u32 cdiv, speed_hz = xfer.speed_hz ?: spi.max_speed_hz,
    bits_per_word = xfer.bits_per_word ?: spi.bits_per_word;
    cdiv = clk_hz / (speed_hz * 2);
    cdiv = clamp(cdiv, 1u, 0x100u) - 1;
    regmap_write(priv.map, REG_SSIGR, cdiv);
    regmap_field_write(priv.flen_field, bits_per_word - 2);
    }
#[no_mangle]
unsafe extern "C" fn spi_ingenic_finalize_transfer(controller: *mut c_void) {
    static void spi_ingenic_finalize_transfer(void *controller)
    {
    spi_finalize_current_transfer(controller);
    }
    static struct dma_async_tx_descriptor *
    spi_ingenic_prepare_dma(struct spi_controller *ctlr, struct dma_chan *chan,
    struct sg_table *sg, enum dma_transfer_direction dir,
    unsigned int bits)
    {
    struct ingenic_spi *priv = spi_controller_get_devdata(ctlr);
    struct dma_slave_config cfg = {
    .direction = dir,
    .src_addr = priv.mem_res.start + REG_SSIDR,
    .dst_addr = priv.mem_res.start + REG_SSIDR,
    };
    struct dma_async_tx_descriptor *desc;
    dma_cookie_t cookie;
    int ret;
    if (bits > 16) {
    cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    cfg.src_maxburst = cfg.dst_maxburst = 4;
    } else if (bits > 8) {
    cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    cfg.src_maxburst = cfg.dst_maxburst = 2;
    } else {
    cfg.src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    cfg.dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    cfg.src_maxburst = cfg.dst_maxburst = 1;
    }
    ret = dmaengine_slave_config(chan, &cfg);
    if (ret)
    return ERR_PTR(ret);
    desc = dmaengine_prep_slave_sg(chan, sg.sgl, sg.nents, dir,
    DMA_PREP_INTERRUPT);
    if (!desc)
    return ERR_PTR(-ENOMEM);
    if (dir == DMA_DEV_TO_MEM) {
    desc.callback = spi_ingenic_finalize_transfer;
    desc.callback_param = ctlr;
    }
    cookie = dmaengine_submit(desc);
    ret = dma_submit_error(cookie);
    if (ret) {
    dmaengine_desc_free(desc);
    return ERR_PTR(ret);
    }
    return desc;
    }
    static int spi_ingenic_dma_tx(struct spi_controller *ctlr,
    struct spi_transfer *xfer, unsigned int bits)
    {
    struct dma_async_tx_descriptor *rx_desc, *tx_desc;
    rx_desc = spi_ingenic_prepare_dma(ctlr, ctlr.dma_rx,
    &xfer.rx_sg, DMA_DEV_TO_MEM, bits);
    if (IS_ERR(rx_desc))
    return PTR_ERR(rx_desc);
    tx_desc = spi_ingenic_prepare_dma(ctlr, ctlr.dma_tx,
    &xfer.tx_sg, DMA_MEM_TO_DEV, bits);
    if (IS_ERR(tx_desc)) {
    dmaengine_terminate_async(ctlr.dma_rx);
    dmaengine_desc_free(rx_desc);
    return PTR_ERR(tx_desc);
    }
    dma_async_issue_pending(ctlr.dma_rx);
    dma_async_issue_pending(ctlr.dma_tx);
    return 1;
    }

    static int spi_ingenic_tx##x(struct ingenic_spi *priv,				\
    struct spi_transfer *xfer)				\
    {										\
    unsigned int count = xfer.len / (x / 8);				\
    unsigned int prefill = min(count, SPI_INGENIC_FIFO_SIZE);		\
    const u##x *tx_buf = xfer.tx_buf;					\
    u##x *rx_buf = xfer.rx_buf;						\
    unsigned int i, val;							\
    int err;								\
    \
// Fill up the TX fifo */						\
    for (i = 0; i < prefill; i++) {						\
    val = tx_buf ? tx_buf[i] : 0;					\
    \
    regmap_write(priv.map, REG_SSIDR, val);			\
    }									\
    \
    for (i = 0; i < count; i++) {						\
    err = spi_ingenic_wait(priv, REG_SSISR_RFE, false);		\
    if (err)							\
    return err;						\
    \
    regmap_read(priv.map, REG_SSIDR, &val);			\
    if (rx_buf)							\
    rx_buf[i] = val;					\
    \
    if (i < count - prefill) {					\
    val = tx_buf ? tx_buf[i + prefill] : 0;			\
    \
    regmap_write(priv.map, REG_SSIDR, val);		\
    }								\
    }									\
    \
    return 0;								\
    }
    SPI_INGENIC_TX(8)
    SPI_INGENIC_TX(16)
    SPI_INGENIC_TX(32)

    static int spi_ingenic_transfer_one(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct ingenic_spi *priv = spi_controller_get_devdata(ctlr);
    let mut bits: c_uint = xfer.bits_per_word ?: spi.bits_per_word;
    spi_ingenic_prepare_transfer(priv, spi, xfer);
    if (spi_xfer_is_dma_mapped(ctlr, spi, xfer))
    return spi_ingenic_dma_tx(ctlr, xfer, bits);
    if (bits > 16)
    return spi_ingenic_tx32(priv, xfer);
    if (bits > 8)
    return spi_ingenic_tx16(priv, xfer);
    return spi_ingenic_tx8(priv, xfer);
    }
    static int spi_ingenic_prepare_message(struct spi_controller *ctlr,
    struct spi_message *message)
    {
    struct ingenic_spi *priv = spi_controller_get_devdata(ctlr);
    struct spi_device *spi = message.spi;
    let mut cs: c_uint = REG_SSICR1_FRMHL << spi_get_chipselect(spi, 0);
    let mut ssicr0_mask: c_uint = REG_SSICR0_LOOP | REG_SSICR0_FSEL;
    let mut ssicr1_mask: c_uint = REG_SSICR1_PHA | REG_SSICR1_POL | cs;
    let mut ssicr0: c_uint = 0, ssicr1 = 0;
    if (priv.soc_info.has_trendian) {
    ssicr0_mask |= REG_SSICR0_RENDIAN_LSB | REG_SSICR0_TENDIAN_LSB;
    if (spi.mode & SPI_LSB_FIRST)
    ssicr0 |= REG_SSICR0_RENDIAN_LSB | REG_SSICR0_TENDIAN_LSB;
    } else {
    ssicr1_mask |= REG_SSICR1_LFST;
    if (spi.mode & SPI_LSB_FIRST)
    ssicr1 |= REG_SSICR1_LFST;
    }
    if (spi.mode & SPI_LOOP)
    ssicr0 |= REG_SSICR0_LOOP;
    if (spi_get_chipselect(spi, 0))
    ssicr0 |= REG_SSICR0_FSEL;
    if (spi.mode & SPI_CPHA)
    ssicr1 |= REG_SSICR1_PHA;
    if (spi.mode & SPI_CPOL)
    ssicr1 |= REG_SSICR1_POL;
    if (spi.mode & SPI_CS_HIGH)
    ssicr1 |= cs;
    regmap_update_bits(priv.map, REG_SSICR0, ssicr0_mask, ssicr0);
    regmap_update_bits(priv.map, REG_SSICR1, ssicr1_mask, ssicr1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_ingenic_prepare_hardware(ctlr: *mut spi_controller) -> c_int {
    static int spi_ingenic_prepare_hardware(struct spi_controller *ctlr)
    {
    struct ingenic_spi *priv = spi_controller_get_devdata(ctlr);
    int ret;
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    regmap_write(priv.map, REG_SSICR0, REG_SSICR0_EACLRUN);
    regmap_write(priv.map, REG_SSICR1, 0);
    regmap_write(priv.map, REG_SSISR, 0);
    regmap_set_bits(priv.map, REG_SSICR0, REG_SSICR0_SSIE);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_ingenic_unprepare_hardware(ctlr: *mut spi_controller) -> c_int {
    static int spi_ingenic_unprepare_hardware(struct spi_controller *ctlr)
    {
    struct ingenic_spi *priv = spi_controller_get_devdata(ctlr);
    regmap_clear_bits(priv.map, REG_SSICR0, REG_SSICR0_SSIE);
    clk_disable_unprepare(priv.clk);
    return 0;
    }
    static bool spi_ingenic_can_dma(struct spi_controller *ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct dma_slave_caps caps;
    int ret;
    ret = dma_get_slave_caps(ctlr.dma_tx, &caps);
    if (ret) {
    dev_err(&spi.dev, "Unable to get slave caps: %d\n", ret);
    return false;
    }
    return !caps.max_sg_burst ||
    xfer.len <= caps.max_sg_burst * SPI_INGENIC_FIFO_SIZE;
    }
    static int spi_ingenic_request_dma(struct spi_controller *ctlr,
    struct device *dev)
    {
    struct dma_chan *chan;
    chan = dma_request_chan(dev, "tx");
    if (IS_ERR(chan))
    return PTR_ERR(chan);
    ctlr.dma_tx = chan;
    chan = dma_request_chan(dev, "rx");
    if (IS_ERR(chan))
    return PTR_ERR(chan);
    ctlr.dma_rx = chan;
    ctlr.can_dma = spi_ingenic_can_dma;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn spi_ingenic_release_dma(data: *mut c_void) {
    static void spi_ingenic_release_dma(void *data)
    {
    struct spi_controller *ctlr = data;
    if (ctlr.dma_tx)
    dma_release_channel(ctlr.dma_tx);
    if (ctlr.dma_rx)
    dma_release_channel(ctlr.dma_rx);
    }
    static const struct regmap_config spi_ingenic_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = REG_SSIGR,
    };
#[no_mangle]
unsafe extern "C" fn spi_ingenic_probe(pdev: *mut platform_device) -> c_int {
    static int spi_ingenic_probe(struct platform_device *pdev)
    {
    const struct jz_soc_info *pdata;
    struct device *dev = &pdev.dev;
    struct spi_controller *ctlr;
    struct ingenic_spi *priv;
    void __iomem *base;
    int num_cs, ret;
    pdata = of_device_get_match_data(dev);
    if (!pdata) {
    dev_err(dev, "Missing platform data.\n");
    return -EINVAL;
    }
    ctlr = devm_spi_alloc_host(dev, sizeof(*priv));
    if (!ctlr) {
    dev_err(dev, "Unable to allocate SPI controller.\n");
    return -ENOMEM;
    }
    priv = spi_controller_get_devdata(ctlr);
    priv.soc_info = pdata;
    priv.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "Unable to get clock.\n");
    }
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &priv.mem_res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    priv.map = devm_regmap_init_mmio(dev, base, &spi_ingenic_regmap_config);
    if (IS_ERR(priv.map))
    return PTR_ERR(priv.map);
    priv.flen_field = devm_regmap_field_alloc(dev, priv.map,
    pdata.flen_field);
    if (IS_ERR(priv.flen_field))
    return PTR_ERR(priv.flen_field);
    if (device_property_read_u32(dev, "num-cs", &num_cs))
    num_cs = pdata.max_native_cs;
    platform_set_drvdata(pdev, ctlr);
    ctlr.prepare_transfer_hardware = spi_ingenic_prepare_hardware;
    ctlr.unprepare_transfer_hardware = spi_ingenic_unprepare_hardware;
    ctlr.prepare_message = spi_ingenic_prepare_message;
    ctlr.set_cs = spi_ingenic_set_cs;
    ctlr.transfer_one = spi_ingenic_transfer_one;
    ctlr.mode_bits = SPI_MODE_3 | SPI_LSB_FIRST | SPI_LOOP | SPI_CS_HIGH;
    ctlr.flags = SPI_CONTROLLER_MUST_RX | SPI_CONTROLLER_MUST_TX;
    ctlr.max_dma_len = SPI_INGENIC_FIFO_SIZE;
    ctlr.bits_per_word_mask = pdata.bits_per_word_mask;
    ctlr.min_speed_hz = 7200;
    ctlr.max_speed_hz = pdata.max_speed_hz;
    ctlr.use_gpio_descriptors = true;
    ctlr.max_native_cs = pdata.max_native_cs;
    ctlr.num_chipselect = num_cs;
    if (spi_ingenic_request_dma(ctlr, dev))
    dev_warn(dev, "DMA not available.\n");
    ret = devm_add_action_or_reset(dev, spi_ingenic_release_dma, ctlr);
    if (ret) {
    dev_err(dev, "Unable to add action.\n");
    return ret;
    }
    ret = devm_spi_register_controller(dev, ctlr);
    if (ret)
    dev_err(dev, "Unable to register SPI controller.\n");
    return ret;
    }
    static const struct jz_soc_info jz4750_soc_info = {
    .bits_per_word_mask = SPI_BPW_RANGE_MASK(2, 17),
    .flen_field = REG_FIELD(REG_SSICR1, 4, 7),
    .has_trendian = false,
    .max_speed_hz = 54000000,
    .max_native_cs = 2,
    };
    static const struct jz_soc_info jz4780_soc_info = {
    .bits_per_word_mask = SPI_BPW_RANGE_MASK(2, 32),
    .flen_field = REG_FIELD(REG_SSICR1, 3, 7),
    .has_trendian = true,
    .max_speed_hz = 54000000,
    .max_native_cs = 2,
    };
    static const struct jz_soc_info x1000_soc_info = {
    .bits_per_word_mask = SPI_BPW_RANGE_MASK(2, 32),
    .flen_field = REG_FIELD(REG_SSICR1, 3, 7),
    .has_trendian = true,
    .max_speed_hz = 50000000,
    .max_native_cs = 2,
    };
    static const struct jz_soc_info x2000_soc_info = {
    .bits_per_word_mask = SPI_BPW_RANGE_MASK(2, 32),
    .flen_field = REG_FIELD(REG_SSICR1, 3, 7),
    .has_trendian = true,
    .max_speed_hz = 50000000,
    .max_native_cs = 1,
    };
    static const struct of_device_id spi_ingenic_of_match[] = {
    { .compatible = "ingenic,jz4750-spi", .data = &jz4750_soc_info },
    { .compatible = "ingenic,jz4775-spi", .data = &jz4780_soc_info },
    { .compatible = "ingenic,jz4780-spi", .data = &jz4780_soc_info },
    { .compatible = "ingenic,x1000-spi", .data = &x1000_soc_info },
    { .compatible = "ingenic,x2000-spi", .data = &x2000_soc_info },
    {}
    };
    MODULE_DEVICE_TABLE(of, spi_ingenic_of_match);
    static struct platform_driver spi_ingenic_driver = {
    .driver = {
    .name = "spi-ingenic",
    .of_match_table = spi_ingenic_of_match,
    },
    .probe = spi_ingenic_probe,
    };
    module_platform_driver(spi_ingenic_driver);
    MODULE_DESCRIPTION("SPI bus driver for the Ingenic SoCs");
    MODULE_AUTHOR("Artur Rojek <contact@artur-rojek.eu>");
    MODULE_AUTHOR("Paul Cercueil <paul@crapouillou.net>");
    MODULE_AUTHOR("周琰杰 (Zhou Yanjie) <zhouyanjie@wanyeetech.com>");
    MODULE_LICENSE("GPL");
