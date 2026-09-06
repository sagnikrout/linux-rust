//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-imx.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2002 Motorola GSG-China
//
// Author:
// Darius Augulis, Teltonika Inc.
//
// Desc.:
// Implementation of I2C Adapter/Algorithm Driver
// for I2C Bus integrated in Freescale i.MX/MXC processors
//
// Derived from Motorola GSG China I2C example driver
//
// Copyright (C) 2005 Torsten Koschorrek <koschorrek at synertronixx.de
// Copyright (C) 2005 Matthias Blaschke <blaschke at synertronixx.de
// Copyright (C) 2007 RightHand Technologies, Inc.
// Copyright (C) 2008 Darius Augulis <darius.augulis at teltonika.lt>
//
// Copyright 2013 Freescale Semiconductor, Inc.
// Copyright 2020, 2024 NXP
//

// This will be the driver name the kernel reports

//
// Enable DMA if transfer byte size is bigger than this threshold.
// As the hardware request, it must bigger than 4 bytes.\
// I have set '16' here, maybe it's not the best but I think it's
// the appropriate.
//
pub const DMA_THRESHOLD: c_int = 16;
pub const DMA_TIMEOUT: c_int = 1000;
// IMX I2C registers:
// the I2C register offset is different between SoCs,
// to provide support for all these chips, split the
// register offset into a fixed base address and a
// variable shift value, then the full register offset
// will be calculated by
// reg_off = ( reg_base_addr << reg_shift)
//
pub const IMX_I2C_IADR: c_uint = 0x00	/* i2c slave address */;
pub const IMX_I2C_IFDR: c_uint = 0x01	/* i2c frequency divider */;
pub const IMX_I2C_I2CR: c_uint = 0x02	/* i2c control */;
pub const IMX_I2C_I2SR: c_uint = 0x03	/* i2c status */;
pub const IMX_I2C_I2DR: c_uint = 0x04	/* i2c transfer data */;
//
// All of the layerscape series SoCs support IBIC register.
//
pub const IMX_I2C_IBIC: c_uint = 0x05    /* i2c bus interrupt config */;
pub const IMX_I2C_REGSHIFT: c_int = 2;
pub const VF610_I2C_REGSHIFT: c_int = 0;
pub const S32G_I2C_REGSHIFT: c_int = 0;
// Bits of IMX I2C registers
pub const I2SR_RXAK: c_uint = 0x01;
pub const I2SR_IIF: c_uint = 0x02;
pub const I2SR_SRW: c_uint = 0x04;
pub const I2SR_IAL: c_uint = 0x10;
pub const I2SR_IBB: c_uint = 0x20;
pub const I2SR_IAAS: c_uint = 0x40;
pub const I2SR_ICF: c_uint = 0x80;
pub const I2CR_DMAEN: c_uint = 0x02;
pub const I2CR_RSTA: c_uint = 0x04;
pub const I2CR_TXAK: c_uint = 0x08;
pub const I2CR_MTX: c_uint = 0x10;
pub const I2CR_MSTA: c_uint = 0x20;
pub const I2CR_IIEN: c_uint = 0x40;
pub const I2CR_IEN: c_uint = 0x80;
pub const IBIC_BIIE: c_uint = 0x80 /* Bus idle interrupt enable */;
// register bits different operating codes definition:
// 1) I2SR: Interrupt flags clear operation differ between SoCs:
// - write zero to clear(w0c) INT flag on i.MX,
// - but write one to clear(w1c) INT flag on Vybrid.
// 2) I2CR: I2C module enable operation also differ between SoCs:
// - set I2CR_IEN bit enable the module on i.MX,
// - but clear I2CR_IEN bit enable the module on Vybrid.
//
pub const I2SR_CLR_OPCODE_W0C: c_uint = 0x0;

pub const I2CR_IEN_OPCODE_0: c_uint = 0x0;

//
// sorted list of clock divider, register value pairs
// taken from table 26-5, p.26-9, Freescale i.MX
// Integrated Portable System Processor Reference Manual
// Document Number: MC9328MXLRM, Rev. 5.1, 06/2007
//
// Duplicated divider values removed from list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_i2c_clk_pair {
    pub div: u16,
    pub val: u16,
}

    static struct imx_i2c_clk_pair imx_i2c_clk_div[] = {
    { 22,	0x20 }, { 24,	0x21 }, { 26,	0x22 }, { 28,	0x23 },
    { 30,	0x00 },	{ 32,	0x24 }, { 36,	0x25 }, { 40,	0x26 },
    { 42,	0x03 }, { 44,	0x27 },	{ 48,	0x28 }, { 52,	0x05 },
    { 56,	0x29 }, { 60,	0x06 }, { 64,	0x2A },	{ 72,	0x2B },
    { 80,	0x2C }, { 88,	0x09 }, { 96,	0x2D }, { 104,	0x0A },
    { 112,	0x2E }, { 128,	0x2F }, { 144,	0x0C }, { 160,	0x30 },
    { 192,	0x31 },	{ 224,	0x32 }, { 240,	0x0F }, { 256,	0x33 },
    { 288,	0x10 }, { 320,	0x34 },	{ 384,	0x35 }, { 448,	0x36 },
    { 480,	0x13 }, { 512,	0x37 }, { 576,	0x14 },	{ 640,	0x38 },
    { 768,	0x39 }, { 896,	0x3A }, { 960,	0x17 }, { 1024,	0x3B },
    { 1152,	0x18 }, { 1280,	0x3C }, { 1536,	0x3D }, { 1792,	0x3E },
    { 1920,	0x1B },	{ 2048,	0x3F }, { 2304,	0x1C }, { 2560,	0x1D },
    { 3072,	0x1E }, { 3840,	0x1F }
    };
// Vybrid VF610 clock divider, register value pairs
    static struct imx_i2c_clk_pair vf610_i2c_clk_div[] = {
    { 20,   0x00 }, { 22,   0x01 }, { 24,   0x02 }, { 26,   0x03 },
    { 28,   0x04 }, { 30,   0x05 }, { 32,   0x09 }, { 34,   0x06 },
    { 36,   0x0A }, { 40,   0x07 }, { 44,   0x0C }, { 48,   0x0D },
    { 52,   0x43 }, { 56,   0x0E }, { 60,   0x45 }, { 64,   0x12 },
    { 68,   0x0F }, { 72,   0x13 }, { 80,   0x14 }, { 88,   0x15 },
    { 96,   0x19 }, { 104,  0x16 }, { 112,  0x1A }, { 128,  0x17 },
    { 136,  0x4F }, { 144,  0x1C }, { 160,  0x1D }, { 176,  0x55 },
    { 192,  0x1E }, { 208,  0x56 }, { 224,  0x22 }, { 228,  0x24 },
    { 240,  0x1F }, { 256,  0x23 }, { 288,  0x5C }, { 320,  0x25 },
    { 384,  0x26 }, { 448,  0x2A }, { 480,  0x27 }, { 512,  0x2B },
    { 576,  0x2C }, { 640,  0x2D }, { 768,  0x31 }, { 896,  0x32 },
    { 960,  0x2F }, { 1024, 0x33 }, { 1152, 0x34 }, { 1280, 0x35 },
    { 1536, 0x36 }, { 1792, 0x3A }, { 1920, 0x37 }, { 2048, 0x3B },
    { 2304, 0x3C }, { 2560, 0x3D }, { 3072, 0x3E }, { 3584, 0x7A },
    { 3840, 0x3F }, { 4096, 0x7B }, { 5120, 0x7D }, { 6144, 0x7E },
    };
