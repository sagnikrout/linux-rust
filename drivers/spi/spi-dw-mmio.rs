//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-dw-mmio.c
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
// Memory-mapped interface driver for DW SPI Core
//
// Copyright (c) 2010, Octasic semiconductor.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_spi_mmio {
    pub dws: dw_spi,
    pub clk: *mut clk,
    pub pclk: *mut clk,
    pub priv: *mut c_void,
    pub rstc: *mut reset_control,
    pub dwsmmio): *mut *mut void (platform_suspend)(struct dw_spi_mmio,
    pub dwsmmio): *mut *mut int (platform_resume)(struct dw_spi_mmio,
}

pub const MSCC_CPU_SYSTEM_CTRL_GENERAL_CTRL: c_uint = 0x24;
pub const OCELOT_IF_SI_OWNER_OFFSET: c_int = 4;
pub const JAGUAR2_IF_SI_OWNER_OFFSET: c_int = 6;

pub const MSCC_IF_SI_OWNER_SISL: c_int = 0;
pub const MSCC_IF_SI_OWNER_SIBM: c_int = 1;
pub const MSCC_IF_SI_OWNER_SIMC: c_int = 2;
pub const MSCC_SPI_MST_SW_MODE: c_uint = 0x14;

pub const SPARX5_FORCE_ENA: c_uint = 0xa4;
pub const SPARX5_FORCE_VAL: c_uint = 0xa8;
pub const JHB100_ADDRMODE_CS: c_uint = 0x00;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_spi_mscc {
    pub syscon: *mut regmap,
    pub /: *mut *mut *mut void __iomem spi_mst; / Not sparx5,
}

//
// Elba SoC does not use ssi, pin override is used for cs 0,1 and
// gpios for cs 2,3 as defined in the device tree.
//
// cs:  |       1               0
// bit: |---3-------2-------1-------0
// |  cs1   cs1_ovr   cs0   cs0_ovr
//
pub const ELBA_SPICS_REG: c_uint = 0x2468;

    ((((val) << 1) | BIT(0)) << ELBA_SPICS_OFFSET(cs))
//
// The Designware SPI controller (referred to as master in the documentation)
// automatically deasserts chip select when the tx fifo is empty. The chip
// selects then needs to be either driven as GPIOs or, for the first 4 using
// the SPI boot controller registers. the final chip select is an OR gate
// between the Designware SPI controller and the SPI boot controller.
//
#[no_mangle]
unsafe extern "C" fn dw_spi_mscc_set_cs(spi: *mut spi_device, enable: bool) {
    static void dw_spi_mscc_set_cs(struct spi_device *spi, bool enable)
    {
    struct dw_spi *dws = spi_controller_get_devdata(spi.controller);
    struct dw_spi_mmio *dwsmmio = container_of(dws, struct dw_spi_mmio, dws);
    struct dw_spi_mscc *dwsmscc = dwsmmio.priv;
    let mut cs: u32 = spi_get_chipselect(spi, 0);
    if (cs < 4) {
    let mut sw_mode: u32 = MSCC_SPI_MST_SW_MODE_SW_PIN_CTRL_MODE;
    if (!enable)
    sw_mode |= MSCC_SPI_MST_SW_MODE_SW_SPI_CS(BIT(cs));
    writel(sw_mode, dwsmscc.spi_mst + MSCC_SPI_MST_SW_MODE);
    }
    dw_spi_set_cs(spi, enable);
    }
    static int dw_spi_mscc_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio,
    const char *cpu_syscon, u32 if_si_owner_offset)
    {
    struct dw_spi_mscc *dwsmscc;
    dwsmscc = devm_kzalloc(&pdev.dev, sizeof(*dwsmscc), GFP_KERNEL);
    if (!dwsmscc)
    return -ENOMEM;
    dwsmscc.spi_mst = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(dwsmscc.spi_mst))
    return PTR_ERR(dwsmscc.spi_mst);
    dwsmscc.syscon = syscon_regmap_lookup_by_compatible(cpu_syscon);
    if (IS_ERR(dwsmscc.syscon))
    return PTR_ERR(dwsmscc.syscon);
