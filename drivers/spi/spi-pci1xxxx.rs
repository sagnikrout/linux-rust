//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-pci1xxxx.c
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
// PCI1xxxx SPI driver
// Copyright (C) 2022 Microchip Technology Inc.
// Authors: Tharun Kumar P <tharunkumar.pasumarthi@microchip.com>
// Kumaravel Thiagarajan <Kumaravel.Thiagarajan@microchip.com>

// DMA Related Registers

// x refers to SPI Host Controller HW instance id in the below macros - 0 or 1

pub const SPI_MAX_DATA_LEN: c_int = 320;

pub const SPI_CHIP_SEL_COUNT: c_int = 7;
pub const VENDOR_ID_MCHP: c_uint = 0x1055;
pub const SPI_SUSPEND_CONFIG: c_uint = 0x101;
pub const SPI_RESUME_CONFIG: c_uint = 0x203;
pub const NUM_VEC_PER_INST: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci1xxxx_spi_internal {
    pub hw_inst: u8,
    pub clkdiv: u8,
    pub irq: [c_int; NUM_VEC_PER_INST],
    pub mode: c_int,
    pub spi_xfer_in_progress: bool,
    pub dma_completion_count: core::sync::atomic::AtomicI32,
    pub rx_buf: *mut c_void,
    pub dma_aborted_rd: bool,
    pub bytes_recvd: u32,
    pub tx_sgl_len: u32,
    pub rx_sgl_len: u32,
    pub rx_sgl: *mut *mut scatterlist tx_sgl,,
    pub dma_aborted_wr: bool,
    pub spi_xfer_done: completion,
    pub spi_host: *mut spi_controller,
    pub parent: *mut pci1xxxx_spi,
    pub xfer: *mut spi_transfer,
    struct {
    pub 3: unsigned int dev_sel :,
    pub 1: unsigned int msi_vector_sel :,
    pub prev_val: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci1xxxx_spi {
    pub dev: *mut pci_dev,
    pub total_hw_instances: u8,
    pub dev_rev: u8,
    pub reg_base: *mut void __iomem,
    pub dma_offset_bar: *mut void __iomem,
// lock to safely access the DMA RD registers in isr
    pub dma_rd_reg_lock: spinlock_t,
// lock to safely access the DMA RD registers in isr
    pub dma_wr_reg_lock: spinlock_t,
    pub can_dma: bool,
    pub __counted_by(total_hw_instances): *mut *mut pci1xxxx_spi_internal spi_int[],
}

    static const struct pci_device_id pci1xxxx_spi_pci_id_table[] = {
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa004, PCI_ANY_ID, 0x0001), .driver_data = 0x02 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa004, PCI_ANY_ID, 0x0002), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa004, PCI_ANY_ID, 0x0003), .driver_data = 0x11 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa004, PCI_ANY_ID, PCI_ANY_ID), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa014, PCI_ANY_ID, 0x0001), .driver_data = 0x02 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa014, PCI_ANY_ID, 0x0002), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa014, PCI_ANY_ID, 0x0003), .driver_data = 0x11 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa014, PCI_ANY_ID, PCI_ANY_ID), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa024, PCI_ANY_ID, 0x0001), .driver_data = 0x02 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa024, PCI_ANY_ID, 0x0002), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa024, PCI_ANY_ID, 0x0003), .driver_data = 0x11 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa024, PCI_ANY_ID, PCI_ANY_ID), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa034, PCI_ANY_ID, 0x0001), .driver_data = 0x02 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa034, PCI_ANY_ID, 0x0002), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa034, PCI_ANY_ID, 0x0003), .driver_data = 0x11 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa034, PCI_ANY_ID, PCI_ANY_ID), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa044, PCI_ANY_ID, 0x0001), .driver_data = 0x02 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa044, PCI_ANY_ID, 0x0002), .driver_data = 0x01 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa044, PCI_ANY_ID, 0x0003), .driver_data = 0x11 },
    { PCI_DEVICE_SUB(VENDOR_ID_MCHP, 0xa044, PCI_ANY_ID, PCI_ANY_ID), .driver_data = 0x01 },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pci1xxxx_spi_pci_id_table);
    static irqreturn_t pci1xxxx_spi_isr_dma_rd(int irq, void *dev);
    static irqreturn_t pci1xxxx_spi_isr_dma_wr(int irq, void *dev);
