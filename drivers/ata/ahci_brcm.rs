//! Automatically rewritten from C to Rust
//! Source: drivers/ata/ahci_brcm.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Broadcom SATA3 AHCI Controller Driver
//
// Copyright © 2009-2015 Broadcom Corporation
//

pub const SATA_TOP_CTRL_VERSION: c_uint = 0x0;
pub const SATA_TOP_CTRL_BUS_CTRL: c_uint = 0x4;

pub const PIODATA_ENDIAN_SHIFT: c_int = 6;
pub const ENDIAN_SWAP_NONE: c_int = 0;
pub const ENDIAN_SWAP_FULL: c_int = 2;
pub const SATA_TOP_CTRL_TP_CTRL: c_uint = 0x8;
pub const SATA_TOP_CTRL_PHY_CTRL: c_uint = 0xc;
pub const SATA_TOP_CTRL_PHY_CTRL_1: c_uint = 0x0;

pub const SATA_TOP_CTRL_PHY_CTRL_2: c_uint = 0x4;

pub const SATA_TOP_CTRL_PHY_OFFS: c_uint = 0x8;
pub const SATA_TOP_MAX_PHYS: c_int = 2;
pub const SATA_FIRST_PORT_CTRL: c_uint = 0x700;
pub const SATA_NEXT_PORT_CTRL_OFFSET: c_uint = 0x80;

// On big-endian MIPS, buses are reversed to big endian, so switch them back