// Deassert all CS
    writel(0, dwsmscc.spi_mst + MSCC_SPI_MST_SW_MODE);
// Select the owner of the SI interface
    regmap_update_bits(dwsmscc.syscon, MSCC_CPU_SYSTEM_CTRL_GENERAL_CTRL,
    MSCC_IF_SI_OWNER_MASK << if_si_owner_offset,
    MSCC_IF_SI_OWNER_SIMC << if_si_owner_offset);
    dwsmmio.dws.set_cs = dw_spi_mscc_set_cs;
    dwsmmio.priv = dwsmscc;
    return 0;
    }
    static int dw_spi_mscc_ocelot_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    return dw_spi_mscc_init(pdev, dwsmmio, "mscc,ocelot-cpu-syscon",
    OCELOT_IF_SI_OWNER_OFFSET);
    }
    static int dw_spi_mscc_jaguar2_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    return dw_spi_mscc_init(pdev, dwsmmio, "mscc,jaguar2-cpu-syscon",
    JAGUAR2_IF_SI_OWNER_OFFSET);
    }
//
// The Designware SPI controller (referred to as master in the
// documentation) automatically deasserts chip select when the tx fifo
// is empty. The chip selects then needs to be driven by a CS override
// register. enable is an active low signal.
//
#[no_mangle]
unsafe extern "C" fn dw_spi_sparx5_set_cs(spi: *mut spi_device, enable: bool) {
    static void dw_spi_sparx5_set_cs(struct spi_device *spi, bool enable)
    {
    struct dw_spi *dws = spi_controller_get_devdata(spi.controller);
    struct dw_spi_mmio *dwsmmio = container_of(dws, struct dw_spi_mmio, dws);
    struct dw_spi_mscc *dwsmscc = dwsmmio.priv;
    let mut cs: u8 = spi_get_chipselect(spi, 0);
    if (!enable) {
// CS override drive enable
    regmap_write(dwsmscc.syscon, SPARX5_FORCE_ENA, 1);
// Now set CSx enabled
    regmap_write(dwsmscc.syscon, SPARX5_FORCE_VAL, ~BIT(cs));
// Allow settle
    usleep_range(1, 5);
    } else {
// CS value
    regmap_write(dwsmscc.syscon, SPARX5_FORCE_VAL, ~0);
// Allow settle
    usleep_range(1, 5);
// CS override drive disable
    regmap_write(dwsmscc.syscon, SPARX5_FORCE_ENA, 0);
    }
    dw_spi_set_cs(spi, enable);
    }
    static int dw_spi_mscc_sparx5_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    const char *syscon_name = "microchip,sparx5-cpu-syscon";
    struct device *dev = &pdev.dev;
    struct dw_spi_mscc *dwsmscc;
    if (!IS_ENABLED(CONFIG_SPI_MUX)) {
    dev_err(dev, "This driver needs CONFIG_SPI_MUX\n");
    return -EOPNOTSUPP;
    }
    dwsmscc = devm_kzalloc(dev, sizeof(*dwsmscc), GFP_KERNEL);
    if (!dwsmscc)
    return -ENOMEM;
    dwsmscc.syscon =
    syscon_regmap_lookup_by_compatible(syscon_name);
    if (IS_ERR(dwsmscc.syscon)) {
    dev_err(dev, "No syscon map %s\n", syscon_name);
    return PTR_ERR(dwsmscc.syscon);
    }
    dwsmmio.dws.set_cs = dw_spi_sparx5_set_cs;
    dwsmmio.priv = dwsmscc;
    return 0;
    }
    static int dw_spi_alpine_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    dwsmmio.dws.caps = DW_SPI_CAP_CS_OVERRIDE;
    return 0;
    }
    static int dw_spi_pssi_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    dw_spi_dma_setup_generic(&dwsmmio.dws);
    return 0;
    }
    static int dw_spi_hssi_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    dwsmmio.dws.ip = DW_HSSI_ID;
    dw_spi_dma_setup_generic(&dwsmmio.dws);
    return 0;
    }
    static int dw_spi_hssi_no_dma_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    dwsmmio.dws.ip = DW_HSSI_ID;
    return 0;
    }
