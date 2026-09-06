//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/plda/pcie-microchip-host.c
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
// Microchip AXI PCIe Bridge host controller driver
//
// Copyright (c) 2018 - 2020 Microchip Corporation. All rights reserved.
//
// Author: Daire McNamara <daire.mcnamara@microchip.com>
//

pub const MC_MAX_NUM_INBOUND_WINDOWS: c_int = 8;
pub const MPFS_NC_BOUNCE_ADDR: c_uint = 0x80000000;
// PCIe Bridge Phy and Controller Phy offsets
pub const MC_PCIE1_BRIDGE_ADDR: c_uint = 0x00008000u;
pub const MC_PCIE1_CTRL_ADDR: c_uint = 0x0000a000u;
// PCIe Controller Phy Regs
pub const SEC_ERROR_EVENT_CNT: c_uint = 0x20;
pub const DED_ERROR_EVENT_CNT: c_uint = 0x24;
pub const SEC_ERROR_INT: c_uint = 0x28;

pub const SEC_ERROR_INT_MASK: c_uint = 0x2c;
pub const DED_ERROR_INT: c_uint = 0x30;

pub const DED_ERROR_INT_MASK: c_uint = 0x34;
pub const ECC_CONTROL: c_uint = 0x38;

pub const PCIE_EVENT_INT: c_uint = 0x14c;

pub const PCIE_EVENT_INT_ENB_SHIFT: c_int = 16;