pub const DATA_ENDIAN: c_int = 0;
pub const MMIO_ENDIAN: c_int = 0;

    ((DATA_ENDIAN << DMADATA_ENDIAN_SHIFT) |	\
    (DATA_ENDIAN << DMADESC_ENDIAN_SHIFT) |		\
    (MMIO_ENDIAN << MMIO_ENDIAN_SHIFT))

    (0x02 << DMADATA_ENDIAN_SHIFT | 0x02 << DMADESC_ENDIAN_SHIFT)

    (0x3 << MMIO_ENDIAN_SHIFT | 0x3 << DMADESC_ENDIAN_SHIFT |	\
    0x3 << DMADATA_ENDIAN_SHIFT | 0x3 << PIODATA_ENDIAN_SHIFT)
    enum brcm_ahci_version {
    BRCM_SATA_BCM7425 = 1,
    BRCM_SATA_BCM7445,
    BRCM_SATA_NSP,
    BRCM_SATA_BCM7216,
    };
    enum brcm_ahci_quirks {
    BRCM_AHCI_QUIRK_SKIP_PHY_ENABLE	= BIT(0),
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcm_ahci_priv {
    pub dev: *mut device,
    pub top_ctrl: *mut void __iomem,
    pub port_mask: u32,
    pub quirks: u32,
    pub version: enum brcm_ahci_version,
    pub rcdev_rescal: *mut reset_control,
    pub rcdev_ahci: *mut reset_control,
}

#[no_mangle]
pub unsafe extern "C" fn brcm_sata_readreg(addr: *mut void __iomem) -> u32 {
    static inline u32 brcm_sata_readreg(void __iomem *addr)
    {
//
// MIPS endianness is configured by boot strap, which also reverses all
// bus endianness (i.e., big-endian CPU + big endian bus ==> native
// endian I/O).
//
// Other architectures (e.g., ARM) either do not support big endian, or
// else leave I/O in little endian mode.
//
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    return __raw_readl(addr);
    else
    return readl_relaxed(addr);
    }
#[no_mangle]
pub unsafe extern "C" fn brcm_sata_writereg(val: u32, addr: *mut void __iomem) {
    static inline void brcm_sata_writereg(u32 val, void __iomem *addr)
    {
// See brcm_sata_readreg() comments
    if (IS_ENABLED(CONFIG_MIPS) && IS_ENABLED(CONFIG_CPU_BIG_ENDIAN))
    __raw_writel(val, addr);
    else
    writel_relaxed(val, addr);
    }
#[no_mangle]
unsafe extern "C" fn brcm_sata_alpm_init(hpriv: *mut ahci_host_priv) {
    static void brcm_sata_alpm_init(struct ahci_host_priv *hpriv)
    {
    struct brcm_ahci_priv *priv = hpriv.plat_data;
    u32 port_ctrl, host_caps;
    int i;
// Enable support for ALPM
    host_caps = readl(hpriv.mmio + HOST_CAP);
    if (!(host_caps & HOST_CAP_ALPM))
    hpriv.flags |= AHCI_HFLAG_YES_ALPM;
//
// Adjust timeout to allow PLL sufficient time to lock while waking
// up from slumber mode.
//
    for (i = 0, port_ctrl = SATA_FIRST_PORT_CTRL;
    i < SATA_TOP_MAX_PHYS;
    i++, port_ctrl += SATA_NEXT_PORT_CTRL_OFFSET) {
    if (priv.port_mask & BIT(i))
    writel(0xff1003fc,
    hpriv.mmio + SATA_PORT_PCTRL6(port_ctrl));
    }
    }
#[no_mangle]
unsafe extern "C" fn brcm_sata_phy_enable(priv: *mut brcm_ahci_priv, port: c_int) {
    static void brcm_sata_phy_enable(struct brcm_ahci_priv *priv, int port)
    {
    void __iomem *phyctrl = priv.top_ctrl + SATA_TOP_CTRL_PHY_CTRL +
    (port * SATA_TOP_CTRL_PHY_OFFS);
    void __iomem *p;
    u32 reg;
    if (priv.quirks & BRCM_AHCI_QUIRK_SKIP_PHY_ENABLE)
    return;
// clear PHY_DEFAULT_POWER_STATE
    p = phyctrl + SATA_TOP_CTRL_PHY_CTRL_1;
    reg = brcm_sata_readreg(p);
    reg &= ~SATA_TOP_CTRL_1_PHY_DEFAULT_POWER_STATE;
    brcm_sata_writereg(reg, p);
// reset the PHY digital logic
    p = phyctrl + SATA_TOP_CTRL_PHY_CTRL_2;
    reg = brcm_sata_readreg(p);
    reg &= ~(SATA_TOP_CTRL_2_SW_RST_MDIOREG | SATA_TOP_CTRL_2_SW_RST_OOB |
    SATA_TOP_CTRL_2_SW_RST_RX);
    reg |= SATA_TOP_CTRL_2_SW_RST_TX;
    brcm_sata_writereg(reg, p);
    reg = brcm_sata_readreg(p);
    reg |= SATA_TOP_CTRL_2_PHY_GLOBAL_RESET;
    brcm_sata_writereg(reg, p);
    reg = brcm_sata_readreg(p);
    reg &= ~SATA_TOP_CTRL_2_PHY_GLOBAL_RESET;
    brcm_sata_writereg(reg, p);
    (void)brcm_sata_readreg(p);
    }
#[no_mangle]
unsafe extern "C" fn brcm_sata_phy_disable(priv: *mut brcm_ahci_priv, port: c_int) {
    static void brcm_sata_phy_disable(struct brcm_ahci_priv *priv, int port)
    {
    void __iomem *phyctrl = priv.top_ctrl + SATA_TOP_CTRL_PHY_CTRL +
    (port * SATA_TOP_CTRL_PHY_OFFS);
    void __iomem *p;
    u32 reg;
    if (priv.quirks & BRCM_AHCI_QUIRK_SKIP_PHY_ENABLE)
    return;
// power-off the PHY digital logic
    p = phyctrl + SATA_TOP_CTRL_PHY_CTRL_2;
    reg = brcm_sata_readreg(p);
    reg |= (SATA_TOP_CTRL_2_SW_RST_MDIOREG | SATA_TOP_CTRL_2_SW_RST_OOB |
    SATA_TOP_CTRL_2_SW_RST_RX | SATA_TOP_CTRL_2_SW_RST_TX |
    SATA_TOP_CTRL_2_PHY_GLOBAL_RESET);
    brcm_sata_writereg(reg, p);
// set PHY_DEFAULT_POWER_STATE
    p = phyctrl + SATA_TOP_CTRL_PHY_CTRL_1;
    reg = brcm_sata_readreg(p);
    reg |= SATA_TOP_CTRL_1_PHY_DEFAULT_POWER_STATE;
    brcm_sata_writereg(reg, p);
    }
#[no_mangle]
unsafe extern "C" fn brcm_sata_phys_enable(priv: *mut brcm_ahci_priv) {
    static void brcm_sata_phys_enable(struct brcm_ahci_priv *priv)
    {
    int i;
    for (i = 0; i < SATA_TOP_MAX_PHYS; i++)
    if (priv.port_mask & BIT(i))
    brcm_sata_phy_enable(priv, i);
    }
#[no_mangle]
unsafe extern "C" fn brcm_sata_phys_disable(priv: *mut brcm_ahci_priv) {
    static void brcm_sata_phys_disable(struct brcm_ahci_priv *priv)
    {
    int i;
    for (i = 0; i < SATA_TOP_MAX_PHYS; i++)
    if (priv.port_mask & BIT(i))
    brcm_sata_phy_disable(priv, i);
    }
    static u32 brcm_ahci_get_portmask(struct ahci_host_priv *hpriv,
    struct brcm_ahci_priv *priv)
    {
    u32 impl;
    impl = readl(hpriv.mmio + HOST_PORTS_IMPL);
    if (fls(impl) > SATA_TOP_MAX_PHYS)
    dev_warn(priv.dev, "warning: more ports than PHYs (%#x)\n",
    impl);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !impl) -> else {
    else if (!impl)
    dev_info(priv.dev, "no ports found\n");
    return impl;
    }
#[no_mangle]
unsafe extern "C" fn brcm_sata_init(priv: *mut brcm_ahci_priv) {
    static void brcm_sata_init(struct brcm_ahci_priv *priv)
    {
    void __iomem *ctrl = priv.top_ctrl + SATA_TOP_CTRL_BUS_CTRL;
    u32 data;
// Configure endianness
    data = brcm_sata_readreg(ctrl);
    data &= ~BUS_CTRL_ENDIAN_CONF_MASK;
    if (priv.version == BRCM_SATA_NSP)
    data |= BUS_CTRL_ENDIAN_NSP_CONF;
    else
    data |= BUS_CTRL_ENDIAN_CONF;
    brcm_sata_writereg(data, ctrl);
    }
    static unsigned int brcm_ahci_read_id(struct ata_device *dev,
    struct ata_taskfile *tf, __le16 *id)
    {
    struct ata_port *ap = dev.link.ap;
    struct ata_host *host = ap.host;
    struct ahci_host_priv *hpriv = host.private_data;
    struct brcm_ahci_priv *priv = hpriv.plat_data;
    void __iomem *mmio = hpriv.mmio;
    unsigned int err_mask;
    unsigned long flags;
    int i, rc;
    u32 ctl;
// Try to read the device ID and, if this fails, proceed with the
// recovery sequence below
//
    err_mask = ata_do_dev_read_id(dev, tf, id);
    if (likely(!err_mask))
    return err_mask;
// Disable host interrupts
    spin_lock_irqsave(&host.lock, flags);
    ctl = readl(mmio + HOST_CTL);
    ctl &= ~HOST_IRQ_EN;
    writel(ctl, mmio + HOST_CTL);
    readl(mmio + HOST_CTL); /* flush */
    spin_unlock_irqrestore(&host.lock, flags);
// Perform the SATA PHY reset sequence
    brcm_sata_phy_disable(priv, ap.port_no);
// Reset the SATA clock
    ahci_platform_disable_clks(hpriv);
    msleep(10);
    ahci_platform_enable_clks(hpriv);
    msleep(10);
// Bring the PHY back on
    brcm_sata_phy_enable(priv, ap.port_no);
// Re-initialize and calibrate the PHY
    for (i = 0; i < hpriv.nports; i++) {
    if (ahci_ignore_port(hpriv, i))
    continue;
    rc = phy_init(hpriv.phys[i]);
    if (rc)
    goto disable_phys;
    rc = phy_calibrate(hpriv.phys[i]);
    if (rc) {
    phy_exit(hpriv.phys[i]);
    goto disable_phys;
    }
    }
// Re-enable host interrupts
    spin_lock_irqsave(&host.lock, flags);
    ctl = readl(mmio + HOST_CTL);
    ctl |= HOST_IRQ_EN;
    writel(ctl, mmio + HOST_CTL);
    readl(mmio + HOST_CTL); /* flush */
    spin_unlock_irqrestore(&host.lock, flags);
    return ata_do_dev_read_id(dev, tf, id);
    disable_phys:
    while (--i >= 0) {
    phy_power_off(hpriv.phys[i]);
    phy_exit(hpriv.phys[i]);
    }
    return AC_ERR_OTHER;
    }
#[no_mangle]
unsafe extern "C" fn brcm_ahci_host_stop(host: *mut ata_host) {
    static void brcm_ahci_host_stop(struct ata_host *host)
    {
    struct ahci_host_priv *hpriv = host.private_data;
    ahci_platform_disable_resources(hpriv);
    }
    static struct ata_port_operations ahci_brcm_platform_ops = {
    .inherits	= &ahci_ops,
    .host_stop	= brcm_ahci_host_stop,
    .read_id	= brcm_ahci_read_id,
    };
    static const struct ata_port_info ahci_brcm_port_info = {
    .flags		= AHCI_FLAG_COMMON | ATA_FLAG_NO_DIPM,
    .link_flags	= ATA_LFLAG_NO_DEBOUNCE_DELAY,
    .pio_mask	= ATA_PIO4,
    .udma_mask	= ATA_UDMA6,
    .port_ops	= &ahci_brcm_platform_ops,
    };
#[no_mangle]
unsafe extern "C" fn brcm_ahci_suspend(dev: *mut device) -> c_int {
    static int brcm_ahci_suspend(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    struct brcm_ahci_priv *priv = hpriv.plat_data;
    int ret;
    brcm_sata_phys_disable(priv);
    if (IS_ENABLED(CONFIG_PM_SLEEP))
    ret = ahci_platform_suspend(dev);
    else
    ret = 0;
    reset_control_assert(priv.rcdev_ahci);
    reset_control_rearm(priv.rcdev_rescal);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn brcm_ahci_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused brcm_ahci_resume(struct device *dev)
    {
    struct ata_host *host = dev_get_drvdata(dev);
    struct ahci_host_priv *hpriv = host.private_data;
    struct brcm_ahci_priv *priv = hpriv.plat_data;
    let mut ret: c_int = 0;
    ret = reset_control_deassert(priv.rcdev_ahci);
    if (ret)
    return ret;
    ret = reset_control_reset(priv.rcdev_rescal);
    if (ret)
    return ret;
// Make sure clocks are turned on before re-configuration
    ret = ahci_platform_enable_clks(hpriv);
    if (ret)
    return ret;
    ret = ahci_platform_enable_regulators(hpriv);
    if (ret)
    goto out_disable_clks;
    brcm_sata_init(priv);
    brcm_sata_phys_enable(priv);
    brcm_sata_alpm_init(hpriv);
// Since we had to enable clocks earlier on, we cannot use
// ahci_platform_resume() as-is since a second call to
// ahci_platform_enable_resources() would bump up the resources
// (regulators, clocks, PHYs) count artificially so we copy the part
// after ahci_platform_enable_resources().
//
    ret = ahci_platform_enable_phys(hpriv);
    if (ret)
    goto out_disable_phys;
    ret = ahci_platform_resume_host(dev);
    if (ret)
    goto out_disable_platform_phys;
// We resumed so update PM runtime state
    pm_runtime_disable(dev);
    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);
    return 0;
    out_disable_platform_phys:
    ahci_platform_disable_phys(hpriv);
    out_disable_phys:
    brcm_sata_phys_disable(priv);
    ahci_platform_disable_regulators(hpriv);
    out_disable_clks:
    ahci_platform_disable_clks(hpriv);
    return ret;
    }
    static const struct scsi_host_template ahci_platform_sht = {
    AHCI_SHT(DRV_NAME),
    };
    static const struct of_device_id ahci_of_match[] = {
    {.compatible = "brcm,bcm7425-ahci", .data = (void *)BRCM_SATA_BCM7425},
    {.compatible = "brcm,bcm7445-ahci", .data = (void *)BRCM_SATA_BCM7445},
    {.compatible = "brcm,bcm63138-ahci", .data = (void *)BRCM_SATA_BCM7445},
    {.compatible = "brcm,bcm-nsp-ahci", .data = (void *)BRCM_SATA_NSP},
    {.compatible = "brcm,bcm7216-ahci", .data = (void *)BRCM_SATA_BCM7216},
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, ahci_of_match);
#[no_mangle]
unsafe extern "C" fn brcm_ahci_probe(pdev: *mut platform_device) -> c_int {
    static int brcm_ahci_probe(struct platform_device *pdev)
    {
    const struct of_device_id *of_id;
    struct device *dev = &pdev.dev;
    struct brcm_ahci_priv *priv;
    struct ahci_host_priv *hpriv;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    of_id = of_match_node(ahci_of_match, pdev.dev.of_node);
    if (!of_id)
    return -ENODEV;
    priv.version = (unsigned long)of_id.data;
    priv.dev = dev;
    priv.top_ctrl = devm_platform_ioremap_resource_byname(pdev, "top-ctrl");
    if (IS_ERR(priv.top_ctrl))
    return PTR_ERR(priv.top_ctrl);
    if (priv.version == BRCM_SATA_BCM7216) {
    priv.rcdev_rescal = devm_reset_control_get_optional_shared(
    &pdev.dev, "rescal");
    if (IS_ERR(priv.rcdev_rescal))
    return PTR_ERR(priv.rcdev_rescal);
    }
    priv.rcdev_ahci = devm_reset_control_get_optional(&pdev.dev, "ahci");
    if (IS_ERR(priv.rcdev_ahci))
    return PTR_ERR(priv.rcdev_ahci);
    hpriv = ahci_platform_get_resources(pdev, 0);
    if (IS_ERR(hpriv))
    return PTR_ERR(hpriv);
    hpriv.plat_data = priv;
    hpriv.flags = AHCI_HFLAG_WAKE_BEFORE_STOP | AHCI_HFLAG_NO_WRITE_TO_RO;
    switch (priv.version) {
    case BRCM_SATA_BCM7425:
    hpriv.flags |= AHCI_HFLAG_DELAY_ENGINE;
    fallthrough;
    case BRCM_SATA_NSP:
    hpriv.flags |= AHCI_HFLAG_NO_NCQ;
    priv.quirks |= BRCM_AHCI_QUIRK_SKIP_PHY_ENABLE;
    break;
    default:
    break;
    }
    ret = reset_control_reset(priv.rcdev_rescal);
    if (ret)
    return ret;
    ret = reset_control_deassert(priv.rcdev_ahci);
    if (ret)
    return ret;
    ret = ahci_platform_enable_clks(hpriv);
    if (ret)
    goto out_reset;
    ret = ahci_platform_enable_regulators(hpriv);
    if (ret)
    goto out_disable_clks;
// Must be first so as to configure endianness including that
// of the standard AHCI register space.
//
    brcm_sata_init(priv);
// Initializes priv->port_mask which is used below
    priv.port_mask = brcm_ahci_get_portmask(hpriv, priv);
    if (!priv.port_mask) {
    ret = -ENODEV;
    goto out_disable_regulators;
    }
// Must be done before ahci_platform_enable_phys()
    brcm_sata_phys_enable(priv);
    brcm_sata_alpm_init(hpriv);
    ret = ahci_platform_enable_phys(hpriv);
    if (ret)
    goto out_disable_phys;
    ret = ahci_platform_init_host(pdev, hpriv, &ahci_brcm_port_info,
    &ahci_platform_sht);
    if (ret)
    goto out_disable_platform_phys;
    dev_info(dev, "Broadcom AHCI SATA3 registered\n");
    return 0;
    out_disable_platform_phys:
    ahci_platform_disable_phys(hpriv);
    out_disable_phys:
    brcm_sata_phys_disable(priv);
    out_disable_regulators:
    ahci_platform_disable_regulators(hpriv);
    out_disable_clks:
    ahci_platform_disable_clks(hpriv);
    out_reset:
    reset_control_assert(priv.rcdev_ahci);
    reset_control_rearm(priv.rcdev_rescal);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn brcm_ahci_remove(pdev: *mut platform_device) {
    static void brcm_ahci_remove(struct platform_device *pdev)
    {
    struct ata_host *host = dev_get_drvdata(&pdev.dev);
    struct ahci_host_priv *hpriv = host.private_data;
    struct brcm_ahci_priv *priv = hpriv.plat_data;
    brcm_sata_phys_disable(priv);
    ata_platform_remove_one(pdev);
    }
#[no_mangle]
unsafe extern "C" fn brcm_ahci_shutdown(pdev: *mut platform_device) {
    static void brcm_ahci_shutdown(struct platform_device *pdev)
    {
    int ret;
// All resources releasing happens via devres, but our device, unlike a
// proper remove is not disappearing, therefore using
// brcm_ahci_suspend() here which does explicit power management is
// appropriate.
//
    ret = brcm_ahci_suspend(&pdev.dev);
    if (ret)
    dev_err(&pdev.dev, "failed to shutdown\n");
    }
    static SIMPLE_DEV_PM_OPS(ahci_brcm_pm_ops, brcm_ahci_suspend, brcm_ahci_resume);
    static struct platform_driver brcm_ahci_driver = {
    .probe = brcm_ahci_probe,
    .remove = brcm_ahci_remove,
    .shutdown = brcm_ahci_shutdown,
    .driver = {
    .name = DRV_NAME,
    .of_match_table = ahci_of_match,
    .pm = &ahci_brcm_pm_ops,
    },
    };
    module_platform_driver(brcm_ahci_driver);
    MODULE_DESCRIPTION("Broadcom SATA3 AHCI Controller Driver");
    MODULE_AUTHOR("Brian Norris");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:sata-brcmstb");