//
// DMA-based mem ops are not configured for this device and are not tested.
//
    static int dw_spi_mountevans_imc_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
//
// The Intel Mount Evans SoC's Integrated Management Complex DW
// apb_ssi_v4.02a controller has an errata where a full TX FIFO can
// result in data corruption. The suggested workaround is to never
// completely fill the FIFO. The TX FIFO has a size of 32 so the
// fifo_len is set to 31.
//
    dwsmmio.dws.fifo_len = 31;
    return 0;
    }
    static int dw_spi_canaan_k210_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
//
// The Canaan Kendryte K210 SoC DW apb_ssi v4 spi controller is
// documented to have a 32 word deep TX and RX FIFO, which
// spi_hw_init() detects. However, when the RX FIFO is filled up to
// 32 entries (RXFLR = 32), an RX FIFO overrun error occurs. Avoid this
// problem by force setting fifo_len to 31.
//
    dwsmmio.dws.fifo_len = 31;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_elba_override_cs(syscon: *mut regmap, cs: c_int, enable: c_int) {
    static void dw_spi_elba_override_cs(struct regmap *syscon, int cs, int enable)
    {
    regmap_update_bits(syscon, ELBA_SPICS_REG, ELBA_SPICS_MASK(cs),
    ELBA_SPICS_SET(cs, enable));
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_elba_set_cs(spi: *mut spi_device, enable: bool) {
    static void dw_spi_elba_set_cs(struct spi_device *spi, bool enable)
    {
    struct dw_spi *dws = spi_controller_get_devdata(spi.controller);
    struct dw_spi_mmio *dwsmmio = container_of(dws, struct dw_spi_mmio, dws);
    struct regmap *syscon = dwsmmio.priv;
    u8 cs;
    cs = spi_get_chipselect(spi, 0);
    if (cs < 2)
    dw_spi_elba_override_cs(syscon, spi_get_chipselect(spi, 0), enable);
//
// The DW SPI controller needs a native CS bit selected to start
// the serial engine.
//
    spi_set_chipselect(spi, 0, 0);
    dw_spi_set_cs(spi, enable);
    spi_set_chipselect(spi, 0, cs);
    }
    static int dw_spi_elba_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    struct regmap *syscon;
    syscon = syscon_regmap_lookup_by_phandle(dev_of_node(&pdev.dev),
    "amd,pensando-elba-syscon");
    if (IS_ERR(syscon))
    return dev_err_probe(&pdev.dev, PTR_ERR(syscon),
    "syscon regmap lookup failed\n");
    dwsmmio.priv = syscon;
    dwsmmio.dws.set_cs = dw_spi_elba_set_cs;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_jhb100_set_addr_nbyte(spi: *mut spi_device, nbyte: u8) -> c_int {
    static int dw_spi_jhb100_set_addr_nbyte(struct spi_device *spi, u8 nbyte)
    {
    struct dw_spi *dws = spi_controller_get_devdata(spi.controller);
    struct dw_spi_mmio *dwsmmio = container_of(dws, struct dw_spi_mmio, dws);
    struct regmap *syscon = dwsmmio.priv;
    if (nbyte == 3) {
    regmap_update_bits(syscon, JHB100_ADDRMODE_CS,
    BIT(spi_get_chipselect(spi, 0)),
    0);
    } else if (nbyte == 4) {
    regmap_update_bits(syscon, JHB100_ADDRMODE_CS,
    BIT(spi_get_chipselect(spi, 0)),
    BIT(spi_get_chipselect(spi, 0)));
    } else {
    dev_err(&spi.dev, "Unsupported address nbyte %d\n", nbyte);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_jhb100_resume(dwsmmio: *mut dw_spi_mmio) -> c_int {
    static int dw_spi_jhb100_resume(struct dw_spi_mmio *dwsmmio)
    {
    dw_spi_jhb100_mask_intr(&dwsmmio.dws, 0xff);
    return 0;
    }
    static int dw_spi_jhb100_init(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio)
    {
    struct regmap *syscon;
    syscon = syscon_regmap_lookup_by_phandle(dev_of_node(&pdev.dev),
    "starfive,sfc-filter-syscon");
    if (IS_ERR(syscon))
    return dev_err_probe(&pdev.dev, PTR_ERR(syscon),
    "syscon regmap lookup failed\n");
    dwsmmio.priv = syscon;
    dwsmmio.platform_resume = dw_spi_jhb100_resume;
    dwsmmio.dws.set_addr_nbyte = dw_spi_jhb100_set_addr_nbyte;
    dwsmmio.dws.ip = DW_HSSI_ID;
    dwsmmio.dws.quirk_flags = DW_SPI_QUIRK_JHB100;
    dw_spi_jhb100_mask_intr(&dwsmmio.dws, 0xff);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_mmio_probe(pdev: *mut platform_device) -> c_int {
    static int dw_spi_mmio_probe(struct platform_device *pdev)
    {
    int (*init_func)(struct platform_device *pdev,
    struct dw_spi_mmio *dwsmmio);
    struct dw_spi_mmio *dwsmmio;
    struct resource *mem;
    struct dw_spi *dws;
    int ret;
    dwsmmio = devm_kzalloc(&pdev.dev, sizeof(struct dw_spi_mmio),
    GFP_KERNEL);
    if (!dwsmmio)
    return -ENOMEM;
    dws = &dwsmmio.dws;
// Get basic io resource and map it
    dws.regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mem);
    if (IS_ERR(dws.regs))
    return PTR_ERR(dws.regs);
    dws.paddr = mem.start;
    dws.irq = platform_get_irq(pdev, 0);
    if (dws.irq < 0)
    return dws.irq; /* -ENXIO */
    dwsmmio.clk = devm_clk_get_enabled(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(dwsmmio.clk))
    return PTR_ERR(dwsmmio.clk);
// Optional clock needed to access the registers
    dwsmmio.pclk = devm_clk_get_optional_enabled(&pdev.dev, "pclk");
    if (IS_ERR(dwsmmio.pclk))
    return PTR_ERR(dwsmmio.pclk);
// find an optional reset controller
    dwsmmio.rstc = devm_reset_control_get_optional_exclusive(&pdev.dev, "spi");
    if (IS_ERR(dwsmmio.rstc))
    return PTR_ERR(dwsmmio.rstc);
    ret = reset_control_deassert(dwsmmio.rstc);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "Failed to deassert resets\n");
    dws.bus_num = pdev.id;
    dws.max_freq = clk_get_rate(dwsmmio.clk);
    if (device_property_read_u32(&pdev.dev, "reg-io-width",
    &dws.reg_io_width))
    dws.reg_io_width = 4;
// Rely on the auto-detection if no property specified
    device_property_read_u32(&pdev.dev, "num-cs", &dws.num_cs);
    init_func = device_get_match_data(&pdev.dev);
    if (init_func) {
    ret = init_func(pdev, dwsmmio);
    if (ret)
    goto out_reset;
    }
    pm_runtime_enable(&pdev.dev);
    ret = dw_spi_add_controller(&pdev.dev, dws);
    if (ret)
    goto out;
    platform_set_drvdata(pdev, dwsmmio);
    return 0;
    out:
    pm_runtime_disable(&pdev.dev);
    out_reset:
    reset_control_assert(dwsmmio.rstc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_mmio_suspend(dev: *mut device) -> c_int {
    static int dw_spi_mmio_suspend(struct device *dev)
    {
    struct dw_spi_mmio *dwsmmio = dev_get_drvdata(dev);
    int ret;
    ret = dw_spi_suspend_controller(&dwsmmio.dws);
    if (ret)
    return ret;
    if (dwsmmio.platform_suspend)
    dwsmmio.platform_suspend(dwsmmio);
    reset_control_assert(dwsmmio.rstc);
    clk_disable_unprepare(dwsmmio.pclk);
    clk_disable_unprepare(dwsmmio.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dw_spi_mmio_resume(dev: *mut device) -> c_int {
    static int dw_spi_mmio_resume(struct device *dev)
    {
    struct dw_spi_mmio *dwsmmio = dev_get_drvdata(dev);
    int ret;
    clk_prepare_enable(dwsmmio.clk);
    clk_prepare_enable(dwsmmio.pclk);
    reset_control_deassert(dwsmmio.rstc);
    if (dwsmmio.platform_resume) {
    ret = dwsmmio.platform_resume(dwsmmio);
    if (ret) {
    reset_control_assert(dwsmmio.rstc);
    clk_disable_unprepare(dwsmmio.pclk);
    clk_disable_unprepare(dwsmmio.clk);
    return ret;
    }
    }
    return dw_spi_resume_controller(&dwsmmio.dws);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(dw_spi_mmio_pm_ops,
    dw_spi_mmio_suspend, dw_spi_mmio_resume);
#[no_mangle]
unsafe extern "C" fn dw_spi_mmio_remove(pdev: *mut platform_device) {
    static void dw_spi_mmio_remove(struct platform_device *pdev)
    {
    struct dw_spi_mmio *dwsmmio = platform_get_drvdata(pdev);
    dw_spi_remove_controller(&dwsmmio.dws);
    pm_runtime_disable(&pdev.dev);
    reset_control_assert(dwsmmio.rstc);
    }
    static const struct of_device_id dw_spi_mmio_of_match[] = {
    { .compatible = "snps,dw-apb-ssi", .data = dw_spi_pssi_init},
    { .compatible = "mscc,ocelot-spi", .data = dw_spi_mscc_ocelot_init},
    { .compatible = "mscc,jaguar2-spi", .data = dw_spi_mscc_jaguar2_init},
    { .compatible = "amazon,alpine-dw-apb-ssi", .data = dw_spi_alpine_init},
    { .compatible = "renesas,rzn1-spi", .data = dw_spi_pssi_init},
    { .compatible = "snps,dwc-ssi-1.01a", .data = dw_spi_hssi_init},
    { .compatible = "snps,dwc-ssi-2.00a", .data = dw_spi_hssi_init},
    { .compatible = "intel,keembay-ssi", .data = dw_spi_hssi_no_dma_init},
    {
    .compatible = "intel,mountevans-imc-ssi",
    .data = dw_spi_mountevans_imc_init,
    },
    { .compatible = "microchip,sparx5-spi", dw_spi_mscc_sparx5_init},
    { .compatible = "canaan,k210-spi", dw_spi_canaan_k210_init},
    { .compatible = "amd,pensando-elba-spi", .data = dw_spi_elba_init},
    { .compatible = "starfive,jhb100-sfc", .data = dw_spi_jhb100_init},
    { /* end of table */}
    };
    MODULE_DEVICE_TABLE(of, dw_spi_mmio_of_match);

    static const struct acpi_device_id dw_spi_mmio_acpi_match[] = {
    {"HISI0173", (kernel_ulong_t)dw_spi_pssi_init},
    {"LECA0002", (kernel_ulong_t)dw_spi_hssi_no_dma_init},
    {},
    };
    MODULE_DEVICE_TABLE(acpi, dw_spi_mmio_acpi_match);

    static struct platform_driver dw_spi_mmio_driver = {
    .probe		= dw_spi_mmio_probe,
    .remove		= dw_spi_mmio_remove,
    .driver		= {
    .name	= DRIVER_NAME,
    .of_match_table = dw_spi_mmio_of_match,
    .acpi_match_table = ACPI_PTR(dw_spi_mmio_acpi_match),
    .pm	= pm_sleep_ptr(&dw_spi_mmio_pm_ops),
    },
    };
    module_platform_driver(dw_spi_mmio_driver);
    MODULE_AUTHOR("Jean-Hugues Deschenes <jean-hugues.deschenes@octasic.com>");
    MODULE_DESCRIPTION("Memory-mapped I/O interface driver for DW SPI Core");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("SPI_DW_CORE");
