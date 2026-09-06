//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pcie-apple.c
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
// PCIe host bridge driver for Apple system-on-chips.
//
// The HW is ECAM compliant, so once the controller is initialized,
// the driver mostly deals MSI mapping and handling of per-port
// interrupts (INTx, management and error signals).
//
// Initialization requires enabling power and clocks, along with a
// number of register pokes.
//
// Copyright (C) 2021 Alyssa Rosenzweig <alyssa@rosenzweig.io>
// Copyright (C) 2021 Google LLC
// Copyright (C) 2021 Corellium LLC
// Copyright (C) 2021 Mark Kettenis <kettenis@openbsd.org>
//
// Author: Alyssa Rosenzweig <alyssa@rosenzweig.io>
// Author: Marc Zyngier <maz@kernel.org>
//

// T8103 (original M1) and related SoCs
pub const CORE_RC_PHYIF_CTL: c_uint = 0x00024;

pub const CORE_RC_PHYIF_STAT: c_uint = 0x00028;

pub const CORE_RC_CTL: c_uint = 0x00050;

pub const CORE_RC_STAT: c_uint = 0x00058;

pub const CORE_FABRIC_STAT: c_uint = 0x04000;
pub const CORE_FABRIC_STAT_MASK: c_uint = 0x001F001F;

pub const PHY_LANE_CFG: c_uint = 0x00000;

pub const PHY_LANE_CTL: c_uint = 0x00004;

pub const PORT_LTSSMCTL: c_uint = 0x00080;

pub const PORT_INTSTAT: c_uint = 0x00100;
pub const PORT_INT_TUNNEL_ERR: c_int = 31;
pub const PORT_INT_CPL_TIMEOUT: c_int = 23;
pub const PORT_INT_RID2SID_MAPERR: c_int = 22;
pub const PORT_INT_CPL_ABORT: c_int = 21;
pub const PORT_INT_MSI_BAD_DATA: c_int = 19;
pub const PORT_INT_MSI_ERR: c_int = 18;
pub const PORT_INT_REQADDR_GT32: c_int = 17;
pub const PORT_INT_AF_TIMEOUT: c_int = 15;
pub const PORT_INT_LINK_DOWN: c_int = 14;
pub const PORT_INT_LINK_UP: c_int = 12;
pub const PORT_INT_LINK_BWMGMT: c_int = 11;

pub const PORT_INT_PORT_ERR: c_int = 4;

pub const PORT_INT_INTx_MASK: c_int = 15;
pub const PORT_INTMSK: c_uint = 0x00104;
pub const PORT_INTMSKSET: c_uint = 0x00108;
pub const PORT_INTMSKCLR: c_uint = 0x0010c;
pub const PORT_MSICFG: c_uint = 0x00124;

pub const PORT_MSICFG_L2MSINUM_SHIFT: c_int = 4;
pub const PORT_MSIBASE: c_uint = 0x00128;
pub const PORT_MSIBASE_1_SHIFT: c_int = 16;
pub const PORT_MSIADDR: c_uint = 0x00168;
pub const PORT_LINKSTS: c_uint = 0x00208;

pub const PORT_LINKCMDSTS: c_uint = 0x00210;
pub const PORT_OUTS_NPREQS: c_uint = 0x00284;

pub const PORT_RXWR_FIFO: c_uint = 0x00288;

pub const PORT_RXRD_FIFO: c_uint = 0x0028C;

pub const PORT_OUTS_CPLS: c_uint = 0x00290;

pub const PORT_APPCLK: c_uint = 0x00800;

pub const PORT_STATUS: c_uint = 0x00804;

pub const PORT_REFCLK: c_uint = 0x00810;

pub const PORT_PERST: c_uint = 0x00814;

pub const PORT_RID2SID: c_uint = 0x00828;

pub const PORT_RID2SID_SID_SHIFT: c_int = 16;
pub const PORT_RID2SID_BUS_SHIFT: c_int = 8;
pub const PORT_RID2SID_DEV_SHIFT: c_int = 3;
pub const PORT_RID2SID_FUNC_SHIFT: c_int = 0;
pub const PORT_OUTS_PREQS_HDR: c_uint = 0x00980;

