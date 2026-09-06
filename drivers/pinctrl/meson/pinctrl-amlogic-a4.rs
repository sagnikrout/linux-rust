//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/meson/pinctrl-amlogic-a4.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// Copyright (c) 2024 Amlogic, Inc. All rights reserved.
// Author: Xianwei Zhao <xianwei.zhao@amlogic.com>
//

    container_of(chip, struct aml_gpio_bank, gpio_chip)
pub const AML_REG_PULLEN: c_int = 0;
pub const AML_REG_PULL: c_int = 1;
pub const AML_REG_DIR: c_int = 2;
pub const AML_REG_OUT: c_int = 3;
pub const AML_REG_IN: c_int = 4;
pub const AML_REG_DS: c_int = 5;
pub const AML_NUM_REG: c_int = 6;
    enum aml_pinconf_drv {
    PINCONF_DRV_500UA,
    PINCONF_DRV_2500UA,
    PINCONF_DRV_3000UA,
    PINCONF_DRV_4000UA,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_pio_control {
    pub gpio_offset: u32,
    pub reg_offset: [u32; AML_NUM_REG],
    pub bit_offset: [u32; AML_NUM_REG],
}

//
// partial bank(subordinate) pins mux config use other bank(main) mux registgers
// m_bank_id:	the main bank which pin_id from 0, but register bit not from bit 0
// m_bit_offs:	bit offset the main bank mux register
// s_bit_offs:	start bit that subordinate bank use mux register
// sid:         start pin_id of subordinate bank
// eid:         end pin_id of subordinate bank
// next:	subordinate bank reused multiple other bank groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct multi_mux {
    pub m_bank_id: c_uint,
    pub m_bit_offs: c_uint,
    pub s_bit_offs: c_uint,
    pub sid: c_uint,
    pub eid: c_uint,
    pub next: *const multi_mux,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_pctl_data {
    pub number: c_uint,
    pub p_mux: *const multi_mux,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_pmx_func {
    pub name: *const c_char,
    pub groups: *const c_char,
    pub ngroups: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_pctl_group {
    pub name: *const c_char,
    pub npins: c_uint,
    pub pins: *mut c_uint,
    pub func: *mut c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_gpio_bank {
    pub gpio_chip: gpio_chip,
    pub pc: aml_pio_control,
    pub bank_id: u32,
    pub mux_bit_offs: u32,
    pub pin_base: c_uint,
    pub reg_mux: *mut regmap,
    pub reg_gpio: *mut regmap,
    pub reg_ds: *mut regmap,
    pub p_mux: *const multi_mux,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_pinctrl {
    pub dev: *mut device,
    pub pctl: *mut pinctrl_dev,
    pub banks: *mut aml_gpio_bank,
    pub nbanks: c_int,
    pub functions: *mut aml_pmx_func,
    pub nfunctions: c_int,
    pub groups: *mut aml_pctl_group,
    pub ngroups: c_int,
    pub data: *const aml_pctl_data,
}

    static const unsigned int aml_bit_strides[AML_NUM_REG] = {
    1, 1, 1, 1, 1, 2
    };
    static const unsigned int aml_def_regoffs[AML_NUM_REG] = {
    3, 4, 2, 1, 0, 7
    };
    static const char *aml_bank_name[31] = {
    "GPIOA", "GPIOB", "GPIOC", "GPIOD", "GPIOE", "GPIOF", "GPIOG",
    "GPIOH", "GPIOI", "GPIOJ", "GPIOK", "GPIOL", "GPIOM", "GPION",
    "GPIOO", "GPIOP", "GPIOQ", "GPIOR", "GPIOS", "GPIOT", "GPIOU",
    "GPIOV", "GPIOW", "GPIOX", "GPIOY", "GPIOZ", "GPIODV", "GPIOAO",
    "GPIOCC", "TEST_N", "ANALOG"
    };
    static const struct multi_mux multi_mux_a9[] = {
    {
    .m_bank_id = AMLOGIC_GPIO_C,
    .m_bit_offs = 4,
    .s_bit_offs = 0,
    .sid = (AMLOGIC_GPIO_D << 8) + 16,
    .eid = (AMLOGIC_GPIO_D << 8) + 16,
    .next = &multi_mux_a9[1],
    }, {
    .m_bank_id = AMLOGIC_GPIO_AO,
    .m_bit_offs = 0,
    .s_bit_offs = 52,
    .sid = (AMLOGIC_GPIO_D << 8) + 17,
    .eid = (AMLOGIC_GPIO_D << 8) + 17,
    .next = core::ptr::null_mut(),
    }, {
    .m_bank_id = AMLOGIC_GPIO_A,
    .m_bit_offs = 0,
    .s_bit_offs = 80,
    .sid = (AMLOGIC_GPIO_Y << 8) + 8,
    .eid = (AMLOGIC_GPIO_Y << 8) + 9,
    .next = core::ptr::null_mut(),
    }, {
    .m_bank_id = AMLOGIC_GPIO_CC,
    .m_bit_offs = 24,
    .s_bit_offs = 0,
    .sid = (AMLOGIC_GPIO_X << 8) + 16,
    .eid = (AMLOGIC_GPIO_X << 8) + 17,
    .next = core::ptr::null_mut(),
    },
    };
    static const struct aml_pctl_data a9_priv_data = {
    .number = ARRAY_SIZE(multi_mux_a9),
    .p_mux = multi_mux_a9,
    };
    static const struct multi_mux multi_mux_s7[] = {
    {
    .m_bank_id = AMLOGIC_GPIO_CC,
    .m_bit_offs = 24,
    .s_bit_offs = 0,
    .sid = (AMLOGIC_GPIO_X << 8) + 16,
    .eid = (AMLOGIC_GPIO_X << 8) + 19,
    .next = core::ptr::null_mut(),
    },
    };
    static const struct aml_pctl_data s7_priv_data = {
    .number = ARRAY_SIZE(multi_mux_s7),
    .p_mux = multi_mux_s7,
    };
    static const struct multi_mux multi_mux_s6[] = {
    {
    .m_bank_id = AMLOGIC_GPIO_CC,
    .m_bit_offs = 24,
    .s_bit_offs = 0,
    .sid = (AMLOGIC_GPIO_X << 8) + 16,
    .eid = (AMLOGIC_GPIO_X << 8) + 19,
    .next = core::ptr::null_mut(),
    }, {
    .m_bank_id = AMLOGIC_GPIO_F,
    .m_bit_offs = 4,
    .s_bit_offs = 0,
    .sid = (AMLOGIC_GPIO_D << 8) + 6,
    .eid = (AMLOGIC_GPIO_D << 8) + 6,
    .next = core::ptr::null_mut(),
    },
    };
    static const struct aml_pctl_data s6_priv_data = {
    .number = ARRAY_SIZE(multi_mux_s6),
    .p_mux = multi_mux_s6,
    };
    static int aml_pmx_calc_reg_and_offset(struct pinctrl_gpio_range *range,
    unsigned int pin, unsigned int *reg,
    unsigned int *offset)
    {
    unsigned int shift;
    shift = ((pin - range.pin_base) << 2) + *offset;
// reg = (shift / 32) * 4;
// offset = shift % 32;
    return 0;
    }
    static int aml_pctl_set_function(struct aml_pinctrl *info,
    struct pinctrl_gpio_range *range,
    int pin_id, int func)
    {
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
    unsigned int shift;
    int reg;
    int i, loop_count;
    let mut offset: c_uint = bank.mux_bit_offs;
    const struct multi_mux *p_mux;
// peculiar mux reg set
    loop_count = 10;
    p_mux = bank.p_mux;
    while (p_mux && loop_count) {
    if (pin_id >= p_mux.sid && pin_id <= p_mux.eid) {
    bank = core::ptr::null_mut();
    for (i = 0; i < info.nbanks; i++) {
    if (info.banks[i].bank_id == p_mux.m_bank_id) {
    bank = &info.banks[i];
    break;
    }
    }
    if (!bank || !bank.reg_mux)
    return -EINVAL;
    shift = ((pin_id - p_mux.sid) << 2) + p_mux.s_bit_offs;
    reg = (shift / 32) * 4;
    offset = shift % 32;
    return regmap_update_bits(bank.reg_mux, reg,
    0xf << offset, (func & 0xf) << offset);
    }
    p_mux = p_mux.next;
    loop_count--;
    }
// normal mux reg set
    if (!bank.reg_mux)
    return 0;
    aml_pmx_calc_reg_and_offset(range, pin_id, &reg, &offset);
    return regmap_update_bits(bank.reg_mux, reg,
    0xf << offset, (func & 0xf) << offset);
    }
#[no_mangle]
unsafe extern "C" fn aml_pmx_get_funcs_count(pctldev: *mut pinctrl_dev) -> c_int {
    static int aml_pmx_get_funcs_count(struct pinctrl_dev *pctldev)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pctldev);
    return info.nfunctions;
    }
    static const char *aml_pmx_get_fname(struct pinctrl_dev *pctldev,
    unsigned int selector)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pctldev);
    return info.functions[selector].name;
    }
    static int aml_pmx_get_groups(struct pinctrl_dev *pctldev,
    unsigned int selector,
    const char * const **grps,
    unsigned * const ngrps)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pctldev);
// grps = info->functions[selector].groups;
// ngrps = info->functions[selector].ngroups;
    return 0;
    }
    static int aml_pmx_set_mux(struct pinctrl_dev *pctldev, unsigned int fselector,
    unsigned int group_id)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pctldev);
    struct aml_pctl_group *group = &info.groups[group_id];
    struct pinctrl_gpio_range *range;
    int i;
    for (i = 0; i < group.npins; i++) {
    range =  pinctrl_find_gpio_range_from_pin_nolock(pctldev, group.pins[i]);
    aml_pctl_set_function(info, range, group.pins[i], group.func[i]);
    }
    return 0;
    }
    static int aml_pmx_request_gpio(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range,
    unsigned int pin)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pctldev);
    return aml_pctl_set_function(info, range, pin, 0);
    }
    static const struct pinmux_ops aml_pmx_ops = {
    .set_mux		= aml_pmx_set_mux,
    .get_functions_count	= aml_pmx_get_funcs_count,
    .get_function_name	= aml_pmx_get_fname,
    .get_function_groups	= aml_pmx_get_groups,
    .gpio_request_enable	= aml_pmx_request_gpio,
    };
    static int aml_calc_reg_and_bit(struct pinctrl_gpio_range *range,
    unsigned int pin,
    unsigned int reg_type,
    unsigned int *reg, unsigned int *bit)
    {
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
// bit = (pin - range->pin_base) * aml_bit_strides[reg_type]
    + bank.pc.bit_offset[reg_type];
// reg = (bank->pc.reg_offset[reg_type] + (*bit / 32)) * 4;
// bit &= 0x1f;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_pinconf_get_pull(info: *mut aml_pinctrl, pin: c_uint) -> c_int {
    static int aml_pinconf_get_pull(struct aml_pinctrl *info, unsigned int pin)
    {
    struct pinctrl_gpio_range *range =
    pinctrl_find_gpio_range_from_pin_nolock(info.pctl, pin);
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
    unsigned int reg, bit, val;
    int ret, conf;
    aml_calc_reg_and_bit(range, pin, AML_REG_PULLEN, &reg, &bit);
    ret = regmap_read(bank.reg_gpio, reg, &val);
    if (ret)
    return ret;
    if (!(val & BIT(bit))) {
    conf = PIN_CONFIG_BIAS_DISABLE;
    } else {
    aml_calc_reg_and_bit(range, pin, AML_REG_PULL, &reg, &bit);
    ret = regmap_read(bank.reg_gpio, reg, &val);
    if (ret)
    return ret;
    if (val & BIT(bit))
    conf = PIN_CONFIG_BIAS_PULL_UP;
    else
    conf = PIN_CONFIG_BIAS_PULL_DOWN;
    }
    return conf;
    }
    static int aml_pinconf_get_drive_strength(struct aml_pinctrl *info,
    unsigned int pin,
    u16 *drive_strength_ua)
    {
    struct pinctrl_gpio_range *range =
    pinctrl_find_gpio_range_from_pin_nolock(info.pctl, pin);
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
    unsigned int reg, bit;
    unsigned int val;
    int ret;
    if (!bank.reg_ds)
    return -EOPNOTSUPP;
    aml_calc_reg_and_bit(range, pin, AML_REG_DS, &reg, &bit);
    ret = regmap_read(bank.reg_ds, reg, &val);
    if (ret)
    return ret;
    switch ((val >> bit) & 0x3) {
    case PINCONF_DRV_500UA:
// drive_strength_ua = 500;
    break;
    case PINCONF_DRV_2500UA:
// drive_strength_ua = 2500;
    break;
    case PINCONF_DRV_3000UA:
// drive_strength_ua = 3000;
    break;
    case PINCONF_DRV_4000UA:
// drive_strength_ua = 4000;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int aml_pinconf_get_gpio_bit(struct aml_pinctrl *info,
    unsigned int pin,
    unsigned int reg_type)
    {
    struct pinctrl_gpio_range *range =
    pinctrl_find_gpio_range_from_pin_nolock(info.pctl, pin);
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
    unsigned int reg, bit, val;
    int ret;
    aml_calc_reg_and_bit(range, pin, reg_type, &reg, &bit);
    ret = regmap_read(bank.reg_gpio, reg, &val);
    if (ret)
    return ret;
    return BIT(bit) & val ? 1 : 0;
    }
    static int aml_pinconf_get_output(struct aml_pinctrl *info,
    unsigned int pin)
    {
    let mut ret: c_int = aml_pinconf_get_gpio_bit(info, pin, AML_REG_DIR);
    if (ret < 0)
    return ret;
    return !ret;
    }
    static int aml_pinconf_get_drive(struct aml_pinctrl *info,
    unsigned int pin)
    {
    return aml_pinconf_get_gpio_bit(info, pin, AML_REG_OUT);
    }
    static int aml_pinconf_get(struct pinctrl_dev *pcdev, unsigned int pin,
    unsigned long *config)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pcdev);
    let mut param: enum pin_config_param = pinconf_to_config_param(*config);
    u16 arg;
    int ret;
    switch (param) {
    case PIN_CONFIG_BIAS_DISABLE:
    case PIN_CONFIG_BIAS_PULL_DOWN:
    case PIN_CONFIG_BIAS_PULL_UP:
    if (aml_pinconf_get_pull(info, pin) == param)
    arg = 1;
    else
    return -EINVAL;
    break;
    case PIN_CONFIG_DRIVE_STRENGTH_UA:
    ret = aml_pinconf_get_drive_strength(info, pin, &arg);
    if (ret)
    return ret;
    break;
    case PIN_CONFIG_OUTPUT_ENABLE:
    ret = aml_pinconf_get_output(info, pin);
    if (ret < 0)
    return -EINVAL;
    arg = ret;
    break;
    case PIN_CONFIG_INPUT_ENABLE:
    ret = aml_pinconf_get_output(info, pin);
    if (ret < 0)
    return -EINVAL;
    arg = !ret;
    break;
    case PIN_CONFIG_LEVEL:
    ret = aml_pinconf_get_output(info, pin);
    if (ret <= 0)
    return -EINVAL;
    ret = aml_pinconf_get_drive(info, pin);
    if (ret < 0)
    return -EINVAL;
    arg = ret;
    break;
    default:
    return -ENOTSUPP;
    }
// config = pinconf_to_config_packed(param, arg);
    dev_dbg(info.dev, "pinconf for pin %u is %lu\n", pin, *config);
    return 0;
    }
    static int aml_pinconf_disable_bias(struct aml_pinctrl *info,
    unsigned int pin)
    {
    struct pinctrl_gpio_range *range =
    pinctrl_find_gpio_range_from_pin_nolock(info.pctl, pin);
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
    unsigned int reg, bit = 0;
    aml_calc_reg_and_bit(range, pin, AML_REG_PULLEN, &reg, &bit);
    return regmap_update_bits(bank.reg_gpio, reg, BIT(bit), 0);
    }
    static int aml_pinconf_enable_bias(struct aml_pinctrl *info, unsigned int pin,
    bool pull_up)
    {
    struct pinctrl_gpio_range *range =
    pinctrl_find_gpio_range_from_pin_nolock(info.pctl, pin);
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
    unsigned int reg, bit, val = 0;
    int ret;
    aml_calc_reg_and_bit(range, pin, AML_REG_PULL, &reg, &bit);
    if (pull_up)
    val = BIT(bit);
    ret = regmap_update_bits(bank.reg_gpio, reg, BIT(bit), val);
    if (ret)
    return ret;
    aml_calc_reg_and_bit(range, pin, AML_REG_PULLEN, &reg, &bit);
    return regmap_update_bits(bank.reg_gpio, reg, BIT(bit), BIT(bit));
    }
    static int aml_pinconf_set_drive_strength(struct aml_pinctrl *info,
    unsigned int pin,
    u16 drive_strength_ua)
    {
    struct pinctrl_gpio_range *range =
    pinctrl_find_gpio_range_from_pin_nolock(info.pctl, pin);
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
    unsigned int reg, bit, ds_val;
    if (!bank.reg_ds) {
    dev_err(info.dev, "drive-strength not supported\n");
    return -EOPNOTSUPP;
    }
    aml_calc_reg_and_bit(range, pin, AML_REG_DS, &reg, &bit);
    if (drive_strength_ua <= 500) {
    ds_val = PINCONF_DRV_500UA;
    } else if (drive_strength_ua <= 2500) {
    ds_val = PINCONF_DRV_2500UA;
    } else if (drive_strength_ua <= 3000) {
    ds_val = PINCONF_DRV_3000UA;
    } else if (drive_strength_ua <= 4000) {
    ds_val = PINCONF_DRV_4000UA;
    } else {
    dev_warn_once(info.dev,
    "pin %u: invalid drive-strength : %d , default to 4mA\n",
    pin, drive_strength_ua);
    ds_val = PINCONF_DRV_4000UA;
    }
    return regmap_update_bits(bank.reg_ds, reg, 0x3 << bit, ds_val << bit);
    }
    static int aml_pinconf_set_gpio_bit(struct aml_pinctrl *info,
    unsigned int pin,
    unsigned int reg_type,
    bool arg)
    {
    struct pinctrl_gpio_range *range =
    pinctrl_find_gpio_range_from_pin_nolock(info.pctl, pin);
    struct aml_gpio_bank *bank = gpio_chip_to_bank(range.gc);
    unsigned int reg, bit;
    aml_calc_reg_and_bit(range, pin, reg_type, &reg, &bit);
    return regmap_update_bits(bank.reg_gpio, reg, BIT(bit),
    arg ? BIT(bit) : 0);
    }
    static int aml_pinconf_set_output(struct aml_pinctrl *info,
    unsigned int pin,
    bool out)
    {
    return aml_pinconf_set_gpio_bit(info, pin, AML_REG_DIR, !out);
    }
    static int aml_pinconf_set_drive(struct aml_pinctrl *info,
    unsigned int pin,
    bool high)
    {
    return aml_pinconf_set_gpio_bit(info, pin, AML_REG_OUT, high);
    }
    static int aml_pinconf_set_output_drive(struct aml_pinctrl *info,
    unsigned int pin,
    bool high)
    {
    int ret;
    ret = aml_pinconf_set_drive(info, pin, high);
    if (ret)
    return ret;
    return aml_pinconf_set_output(info, pin, true);
    }
    static int aml_pinconf_set(struct pinctrl_dev *pcdev, unsigned int pin,
    unsigned long *configs, unsigned int num_configs)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pcdev);
    enum pin_config_param param;
    let mut arg: c_uint = 0;
    int i, ret;
    for (i = 0; i < num_configs; i++) {
    param = pinconf_to_config_param(configs[i]);
    switch (param) {
    case PIN_CONFIG_DRIVE_STRENGTH_UA:
    case PIN_CONFIG_OUTPUT_ENABLE:
    case PIN_CONFIG_INPUT_ENABLE:
    case PIN_CONFIG_LEVEL:
    arg = pinconf_to_config_argument(configs[i]);
    break;
    default:
    break;
    }
    switch (param) {
    case PIN_CONFIG_BIAS_DISABLE:
    ret = aml_pinconf_disable_bias(info, pin);
    break;
    case PIN_CONFIG_BIAS_PULL_UP:
    ret = aml_pinconf_enable_bias(info, pin, true);
    break;
    case PIN_CONFIG_BIAS_PULL_DOWN:
    ret = aml_pinconf_enable_bias(info, pin, false);
    break;
    case PIN_CONFIG_DRIVE_STRENGTH_UA:
    ret = aml_pinconf_set_drive_strength(info, pin, arg);
    break;
    case PIN_CONFIG_OUTPUT_ENABLE:
    ret = aml_pinconf_set_output(info, pin, arg);
    break;
    case PIN_CONFIG_INPUT_ENABLE:
    ret = aml_pinconf_set_output(info, pin, !arg);
    break;
    case PIN_CONFIG_LEVEL:
    ret = aml_pinconf_set_output_drive(info, pin, arg);
    break;
    default:
    ret = -ENOTSUPP;
    }
    if (ret)
    return ret;
    }
    return 0;
    }
    static int aml_pinconf_group_set(struct pinctrl_dev *pcdev,
    unsigned int num_group,
    unsigned long *configs,
    unsigned int num_configs)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pcdev);
    int i;
    for (i = 0; i < info.groups[num_group].npins; i++) {
    aml_pinconf_set(pcdev, info.groups[num_group].pins[i], configs,
    num_configs);
    }
    return 0;
    }
    static int aml_pinconf_group_get(struct pinctrl_dev *pcdev,
    unsigned int group, unsigned long *config)
    {
    return -EOPNOTSUPP;
    }
    static const struct pinconf_ops aml_pinconf_ops = {
    .pin_config_get		= aml_pinconf_get,
    .pin_config_set		= aml_pinconf_set,
    .pin_config_group_get	= aml_pinconf_group_get,
    .pin_config_group_set	= aml_pinconf_group_set,
    .is_generic		= true,
    };