// PCIe Config space MSI capability structure
pub const MC_MSI_CAP_CTRL_OFFSET: c_uint = 0xe0u;
// Events
pub const EVENT_PCIE_L2_EXIT: c_int = 0;
pub const EVENT_PCIE_HOTRST_EXIT: c_int = 1;
pub const EVENT_PCIE_DLUP_EXIT: c_int = 2;
pub const EVENT_SEC_TX_RAM_SEC_ERR: c_int = 3;
pub const EVENT_SEC_RX_RAM_SEC_ERR: c_int = 4;
pub const EVENT_SEC_PCIE2AXI_RAM_SEC_ERR: c_int = 5;
pub const EVENT_SEC_AXI2PCIE_RAM_SEC_ERR: c_int = 6;
pub const EVENT_DED_TX_RAM_DED_ERR: c_int = 7;
pub const EVENT_DED_RX_RAM_DED_ERR: c_int = 8;
pub const EVENT_DED_PCIE2AXI_RAM_DED_ERR: c_int = 9;
pub const EVENT_DED_AXI2PCIE_RAM_DED_ERR: c_int = 10;
pub const EVENT_LOCAL_DMA_END_ENGINE_0: c_int = 11;
pub const EVENT_LOCAL_DMA_END_ENGINE_1: c_int = 12;
pub const EVENT_LOCAL_DMA_ERROR_ENGINE_0: c_int = 13;
pub const EVENT_LOCAL_DMA_ERROR_ENGINE_1: c_int = 14;
pub const NUM_MC_EVENTS: c_int = 15;

    [EVENT_PCIE_ ## x] = { __stringify(x), s }

    [EVENT_SEC_ ## x] = { __stringify(x), s }

    [EVENT_DED_ ## x] = { __stringify(x), s }

    [EVENT_LOCAL_ ## x] = { __stringify(x), s }

    .offset = PCIE_EVENT_INT, \
    .mask_offset = PCIE_EVENT_INT, \
    .mask_high = 1, \
    .mask = PCIE_EVENT_INT_ ## x ## _INT, \
    .enb_mask = PCIE_EVENT_INT_ENB_MASK

    .offset = SEC_ERROR_INT, \
    .mask_offset = SEC_ERROR_INT_MASK, \
    .mask = SEC_ERROR_INT_ ## x ## _INT, \
    .mask_high = 1, \
    .enb_mask = 0

    .offset = DED_ERROR_INT, \
    .mask_offset = DED_ERROR_INT_MASK, \
    .mask_high = 1, \
    .mask = DED_ERROR_INT_ ## x ## _INT, \
    .enb_mask = 0

    .offset = ISTATUS_LOCAL, \
    .mask_offset = IMASK_LOCAL, \
    .mask_high = 0, \
    .mask = x ## _MASK, \
    .enb_mask = 0

    { PCIE_EVENT_INT_ ## x ## _INT, EVENT_PCIE_ ## x }

    { SEC_ERROR_INT_ ## x ## _INT, EVENT_SEC_ ## x }

    { DED_ERROR_INT_ ## x ## _INT, EVENT_DED_ ## x }

    { x ## _MASK, EVENT_LOCAL_ ## x }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_map {
    pub reg_mask: u32,
    pub event_bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mc_pcie {
    pub plda: plda_pcie_rp,
    pub bridge_base_addr: *mut void __iomem,
    pub ctrl_base_addr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cause {
    pub sym: *const c_char,
    pub str: *const c_char,
}

    static const struct cause event_cause[NUM_EVENTS] = {
    PCIE_EVENT_CAUSE(L2_EXIT, "L2 exit event"),
    PCIE_EVENT_CAUSE(HOTRST_EXIT, "Hot reset exit event"),
    PCIE_EVENT_CAUSE(DLUP_EXIT, "DLUP exit event"),
    SEC_ERROR_CAUSE(TX_RAM_SEC_ERR,  "sec error in tx buffer"),
    SEC_ERROR_CAUSE(RX_RAM_SEC_ERR,  "sec error in rx buffer"),
    SEC_ERROR_CAUSE(PCIE2AXI_RAM_SEC_ERR,  "sec error in pcie2axi buffer"),
    SEC_ERROR_CAUSE(AXI2PCIE_RAM_SEC_ERR,  "sec error in axi2pcie buffer"),
    DED_ERROR_CAUSE(TX_RAM_DED_ERR,  "ded error in tx buffer"),
    DED_ERROR_CAUSE(RX_RAM_DED_ERR,  "ded error in rx buffer"),
    DED_ERROR_CAUSE(PCIE2AXI_RAM_DED_ERR,  "ded error in pcie2axi buffer"),
    DED_ERROR_CAUSE(AXI2PCIE_RAM_DED_ERR,  "ded error in axi2pcie buffer"),
    LOCAL_EVENT_CAUSE(DMA_ERROR_ENGINE_0, "dma engine 0 error"),
    LOCAL_EVENT_CAUSE(DMA_ERROR_ENGINE_1, "dma engine 1 error"),
    LOCAL_EVENT_CAUSE(A_ATR_EVT_POST_ERR, "axi write request error"),
    LOCAL_EVENT_CAUSE(A_ATR_EVT_FETCH_ERR, "axi read request error"),
    LOCAL_EVENT_CAUSE(A_ATR_EVT_DISCARD_ERR, "axi read timeout"),
    LOCAL_EVENT_CAUSE(P_ATR_EVT_POST_ERR, "pcie write request error"),
    LOCAL_EVENT_CAUSE(P_ATR_EVT_FETCH_ERR, "pcie read request error"),
    LOCAL_EVENT_CAUSE(P_ATR_EVT_DISCARD_ERR, "pcie read timeout"),
    LOCAL_EVENT_CAUSE(PM_MSI_INT_AER_EVT, "aer event"),
    LOCAL_EVENT_CAUSE(PM_MSI_INT_EVENTS, "pm/ltr/hotplug event"),
    LOCAL_EVENT_CAUSE(PM_MSI_INT_SYS_ERR, "system error"),
    };
    static struct event_map pcie_event_to_event[] = {
    PCIE_EVENT_TO_EVENT_MAP(L2_EXIT),
    PCIE_EVENT_TO_EVENT_MAP(HOTRST_EXIT),
    PCIE_EVENT_TO_EVENT_MAP(DLUP_EXIT),
    };
    static struct event_map sec_error_to_event[] = {
    SEC_ERROR_TO_EVENT_MAP(TX_RAM_SEC_ERR),
    SEC_ERROR_TO_EVENT_MAP(RX_RAM_SEC_ERR),
    SEC_ERROR_TO_EVENT_MAP(PCIE2AXI_RAM_SEC_ERR),
    SEC_ERROR_TO_EVENT_MAP(AXI2PCIE_RAM_SEC_ERR),
    };
    static struct event_map ded_error_to_event[] = {
    DED_ERROR_TO_EVENT_MAP(TX_RAM_DED_ERR),
    DED_ERROR_TO_EVENT_MAP(RX_RAM_DED_ERR),
    DED_ERROR_TO_EVENT_MAP(PCIE2AXI_RAM_DED_ERR),
    DED_ERROR_TO_EVENT_MAP(AXI2PCIE_RAM_DED_ERR),
    };
    static struct event_map local_status_to_event[] = {
    LOCAL_STATUS_TO_EVENT_MAP(DMA_END_ENGINE_0),
    LOCAL_STATUS_TO_EVENT_MAP(DMA_END_ENGINE_1),
    LOCAL_STATUS_TO_EVENT_MAP(DMA_ERROR_ENGINE_0),
    LOCAL_STATUS_TO_EVENT_MAP(DMA_ERROR_ENGINE_1),
    LOCAL_STATUS_TO_EVENT_MAP(A_ATR_EVT_POST_ERR),
    LOCAL_STATUS_TO_EVENT_MAP(A_ATR_EVT_FETCH_ERR),
    LOCAL_STATUS_TO_EVENT_MAP(A_ATR_EVT_DISCARD_ERR),
    LOCAL_STATUS_TO_EVENT_MAP(A_ATR_EVT_DOORBELL),
    LOCAL_STATUS_TO_EVENT_MAP(P_ATR_EVT_POST_ERR),
    LOCAL_STATUS_TO_EVENT_MAP(P_ATR_EVT_FETCH_ERR),
    LOCAL_STATUS_TO_EVENT_MAP(P_ATR_EVT_DISCARD_ERR),
    LOCAL_STATUS_TO_EVENT_MAP(P_ATR_EVT_DOORBELL),
    LOCAL_STATUS_TO_EVENT_MAP(PM_MSI_INT_INTX),
    LOCAL_STATUS_TO_EVENT_MAP(PM_MSI_INT_MSI),
    LOCAL_STATUS_TO_EVENT_MAP(PM_MSI_INT_AER_EVT),
    LOCAL_STATUS_TO_EVENT_MAP(PM_MSI_INT_EVENTS),
    LOCAL_STATUS_TO_EVENT_MAP(PM_MSI_INT_SYS_ERR),
    };
    static struct {
    u32 offset;
    u32 mask;
    u32 shift;
    u32 enb_mask;
    u32 mask_high;
    u32 mask_offset;
    } event_descs[] = {
    { PCIE_EVENT(L2_EXIT) },
    { PCIE_EVENT(HOTRST_EXIT) },
    { PCIE_EVENT(DLUP_EXIT) },
    { SEC_EVENT(TX_RAM_SEC_ERR) },
    { SEC_EVENT(RX_RAM_SEC_ERR) },
    { SEC_EVENT(PCIE2AXI_RAM_SEC_ERR) },
    { SEC_EVENT(AXI2PCIE_RAM_SEC_ERR) },
    { DED_EVENT(TX_RAM_DED_ERR) },
    { DED_EVENT(RX_RAM_DED_ERR) },
    { DED_EVENT(PCIE2AXI_RAM_DED_ERR) },
    { DED_EVENT(AXI2PCIE_RAM_DED_ERR) },
    { LOCAL_EVENT(DMA_END_ENGINE_0) },
    { LOCAL_EVENT(DMA_END_ENGINE_1) },
    { LOCAL_EVENT(DMA_ERROR_ENGINE_0) },
    { LOCAL_EVENT(DMA_ERROR_ENGINE_1) },
    { LOCAL_EVENT(A_ATR_EVT_POST_ERR) },
    { LOCAL_EVENT(A_ATR_EVT_FETCH_ERR) },
    { LOCAL_EVENT(A_ATR_EVT_DISCARD_ERR) },
    { LOCAL_EVENT(A_ATR_EVT_DOORBELL) },
    { LOCAL_EVENT(P_ATR_EVT_POST_ERR) },
    { LOCAL_EVENT(P_ATR_EVT_FETCH_ERR) },
    { LOCAL_EVENT(P_ATR_EVT_DISCARD_ERR) },
    { LOCAL_EVENT(P_ATR_EVT_DOORBELL) },
    { LOCAL_EVENT(PM_MSI_INT_INTX) },
    { LOCAL_EVENT(PM_MSI_INT_MSI) },
    { LOCAL_EVENT(PM_MSI_INT_AER_EVT) },
    { LOCAL_EVENT(PM_MSI_INT_EVENTS) },
    { LOCAL_EVENT(PM_MSI_INT_SYS_ERR) },
    };
    static char poss_clks[][5] = { "fic0", "fic1", "fic2", "fic3" };
    static struct mc_pcie *port;
#[no_mangle]
unsafe extern "C" fn mc_pcie_enable_msi(port: *mut mc_pcie, ecam: *mut void __iomem) {
    static void mc_pcie_enable_msi(struct mc_pcie *port, void __iomem *ecam)
    {
    struct plda_msi *msi = &port.plda.msi;
    u16 reg;
    u8 queue_size;
// Fixup MSI enable flag
    reg = readw_relaxed(ecam + MC_MSI_CAP_CTRL_OFFSET + PCI_MSI_FLAGS);
    reg |= PCI_MSI_FLAGS_ENABLE;
    writew_relaxed(reg, ecam + MC_MSI_CAP_CTRL_OFFSET + PCI_MSI_FLAGS);
// Fixup PCI MSI queue flags
    queue_size = FIELD_GET(PCI_MSI_FLAGS_QMASK, reg);
    reg |= FIELD_PREP(PCI_MSI_FLAGS_QSIZE, queue_size);
    writew_relaxed(reg, ecam + MC_MSI_CAP_CTRL_OFFSET + PCI_MSI_FLAGS);
// Fixup MSI addr fields
    writel_relaxed(lower_32_bits(msi.vector_phy),
    ecam + MC_MSI_CAP_CTRL_OFFSET + PCI_MSI_ADDRESS_LO);
    writel_relaxed(upper_32_bits(msi.vector_phy),
    ecam + MC_MSI_CAP_CTRL_OFFSET + PCI_MSI_ADDRESS_HI);
    }
#[no_mangle]
pub unsafe extern "C" fn reg_to_event(reg: u32, field: event_map) -> u32 {
    static inline u32 reg_to_event(u32 reg, struct event_map field)
    {
    return (reg & field.reg_mask) ? BIT(field.event_bit) : 0;
    }
#[no_mangle]
unsafe extern "C" fn pcie_events(port: *mut mc_pcie) -> u32 {
    static u32 pcie_events(struct mc_pcie *port)
    {
    let mut reg: u32 = readl_relaxed(port.ctrl_base_addr + PCIE_EVENT_INT);
    let mut val: u32 = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(pcie_event_to_event); i++)
    val |= reg_to_event(reg, pcie_event_to_event[i]);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn sec_errors(port: *mut mc_pcie) -> u32 {
    static u32 sec_errors(struct mc_pcie *port)
    {
    let mut reg: u32 = readl_relaxed(port.ctrl_base_addr + SEC_ERROR_INT);
    let mut val: u32 = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(sec_error_to_event); i++)
    val |= reg_to_event(reg, sec_error_to_event[i]);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn ded_errors(port: *mut mc_pcie) -> u32 {
    static u32 ded_errors(struct mc_pcie *port)
    {
    let mut reg: u32 = readl_relaxed(port.ctrl_base_addr + DED_ERROR_INT);
    let mut val: u32 = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(ded_error_to_event); i++)
    val |= reg_to_event(reg, ded_error_to_event[i]);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn local_events(port: *mut mc_pcie) -> u32 {
    static u32 local_events(struct mc_pcie *port)
    {
    let mut reg: u32 = readl_relaxed(port.bridge_base_addr + ISTATUS_LOCAL);
    let mut val: u32 = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(local_status_to_event); i++)
    val |= reg_to_event(reg, local_status_to_event[i]);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn mc_get_events(port: *mut plda_pcie_rp) -> u32 {
    static u32 mc_get_events(struct plda_pcie_rp *port)
    {
    struct mc_pcie *mc_port = container_of(port, struct mc_pcie, plda);
    let mut events: u32 = 0;
    events |= pcie_events(mc_port);
    events |= sec_errors(mc_port);
    events |= ded_errors(mc_port);
    events |= local_events(mc_port);
    return events;
    }
#[no_mangle]
unsafe extern "C" fn mc_event_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mc_event_handler(int irq, void *dev_id)
    {
    struct plda_pcie_rp *port = dev_id;
    struct device *dev = port.dev;
    struct irq_data *data;
    data = irq_domain_get_irq_data(port.event_domain, irq);
    if (event_cause[data.hwirq].str)
    dev_err_ratelimited(dev, "%s\n", event_cause[data.hwirq].str);
    else
    dev_err_ratelimited(dev, "bad event IRQ %ld\n", data.hwirq);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mc_ack_event_irq(data: *mut irq_data) {
    static void mc_ack_event_irq(struct irq_data *data)
    {
    struct plda_pcie_rp *port = irq_data_get_irq_chip_data(data);
    struct mc_pcie *mc_port = container_of(port, struct mc_pcie, plda);
    let mut event: u32 = data.hwirq;
    void __iomem *addr;
    u32 mask;
    if (event_descs[event].offset == ISTATUS_LOCAL)
    addr = mc_port.bridge_base_addr;
    else
    addr = mc_port.ctrl_base_addr;
    addr += event_descs[event].offset;
    mask = event_descs[event].mask;
    mask |= event_descs[event].enb_mask;
    writel_relaxed(mask, addr);
    }
#[no_mangle]
unsafe extern "C" fn mc_mask_event_irq(data: *mut irq_data) {
    static void mc_mask_event_irq(struct irq_data *data)
    {
    struct plda_pcie_rp *port = irq_data_get_irq_chip_data(data);
    struct mc_pcie *mc_port = container_of(port, struct mc_pcie, plda);
    let mut event: u32 = data.hwirq;
    void __iomem *addr;
    u32 mask;
    u32 val;
    if (event_descs[event].offset == ISTATUS_LOCAL)
    addr = mc_port.bridge_base_addr;
    else
    addr = mc_port.ctrl_base_addr;
    addr += event_descs[event].mask_offset;
    mask = event_descs[event].mask;
    if (event_descs[event].enb_mask) {
    mask <<= PCIE_EVENT_INT_ENB_SHIFT;
    mask &= PCIE_EVENT_INT_ENB_MASK;
    }
    if (!event_descs[event].mask_high)
    mask = ~mask;
    raw_spin_lock(&port.lock);
    val = readl_relaxed(addr);
    if (event_descs[event].mask_high)
    val |= mask;
    else
    val &= mask;
    writel_relaxed(val, addr);
    raw_spin_unlock(&port.lock);
    }
#[no_mangle]
unsafe extern "C" fn mc_unmask_event_irq(data: *mut irq_data) {
    static void mc_unmask_event_irq(struct irq_data *data)
    {
    struct plda_pcie_rp *port = irq_data_get_irq_chip_data(data);
    struct mc_pcie *mc_port = container_of(port, struct mc_pcie, plda);
    let mut event: u32 = data.hwirq;
    void __iomem *addr;
    u32 mask;
    u32 val;
    if (event_descs[event].offset == ISTATUS_LOCAL)
    addr = mc_port.bridge_base_addr;
    else
    addr = mc_port.ctrl_base_addr;
    addr += event_descs[event].mask_offset;
    mask = event_descs[event].mask;
    if (event_descs[event].enb_mask)
    mask <<= PCIE_EVENT_INT_ENB_SHIFT;
    if (event_descs[event].mask_high)
    mask = ~mask;
    if (event_descs[event].enb_mask)
    mask &= PCIE_EVENT_INT_ENB_MASK;
    raw_spin_lock(&port.lock);
    val = readl_relaxed(addr);
    if (event_descs[event].mask_high)
    val &= mask;
    else
    val |= mask;
    writel_relaxed(val, addr);
    raw_spin_unlock(&port.lock);
    }
    static struct irq_chip mc_event_irq_chip = {
    .name = "Microchip PCIe EVENT",
    .irq_ack = mc_ack_event_irq,
    .irq_mask = mc_mask_event_irq,
    .irq_unmask = mc_unmask_event_irq,
    };
#[no_mangle]
pub unsafe extern "C" fn mc_pcie_deinit_clk(data: *mut c_void) {
    static inline void mc_pcie_deinit_clk(void *data)
    {
    struct clk *clk = data;
    clk_disable_unprepare(clk);
    }
    static inline struct clk *mc_pcie_init_clk(struct device *dev, const char *id)
    {
    struct clk *clk;
    int ret;
    clk = devm_clk_get_optional(dev, id);
    if (IS_ERR(clk))
    return clk;
    if (!clk)
    return clk;
    ret = clk_prepare_enable(clk);
    if (ret)
    return ERR_PTR(ret);
    devm_add_action_or_reset(dev, mc_pcie_deinit_clk, clk);
    return clk;
    }
#[no_mangle]
unsafe extern "C" fn mc_pcie_init_clks(dev: *mut device) -> c_int {
    static int mc_pcie_init_clks(struct device *dev)
    {
    int i;
    struct clk *fic;
//
// PCIe may be clocked via Fabric Interface using between 1 and 4
// clocks. Scan DT for clocks and enable them if present
//
    for (i = 0; i < ARRAY_SIZE(poss_clks); i++) {
    fic = mc_pcie_init_clk(dev, poss_clks[i]);
    if (IS_ERR(fic))
    return PTR_ERR(fic);
    }
    return 0;
    }
    static int mc_request_event_irq(struct plda_pcie_rp *plda, int event_irq,
    int event)
    {
    return devm_request_irq(plda.dev, event_irq, mc_event_handler,
    0, event_cause[event].sym, plda);
    }
    static const struct plda_event_ops mc_event_ops = {
    .get_events = mc_get_events,
    };
    static const struct plda_event mc_event = {
    .request_event_irq = mc_request_event_irq,
    .intx_event        = EVENT_LOCAL_PM_MSI_INT_INTX,
    .msi_event         = EVENT_LOCAL_PM_MSI_INT_MSI,
    };
#[no_mangle]
pub unsafe extern "C" fn mc_clear_secs(port: *mut mc_pcie) {
    static inline void mc_clear_secs(struct mc_pcie *port)
    {
    writel_relaxed(SEC_ERROR_INT_ALL_RAM_SEC_ERR_INT,
    port.ctrl_base_addr + SEC_ERROR_INT);
    writel_relaxed(0, port.ctrl_base_addr + SEC_ERROR_EVENT_CNT);
    }
#[no_mangle]
pub unsafe extern "C" fn mc_clear_deds(port: *mut mc_pcie) {
    static inline void mc_clear_deds(struct mc_pcie *port)
    {
    writel_relaxed(DED_ERROR_INT_ALL_RAM_DED_ERR_INT,
    port.ctrl_base_addr + DED_ERROR_INT);
    writel_relaxed(0, port.ctrl_base_addr + DED_ERROR_EVENT_CNT);
    }
#[no_mangle]
unsafe extern "C" fn mc_disable_interrupts(port: *mut mc_pcie) {
    static void mc_disable_interrupts(struct mc_pcie *port)
    {
    u32 val;
// Ensure ECC bypass is enabled
    val = ECC_CONTROL_TX_RAM_ECC_BYPASS |
    ECC_CONTROL_RX_RAM_ECC_BYPASS |
    ECC_CONTROL_PCIE2AXI_RAM_ECC_BYPASS |
    ECC_CONTROL_AXI2PCIE_RAM_ECC_BYPASS;
    writel_relaxed(val, port.ctrl_base_addr + ECC_CONTROL);
// Disable SEC errors and clear any outstanding
    writel_relaxed(SEC_ERROR_INT_ALL_RAM_SEC_ERR_INT,
    port.ctrl_base_addr + SEC_ERROR_INT_MASK);
    mc_clear_secs(port);
// Disable DED errors and clear any outstanding
    writel_relaxed(DED_ERROR_INT_ALL_RAM_DED_ERR_INT,
    port.ctrl_base_addr + DED_ERROR_INT_MASK);
    mc_clear_deds(port);
// Disable local interrupts and clear any outstanding
    writel_relaxed(0, port.bridge_base_addr + IMASK_LOCAL);
    writel_relaxed(GENMASK(31, 0), port.bridge_base_addr + ISTATUS_LOCAL);
    writel_relaxed(GENMASK(31, 0), port.bridge_base_addr + ISTATUS_MSI);
// Disable PCIe events and clear any outstanding
    val = PCIE_EVENT_INT_L2_EXIT_INT |
    PCIE_EVENT_INT_HOTRST_EXIT_INT |
    PCIE_EVENT_INT_DLUP_EXIT_INT |
    PCIE_EVENT_INT_L2_EXIT_INT_MASK |
    PCIE_EVENT_INT_HOTRST_EXIT_INT_MASK |
    PCIE_EVENT_INT_DLUP_EXIT_INT_MASK;
    writel_relaxed(val, port.ctrl_base_addr + PCIE_EVENT_INT);
// Disable host interrupts and clear any outstanding
    writel_relaxed(0, port.bridge_base_addr + IMASK_HOST);
    writel_relaxed(GENMASK(31, 0), port.bridge_base_addr + ISTATUS_HOST);
    }
    static void mc_pcie_setup_inbound_atr(struct mc_pcie *port, int window_index,
    u64 axi_addr, u64 pcie_addr, u64 size)
    {
    let mut table_offset: u32 = window_index * ATR_ENTRY_SIZE;
    void __iomem *table_addr = port.bridge_base_addr + table_offset;
    u32 atr_sz;
    u32 val;
    atr_sz = ilog2(size) - 1;
    val = ALIGN_DOWN(lower_32_bits(pcie_addr), SZ_4K);
    val |= FIELD_PREP(ATR_SIZE_MASK, atr_sz);
    val |= ATR_IMPL_ENABLE;
    writel(val, table_addr + ATR0_PCIE_WIN0_SRCADDR_PARAM);
    writel(upper_32_bits(pcie_addr), table_addr + ATR0_PCIE_WIN0_SRC_ADDR);
    writel(lower_32_bits(axi_addr), table_addr + ATR0_PCIE_WIN0_TRSL_ADDR_LSB);
    writel(upper_32_bits(axi_addr), table_addr + ATR0_PCIE_WIN0_TRSL_ADDR_UDW);
    writel(TRSL_ID_AXI4_MASTER_0, table_addr + ATR0_PCIE_WIN0_TRSL_PARAM);
    }
    static int mc_pcie_setup_inbound_ranges(struct platform_device *pdev,
    struct mc_pcie *port)
    {
    struct device *dev = &pdev.dev;
    struct device_node *dn = dev.of_node;
    struct of_range_parser parser;
    struct of_range range;
    let mut atr_index: c_int = 0;
//
// MPFS PCIe Root Port is 32-bit only, behind a Fabric Interface
// Controller FPGA logic block which contains the AXI-S interface.
//
// From the point of view of the PCIe Root Port, there are only two
// supported Root Port configurations:
//
// Configuration 1: for use with fully coherent designs; supports a
// window from 0x0 (CPU space) to specified PCIe space.
//
// Configuration 2: for use with non-coherent designs; supports two
// 1 GB windows to CPU space; one mapping CPU space 0 to PCIe space
// 0x80000000 and a second mapping CPU space 0x40000000 to PCIe
// space 0xc0000000. This cfg needs two windows because of how the
// MSI space is allocated in the AXI-S range on MPFS.
//
// The FIC interface outside the PCIe block *must* complete the
// inbound address translation as per MCHP MPFS FPGA design
// guidelines.
//
    if (device_property_read_bool(dev, "dma-noncoherent")) {
//
// Always need same two tables in this case.  Need two tables
// due to hardware interactions between address and size.
//
    mc_pcie_setup_inbound_atr(port, 0, 0,
    MPFS_NC_BOUNCE_ADDR, SZ_1G);
    mc_pcie_setup_inbound_atr(port, 1, SZ_1G,
    MPFS_NC_BOUNCE_ADDR + SZ_1G, SZ_1G);
    } else {
// Find any DMA ranges
    if (of_pci_dma_range_parser_init(&parser, dn)) {
// No DMA range property - setup default
    mc_pcie_setup_inbound_atr(port, 0, 0, 0, SZ_4G);
    return 0;
    }
    for_each_of_range(&parser, &range) {
    if (atr_index >= MC_MAX_NUM_INBOUND_WINDOWS) {
    dev_err(dev, "too many inbound ranges; %d available tables\n",
    MC_MAX_NUM_INBOUND_WINDOWS);
    return -EINVAL;
    }
    mc_pcie_setup_inbound_atr(port, atr_index, 0,
    range.pci_addr, range.size);
    atr_index++;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mc_platform_init(cfg: *mut pci_config_window) -> c_int {
    static int mc_platform_init(struct pci_config_window *cfg)
    {
    struct device *dev = cfg.parent;
    struct platform_device *pdev = to_platform_device(dev);
    struct pci_host_bridge *bridge = platform_get_drvdata(pdev);
    int ret;
// Configure address translation table 0 for PCIe config space
    plda_pcie_setup_window(port.bridge_base_addr, 0, cfg.res.start,
    cfg.res.start,
    resource_size(&cfg.res));
// Need some fixups in config space
    mc_pcie_enable_msi(port, cfg.win);
// Configure non-config space outbound ranges
    ret = plda_pcie_setup_iomems(bridge, &port.plda);
    if (ret)
    return ret;
    ret = mc_pcie_setup_inbound_ranges(pdev, port);
    if (ret)
    return ret;
    port.plda.event_ops = &mc_event_ops;
    port.plda.event_irq_chip = &mc_event_irq_chip;
    port.plda.events_bitmap = GENMASK(NUM_EVENTS - 1, 0);
// Address translation is up; safe to enable interrupts
    ret = plda_init_interrupts(pdev, &port.plda, &mc_event);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mc_host_probe(pdev: *mut platform_device) -> c_int {
    static int mc_host_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    void __iomem *apb_base_addr;
    struct plda_pcie_rp *plda;
    int ret;
    u32 val;
    port = devm_kzalloc(dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    plda = &port.plda;
    plda.dev = dev;
    port.bridge_base_addr = devm_platform_ioremap_resource_byname(pdev,
    "bridge");
    port.ctrl_base_addr = devm_platform_ioremap_resource_byname(pdev,
    "ctrl");
    if (!IS_ERR(port.bridge_base_addr) && !IS_ERR(port.ctrl_base_addr))
    goto addrs_set;
//
// The original, incorrect, binding that lumped the control and
// bridge addresses together still needs to be handled by the driver.
//
    apb_base_addr = devm_platform_ioremap_resource_byname(pdev, "apb");
    if (IS_ERR(apb_base_addr))
    return dev_err_probe(dev, PTR_ERR(apb_base_addr),
    "both legacy apb register and ctrl/bridge regions missing");
    port.bridge_base_addr = apb_base_addr + MC_PCIE1_BRIDGE_ADDR;
    port.ctrl_base_addr = apb_base_addr + MC_PCIE1_CTRL_ADDR;
    addrs_set:
    mc_disable_interrupts(port);
    plda.bridge_addr = port.bridge_base_addr;
    plda.num_events = NUM_EVENTS;
// Allow enabling MSI by disabling MSI-X
    val = readl(port.bridge_base_addr + PCIE_PCI_IRQ_DW0);
    val &= ~MSIX_CAP_MASK;
    writel(val, port.bridge_base_addr + PCIE_PCI_IRQ_DW0);
// Pick num vectors from bitfile programmed onto FPGA fabric
    val = readl(port.bridge_base_addr + PCIE_PCI_IRQ_DW0);
    val &= NUM_MSI_MSGS_MASK;
    val >>= NUM_MSI_MSGS_SHIFT;
    plda.msi.num_vectors = 1 << val;
// Pick vector address from design
    plda.msi.vector_phy = readl_relaxed(port.bridge_base_addr + IMSI_ADDR);
    ret = mc_pcie_init_clks(dev);
    if (ret) {
    dev_err(dev, "failed to get clock resources, error %d\n", ret);
    return -ENODEV;
    }
    return pci_host_common_probe(pdev);
    }
    static const struct pci_ecam_ops mc_ecam_ops = {
    .init = mc_platform_init,
    .pci_ops = {
    .map_bus = pci_ecam_map_bus,
    .read = pci_generic_config_read,
    .write = pci_generic_config_write,
    }
    };
    static const struct of_device_id mc_pcie_of_match[] = {
    {
    .compatible = "microchip,pcie-host-1.0",
    .data = &mc_ecam_ops,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, mc_pcie_of_match);
    static struct platform_driver mc_pcie_driver = {
    .probe = mc_host_probe,
    .driver = {
    .name = "microchip-pcie",
    .of_match_table = mc_pcie_of_match,
    .suppress_bind_attrs = true,
    },
    };
    builtin_platform_driver(mc_pcie_driver);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Microchip PCIe host controller driver");
    MODULE_AUTHOR("Daire McNamara <daire.mcnamara@microchip.com>");
