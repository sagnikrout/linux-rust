//! Automatically rewritten from C to Rust
//! Source: drivers/phy/rockchip/phy-rockchip-emmc.c
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
// Rockchip emmc PHY driver
//
// Copyright (C) 2016 Shawn Lin <shawn.lin@rock-chips.com>
// Copyright (C) 2016 ROCKCHIP, Inc.
//

//
// The higher 16-bit of this register is used for write protection
// only if BIT(x + 16) set to 1 the BIT(x) can be written.
//

    (FIELD_PREP_WM16((mask) << (shift), (val)))
// Register definition
pub const GRF_EMMCPHY_CON0: c_uint = 0x0;
pub const GRF_EMMCPHY_CON1: c_uint = 0x4;
pub const GRF_EMMCPHY_CON2: c_uint = 0x8;
pub const GRF_EMMCPHY_CON3: c_uint = 0xc;
pub const GRF_EMMCPHY_CON4: c_uint = 0x10;
pub const GRF_EMMCPHY_CON5: c_uint = 0x14;
pub const GRF_EMMCPHY_CON6: c_uint = 0x18;
pub const GRF_EMMCPHY_STATUS: c_uint = 0x20;
pub const PHYCTRL_PDB_MASK: c_uint = 0x1;
pub const PHYCTRL_PDB_SHIFT: c_uint = 0x0;
pub const PHYCTRL_PDB_PWR_ON: c_uint = 0x1;
pub const PHYCTRL_PDB_PWR_OFF: c_uint = 0x0;
pub const PHYCTRL_ENDLL_MASK: c_uint = 0x1;
pub const PHYCTRL_ENDLL_SHIFT: c_uint = 0x1;
pub const PHYCTRL_ENDLL_ENABLE: c_uint = 0x1;
pub const PHYCTRL_ENDLL_DISABLE: c_uint = 0x0;
pub const PHYCTRL_CALDONE_MASK: c_uint = 0x1;
pub const PHYCTRL_CALDONE_SHIFT: c_uint = 0x6;
pub const PHYCTRL_CALDONE_DONE: c_uint = 0x1;
pub const PHYCTRL_CALDONE_GOING: c_uint = 0x0;
pub const PHYCTRL_DLLRDY_MASK: c_uint = 0x1;
pub const PHYCTRL_DLLRDY_SHIFT: c_uint = 0x5;
pub const PHYCTRL_DLLRDY_DONE: c_uint = 0x1;
pub const PHYCTRL_DLLRDY_GOING: c_uint = 0x0;
pub const PHYCTRL_FREQSEL_200M: c_uint = 0x0;
pub const PHYCTRL_FREQSEL_50M: c_uint = 0x1;
pub const PHYCTRL_FREQSEL_100M: c_uint = 0x2;
pub const PHYCTRL_FREQSEL_150M: c_uint = 0x3;
pub const PHYCTRL_FREQSEL_MASK: c_uint = 0x3;
pub const PHYCTRL_FREQSEL_SHIFT: c_uint = 0xc;
pub const PHYCTRL_DR_MASK: c_uint = 0x7;
pub const PHYCTRL_DR_SHIFT: c_uint = 0x4;
pub const PHYCTRL_DR_50OHM: c_uint = 0x0;
pub const PHYCTRL_DR_33OHM: c_uint = 0x1;
pub const PHYCTRL_DR_66OHM: c_uint = 0x2;
pub const PHYCTRL_DR_100OHM: c_uint = 0x3;
pub const PHYCTRL_DR_40OHM: c_uint = 0x4;
pub const PHYCTRL_OTAPDLYENA: c_uint = 0x1;
pub const PHYCTRL_OTAPDLYENA_MASK: c_uint = 0x1;
pub const PHYCTRL_OTAPDLYENA_SHIFT: c_uint = 0xb;
pub const PHYCTRL_OTAPDLYSEL_DEFAULT: c_uint = 0x4;
pub const PHYCTRL_OTAPDLYSEL_MAXVALUE: c_uint = 0xf;
pub const PHYCTRL_OTAPDLYSEL_MASK: c_uint = 0xf;
pub const PHYCTRL_OTAPDLYSEL_SHIFT: c_uint = 0x7;
pub const PHYCTRL_REN_STRB_DISABLE: c_uint = 0x0;
pub const PHYCTRL_REN_STRB_ENABLE: c_uint = 0x1;
pub const PHYCTRL_REN_STRB_MASK: c_uint = 0x1;
pub const PHYCTRL_REN_STRB_SHIFT: c_uint = 0x9;

    ((((x) >> PHYCTRL_CALDONE_SHIFT) & \
    PHYCTRL_CALDONE_MASK) == PHYCTRL_CALDONE_DONE)

    ((((x) >> PHYCTRL_DLLRDY_SHIFT) & \
    PHYCTRL_DLLRDY_MASK) == PHYCTRL_DLLRDY_DONE)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rockchip_emmc_phy {
    pub reg_offset: c_uint,
    pub reg_base: *mut regmap,
    pub emmcclk: *mut clk,
    pub drive_impedance: c_uint,
    pub enable_strobe_pulldown: c_uint,
    pub output_tapdelay_select: c_uint,
}

