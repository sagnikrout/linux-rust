//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/pinctrl-upboard.c
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
// UP board pin control driver.
//
// Copyright (C) 2025 Bootlin
//
// Author: Thomas Richard <thomas.richard@bootlin.com>
//

    enum upboard_pin_mode {
    UPBOARD_PIN_MODE_FUNCTION,
    UPBOARD_PIN_MODE_GPIO_IN,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_DISABLED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_pin {
    pub funcbit: *mut regmap_field,
    pub enbit: *mut regmap_field,
    pub dirbit: *mut regmap_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_pingroup {
    pub grp: pingroup,
    pub mode: enum upboard_pin_mode,
    pub modes: *const enum upboard_pin_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_pinctrl_data {
    pub groups: *const upboard_pingroup,
    pub ngroups: usize,
    pub funcs: *const pinfunction,
    pub nfuncs: usize,
    pub pin_header: *const c_uint,
    pub ngpio: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_pinctrl {
    pub dev: *mut device,
    pub pctldev: *mut pinctrl_dev,
    pub pctrl_data: *const upboard_pinctrl_data,
    pub pin_range: gpio_pin_range,
    pub pins: *mut upboard_pin,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upboard_pinctrl_map {
    pub maps: *const pinctrl_map,
    pub nmaps: usize,
}

    enum upboard_func0_fpgabit {
    UPBOARD_FUNC_I2C0_EN = 8,
    UPBOARD_FUNC_I2C1_EN = 9,
    UPBOARD_FUNC_CEC0_EN = 12,
    UPBOARD_FUNC_ADC0_EN = 14,
    };
    static const struct reg_field upboard_i2c0_reg =
    REG_FIELD(UPBOARD_REG_FUNC_EN0, UPBOARD_FUNC_I2C0_EN, UPBOARD_FUNC_I2C0_EN);
    static const struct reg_field upboard_i2c1_reg =
    REG_FIELD(UPBOARD_REG_FUNC_EN0, UPBOARD_FUNC_I2C1_EN, UPBOARD_FUNC_I2C1_EN);
    static const struct reg_field upboard_adc0_reg =
    REG_FIELD(UPBOARD_REG_FUNC_EN0, UPBOARD_FUNC_ADC0_EN, UPBOARD_FUNC_ADC0_EN);

    {							\
    .number = UPBOARD_UP_BIT_##id,			\
    .name = #id,					\
    }

    {							\
    .number = UPBOARD_UP_BIT_##bit,			\
    .name = "PINMUX_"#bit,				\
    .drv_data = (void *)(data),			\
    }

    {							\
    .number = UPBOARD_UP_BIT_##id,			\
    .name = #id,					\
    .drv_data = (void *)(data),			\
    }
    enum upboard_up_fpgabit {
    UPBOARD_UP_BIT_I2C1_SDA,
    UPBOARD_UP_BIT_I2C1_SCL,
    UPBOARD_UP_BIT_ADC0,
    UPBOARD_UP_BIT_UART1_RTS,
    UPBOARD_UP_BIT_GPIO27,
    UPBOARD_UP_BIT_GPIO22,
    UPBOARD_UP_BIT_SPI_MOSI,
    UPBOARD_UP_BIT_SPI_MISO,
    UPBOARD_UP_BIT_SPI_CLK,
    UPBOARD_UP_BIT_I2C0_SDA,
    UPBOARD_UP_BIT_GPIO5,
    UPBOARD_UP_BIT_GPIO6,
    UPBOARD_UP_BIT_PWM1,
    UPBOARD_UP_BIT_I2S_FRM,
    UPBOARD_UP_BIT_GPIO26,
    UPBOARD_UP_BIT_UART1_TX,
    UPBOARD_UP_BIT_UART1_RX,
    UPBOARD_UP_BIT_I2S_CLK,
    UPBOARD_UP_BIT_GPIO23,
    UPBOARD_UP_BIT_GPIO24,
    UPBOARD_UP_BIT_GPIO25,
    UPBOARD_UP_BIT_SPI_CS0,
    UPBOARD_UP_BIT_SPI_CS1,
    UPBOARD_UP_BIT_I2C0_SCL,
    UPBOARD_UP_BIT_PWM0,
    UPBOARD_UP_BIT_UART1_CTS,
    UPBOARD_UP_BIT_I2S_DIN,
    UPBOARD_UP_BIT_I2S_DOUT,
    };
    static const struct pinctrl_pin_desc upboard_up_pins[] = {
    UPBOARD_UP_PIN_FUNC(I2C1_SDA, &upboard_i2c1_reg),
    UPBOARD_UP_PIN_FUNC(I2C1_SCL, &upboard_i2c1_reg),
    UPBOARD_UP_PIN_FUNC(ADC0, &upboard_adc0_reg),
    UPBOARD_UP_PIN_NAME(UART1_RTS),
    UPBOARD_UP_PIN_NAME(GPIO27),
    UPBOARD_UP_PIN_NAME(GPIO22),
    UPBOARD_UP_PIN_NAME(SPI_MOSI),
    UPBOARD_UP_PIN_NAME(SPI_MISO),
    UPBOARD_UP_PIN_NAME(SPI_CLK),
    UPBOARD_UP_PIN_FUNC(I2C0_SDA, &upboard_i2c0_reg),
    UPBOARD_UP_PIN_NAME(GPIO5),
    UPBOARD_UP_PIN_NAME(GPIO6),
    UPBOARD_UP_PIN_NAME(PWM1),
    UPBOARD_UP_PIN_NAME(I2S_FRM),
    UPBOARD_UP_PIN_NAME(GPIO26),
    UPBOARD_UP_PIN_NAME(UART1_TX),
    UPBOARD_UP_PIN_NAME(UART1_RX),
    UPBOARD_UP_PIN_NAME(I2S_CLK),
    UPBOARD_UP_PIN_NAME(GPIO23),
    UPBOARD_UP_PIN_NAME(GPIO24),
    UPBOARD_UP_PIN_NAME(GPIO25),
    UPBOARD_UP_PIN_NAME(SPI_CS0),
    UPBOARD_UP_PIN_NAME(SPI_CS1),
    UPBOARD_UP_PIN_FUNC(I2C0_SCL, &upboard_i2c0_reg),
    UPBOARD_UP_PIN_NAME(PWM0),
    UPBOARD_UP_PIN_NAME(UART1_CTS),
    UPBOARD_UP_PIN_NAME(I2S_DIN),
    UPBOARD_UP_PIN_NAME(I2S_DOUT),
    };
    static const unsigned int upboard_up_pin_header[] = {
    UPBOARD_UP_BIT_TO_PIN(I2C0_SDA),
    UPBOARD_UP_BIT_TO_PIN(I2C0_SCL),
    UPBOARD_UP_BIT_TO_PIN(I2C1_SDA),
    UPBOARD_UP_BIT_TO_PIN(I2C1_SCL),
    UPBOARD_UP_BIT_TO_PIN(ADC0),
    UPBOARD_UP_BIT_TO_PIN(GPIO5),
    UPBOARD_UP_BIT_TO_PIN(GPIO6),
    UPBOARD_UP_BIT_TO_PIN(SPI_CS1),
    UPBOARD_UP_BIT_TO_PIN(SPI_CS0),
    UPBOARD_UP_BIT_TO_PIN(SPI_MISO),
    UPBOARD_UP_BIT_TO_PIN(SPI_MOSI),
    UPBOARD_UP_BIT_TO_PIN(SPI_CLK),
    UPBOARD_UP_BIT_TO_PIN(PWM0),
    UPBOARD_UP_BIT_TO_PIN(PWM1),
    UPBOARD_UP_BIT_TO_PIN(UART1_TX),
    UPBOARD_UP_BIT_TO_PIN(UART1_RX),
    UPBOARD_UP_BIT_TO_PIN(UART1_CTS),
    UPBOARD_UP_BIT_TO_PIN(UART1_RTS),
    UPBOARD_UP_BIT_TO_PIN(I2S_CLK),
    UPBOARD_UP_BIT_TO_PIN(I2S_FRM),
    UPBOARD_UP_BIT_TO_PIN(I2S_DIN),
    UPBOARD_UP_BIT_TO_PIN(I2S_DOUT),
    UPBOARD_UP_BIT_TO_PIN(GPIO22),
    UPBOARD_UP_BIT_TO_PIN(GPIO23),
    UPBOARD_UP_BIT_TO_PIN(GPIO24),
    UPBOARD_UP_BIT_TO_PIN(GPIO25),
    UPBOARD_UP_BIT_TO_PIN(GPIO26),
    UPBOARD_UP_BIT_TO_PIN(GPIO27),
    };
    static const unsigned int upboard_up_uart1_pins[] = {
    UPBOARD_UP_BIT_TO_PIN(UART1_TX),
    UPBOARD_UP_BIT_TO_PIN(UART1_RX),
    UPBOARD_UP_BIT_TO_PIN(UART1_RTS),
    UPBOARD_UP_BIT_TO_PIN(UART1_CTS),
    };
    static const enum upboard_pin_mode upboard_up_uart1_modes[] = {
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_IN,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_IN,
    };
    static_assert(ARRAY_SIZE(upboard_up_uart1_modes) == ARRAY_SIZE(upboard_up_uart1_pins));
    static const unsigned int upboard_up_i2c0_pins[] = {
    UPBOARD_UP_BIT_TO_PIN(I2C0_SCL),
    UPBOARD_UP_BIT_TO_PIN(I2C0_SDA),
    };
    static const unsigned int upboard_up_i2c1_pins[] = {
    UPBOARD_UP_BIT_TO_PIN(I2C1_SCL),
    UPBOARD_UP_BIT_TO_PIN(I2C1_SDA),
    };
    static const unsigned int upboard_up_spi2_pins[] = {
    UPBOARD_UP_BIT_TO_PIN(SPI_MOSI),
    UPBOARD_UP_BIT_TO_PIN(SPI_MISO),
    UPBOARD_UP_BIT_TO_PIN(SPI_CLK),
    UPBOARD_UP_BIT_TO_PIN(SPI_CS0),
    UPBOARD_UP_BIT_TO_PIN(SPI_CS1),
    };
    static const enum upboard_pin_mode upboard_up_spi2_modes[] = {
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_IN,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_OUT,
    };
    static_assert(ARRAY_SIZE(upboard_up_spi2_modes) == ARRAY_SIZE(upboard_up_spi2_pins));
    static const unsigned int upboard_up_i2s0_pins[]  = {
    UPBOARD_UP_BIT_TO_PIN(I2S_FRM),
    UPBOARD_UP_BIT_TO_PIN(I2S_CLK),
    UPBOARD_UP_BIT_TO_PIN(I2S_DIN),
    UPBOARD_UP_BIT_TO_PIN(I2S_DOUT),
    };
    static const enum upboard_pin_mode upboard_up_i2s0_modes[] = {
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_IN,
    UPBOARD_PIN_MODE_GPIO_OUT,
    };
    static_assert(ARRAY_SIZE(upboard_up_i2s0_pins) == ARRAY_SIZE(upboard_up_i2s0_modes));
    static const unsigned int upboard_up_pwm0_pins[] = {
    UPBOARD_UP_BIT_TO_PIN(PWM0),
    };
    static const unsigned int upboard_up_pwm1_pins[] = {
    UPBOARD_UP_BIT_TO_PIN(PWM1),
    };
    static const unsigned int upboard_up_adc0_pins[] = {
    UPBOARD_UP_BIT_TO_PIN(ADC0),
    };

    {												\
    .grp = PINCTRL_PINGROUP(n, p, ARRAY_SIZE(p)),						\
    .mode = __builtin_choose_expr(								\
    __builtin_types_compatible_p(typeof(m), const enum upboard_pin_mode *),	\
    0, m),									\
    .modes = __builtin_choose_expr(								\
    __builtin_types_compatible_p(typeof(m), const enum upboard_pin_mode *), \
    m, core::ptr::null_mut()),								\
    }
    static const struct upboard_pingroup upboard_up_pin_groups[] = {
    UPBOARD_PINGROUP("uart1_grp", upboard_up_uart1_pins, &upboard_up_uart1_modes[0]),
    UPBOARD_PINGROUP("i2c0_grp", upboard_up_i2c0_pins, UPBOARD_PIN_MODE_GPIO_OUT),
    UPBOARD_PINGROUP("i2c1_grp", upboard_up_i2c1_pins, UPBOARD_PIN_MODE_GPIO_OUT),
    UPBOARD_PINGROUP("spi2_grp", upboard_up_spi2_pins, &upboard_up_spi2_modes[0]),
    UPBOARD_PINGROUP("i2s0_grp", upboard_up_i2s0_pins, &upboard_up_i2s0_modes[0]),
    UPBOARD_PINGROUP("pwm0_grp", upboard_up_pwm0_pins, UPBOARD_PIN_MODE_GPIO_OUT),
    UPBOARD_PINGROUP("pwm1_grp", upboard_up_pwm1_pins, UPBOARD_PIN_MODE_GPIO_OUT),
    UPBOARD_PINGROUP("adc0_grp", upboard_up_adc0_pins, UPBOARD_PIN_MODE_GPIO_IN),
    };
    static const char * const upboard_up_uart1_groups[] = { "uart1_grp" };
    static const char * const upboard_up_i2c0_groups[]  = { "i2c0_grp" };
    static const char * const upboard_up_i2c1_groups[]  = { "i2c1_grp" };
    static const char * const upboard_up_spi2_groups[]  = { "spi2_grp" };
    static const char * const upboard_up_i2s0_groups[]  = { "i2s0_grp" };
    static const char * const upboard_up_pwm0_groups[]  = { "pwm0_grp" };
    static const char * const upboard_up_pwm1_groups[]  = { "pwm1_grp" };
    static const char * const upboard_up_adc0_groups[]  = { "adc0_grp" };

    static const struct pinfunction upboard_up_pin_functions[] = {
    UPBOARD_FUNCTION("uart1", upboard_up_uart1_groups),
    UPBOARD_FUNCTION("i2c0", upboard_up_i2c0_groups),
    UPBOARD_FUNCTION("i2c1", upboard_up_i2c1_groups),
    UPBOARD_FUNCTION("spi2", upboard_up_spi2_groups),
    UPBOARD_FUNCTION("i2s0", upboard_up_i2s0_groups),
    UPBOARD_FUNCTION("pwm0", upboard_up_pwm0_groups),
    UPBOARD_FUNCTION("pwm1", upboard_up_pwm1_groups),
    UPBOARD_FUNCTION("adc0", upboard_up_adc0_groups),
    };
    static const struct upboard_pinctrl_data upboard_up_pinctrl_data = {
    .groups = &upboard_up_pin_groups[0],
    .ngroups = ARRAY_SIZE(upboard_up_pin_groups),
    .funcs = &upboard_up_pin_functions[0],
    .nfuncs = ARRAY_SIZE(upboard_up_pin_functions),
    .pin_header = &upboard_up_pin_header[0],
    .ngpio = ARRAY_SIZE(upboard_up_pin_header),
    };

    {								\
    .number = UPBOARD_UP2_BIT_##id,				\
    .name = #id,						\
    }

    {								\
    .number = UPBOARD_UP2_BIT_##bit,			\
    .name = "PINMUX_"#bit,					\
    .drv_data = (void *)(data),				\
    }

    {								\
    .number = UPBOARD_UP2_BIT_##id,				\
    .name = #id,						\
    .drv_data = (void *)(data),				\
    }
    enum upboard_up2_fpgabit {
    UPBOARD_UP2_BIT_UART1_TXD,
    UPBOARD_UP2_BIT_UART1_RXD,
    UPBOARD_UP2_BIT_UART1_RTS,
    UPBOARD_UP2_BIT_UART1_CTS,
    UPBOARD_UP2_BIT_GPIO3_ADC0,
    UPBOARD_UP2_BIT_GPIO5_ADC2,
    UPBOARD_UP2_BIT_GPIO6_ADC3,
    UPBOARD_UP2_BIT_GPIO11,
    UPBOARD_UP2_BIT_EXHAT_LVDS1n,
    UPBOARD_UP2_BIT_EXHAT_LVDS1p,
    UPBOARD_UP2_BIT_SPI2_TXD,
    UPBOARD_UP2_BIT_SPI2_RXD,
    UPBOARD_UP2_BIT_SPI2_FS1,
    UPBOARD_UP2_BIT_SPI2_FS0,
    UPBOARD_UP2_BIT_SPI2_CLK,
    UPBOARD_UP2_BIT_SPI1_TXD,
    UPBOARD_UP2_BIT_SPI1_RXD,
    UPBOARD_UP2_BIT_SPI1_FS1,
    UPBOARD_UP2_BIT_SPI1_FS0,
    UPBOARD_UP2_BIT_SPI1_CLK,
    UPBOARD_UP2_BIT_I2C0_SCL,
    UPBOARD_UP2_BIT_I2C0_SDA,
    UPBOARD_UP2_BIT_I2C1_SCL,
    UPBOARD_UP2_BIT_I2C1_SDA,
    UPBOARD_UP2_BIT_PWM1,
    UPBOARD_UP2_BIT_PWM0,
    UPBOARD_UP2_BIT_EXHAT_LVDS0n,
    UPBOARD_UP2_BIT_EXHAT_LVDS0p,
    UPBOARD_UP2_BIT_GPIO24,
    UPBOARD_UP2_BIT_GPIO10,
    UPBOARD_UP2_BIT_GPIO2,
    UPBOARD_UP2_BIT_GPIO1,
    UPBOARD_UP2_BIT_EXHAT_LVDS3n,
    UPBOARD_UP2_BIT_EXHAT_LVDS3p,
    UPBOARD_UP2_BIT_EXHAT_LVDS4n,
    UPBOARD_UP2_BIT_EXHAT_LVDS4p,
    UPBOARD_UP2_BIT_EXHAT_LVDS5n,
    UPBOARD_UP2_BIT_EXHAT_LVDS5p,
    UPBOARD_UP2_BIT_I2S_SDO,
    UPBOARD_UP2_BIT_I2S_SDI,
    UPBOARD_UP2_BIT_I2S_WS_SYNC,
    UPBOARD_UP2_BIT_I2S_BCLK,
    UPBOARD_UP2_BIT_EXHAT_LVDS6n,
    UPBOARD_UP2_BIT_EXHAT_LVDS6p,
    UPBOARD_UP2_BIT_EXHAT_LVDS7n,
    UPBOARD_UP2_BIT_EXHAT_LVDS7p,
    UPBOARD_UP2_BIT_EXHAT_LVDS2n,
    UPBOARD_UP2_BIT_EXHAT_LVDS2p,
    };
    static const struct pinctrl_pin_desc upboard_up2_pins[] = {
    UPBOARD_UP2_PIN_NAME(UART1_TXD),
    UPBOARD_UP2_PIN_NAME(UART1_RXD),
    UPBOARD_UP2_PIN_NAME(UART1_RTS),
    UPBOARD_UP2_PIN_NAME(UART1_CTS),
    UPBOARD_UP2_PIN_NAME(GPIO3_ADC0),
    UPBOARD_UP2_PIN_NAME(GPIO5_ADC2),
    UPBOARD_UP2_PIN_NAME(GPIO6_ADC3),
    UPBOARD_UP2_PIN_NAME(GPIO11),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS1n),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS1p),
    UPBOARD_UP2_PIN_NAME(SPI2_TXD),
    UPBOARD_UP2_PIN_NAME(SPI2_RXD),
    UPBOARD_UP2_PIN_NAME(SPI2_FS1),
    UPBOARD_UP2_PIN_NAME(SPI2_FS0),
    UPBOARD_UP2_PIN_NAME(SPI2_CLK),
    UPBOARD_UP2_PIN_NAME(SPI1_TXD),
    UPBOARD_UP2_PIN_NAME(SPI1_RXD),
    UPBOARD_UP2_PIN_NAME(SPI1_FS1),
    UPBOARD_UP2_PIN_NAME(SPI1_FS0),
    UPBOARD_UP2_PIN_NAME(SPI1_CLK),
    UPBOARD_UP2_PIN_MUX(I2C0_SCL, &upboard_i2c0_reg),
    UPBOARD_UP2_PIN_MUX(I2C0_SDA, &upboard_i2c0_reg),
    UPBOARD_UP2_PIN_MUX(I2C1_SCL, &upboard_i2c1_reg),
    UPBOARD_UP2_PIN_MUX(I2C1_SDA, &upboard_i2c1_reg),
    UPBOARD_UP2_PIN_NAME(PWM1),
    UPBOARD_UP2_PIN_NAME(PWM0),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS0n),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS0p),
    UPBOARD_UP2_PIN_MUX(GPIO24, &upboard_i2c0_reg),
    UPBOARD_UP2_PIN_MUX(GPIO10, &upboard_i2c0_reg),
    UPBOARD_UP2_PIN_MUX(GPIO2, &upboard_i2c1_reg),
    UPBOARD_UP2_PIN_MUX(GPIO1, &upboard_i2c1_reg),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS3n),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS3p),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS4n),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS4p),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS5n),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS5p),
    UPBOARD_UP2_PIN_NAME(I2S_SDO),
    UPBOARD_UP2_PIN_NAME(I2S_SDI),
    UPBOARD_UP2_PIN_NAME(I2S_WS_SYNC),
    UPBOARD_UP2_PIN_NAME(I2S_BCLK),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS6n),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS6p),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS7n),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS7p),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS2n),
    UPBOARD_UP2_PIN_NAME(EXHAT_LVDS2p),
    };
    static const unsigned int upboard_up2_pin_header[] = {
    UPBOARD_UP2_BIT_TO_PIN(GPIO10),
    UPBOARD_UP2_BIT_TO_PIN(GPIO24),
    UPBOARD_UP2_BIT_TO_PIN(GPIO1),
    UPBOARD_UP2_BIT_TO_PIN(GPIO2),
    UPBOARD_UP2_BIT_TO_PIN(GPIO3_ADC0),
    UPBOARD_UP2_BIT_TO_PIN(GPIO11),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_CLK),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_FS1),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_FS0),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_RXD),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_TXD),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_CLK),
    UPBOARD_UP2_BIT_TO_PIN(PWM0),
    UPBOARD_UP2_BIT_TO_PIN(PWM1),
    UPBOARD_UP2_BIT_TO_PIN(UART1_TXD),
    UPBOARD_UP2_BIT_TO_PIN(UART1_RXD),
    UPBOARD_UP2_BIT_TO_PIN(UART1_CTS),
    UPBOARD_UP2_BIT_TO_PIN(UART1_RTS),
    UPBOARD_UP2_BIT_TO_PIN(I2S_BCLK),
    UPBOARD_UP2_BIT_TO_PIN(I2S_WS_SYNC),
    UPBOARD_UP2_BIT_TO_PIN(I2S_SDI),
    UPBOARD_UP2_BIT_TO_PIN(I2S_SDO),
    UPBOARD_UP2_BIT_TO_PIN(GPIO6_ADC3),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_FS1),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_RXD),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_TXD),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_FS0),
    UPBOARD_UP2_BIT_TO_PIN(GPIO5_ADC2),
    };
    static const unsigned int upboard_up2_uart1_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(UART1_TXD),
    UPBOARD_UP2_BIT_TO_PIN(UART1_RXD),
    UPBOARD_UP2_BIT_TO_PIN(UART1_RTS),
    UPBOARD_UP2_BIT_TO_PIN(UART1_CTS),
    };
    static const enum upboard_pin_mode upboard_up2_uart1_modes[] = {
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_IN,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_IN,
    };
    static_assert(ARRAY_SIZE(upboard_up2_uart1_modes) == ARRAY_SIZE(upboard_up2_uart1_pins));
    static const unsigned int upboard_up2_i2c0_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(I2C0_SCL),
    UPBOARD_UP2_BIT_TO_PIN(I2C0_SDA),
    UPBOARD_UP2_BIT_TO_PIN(GPIO24),
    UPBOARD_UP2_BIT_TO_PIN(GPIO10),
    };
    static const unsigned int upboard_up2_i2c1_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(I2C1_SCL),
    UPBOARD_UP2_BIT_TO_PIN(I2C1_SDA),
    UPBOARD_UP2_BIT_TO_PIN(GPIO2),
    UPBOARD_UP2_BIT_TO_PIN(GPIO1),
    };
    static const unsigned int upboard_up2_spi1_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(SPI1_TXD),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_RXD),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_FS1),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_FS0),
    UPBOARD_UP2_BIT_TO_PIN(SPI1_CLK),
    };
    static const unsigned int upboard_up2_spi2_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(SPI2_TXD),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_RXD),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_FS1),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_FS0),
    UPBOARD_UP2_BIT_TO_PIN(SPI2_CLK),
    };
    static const enum upboard_pin_mode upboard_up2_spi_modes[] = {
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_IN,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_OUT,
    };
    static_assert(ARRAY_SIZE(upboard_up2_spi_modes) == ARRAY_SIZE(upboard_up2_spi1_pins));
    static_assert(ARRAY_SIZE(upboard_up2_spi_modes) == ARRAY_SIZE(upboard_up2_spi2_pins));
    static const unsigned int upboard_up2_i2s0_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(I2S_BCLK),
    UPBOARD_UP2_BIT_TO_PIN(I2S_WS_SYNC),
    UPBOARD_UP2_BIT_TO_PIN(I2S_SDI),
    UPBOARD_UP2_BIT_TO_PIN(I2S_SDO),
    };
    static const enum upboard_pin_mode upboard_up2_i2s0_modes[] = {
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_OUT,
    UPBOARD_PIN_MODE_GPIO_IN,
    UPBOARD_PIN_MODE_GPIO_OUT,
    };
    static_assert(ARRAY_SIZE(upboard_up2_i2s0_modes) == ARRAY_SIZE(upboard_up2_i2s0_pins));
    static const unsigned int upboard_up2_pwm0_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(PWM0),
    };
    static const unsigned int upboard_up2_pwm1_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(PWM1),
    };
    static const unsigned int upboard_up2_adc0_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(GPIO3_ADC0),
    };
    static const unsigned int upboard_up2_adc2_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(GPIO5_ADC2),
    };
    static const unsigned int upboard_up2_adc3_pins[] = {
    UPBOARD_UP2_BIT_TO_PIN(GPIO6_ADC3),
    };
    static const struct upboard_pingroup upboard_up2_pin_groups[] = {
    UPBOARD_PINGROUP("uart1_grp", upboard_up2_uart1_pins, &upboard_up2_uart1_modes[0]),
    UPBOARD_PINGROUP("i2c0_grp", upboard_up2_i2c0_pins, UPBOARD_PIN_MODE_FUNCTION),
    UPBOARD_PINGROUP("i2c1_grp", upboard_up2_i2c1_pins, UPBOARD_PIN_MODE_FUNCTION),
    UPBOARD_PINGROUP("spi1_grp", upboard_up2_spi1_pins, &upboard_up2_spi_modes[0]),
    UPBOARD_PINGROUP("spi2_grp", upboard_up2_spi2_pins, &upboard_up2_spi_modes[0]),
    UPBOARD_PINGROUP("i2s0_grp", upboard_up2_i2s0_pins, &upboard_up2_i2s0_modes[0]),
    UPBOARD_PINGROUP("pwm0_grp", upboard_up2_pwm0_pins, UPBOARD_PIN_MODE_GPIO_OUT),
    UPBOARD_PINGROUP("pwm1_grp", upboard_up2_pwm1_pins, UPBOARD_PIN_MODE_GPIO_OUT),
    UPBOARD_PINGROUP("adc0_grp", upboard_up2_adc0_pins, UPBOARD_PIN_MODE_GPIO_IN),
    UPBOARD_PINGROUP("adc2_grp", upboard_up2_adc2_pins, UPBOARD_PIN_MODE_GPIO_IN),
    UPBOARD_PINGROUP("adc3_grp", upboard_up2_adc3_pins, UPBOARD_PIN_MODE_GPIO_IN),
    };
    static const char * const upboard_up2_uart1_groups[] = { "uart1_grp" };
    static const char * const upboard_up2_i2c0_groups[]  = { "i2c0_grp" };
    static const char * const upboard_up2_i2c1_groups[]  = { "i2c1_grp" };
    static const char * const upboard_up2_spi1_groups[]  = { "spi1_grp" };
    static const char * const upboard_up2_spi2_groups[]  = { "spi2_grp" };
    static const char * const upboard_up2_i2s0_groups[]  = { "i2s0_grp" };
    static const char * const upboard_up2_pwm0_groups[]  = { "pwm0_grp" };
    static const char * const upboard_up2_pwm1_groups[]  = { "pwm1_grp" };
    static const char * const upboard_up2_adc0_groups[]  = { "adc0_grp" };
    static const char * const upboard_up2_adc2_groups[]  = { "adc2_grp" };
    static const char * const upboard_up2_adc3_groups[]  = { "adc3_grp" };
    static const struct pinfunction upboard_up2_pin_functions[] = {
    UPBOARD_FUNCTION("uart1", upboard_up2_uart1_groups),
    UPBOARD_FUNCTION("i2c0", upboard_up2_i2c0_groups),
    UPBOARD_FUNCTION("i2c1", upboard_up2_i2c1_groups),
    UPBOARD_FUNCTION("spi1", upboard_up2_spi1_groups),
    UPBOARD_FUNCTION("spi2", upboard_up2_spi2_groups),
    UPBOARD_FUNCTION("i2s0", upboard_up2_i2s0_groups),
    UPBOARD_FUNCTION("pwm0", upboard_up2_pwm0_groups),
    UPBOARD_FUNCTION("pwm1", upboard_up2_pwm1_groups),
    UPBOARD_FUNCTION("adc0", upboard_up2_adc0_groups),
    UPBOARD_FUNCTION("adc2", upboard_up2_adc2_groups),
    UPBOARD_FUNCTION("adc3", upboard_up2_adc3_groups),
    };
    static const struct upboard_pinctrl_data upboard_up2_pinctrl_data = {
    .groups = &upboard_up2_pin_groups[0],
    .ngroups = ARRAY_SIZE(upboard_up2_pin_groups),
    .funcs = &upboard_up2_pin_functions[0],
    .nfuncs = ARRAY_SIZE(upboard_up2_pin_functions),
    .pin_header = &upboard_up2_pin_header[0],
    .ngpio = ARRAY_SIZE(upboard_up2_pin_header),
    };