// S32G2/S32G3 clock divider, register value pairs
    static struct imx_i2c_clk_pair s32g2_i2c_clk_div[] = {
    { 34,    0x00 }, { 36,    0x01 }, { 38,    0x02 }, { 40,    0x03 },
    { 42,    0x04 }, { 44,    0x05 }, { 46,    0x06 }, { 48,    0x09 },
    { 52,    0x0A }, { 54,    0x07 }, { 56,    0x0B }, { 60,    0x0C },
    { 64,    0x0D }, { 68,    0x40 }, { 72,    0x0E }, { 76,    0x42 },
    { 80,    0x12 }, { 84,    0x0F }, { 88,    0x13 }, { 96,    0x14 },
    { 104,   0x15 }, { 108,   0x47 }, { 112,   0x19 }, { 120,   0x16 },
    { 128,   0x1A }, { 136,   0x80 }, { 144,   0x17 }, { 152,   0x82 },
    { 160,   0x1C }, { 168,   0x84 }, { 176,   0x1D }, { 192,   0x21 },
    { 208,   0x1E }, { 216,   0x87 }, { 224,   0x22 }, { 240,   0x56 },
    { 256,   0x1F }, { 288,   0x24 }, { 320,   0x25 }, { 336,   0x8F },
    { 352,   0x93 }, { 356,   0x5D }, { 358,   0x98 }, { 384,   0x26 },
    { 416,   0x56 }, { 448,   0x2A }, { 480,   0x27 }, { 512,   0x2B },
    { 576,   0x2C }, { 640,   0x2D }, { 704,   0x9D }, { 768,   0x2E },
    { 832,   0x9D }, { 896,   0x32 }, { 960,   0x2F }, { 1024,  0x33 },
    { 1152,  0x34 }, { 1280,  0x35 }, { 1536,  0x36 }, { 1792,  0x3A },
    { 1920,  0x37 }, { 2048,  0x3B }, { 2304,  0x74 }, { 2560,  0x3D },
    { 3072,  0x3E }, { 3584,  0x7A }, { 3840,  0x3F }, { 4096,  0x7B },
    { 4608,  0x7C }, { 5120,  0x7D }, { 6144,  0x7E }, { 7168,  0xBA },
    { 7680,  0x7F }, { 8192,  0xBB }, { 9216,  0xBC }, { 10240, 0xBD },
    { 12288, 0xBE }, { 15360, 0xBF },
    };
    enum imx_i2c_type {
    IMX1_I2C,
    IMX21_I2C,
    S32G_I2C,
    VF610_I2C,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_i2c_hwdata {
    pub devtype: enum imx_i2c_type,
    pub regshift: c_uint,
    pub clk_div: *mut imx_i2c_clk_pair,
    pub ndivs: c_uint,
    pub i2sr_clr_opcode: c_uint,
    pub i2cr_ien_opcode: c_uint,
//
// Errata ERR007805 or e7805:
// I2C: When the I2C clock speed is configured for 400 kHz,
// the SCL low period violates the I2C spec of 1.3 uS min.
//
    pub has_err007805: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_i2c_dma {
    pub chan_tx: *mut dma_chan,
    pub chan_rx: *mut dma_chan,
    pub chan_using: *mut dma_chan,
    pub cmd_complete: completion,
    pub dma_buf: dma_addr_t,
    pub dma_len: c_uint,
    pub dma_transfer_dir: enum dma_transfer_direction,
    pub dma_data_dir: enum dma_data_direction,
}

    enum imx_i2c_state {
    IMX_I2C_STATE_DONE,
    IMX_I2C_STATE_FAILED,
    IMX_I2C_STATE_WRITE,
    IMX_I2C_STATE_DMA,
    IMX_I2C_STATE_READ,
    IMX_I2C_STATE_READ_CONTINUE,
    IMX_I2C_STATE_READ_BLOCK_DATA,
    IMX_I2C_STATE_READ_BLOCK_DATA_LEN,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_i2c_struct {
    pub adapter: i2c_adapter,
    pub clk: *mut clk,
    pub clk_change_nb: notifier_block,
    pub base: *mut void __iomem,
    pub queue: wait_queue_head_t,
    pub i2csr: c_ulong,
    pub disable_delay: c_uint,
    pub stopped: c_int,
    pub /: *mut *mut unsigned int ifdr; / IMX_I2C_IFDR,
    pub cur_clk: c_uint,
    pub bitrate: c_uint,
    pub hwdata: *const imx_i2c_hwdata,
    pub rinfo: i2c_bus_recovery_info,
    pub dma: *mut imx_i2c_dma,
    pub slave: *mut i2c_client,
    pub last_slave_event: enum i2c_slave_event,
    pub msg: *mut i2c_msg,
    pub msg_buf_idx: c_uint,
    pub isr_result: c_int,
    pub is_lastmsg: bool,
    pub state: enum imx_i2c_state,
    pub multi_master: bool,
// For checking slave events.
    pub slave_lock: spinlock_t,
    pub slave_timer: hrtimer,
}

    static const struct imx_i2c_hwdata imx1_i2c_hwdata = {
    .devtype		= IMX1_I2C,
    .regshift		= IMX_I2C_REGSHIFT,
    .clk_div		= imx_i2c_clk_div,
    .ndivs			= ARRAY_SIZE(imx_i2c_clk_div),
    .i2sr_clr_opcode	= I2SR_CLR_OPCODE_W0C,
    .i2cr_ien_opcode	= I2CR_IEN_OPCODE_1,
    };
    static const struct imx_i2c_hwdata imx21_i2c_hwdata = {
    .devtype		= IMX21_I2C,
    .regshift		= IMX_I2C_REGSHIFT,
    .clk_div		= imx_i2c_clk_div,
    .ndivs			= ARRAY_SIZE(imx_i2c_clk_div),
    .i2sr_clr_opcode	= I2SR_CLR_OPCODE_W0C,
    .i2cr_ien_opcode	= I2CR_IEN_OPCODE_1,
    };
    static const struct imx_i2c_hwdata imx6_i2c_hwdata = {
    .devtype		= IMX21_I2C,
    .regshift		= IMX_I2C_REGSHIFT,
    .clk_div		= imx_i2c_clk_div,
    .ndivs			= ARRAY_SIZE(imx_i2c_clk_div),
    .i2sr_clr_opcode	= I2SR_CLR_OPCODE_W0C,
    .i2cr_ien_opcode	= I2CR_IEN_OPCODE_1,
    .has_err007805		= true,
    };
    static struct imx_i2c_hwdata vf610_i2c_hwdata = {
    .devtype		= VF610_I2C,
    .regshift		= VF610_I2C_REGSHIFT,
    .clk_div		= vf610_i2c_clk_div,
    .ndivs			= ARRAY_SIZE(vf610_i2c_clk_div),
    .i2sr_clr_opcode	= I2SR_CLR_OPCODE_W1C,
    .i2cr_ien_opcode	= I2CR_IEN_OPCODE_0,
    };
    static const struct imx_i2c_hwdata s32g2_i2c_hwdata = {
    .devtype		= S32G_I2C,
    .regshift		= S32G_I2C_REGSHIFT,
    .clk_div		= s32g2_i2c_clk_div,
    .ndivs			= ARRAY_SIZE(s32g2_i2c_clk_div),
    .i2sr_clr_opcode	= I2SR_CLR_OPCODE_W1C,
    .i2cr_ien_opcode	= I2CR_IEN_OPCODE_0,
    };
    static const struct platform_device_id imx_i2c_devtype[] = {
    {
    .name = "imx1-i2c",
    .driver_data = (kernel_ulong_t)&imx1_i2c_hwdata,
    }, {
    .name = "imx21-i2c",
    .driver_data = (kernel_ulong_t)&imx21_i2c_hwdata,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(platform, imx_i2c_devtype);
    static const struct of_device_id i2c_imx_dt_ids[] = {
    { .compatible = "fsl,imx1-i2c", .data = &imx1_i2c_hwdata, },
    { .compatible = "fsl,imx21-i2c", .data = &imx21_i2c_hwdata, },
    { .compatible = "fsl,imx6q-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx6sl-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx6sll-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx6sx-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx6ul-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx7d-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx7s-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx8mm-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx8mn-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx8mp-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,imx8mq-i2c", .data = &imx6_i2c_hwdata, },
    { .compatible = "fsl,vf610-i2c", .data = &vf610_i2c_hwdata, },
    { .compatible = "nxp,s32g2-i2c", .data = &s32g2_i2c_hwdata, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, i2c_imx_dt_ids);
    static const struct acpi_device_id i2c_imx_acpi_ids[] = {
    {"NXP0001", .driver_data = (kernel_ulong_t)&vf610_i2c_hwdata},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, i2c_imx_acpi_ids);
#[no_mangle]
pub unsafe extern "C" fn is_imx1_i2c(i2c_imx: *mut imx_i2c_struct) -> c_int {
    static inline int is_imx1_i2c(struct imx_i2c_struct *i2c_imx)
    {
    return i2c_imx.hwdata.devtype == IMX1_I2C;
    }
#[no_mangle]
pub unsafe extern "C" fn is_vf610_i2c(i2c_imx: *mut imx_i2c_struct) -> c_int {
    static inline int is_vf610_i2c(struct imx_i2c_struct *i2c_imx)
    {
    return i2c_imx.hwdata.devtype == VF610_I2C;
    }
    static inline void imx_i2c_write_reg(unsigned int val,
    struct imx_i2c_struct *i2c_imx, unsigned int reg)
    {
    writeb(val, i2c_imx.base + (reg << i2c_imx.hwdata.regshift));
    }
    static inline unsigned char imx_i2c_read_reg(struct imx_i2c_struct *i2c_imx,
    unsigned int reg)
    {
    return readb(i2c_imx.base + (reg << i2c_imx.hwdata.regshift));
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_clear_irq(i2c_imx: *mut imx_i2c_struct, bits: c_uint) {
    static void i2c_imx_clear_irq(struct imx_i2c_struct *i2c_imx, unsigned int bits)
    {
    unsigned int temp;
//
// i2sr_clr_opcode is the value to clear all interrupts. Here we want to
// clear only <bits>, so we write ~i2sr_clr_opcode with just <bits>
// toggled. This is required because i.MX needs W0C and Vybrid uses W1C.
//
    temp = ~i2c_imx.hwdata.i2sr_clr_opcode ^ bits;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2SR);
    }
// Set up i2c controller register and i2c status register to default value.
#[no_mangle]
unsafe extern "C" fn i2c_imx_reset_regs(i2c_imx: *mut imx_i2c_struct) {
    static void i2c_imx_reset_regs(struct imx_i2c_struct *i2c_imx)
    {
    imx_i2c_write_reg(i2c_imx.hwdata.i2cr_ien_opcode ^ I2CR_IEN,
    i2c_imx, IMX_I2C_I2CR);
    i2c_imx_clear_irq(i2c_imx, I2SR_IIF | I2SR_IAL);
    }
// Functions for DMA support
#[no_mangle]
unsafe extern "C" fn i2c_imx_dma_request(i2c_imx: *mut imx_i2c_struct, phy_addr: dma_addr_t) -> c_int {
    static int i2c_imx_dma_request(struct imx_i2c_struct *i2c_imx, dma_addr_t phy_addr)
    {
    struct imx_i2c_dma *dma;
    let mut dma_sconfig: dma_slave_config = {};
    struct device *dev = i2c_imx.adapter.dev.parent;
    int ret;
    dma = devm_kzalloc(dev, sizeof(*dma), GFP_KERNEL);
    if (!dma)
    return -ENOMEM;
    dma.chan_tx = dma_request_chan(dev, "tx");
    if (IS_ERR(dma.chan_tx)) {
    ret = PTR_ERR(dma.chan_tx);
    if (ret != -ENODEV && ret != -EPROBE_DEFER)
    dev_err(dev, "can't request DMA tx channel (%d)\n", ret);
    goto fail_al;
    }
    dma_sconfig.dst_addr = phy_addr +
    (IMX_I2C_I2DR << i2c_imx.hwdata.regshift);
    dma_sconfig.dst_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    dma_sconfig.dst_maxburst = 1;
    dma_sconfig.direction = DMA_MEM_TO_DEV;
    ret = dmaengine_slave_config(dma.chan_tx, &dma_sconfig);
    if (ret < 0) {
    dev_err(dev, "can't configure tx channel (%d)\n", ret);
    goto fail_tx;
    }
    dma.chan_rx = dma_request_chan(dev, "rx");
    if (IS_ERR(dma.chan_rx)) {
    ret = PTR_ERR(dma.chan_rx);
    if (ret != -ENODEV && ret != -EPROBE_DEFER)
    dev_err(dev, "can't request DMA rx channel (%d)\n", ret);
    goto fail_tx;
    }
    dma_sconfig.src_addr = phy_addr +
    (IMX_I2C_I2DR << i2c_imx.hwdata.regshift);
    dma_sconfig.src_addr_width = DMA_SLAVE_BUSWIDTH_1_BYTE;
    dma_sconfig.src_maxburst = 1;
    dma_sconfig.direction = DMA_DEV_TO_MEM;
    ret = dmaengine_slave_config(dma.chan_rx, &dma_sconfig);
    if (ret < 0) {
    dev_err(dev, "can't configure rx channel (%d)\n", ret);
    goto fail_rx;
    }
    i2c_imx.dma = dma;
    init_completion(&dma.cmd_complete);
    dev_info(dev, "using %s (tx) and %s (rx) for DMA transfers\n",
    dma_chan_name(dma.chan_tx), dma_chan_name(dma.chan_rx));
    return 0;
    fail_rx:
    dma_release_channel(dma.chan_rx);
    fail_tx:
    dma_release_channel(dma.chan_tx);
    fail_al:
    devm_kfree(dev, dma);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_dma_callback(arg: *mut c_void) {
    static void i2c_imx_dma_callback(void *arg)
    {
    struct imx_i2c_struct *i2c_imx = (struct imx_i2c_struct *)arg;
    struct imx_i2c_dma *dma = i2c_imx.dma;
    dma_unmap_single(dma.chan_using.device.dev, dma.dma_buf,
    dma.dma_len, dma.dma_data_dir);
    complete(&dma.cmd_complete);
    }
    static int i2c_imx_dma_xfer(struct imx_i2c_struct *i2c_imx,
    struct i2c_msg *msgs)
    {
    struct imx_i2c_dma *dma = i2c_imx.dma;
    struct dma_async_tx_descriptor *txdesc;
    struct device *dev = &i2c_imx.adapter.dev;
    struct device *chan_dev = dma.chan_using.device.dev;
    dma.dma_buf = dma_map_single(chan_dev, msgs.buf,
    dma.dma_len, dma.dma_data_dir);
    if (dma_mapping_error(chan_dev, dma.dma_buf)) {
    dev_err(dev, "DMA mapping failed\n");
    goto err_map;
    }
    txdesc = dmaengine_prep_slave_single(dma.chan_using, dma.dma_buf,
    dma.dma_len, dma.dma_transfer_dir,
    DMA_PREP_INTERRUPT | DMA_CTRL_ACK);
    if (!txdesc) {
    dev_err(dev, "Not able to get desc for DMA xfer\n");
    goto err_desc;
    }
    reinit_completion(&dma.cmd_complete);
    txdesc.callback = i2c_imx_dma_callback;
    txdesc.callback_param = i2c_imx;
    if (dma_submit_error(dmaengine_submit(txdesc))) {
    dev_err(dev, "DMA submit failed\n");
    goto err_submit;
    }
    dma_async_issue_pending(dma.chan_using);
    return 0;
    err_submit:
    dmaengine_terminate_sync(dma.chan_using);
    err_desc:
    dma_unmap_single(chan_dev, dma.dma_buf,
    dma.dma_len, dma.dma_data_dir);
    err_map:
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_dma_free(i2c_imx: *mut imx_i2c_struct) {
    static void i2c_imx_dma_free(struct imx_i2c_struct *i2c_imx)
    {
    struct imx_i2c_dma *dma = i2c_imx.dma;
    dma.dma_buf = 0;
    dma.dma_len = 0;
    dma_release_channel(dma.chan_tx);
    dma.chan_tx = core::ptr::null_mut();
    dma_release_channel(dma.chan_rx);
    dma.chan_rx = core::ptr::null_mut();
    dma.chan_using = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_bus_busy(i2c_imx: *mut imx_i2c_struct, for_busy: c_int, atomic: bool) -> c_int {
    static int i2c_imx_bus_busy(struct imx_i2c_struct *i2c_imx, int for_busy, bool atomic)
    {
    let mut multi_master: bool = i2c_imx.multi_master;
    let mut orig_jiffies: c_ulong = jiffies;
    unsigned int temp;
    while (1) {
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2SR);
// check for arbitration lost
    if (multi_master && (temp & I2SR_IAL)) {
    i2c_imx_clear_irq(i2c_imx, I2SR_IAL);
    return -EAGAIN;
    }
    if (for_busy && (!multi_master || (temp & I2SR_IBB))) {
    i2c_imx.stopped = 0;
    break;
    }
    if (!for_busy && !(temp & I2SR_IBB)) {
    i2c_imx.stopped = 1;
    break;
    }
    if (time_after(jiffies, orig_jiffies + msecs_to_jiffies(500))) {
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> I2C bus is busy\n", __func__);
    return -ETIMEDOUT;
    }
    if (atomic)
    udelay(100);
    else
    schedule();
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_trx_complete(i2c_imx: *mut imx_i2c_struct, atomic: bool) -> c_int {
    static int i2c_imx_trx_complete(struct imx_i2c_struct *i2c_imx, bool atomic)
    {
    if (atomic) {
    void __iomem *addr = i2c_imx.base + (IMX_I2C_I2SR << i2c_imx.hwdata.regshift);
    unsigned int regval;
//
// The formula for the poll timeout is documented in the RM
// Rev.5 on page 1878:
// T_min = 10/F_scl
// Set the value hard as it is done for the non-atomic use-case.
// Use 10 kHz for the calculation since this is the minimum
// allowed SMBus frequency. Also add an offset of 100us since it
// turned out that the I2SR_IIF bit isn't set correctly within
// the minimum timeout in polling mode.
//
    readb_poll_timeout_atomic(addr, regval, regval & I2SR_IIF, 5, 1000 + 100);
    i2c_imx.i2csr = regval;
    i2c_imx_clear_irq(i2c_imx, I2SR_IIF | I2SR_IAL);
    } else {
    wait_event_timeout(i2c_imx.queue, i2c_imx.i2csr & I2SR_IIF, HZ / 10);
    }
    if (unlikely(!(i2c_imx.i2csr & I2SR_IIF))) {
    dev_dbg(&i2c_imx.adapter.dev, "<%s> Timeout\n", __func__);
    return -ETIMEDOUT;
    }
// In multi-master mode check for arbitration lost
    if (i2c_imx.multi_master && (i2c_imx.i2csr & I2SR_IAL)) {
    dev_dbg(&i2c_imx.adapter.dev, "<%s> Arbitration lost\n", __func__);
    i2c_imx_clear_irq(i2c_imx, I2SR_IAL);
    i2c_imx.i2csr = 0;
    return -EAGAIN;
    }
    dev_dbg(&i2c_imx.adapter.dev, "<%s> TRX complete\n", __func__);
    i2c_imx.i2csr = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_acked(i2c_imx: *mut imx_i2c_struct) -> c_int {
    static int i2c_imx_acked(struct imx_i2c_struct *i2c_imx)
    {
    if (imx_i2c_read_reg(i2c_imx, IMX_I2C_I2SR) & I2SR_RXAK) {
    dev_dbg(&i2c_imx.adapter.dev, "<%s> No ACK\n", __func__);
    return -ENXIO;  /* No ACK */
    }
    dev_dbg(&i2c_imx.adapter.dev, "<%s> ACK received\n", __func__);
    return 0;
    }
    static int i2c_imx_set_clk(struct imx_i2c_struct *i2c_imx,
    unsigned int i2c_clk_rate)
    {
    struct imx_i2c_clk_pair *i2c_clk_div = i2c_imx.hwdata.clk_div;
    unsigned int div;
    int i;
    if (i2c_imx.hwdata.has_err007805 && i2c_imx.bitrate > 384000) {
    dev_dbg(&i2c_imx.adapter.dev,
    "SoC errata ERR007805 or e7805 applies, bus frequency limited from %d Hz to 384000 Hz.\n",
    i2c_imx.bitrate);
    i2c_imx.bitrate = 384000;
    }
// Divider value calculation
    if (i2c_imx.cur_clk == i2c_clk_rate)
    return 0;
// Keep the denominator of the following program always NOT equal to 0.
    if (!(i2c_clk_rate / 2))
    return -EINVAL;
    i2c_imx.cur_clk = i2c_clk_rate;
    div = DIV_ROUND_UP(i2c_clk_rate, i2c_imx.bitrate);
    if (div < i2c_clk_div[0].div)
    i = 0;
#[no_mangle]
pub unsafe extern "C" fn if(1].div: div > i2c_clk_div[i2c_imx->hwdata->ndivs -) -> else {
    else if (div > i2c_clk_div[i2c_imx.hwdata.ndivs - 1].div)
    i = i2c_imx.hwdata.ndivs - 1;
    else
    for (i = 0; i2c_clk_div[i].div < div; i++)
    ;
// Store divider value
    i2c_imx.ifdr = i2c_clk_div[i].val;
//
// There dummy delay is calculated.
// It should be about one I2C clock period long.
// This delay is used in I2C bus disable function
// to fix chip hardware bug.
//
    i2c_imx.disable_delay = DIV_ROUND_UP(500000U * i2c_clk_div[i].div,
    i2c_clk_rate / 2);

    dev_dbg(&i2c_imx.adapter.dev, "I2C_CLK=%d, REQ DIV=%d\n",
    i2c_clk_rate, div);
    dev_dbg(&i2c_imx.adapter.dev, "IFDR[IC]=0x%x, REAL DIV=%d\n",
    i2c_clk_div[i].val, i2c_clk_div[i].div);

    return 0;
    }
    static int i2c_imx_clk_notifier_call(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    struct clk_notifier_data *ndata = data;
    struct imx_i2c_struct *i2c_imx = container_of(nb,
    struct imx_i2c_struct,
    clk_change_nb);
    let mut ret: c_int = 0;
    if (action & POST_RATE_CHANGE)
    ret = i2c_imx_set_clk(i2c_imx, ndata.new_rate);
    return notifier_from_errno(ret);
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_start(i2c_imx: *mut imx_i2c_struct, atomic: bool) -> c_int {
    static int i2c_imx_start(struct imx_i2c_struct *i2c_imx, bool atomic)
    {
    let mut temp: c_uint = 0;
    int result;
    imx_i2c_write_reg(i2c_imx.ifdr, i2c_imx, IMX_I2C_IFDR);
// Enable I2C controller
    imx_i2c_write_reg(i2c_imx.hwdata.i2sr_clr_opcode, i2c_imx, IMX_I2C_I2SR);
    imx_i2c_write_reg(i2c_imx.hwdata.i2cr_ien_opcode, i2c_imx, IMX_I2C_I2CR);
// Wait controller to be stable
    if (atomic)
    udelay(50);
    else
    usleep_range(50, 150);
// Start I2C transaction
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_MSTA;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    result = i2c_imx_bus_busy(i2c_imx, 1, atomic);
    if (result)
    return result;
    temp |= I2CR_IIEN | I2CR_MTX | I2CR_TXAK;
    if (atomic)
    temp &= ~I2CR_IIEN; /* Disable interrupt */
    temp &= ~I2CR_DMAEN;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_stop(i2c_imx: *mut imx_i2c_struct, atomic: bool) {
    static void i2c_imx_stop(struct imx_i2c_struct *i2c_imx, bool atomic)
    {
    let mut temp: c_uint = 0;
    if (!i2c_imx.stopped) {
// Stop I2C transaction
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    if (!(temp & I2CR_MSTA))
    i2c_imx.stopped = 1;
    temp &= ~(I2CR_MSTA | I2CR_MTX);
    if (i2c_imx.dma)
    temp &= ~I2CR_DMAEN;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    }
    if (is_imx1_i2c(i2c_imx)) {
//
// This delay caused by an i.MXL hardware bug.
// If no (or too short) delay, no "STOP" bit will be generated.
//
    udelay(i2c_imx.disable_delay);
    }
    if (!i2c_imx.stopped)
    i2c_imx_bus_busy(i2c_imx, 0, atomic);
// Disable I2C controller
    temp = i2c_imx.hwdata.i2cr_ien_opcode ^ I2CR_IEN;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    }
//
// Enable bus idle interrupts
// Note: IBIC register will be cleared after disabled i2c module.
// All of layerscape series SoCs support IBIC register.
//
#[no_mangle]
unsafe extern "C" fn i2c_imx_enable_bus_idle(i2c_imx: *mut imx_i2c_struct) {
    static void i2c_imx_enable_bus_idle(struct imx_i2c_struct *i2c_imx)
    {
    if (is_vf610_i2c(i2c_imx)) {
    unsigned int temp;
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_IBIC);
    temp |= IBIC_BIIE;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_IBIC);
    }
    }
    static void i2c_imx_slave_event(struct imx_i2c_struct *i2c_imx,
    enum i2c_slave_event event, u8 *val)
    {
    i2c_slave_event(i2c_imx.slave, event, val);
    i2c_imx.last_slave_event = event;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_slave_finish_op(i2c_imx: *mut imx_i2c_struct) {
    static void i2c_imx_slave_finish_op(struct imx_i2c_struct *i2c_imx)
    {
    let mut val: u8 = 0;
    while (i2c_imx.last_slave_event != I2C_SLAVE_STOP) {
    switch (i2c_imx.last_slave_event) {
    case I2C_SLAVE_READ_REQUESTED:
    i2c_imx_slave_event(i2c_imx, I2C_SLAVE_READ_PROCESSED,
    &val);
    break;
    case I2C_SLAVE_WRITE_REQUESTED:
    case I2C_SLAVE_READ_PROCESSED:
    case I2C_SLAVE_WRITE_RECEIVED:
    i2c_imx_slave_event(i2c_imx, I2C_SLAVE_STOP, &val);
    break;
    case I2C_SLAVE_STOP:
    break;
    }
    }
    }
// Returns true if the timer should be restarted, false if not.
    static irqreturn_t i2c_imx_slave_handle(struct imx_i2c_struct *i2c_imx,
    unsigned int status, unsigned int ctl)
    {
    let mut value: u8 = 0;
    if (status & I2SR_IAL) { /* Arbitration lost */
    i2c_imx_clear_irq(i2c_imx, I2SR_IAL);
    if (!(status & I2SR_IAAS))
    return IRQ_HANDLED;
    }
    if (!(status & I2SR_IBB)) {
// No master on the bus, that could mean a stop condition.
    i2c_imx_slave_finish_op(i2c_imx);
    return IRQ_HANDLED;
    }
    if (!(status & I2SR_ICF))
// Data transfer still in progress, ignore this.
    goto out;
    if (status & I2SR_IAAS) { /* Addressed as a slave */
    i2c_imx_slave_finish_op(i2c_imx);
    if (status & I2SR_SRW) { /* Master wants to read from us*/
    dev_dbg(&i2c_imx.adapter.dev, "read requested");
    i2c_imx_slave_event(i2c_imx,
    I2C_SLAVE_READ_REQUESTED, &value);
// Slave transmit
    ctl |= I2CR_MTX;
    imx_i2c_write_reg(ctl, i2c_imx, IMX_I2C_I2CR);
// Send data
    imx_i2c_write_reg(value, i2c_imx, IMX_I2C_I2DR);
    } else { /* Master wants to write to us */
    dev_dbg(&i2c_imx.adapter.dev, "write requested");
    i2c_imx_slave_event(i2c_imx,
    I2C_SLAVE_WRITE_REQUESTED, &value);
// Slave receive
    ctl &= ~I2CR_MTX;
    imx_i2c_write_reg(ctl, i2c_imx, IMX_I2C_I2CR);
// Dummy read
    imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
    }
    } else if (!(ctl & I2CR_MTX)) { /* Receive mode */
    value = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
    i2c_imx_slave_event(i2c_imx,
    I2C_SLAVE_WRITE_RECEIVED, &value);
    } else if (!(status & I2SR_RXAK)) { /* Transmit mode received ACK */
    ctl |= I2CR_MTX;
    imx_i2c_write_reg(ctl, i2c_imx, IMX_I2C_I2CR);
    i2c_imx_slave_event(i2c_imx,
    I2C_SLAVE_READ_PROCESSED, &value);
    imx_i2c_write_reg(value, i2c_imx, IMX_I2C_I2DR);
    } else { /* Transmit mode received NAK, operation is done */
    ctl &= ~I2CR_MTX;
    imx_i2c_write_reg(ctl, i2c_imx, IMX_I2C_I2CR);
    imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
// flag the last byte as processed
    i2c_imx_slave_event(i2c_imx,
    I2C_SLAVE_READ_PROCESSED, &value);
    i2c_imx_slave_finish_op(i2c_imx);
    return IRQ_HANDLED;
    }
    out:
//
// No need to check the return value here.  If it returns 0 or
// 1, then everything is fine.  If it returns -1, then the
// timer is running in the handler.  This will still work,
// though it may be redone (or already have been done) by the
// timer function.
//
    hrtimer_try_to_cancel(&i2c_imx.slave_timer);
    hrtimer_forward_now(&i2c_imx.slave_timer, I2C_IMX_CHECK_DELAY);
    hrtimer_restart(&i2c_imx.slave_timer);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_slave_timeout(t: *mut hrtimer) -> enum hrtimer_restart {
    static enum hrtimer_restart i2c_imx_slave_timeout(struct hrtimer *t)
    {
    struct imx_i2c_struct *i2c_imx = container_of(t, struct imx_i2c_struct,
    slave_timer);
    unsigned int ctl, status;
    guard(spinlock_irqsave)(&i2c_imx.slave_lock);
    status = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2SR);
    ctl = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    i2c_imx_slave_handle(i2c_imx, status, ctl);
    return HRTIMER_NORESTART;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_slave_init(i2c_imx: *mut imx_i2c_struct) {
    static void i2c_imx_slave_init(struct imx_i2c_struct *i2c_imx)
    {
    int temp;
// Set slave addr.
    imx_i2c_write_reg((i2c_imx.slave.addr << 1), i2c_imx, IMX_I2C_IADR);
    i2c_imx_reset_regs(i2c_imx);
// Enable module
    temp = i2c_imx.hwdata.i2cr_ien_opcode;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
// Enable interrupt from i2c module
    temp |= I2CR_IIEN;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    i2c_imx_enable_bus_idle(i2c_imx);
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_reg_slave(client: *mut i2c_client) -> c_int {
    static int i2c_imx_reg_slave(struct i2c_client *client)
    {
    struct imx_i2c_struct *i2c_imx = i2c_get_adapdata(client.adapter);
    int ret;
    if (i2c_imx.slave)
    return -EBUSY;
// Resume
    ret = pm_runtime_resume_and_get(i2c_imx.adapter.dev.parent);
    if (ret < 0) {
    dev_err(&i2c_imx.adapter.dev, "failed to resume i2c controller");
    return ret;
    }
    scoped_guard(spinlock_irqsave, &i2c_imx.slave_lock) {
    i2c_imx.slave = client;
    i2c_imx.last_slave_event = I2C_SLAVE_STOP;
    }
    i2c_imx_slave_init(i2c_imx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_unreg_slave(client: *mut i2c_client) -> c_int {
    static int i2c_imx_unreg_slave(struct i2c_client *client)
    {
    struct imx_i2c_struct *i2c_imx = i2c_get_adapdata(client.adapter);
    int ret;
    if (!i2c_imx.slave)
    return -EINVAL;
// Reset slave address.
    imx_i2c_write_reg(0, i2c_imx, IMX_I2C_IADR);
    i2c_imx_reset_regs(i2c_imx);
    hrtimer_cancel(&i2c_imx.slave_timer);
    i2c_imx.slave = core::ptr::null_mut();
// Suspend
    ret = pm_runtime_put_sync(i2c_imx.adapter.dev.parent);
    if (ret < 0)
    dev_err(&i2c_imx.adapter.dev, "failed to suspend i2c controller");
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_imx_isr_acked(i2c_imx: *mut imx_i2c_struct) -> c_int {
    static inline int i2c_imx_isr_acked(struct imx_i2c_struct *i2c_imx)
    {
    i2c_imx.isr_result = 0;
    if (imx_i2c_read_reg(i2c_imx, IMX_I2C_I2SR) & I2SR_RXAK) {
    i2c_imx.state = IMX_I2C_STATE_FAILED;
    i2c_imx.isr_result = -ENXIO;
    wake_up(&i2c_imx.queue);
    }
    return i2c_imx.isr_result;
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_imx_isr_write(i2c_imx: *mut imx_i2c_struct) -> c_int {
    static inline int i2c_imx_isr_write(struct imx_i2c_struct *i2c_imx)
    {
    int result;
    result = i2c_imx_isr_acked(i2c_imx);
    if (result)
    return result;
    if (i2c_imx.msg.len == i2c_imx.msg_buf_idx)
    return 0;
    imx_i2c_write_reg(i2c_imx.msg.buf[i2c_imx.msg_buf_idx++], i2c_imx, IMX_I2C_I2DR);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_imx_isr_read(i2c_imx: *mut imx_i2c_struct) -> c_int {
    static inline int i2c_imx_isr_read(struct imx_i2c_struct *i2c_imx)
    {
    int result;
    unsigned int temp;
    result = i2c_imx_isr_acked(i2c_imx);
    if (result)
    return result;
// setup bus to read data
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp &= ~I2CR_MTX;
    if ((i2c_imx.msg.len - 1) || (i2c_imx.msg.flags & I2C_M_RECV_LEN))
    temp &= ~I2CR_TXAK;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR); /* dummy read */
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_imx_isr_read_continue(i2c_imx: *mut imx_i2c_struct) -> enum imx_i2c_state {
    static inline enum imx_i2c_state i2c_imx_isr_read_continue(struct imx_i2c_struct *i2c_imx)
    {
    let mut next_state: enum imx_i2c_state = IMX_I2C_STATE_READ_CONTINUE;
    unsigned int temp;
    if ((i2c_imx.msg.len - 1) == i2c_imx.msg_buf_idx) {
    if (i2c_imx.is_lastmsg) {
//
// It must generate STOP before read I2DR to prevent
// controller from generating another clock cycle
//
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    if (!(temp & I2CR_MSTA))
    i2c_imx.stopped =  1;
    temp &= ~(I2CR_MSTA | I2CR_MTX);
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    return IMX_I2C_STATE_DONE;
    }
//
// For i2c master receiver repeat restart operation like:
// read -> repeat MSTA -> read/write
// The controller must set MTX before read the last byte in
// the first read operation, otherwise the first read cost
// one extra clock cycle.
//
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_MTX;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    next_state = IMX_I2C_STATE_DONE;
    } else if (i2c_imx.msg_buf_idx == (i2c_imx.msg.len - 2)) {
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_TXAK;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    }
    i2c_imx.msg.buf[i2c_imx.msg_buf_idx++] = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
    return next_state;
    }
#[no_mangle]
pub unsafe extern "C" fn i2c_imx_isr_read_block_data_len(i2c_imx: *mut imx_i2c_struct) {
    static inline void i2c_imx_isr_read_block_data_len(struct imx_i2c_struct *i2c_imx)
    {
    let mut len: u8 = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
    unsigned int temp;
    if (len == 0 || len > I2C_SMBUS_BLOCK_MAX) {
//
// SMBus 3.1 6.5.7: support count byte of 0.
// I2C_SMBUS_BLOCK_MAX case should not hold the SDA either.
// So NACK it (TXAK) to not hold the bus.
//
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_TXAK;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    if (len == 0) {
    i2c_imx.msg.buf[i2c_imx.msg_buf_idx++] = 0;
    i2c_imx.msg.len = 2;
    return;
    }
    i2c_imx.isr_result = -EPROTO;
    i2c_imx.state = IMX_I2C_STATE_FAILED;
    wake_up(&i2c_imx.queue);
    return;
    }
    i2c_imx.msg.len += len;
    i2c_imx.msg.buf[i2c_imx.msg_buf_idx++] = len;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_master_isr(i2c_imx: *mut imx_i2c_struct, status: c_uint) -> irqreturn_t {
    static irqreturn_t i2c_imx_master_isr(struct imx_i2c_struct *i2c_imx, unsigned int status)
    {
//
// This state machine handles I2C reception and transmission in non-DMA
// mode. We must process all the data in the ISR to reduce the delay
// between two consecutive messages. If the data is not processed in
// the ISR, SMBus devices may timeout, leading to a bus error.
//
    switch (i2c_imx.state) {
    case IMX_I2C_STATE_DMA:
    i2c_imx.i2csr = status;
    wake_up(&i2c_imx.queue);
    break;
    case IMX_I2C_STATE_READ:
    if (i2c_imx_isr_read(i2c_imx))
    break;
    i2c_imx.state = IMX_I2C_STATE_READ_CONTINUE;
    break;
    case IMX_I2C_STATE_READ_CONTINUE:
    i2c_imx.state = i2c_imx_isr_read_continue(i2c_imx);
    if (i2c_imx.state == IMX_I2C_STATE_DONE)
    wake_up(&i2c_imx.queue);
    break;
    case IMX_I2C_STATE_READ_BLOCK_DATA:
    if (i2c_imx_isr_read(i2c_imx))
    break;
    i2c_imx.state = IMX_I2C_STATE_READ_BLOCK_DATA_LEN;
    break;
    case IMX_I2C_STATE_READ_BLOCK_DATA_LEN:
    i2c_imx_isr_read_block_data_len(i2c_imx);
    if (i2c_imx.state == IMX_I2C_STATE_READ_BLOCK_DATA_LEN)
    i2c_imx.state = IMX_I2C_STATE_READ_CONTINUE;
    break;
    case IMX_I2C_STATE_WRITE:
    if (i2c_imx_isr_write(i2c_imx))
    break;
    i2c_imx.state = IMX_I2C_STATE_DONE;
    wake_up(&i2c_imx.queue);
    break;
    default:
    i2c_imx.i2csr = status;
    i2c_imx.state = IMX_I2C_STATE_FAILED;
    i2c_imx.isr_result = -EINVAL;
    wake_up(&i2c_imx.queue);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t i2c_imx_isr(int irq, void *dev_id)
    {
    struct imx_i2c_struct *i2c_imx = dev_id;
    unsigned int ctl, status;
    scoped_guard(spinlock_irqsave, &i2c_imx.slave_lock) {
    status = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2SR);
    ctl = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    if (!(status & I2SR_IIF))
    return IRQ_NONE;
    i2c_imx_clear_irq(i2c_imx, I2SR_IIF);
    if (i2c_imx.slave) {
    if (!(ctl & I2CR_MSTA))
    return i2c_imx_slave_handle(i2c_imx,
    status, ctl);
    i2c_imx_slave_finish_op(i2c_imx);
    }
    }
    return i2c_imx_master_isr(i2c_imx, status);
    }
    static int i2c_imx_dma_write(struct imx_i2c_struct *i2c_imx,
    struct i2c_msg *msgs)
    {
    int result;
    unsigned long time_left;
    let mut temp: c_uint = 0;
    let mut orig_jiffies: c_ulong = jiffies;
    struct imx_i2c_dma *dma = i2c_imx.dma;
    struct device *dev = &i2c_imx.adapter.dev;
    i2c_imx.state = IMX_I2C_STATE_DMA;
    dma.chan_using = dma.chan_tx;
    dma.dma_transfer_dir = DMA_MEM_TO_DEV;
    dma.dma_data_dir = DMA_TO_DEVICE;
    dma.dma_len = msgs.len - 1;
    result = i2c_imx_dma_xfer(i2c_imx, msgs);
    if (result)
    return result;
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_DMAEN;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
//
// Write slave address.
// The first byte must be transmitted by the CPU.
//
    imx_i2c_write_reg(i2c_8bit_addr_from_msg(msgs), i2c_imx, IMX_I2C_I2DR);
    time_left = wait_for_completion_timeout(
    &i2c_imx.dma.cmd_complete,
    msecs_to_jiffies(DMA_TIMEOUT));
    if (time_left == 0) {
    dmaengine_terminate_sync(dma.chan_using);
    return -ETIMEDOUT;
    }
// Waiting for transfer complete.
    while (1) {
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2SR);
    if (temp & I2SR_ICF)
    break;
    if (time_after(jiffies, orig_jiffies +
    msecs_to_jiffies(DMA_TIMEOUT))) {
    dev_dbg(dev, "<%s> Timeout\n", __func__);
    return -ETIMEDOUT;
    }
    schedule();
    }
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp &= ~I2CR_DMAEN;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
// The last data byte must be transferred by the CPU.
    imx_i2c_write_reg(msgs.buf[msgs.len-1],
    i2c_imx, IMX_I2C_I2DR);
    result = i2c_imx_trx_complete(i2c_imx, false);
    if (result)
    return result;
    return i2c_imx_acked(i2c_imx);
    }
    static int i2c_imx_prepare_read(struct imx_i2c_struct *i2c_imx,
    struct i2c_msg *msgs, bool use_dma)
    {
    int result;
    let mut temp: c_uint = 0;
// write slave address
    imx_i2c_write_reg(i2c_8bit_addr_from_msg(msgs), i2c_imx, IMX_I2C_I2DR);
    result = i2c_imx_trx_complete(i2c_imx, !use_dma);
    if (result)
    return result;
    result = i2c_imx_acked(i2c_imx);
    if (result)
    return result;
    dev_dbg(&i2c_imx.adapter.dev, "<%s> setup bus\n", __func__);
// setup bus to read data
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp &= ~I2CR_MTX;
//
// Reset the I2CR_TXAK flag initially for SMBus block read since the
// length is unknown
//
    if (msgs.len - 1)
    temp &= ~I2CR_TXAK;
    if (use_dma)
    temp |= I2CR_DMAEN;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR); /* dummy read */
    return 0;
    }
    static int i2c_imx_dma_read(struct imx_i2c_struct *i2c_imx,
    struct i2c_msg *msgs, bool is_lastmsg)
    {
    int result;
    unsigned long time_left;
    unsigned int temp;
    let mut orig_jiffies: c_ulong = jiffies;
    struct imx_i2c_dma *dma = i2c_imx.dma;
    struct device *dev = &i2c_imx.adapter.dev;
    i2c_imx.state = IMX_I2C_STATE_DMA;
    result = i2c_imx_prepare_read(i2c_imx, msgs, true);
    if (result)
    return result;
    dev_dbg(&i2c_imx.adapter.dev, "<%s> read data\n", __func__);
    dma.chan_using = dma.chan_rx;
    dma.dma_transfer_dir = DMA_DEV_TO_MEM;
    dma.dma_data_dir = DMA_FROM_DEVICE;
// The last two data bytes must be transferred by the CPU.
    dma.dma_len = msgs.len - 2;
    result = i2c_imx_dma_xfer(i2c_imx, msgs);
    if (result)
    return result;
    time_left = wait_for_completion_timeout(
    &i2c_imx.dma.cmd_complete,
    msecs_to_jiffies(DMA_TIMEOUT));
    if (time_left == 0) {
    dmaengine_terminate_sync(dma.chan_using);
    return -ETIMEDOUT;
    }
// waiting for transfer complete.
    while (1) {
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2SR);
    if (temp & I2SR_ICF)
    break;
    if (time_after(jiffies, orig_jiffies +
    msecs_to_jiffies(DMA_TIMEOUT))) {
    dev_dbg(dev, "<%s> Timeout\n", __func__);
    return -ETIMEDOUT;
    }
    schedule();
    }
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp &= ~I2CR_DMAEN;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
// read n-1 byte data
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_TXAK;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    msgs.buf[msgs.len-2] = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
// read n byte data
    result = i2c_imx_trx_complete(i2c_imx, false);
    if (result)
    return result;
    if (is_lastmsg) {
//
// It must generate STOP before read I2DR to prevent
// controller from generating another clock cycle
//
    dev_dbg(dev, "<%s> clear MSTA\n", __func__);
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    if (!(temp & I2CR_MSTA))
    i2c_imx.stopped = 1;
    temp &= ~(I2CR_MSTA | I2CR_MTX);
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    if (!i2c_imx.stopped)
    i2c_imx_bus_busy(i2c_imx, 0, false);
    } else {
//
// For i2c master receiver repeat restart operation like:
// read -> repeat MSTA -> read/write
// The controller must set MTX before read the last byte in
// the first read operation, otherwise the first read cost
// one extra clock cycle.
//
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_MTX;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    }
    msgs.buf[msgs.len-1] = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
    return 0;
    }
    static int i2c_imx_atomic_write(struct imx_i2c_struct *i2c_imx,
    struct i2c_msg *msgs)
    {
    int i, result;
    dev_dbg(&i2c_imx.adapter.dev, "<%s> write slave address: addr=0x%x\n",
    __func__, i2c_8bit_addr_from_msg(msgs));
// write slave address
    imx_i2c_write_reg(i2c_8bit_addr_from_msg(msgs), i2c_imx, IMX_I2C_I2DR);
    result = i2c_imx_trx_complete(i2c_imx, true);
    if (result)
    return result;
    result = i2c_imx_acked(i2c_imx);
    if (result)
    return result;
    dev_dbg(&i2c_imx.adapter.dev, "<%s> write data\n", __func__);
// write data
    for (i = 0; i < msgs.len; i++) {
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> write byte: B%d=0x%X\n",
    __func__, i, msgs.buf[i]);
    imx_i2c_write_reg(msgs.buf[i], i2c_imx, IMX_I2C_I2DR);
    result = i2c_imx_trx_complete(i2c_imx, true);
    if (result)
    return result;
    result = i2c_imx_acked(i2c_imx);
    if (result)
    return result;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_write(i2c_imx: *mut imx_i2c_struct, msgs: *mut i2c_msg) -> c_int {
    static int i2c_imx_write(struct imx_i2c_struct *i2c_imx, struct i2c_msg *msgs)
    {
    dev_dbg(&i2c_imx.adapter.dev, "<%s> write slave address: addr=0x%x\n",
    __func__, i2c_8bit_addr_from_msg(msgs));
    i2c_imx.state = IMX_I2C_STATE_WRITE;
    i2c_imx.msg = msgs;
    i2c_imx.msg_buf_idx = 0;
//
// By writing the device address we start the state machine in the ISR.
// The ISR will report when it is done or when it fails.
//
    imx_i2c_write_reg(i2c_8bit_addr_from_msg(msgs), i2c_imx, IMX_I2C_I2DR);
    wait_event_timeout(i2c_imx.queue,
    i2c_imx.state == IMX_I2C_STATE_DONE ||
    i2c_imx.state == IMX_I2C_STATE_FAILED,
    (msgs.len + 1) * HZ / 10);
    if (i2c_imx.state == IMX_I2C_STATE_FAILED) {
    dev_dbg(&i2c_imx.adapter.dev, "<%s> write failed with %d\n",
    __func__, i2c_imx.isr_result);
    return i2c_imx.isr_result;
    }
    if (i2c_imx.state != IMX_I2C_STATE_DONE) {
    dev_err(&i2c_imx.adapter.dev, "<%s> write timedout\n", __func__);
    return -ETIMEDOUT;
    }
    return 0;
    }
    static int i2c_imx_atomic_read(struct imx_i2c_struct *i2c_imx,
    struct i2c_msg *msgs, bool is_lastmsg)
    {
    int i, result;
    unsigned int temp;
    let mut block_data: c_int = msgs.flags & I2C_M_RECV_LEN;
    let mut block_err: c_int = 0;
    result = i2c_imx_prepare_read(i2c_imx, msgs, false);
    if (result)
    return result;
    dev_dbg(&i2c_imx.adapter.dev, "<%s> read data\n", __func__);
// read data
    for (i = 0; i < msgs.len; i++) {
    let mut len: u8 = 0;
    result = i2c_imx_trx_complete(i2c_imx, true);
    if (result)
    return result;
//
// First byte is the length of remaining packet
// in the SMBus block data read. Add it to
// msgs->len.
//
    if ((!i) && block_data) {
    len = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
    if ((len == 0) || (len > I2C_SMBUS_BLOCK_MAX)) {
//
// SMBus 3.1 6.5.7: support count byte of 0.
// I2C_SMBUS_BLOCK_MAX case should not hold the SDA either.
//
    if (len > I2C_SMBUS_BLOCK_MAX)
    block_err = -EPROTO;
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_TXAK;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    msgs.buf[0] = 0;
    msgs.len = 2;
    continue;
    }
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> read length: 0x%X\n",
    __func__, len);
    msgs.len += len;
    }
    if (i == (msgs.len - 1)) {
    if (is_lastmsg) {
//
// It must generate STOP before read I2DR to prevent
// controller from generating another clock cycle
//
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> clear MSTA\n", __func__);
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    if (!(temp & I2CR_MSTA))
    i2c_imx.stopped =  1;
    temp &= ~(I2CR_MSTA | I2CR_MTX);
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    if (!i2c_imx.stopped)
    i2c_imx_bus_busy(i2c_imx, 0, true);
    } else {
//
// For i2c master receiver repeat restart operation like:
// read -> repeat MSTA -> read/write
// The controller must set MTX before read the last byte in
// the first read operation, otherwise the first read cost
// one extra clock cycle.
//
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_MTX;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    }
    } else if (i == (msgs.len - 2)) {
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> set TXAK\n", __func__);
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_TXAK;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    }
    if ((!i) && block_data)
    msgs.buf[0] = len;
    else
    msgs.buf[i] = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2DR);
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> read byte: B%d=0x%X\n",
    __func__, i, msgs.buf[i]);
    }
    return block_err;
    }
    static int i2c_imx_read(struct imx_i2c_struct *i2c_imx, struct i2c_msg *msgs,
    bool is_lastmsg)
    {
    let mut block_data: c_int = msgs.flags & I2C_M_RECV_LEN;
    let mut ret: c_int = 0;
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> write slave address: addr=0x%x\n",
    __func__, i2c_8bit_addr_from_msg(msgs));
    i2c_imx.is_lastmsg = is_lastmsg;
    if (block_data)
    i2c_imx.state = IMX_I2C_STATE_READ_BLOCK_DATA;
    else
    i2c_imx.state = IMX_I2C_STATE_READ;
    i2c_imx.msg = msgs;
    i2c_imx.msg_buf_idx = 0;
//
// By writing the device address we start the state machine in the ISR.
// The ISR will report when it is done or when it fails.
//
    imx_i2c_write_reg(i2c_8bit_addr_from_msg(msgs), i2c_imx, IMX_I2C_I2DR);
    wait_event_timeout(i2c_imx.queue,
    i2c_imx.state == IMX_I2C_STATE_DONE ||
    i2c_imx.state == IMX_I2C_STATE_FAILED,
    (msgs.len + 1) * HZ / 10);
    if (i2c_imx.state == IMX_I2C_STATE_FAILED) {
    dev_dbg(&i2c_imx.adapter.dev, "<%s> read failed with %d\n",
    __func__, i2c_imx.isr_result);
    return i2c_imx.isr_result;
    }
    if (i2c_imx.state != IMX_I2C_STATE_DONE) {
    dev_err(&i2c_imx.adapter.dev, "<%s> read timedout\n", __func__);
    return -ETIMEDOUT;
    }
    if (i2c_imx.is_lastmsg) {
    if (!i2c_imx.stopped)
    ret = i2c_imx_bus_busy(i2c_imx, 0, false);
//
// Only read the last byte of the last message after the bus is
// not busy. Else the controller generates another clock which
// might confuse devices.
//
    if (!ret)
    i2c_imx.msg.buf[i2c_imx.msg_buf_idx++] = imx_i2c_read_reg(i2c_imx,
    IMX_I2C_I2DR);
    }
    return ret;
    }
    static int i2c_imx_xfer_common(struct i2c_adapter *adapter,
    struct i2c_msg *msgs, int num, bool atomic)
    {
    unsigned int i, temp;
    int result;
    let mut is_lastmsg: bool = false;
    struct imx_i2c_struct *i2c_imx = i2c_get_adapdata(adapter);
    let mut use_dma: c_int = 0;
// Start I2C transfer
    result = i2c_imx_start(i2c_imx, atomic);
    if (result) {
//
// Bus recovery uses gpiod_get_value_cansleep() which is not
// allowed within atomic context.
//
    if (!atomic && i2c_imx.adapter.bus_recovery_info) {
    i2c_recover_bus(&i2c_imx.adapter);
    result = i2c_imx_start(i2c_imx, atomic);
    }
    }
    if (result)
    goto fail0;
// read/write data
    for (i = 0; i < num; i++) {
    if (i == num - 1)
    is_lastmsg = true;
    if (i) {
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> repeated start\n", __func__);
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    temp |= I2CR_RSTA;
    imx_i2c_write_reg(temp, i2c_imx, IMX_I2C_I2CR);
    result = i2c_imx_bus_busy(i2c_imx, 1, atomic);
    if (result)
    goto fail0;
    }
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> transfer message: %d\n", __func__, i);
// write/read data

    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2CR);
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> CONTROL: IEN=%d, IIEN=%d, MSTA=%d, MTX=%d, TXAK=%d, RSTA=%d\n",
    __func__,
    (temp & I2CR_IEN ? 1 : 0), (temp & I2CR_IIEN ? 1 : 0),
    (temp & I2CR_MSTA ? 1 : 0), (temp & I2CR_MTX ? 1 : 0),
    (temp & I2CR_TXAK ? 1 : 0), (temp & I2CR_RSTA ? 1 : 0));
    temp = imx_i2c_read_reg(i2c_imx, IMX_I2C_I2SR);
    dev_dbg(&i2c_imx.adapter.dev,
    "<%s> STATUS: ICF=%d, IAAS=%d, IBB=%d, IAL=%d, SRW=%d, IIF=%d, RXAK=%d\n",
    __func__,
    (temp & I2SR_ICF ? 1 : 0), (temp & I2SR_IAAS ? 1 : 0),
    (temp & I2SR_IBB ? 1 : 0), (temp & I2SR_IAL ? 1 : 0),
    (temp & I2SR_SRW ? 1 : 0), (temp & I2SR_IIF ? 1 : 0),
    (temp & I2SR_RXAK ? 1 : 0));

    use_dma = i2c_imx.dma && msgs[i].len >= DMA_THRESHOLD &&
    msgs[i].flags & I2C_M_DMA_SAFE;
    if (msgs[i].flags & I2C_M_RD) {
    let mut block_data: c_int = msgs.flags & I2C_M_RECV_LEN;
    if (atomic)
    result = i2c_imx_atomic_read(i2c_imx, &msgs[i], is_lastmsg);
#[no_mangle]
pub unsafe extern "C" fn if(!block_data: use_dma &&) -> else {
    else if (use_dma && !block_data)
    result = i2c_imx_dma_read(i2c_imx, &msgs[i], is_lastmsg);
    else
    result = i2c_imx_read(i2c_imx, &msgs[i], is_lastmsg);
    } else {
    if (atomic)
    result = i2c_imx_atomic_write(i2c_imx, &msgs[i]);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: use_dma) -> else {
    else if (use_dma)
    result = i2c_imx_dma_write(i2c_imx, &msgs[i]);
    else
    result = i2c_imx_write(i2c_imx, &msgs[i]);
    }
    if (result)
    goto fail0;
    }
    fail0:
// Stop I2C transfer
    i2c_imx_stop(i2c_imx, atomic);
    dev_dbg(&i2c_imx.adapter.dev, "<%s> exit with: %s: %d\n", __func__,
    (result < 0) ? "error" : "success msg",
    (result < 0) ? result : num);
// After data is transferred, switch to slave mode(as a receiver)
    if (i2c_imx.slave)
    i2c_imx_slave_init(i2c_imx);
    return (result < 0) ? result : num;
    }
    static int i2c_imx_xfer(struct i2c_adapter *adapter,
    struct i2c_msg *msgs, int num)
    {
    struct imx_i2c_struct *i2c_imx = i2c_get_adapdata(adapter);
    int result;
    result = pm_runtime_resume_and_get(i2c_imx.adapter.dev.parent);
    if (result < 0)
    return result;
    result = i2c_imx_xfer_common(adapter, msgs, num, false);
    pm_runtime_put_autosuspend(i2c_imx.adapter.dev.parent);
    return result;
    }
    static int i2c_imx_xfer_atomic(struct i2c_adapter *adapter,
    struct i2c_msg *msgs, int num)
    {
    struct imx_i2c_struct *i2c_imx = i2c_get_adapdata(adapter);
    int result;
    result = clk_enable(i2c_imx.clk);
    if (result)
    return result;
    result = i2c_imx_xfer_common(adapter, msgs, num, true);
    clk_disable(i2c_imx.clk);
    return result;
    }
//
// We switch SCL and SDA to their GPIO function and do some bitbanging
// for bus recovery. These alternative pinmux settings can be
// described in the device tree by a separate pinctrl state "gpio". If
// this is missing this is not a big problem, the only implication is
// that we can't do bus recovery.
//
    static int i2c_imx_init_recovery_info(struct imx_i2c_struct *i2c_imx,
    struct platform_device *pdev)
    {
    struct i2c_bus_recovery_info *bri = &i2c_imx.rinfo;
    bri.pinctrl = devm_pinctrl_get(&pdev.dev);
    if (IS_ERR(bri.pinctrl))
    return PTR_ERR(bri.pinctrl);
    i2c_imx.adapter.bus_recovery_info = bri;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_func(adapter: *mut i2c_adapter) -> u32 {
    static u32 i2c_imx_func(struct i2c_adapter *adapter)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL
    | I2C_FUNC_SMBUS_READ_BLOCK_DATA;
    }
    static const struct i2c_algorithm i2c_imx_algo = {
    .xfer = i2c_imx_xfer,
    .xfer_atomic = i2c_imx_xfer_atomic,
    .functionality = i2c_imx_func,
    .reg_slave = i2c_imx_reg_slave,
    .unreg_slave = i2c_imx_unreg_slave,
    };
#[no_mangle]
unsafe extern "C" fn i2c_imx_probe(pdev: *mut platform_device) -> c_int {
    static int i2c_imx_probe(struct platform_device *pdev)
    {
    struct imx_i2c_struct *i2c_imx;
    struct resource *res;
    struct imxi2c_platform_data *pdata = dev_get_platdata(&pdev.dev);
    void __iomem *base;
    int irq, ret;
    dma_addr_t phy_addr;
    const struct imx_i2c_hwdata *match;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return dev_err_probe(&pdev.dev, PTR_ERR(base), "can't get IO memory\n");
    phy_addr = (dma_addr_t)res.start;
    i2c_imx = devm_kzalloc(&pdev.dev, sizeof(*i2c_imx), GFP_KERNEL);
    if (!i2c_imx)
    return -ENOMEM;
    spin_lock_init(&i2c_imx.slave_lock);
    hrtimer_setup(&i2c_imx.slave_timer, i2c_imx_slave_timeout, CLOCK_MONOTONIC,
    HRTIMER_MODE_ABS);
    match = device_get_match_data(&pdev.dev);
    if (match)
    i2c_imx.hwdata = match;
    else
    i2c_imx.hwdata = (struct imx_i2c_hwdata *)
    platform_get_device_id(pdev).driver_data;
// Setup i2c_imx driver structure
    strscpy(i2c_imx.adapter.name, pdev.name, sizeof(i2c_imx.adapter.name));
    i2c_imx.adapter.owner		= THIS_MODULE;
    i2c_imx.adapter.algo		= &i2c_imx_algo;
    i2c_imx.adapter.dev.parent	= &pdev.dev;
    i2c_imx.adapter.nr		= pdev.id;
    i2c_imx.adapter.dev.of_node	= pdev.dev.of_node;
    i2c_imx.base			= base;
    ACPI_COMPANION_SET(&i2c_imx.adapter.dev, ACPI_COMPANION(&pdev.dev));
// Get I2C clock
    i2c_imx.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(i2c_imx.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(i2c_imx.clk),
    "can't get I2C clock\n");
// Init queue
    init_waitqueue_head(&i2c_imx.queue);
// Set up adapter data
    i2c_set_adapdata(&i2c_imx.adapter, i2c_imx);
// Set up platform driver data
    platform_set_drvdata(pdev, i2c_imx);
    pm_runtime_set_autosuspend_delay(&pdev.dev, I2C_PM_TIMEOUT);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = pm_runtime_get_sync(&pdev.dev);
    if (ret < 0)
    goto rpm_disable;
// Request IRQ
    ret = request_irq(irq, i2c_imx_isr, IRQF_SHARED | IRQF_NO_SUSPEND,
    pdev.name, i2c_imx);
    if (ret) {
    dev_err(&pdev.dev, "can't claim irq %d\n", irq);
    goto rpm_disable;
    }
//
// We use the single-master property for backward compatibility.
// By default multi master mode is enabled.
//
    i2c_imx.multi_master = !of_property_read_bool(pdev.dev.of_node, "single-master");
// Set up clock divider
    i2c_imx.bitrate = I2C_MAX_STANDARD_MODE_FREQ;
    ret = of_property_read_u32(pdev.dev.of_node,
    "clock-frequency", &i2c_imx.bitrate);
    if (ret < 0 && pdata && pdata.bitrate)
    i2c_imx.bitrate = pdata.bitrate;
    i2c_imx.clk_change_nb.notifier_call = i2c_imx_clk_notifier_call;
    clk_notifier_register(i2c_imx.clk, &i2c_imx.clk_change_nb);
    ret = i2c_imx_set_clk(i2c_imx, clk_get_rate(i2c_imx.clk));
    if (ret < 0) {
    dev_err(&pdev.dev, "can't get I2C clock\n");
    goto clk_notifier_unregister;
    }
    i2c_imx_reset_regs(i2c_imx);
// Init optional bus recovery function
    ret = i2c_imx_init_recovery_info(i2c_imx, pdev);
// Give it another chance if pinctrl used is not ready yet
    if (ret == -EPROBE_DEFER)
    goto clk_notifier_unregister;
//
// DMA mode should be optional for I2C, when encountering DMA errors,
// no need to exit I2C probe. Only print warning to show DMA error and
// use PIO mode directly to ensure I2C bus available as much as possible.
//
    ret = i2c_imx_dma_request(i2c_imx, phy_addr);
    if (ret) {
    if (ret == -EPROBE_DEFER) {
    dev_err_probe(&pdev.dev, ret, "can't get DMA channels\n");
    goto clk_notifier_unregister;
    } else if (ret == -ENODEV) {
    dev_dbg(&pdev.dev, "Only use PIO mode\n");
    } else {
    dev_warn(&pdev.dev, "Failed to setup DMA (%pe), only use PIO mode\n",
    ERR_PTR(ret));
    }
    }
// Add I2C adapter
    ret = i2c_add_numbered_adapter(&i2c_imx.adapter);
    if (ret < 0)
    goto clk_notifier_unregister;
    pm_runtime_put_autosuspend(&pdev.dev);
    dev_dbg(&i2c_imx.adapter.dev, "claimed irq %d\n", irq);
    dev_dbg(&i2c_imx.adapter.dev, "device resources: %pR\n", res);
    dev_dbg(&i2c_imx.adapter.dev, "adapter name: \"%s\"\n",
    i2c_imx.adapter.name);
    dev_info(&i2c_imx.adapter.dev, "IMX I2C adapter registered\n");
    return 0;   /* Return OK */
    clk_notifier_unregister:
    clk_notifier_unregister(i2c_imx.clk, &i2c_imx.clk_change_nb);
    free_irq(irq, i2c_imx);
    rpm_disable:
    pm_runtime_put_noidle(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    pm_runtime_set_suspended(&pdev.dev);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_remove(pdev: *mut platform_device) {
    static void i2c_imx_remove(struct platform_device *pdev)
    {
    struct imx_i2c_struct *i2c_imx = platform_get_drvdata(pdev);
    int irq, ret;
    ret = pm_runtime_get_sync(&pdev.dev);
    hrtimer_cancel(&i2c_imx.slave_timer);
// remove adapter
    dev_dbg(&i2c_imx.adapter.dev, "adapter removed\n");
    i2c_del_adapter(&i2c_imx.adapter);
    if (i2c_imx.dma)
    i2c_imx_dma_free(i2c_imx);
    if (ret >= 0) {
// setup chip registers to defaults
    imx_i2c_write_reg(0, i2c_imx, IMX_I2C_IADR);
    imx_i2c_write_reg(0, i2c_imx, IMX_I2C_IFDR);
    imx_i2c_write_reg(0, i2c_imx, IMX_I2C_I2CR);
    imx_i2c_write_reg(0, i2c_imx, IMX_I2C_I2SR);
    }
    clk_notifier_unregister(i2c_imx.clk, &i2c_imx.clk_change_nb);
    irq = platform_get_irq(pdev, 0);
    if (irq >= 0)
    free_irq(irq, i2c_imx);
    pm_runtime_put_noidle(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_runtime_suspend(dev: *mut device) -> c_int {
    static int i2c_imx_runtime_suspend(struct device *dev)
    {
    struct imx_i2c_struct *i2c_imx = dev_get_drvdata(dev);
    int ret;
    ret = pinctrl_pm_select_sleep_state(dev);
    if (ret)
    return ret;
    clk_disable(i2c_imx.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_runtime_resume(dev: *mut device) -> c_int {
    static int i2c_imx_runtime_resume(struct device *dev)
    {
    struct imx_i2c_struct *i2c_imx = dev_get_drvdata(dev);
    int ret;
    ret = pinctrl_pm_select_default_state(dev);
    if (ret)
    return ret;
    ret = clk_enable(i2c_imx.clk);
    if (ret) {
    dev_err(dev, "can't enable I2C clock, ret=%d\n", ret);
    pinctrl_pm_select_sleep_state(dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_suspend_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused i2c_imx_suspend_noirq(struct device *dev)
    {
    struct imx_i2c_struct *i2c_imx = dev_get_drvdata(dev);
    int ret;
    i2c_mark_adapter_suspended(&i2c_imx.adapter);
//
// Cancel the slave timer before powering down to prevent
// i2c_imx_slave_timeout() from accessing hardware registers
// while the clock is disabled.
//
    hrtimer_cancel(&i2c_imx.slave_timer);
    ret = pm_runtime_force_suspend(dev);
    if (ret) {
    i2c_mark_adapter_resumed(&i2c_imx.adapter);
    if (i2c_imx.slave) {
    hrtimer_forward_now(&i2c_imx.slave_timer, I2C_IMX_CHECK_DELAY);
    hrtimer_restart(&i2c_imx.slave_timer);
    }
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_resume_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused i2c_imx_resume_noirq(struct device *dev)
    {
    struct imx_i2c_struct *i2c_imx = dev_get_drvdata(dev);
    int ret;
    ret = pm_runtime_force_resume(dev);
    if (ret)
    return ret;
    i2c_mark_adapter_resumed(&i2c_imx.adapter);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_suspend(dev: *mut device) -> c_int {
    static int i2c_imx_suspend(struct device *dev)
    {
//
// Some I2C devices may need the I2C controller to remain active
// during resume_noirq() or suspend_noirq(). If the controller is
// autosuspended, there is no way to wake it up once runtime PM is
// disabled (in suspend_late()).
//
// During system resume, the I2C controller will be available only
// after runtime PM is re-enabled (in resume_early()). However, this
// may be too late for some devices.
//
// Wake up the controller in the suspend() callback while runtime PM
// is still enabled. The I2C controller will remain available until
// the suspend_noirq() callback (pm_runtime_force_suspend()) is
// called. During resume, the I2C controller can be restored by the
// resume_noirq() callback (pm_runtime_force_resume()).
//
// Finally, the resume() callback re-enables autosuspend, ensuring
// the I2C controller remains available until the system enters
// suspend_noirq() and from resume_noirq().
//
    return pm_runtime_resume_and_get(dev);
    }
#[no_mangle]
unsafe extern "C" fn i2c_imx_resume(dev: *mut device) -> c_int {
    static int i2c_imx_resume(struct device *dev)
    {
    pm_runtime_put_autosuspend(dev);
    return 0;
    }
    static const struct dev_pm_ops i2c_imx_pm_ops = {
    NOIRQ_SYSTEM_SLEEP_PM_OPS(i2c_imx_suspend_noirq,
    i2c_imx_resume_noirq)
    SYSTEM_SLEEP_PM_OPS(i2c_imx_suspend, i2c_imx_resume)
    RUNTIME_PM_OPS(i2c_imx_runtime_suspend, i2c_imx_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver i2c_imx_driver = {
    .probe = i2c_imx_probe,
    .remove = i2c_imx_remove,
    .driver = {
    .name = DRIVER_NAME,
    .pm = pm_ptr(&i2c_imx_pm_ops),
    .of_match_table = i2c_imx_dt_ids,
    .acpi_match_table = i2c_imx_acpi_ids,
    },
    .id_table = imx_i2c_devtype,
    };
#[no_mangle]
unsafe extern "C" fn i2c_adap_imx_init() -> int __init {
    static int __init i2c_adap_imx_init(void)
    {
    return platform_driver_register(&i2c_imx_driver);
    }
    subsys_initcall(i2c_adap_imx_init);
#[no_mangle]
unsafe extern "C" fn i2c_adap_imx_exit() -> void __exit {
    static void __exit i2c_adap_imx_exit(void)
    {
    platform_driver_unregister(&i2c_imx_driver);
    }
    module_exit(i2c_adap_imx_exit);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Darius Augulis");
    MODULE_DESCRIPTION("I2C adapter driver for IMX I2C bus");
    MODULE_ALIAS("platform:" DRIVER_NAME);