#[no_mangle]
unsafe extern "C" fn rockchip_emmc_phy_power(phy: *mut phy, on_off: bool) -> c_int {
    static int rockchip_emmc_phy_power(struct phy *phy, bool on_off)
    {
    struct rockchip_emmc_phy *rk_phy = phy_get_drvdata(phy);
    unsigned int caldone;
    unsigned int dllrdy;
    let mut freqsel: c_uint = PHYCTRL_FREQSEL_200M;
    unsigned long rate;
    int ret;
//
// Keep phyctrl_pdb and phyctrl_endll low to allow
// initialization of CALIO state M/C DFFs
//
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON6,
    HIWORD_UPDATE(PHYCTRL_PDB_PWR_OFF,
    PHYCTRL_PDB_MASK,
    PHYCTRL_PDB_SHIFT));
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON6,
    HIWORD_UPDATE(PHYCTRL_ENDLL_DISABLE,
    PHYCTRL_ENDLL_MASK,
    PHYCTRL_ENDLL_SHIFT));
// Already finish power_off above
    if (on_off == PHYCTRL_PDB_PWR_OFF)
    return 0;
    rate = clk_get_rate(rk_phy.emmcclk);
    if (rate != 0) {
    unsigned long ideal_rate;
    unsigned long diff;
    switch (rate) {
    case 1 ... 74999999:
    ideal_rate = 50000000;
    freqsel = PHYCTRL_FREQSEL_50M;
    break;
    case 75000000 ... 124999999:
    ideal_rate = 100000000;
    freqsel = PHYCTRL_FREQSEL_100M;
    break;
    case 125000000 ... 174999999:
    ideal_rate = 150000000;
    freqsel = PHYCTRL_FREQSEL_150M;
    break;
    default:
    ideal_rate = 200000000;
    break;
    }
    diff = (rate > ideal_rate) ?
    rate - ideal_rate : ideal_rate - rate;
//
// In order for tuning delays to be accurate we need to be
// pretty spot on for the DLL range, so warn if we're too
// far off.  Also warn if we're above the 200 MHz max.  Don't
// warn for really slow rates since we won't be tuning then.
//
    if ((rate > 50000000 && diff > 15000000) || (rate > 200000000))
    dev_warn(&phy.dev, "Unsupported rate: %lu\n", rate);
    }
//
// According to the user manual, calpad calibration
// cycle takes more than 2us without the minimal recommended
// value, so we may need a little margin here
//
    udelay(3);
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON6,
    HIWORD_UPDATE(PHYCTRL_PDB_PWR_ON,
    PHYCTRL_PDB_MASK,
    PHYCTRL_PDB_SHIFT));
//
// According to the user manual, it asks driver to wait 5us for
// calpad busy trimming. However it is documented that this value is
// PVT(A.K.A process,voltage and temperature) relevant, so some
// failure cases are found which indicates we should be more tolerant
// to calpad busy trimming.
//
    ret = regmap_read_poll_timeout(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_STATUS,
    caldone, PHYCTRL_IS_CALDONE(caldone),
    0, 50);
    if (ret) {
    pr_err("%s: caldone failed, ret=%d\n", __func__, ret);
    return ret;
    }
// Set the frequency of the DLL operation
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON0,
    HIWORD_UPDATE(freqsel, PHYCTRL_FREQSEL_MASK,
    PHYCTRL_FREQSEL_SHIFT));
