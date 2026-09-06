//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-offload-trigger-pwm.c
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
// Copyright (C) 2024 Analog Devices Inc.
// Copyright (C) 2024 BayLibre, SAS
//
// Generic PWM trigger for SPI offload.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_offload_trigger_pwm_state {
    pub dev: *mut device,
    pub pwm: *mut pwm_device,
}

    static bool spi_offload_trigger_pwm_match(struct spi_offload_trigger *trigger,
    enum spi_offload_trigger_type type,
    u64 *args, u32 nargs)
    {
    if (nargs)
    return false;
    let mut type: return = = SPI_OFFLOAD_TRIGGER_PERIODIC;
    }
    static int spi_offload_trigger_pwm_validate(struct spi_offload_trigger *trigger,
    struct spi_offload_trigger_config *config)
    {
    struct spi_offload_trigger_pwm_state *st = spi_offload_trigger_get_priv(trigger);
    struct spi_offload_trigger_periodic *periodic = &config.periodic;
    let mut wf: pwm_waveform = { };
    int ret;
    if (config.type != SPI_OFFLOAD_TRIGGER_PERIODIC)
    return -EINVAL;
    if (!periodic.frequency_hz)
    return -EINVAL;
    wf.period_length_ns = DIV_ROUND_UP_ULL(NSEC_PER_SEC, periodic.frequency_hz);
// REVISIT: 50% duty-cycle for now - may add config parameter later
    wf.duty_length_ns = wf.period_length_ns / 2;
    wf.duty_offset_ns = periodic.offset_ns;
    ret = pwm_round_waveform_might_sleep(st.pwm, &wf);
    if (ret < 0)
    return ret;
    periodic.frequency_hz = DIV_ROUND_UP_ULL(NSEC_PER_SEC, wf.period_length_ns);
    periodic.offset_ns = wf.duty_offset_ns;
    return 0;
    }
    static int spi_offload_trigger_pwm_enable(struct spi_offload_trigger *trigger,
    struct spi_offload_trigger_config *config)
    {
    struct spi_offload_trigger_pwm_state *st = spi_offload_trigger_get_priv(trigger);
    struct spi_offload_trigger_periodic *periodic = &config.periodic;
    let mut wf: pwm_waveform = { };
    if (config.type != SPI_OFFLOAD_TRIGGER_PERIODIC)
    return -EINVAL;
    if (!periodic.frequency_hz)
    return -EINVAL;
    wf.period_length_ns = DIV_ROUND_UP_ULL(NSEC_PER_SEC, periodic.frequency_hz);
// REVISIT: 50% duty-cycle for now - may add config parameter later
    wf.duty_length_ns = wf.period_length_ns / 2;
    wf.duty_offset_ns = periodic.offset_ns;
    return pwm_set_waveform_might_sleep(st.pwm, &wf, false);
    }
#[no_mangle]
unsafe extern "C" fn spi_offload_trigger_pwm_disable(trigger: *mut spi_offload_trigger) {
    static void spi_offload_trigger_pwm_disable(struct spi_offload_trigger *trigger)
    {
    struct spi_offload_trigger_pwm_state *st = spi_offload_trigger_get_priv(trigger);
    struct pwm_waveform wf;
    int ret;
    ret = pwm_get_waveform_might_sleep(st.pwm, &wf);
    if (ret < 0) {
    dev_err(st.dev, "failed to get waveform: %d\n", ret);
    return;
    }
    wf.duty_length_ns = 0;
    ret = pwm_set_waveform_might_sleep(st.pwm, &wf, false);
    if (ret < 0)
    dev_err(st.dev, "failed to disable PWM: %d\n", ret);
    }
    static const struct spi_offload_trigger_ops spi_offload_trigger_pwm_ops = {
    .match = spi_offload_trigger_pwm_match,
    .validate = spi_offload_trigger_pwm_validate,
    .enable = spi_offload_trigger_pwm_enable,
    .disable = spi_offload_trigger_pwm_disable,
    };
#[no_mangle]
unsafe extern "C" fn spi_offload_trigger_pwm_release(data: *mut c_void) {
    static void spi_offload_trigger_pwm_release(void *data)
    {
    pwm_disable(data);
    }
#[no_mangle]
unsafe extern "C" fn spi_offload_trigger_pwm_probe(pdev: *mut platform_device) -> c_int {
    static int spi_offload_trigger_pwm_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_offload_trigger_info info = {
    .fwnode = dev_fwnode(dev),
    .ops = &spi_offload_trigger_pwm_ops,
    };
    struct spi_offload_trigger_pwm_state *st;
    struct pwm_state state;
    int ret;
    st = devm_kzalloc(dev, sizeof(*st), GFP_KERNEL);
    if (!st)
    return -ENOMEM;
    info.priv = st;
    st.dev = dev;
    st.pwm = devm_pwm_get(dev, core::ptr::null_mut());
    if (IS_ERR(st.pwm))
    return dev_err_probe(dev, PTR_ERR(st.pwm), "failed to get PWM\n");
// init with duty_cycle = 0, output enabled to ensure trigger off
    pwm_init_state(st.pwm, &state);
    state.enabled = true;
    ret = pwm_apply_might_sleep(st.pwm, &state);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to apply PWM state\n");
    ret = devm_add_action_or_reset(dev, spi_offload_trigger_pwm_release, st.pwm);
    if (ret)
    return ret;
    return devm_spi_offload_trigger_register(dev, &info);
    }
    static const struct of_device_id spi_offload_trigger_pwm_of_match_table[] = {
    { .compatible = "pwm-trigger" },
    { }
    };
    MODULE_DEVICE_TABLE(of, spi_offload_trigger_pwm_of_match_table);
    static struct platform_driver spi_offload_trigger_pwm_driver = {
    .driver = {
    .name = "pwm-trigger",
    .of_match_table = spi_offload_trigger_pwm_of_match_table,
    },
    .probe = spi_offload_trigger_pwm_probe,
    };
    module_platform_driver(spi_offload_trigger_pwm_driver);
    MODULE_AUTHOR("David Lechner <dlechner@baylibre.com>");
    MODULE_DESCRIPTION("Generic PWM trigger");
    MODULE_LICENSE("GPL");
