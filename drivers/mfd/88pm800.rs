//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/88pm800.c
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


//
// Base driver for Marvell 88PM800
//
// Copyright (C) 2012 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
// Joseph(Yossi) Hanin <yhanin@marvell.com>
// Qiao Zhou <zhouqiao@marvell.com>
//
// This file is subject to the terms and conditions of the GNU General
// Public License. See the file "COPYING" in the main directory of this
// archive for more details.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
//

// Interrupt Registers

// number of INT_ENA & INT_STATUS regs

// Interrupt Number in 88PM800
    enum {
    PM800_IRQ_ONKEY,	/*EN1b0 *//*0 */
    PM800_IRQ_EXTON,	/*EN1b1 */
    PM800_IRQ_CHG,		/*EN1b2 */
    PM800_IRQ_BAT,		/*EN1b3 */
    PM800_IRQ_RTC,		/*EN1b4 */
    PM800_IRQ_CLASSD,	/*EN1b5 *//*5 */
    PM800_IRQ_VBAT,		/*EN2b0 */
    PM800_IRQ_VSYS,		/*EN2b1 */
    PM800_IRQ_VCHG,		/*EN2b2 */
    PM800_IRQ_TINT,		/*EN2b3 */
    PM800_IRQ_GPADC0,	/*EN3b0 *//*10 */
    PM800_IRQ_GPADC1,	/*EN3b1 */
    PM800_IRQ_GPADC2,	/*EN3b2 */
    PM800_IRQ_GPADC3,	/*EN3b3 */
    PM800_IRQ_GPADC4,	/*EN3b4 */
    PM800_IRQ_GPIO0,	/*EN4b0 *//*15 */
    PM800_IRQ_GPIO1,	/*EN4b1 */
    PM800_IRQ_GPIO2,	/*EN4b2 */
    PM800_IRQ_GPIO3,	/*EN4b3 */
    PM800_IRQ_GPIO4,	/*EN4b4 *//*19 */
    PM800_MAX_IRQ,
    };