pub const PORT_OUTS_PREQS_DATA: c_uint = 0x00984;

pub const PORT_TUNCTRL: c_uint = 0x00988;

pub const PORT_TUNSTAT: c_uint = 0x0098c;

pub const PORT_PREFMEM_ENABLE: c_uint = 0x00994;
// T602x (M2-pro and co)
pub const PORT_T602X_MSIADDR: c_uint = 0x016c;
pub const PORT_T602X_MSIADDR_HI: c_uint = 0x0170;
pub const PORT_T602X_PERST: c_uint = 0x082c;
pub const PORT_T602X_RID2SID: c_uint = 0x3000;
pub const PORT_T602X_MSIMAP: c_uint = 0x3800;

//
// The doorbell address is set to 0xfffff000, which by convention
// matches what MacOS does, and it is possible to use any other
// address (in the bottom 4GB, as the base register is only 32bit).
// However, it has to be excluded from the IOVA range, and the DART
// driver has to know about it.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_info {
    pub phy_lane_ctl: u32,
    pub port_msiaddr: u32,
    pub port_msiaddr_hi: u32,
    pub port_refclk: u32,
    pub port_perst: u32,
    pub port_rid2sid: u32,
    pub port_msimap: u32,
    pub max_rid2sid: u32,
}

    static const struct hw_info t8103_hw = {
    .phy_lane_ctl		= PHY_LANE_CTL,
    .port_msiaddr		= PORT_MSIADDR,
    .port_msiaddr_hi	= 0,
    .port_refclk		= PORT_REFCLK,
    .port_perst		= PORT_PERST,
    .port_rid2sid		= PORT_RID2SID,
    .port_msimap		= 0,
    .max_rid2sid		= 64,
    };
    static const struct hw_info t602x_hw = {
    .phy_lane_ctl		= 0,
    .port_msiaddr		= PORT_T602X_MSIADDR,
    .port_msiaddr_hi	= PORT_T602X_MSIADDR_HI,
    .port_refclk		= 0,
    .port_perst		= PORT_T602X_PERST,
    .port_rid2sid		= PORT_T602X_RID2SID,
    .port_msimap		= PORT_T602X_MSIMAP,
// 16 on t602x, guess for autodetect on future HW
    .max_rid2sid		= 512,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_pcie {
    pub lock: mutex,
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub hw: *const hw_info,
    pub bitmap: *mut c_ulong,
    pub ports: list_head,
    pub event: completion,
    pub fwspec: irq_fwspec,
    pub nvecs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_pcie_port {
    pub lock: raw_spinlock_t,
    pub pcie: *mut apple_pcie,
    pub np: *mut device_node,
    pub base: *mut void __iomem,
    pub phy: *mut void __iomem,
    pub domain: *mut irq_domain,
    pub entry: list_head,
    pub sid_map: *mut c_ulong,
    pub sid_map_sz: c_int,
    pub idx: c_int,
}

#[no_mangle]
unsafe extern "C" fn rmw_set(set: u32, addr: *mut void __iomem) {
    static void rmw_set(u32 set, void __iomem *addr)
    {
    writel_relaxed(readl_relaxed(addr) | set, addr);
    }
#[no_mangle]
unsafe extern "C" fn rmw_clear(clr: u32, addr: *mut void __iomem) {
    static void rmw_clear(u32 clr, void __iomem *addr)
    {
    writel_relaxed(readl_relaxed(addr) & ~clr, addr);
    }
#[no_mangle]
unsafe extern "C" fn apple_msi_compose_msg(data: *mut irq_data, msg: *mut msi_msg) {
    static void apple_msi_compose_msg(struct irq_data *data, struct msi_msg *msg)
    {
    msg.address_hi = upper_32_bits(DOORBELL_ADDR);
    msg.address_lo = lower_32_bits(DOORBELL_ADDR);
    msg.data = data.hwirq;
    }
    static struct irq_chip apple_msi_bottom_chip = {
    .name			= "MSI",
    .irq_mask		= irq_chip_mask_parent,
    .irq_unmask		= irq_chip_unmask_parent,
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .irq_set_type		= irq_chip_set_type_parent,
    .irq_compose_msi_msg	= apple_msi_compose_msg,
    };
    static int apple_msi_domain_alloc(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs, void *args)
    {
    struct apple_pcie *pcie = domain.host_data;
    let mut fwspec: irq_fwspec = pcie.fwspec;
    unsigned int i;
    int ret, hwirq;
    mutex_lock(&pcie.lock);
    hwirq = bitmap_find_free_region(pcie.bitmap, pcie.nvecs,
    order_base_2(nr_irqs));
    mutex_unlock(&pcie.lock);
    if (hwirq < 0)
    return -ENOSPC;
    fwspec.param[fwspec.param_count - 2] += hwirq;
    ret = irq_domain_alloc_irqs_parent(domain, virq, nr_irqs, &fwspec);
    if (ret)
    return ret;
    for (i = 0; i < nr_irqs; i++) {
    irq_domain_set_hwirq_and_chip(domain, virq + i, hwirq + i,
    &apple_msi_bottom_chip, pcie);
    }
    return 0;
    }
    static void apple_msi_domain_free(struct irq_domain *domain, unsigned int virq,
    unsigned int nr_irqs)
    {
    struct irq_data *d = irq_domain_get_irq_data(domain, virq);
    struct apple_pcie *pcie = domain.host_data;
    mutex_lock(&pcie.lock);
    bitmap_release_region(pcie.bitmap, d.hwirq, order_base_2(nr_irqs));
    mutex_unlock(&pcie.lock);
    }
    static const struct irq_domain_ops apple_msi_domain_ops = {
    .alloc	= apple_msi_domain_alloc,
    .free	= apple_msi_domain_free,
    };
#[no_mangle]
unsafe extern "C" fn apple_port_irq_mask(data: *mut irq_data) {
    static void apple_port_irq_mask(struct irq_data *data)
    {
    struct apple_pcie_port *port = irq_data_get_irq_chip_data(data);
    guard(raw_spinlock_irqsave)(&port.lock);
    rmw_set(BIT(data.hwirq), port.base + PORT_INTMSK);
    }
#[no_mangle]
unsafe extern "C" fn apple_port_irq_unmask(data: *mut irq_data) {
    static void apple_port_irq_unmask(struct irq_data *data)
    {
    struct apple_pcie_port *port = irq_data_get_irq_chip_data(data);
    guard(raw_spinlock_irqsave)(&port.lock);
    rmw_clear(BIT(data.hwirq), port.base + PORT_INTMSK);
    }
#[no_mangle]
unsafe extern "C" fn hwirq_is_intx(hwirq: c_uint) -> bool {
    static bool hwirq_is_intx(unsigned int hwirq)
    {
    return BIT(hwirq) & PORT_INT_INTx_MASK;
    }
#[no_mangle]
unsafe extern "C" fn apple_port_irq_ack(data: *mut irq_data) {
    static void apple_port_irq_ack(struct irq_data *data)
    {
    struct apple_pcie_port *port = irq_data_get_irq_chip_data(data);
    if (!hwirq_is_intx(data.hwirq))
    writel_relaxed(BIT(data.hwirq), port.base + PORT_INTSTAT);
    }
#[no_mangle]
unsafe extern "C" fn apple_port_irq_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int apple_port_irq_set_type(struct irq_data *data, unsigned int type)
    {
//
// It doesn't seem that there is any way to configure the
// trigger, so assume INTx have to be level (as per the spec),
// and the rest is edge (which looks likely).
//
    if (hwirq_is_intx(data.hwirq) ^ !!(type & IRQ_TYPE_LEVEL_MASK))
    return -EINVAL;
    irqd_set_trigger_type(data, type);
    return 0;
    }
    static struct irq_chip apple_port_irqchip = {
    .name		= "PCIe",
    .irq_ack	= apple_port_irq_ack,
    .irq_mask	= apple_port_irq_mask,
    .irq_unmask	= apple_port_irq_unmask,
    .irq_set_type	= apple_port_irq_set_type,
    };
    static int apple_port_irq_domain_alloc(struct irq_domain *domain,
    unsigned int virq, unsigned int nr_irqs,
    void *args)
    {
    struct apple_pcie_port *port = domain.host_data;
    struct irq_fwspec *fwspec = args;
    int i;
    for (i = 0; i < nr_irqs; i++) {
    let mut flow: irq_flow_handler_t = handle_edge_irq;
    let mut type: c_uint = IRQ_TYPE_EDGE_RISING;
    if (hwirq_is_intx(fwspec.param[0] + i)) {
    flow = handle_level_irq;
    type = IRQ_TYPE_LEVEL_HIGH;
    }
    irq_domain_set_info(domain, virq + i, fwspec.param[0] + i,
    &apple_port_irqchip, port, flow,
    core::ptr::null_mut(), core::ptr::null_mut());
    irq_set_irq_type(virq + i, type);
    }
    return 0;
    }
    static void apple_port_irq_domain_free(struct irq_domain *domain,
    unsigned int virq, unsigned int nr_irqs)
    {
    int i;
    for (i = 0; i < nr_irqs; i++) {
    struct irq_data *d = irq_domain_get_irq_data(domain, virq + i);
    irq_set_handler(virq + i, core::ptr::null_mut());
    irq_domain_reset_irq_data(d);
    }
    }
    static const struct irq_domain_ops apple_port_irq_domain_ops = {
    .translate	= irq_domain_translate_onecell,
    .alloc		= apple_port_irq_domain_alloc,
    .free		= apple_port_irq_domain_free,
    };
#[no_mangle]
unsafe extern "C" fn apple_port_irq_handler(desc: *mut irq_desc) {
    static void apple_port_irq_handler(struct irq_desc *desc)
    {
    struct apple_pcie_port *port = irq_desc_get_handler_data(desc);
    struct irq_chip *chip = irq_desc_get_chip(desc);
    unsigned long stat;
    int i;
    chained_irq_enter(chip, desc);
    stat = readl_relaxed(port.base + PORT_INTSTAT);
    for_each_set_bit(i, &stat, 32)
    generic_handle_domain_irq(port.domain, i);
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn apple_pcie_port_setup_irq(port: *mut apple_pcie_port) -> c_int {
    static int apple_pcie_port_setup_irq(struct apple_pcie_port *port)
    {
    struct fwnode_handle *fwnode = &port.np.fwnode;
    struct apple_pcie *pcie = port.pcie;
    unsigned int irq;
    let mut val: u32 = 0;
// FIXME: consider moving each interrupt under each port
    irq = irq_of_parse_and_map(to_of_node(dev_fwnode(port.pcie.dev)),
    port.idx);
    if (!irq)
    return -ENXIO;
    port.domain = irq_domain_create_linear(fwnode, 32,
    &apple_port_irq_domain_ops,
    port);
    if (!port.domain)
    return -ENOMEM;
// Disable all interrupts
    writel_relaxed(~0, port.base + PORT_INTMSK);
    writel_relaxed(~0, port.base + PORT_INTSTAT);
    writel_relaxed(~0, port.base + PORT_LINKCMDSTS);
    irq_set_chained_handler_and_data(irq, apple_port_irq_handler, port);
// Configure MSI base address
    BUILD_BUG_ON(upper_32_bits(DOORBELL_ADDR));
    writel_relaxed(lower_32_bits(DOORBELL_ADDR),
    port.base + pcie.hw.port_msiaddr);
    if (pcie.hw.port_msiaddr_hi)
    writel_relaxed(0, port.base + pcie.hw.port_msiaddr_hi);
// Enable MSIs, shared between all ports
    if (pcie.hw.port_msimap) {
    for (int i = 0; i < pcie.nvecs; i++)
    writel_relaxed(FIELD_PREP(PORT_MSIMAP_TARGET, i) |
    PORT_MSIMAP_ENABLE,
    port.base + pcie.hw.port_msimap + 4 * i);
    } else {
    writel_relaxed(0, port.base + PORT_MSIBASE);
    val = ilog2(pcie.nvecs) << PORT_MSICFG_L2MSINUM_SHIFT;
    }
    writel_relaxed(val | PORT_MSICFG_EN, port.base + PORT_MSICFG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apple_pcie_port_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t apple_pcie_port_irq(int irq, void *data)
    {
    struct apple_pcie_port *port = data;
    let mut hwirq: c_uint = irq_domain_get_irq_data(port.domain, irq).hwirq;
    switch (hwirq) {
    case PORT_INT_LINK_UP:
    dev_info_ratelimited(port.pcie.dev, "Link up on %pOF\n",
    port.np);
    complete_all(&port.pcie.event);
    break;
    case PORT_INT_LINK_DOWN:
    dev_info_ratelimited(port.pcie.dev, "Link down on %pOF\n",
    port.np);
    break;
    default:
    return IRQ_NONE;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn apple_pcie_port_register_irqs(port: *mut apple_pcie_port) -> c_int {
    static int apple_pcie_port_register_irqs(struct apple_pcie_port *port)
    {
    static struct {
    unsigned int	hwirq;
    const char	*name;
    } port_irqs[] = {
    { PORT_INT_LINK_UP,	"Link up",	},
    { PORT_INT_LINK_DOWN,	"Link down",	},
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(port_irqs); i++) {
    struct irq_fwspec fwspec = {
    .fwnode		= &port.np.fwnode,
    .param_count	= 1,
    .param		= {
    [0]	= port_irqs[i].hwirq,
    },
    };
    unsigned int irq;
    int ret;
    irq = irq_domain_alloc_irqs(port.domain, 1, NUMA_NO_NODE,
    &fwspec);
    if (WARN_ON(!irq))
    continue;
    ret = request_irq(irq, apple_pcie_port_irq, 0,
    port_irqs[i].name, port);
    WARN_ON(ret);
    }
    return 0;
    }
    static int apple_pcie_setup_refclk(struct apple_pcie *pcie,
    struct apple_pcie_port *port)
    {
    u32 stat;
    int res;
    if (pcie.hw.phy_lane_ctl)
    rmw_set(PHY_LANE_CTL_CFGACC, port.phy + pcie.hw.phy_lane_ctl);
    rmw_set(PHY_LANE_CFG_REFCLK0REQ, port.phy + PHY_LANE_CFG);
    res = readl_relaxed_poll_timeout(port.phy + PHY_LANE_CFG,
    stat, stat & PHY_LANE_CFG_REFCLK0ACK,
    100, 50000);
    if (res < 0)
    return res;
    rmw_set(PHY_LANE_CFG_REFCLK1REQ, port.phy + PHY_LANE_CFG);
    res = readl_relaxed_poll_timeout(port.phy + PHY_LANE_CFG,
    stat, stat & PHY_LANE_CFG_REFCLK1ACK,
    100, 50000);
    if (res < 0)
    return res;
    if (pcie.hw.phy_lane_ctl)
    rmw_clear(PHY_LANE_CTL_CFGACC, port.phy + pcie.hw.phy_lane_ctl);
    rmw_set(PHY_LANE_CFG_REFCLKEN, port.phy + PHY_LANE_CFG);
    if (pcie.hw.port_refclk)
    rmw_set(PORT_REFCLK_EN, port.base + pcie.hw.port_refclk);
    return 0;
    }
    static void __iomem *port_rid2sid_addr(struct apple_pcie_port *port, int idx)
    {
    return port.base + port.pcie.hw.port_rid2sid + 4 * idx;
    }
    static u32 apple_pcie_rid2sid_write(struct apple_pcie_port *port,
    int idx, u32 val)
    {
    writel_relaxed(val, port_rid2sid_addr(port, idx));
// Read back to ensure completion of the write
    return readl_relaxed(port_rid2sid_addr(port, idx));
    }
    static int apple_pcie_setup_port(struct apple_pcie *pcie,
    struct device_node *np)
    {
    struct platform_device *platform = to_platform_device(pcie.dev);
    struct apple_pcie_port *port;
    struct gpio_desc *reset;
    struct resource *res;
    char name[16];
    u32 stat, idx;
    int ret, i;
    reset = devm_fwnode_gpiod_get(pcie.dev, of_fwnode_handle(np), "reset",
    GPIOD_OUT_LOW, "PERST#");
    if (IS_ERR(reset))
    return PTR_ERR(reset);
    port = devm_kzalloc(pcie.dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    port.sid_map = devm_bitmap_zalloc(pcie.dev, pcie.hw.max_rid2sid, GFP_KERNEL);
    if (!port.sid_map)
    return -ENOMEM;
    ret = of_property_read_u32_index(np, "reg", 0, &idx);
    if (ret)
    return ret;
// Use the first reg entry to work out the port index
    port.idx = idx >> 11;
    port.pcie = pcie;
    port.np = np;
    raw_spin_lock_init(&port.lock);
    snprintf(name, sizeof(name), "port%d", port.idx);
    res = platform_get_resource_byname(platform, IORESOURCE_MEM, name);
    if (!res)
    res = platform_get_resource(platform, IORESOURCE_MEM, port.idx + 2);
    port.base = devm_ioremap_resource(&platform.dev, res);
    if (IS_ERR(port.base))
    return PTR_ERR(port.base);
    snprintf(name, sizeof(name), "phy%d", port.idx);
    res = platform_get_resource_byname(platform, IORESOURCE_MEM, name);
    if (res)
    port.phy = devm_ioremap_resource(&platform.dev, res);
    else
    port.phy = pcie.base + CORE_PHY_DEFAULT_BASE(port.idx);
    rmw_set(PORT_APPCLK_EN, port.base + PORT_APPCLK);
// Assert PERST# before setting up the clock
    gpiod_set_value_cansleep(reset, 1);
    ret = apple_pcie_setup_refclk(pcie, port);
    if (ret < 0)
    return ret;
// The minimal Tperst-clk value is 100us (PCIe CEM r5.0, 2.9.2)
    usleep_range(100, 200);
// Deassert PERST#
    rmw_set(PORT_PERST_OFF, port.base + pcie.hw.port_perst);
    gpiod_set_value_cansleep(reset, 0);
// Wait for 100ms after PERST# deassertion (PCIe r5.0, 6.6.1)
    msleep(100);
    ret = readl_relaxed_poll_timeout(port.base + PORT_STATUS, stat,
    stat & PORT_STATUS_READY, 100, 250000);
    if (ret < 0) {
    dev_err(pcie.dev, "port %pOF ready wait timeout\n", np);
    return ret;
    }
    if (pcie.hw.port_refclk)
    rmw_clear(PORT_REFCLK_CGDIS, port.base + pcie.hw.port_refclk);
    else
    rmw_set(PHY_LANE_CFG_REFCLKCGEN, port.phy + PHY_LANE_CFG);
    rmw_clear(PORT_APPCLK_CGDIS, port.base + PORT_APPCLK);
    ret = apple_pcie_port_setup_irq(port);
    if (ret)
    return ret;
// Reset all RID/SID mappings, and check for RAZ/WI registers
    for (i = 0; i < pcie.hw.max_rid2sid; i++) {
    if (apple_pcie_rid2sid_write(port, i, 0xbad1d) != 0xbad1d)
    break;
    apple_pcie_rid2sid_write(port, i, 0);
    }
    dev_dbg(pcie.dev, "%pOF: %d RID/SID mapping entries\n", np, i);
    port.sid_map_sz = i;
    list_add_tail(&port.entry, &pcie.ports);
    init_completion(&pcie.event);
// In the success path, we keep a reference to np around
    of_node_get(np);
    ret = apple_pcie_port_register_irqs(port);
    WARN_ON(ret);
    writel_relaxed(PORT_LTSSMCTL_START, port.base + PORT_LTSSMCTL);
    if (!wait_for_completion_timeout(&pcie.event, HZ / 10))
    dev_warn(pcie.dev, "%pOF link didn't come up\n", np);
    return 0;
    }
    static const struct msi_parent_ops apple_msi_parent_ops = {
    .supported_flags	= (MSI_GENERIC_FLAGS_MASK	|
    MSI_FLAG_PCI_MSIX		|
    MSI_FLAG_MULTI_PCI_MSI),
    .required_flags		= (MSI_FLAG_USE_DEF_DOM_OPS	|
    MSI_FLAG_USE_DEF_CHIP_OPS	|
    MSI_FLAG_PCI_MSI_MASK_PARENT),
    .chip_flags		= MSI_CHIP_FLAG_SET_EOI,
    .bus_select_token	= DOMAIN_BUS_PCI_MSI,
    .init_dev_msi_info	= msi_lib_init_dev_msi_info,
    };
#[no_mangle]
unsafe extern "C" fn apple_msi_init(pcie: *mut apple_pcie) -> c_int {
    static int apple_msi_init(struct apple_pcie *pcie)
    {
    struct fwnode_handle *fwnode = dev_fwnode(pcie.dev);
    struct irq_domain_info info = {
    .fwnode		= fwnode,
    .ops		= &apple_msi_domain_ops,
    .size		= pcie.nvecs,
    .host_data	= pcie,
    };
    let mut args: of_phandle_args = {};
    int ret;
    ret = of_parse_phandle_with_args(to_of_node(fwnode), "msi-ranges",
    "#interrupt-cells", 0, &args);
    if (ret)
    return ret;
    ret = of_property_read_u32_index(to_of_node(fwnode), "msi-ranges",
    args.args_count + 1, &pcie.nvecs);
    if (ret)
    return ret;
    of_phandle_args_to_fwspec(args.np, args.args, args.args_count,
    &pcie.fwspec);
    pcie.bitmap = devm_bitmap_zalloc(pcie.dev, pcie.nvecs, GFP_KERNEL);
    if (!pcie.bitmap)
    return -ENOMEM;
    info.parent = irq_find_matching_fwspec(&pcie.fwspec, DOMAIN_BUS_WIRED);
    if (!info.parent) {
    dev_err(pcie.dev, "failed to find parent domain\n");
    return -ENXIO;
    }
    if (!msi_create_parent_irq_domain(&info, &apple_msi_parent_ops)) {
    dev_err(pcie.dev, "failed to create IRQ domain\n");
    return -ENOMEM;
    }
    return 0;
    }
    static struct apple_pcie *apple_pcie_lookup(struct device *dev)
    {
    return pci_host_bridge_priv(dev_get_drvdata(dev));
    }
    static struct apple_pcie_port *apple_pcie_get_port(struct pci_dev *pdev)
    {
    struct pci_config_window *cfg = pdev.sysdata;
    struct apple_pcie *pcie;
    struct pci_dev *port_pdev;
    struct apple_pcie_port *port;
    pcie = apple_pcie_lookup(cfg.parent);
    if (WARN_ON(!pcie))
    return core::ptr::null_mut();
// Find the root port this device is on
    port_pdev = pcie_find_root_port(pdev);
// If finding the port itself, nothing to do
    if (WARN_ON(!port_pdev) || pdev == port_pdev)
    return core::ptr::null_mut();
    list_for_each_entry(port, &pcie.ports, entry) {
    if (port.idx == PCI_SLOT(port_pdev.devfn))
    return port;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn apple_pcie_enable_device(bridge: *mut pci_host_bridge, pdev: *mut pci_dev) -> c_int {
    static int apple_pcie_enable_device(struct pci_host_bridge *bridge, struct pci_dev *pdev)
    {
    u32 sid, rid = pci_dev_id(pdev);
    struct apple_pcie_port *port;
    let mut iommu_spec: of_phandle_args = {};
    int idx, err;
    port = apple_pcie_get_port(pdev);
    if (!port)
    return 0;
    dev_dbg(&pdev.dev, "added to bus %s, index %d\n",
    pci_name(pdev.bus.self), port.idx);
    err = of_map_iommu_id(port.pcie.dev.of_node, rid, &iommu_spec);
    if (err)
    return err;
    of_node_put(iommu_spec.np);
    sid = iommu_spec.args[0];
    mutex_lock(&port.pcie.lock);
    idx = bitmap_find_free_region(port.sid_map, port.sid_map_sz, 0);
    if (idx >= 0) {
    apple_pcie_rid2sid_write(port, idx,
    PORT_RID2SID_VALID |
    (sid << PORT_RID2SID_SID_SHIFT) | rid);
    dev_dbg(&pdev.dev, "mapping RID%x to SID%x (index %d)\n",
    rid, sid, idx);
    }
    mutex_unlock(&port.pcie.lock);
    return idx >= 0 ? 0 : -ENOSPC;
    }
#[no_mangle]
unsafe extern "C" fn apple_pcie_disable_device(bridge: *mut pci_host_bridge, pdev: *mut pci_dev) {
    static void apple_pcie_disable_device(struct pci_host_bridge *bridge, struct pci_dev *pdev)
    {
    struct apple_pcie_port *port;
    let mut rid: u32 = pci_dev_id(pdev);
    int idx;
    port = apple_pcie_get_port(pdev);
    if (!port)
    return;
    mutex_lock(&port.pcie.lock);
    for_each_set_bit(idx, port.sid_map, port.sid_map_sz) {
    u32 val;
    val = readl_relaxed(port_rid2sid_addr(port, idx));
    if ((val & 0xffff) == rid) {
    apple_pcie_rid2sid_write(port, idx, 0);
    bitmap_release_region(port.sid_map, idx, 0);
    dev_dbg(&pdev.dev, "Released %x (%d)\n", val, idx);
    break;
    }
    }
    mutex_unlock(&port.pcie.lock);
    }
#[no_mangle]
unsafe extern "C" fn apple_pcie_init(cfg: *mut pci_config_window) -> c_int {
    static int apple_pcie_init(struct pci_config_window *cfg)
    {
    struct device *dev = cfg.parent;
    struct apple_pcie *pcie;
    int ret;
    pcie = apple_pcie_lookup(dev);
    if (WARN_ON(!pcie))
    return -ENOENT;
    for_each_available_child_of_node_scoped(dev.of_node, of_port) {
    ret = apple_pcie_setup_port(pcie, of_port);
    if (ret) {
    dev_err(dev, "Port %pOF setup fail: %d\n", of_port, ret);
    return ret;
    }
    }
    return 0;
    }
    static const struct pci_ecam_ops apple_pcie_cfg_ecam_ops = {
    .init		= apple_pcie_init,
    .enable_device	= apple_pcie_enable_device,
    .disable_device	= apple_pcie_disable_device,
    .pci_ops	= {
    .map_bus	= pci_ecam_map_bus,
    .read		= pci_generic_config_read,
    .write		= pci_generic_config_write,
    }
    };
#[no_mangle]
unsafe extern "C" fn apple_pcie_probe(pdev: *mut platform_device) -> c_int {
    static int apple_pcie_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct pci_host_bridge *bridge;
    struct apple_pcie *pcie;
    int ret;
    bridge = devm_pci_alloc_host_bridge(dev, sizeof(*pcie));
    if (!bridge)
    return -ENOMEM;
    pcie = pci_host_bridge_priv(bridge);
    pcie.dev = dev;
    pcie.hw = of_device_get_match_data(dev);
    if (!pcie.hw)
    return -ENODEV;
    pcie.base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(pcie.base))
    return PTR_ERR(pcie.base);
    mutex_init(&pcie.lock);
    INIT_LIST_HEAD(&pcie.ports);
    ret = apple_msi_init(pcie);
    if (ret)
    return ret;
    return pci_host_common_init(pdev, bridge, &apple_pcie_cfg_ecam_ops);
    }
    static const struct of_device_id apple_pcie_of_match[] = {
    { .compatible = "apple,t6020-pcie",	.data = &t602x_hw },
    { .compatible = "apple,pcie",		.data = &t8103_hw },
    { }
    };
    MODULE_DEVICE_TABLE(of, apple_pcie_of_match);
    static struct platform_driver apple_pcie_driver = {
    .probe	= apple_pcie_probe,
    .driver	= {
    .name			= "pcie-apple",
    .of_match_table		= apple_pcie_of_match,
    .suppress_bind_attrs	= true,
    },
    };
    module_platform_driver(apple_pcie_driver);
    MODULE_DESCRIPTION("Apple PCIe host bridge driver");
    MODULE_LICENSE("GPL v2");