#[no_mangle]
unsafe extern "C" fn upboard_pinctrl_set_function(pctldev: *mut pinctrl_dev, offset: c_uint) -> c_int {
    static int upboard_pinctrl_set_function(struct pinctrl_dev *pctldev, unsigned int offset)
    {
    struct upboard_pinctrl *pctrl = pinctrl_dev_get_drvdata(pctldev);
    struct upboard_pin *p = &pctrl.pins[offset];
    int ret;
    if (!p.funcbit)
    return -EPERM;
    ret = regmap_field_write(p.enbit, 0);
    if (ret)
    return ret;
    return regmap_field_write(p.funcbit, 1);
    }
#[no_mangle]
unsafe extern "C" fn upboard_pinctrl_gpio_commit_enable(pctldev: *mut pinctrl_dev, offset: c_uint) -> c_int {
    static int upboard_pinctrl_gpio_commit_enable(struct pinctrl_dev *pctldev, unsigned int offset)
    {
    struct upboard_pinctrl *pctrl = pinctrl_dev_get_drvdata(pctldev);
    struct upboard_pin *p = &pctrl.pins[offset];
    int ret;
    if (p.funcbit) {
    ret = regmap_field_write(p.funcbit, 0);
    if (ret)
    return ret;
    }
    return regmap_field_write(p.enbit, 1);
    }
    static int upboard_pinctrl_gpio_request_enable(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range,
    unsigned int offset)
    {
    return upboard_pinctrl_gpio_commit_enable(pctldev, offset);
    }
#[no_mangle]
unsafe extern "C" fn upboard_pinctrl_gpio_commit_disable(pctldev: *mut pinctrl_dev, offset: c_uint) {
    static void upboard_pinctrl_gpio_commit_disable(struct pinctrl_dev *pctldev, unsigned int offset)
    {
    struct upboard_pinctrl *pctrl = pinctrl_dev_get_drvdata(pctldev);
    struct upboard_pin *p = &pctrl.pins[offset];
    regmap_field_write(p.enbit, 0);
    };
    static void upboard_pinctrl_gpio_disable_free(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range, unsigned int offset)
    {
    return upboard_pinctrl_gpio_commit_disable(pctldev, offset);
    }
    static int upboard_pinctrl_gpio_commit_direction(struct pinctrl_dev *pctldev, unsigned int offset,
    bool input)
    {
    struct upboard_pinctrl *pctrl = pinctrl_dev_get_drvdata(pctldev);
    struct upboard_pin *p = &pctrl.pins[offset];
    return regmap_field_write(p.dirbit, input);
    }
    static int upboard_pinctrl_gpio_set_direction(struct pinctrl_dev *pctldev,
    struct pinctrl_gpio_range *range,
    unsigned int offset, bool input)
    {
    return upboard_pinctrl_gpio_commit_direction(pctldev, offset, input);
    }
    static int upboard_pinctrl_set_mux(struct pinctrl_dev *pctldev, unsigned int func_selector,
    unsigned int group_selector)
    {
    struct upboard_pinctrl *pctrl = pinctrl_dev_get_drvdata(pctldev);
    const struct upboard_pinctrl_data *pctrl_data = pctrl.pctrl_data;
    const struct upboard_pingroup *upgroups = pctrl_data.groups;
    struct group_desc *grp;
    unsigned int mode, i;
    int ret;
    grp = pinctrl_generic_get_group(pctldev, group_selector);
    if (!grp)
    return -EINVAL;
    for (i = 0; i < grp.grp.npins; i++) {
    mode = upgroups[group_selector].mode ?: upgroups[group_selector].modes[i];
    if (mode == UPBOARD_PIN_MODE_FUNCTION) {
    ret = upboard_pinctrl_set_function(pctldev, grp.grp.pins[i]);
    if (ret)
    return ret;
    continue;
    }
    ret = upboard_pinctrl_gpio_commit_enable(pctldev, grp.grp.pins[i]);
    if (ret)
    return ret;
    ret = upboard_pinctrl_gpio_commit_direction(pctldev, grp.grp.pins[i],
    mode == UPBOARD_PIN_MODE_GPIO_IN);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct pinmux_ops upboard_pinmux_ops = {
    .get_functions_count = pinmux_generic_get_function_count,
    .get_function_name = pinmux_generic_get_function_name,
    .get_function_groups = pinmux_generic_get_function_groups,
    .set_mux = upboard_pinctrl_set_mux,
    .gpio_request_enable = upboard_pinctrl_gpio_request_enable,
    .gpio_disable_free = upboard_pinctrl_gpio_disable_free,
    .gpio_set_direction = upboard_pinctrl_gpio_set_direction,
    };
#[no_mangle]
unsafe extern "C" fn upboard_pinctrl_pin_get_mode(pctldev: *mut pinctrl_dev, pin: c_uint) -> c_int {
    static int upboard_pinctrl_pin_get_mode(struct pinctrl_dev *pctldev, unsigned int pin)
    {
    struct upboard_pinctrl *pctrl = pinctrl_dev_get_drvdata(pctldev);
    struct upboard_pin *p = &pctrl.pins[pin];
    unsigned int val;
    int ret;
    if (p.funcbit) {
    ret = regmap_field_read(p.funcbit, &val);
    if (ret)
    return ret;
    if (val)
    return UPBOARD_PIN_MODE_FUNCTION;
    }
    ret = regmap_field_read(p.enbit, &val);
    if (ret)
    return ret;
    if (!val)
    return UPBOARD_PIN_MODE_DISABLED;
    ret = regmap_field_read(p.dirbit, &val);
    if (ret)
    return ret;
    return val ? UPBOARD_PIN_MODE_GPIO_IN : UPBOARD_PIN_MODE_GPIO_OUT;
    }
    static void upboard_pinctrl_dbg_show(struct pinctrl_dev *pctldev, struct seq_file *s,
    unsigned int offset)
    {
    int ret;
    ret = upboard_pinctrl_pin_get_mode(pctldev, offset);
    if (ret == UPBOARD_PIN_MODE_FUNCTION)
    seq_puts(s, "mode function ");
#[no_mangle]
pub unsafe extern "C" fn if(UPBOARD_PIN_MODE_DISABLED: ret ==) -> else {
    else if (ret == UPBOARD_PIN_MODE_DISABLED)
    seq_puts(s, "HIGH-Z ");
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    seq_puts(s, "N/A ");
    else
    seq_printf(s, "GPIO (%s) ", str_input_output(ret == UPBOARD_PIN_MODE_GPIO_IN));
    }
    static const struct pinctrl_ops upboard_pinctrl_ops = {
    .get_groups_count = pinctrl_generic_get_group_count,
    .get_group_name = pinctrl_generic_get_group_name,
    .get_group_pins = pinctrl_generic_get_group_pins,
    .pin_dbg_show = upboard_pinctrl_dbg_show,
    };
#[no_mangle]
unsafe extern "C" fn upboard_gpio_request(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int upboard_gpio_request(struct gpio_chip *gc, unsigned int offset)
    {
    struct gpiochip_fwd *fwd = gpiochip_get_data(gc);
    struct upboard_pinctrl *pctrl = gpiochip_fwd_get_data(fwd);
    let mut pin: c_uint = pctrl.pctrl_data.pin_header[offset];
    struct gpio_desc *desc;
    int ret;
    ret = pinctrl_gpio_request(gc, offset);
    if (ret)
    return ret;
    desc = gpiod_get_index(pctrl.dev, "external", pin, 0);
    if (IS_ERR(desc)) {
    pinctrl_gpio_free(gc, offset);
    return PTR_ERR(desc);
    }
    return gpiochip_fwd_desc_add(fwd, desc, offset);
    }
#[no_mangle]
unsafe extern "C" fn upboard_gpio_free(gc: *mut gpio_chip, offset: c_uint) {
    static void upboard_gpio_free(struct gpio_chip *gc, unsigned int offset)
    {
    struct gpiochip_fwd *fwd = gpiochip_get_data(gc);
    gpiochip_fwd_desc_free(fwd, offset);
    pinctrl_gpio_free(gc, offset);
    }
#[no_mangle]
unsafe extern "C" fn upboard_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int upboard_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    struct gpiochip_fwd *fwd = gpiochip_get_data(gc);
    struct upboard_pinctrl *pctrl = gpiochip_fwd_get_data(fwd);
    let mut pin: c_uint = pctrl.pctrl_data.pin_header[offset];
    int mode;
// If the pin is in function mode or high-z, input direction is returned
    mode = upboard_pinctrl_pin_get_mode(pctrl.pctldev, pin);
    if (mode < 0)
    return mode;
    if (mode == UPBOARD_PIN_MODE_GPIO_OUT)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn upboard_gpio_direction_input(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int upboard_gpio_direction_input(struct gpio_chip *gc, unsigned int offset)
    {
    struct gpiochip_fwd *fwd = gpiochip_get_data(gc);
    int ret;
    ret = pinctrl_gpio_direction_input(gc, offset);
    if (ret)
    return ret;
    return gpiochip_fwd_gpio_direction_input(fwd, offset);
    }
#[no_mangle]
unsafe extern "C" fn upboard_gpio_direction_output(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int upboard_gpio_direction_output(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct gpiochip_fwd *fwd = gpiochip_get_data(gc);
    int ret;
    ret = pinctrl_gpio_direction_output(gc, offset);
    if (ret)
    return ret;
    return gpiochip_fwd_gpio_direction_output(fwd, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn upboard_pinctrl_register_groups(pctrl: *mut upboard_pinctrl) -> c_int {
    static int upboard_pinctrl_register_groups(struct upboard_pinctrl *pctrl)
    {
    const struct upboard_pingroup *groups = pctrl.pctrl_data.groups;
    let mut ngroups: usize = pctrl.pctrl_data.ngroups;
    unsigned int i;
    int ret;
    for (i = 0; i < ngroups; i++) {
    ret = pinctrl_generic_add_group(pctrl.pctldev, groups[i].grp.name,
    groups[i].grp.pins, groups[i].grp.npins, pctrl);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn upboard_pinctrl_register_functions(pctrl: *mut upboard_pinctrl) -> c_int {
    static int upboard_pinctrl_register_functions(struct upboard_pinctrl *pctrl)
    {
    const struct pinfunction *funcs = pctrl.pctrl_data.funcs;
    let mut nfuncs: usize = pctrl.pctrl_data.nfuncs;
    unsigned int i;
    int ret;
    for (i = 0; i < nfuncs ; i++) {
    ret = pinmux_generic_add_function(pctrl.pctldev, funcs[i].name,
    funcs[i].groups, funcs[i].ngroups, core::ptr::null_mut());
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static const struct pinctrl_map pinctrl_map_apl01[] = {
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INT3452:00", "pwm0_grp", "pwm0"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INT3452:00", "pwm1_grp", "pwm1"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INT3452:00", "uart1_grp", "uart1"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INT3452:02", "i2c0_grp", "i2c0"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INT3452:02", "i2c1_grp", "i2c1"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INT3452:01", "ssp0_grp", "ssp0"),
    };
    static const struct upboard_pinctrl_map upboard_pinctrl_map_apl01 = {
    .maps = &pinctrl_map_apl01[0],
    .nmaps = ARRAY_SIZE(pinctrl_map_apl01),
    };
    static const struct pinctrl_map pinctrl_map_adl[] = {
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INTC1055:00", "i2c0_grp", "i2c0"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INTC1055:00", "i2c1_grp", "i2c1"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INTC1055:00", "pwm0_grp", "pwm0"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INTC1055:00", "uart1_grp", "uart1"),
    PIN_MAP_MUX_GROUP_DEFAULT("upboard-pinctrl", "INTC1055:00", "ssp2_grp", "ssp2"),
    };
    static const struct upboard_pinctrl_map upboard_pinctrl_map_adl = {
    .maps = &pinctrl_map_adl[0],
    .nmaps = ARRAY_SIZE(pinctrl_map_adl),
    };
    static const struct dmi_system_id dmi_platform_info[] = {
    {
// UP Squared
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "AAEON"),
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "UP-APL01"),
    },
    .driver_data = (void *)&upboard_pinctrl_map_apl01,
    },
    {
// UP Xtreme i12
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "AAEON"),
    DMI_EXACT_MATCH(DMI_BOARD_NAME, "UPX-ADLP01"),
    },
    .driver_data = (void *)&upboard_pinctrl_map_adl,
    },
    { }
    };
#[no_mangle]
unsafe extern "C" fn upboard_pinctrl_probe(pdev: *mut platform_device) -> c_int {
    static int upboard_pinctrl_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct upboard_fpga *fpga = dev_get_drvdata(dev.parent);
    const struct upboard_pinctrl_map *board_map;
    const struct dmi_system_id *dmi_id;
    struct pinctrl_desc *pctldesc;
    struct upboard_pinctrl *pctrl;
    struct upboard_pin *pins;
    struct gpiochip_fwd *fwd;
    struct pinctrl *pinctrl;
    struct gpio_chip *chip;
    unsigned int i;
    int ret;
    pctldesc = devm_kzalloc(dev, sizeof(*pctldesc), GFP_KERNEL);
    if (!pctldesc)
    return -ENOMEM;
    pctrl = devm_kzalloc(dev, sizeof(*pctrl), GFP_KERNEL);
    if (!pctrl)
    return -ENOMEM;
    switch (fpga.fpga_data.type) {
    case UPBOARD_UP_FPGA:
    pctldesc.pins = upboard_up_pins;
    pctldesc.npins = ARRAY_SIZE(upboard_up_pins);
    pctrl.pctrl_data = &upboard_up_pinctrl_data;
    break;
    case UPBOARD_UP2_FPGA:
    pctldesc.pins = upboard_up2_pins;
    pctldesc.npins = ARRAY_SIZE(upboard_up2_pins);
    pctrl.pctrl_data = &upboard_up2_pinctrl_data;
    break;
    default:
    return dev_err_probe(dev, -ENODEV, "Unsupported device type %d\n",
    fpga.fpga_data.type);
    }
    dmi_id = dmi_first_match(dmi_platform_info);
    if (!dmi_id)
    return dev_err_probe(dev, -ENODEV, "Unsupported board\n");
    board_map = (const struct upboard_pinctrl_map *)dmi_id.driver_data;
    pctldesc.name = dev_name(dev);
    pctldesc.owner = THIS_MODULE;
    pctldesc.pctlops = &upboard_pinctrl_ops;
    pctldesc.pmxops = &upboard_pinmux_ops;
    pctrl.dev = dev;
    pins = devm_kcalloc(dev, pctldesc.npins, sizeof(*pins), GFP_KERNEL);
    if (!pins)
    return -ENOMEM;
// Initialize pins
    for (i = 0; i < pctldesc.npins; i++) {
    const struct pinctrl_pin_desc *pin_desc = &pctldesc.pins[i];
    let mut regoff: c_uint = pin_desc.number / UPBOARD_REGISTER_SIZE;
    let mut lsb: c_uint = pin_desc.number % UPBOARD_REGISTER_SIZE;
    let mut fld_func: *mut reg_field  const = pin_desc.drv_data;
    struct upboard_pin *pin = &pins[i];
    let mut fldconf: reg_field = {};
    if (fld_func) {
    pin.funcbit = devm_regmap_field_alloc(dev, fpga.regmap, *fld_func);
    if (IS_ERR(pin.funcbit))
    return PTR_ERR(pin.funcbit);
    }
    fldconf.reg = UPBOARD_REG_GPIO_EN0 + regoff;
    fldconf.lsb = lsb;
    fldconf.msb = lsb;
    pin.enbit = devm_regmap_field_alloc(dev, fpga.regmap, fldconf);
    if (IS_ERR(pin.enbit))
    return PTR_ERR(pin.enbit);
    fldconf.reg = UPBOARD_REG_GPIO_DIR0 + regoff;
    fldconf.lsb = lsb;
    fldconf.msb = lsb;
    pin.dirbit = devm_regmap_field_alloc(dev, fpga.regmap, fldconf);
    if (IS_ERR(pin.dirbit))
    return PTR_ERR(pin.dirbit);
    }
    pctrl.pins = pins;
    ret = devm_pinctrl_register_and_init(dev, pctldesc, pctrl, &pctrl.pctldev);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register pinctrl\n");
    ret = upboard_pinctrl_register_groups(pctrl);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register groups\n");
    ret = upboard_pinctrl_register_functions(pctrl);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register functions\n");
    ret = devm_pinctrl_register_mappings(dev, board_map.maps, board_map.nmaps);
    if (ret)
    return ret;
    pinctrl = devm_pinctrl_get_select_default(dev);
    if (IS_ERR(pinctrl))
    return dev_err_probe(dev, PTR_ERR(pinctrl), "Failed to select pinctrl\n");
    ret = pinctrl_enable(pctrl.pctldev);
    if (ret)
    return ret;
    fwd = devm_gpiochip_fwd_alloc(dev, pctrl.pctrl_data.ngpio);
    if (IS_ERR(fwd))
    return dev_err_probe(dev, PTR_ERR(fwd), "Failed to allocate the gpiochip forwarder\n");
    chip = gpiochip_fwd_get_gpiochip(fwd);
    chip.request = upboard_gpio_request;
    chip.free = upboard_gpio_free;
    chip.get_direction = upboard_gpio_get_direction;
    chip.direction_output = upboard_gpio_direction_output;
    chip.direction_input = upboard_gpio_direction_input;
    ret = gpiochip_fwd_register(fwd, pctrl);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to register the gpiochip forwarder\n");
    return gpiochip_add_sparse_pin_range(chip, dev_name(dev), 0, pctrl.pctrl_data.pin_header,
    pctrl.pctrl_data.ngpio);
    }
    static struct platform_driver upboard_pinctrl_driver = {
    .driver = {
    .name = "upboard-pinctrl",
    },
    .probe = upboard_pinctrl_probe,
    };
    module_platform_driver(upboard_pinctrl_driver);
    MODULE_AUTHOR("Thomas Richard <thomas.richard@bootlin.com");
    MODULE_DESCRIPTION("UP Board HAT pin controller driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:upboard-pinctrl");
    MODULE_IMPORT_NS("GPIO_FORWARDER");