// PM800: generation identification number
pub const PM800_CHIP_GEN_ID_NUM: c_uint = 0x3;
    static const struct i2c_device_id pm80x_id_table[] = {
    { "88PM800" },
    {} /* core::ptr::null_mut() terminated */
    };
    MODULE_DEVICE_TABLE(i2c, pm80x_id_table);
    static const struct resource rtc_resources[] = {
    DEFINE_RES_IRQ_NAMED(PM800_IRQ_RTC, "88pm80x-rtc"),
    };
    static struct mfd_cell rtc_devs[] = {
    {
    .name = "88pm80x-rtc",
    .num_resources = ARRAY_SIZE(rtc_resources),
    .resources = &rtc_resources[0],
    .id = -1,
    },
    };
    static struct resource onkey_resources[] = {
    DEFINE_RES_IRQ_NAMED(PM800_IRQ_ONKEY, "88pm80x-onkey"),
    };
    static const struct mfd_cell onkey_devs[] = {
    {
    .name = "88pm80x-onkey",
    .num_resources = 1,
    .resources = &onkey_resources[0],
    .id = -1,
    },
    };
    static const struct mfd_cell regulator_devs[] = {
    {
    .name = "88pm80x-regulator",
    .id = -1,
    },
    };
    static const struct regmap_irq pm800_irqs[] = {
// INT0
    [PM800_IRQ_ONKEY] = {
    .mask = PM800_ONKEY_INT_ENA1,
    },
    [PM800_IRQ_EXTON] = {
    .mask = PM800_EXTON_INT_ENA1,
    },
    [PM800_IRQ_CHG] = {
    .mask = PM800_CHG_INT_ENA1,
    },
    [PM800_IRQ_BAT] = {
    .mask = PM800_BAT_INT_ENA1,
    },
    [PM800_IRQ_RTC] = {
    .mask = PM800_RTC_INT_ENA1,
    },
    [PM800_IRQ_CLASSD] = {
    .mask = PM800_CLASSD_OC_INT_ENA1,
    },
// INT1
    [PM800_IRQ_VBAT] = {
    .reg_offset = 1,
    .mask = PM800_VBAT_INT_ENA2,
    },
    [PM800_IRQ_VSYS] = {
    .reg_offset = 1,
    .mask = PM800_VSYS_INT_ENA2,
    },
    [PM800_IRQ_VCHG] = {
    .reg_offset = 1,
    .mask = PM800_VCHG_INT_ENA2,
    },
    [PM800_IRQ_TINT] = {
    .reg_offset = 1,
    .mask = PM800_TINT_INT_ENA2,
    },
// INT2
    [PM800_IRQ_GPADC0] = {
    .reg_offset = 2,
    .mask = PM800_GPADC0_INT_ENA3,
    },
    [PM800_IRQ_GPADC1] = {
    .reg_offset = 2,
    .mask = PM800_GPADC1_INT_ENA3,
    },
    [PM800_IRQ_GPADC2] = {
    .reg_offset = 2,
    .mask = PM800_GPADC2_INT_ENA3,
    },
    [PM800_IRQ_GPADC3] = {
    .reg_offset = 2,
    .mask = PM800_GPADC3_INT_ENA3,
    },
    [PM800_IRQ_GPADC4] = {
    .reg_offset = 2,
    .mask = PM800_GPADC4_INT_ENA3,
    },
// INT3
    [PM800_IRQ_GPIO0] = {
    .reg_offset = 3,
    .mask = PM800_GPIO0_INT_ENA4,
    },
    [PM800_IRQ_GPIO1] = {
    .reg_offset = 3,
    .mask = PM800_GPIO1_INT_ENA4,
    },
    [PM800_IRQ_GPIO2] = {
    .reg_offset = 3,
    .mask = PM800_GPIO2_INT_ENA4,
    },
    [PM800_IRQ_GPIO3] = {
    .reg_offset = 3,
    .mask = PM800_GPIO3_INT_ENA4,
    },
    [PM800_IRQ_GPIO4] = {
    .reg_offset = 3,
    .mask = PM800_GPIO4_INT_ENA4,
    },
    };
    static int device_gpadc_init(struct pm80x_chip *chip,
    struct pm80x_platform_data *pdata)
    {
    struct pm80x_subchip *subchip = chip.subchip;
    struct regmap *map = subchip.regmap_gpadc;
    let mut data: c_int = 0, mask = 0, ret = 0;
    if (!map) {
    dev_warn(chip.dev,
    "Warning: gpadc regmap is not available!\n");
    return -EINVAL;
    }
//
// initialize GPADC without activating it turn on GPADC
// measurments
//
    ret = regmap_update_bits(map,
    PM800_GPADC_MISC_CONFIG2,
    PM800_GPADC_MISC_GPFSM_EN,
    PM800_GPADC_MISC_GPFSM_EN);
    if (ret < 0)
    goto out;
//
// This function configures the ADC as requires for
// CP implementation.CP does not "own" the ADC configuration
// registers and relies on AP.
// Reason: enable automatic ADC measurements needed
// for CP to get VBAT and RF temperature readings.
//
    ret = regmap_update_bits(map, PM800_GPADC_MEAS_EN1,
    PM800_MEAS_EN1_VBAT, PM800_MEAS_EN1_VBAT);
    if (ret < 0)
    goto out;
    ret = regmap_update_bits(map, PM800_GPADC_MEAS_EN2,
    (PM800_MEAS_EN2_RFTMP | PM800_MEAS_GP0_EN),
    (PM800_MEAS_EN2_RFTMP | PM800_MEAS_GP0_EN));
    if (ret < 0)
    goto out;
//
// the defult of PM800 is GPADC operates at 100Ks/s rate
// and Number of GPADC slots with active current bias prior
// to GPADC sampling = 1 slot for all GPADCs set for
// Temprature mesurmants
//
    mask = (PM800_GPADC_GP_BIAS_EN0 | PM800_GPADC_GP_BIAS_EN1 |
    PM800_GPADC_GP_BIAS_EN2 | PM800_GPADC_GP_BIAS_EN3);
    if (pdata && (pdata.batt_det == 0))
    data = (PM800_GPADC_GP_BIAS_EN0 | PM800_GPADC_GP_BIAS_EN1 |
    PM800_GPADC_GP_BIAS_EN2 | PM800_GPADC_GP_BIAS_EN3);
    else
    data = (PM800_GPADC_GP_BIAS_EN0 | PM800_GPADC_GP_BIAS_EN2 |
    PM800_GPADC_GP_BIAS_EN3);
    ret = regmap_update_bits(map, PM800_GP_BIAS_ENA1, mask, data);
    if (ret < 0)
    goto out;
    dev_info(chip.dev, "pm800 device_gpadc_init: Done\n");
    return 0;
    out:
    dev_info(chip.dev, "pm800 device_gpadc_init: Failed!\n");
    return ret;
    }
    static int device_onkey_init(struct pm80x_chip *chip,
    struct pm80x_platform_data *pdata)
    {
    int ret;
    ret = mfd_add_devices(chip.dev, 0, &onkey_devs[0],
    ARRAY_SIZE(onkey_devs), &onkey_resources[0], 0,
    core::ptr::null_mut());
    if (ret) {
    dev_err(chip.dev, "Failed to add onkey subdev\n");
    return ret;
    }
    return 0;
    }
    static int device_rtc_init(struct pm80x_chip *chip,
    struct pm80x_platform_data *pdata)
    {
    int ret;
    if (pdata) {
    rtc_devs[0].platform_data = pdata.rtc;
    rtc_devs[0].pdata_size =
    pdata.rtc ? sizeof(struct pm80x_rtc_pdata) : 0;
    }
    ret = mfd_add_devices(chip.dev, 0, &rtc_devs[0],
    ARRAY_SIZE(rtc_devs), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret) {
    dev_err(chip.dev, "Failed to add rtc subdev\n");
    return ret;
    }
    return 0;
    }
    static int device_regulator_init(struct pm80x_chip *chip,
    struct pm80x_platform_data *pdata)
    {
    int ret;
    ret = mfd_add_devices(chip.dev, 0, &regulator_devs[0],
    ARRAY_SIZE(regulator_devs), core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret) {
    dev_err(chip.dev, "Failed to add regulator subdev\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn device_irq_init_800(chip: *mut pm80x_chip) -> c_int {
    static int device_irq_init_800(struct pm80x_chip *chip)
    {
    struct regmap *map = chip.regmap;
    let mut flags: c_ulong = IRQF_ONESHOT;
    int data, mask, ret = -EINVAL;
    if (!map || !chip.irq) {
    dev_err(chip.dev, "incorrect parameters\n");
    return -EINVAL;
    }
//
// irq_mode defines the way of clearing interrupt. it's read-clear by
// default.
//
    mask =
    PM800_WAKEUP2_INV_INT | PM800_WAKEUP2_INT_CLEAR |
    PM800_WAKEUP2_INT_MASK;
    data = PM800_WAKEUP2_INT_CLEAR;
    ret = regmap_update_bits(map, PM800_WAKEUP2, mask, data);
    if (ret < 0)
    goto out;
    ret =
    regmap_add_irq_chip(chip.regmap, chip.irq, flags, -1,
    chip.regmap_irq_chip, &chip.irq_data);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn device_irq_exit_800(chip: *mut pm80x_chip) {
    static void device_irq_exit_800(struct pm80x_chip *chip)
    {
    regmap_del_irq_chip(chip.irq, chip.irq_data);
    }
    static const struct regmap_irq_chip pm800_irq_chip = {
    .name = "88pm800",
    .irqs = pm800_irqs,
    .num_irqs = ARRAY_SIZE(pm800_irqs),
    .num_regs = 4,
    .status_base = PM800_INT_STATUS1,
    .unmask_base = PM800_INT_ENA_1,
    .ack_base = PM800_INT_STATUS1,
    };
#[no_mangle]
unsafe extern "C" fn pm800_pages_init(chip: *mut pm80x_chip) -> c_int {
    static int pm800_pages_init(struct pm80x_chip *chip)
    {
    struct pm80x_subchip *subchip;
    struct i2c_client *client = chip.client;
    let mut ret: c_int = 0;
    subchip = chip.subchip;
    if (!subchip || !subchip.power_page_addr || !subchip.gpadc_page_addr)
    return -ENODEV;
// PM800 block power page
    subchip.power_page = i2c_new_dummy_device(client.adapter,
    subchip.power_page_addr);
    if (IS_ERR(subchip.power_page)) {
    ret = PTR_ERR(subchip.power_page);
    goto out;
    }
    subchip.regmap_power = devm_regmap_init_i2c(subchip.power_page,
    &pm80x_regmap_config);
    if (IS_ERR(subchip.regmap_power)) {
    ret = PTR_ERR(subchip.regmap_power);
    dev_err(chip.dev,
    "Failed to allocate regmap_power: %d\n", ret);
    goto out;
    }
    i2c_set_clientdata(subchip.power_page, chip);
// PM800 block GPADC
    subchip.gpadc_page = i2c_new_dummy_device(client.adapter,
    subchip.gpadc_page_addr);
    if (IS_ERR(subchip.gpadc_page)) {
    ret = PTR_ERR(subchip.gpadc_page);
    goto out;
    }
    subchip.regmap_gpadc = devm_regmap_init_i2c(subchip.gpadc_page,
    &pm80x_regmap_config);
    if (IS_ERR(subchip.regmap_gpadc)) {
    ret = PTR_ERR(subchip.regmap_gpadc);
    dev_err(chip.dev,
    "Failed to allocate regmap_gpadc: %d\n", ret);
    goto out;
    }
    i2c_set_clientdata(subchip.gpadc_page, chip);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm800_pages_exit(chip: *mut pm80x_chip) {
    static void pm800_pages_exit(struct pm80x_chip *chip)
    {
    struct pm80x_subchip *subchip;
    subchip = chip.subchip;
    if (subchip && subchip.power_page)
    i2c_unregister_device(subchip.power_page);
    if (subchip && subchip.gpadc_page)
    i2c_unregister_device(subchip.gpadc_page);
    }
    static int device_800_init(struct pm80x_chip *chip,
    struct pm80x_platform_data *pdata)
    {
    int ret;
    unsigned int val;
//
// alarm wake up bit will be clear in device_irq_init(),
// read before that
//
    ret = regmap_read(chip.regmap, PM800_RTC_CONTROL, &val);
    if (ret < 0) {
    dev_err(chip.dev, "Failed to read RTC register: %d\n", ret);
    goto out;
    }
    if (val & PM800_ALARM_WAKEUP) {
    if (pdata && pdata.rtc)
    pdata.rtc.rtc_wakeup = 1;
    }
    ret = device_gpadc_init(chip, pdata);
    if (ret < 0) {
    dev_err(chip.dev, "[%s]Failed to init gpadc\n", __func__);
    goto out;
    }
    chip.regmap_irq_chip = &pm800_irq_chip;
    ret = device_irq_init_800(chip);
    if (ret < 0) {
    dev_err(chip.dev, "[%s]Failed to init pm800 irq\n", __func__);
    goto out;
    }
    ret = device_onkey_init(chip, pdata);
    if (ret) {
    dev_err(chip.dev, "Failed to add onkey subdev\n");
    goto out_dev;
    }
    ret = device_rtc_init(chip, pdata);
    if (ret) {
    dev_err(chip.dev, "Failed to add rtc subdev\n");
    goto out;
    }
    ret = device_regulator_init(chip, pdata);
    if (ret) {
    dev_err(chip.dev, "Failed to add regulators subdev\n");
    goto out;
    }
    return 0;
    out_dev:
    mfd_remove_devices(chip.dev);
    device_irq_exit_800(chip);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm800_probe(client: *mut i2c_client) -> c_int {
    static int pm800_probe(struct i2c_client *client)
    {
    let mut ret: c_int = 0;
    struct pm80x_chip *chip;
    struct pm80x_platform_data *pdata = dev_get_platdata(&client.dev);
    struct pm80x_subchip *subchip;
    ret = pm80x_init(client);
    if (ret) {
    dev_err(&client.dev, "pm800_init fail\n");
    goto out_init;
    }
    chip = i2c_get_clientdata(client);
// init subchip for PM800
    subchip =
    devm_kzalloc(&client.dev, sizeof(struct pm80x_subchip),
    GFP_KERNEL);
    if (!subchip) {
    ret = -ENOMEM;
    goto err_subchip_alloc;
    }
// pm800 has 2 addtional pages to support power and gpadc.
    subchip.power_page_addr = client.addr + 1;
    subchip.gpadc_page_addr = client.addr + 2;
    chip.subchip = subchip;
    ret = pm800_pages_init(chip);
    if (ret) {
    dev_err(&client.dev, "pm800_pages_init failed!\n");
    goto err_device_init;
    }
    ret = device_800_init(chip, pdata);
    if (ret) {
    dev_err(chip.dev, "Failed to initialize 88pm800 devices\n");
    goto err_device_init;
    }
    if (pdata && pdata.plat_config)
    pdata.plat_config(chip, pdata);
    return 0;
    err_device_init:
    pm800_pages_exit(chip);
    err_subchip_alloc:
    pm80x_deinit();
    out_init:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pm800_remove(client: *mut i2c_client) {
    static void pm800_remove(struct i2c_client *client)
    {
    struct pm80x_chip *chip = i2c_get_clientdata(client);
    mfd_remove_devices(chip.dev);
    device_irq_exit_800(chip);
    pm800_pages_exit(chip);
    pm80x_deinit();
    }
    static struct i2c_driver pm800_driver = {
    .driver = {
    .name = "88PM800",
    .pm = pm_sleep_ptr(&pm80x_pm_ops),
    },
    .probe = pm800_probe,
    .remove = pm800_remove,
    .id_table = pm80x_id_table,
    };
#[no_mangle]
unsafe extern "C" fn pm800_i2c_init() -> int __init {
    static int __init pm800_i2c_init(void)
    {
    return i2c_add_driver(&pm800_driver);
    }
    subsys_initcall(pm800_i2c_init);
#[no_mangle]
unsafe extern "C" fn pm800_i2c_exit() -> void __exit {
    static void __exit pm800_i2c_exit(void)
    {
    i2c_del_driver(&pm800_driver);
    }
    module_exit(pm800_i2c_exit);
    MODULE_DESCRIPTION("PMIC Driver for Marvell 88PM800");
    MODULE_AUTHOR("Qiao Zhou <zhouqiao@marvell.com>");
    MODULE_LICENSE("GPL");