// Turn on the DLL
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON6,
    HIWORD_UPDATE(PHYCTRL_ENDLL_ENABLE,
    PHYCTRL_ENDLL_MASK,
    PHYCTRL_ENDLL_SHIFT));
//
// We turned on the DLL even though the rate was 0 because we the
// clock might be turned on later.  ...but we can't wait for the DLL
// to lock when the rate is 0 because it will never lock with no
// input clock.
//
// Technically we should be checking the lock later when the clock
// is turned on, but for now we won't.
//
    if (rate == 0)
    return 0;
//
// After enabling analog DLL circuits docs say that we need 10.2 us if
// our source clock is at 50 MHz and that lock time scales linearly
// with clock speed.  If we are powering on the PHY and the card clock
// is super slow (like 100 kHZ) this could take as long as 5.1 ms as
// per the math: 10.2 us * (50000000 Hz / 100000 Hz) => 5.1 ms
// Hopefully we won't be running at 100 kHz, but we should still make
// sure we wait long enough.
//
// NOTE: There appear to be corner cases where the DLL seems to take
// extra long to lock for reasons that aren't understood.  In some
// extreme cases we've seen it take up to over 10ms (!).  We'll be
// generous and give it 50ms.
//
    ret = regmap_read_poll_timeout(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_STATUS,
    dllrdy, PHYCTRL_IS_DLLRDY(dllrdy),
    0, 50 * USEC_PER_MSEC);
    if (ret) {
    pr_err("%s: dllrdy failed. ret=%d\n", __func__, ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_emmc_phy_init(phy: *mut phy) -> c_int {
    static int rockchip_emmc_phy_init(struct phy *phy)
    {
    struct rockchip_emmc_phy *rk_phy = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
//
// We purposely get the clock here and not in probe to avoid the
// circular dependency problem.  We expect:
// - PHY driver to probe
// - SDHCI driver to start probe
// - SDHCI driver to register it's clock
// - SDHCI driver to get the PHY
// - SDHCI driver to init the PHY
//
// The clock is optional, using clk_get_optional() to get the clock
// and do error processing if the return value != NULL
//
// NOTE: we don't do anything special for EPROBE_DEFER here.  Given the
// above expected use case, EPROBE_DEFER isn't sensible to expect, so
// it's just like any other error.
//
    rk_phy.emmcclk = clk_get_optional(&phy.dev, "emmcclk");
    if (IS_ERR(rk_phy.emmcclk)) {
    ret = PTR_ERR(rk_phy.emmcclk);
    dev_err(&phy.dev, "Error getting emmcclk: %d\n", ret);
    rk_phy.emmcclk = core::ptr::null_mut();
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_emmc_phy_exit(phy: *mut phy) -> c_int {
    static int rockchip_emmc_phy_exit(struct phy *phy)
    {
    struct rockchip_emmc_phy *rk_phy = phy_get_drvdata(phy);
    clk_put(rk_phy.emmcclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_emmc_phy_power_off(phy: *mut phy) -> c_int {
    static int rockchip_emmc_phy_power_off(struct phy *phy)
    {
// Power down emmc phy analog blocks
    return rockchip_emmc_phy_power(phy, PHYCTRL_PDB_PWR_OFF);
    }
#[no_mangle]
unsafe extern "C" fn rockchip_emmc_phy_power_on(phy: *mut phy) -> c_int {
    static int rockchip_emmc_phy_power_on(struct phy *phy)
    {
    struct rockchip_emmc_phy *rk_phy = phy_get_drvdata(phy);
// Drive impedance: from DTS
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON6,
    HIWORD_UPDATE(rk_phy.drive_impedance,
    PHYCTRL_DR_MASK,
    PHYCTRL_DR_SHIFT));
// Output tap delay: enable
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON0,
    HIWORD_UPDATE(PHYCTRL_OTAPDLYENA,
    PHYCTRL_OTAPDLYENA_MASK,
    PHYCTRL_OTAPDLYENA_SHIFT));
// Output tap delay
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON0,
    HIWORD_UPDATE(rk_phy.output_tapdelay_select,
    PHYCTRL_OTAPDLYSEL_MASK,
    PHYCTRL_OTAPDLYSEL_SHIFT));
// Internal pull-down for strobe line
    regmap_write(rk_phy.reg_base,
    rk_phy.reg_offset + GRF_EMMCPHY_CON2,
    HIWORD_UPDATE(rk_phy.enable_strobe_pulldown,
    PHYCTRL_REN_STRB_MASK,
    PHYCTRL_REN_STRB_SHIFT));
// Power up emmc phy analog blocks
    return rockchip_emmc_phy_power(phy, PHYCTRL_PDB_PWR_ON);
    }
    static const struct phy_ops ops = {
    .init		= rockchip_emmc_phy_init,
    .exit		= rockchip_emmc_phy_exit,
    .power_on	= rockchip_emmc_phy_power_on,
    .power_off	= rockchip_emmc_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn convert_drive_impedance_ohm(pdev: *mut platform_device, dr_ohm: u32) -> u32 {
    static u32 convert_drive_impedance_ohm(struct platform_device *pdev, u32 dr_ohm)
    {
    switch (dr_ohm) {
    case 100:
    return PHYCTRL_DR_100OHM;
    case 66:
    return PHYCTRL_DR_66OHM;
    case 50:
    return PHYCTRL_DR_50OHM;
    case 40:
    return PHYCTRL_DR_40OHM;
    case 33:
    return PHYCTRL_DR_33OHM;
    }
    dev_warn(&pdev.dev, "Invalid value %u for drive-impedance-ohm.\n",
    dr_ohm);
    return PHYCTRL_DR_50OHM;
    }
#[no_mangle]
unsafe extern "C" fn rockchip_emmc_phy_probe(pdev: *mut platform_device) -> c_int {
    static int rockchip_emmc_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rockchip_emmc_phy *rk_phy;
    struct phy *generic_phy;
    struct phy_provider *phy_provider;
    struct regmap *grf;
    unsigned int reg_offset;
    u32 val;
    if (!dev.parent || !dev.parent.of_node)
    return -ENODEV;
    grf = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(grf)) {
    dev_err(dev, "Missing rockchip,grf property\n");
    return PTR_ERR(grf);
    }
    rk_phy = devm_kzalloc(dev, sizeof(*rk_phy), GFP_KERNEL);
    if (!rk_phy)
    return -ENOMEM;
    if (of_property_read_u32(dev.of_node, "reg", &reg_offset)) {
    dev_err(dev, "missing reg property in node %pOFn\n",
    dev.of_node);
    return -EINVAL;
    }
    rk_phy.reg_offset = reg_offset;
    rk_phy.reg_base = grf;
    rk_phy.drive_impedance = PHYCTRL_DR_50OHM;
    rk_phy.enable_strobe_pulldown = PHYCTRL_REN_STRB_DISABLE;
    rk_phy.output_tapdelay_select = PHYCTRL_OTAPDLYSEL_DEFAULT;
    if (!of_property_read_u32(dev.of_node, "drive-impedance-ohm", &val))
    rk_phy.drive_impedance = convert_drive_impedance_ohm(pdev, val);
    if (of_property_read_bool(dev.of_node, "rockchip,enable-strobe-pulldown"))
    rk_phy.enable_strobe_pulldown = PHYCTRL_REN_STRB_ENABLE;
    if (!of_property_read_u32(dev.of_node, "rockchip,output-tapdelay-select", &val)) {
    if (val <= PHYCTRL_OTAPDLYSEL_MAXVALUE)
    rk_phy.output_tapdelay_select = val;
    else
    dev_err(dev, "output-tapdelay-select exceeds limit, apply default\n");
    }
    generic_phy = devm_phy_create(dev, dev.of_node, &ops);
    if (IS_ERR(generic_phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(generic_phy);
    }
    phy_set_drvdata(generic_phy, rk_phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id rockchip_emmc_phy_dt_ids[] = {
    { .compatible = "rockchip,rk3399-emmc-phy" },
    {}
    };
    MODULE_DEVICE_TABLE(of, rockchip_emmc_phy_dt_ids);
    static struct platform_driver rockchip_emmc_driver = {
    .probe		= rockchip_emmc_phy_probe,
    .driver		= {
    .name	= "rockchip-emmc-phy",
    .of_match_table = rockchip_emmc_phy_dt_ids,
    },
    };
    module_platform_driver(rockchip_emmc_driver);
    MODULE_AUTHOR("Shawn Lin <shawn.lin@rock-chips.com>");
    MODULE_DESCRIPTION("Rockchip EMMC PHY driver");
    MODULE_LICENSE("GPL v2");
