//! Automatically rewritten from C to Rust
//! Source: drivers/ata/sata_highbank.c
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
// Calxeda Highbank AHCI SATA platform driver
// Copyright 2012 Calxeda, Inc.
//
// based on the AHCI SATA platform driver by Jeff Garzik and Anton Vorontsov
//

pub const SERDES_CR_CTL: c_uint = 0x80a0;
pub const SERDES_CR_ADDR: c_uint = 0x80a1;
pub const SERDES_CR_DATA: c_uint = 0x80a2;
pub const CR_BUSY: c_uint = 0x0001;
pub const CR_START: c_uint = 0x0001;
pub const CR_WR_RDN: c_uint = 0x0002;
pub const CPHY_TX_INPUT_STS: c_uint = 0x2001;
pub const CPHY_RX_INPUT_STS: c_uint = 0x2002;
pub const CPHY_SATA_TX_OVERRIDE: c_uint = 0x8000;
pub const CPHY_SATA_RX_OVERRIDE: c_uint = 0x4000;
pub const CPHY_TX_OVERRIDE: c_uint = 0x2004;
pub const CPHY_RX_OVERRIDE: c_uint = 0x2005;
pub const SPHY_LANE: c_uint = 0x100;
pub const SPHY_HALF_RATE: c_uint = 0x0001;
pub const CPHY_SATA_DPLL_MODE: c_uint = 0x0700;
pub const CPHY_SATA_DPLL_SHIFT: c_int = 8;

pub const CPHY_SATA_TX_ATTEN: c_uint = 0x1c00;
pub const CPHY_SATA_TX_ATTEN_SHIFT: c_int = 10;
pub const CPHY_PHY_COUNT: c_int = 6;
pub const CPHY_LANE_COUNT: c_int = 4;

    static DEFINE_SPINLOCK(cphy_lock);
// Each of the 6 phys can have up to 4 sata ports attached to i. Map 0-based
// sata ports to their phys and then to their lanes within the phys
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_lane_info {
    pub phy_base: *mut void __iomem,
    pub lane_mapping: u8,
    pub phy_devs: u8,
    pub tx_atten: u8,
}

    static struct phy_lane_info port_data[CPHY_PORT_COUNT];
    static DEFINE_SPINLOCK(sgpio_lock);
pub const SCLOCK: c_int = 0;
pub const SLOAD: c_int = 1;
pub const SDATA: c_int = 2;
pub const SGPIO_PINS: c_int = 3;
pub const SGPIO_PORTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ecx_plat_data {
    pub n_ports: u32,
// number of extra clocks that the SGPIO PIC controller expects
    pub pre_clocks: u32,
    pub post_clocks: u32,
    pub sgpio_gpiod: [*mut gpio_desc; SGPIO_PINS],
    pub sgpio_pattern: u32,
    pub port_to_sgpio: [u32; SGPIO_PORTS],
}

pub const SGPIO_SIGNALS: c_int = 3;
pub const ECX_ACTIVITY_BITS: c_uint = 0x300000;
pub const ECX_ACTIVITY_SHIFT: c_int = 0;
pub const ECX_LOCATE_BITS: c_uint = 0x80000;
pub const ECX_LOCATE_SHIFT: c_int = 1;
pub const ECX_FAULT_BITS: c_uint = 0x400000;
pub const ECX_FAULT_SHIFT: c_int = 2;
    static inline int sgpio_bit_shift(struct ecx_plat_data *pdata, u32 port,
    u32 shift)
    {
    return 1 << (3 * pdata.port_to_sgpio[port] + shift);
    }