#[no_mangle]
unsafe extern "C" fn pci1xxxx_set_sys_lock(par: *mut pci1xxxx_spi) -> c_int {
    static int pci1xxxx_set_sys_lock(struct pci1xxxx_spi *par)
    {
    writel(SPI_SYSLOCK, par.reg_base + SPI_SYSLOCK_REG);
    return readl(par.reg_base + SPI_SYSLOCK_REG);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_acquire_sys_lock(par: *mut pci1xxxx_spi) -> c_int {
    static int pci1xxxx_acquire_sys_lock(struct pci1xxxx_spi *par)
    {
    u32 regval;
    return readx_poll_timeout(pci1xxxx_set_sys_lock, par, regval,
    (regval & SPI_SYSLOCK), 100,
    SYSLOCK_RETRY_CNT * 100);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_release_sys_lock(par: *mut pci1xxxx_spi) {
    static void pci1xxxx_release_sys_lock(struct pci1xxxx_spi *par)
    {
    writel(0x0, par.reg_base + SPI_SYSLOCK_REG);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_check_spi_can_dma(spi_bus: *mut pci1xxxx_spi, hw_inst: c_int, num_vector: c_int) -> c_int {
    static int pci1xxxx_check_spi_can_dma(struct pci1xxxx_spi *spi_bus, int hw_inst, int num_vector)
    {
    struct pci_dev *pdev = spi_bus.dev;
    u32 pf_num;
    u32 regval;
    int ret;
    if (num_vector != hw_inst * NUM_VEC_PER_INST)
    return -EOPNOTSUPP;
//
// DEV REV Registers is a system register, HW Syslock bit
// should be acquired before accessing the register
//
    ret = pci1xxxx_acquire_sys_lock(spi_bus);
    if (ret) {
    dev_err(&pdev.dev, "Error failed to acquire syslock\n");
    return ret;
    }
    regval = readl(spi_bus.reg_base + DEV_REV_REG);
    spi_bus.dev_rev = regval & DEV_REV_MASK;
    if (spi_bus.dev_rev >= 0xC0) {
    regval = readl(spi_bus.reg_base +
    SPI_CONFIG_PERI_ENABLE_REG);
    pf_num = regval & SPI_PERI_ENBLE_PF_MASK;
    }
    pci1xxxx_release_sys_lock(spi_bus);
//
// DMA is supported only from C0 and SPI can use DMA only if
// it is mapped to PF0
//
    if (spi_bus.dev_rev < 0xC0 || pf_num)
    return -EOPNOTSUPP;
    spi_bus.dma_offset_bar = pcim_iomap(pdev, 2, pci_resource_len(pdev, 2));
    if (!spi_bus.dma_offset_bar) {
    dev_warn(&pdev.dev, "Error failed to map dma bar, will operate in PIO mode\n");
    return -EOPNOTSUPP;
    }
    if (dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64))) {
    dev_warn(&pdev.dev, "Error failed to set DMA mask, will operate in PIO mode\n");
    pcim_iounmap(pdev, spi_bus.dma_offset_bar);
    spi_bus.dma_offset_bar = core::ptr::null_mut();
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_dma_config(spi_bus: *mut pci1xxxx_spi) {
    static void pci1xxxx_spi_dma_config(struct pci1xxxx_spi *spi_bus)
    {
    struct pci1xxxx_spi_internal *spi_sub_ptr;
    u8 iter, irq_index;
    struct msi_msg msi;
    u32 regval;
    u16 data;
    irq_index = spi_bus.total_hw_instances;
    for (iter = 0; iter < spi_bus.total_hw_instances; iter++) {
    spi_sub_ptr = spi_bus.spi_int[iter];
    get_cached_msi_msg(spi_sub_ptr.irq[1], &msi);
    if (iter == 0) {
    writel(msi.address_hi, spi_bus.dma_offset_bar +
    SPI_DMA_INTR_IMWR_WDONE_HIGH);
    writel(msi.address_hi, spi_bus.dma_offset_bar +
    SPI_DMA_INTR_IMWR_WABORT_HIGH);
    writel(msi.address_hi, spi_bus.dma_offset_bar +
    SPI_DMA_INTR_IMWR_RDONE_HIGH);
    writel(msi.address_hi, spi_bus.dma_offset_bar +
    SPI_DMA_INTR_IMWR_RABORT_HIGH);
    writel(msi.address_lo, spi_bus.dma_offset_bar +
    SPI_DMA_INTR_IMWR_WDONE_LOW);
    writel(msi.address_lo, spi_bus.dma_offset_bar +
    SPI_DMA_INTR_IMWR_WABORT_LOW);
    writel(msi.address_lo, spi_bus.dma_offset_bar +
    SPI_DMA_INTR_IMWR_RDONE_LOW);
    writel(msi.address_lo, spi_bus.dma_offset_bar +
    SPI_DMA_INTR_IMWR_RABORT_LOW);
    writel(0, spi_bus.dma_offset_bar + SPI_DMA_INTR_WR_IMWR_DATA);
    writel(0, spi_bus.dma_offset_bar + SPI_DMA_INTR_RD_IMWR_DATA);
    }
    regval = readl(spi_bus.dma_offset_bar + SPI_DMA_INTR_WR_IMWR_DATA);
    data = msi.data + irq_index;
    writel((regval | (data << (iter * 16))), spi_bus.dma_offset_bar +
    SPI_DMA_INTR_WR_IMWR_DATA);
    regval = readl(spi_bus.dma_offset_bar + SPI_DMA_INTR_WR_IMWR_DATA);
    irq_index++;
    data = msi.data + irq_index;
    regval = readl(spi_bus.dma_offset_bar + SPI_DMA_INTR_RD_IMWR_DATA);
    writel(regval | (data << (iter * 16)), spi_bus.dma_offset_bar +
    SPI_DMA_INTR_RD_IMWR_DATA);
    regval = readl(spi_bus.dma_offset_bar + SPI_DMA_INTR_RD_IMWR_DATA);
    irq_index++;
    }
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_dma_init(spi_bus: *mut pci1xxxx_spi, hw_inst: c_int, num_vector: c_int) -> c_int {
    static int pci1xxxx_spi_dma_init(struct pci1xxxx_spi *spi_bus, int hw_inst, int num_vector)
    {
    struct pci1xxxx_spi_internal *spi_sub_ptr;
    u8 iter, irq_index;
    int ret;
    irq_index = hw_inst;
    ret = pci1xxxx_check_spi_can_dma(spi_bus, hw_inst, num_vector);
    if (ret)
    return ret;
    spin_lock_init(&spi_bus.dma_rd_reg_lock);
    spin_lock_init(&spi_bus.dma_wr_reg_lock);
    writel(SPI_DMA_ENGINE_EN, spi_bus.dma_offset_bar + SPI_DMA_GLOBAL_WR_ENGINE_EN);
    writel(SPI_DMA_ENGINE_EN, spi_bus.dma_offset_bar + SPI_DMA_GLOBAL_RD_ENGINE_EN);
    for (iter = 0; iter < hw_inst; iter++) {
    spi_sub_ptr = spi_bus.spi_int[iter];
    spi_sub_ptr.irq[1] = pci_irq_vector(spi_bus.dev, irq_index);
    ret = devm_request_irq(&spi_bus.dev.dev, spi_sub_ptr.irq[1],
    pci1xxxx_spi_isr_dma_wr, PCI1XXXX_IRQ_FLAGS,
    pci_name(spi_bus.dev), spi_sub_ptr);
    if (ret < 0)
    return ret;
    irq_index++;
    spi_sub_ptr.irq[2] = pci_irq_vector(spi_bus.dev, irq_index);
    ret = devm_request_irq(&spi_bus.dev.dev, spi_sub_ptr.irq[2],
    pci1xxxx_spi_isr_dma_rd, PCI1XXXX_IRQ_FLAGS,
    pci_name(spi_bus.dev), spi_sub_ptr);
    if (ret < 0)
    return ret;
    irq_index++;
    }
    pci1xxxx_spi_dma_config(spi_bus);
    dma_set_max_seg_size(&spi_bus.dev.dev, PCI1XXXX_SPI_BUFFER_SIZE);
    spi_bus.can_dma = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_set_cs(spi: *mut spi_device, enable: bool) {
    static void pci1xxxx_spi_set_cs(struct spi_device *spi, bool enable)
    {
    struct pci1xxxx_spi_internal *p = spi_controller_get_devdata(spi.controller);
    struct pci1xxxx_spi *par = p.parent;
    u32 regval;
// Set the DEV_SEL bits of the SPI_MST_CTL_REG
    regval = readl(par.reg_base + SPI_MST_CTL_REG_OFFSET(p.hw_inst));
    if (!enable) {
    regval |= SPI_FORCE_CE;
    regval &= ~SPI_MST_CTL_DEVSEL_MASK;
    regval |= (spi_get_chipselect(spi, 0) << 25);
    } else {
    regval &= ~SPI_FORCE_CE;
    }
    writel(regval, par.reg_base + SPI_MST_CTL_REG_OFFSET(p.hw_inst));
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_get_clock_div(par: *mut pci1xxxx_spi, hz: u32) -> u8 {
    static u8 pci1xxxx_get_clock_div(struct pci1xxxx_spi *par, u32 hz)
    {
    let mut val: u8 = 0;
    if (hz >= PCI1XXXX_SPI_MAX_CLOCK_HZ)
    val = 2;
#[no_mangle]
pub unsafe extern "C" fn if(PCI1XXXX_SPI_CLK_25MHZ: par->dev_rev >= 0xC0 && hz >=) -> else {
    else if (par.dev_rev >= 0xC0 && hz >= PCI1XXXX_SPI_CLK_25MHZ)
    val = 1;
#[no_mangle]
pub unsafe extern "C" fn if(PCI1XXXX_SPI_CLK_20MHZ): (hz < PCI1XXXX_SPI_MAX_CLOCK_HZ) && (hz >=) -> else {
    else if ((hz < PCI1XXXX_SPI_MAX_CLOCK_HZ) && (hz >= PCI1XXXX_SPI_CLK_20MHZ))
    val = 3;
#[no_mangle]
pub unsafe extern "C" fn if(PCI1XXXX_SPI_CLK_15MHZ): (hz < PCI1XXXX_SPI_CLK_20MHZ) && (hz >=) -> else {
    else if ((hz < PCI1XXXX_SPI_CLK_20MHZ) && (hz >= PCI1XXXX_SPI_CLK_15MHZ))
    val = 4;
#[no_mangle]
pub unsafe extern "C" fn if(PCI1XXXX_SPI_CLK_12MHZ): (hz < PCI1XXXX_SPI_CLK_15MHZ) && (hz >=) -> else {
    else if ((hz < PCI1XXXX_SPI_CLK_15MHZ) && (hz >= PCI1XXXX_SPI_CLK_12MHZ))
    val = 5;
#[no_mangle]
pub unsafe extern "C" fn if(PCI1XXXX_SPI_CLK_10MHZ): (hz < PCI1XXXX_SPI_CLK_12MHZ) && (hz >=) -> else {
    else if ((hz < PCI1XXXX_SPI_CLK_12MHZ) && (hz >= PCI1XXXX_SPI_CLK_10MHZ))
    val = 6;
#[no_mangle]
pub unsafe extern "C" fn if(PCI1XXXX_SPI_MIN_CLOCK_HZ): (hz < PCI1XXXX_SPI_CLK_10MHZ) && (hz >=) -> else {
    else if ((hz < PCI1XXXX_SPI_CLK_10MHZ) && (hz >= PCI1XXXX_SPI_MIN_CLOCK_HZ))
    val = 7;
    else
    val = 2;
    return val;
    }
    static void pci1xxxx_spi_setup_dma_to_io(struct pci1xxxx_spi_internal *p,
    dma_addr_t dma_addr, u32 len)
    {
    void __iomem *base;
    if (!p.hw_inst)
    base = p.parent.dma_offset_bar + SPI_DMA_CH0_RD_BASE;
    else
    base = p.parent.dma_offset_bar + SPI_DMA_CH1_RD_BASE;
    writel(DMA_INTR_EN, base + SPI_DMA_CH_CTL1_OFFSET);
    writel(len, base + SPI_DMA_CH_XFER_LEN_OFFSET);
    writel(lower_32_bits(dma_addr), base + SPI_DMA_CH_SAR_LO_OFFSET);
    writel(upper_32_bits(dma_addr), base + SPI_DMA_CH_SAR_HI_OFFSET);
// Updated SPI Command Registers
    writel(lower_32_bits(SPI_PERI_ADDR_BASE + SPI_MST_CMD_BUF_OFFSET(p.hw_inst)),
    base + SPI_DMA_CH_DAR_LO_OFFSET);
    writel(upper_32_bits(SPI_PERI_ADDR_BASE + SPI_MST_CMD_BUF_OFFSET(p.hw_inst)),
    base + SPI_DMA_CH_DAR_HI_OFFSET);
    }
    static void pci1xxxx_spi_setup_dma_from_io(struct pci1xxxx_spi_internal *p,
    dma_addr_t dma_addr, u32 len)
    {
    void *base;
    if (!p.hw_inst)
    base = p.parent.dma_offset_bar + SPI_DMA_CH0_WR_BASE;
    else
    base = p.parent.dma_offset_bar + SPI_DMA_CH1_WR_BASE;
    writel(DMA_INTR_EN, base + SPI_DMA_CH_CTL1_OFFSET);
    writel(len, base + SPI_DMA_CH_XFER_LEN_OFFSET);
    writel(lower_32_bits(dma_addr), base + SPI_DMA_CH_DAR_LO_OFFSET);
    writel(upper_32_bits(dma_addr), base + SPI_DMA_CH_DAR_HI_OFFSET);
    writel(lower_32_bits(SPI_PERI_ADDR_BASE + SPI_MST_RSP_BUF_OFFSET(p.hw_inst)),
    base + SPI_DMA_CH_SAR_LO_OFFSET);
    writel(upper_32_bits(SPI_PERI_ADDR_BASE + SPI_MST_RSP_BUF_OFFSET(p.hw_inst)),
    base + SPI_DMA_CH_SAR_HI_OFFSET);
    }
    static void pci1xxxx_spi_setup(struct pci1xxxx_spi *par, u8 hw_inst, u32 mode,
    u8 clkdiv, u32 len)
    {
    u32 regval;
    regval = readl(par.reg_base + SPI_MST_CTL_REG_OFFSET(hw_inst));
    regval &= ~(SPI_MST_CTL_MODE_SEL | SPI_MST_CTL_CMD_LEN_MASK |
    SPI_MST_CTL_SPEED_MASK);
    if (mode == SPI_MODE_3)
    regval |= SPI_MST_CTL_MODE_SEL;
    regval |= FIELD_PREP(SPI_MST_CTL_CMD_LEN_MASK, len);
    regval |= FIELD_PREP(SPI_MST_CTL_SPEED_MASK, clkdiv);
    writel(regval, par.reg_base + SPI_MST_CTL_REG_OFFSET(hw_inst));
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_start_spi_xfer(p: *mut pci1xxxx_spi_internal) {
    static void pci1xxxx_start_spi_xfer(struct pci1xxxx_spi_internal *p)
    {
    u32 regval;
    atomic_set(&p.dma_completion_count, 0);
    regval = readl(p.parent.reg_base + SPI_MST_CTL_REG_OFFSET(p.hw_inst));
    regval |= SPI_MST_CTL_GO;
    writel(regval, p.parent.reg_base + SPI_MST_CTL_REG_OFFSET(p.hw_inst));
    }
    static int pci1xxxx_spi_transfer_with_io(struct spi_controller *spi_ctlr,
    struct spi_device *spi, struct spi_transfer *xfer)
    {
    struct pci1xxxx_spi_internal *p = spi_controller_get_devdata(spi_ctlr);
    struct pci1xxxx_spi *par = p.parent;
    int len, loop_iter, transfer_len;
    unsigned long bytes_transfered;
    unsigned long bytes_recvd;
    unsigned long loop_count;
    u8 *rx_buf, result;
    const u8 *tx_buf;
    u32 regval;
    u8 clkdiv;
    p.spi_xfer_in_progress = true;
    p.bytes_recvd = 0;
    clkdiv = pci1xxxx_get_clock_div(par, xfer.speed_hz);
    tx_buf = xfer.tx_buf;
    rx_buf = xfer.rx_buf;
    transfer_len = xfer.len;
    regval = readl(par.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    writel(regval, par.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    if (tx_buf) {
    bytes_transfered = 0;
    bytes_recvd = 0;
    loop_count = transfer_len / SPI_MAX_DATA_LEN;
    if (transfer_len % SPI_MAX_DATA_LEN != 0)
    loop_count += 1;
    for (loop_iter = 0; loop_iter < loop_count; loop_iter++) {
    len = SPI_MAX_DATA_LEN;
    if ((transfer_len % SPI_MAX_DATA_LEN != 0) &&
    (loop_iter == loop_count - 1))
    len = transfer_len % SPI_MAX_DATA_LEN;
    reinit_completion(&p.spi_xfer_done);
    memcpy_toio(par.reg_base + SPI_MST_CMD_BUF_OFFSET(p.hw_inst),
    &tx_buf[bytes_transfered], len);
    bytes_transfered += len;
    pci1xxxx_spi_setup(par, p.hw_inst, spi.mode, clkdiv, len);
    pci1xxxx_start_spi_xfer(p);
// Wait for DMA_TERM interrupt
    result = wait_for_completion_timeout(&p.spi_xfer_done,
    PCI1XXXX_SPI_TIMEOUT);
    if (!result)
    return -ETIMEDOUT;
    if (rx_buf) {
    memcpy_fromio(&rx_buf[bytes_recvd], par.reg_base +
    SPI_MST_RSP_BUF_OFFSET(p.hw_inst), len);
    bytes_recvd += len;
    }
    }
    }
    p.spi_xfer_in_progress = false;
    return 0;
    }
    static int pci1xxxx_spi_transfer_with_dma(struct spi_controller *spi_ctlr,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct pci1xxxx_spi_internal *p = spi_controller_get_devdata(spi_ctlr);
    struct pci1xxxx_spi *par = p.parent;
    let mut tx_dma_addr: dma_addr_t = 0;
    let mut ret: c_int = 0;
    u32 regval;
    p.spi_xfer_in_progress = true;
    p.tx_sgl = xfer.tx_sg.sgl;
    p.rx_sgl = xfer.rx_sg.sgl;
    p.rx_buf = xfer.rx_buf;
    atomic_set(&p.dma_completion_count, 1);
    regval = readl(par.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    writel(regval, par.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    if (!xfer.tx_buf || !p.tx_sgl) {
    ret = -EINVAL;
    goto error;
    }
    p.xfer = xfer;
    p.mode = spi.mode;
    p.clkdiv = pci1xxxx_get_clock_div(par, xfer.speed_hz);
    p.bytes_recvd = 0;
    p.rx_buf = xfer.rx_buf;
    regval = readl(par.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    writel(regval, par.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    tx_dma_addr = sg_dma_address(p.tx_sgl);
    p.tx_sgl_len = sg_dma_len(p.tx_sgl);
    pci1xxxx_spi_setup(par, p.hw_inst, p.mode, p.clkdiv, p.tx_sgl_len);
    pci1xxxx_spi_setup_dma_to_io(p, (tx_dma_addr), p.tx_sgl_len);
    writel(p.hw_inst, par.dma_offset_bar + SPI_DMA_RD_DOORBELL_REG);
    reinit_completion(&p.spi_xfer_done);
// Wait for DMA_TERM interrupt
    ret = wait_for_completion_timeout(&p.spi_xfer_done, PCI1XXXX_SPI_TIMEOUT);
    if (!ret) {
    ret = -ETIMEDOUT;
    if (p.dma_aborted_rd) {
    writel(SPI_DMA_ENGINE_DIS,
    par.dma_offset_bar + SPI_DMA_GLOBAL_RD_ENGINE_EN);
//
// DMA ENGINE reset takes time if any TLP
// completeion in progress, should wait
// till DMA Engine reset is completed.
//
    ret = readl_poll_timeout(par.dma_offset_bar +
    SPI_DMA_GLOBAL_RD_ENGINE_EN, regval,
    (regval == 0x0), 0, USEC_PER_MSEC);
    if (ret) {
    ret = -ECANCELED;
    goto error;
    }
    writel(SPI_DMA_ENGINE_EN,
    par.dma_offset_bar + SPI_DMA_GLOBAL_RD_ENGINE_EN);
    p.dma_aborted_rd = false;
    ret = -ECANCELED;
    }
    if (p.dma_aborted_wr) {
    writel(SPI_DMA_ENGINE_DIS,
    par.dma_offset_bar + SPI_DMA_GLOBAL_WR_ENGINE_EN);
//
// DMA ENGINE reset takes time if any TLP
// completeion in progress, should wait
// till DMA Engine reset is completed.
//
    ret = readl_poll_timeout(par.dma_offset_bar +
    SPI_DMA_GLOBAL_WR_ENGINE_EN, regval,
    (regval == 0x0), 0, USEC_PER_MSEC);
    if (ret) {
    ret = -ECANCELED;
    goto error;
    }
    writel(SPI_DMA_ENGINE_EN,
    par.dma_offset_bar + SPI_DMA_GLOBAL_WR_ENGINE_EN);
    p.dma_aborted_wr = false;
    ret = -ECANCELED;
    }
    goto error;
    }
    ret = 0;
    error:
    p.spi_xfer_in_progress = false;
    return ret;
    }
    static int pci1xxxx_spi_transfer_one(struct spi_controller *spi_ctlr,
    struct spi_device *spi, struct spi_transfer *xfer)
    {
    if (spi_xfer_is_dma_mapped(spi_ctlr, spi, xfer))
    return pci1xxxx_spi_transfer_with_dma(spi_ctlr, spi, xfer);
    else
    return pci1xxxx_spi_transfer_with_io(spi_ctlr, spi, xfer);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_isr_io(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci1xxxx_spi_isr_io(int irq, void *dev)
    {
    struct pci1xxxx_spi_internal *p = dev;
    let mut spi_int_fired: irqreturn_t = IRQ_NONE;
    u32 regval;
// Clear the SPI GO_BIT Interrupt
    regval = readl(p.parent.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    if (regval & SPI_INTR) {
// Clear xfer_done
    if (p.parent.can_dma && p.rx_buf)
    writel(p.hw_inst, p.parent.dma_offset_bar +
    SPI_DMA_WR_DOORBELL_REG);
    else
    complete(&p.parent.spi_int[p.hw_inst].spi_xfer_done);
    spi_int_fired = IRQ_HANDLED;
    }
    writel(regval, p.parent.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    return spi_int_fired;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_setup_next_dma_to_io_transfer(p: *mut pci1xxxx_spi_internal) {
    static void pci1xxxx_spi_setup_next_dma_to_io_transfer(struct pci1xxxx_spi_internal *p)
    {
    let mut tx_dma_addr: dma_addr_t = 0;
    u32 prev_len;
    p.tx_sgl = sg_next(p.tx_sgl);
    if (p.tx_sgl) {
    tx_dma_addr = sg_dma_address(p.tx_sgl);
    prev_len = p.tx_sgl_len;
    p.tx_sgl_len = sg_dma_len(p.tx_sgl);
    pci1xxxx_spi_setup_dma_to_io(p, tx_dma_addr, p.tx_sgl_len);
    writel(p.hw_inst, p.parent.dma_offset_bar + SPI_DMA_RD_DOORBELL_REG);
    if (prev_len != p.tx_sgl_len)
    pci1xxxx_spi_setup(p.parent,
    p.hw_inst, p.mode, p.clkdiv, p.tx_sgl_len);
    }
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_setup_next_dma_from_io_transfer(p: *mut pci1xxxx_spi_internal) {
    static void pci1xxxx_spi_setup_next_dma_from_io_transfer(struct pci1xxxx_spi_internal *p)
    {
    let mut rx_dma_addr: dma_addr_t = 0;
    if (p.rx_sgl) {
    rx_dma_addr = sg_dma_address(p.rx_sgl);
    p.rx_sgl_len = sg_dma_len(p.rx_sgl);
    pci1xxxx_spi_setup_dma_from_io(p, rx_dma_addr, p.rx_sgl_len);
    writel(p.hw_inst, p.parent.dma_offset_bar + SPI_DMA_WR_DOORBELL_REG);
    }
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_isr_dma_rd(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci1xxxx_spi_isr_dma_rd(int irq, void *dev)
    {
    struct pci1xxxx_spi_internal *p = dev;
    let mut spi_int_fired: irqreturn_t = IRQ_NONE;
    unsigned long flags;
    u32 regval;
// Clear the DMA RD INT and start spi xfer
    regval = readl(p.parent.dma_offset_bar + SPI_DMA_INTR_RD_STS);
    if (regval) {
    if (regval & SPI_DMA_DONE_INT_MASK(p.hw_inst)) {
// Start the SPI transfer only if both DMA read and write are completed
    if (atomic_inc_return(&p.dma_completion_count) == 2)
    pci1xxxx_start_spi_xfer(p);
    spi_int_fired = IRQ_HANDLED;
    }
    if (regval & SPI_DMA_ABORT_INT_MASK(p.hw_inst)) {
    p.dma_aborted_rd = true;
    spi_int_fired = IRQ_HANDLED;
    }
    spin_lock_irqsave(&p.parent.dma_rd_reg_lock, flags);
    writel((SPI_DMA_DONE_INT_MASK(p.hw_inst) | SPI_DMA_ABORT_INT_MASK(p.hw_inst)),
    p.parent.dma_offset_bar + SPI_DMA_INTR_RD_CLR);
    spin_unlock_irqrestore(&p.parent.dma_rd_reg_lock, flags);
    }
    return spi_int_fired;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_isr_dma_wr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci1xxxx_spi_isr_dma_wr(int irq, void *dev)
    {
    struct pci1xxxx_spi_internal *p = dev;
    let mut spi_int_fired: irqreturn_t = IRQ_NONE;
    unsigned long flags;
    u32 regval;
// Clear the DMA WR INT
    regval = readl(p.parent.dma_offset_bar + SPI_DMA_INTR_WR_STS);
    if (regval) {
    if (regval & SPI_DMA_DONE_INT_MASK(p.hw_inst)) {
    spi_int_fired = IRQ_HANDLED;
    if (sg_is_last(p.rx_sgl)) {
    complete(&p.spi_xfer_done);
    } else {
    p.rx_sgl =  sg_next(p.rx_sgl);
    if (atomic_inc_return(&p.dma_completion_count) == 2)
    pci1xxxx_start_spi_xfer(p);
    }
    }
    if (regval & SPI_DMA_ABORT_INT_MASK(p.hw_inst)) {
    p.dma_aborted_wr = true;
    spi_int_fired = IRQ_HANDLED;
    }
    spin_lock_irqsave(&p.parent.dma_wr_reg_lock, flags);
    writel((SPI_DMA_DONE_INT_MASK(p.hw_inst) | SPI_DMA_ABORT_INT_MASK(p.hw_inst)),
    p.parent.dma_offset_bar + SPI_DMA_INTR_WR_CLR);
    spin_unlock_irqrestore(&p.parent.dma_wr_reg_lock, flags);
    }
    return spi_int_fired;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_isr_dma(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci1xxxx_spi_isr_dma(int irq, void *dev)
    {
    struct pci1xxxx_spi_internal *p = dev;
    let mut spi_int_fired: irqreturn_t = IRQ_NONE;
    u32 regval;
// Clear the SPI GO_BIT Interrupt
    regval = readl(p.parent.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    if (regval & SPI_INTR) {
    pci1xxxx_spi_setup_next_dma_from_io_transfer(p);
    pci1xxxx_spi_setup_next_dma_to_io_transfer(p);
    spi_int_fired = IRQ_HANDLED;
    writel(regval, p.parent.reg_base + SPI_MST_EVENT_REG_OFFSET(p.hw_inst));
    }
    return spi_int_fired;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci1xxxx_spi_isr(int irq, void *dev)
    {
    struct pci1xxxx_spi_internal *p = dev;
    if (p.spi_host.can_dma(p.spi_host, core::ptr::null_mut(), p.xfer))
    return pci1xxxx_spi_isr_dma(irq, dev);
    else
    return pci1xxxx_spi_isr_io(irq, dev);
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_shared_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci1xxxx_spi_shared_isr(int irq, void *dev)
    {
    struct pci1xxxx_spi *par = dev;
    let mut i: u8 = 0;
    for (i = 0; i < par.total_hw_instances; i++)
    pci1xxxx_spi_isr(irq, par.spi_int[i]);
    return IRQ_HANDLED;
    }
    static bool pci1xxxx_spi_can_dma(struct spi_controller *host,
    struct spi_device *spi,
    struct spi_transfer *xfer)
    {
    struct pci1xxxx_spi_internal *p = spi_controller_get_devdata(host);
    struct pci1xxxx_spi *par = p.parent;
    return par.can_dma;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_probe(pdev: *mut pci_dev, ent: *const pci_device_id) -> c_int {
    static int pci1xxxx_spi_probe(struct pci_dev *pdev, const struct pci_device_id *ent)
    {
    u8 hw_inst_cnt, iter, start, only_sec_inst;
    struct pci1xxxx_spi_internal *spi_sub_ptr;
    struct device *dev = &pdev.dev;
    struct pci1xxxx_spi *spi_bus;
    struct spi_controller *spi_host;
    let mut num_vector: c_int = 0;
    u32 regval;
    int ret;
    hw_inst_cnt = ent.driver_data & 0x0f;
    start = (ent.driver_data & 0xf0) >> 4;
    if (start == 1)
    only_sec_inst = 1;
    else
    only_sec_inst = 0;
    spi_bus = devm_kzalloc(&pdev.dev,
    struct_size(spi_bus, spi_int, hw_inst_cnt),
    GFP_KERNEL);
    if (!spi_bus)
    return -ENOMEM;
    spi_bus.dev = pdev;
    spi_bus.total_hw_instances = hw_inst_cnt;
    pci_set_master(pdev);
    for (iter = 0; iter < hw_inst_cnt; iter++) {
    spi_bus.spi_int[iter] = devm_kzalloc(&pdev.dev,
    sizeof(struct pci1xxxx_spi_internal),
    GFP_KERNEL);
    if (!spi_bus.spi_int[iter])
    return -ENOMEM;
    spi_sub_ptr = spi_bus.spi_int[iter];
    spi_sub_ptr.spi_host = devm_spi_alloc_host(dev, sizeof(struct spi_controller));
    if (!spi_sub_ptr.spi_host)
    return -ENOMEM;
    spi_sub_ptr.parent = spi_bus;
    spi_sub_ptr.spi_xfer_in_progress = false;
    if (!iter) {
    ret = pcim_enable_device(pdev);
    if (ret)
    return -ENOMEM;
    ret = pcim_request_all_regions(pdev, DRV_NAME);
    if (ret)
    return -ENOMEM;
    spi_bus.reg_base = pcim_iomap(pdev, 0, pci_resource_len(pdev, 0));
    if (!spi_bus.reg_base)
    return -EINVAL;
    num_vector = pci_alloc_irq_vectors(pdev, 1, hw_inst_cnt * NUM_VEC_PER_INST,
    PCI_IRQ_INTX | PCI_IRQ_MSI);
    if (num_vector < 0) {
    dev_err(&pdev.dev, "Error allocating MSI vectors\n");
    return num_vector;
    }
    init_completion(&spi_sub_ptr.spi_xfer_done);
// Initialize Interrupts - SPI_INT
    regval = readl(spi_bus.reg_base +
    SPI_MST_EVENT_MASK_REG_OFFSET(spi_sub_ptr.hw_inst));
    regval &= ~SPI_INTR;
    writel(regval, spi_bus.reg_base +
    SPI_MST_EVENT_MASK_REG_OFFSET(spi_sub_ptr.hw_inst));
    spi_sub_ptr.irq[0] = pci_irq_vector(pdev, 0);
    if (num_vector >= hw_inst_cnt)
    ret = devm_request_irq(&pdev.dev, spi_sub_ptr.irq[0],
    pci1xxxx_spi_isr, PCI1XXXX_IRQ_FLAGS,
    pci_name(pdev), spi_sub_ptr);
    else
    ret = devm_request_irq(&pdev.dev, spi_sub_ptr.irq[0],
    pci1xxxx_spi_shared_isr,
    PCI1XXXX_IRQ_FLAGS | IRQF_SHARED,
    pci_name(pdev), spi_bus);
    if (ret < 0) {
    dev_err(&pdev.dev, "Unable to request irq : %d",
    spi_sub_ptr.irq[0]);
    return -ENODEV;
    }
// This register is only applicable for 1st instance
    regval = readl(spi_bus.reg_base + SPI_PCI_CTRL_REG_OFFSET(0));
    if (!only_sec_inst)
    regval |= (BIT(4));
    else
    regval &= ~(BIT(4));
    writel(regval, spi_bus.reg_base + SPI_PCI_CTRL_REG_OFFSET(0));
    }
    spi_sub_ptr.hw_inst = start++;
    if (iter == 1) {
    init_completion(&spi_sub_ptr.spi_xfer_done);
// Initialize Interrupts - SPI_INT
    regval = readl(spi_bus.reg_base +
    SPI_MST_EVENT_MASK_REG_OFFSET(spi_sub_ptr.hw_inst));
    regval &= ~SPI_INTR;
    writel(regval, spi_bus.reg_base +
    SPI_MST_EVENT_MASK_REG_OFFSET(spi_sub_ptr.hw_inst));
    if (num_vector >= hw_inst_cnt) {
    spi_sub_ptr.irq[0] = pci_irq_vector(pdev, iter);
    ret = devm_request_irq(&pdev.dev, spi_sub_ptr.irq[0],
    pci1xxxx_spi_isr, PCI1XXXX_IRQ_FLAGS,
    pci_name(pdev), spi_sub_ptr);
    if (ret < 0) {
    dev_err(&pdev.dev, "Unable to request irq : %d",
    spi_sub_ptr.irq[0]);
    return -ENODEV;
    }
    }
    }
    spi_host = spi_sub_ptr.spi_host;
    spi_host.num_chipselect = SPI_CHIP_SEL_COUNT;
    spi_host.mode_bits = SPI_MODE_0 | SPI_MODE_3 | SPI_RX_DUAL |
    SPI_TX_DUAL | SPI_LOOP;
    spi_host.can_dma = pci1xxxx_spi_can_dma;
    spi_host.transfer_one = pci1xxxx_spi_transfer_one;
    spi_host.set_cs = pci1xxxx_spi_set_cs;
    spi_host.bits_per_word_mask = SPI_BPW_MASK(8);
    spi_host.max_speed_hz = PCI1XXXX_SPI_MAX_CLOCK_HZ;
    spi_host.min_speed_hz = PCI1XXXX_SPI_MIN_CLOCK_HZ;
    spi_host.flags = SPI_CONTROLLER_MUST_TX;
    spi_controller_set_devdata(spi_host, spi_sub_ptr);
    ret = devm_spi_register_controller(dev, spi_host);
    if (ret)
    return ret;
    }
    ret = pci1xxxx_spi_dma_init(spi_bus, hw_inst_cnt, num_vector);
    if (ret && ret != -EOPNOTSUPP)
    return ret;
    pci_set_drvdata(pdev, spi_bus);
    return 0;
    }
    static void store_restore_config(struct pci1xxxx_spi *spi_ptr,
    struct pci1xxxx_spi_internal *spi_sub_ptr,
    u8 inst, bool store)
    {
    u32 regval;
    if (store) {
    regval = readl(spi_ptr.reg_base +
    SPI_MST_CTL_REG_OFFSET(spi_sub_ptr.hw_inst));
    regval &= SPI_MST_CTL_DEVSEL_MASK;
    spi_sub_ptr.prev_val.dev_sel = (regval >> 25) & 7;
    regval = readl(spi_ptr.reg_base +
    SPI_PCI_CTRL_REG_OFFSET(spi_sub_ptr.hw_inst));
    regval &= SPI_MSI_VECTOR_SEL_MASK;
    spi_sub_ptr.prev_val.msi_vector_sel = (regval >> 4) & 1;
    } else {
    regval = readl(spi_ptr.reg_base + SPI_MST_CTL_REG_OFFSET(inst));
    regval &= ~SPI_MST_CTL_DEVSEL_MASK;
    regval |= (spi_sub_ptr.prev_val.dev_sel << 25);
    writel(regval,
    spi_ptr.reg_base + SPI_MST_CTL_REG_OFFSET(inst));
    writel((spi_sub_ptr.prev_val.msi_vector_sel << 4),
    spi_ptr.reg_base + SPI_PCI_CTRL_REG_OFFSET(inst));
    }
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_resume(dev: *mut device) -> c_int {
    static int pci1xxxx_spi_resume(struct device *dev)
    {
    struct pci1xxxx_spi *spi_ptr = dev_get_drvdata(dev);
    struct pci1xxxx_spi_internal *spi_sub_ptr;
    let mut regval: u32 = SPI_RESUME_CONFIG;
    u8 iter;
    for (iter = 0; iter < spi_ptr.total_hw_instances; iter++) {
    spi_sub_ptr = spi_ptr.spi_int[iter];
    spi_controller_resume(spi_sub_ptr.spi_host);
    writel(regval, spi_ptr.reg_base +
    SPI_MST_EVENT_MASK_REG_OFFSET(iter));
// Restore config at resume
    store_restore_config(spi_ptr, spi_sub_ptr, iter, 0);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci1xxxx_spi_suspend(dev: *mut device) -> c_int {
    static int pci1xxxx_spi_suspend(struct device *dev)
    {
    struct pci1xxxx_spi *spi_ptr = dev_get_drvdata(dev);
    struct pci1xxxx_spi_internal *spi_sub_ptr;
    let mut reg1: u32 = SPI_SUSPEND_CONFIG;
    u8 iter;
    for (iter = 0; iter < spi_ptr.total_hw_instances; iter++) {
    spi_sub_ptr = spi_ptr.spi_int[iter];
    while (spi_sub_ptr.spi_xfer_in_progress)
    msleep(20);
// Store existing config before suspend
    store_restore_config(spi_ptr, spi_sub_ptr, iter, 1);
    spi_controller_suspend(spi_sub_ptr.spi_host);
    writel(reg1, spi_ptr.reg_base +
    SPI_MST_EVENT_MASK_REG_OFFSET(iter));
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(spi_pm_ops, pci1xxxx_spi_suspend,
    pci1xxxx_spi_resume);
    static struct pci_driver pci1xxxx_spi_driver = {
    .name		= DRV_NAME,
    .id_table	= pci1xxxx_spi_pci_id_table,
    .probe		= pci1xxxx_spi_probe,
    .driver		=	{
    .pm = pm_sleep_ptr(&spi_pm_ops),
    },
    };
    module_pci_driver(pci1xxxx_spi_driver);
    MODULE_DESCRIPTION("Microchip Technology Inc. pci1xxxx SPI bus driver");
    MODULE_AUTHOR("Tharun Kumar P<tharunkumar.pasumarthi@microchip.com>");
    MODULE_AUTHOR("Kumaravel Thiagarajan<kumaravel.thiagarajan@microchip.com>");
    MODULE_LICENSE("GPL v2");