#[no_mangle]
unsafe extern "C" fn aml_get_groups_count(pctldev: *mut pinctrl_dev) -> c_int {
    static int aml_get_groups_count(struct pinctrl_dev *pctldev)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pctldev);
    return info.ngroups;
    }
    static const char *aml_get_group_name(struct pinctrl_dev *pctldev,
    unsigned int selector)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pctldev);
    return info.groups[selector].name;
    }
    static int aml_get_group_pins(struct pinctrl_dev *pctldev,
    unsigned int selector, const unsigned int **pins,
    unsigned int *npins)
    {
    struct aml_pinctrl *info = pinctrl_dev_get_drvdata(pctldev);
    if (selector >= info.ngroups)
    return -EINVAL;
// pins = info->groups[selector].pins;
// npins = info->groups[selector].npins;
    return 0;
    }
    static void aml_pin_dbg_show(struct pinctrl_dev *pcdev, struct seq_file *s,
    unsigned int offset)
    {
    seq_printf(s, " %s", dev_name(pcdev.dev));
    }
    static int aml_dt_node_to_map_pinmux(struct pinctrl_dev *pctldev,
    struct device_node *np,
    struct pinctrl_map **map,
    unsigned int *num_maps)
    {
    struct device *dev = pctldev.dev;
    unsigned long *configs = core::ptr::null_mut();
    let mut num_configs: c_uint = 0;
    struct property *prop;
    unsigned int reserved_maps;
    int reserve;
    int ret;
    prop = of_find_property(np, "pinmux", core::ptr::null_mut());
    if (!prop) {
    dev_info(dev, "Missing pinmux property\n");
    return -ENOENT;
    }
    struct device_node *pnode __free(device_node) = of_get_parent(np);
    if (!pnode) {
    dev_info(dev, "Missing function node\n");
    return -EINVAL;
    }
    reserved_maps = 0;
// map = NULL;
// num_maps = 0;
    ret = pinconf_generic_parse_dt_config(np, pctldev, &configs,
    &num_configs);
    if (ret < 0) {
    dev_err(dev, "%pOF: could not parse node property\n", np);
    return ret;
    }
    reserve = 1;
    if (num_configs)
    reserve++;
    ret = pinctrl_utils_reserve_map(pctldev, map, &reserved_maps,
    num_maps, reserve);
    if (ret < 0)
    goto exit;
    ret = pinctrl_utils_add_map_mux(pctldev, map,
    &reserved_maps, num_maps, np.name,
    pnode.name);
    if (ret < 0)
    goto exit;
    if (num_configs) {
    ret = pinctrl_utils_add_map_configs(pctldev, map, &reserved_maps,
    num_maps, np.name, configs,
    num_configs, PIN_MAP_TYPE_CONFIGS_GROUP);
    if (ret < 0)
    goto exit;
    }
    exit:
    kfree(configs);
    if (ret)
    pinctrl_utils_free_map(pctldev, *map, *num_maps);
    return ret;
    }
    static const struct pinctrl_ops aml_pctrl_ops = {
    .get_groups_count	= aml_get_groups_count,
    .get_group_name		= aml_get_group_name,
    .get_group_pins		= aml_get_group_pins,
    .dt_node_to_map		= aml_dt_node_to_map_pinmux,
    .dt_free_map		= pinconf_generic_dt_free_map,
    .pin_dbg_show		= aml_pin_dbg_show,
    };
    static int aml_pctl_parse_functions(struct device_node *np,
    struct aml_pinctrl *info, u32 index,
    int *grp_index)
    {
    struct device *dev = info.dev;
    struct aml_pmx_func *func;
    struct aml_pctl_group *grp;
    int ret, i;
    func = &info.functions[index];
    func.name = np.name;
    func.ngroups = of_get_child_count(np);
    if (func.ngroups == 0)
    return dev_err_probe(dev, -EINVAL, "No groups defined\n");
    func.groups = devm_kcalloc(dev, func.ngroups, sizeof(*func.groups), GFP_KERNEL);
    if (!func.groups)
    return -ENOMEM;
    i = 0;
    for_each_child_of_node_scoped(np, child) {
    func.groups[i++] = child.name;
    grp = &info.groups[*grp_index];
    grp.name = child.name;
// grp_index += 1;
    ret = pinconf_generic_parse_dt_pinmux(child, dev, &grp.pins,
    &grp.func, &grp.npins);
    if (ret) {
    dev_err(dev, "function :%s, groups:%s fail\n", func.name, child.name);
    return ret;
    }
    }
    dev_dbg(dev, "Function[%d\t name:%s,\tgroups:%d]\n", index, func.name, func.ngroups);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_bank_pins(np: *mut device_node) -> u32 {
    static u32 aml_bank_pins(struct device_node *np)
    {
    struct of_phandle_args of_args;
    if (of_parse_phandle_with_fixed_args(np, "gpio-ranges", 3,
    0, &of_args))
    return 0;
    of_node_put(of_args.np);
    return of_args.args[2];
    }
#[no_mangle]
unsafe extern "C" fn aml_bank_number(np: *mut device_node) -> c_int {
    static int aml_bank_number(struct device_node *np)
    {
    struct of_phandle_args of_args;
    if (of_parse_phandle_with_fixed_args(np, "gpio-ranges", 3,
    0, &of_args))
    return -EINVAL;
    of_node_put(of_args.np);
    return of_args.args[1] >> 8;
    }
#[no_mangle]
unsafe extern "C" fn aml_count_pins(np: *mut device_node) -> c_uint {
    static unsigned int aml_count_pins(struct device_node *np)
    {
    struct device_node *child;
    let mut pins: c_uint = 0;
    for_each_child_of_node(np, child) {
    if (of_property_read_bool(child, "gpio-controller"))
    pins += aml_bank_pins(child);
    }
    return pins;
    }
//
// A pinctrl device contains two types of nodes. The one named GPIO
// bank which includes gpio-controller property. The other one named
// function which includes one or more pin groups. The pin group
// include pinmux property(global index in pinctrl dev, and mux vlaue
// in mux reg) and pin configuration properties.
//
    static void aml_pctl_dt_child_count(struct aml_pinctrl *info,
    struct device_node *np)
    {
    struct device_node *child;
    for_each_child_of_node(np, child) {
    if (of_property_read_bool(child, "gpio-controller")) {
    info.nbanks++;
    } else {
    info.nfunctions++;
    info.ngroups += of_get_child_count(child);
    }
    }
    }
    static struct regmap *aml_map_resource(struct device *dev, unsigned int id,
    struct device_node *node, char *name)
    {
    struct resource res;
    void __iomem *base;
    int i;
    struct regmap_config aml_regmap_config = {
    .reg_bits = 32,
    .val_bits = 32,
    .reg_stride = 4,
    };
    i = of_property_match_string(node, "reg-names", name);
    if (i < 0)
    return core::ptr::null_mut();
    if (of_address_to_resource(node, i, &res))
    return core::ptr::null_mut();
    base = devm_ioremap_resource(dev, &res);
    if (IS_ERR(base))
    return ERR_CAST(base);
    aml_regmap_config.max_register = resource_size(&res) - 4;
    aml_regmap_config.name = devm_kasprintf(dev, GFP_KERNEL,
    "%s-%s", aml_bank_name[id], name);
    if (!aml_regmap_config.name)
    return ERR_PTR(-ENOMEM);
    return devm_regmap_init_mmio(dev, base, &aml_regmap_config);
    }
    static inline int aml_gpio_calc_reg_and_bit(struct aml_gpio_bank *bank,
    unsigned int reg_type,
    unsigned int gpio,
    unsigned int *reg,
    unsigned int *bit)
    {
// bit = gpio * aml_bit_strides[reg_type] + bank->pc.bit_offset[reg_type];
// reg = (bank->pc.reg_offset[reg_type] + (*bit / 32)) * 4;
// bit &= 0x1f;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_gpio_get_direction(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int aml_gpio_get_direction(struct gpio_chip *chip, unsigned int gpio)
    {
    struct aml_gpio_bank *bank = gpiochip_get_data(chip);
    unsigned int bit, reg, val;
    int ret;
    aml_gpio_calc_reg_and_bit(bank, AML_REG_DIR, gpio, &reg, &bit);
    ret = regmap_read(bank.reg_gpio, reg, &val);
    if (ret)
    return ret;
    return BIT(bit) & val ? GPIO_LINE_DIRECTION_IN : GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn aml_gpio_direction_input(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int aml_gpio_direction_input(struct gpio_chip *chip, unsigned int gpio)
    {
    struct aml_gpio_bank *bank = gpiochip_get_data(chip);
    unsigned int bit, reg;
    aml_gpio_calc_reg_and_bit(bank, AML_REG_DIR, gpio, &reg, &bit);
    return regmap_update_bits(bank.reg_gpio, reg, BIT(bit), BIT(bit));
    }
    static int aml_gpio_direction_output(struct gpio_chip *chip, unsigned int gpio,
    int value)
    {
    struct aml_gpio_bank *bank = gpiochip_get_data(chip);
    unsigned int bit, reg;
    int ret;
    aml_gpio_calc_reg_and_bit(bank, AML_REG_OUT, gpio, &reg, &bit);
    ret = regmap_update_bits(bank.reg_gpio, reg, BIT(bit),
    value ? BIT(bit) : 0);
    if (ret < 0)
    return ret;
    aml_gpio_calc_reg_and_bit(bank, AML_REG_DIR, gpio, &reg, &bit);
    return regmap_update_bits(bank.reg_gpio, reg, BIT(bit), 0);
    }
#[no_mangle]
unsafe extern "C" fn aml_gpio_set(chip: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int aml_gpio_set(struct gpio_chip *chip, unsigned int gpio, int value)
    {
    struct aml_gpio_bank *bank = gpiochip_get_data(chip);
    unsigned int bit, reg;
    aml_gpio_calc_reg_and_bit(bank, AML_REG_OUT, gpio, &reg, &bit);
    return regmap_update_bits(bank.reg_gpio, reg, BIT(bit),
    value ? BIT(bit) : 0);
    }
#[no_mangle]
unsafe extern "C" fn aml_gpio_get(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int aml_gpio_get(struct gpio_chip *chip, unsigned int gpio)
    {
    struct aml_gpio_bank *bank = gpiochip_get_data(chip);
    unsigned int reg, bit, val;
    aml_gpio_calc_reg_and_bit(bank, AML_REG_IN, gpio, &reg, &bit);
    regmap_read(bank.reg_gpio, reg, &val);
    return !!(val & BIT(bit));
    }
    static const struct gpio_chip aml_gpio_template = {
    .request		= gpiochip_generic_request,
    .free			= gpiochip_generic_free,
    .set_config		= gpiochip_generic_config,
    .set			= aml_gpio_set,
    .get			= aml_gpio_get,
    .direction_input	= aml_gpio_direction_input,
    .direction_output	= aml_gpio_direction_output,
    .get_direction		= aml_gpio_get_direction,
    .can_sleep		= true,
    };
    static void init_bank_register_bit(struct aml_pinctrl *info,
    struct aml_gpio_bank *bank)
    {
    const struct aml_pctl_data *data = info.data;
    const struct multi_mux *p_mux;
    int i;
    for (i = 0; i < AML_NUM_REG; i++) {
    bank.pc.reg_offset[i] = aml_def_regoffs[i];
    bank.pc.bit_offset[i] = 0;
    }
    bank.mux_bit_offs = 0;
    if (data) {
    for (i = 0; i < data.number; i++) {
    p_mux = &data.p_mux[i];
    if (bank.bank_id == p_mux.m_bank_id) {
    bank.mux_bit_offs = p_mux.m_bit_offs;
    break;
    }
    if (p_mux.sid >> 8 == bank.bank_id) {
    bank.p_mux = p_mux;
    break;
    }
    }
    }
    }
    static int aml_gpiolib_register_bank(struct aml_pinctrl *info,
    int bank_nr, struct device_node *np)
    {
    struct aml_gpio_bank *bank = &info.banks[bank_nr];
    struct device *dev = info.dev;
    let mut ret: c_int = 0;
    ret = aml_bank_number(np);
    if (ret < 0) {
    dev_err(dev, "get num=%d bank identity fail\n", bank_nr);
    return -EINVAL;
    }
    bank.bank_id = ret;
    bank.reg_mux = aml_map_resource(dev, bank.bank_id, np, "mux");
    if (IS_ERR_OR_NULL(bank.reg_mux)) {
    if (bank.bank_id == AMLOGIC_GPIO_TEST_N ||
    bank.bank_id == AMLOGIC_GPIO_ANALOG)
    bank.reg_mux = core::ptr::null_mut();
    else
    return dev_err_probe(dev, bank.reg_mux ? PTR_ERR(bank.reg_mux) : -ENOENT,
    "mux registers not found\n");
    }
    bank.reg_gpio = aml_map_resource(dev, bank.bank_id, np, "gpio");
    if (IS_ERR_OR_NULL(bank.reg_gpio))
    return dev_err_probe(dev, bank.reg_gpio ? PTR_ERR(bank.reg_gpio) : -ENOENT,
    "gpio registers not found\n");
    bank.reg_ds = aml_map_resource(dev, bank.bank_id, np, "ds");
    if (IS_ERR_OR_NULL(bank.reg_ds)) {
    dev_dbg(info.dev, "ds registers not found - skipping\n");
    bank.reg_ds = bank.reg_gpio;
    }
    bank.gpio_chip = aml_gpio_template;
    bank.gpio_chip.base = -1;
    bank.gpio_chip.ngpio = aml_bank_pins(np);
    bank.gpio_chip.fwnode = of_fwnode_handle(np);
    bank.gpio_chip.parent = dev;
    init_bank_register_bit(info, bank);
    bank.gpio_chip.label = aml_bank_name[bank.bank_id];
    bank.pin_base = bank.bank_id << 8;
    return 0;
    }
    static int aml_pctl_probe_dt(struct platform_device *pdev,
    struct pinctrl_desc *pctl_desc,
    struct aml_pinctrl *info)
    {
    struct device *dev = &pdev.dev;
    struct pinctrl_pin_desc *pdesc;
    struct device_node *np = dev.of_node;
    let mut grp_index: c_int = 0;
    let mut i: c_int = 0, j = 0, k = 0, bank;
    let mut ret: c_int = 0;
    aml_pctl_dt_child_count(info, np);
    if (!info.nbanks)
    return dev_err_probe(dev, -EINVAL, "you need at least one gpio bank\n");
    dev_dbg(dev, "nbanks = %d\n", info.nbanks);
    dev_dbg(dev, "nfunctions = %d\n", info.nfunctions);
    dev_dbg(dev, "ngroups = %d\n", info.ngroups);
    info.functions = devm_kcalloc(dev, info.nfunctions, sizeof(*info.functions), GFP_KERNEL);
    info.groups = devm_kcalloc(dev, info.ngroups, sizeof(*info.groups), GFP_KERNEL);
    info.banks = devm_kcalloc(dev, info.nbanks, sizeof(*info.banks), GFP_KERNEL);
    if (!info.functions || !info.groups || !info.banks)
    return -ENOMEM;
    info.data = (struct aml_pctl_data *)of_device_get_match_data(dev);
    pctl_desc.npins = aml_count_pins(np);
    pdesc =	devm_kcalloc(dev, pctl_desc.npins, sizeof(*pdesc), GFP_KERNEL);
    if (!pdesc)
    return -ENOMEM;
    pctl_desc.pins = pdesc;
    bank = 0;
    for_each_child_of_node_scoped(np, child) {
    if (of_property_read_bool(child, "gpio-controller")) {
    const char *bank_name = core::ptr::null_mut();
    char **pin_names;
    ret = aml_gpiolib_register_bank(info, bank, child);
    if (ret)
    return ret;
    k = info.banks[bank].pin_base;
    bank_name = info.banks[bank].gpio_chip.label;
    pin_names = devm_kasprintf_strarray(dev, bank_name,
    info.banks[bank].gpio_chip.ngpio);
    if (IS_ERR(pin_names))
    return PTR_ERR(pin_names);
    for (j = 0; j < info.banks[bank].gpio_chip.ngpio; j++, k++) {
    pdesc.number = k;
    pdesc.name = pin_names[j];
    pdesc++;
    }
    bank++;
    } else {
    ret = aml_pctl_parse_functions(child, info,
    i++, &grp_index);
    if (ret)
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aml_pctl_probe(pdev: *mut platform_device) -> c_int {
    static int aml_pctl_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct aml_pinctrl *info;
    struct pinctrl_desc *pctl_desc;
    int ret, i;
    pctl_desc = devm_kzalloc(dev, sizeof(*pctl_desc), GFP_KERNEL);
    if (!pctl_desc)
    return -ENOMEM;
    info = devm_kzalloc(dev, sizeof(*info), GFP_KERNEL);
    if (!info)
    return -ENOMEM;
    info.dev = dev;
    platform_set_drvdata(pdev, info);
    ret = aml_pctl_probe_dt(pdev, pctl_desc, info);
    if (ret)
    return ret;
    pctl_desc.owner	= THIS_MODULE;
    pctl_desc.pctlops	= &aml_pctrl_ops;
    pctl_desc.pmxops	= &aml_pmx_ops;
    pctl_desc.confops	= &aml_pinconf_ops;
    pctl_desc.name		= dev_name(dev);
    info.pctl = devm_pinctrl_register(dev, pctl_desc, info);
    if (IS_ERR(info.pctl))
    return dev_err_probe(dev, PTR_ERR(info.pctl), "Failed pinctrl registration\n");
    for (i = 0; i < info.nbanks; i++) {
    ret  = gpiochip_add_data(&info.banks[i].gpio_chip, &info.banks[i]);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to add gpiochip(%d)!\n", i);
    }
    return 0;
    }
    static const struct of_device_id aml_pctl_of_match[] = {
    { .compatible = "amlogic,pinctrl-a4", },
    { .compatible = "amlogic,pinctrl-a9", .data = &a9_priv_data, },
    { .compatible = "amlogic,pinctrl-s7", .data = &s7_priv_data, },
    { .compatible = "amlogic,pinctrl-s6", .data = &s6_priv_data, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, aml_pctl_of_match);
    static struct platform_driver aml_pctl_driver = {
    .driver = {
    .name = "amlogic-pinctrl",
    .of_match_table = aml_pctl_of_match,
    },
    .probe = aml_pctl_probe,
    };
    module_platform_driver(aml_pctl_driver);
    MODULE_AUTHOR("Xianwei Zhao <xianwei.zhao@amlogic.com>");
    MODULE_DESCRIPTION("Pin controller and GPIO driver for Amlogic SoC");
    MODULE_LICENSE("Dual BSD/GPL");