#[no_mangle]
unsafe extern "C" fn ecx_parse_sgpio(pdata: *mut ecx_plat_data, port: u32, state: u32) {
    static void ecx_parse_sgpio(struct ecx_plat_data *pdata, u32 port, u32 state)
    {
    if (state & ECX_ACTIVITY_BITS)
    pdata.sgpio_pattern |= sgpio_bit_shift(pdata, port,
    ECX_ACTIVITY_SHIFT);
    else
    pdata.sgpio_pattern &= ~sgpio_bit_shift(pdata, port,
    ECX_ACTIVITY_SHIFT);
    if (state & ECX_LOCATE_BITS)
    pdata.sgpio_pattern |= sgpio_bit_shift(pdata, port,
    ECX_LOCATE_SHIFT);
    else
    pdata.sgpio_pattern &= ~sgpio_bit_shift(pdata, port,
    ECX_LOCATE_SHIFT);
    if (state & ECX_FAULT_BITS)
    pdata.sgpio_pattern |= sgpio_bit_shift(pdata, port,
    ECX_FAULT_SHIFT);
    else
    pdata.sgpio_pattern &= ~sgpio_bit_shift(pdata, port,
    ECX_FAULT_SHIFT);
    }
//
// Tell the LED controller that the signal has changed by raising the clock
// line for 50 uS and then lowering it for 50 uS.
//
#[no_mangle]
unsafe extern "C" fn ecx_led_cycle_clock(pdata: *mut ecx_plat_data) {
    static void ecx_led_cycle_clock(struct ecx_plat_data *pdata)
    {
    gpiod_set_value(pdata.sgpio_gpiod[SCLOCK], 1);
    udelay(50);
    gpiod_set_value(pdata.sgpio_gpiod[SCLOCK], 0);
    udelay(50);
    }
    static ssize_t ecx_transmit_led_message(struct ata_port *ap, u32 state,
    ssize_t size)
    {
    struct ahci_host_priv *hpriv =  ap.host.private_data;
    struct ecx_plat_data *pdata = hpriv.plat_data;
    struct ahci_port_priv *pp = ap.private_data;
    unsigned long flags;
    int pmp, i;
    struct ahci_em_priv *emp;
    u32 sgpio_out;
// get the slot number from the message
    pmp = (state & EM_MSG_LED_PMP_SLOT) >> 8;
    if (pmp < EM_MAX_SLOTS)
    emp = &pp.em_priv[pmp];
    else
    return -EINVAL;
    if (!(hpriv.em_msg_type & EM_MSG_TYPE_LED))
    return size;
    spin_lock_irqsave(&sgpio_lock, flags);
    ecx_parse_sgpio(pdata, ap.port_no, state);
    sgpio_out = pdata.sgpio_pattern;
    for (i = 0; i < pdata.pre_clocks; i++)
    ecx_led_cycle_clock(pdata);
    gpiod_set_value(pdata.sgpio_gpiod[SLOAD], 1);
    ecx_led_cycle_clock(pdata);
    gpiod_set_value(pdata.sgpio_gpiod[SLOAD], 0);
//
// bit-bang out the SGPIO pattern, by consuming a bit and then
// clocking it out.
//
    for (i = 0; i < (SGPIO_SIGNALS * pdata.n_ports); i++) {
    gpiod_set_value(pdata.sgpio_gpiod[SDATA], sgpio_out & 1);
    sgpio_out >>= 1;
    ecx_led_cycle_clock(pdata);
    }
    for (i = 0; i < pdata.post_clocks; i++)
    ecx_led_cycle_clock(pdata);
// save off new led state for port/slot
    emp.led_state = state;
    spin_unlock_irqrestore(&sgpio_lock, flags);
    return size;
    }
    static void highbank_set_em_messages(struct device *dev,
    struct ahci_host_priv *hpriv,
    struct ata_port_info *pi)
    {
    struct device_node *np = dev.of_node;
    struct ecx_plat_data *pdata = hpriv.plat_data;
    int i;
    for (i = 0; i < SGPIO_PINS; i++) {
    struct gpio_desc *gpiod;
    gpiod = devm_gpiod_get_index(dev, "calxeda,sgpio", i,
    GPIOD_OUT_HIGH);
    if (IS_ERR(gpiod)) {
    dev_err(dev, "failed to get GPIO %d\n", i);
    continue;
    }
    gpiod_set_consumer_name(gpiod, "CX SGPIO");
    pdata.sgpio_gpiod[i] = gpiod;
    }
    of_property_read_u32_array(np, "calxeda,led-order",
    pdata.port_to_sgpio,
    pdata.n_ports);
    if (of_property_read_u32(np, "calxeda,pre-clocks", &pdata.pre_clocks))
    pdata.pre_clocks = 0;
    if (of_property_read_u32(np, "calxeda,post-clocks",
    &pdata.post_clocks))
    pdata.post_clocks = 0;
// store em_loc
    hpriv.em_loc = 0;
    hpriv.em_buf_sz = 4;
    hpriv.em_msg_type = EM_MSG_TYPE_LED;
    pi.flags |= ATA_FLAG_EM | ATA_FLAG_SW_ACTIVITY;
    }
#[no_mangle]
unsafe extern "C" fn __combo_phy_reg_read(sata_port: u8, addr: u32) -> u32 {
    static u32 __combo_phy_reg_read(u8 sata_port, u32 addr)
    {
    u32 data;
    let mut dev: u8 = port_data[sata_port].phy_devs;
    spin_lock(&cphy_lock);
    writel(CPHY_MAP(dev, addr), port_data[sata_port].phy_base + 0x800);
    data = readl(port_data[sata_port].phy_base + CPHY_ADDR(addr));
    spin_unlock(&cphy_lock);
    return data;
    }
#[no_mangle]
unsafe extern "C" fn __combo_phy_reg_write(sata_port: u8, addr: u32, data: u32) {
    static void __combo_phy_reg_write(u8 sata_port, u32 addr, u32 data)
    {
    let mut dev: u8 = port_data[sata_port].phy_devs;
    spin_lock(&cphy_lock);
    writel(CPHY_MAP(dev, addr), port_data[sata_port].phy_base + 0x800);
    writel(data, port_data[sata_port].phy_base + CPHY_ADDR(addr));
    spin_unlock(&cphy_lock);
    }
#[no_mangle]
unsafe extern "C" fn combo_phy_wait_for_ready(sata_port: u8) {
    static void combo_phy_wait_for_ready(u8 sata_port)
    {
    while (__combo_phy_reg_read(sata_port, SERDES_CR_CTL) & CR_BUSY)
    udelay(5);
    }
#[no_mangle]
unsafe extern "C" fn combo_phy_read(sata_port: u8, addr: u32) -> u32 {
    static u32 combo_phy_read(u8 sata_port, u32 addr)
    {
    combo_phy_wait_for_ready(sata_port);
    __combo_phy_reg_write(sata_port, SERDES_CR_ADDR, addr);
    __combo_phy_reg_write(sata_port, SERDES_CR_CTL, CR_START);
    combo_phy_wait_for_ready(sata_port);
    return __combo_phy_reg_read(sata_port, SERDES_CR_DATA);
    }
#[no_mangle]
unsafe extern "C" fn combo_phy_write(sata_port: u8, addr: u32, data: u32) {
    static void combo_phy_write(u8 sata_port, u32 addr, u32 data)
    {
    combo_phy_wait_for_ready(sata_port);
    __combo_phy_reg_write(sata_port, SERDES_CR_ADDR, addr);
    __combo_phy_reg_write(sata_port, SERDES_CR_DATA, data);
    __combo_phy_reg_write(sata_port, SERDES_CR_CTL, CR_WR_RDN | CR_START);
    }
#[no_mangle]
unsafe extern "C" fn highbank_cphy_disable_overrides(sata_port: u8) {
    static void highbank_cphy_disable_overrides(u8 sata_port)
    {
    let mut lane: u8 = port_data[sata_port].lane_mapping;
    u32 tmp;
    if (unlikely(port_data[sata_port].phy_base == core::ptr::null_mut()))
    return;
    tmp = combo_phy_read(sata_port, CPHY_RX_INPUT_STS + lane * SPHY_LANE);
    tmp &= ~CPHY_SATA_RX_OVERRIDE;
    combo_phy_write(sata_port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp);
    }
#[no_mangle]
unsafe extern "C" fn cphy_override_tx_attenuation(sata_port: u8, val: u32) {
    static void cphy_override_tx_attenuation(u8 sata_port, u32 val)
    {
    let mut lane: u8 = port_data[sata_port].lane_mapping;
    u32 tmp;
    if (val & 0x8)
    return;
    tmp = combo_phy_read(sata_port, CPHY_TX_INPUT_STS + lane * SPHY_LANE);
    tmp &= ~CPHY_SATA_TX_OVERRIDE;
    combo_phy_write(sata_port, CPHY_TX_OVERRIDE + lane * SPHY_LANE, tmp);
    tmp |= CPHY_SATA_TX_OVERRIDE;
    combo_phy_write(sata_port, CPHY_TX_OVERRIDE + lane * SPHY_LANE, tmp);
    tmp |= (val << CPHY_SATA_TX_ATTEN_SHIFT) & CPHY_SATA_TX_ATTEN;
    combo_phy_write(sata_port, CPHY_TX_OVERRIDE + lane * SPHY_LANE, tmp);
    }
#[no_mangle]
unsafe extern "C" fn cphy_override_rx_mode(sata_port: u8, val: u32) {
    static void cphy_override_rx_mode(u8 sata_port, u32 val)
    {
    let mut lane: u8 = port_data[sata_port].lane_mapping;
    u32 tmp;
    tmp = combo_phy_read(sata_port, CPHY_RX_INPUT_STS + lane * SPHY_LANE);
    tmp &= ~CPHY_SATA_RX_OVERRIDE;
    combo_phy_write(sata_port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp);
    tmp |= CPHY_SATA_RX_OVERRIDE;
    combo_phy_write(sata_port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp);
    tmp &= ~CPHY_SATA_DPLL_MODE;
    tmp |= val << CPHY_SATA_DPLL_SHIFT;
    combo_phy_write(sata_port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp);
    tmp |= CPHY_SATA_DPLL_RESET;
    combo_phy_write(sata_port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp);
    tmp &= ~CPHY_SATA_DPLL_RESET;
    combo_phy_write(sata_port, CPHY_RX_OVERRIDE + lane * SPHY_LANE, tmp);
    msleep(15);
    }
#[no_mangle]
unsafe extern "C" fn highbank_cphy_override_lane(sata_port: u8) {
    static void highbank_cphy_override_lane(u8 sata_port)
    {
    let mut lane: u8 = port_data[sata_port].lane_mapping;
    u32 tmp, k = 0;
    if (unlikely(port_data[sata_port].phy_base == core::ptr::null_mut()))
    return;
    do {
    tmp = combo_phy_read(sata_port, CPHY_RX_INPUT_STS +
    lane * SPHY_LANE);
    } while ((tmp & SPHY_HALF_RATE) && (k++ < 1000));
    cphy_override_rx_mode(sata_port, 3);
    cphy_override_tx_attenuation(sata_port, port_data[sata_port].tx_atten);
    }
#[no_mangle]
unsafe extern "C" fn highbank_initialize_phys(dev: *mut device, addr: *mut void __iomem) -> c_int {
    static int highbank_initialize_phys(struct device *dev, void __iomem *addr)
    {
    struct device_node *sata_node = dev.of_node;
    let mut phy_count: c_int = 0, phy, port = 0, i;
    void __iomem *cphy_base[CPHY_PHY_COUNT] = {};
    struct device_node *phy_nodes[CPHY_PHY_COUNT] = {};
    u32 tx_atten[CPHY_PORT_COUNT] = {};
    memset(port_data, 0, sizeof(struct phy_lane_info) * CPHY_PORT_COUNT);
    do {
    u32 tmp;
    struct of_phandle_args phy_data;
    if (of_parse_phandle_with_args(sata_node,
    "calxeda,port-phys", "#phy-cells",
    port, &phy_data))
    break;
    for (phy = 0; phy < phy_count; phy++) {
    if (phy_nodes[phy] == phy_data.np)
    break;
    }
    if (phy_nodes[phy] == core::ptr::null_mut()) {
    phy_nodes[phy] = phy_data.np;
    cphy_base[phy] = of_iomap(phy_nodes[phy], 0);
    if (cphy_base[phy] == core::ptr::null_mut()) {
    of_node_put(phy_data.np);
    return 0;
    }
    phy_count += 1;
    }
    port_data[port].lane_mapping = phy_data.args[0];
    of_property_read_u32(phy_nodes[phy], "phydev", &tmp);
    port_data[port].phy_devs = tmp;
    port_data[port].phy_base = cphy_base[phy];
    of_node_put(phy_data.np);
    port += 1;
    } while (port < CPHY_PORT_COUNT);
    of_property_read_u32_array(sata_node, "calxeda,tx-atten",
    tx_atten, port);
    for (i = 0; i < port; i++)
    port_data[i].tx_atten = (u8) tx_atten[i];
    return 0;
    }
//
// The Calxeda SATA phy intermittently fails to bring up a link with Gen3
// Retrying the phy hard reset can work around the issue, but the drive
// may fail again. In less than 150 out of 15000 test runs, it took more
// than 10 tries for the link to be established (but never more than 35).
// Triple the maximum observed retry count to provide plenty of margin for
// rare events and to guarantee that the link is established.
//
// Also, the default 2 second time-out on a failed drive is too long in
// this situation. The uboot implementation of the same driver function
// uses a much shorter time-out period and never experiences a time out
// issue. Reducing the time-out to 500ms improves the responsiveness.
// The other timing constants were kept the same as the stock AHCI driver.
// This change was also tested 15000 times on 24 drives and none of them
// experienced a time out.
//
    static int ahci_highbank_hardreset(struct ata_link *link, unsigned int *class,
    unsigned long deadline)
    {
    static const unsigned int timing[] = { 5, 100, 500};
    struct ata_port *ap = link.ap;
    struct ahci_port_priv *pp = ap.private_data;
    struct ahci_host_priv *hpriv = ap.host.private_data;
    u8 *d2h_fis = pp.rx_fis + RX_FIS_D2H_REG;
    struct ata_taskfile tf;
    bool online;
    u32 sstatus;
    int rc;
    let mut retry: c_int = 100;
    hpriv.stop_engine(ap);
// clear D2H reception area to properly wait for D2H FIS
    ata_tf_init(link.device, &tf);
    tf.status = ATA_BUSY;
    ata_tf_to_fis(&tf, 0, 0, d2h_fis);
    do {
    highbank_cphy_disable_overrides(link.ap.port_no);
    rc = sata_link_hardreset(link, timing, deadline, &online, core::ptr::null_mut());
    highbank_cphy_override_lane(link.ap.port_no);
// If the status is 1, we are connected, but the link did not
// come up. So retry resetting the link again.
//
    if (sata_scr_read(link, SCR_STATUS, &sstatus))
    break;
    if (!(sstatus & 0x3))
    break;
    } while (!online && retry--);
    hpriv.start_engine(ap);
    if (online)
// class = ahci_dev_classify(ap);
    return rc;
    }
    static struct ata_port_operations ahci_highbank_ops = {
    .inherits		= &ahci_ops,
    .reset.hardreset	= ahci_highbank_hardreset,
    .transmit_led_message   = ecx_transmit_led_message,
    };
    static const struct ata_port_info ahci_highbank_port_info = {
    .flags          = AHCI_FLAG_COMMON,
    .pio_mask       = ATA_PIO4,
    .udma_mask      = ATA_UDMA6,
    .port_ops       = &ahci_highbank_ops,
    };
    static const struct scsi_host_template ahci_highbank_platform_sht = {
    AHCI_SHT("sata_highbank"),
    };
    static const struct of_device_id ahci_of_match[] = {
    { .compatible = "calxeda,hb-ahci" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ahci_of_match);
#[no_mangle]
unsafe extern "C" fn ahci_highbank_probe(pdev: *mut platform_device) -> c_int {
    static int ahci_highbank_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ahci_host_priv *hpriv;
    struct ecx_plat_data *pdata;
    struct ata_host *host;
    void __iomem *mmio;
    int irq;
    int i;
    int rc;
    u32 n_ports;
    let mut pi: ata_port_info = ahci_highbank_port_info;
    const struct ata_port_info *ppi[] = { &pi, core::ptr::null_mut() };
    mmio = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mmio))
    return PTR_ERR(mmio);
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    if (!irq)
    return -EINVAL;
    hpriv = devm_kzalloc(dev, sizeof(*hpriv), GFP_KERNEL);
    if (!hpriv) {
    dev_err(dev, "can't alloc ahci_host_priv\n");
    return -ENOMEM;
    }
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata) {
    dev_err(dev, "can't alloc ecx_plat_data\n");
    return -ENOMEM;
    }
    hpriv.irq = irq;
    hpriv.flags |= (unsigned long)pi.private_data;
    hpriv.mmio = mmio;
    rc = highbank_initialize_phys(dev, hpriv.mmio);
    if (rc)
    return rc;
    ahci_save_initial_config(dev, hpriv);
// prepare host
    if (hpriv.cap & HOST_CAP_NCQ)
    pi.flags |= ATA_FLAG_NCQ;
    if (hpriv.cap & HOST_CAP_PMP)
    pi.flags |= ATA_FLAG_PMP;
    if (hpriv.cap & HOST_CAP_64)
    dma_set_coherent_mask(dev, DMA_BIT_MASK(64));
// CAP.NP sometimes indicate the index of the last enabled
// port, at other times, that of the last possible port, so
// determining the maximum port number requires looking at
// both CAP.NP and port_map.
//
    n_ports = max(ahci_nr_ports(hpriv.cap), fls(hpriv.port_map));
    pdata.n_ports = n_ports;
    hpriv.plat_data = pdata;
    highbank_set_em_messages(dev, hpriv, &pi);
    host = ata_host_alloc_pinfo(dev, ppi, n_ports);
    if (!host) {
    rc = -ENOMEM;
    goto err0;
    }
    host.private_data = hpriv;
    if (!(hpriv.cap & HOST_CAP_SSS) || ahci_ignore_sss)
    host.flags |= ATA_HOST_PARALLEL_SCAN;
    for (i = 0; i < host.n_ports; i++) {
    struct ata_port *ap = host.ports[i];
    ata_port_desc(ap, "port 0x%x", 0x100 + ap.port_no * 0x80);
// set enclosure management message type
    if (ap.flags & ATA_FLAG_EM)
    ap.em_message_type = hpriv.em_msg_type;
// disabled/not-implemented port
    if (!(hpriv.port_map & (1 << i)))
    ap.ops = &ata_dummy_port_ops;
    }
    rc = ahci_reset_controller(host);
    if (rc)
    goto err0;
    ahci_init_controller(host);
    ahci_print_info(host, "platform");
    rc = ahci_host_activate(host, &ahci_highbank_platform_sht);
    if (rc)
    goto err0;
    return 0;
    err0:
    return rc;
    }

#[no_mangle]
unsafe extern "C" fn ahci_highbank_suspend(dev: *mut device) -> c_int {
    static int ahci_highbank_suspend(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    void __iomem *mmio = hpriv.mmio;
    u32 ctl;
    if (hpriv.flags & AHCI_HFLAG_NO_SUSPEND) {
    dev_err(dev, "firmware update required for suspend/resume\n");
    return -EIO;
    }
//
// AHCI spec rev1.1 section 8.3.3:
// Software must disable interrupts prior to requesting a
// transition of the HBA to D3 state.
//
    ctl = readl(mmio + HOST_CTL);
    ctl &= ~HOST_IRQ_EN;
    writel(ctl, mmio + HOST_CTL);
    readl(mmio + HOST_CTL); /* flush */
    ata_host_suspend(host, PMSG_SUSPEND);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ahci_highbank_resume(dev: *mut device) -> c_int {
    static int ahci_highbank_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    int rc;
    if (dev.power.power_state.event == PM_EVENT_SUSPEND) {
    rc = ahci_reset_controller(host);
    if (rc)
    return rc;
    ahci_init_controller(host);
    }
    ata_host_resume(host);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(ahci_highbank_pm_ops,
    ahci_highbank_suspend, ahci_highbank_resume);
    static struct platform_driver ahci_highbank_driver = {
    .remove = ata_platform_remove_one,
    .driver = {
    .name = "highbank-ahci",
    .of_match_table = ahci_of_match,
    .pm = &ahci_highbank_pm_ops,
    },
    .probe = ahci_highbank_probe,
    };
    module_platform_driver(ahci_highbank_driver);
    MODULE_DESCRIPTION("Calxeda Highbank AHCI SATA platform driver");
    MODULE_AUTHOR("Mark Langsdorf <mark.langsdorf@calxeda.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("sata:highbank");
