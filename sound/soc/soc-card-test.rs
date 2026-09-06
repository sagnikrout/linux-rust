//! Automatically rewritten from C to Rust
//! Source: sound/soc/soc-card-test.c
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
// Copyright (C) 2024 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_card_test_priv {
    pub card_dev: *mut device,
    pub card: *mut snd_soc_card,
}

    static const struct snd_kcontrol_new test_card_controls[] = {
    SOC_SINGLE("Fee", SND_SOC_NOPM, 0, 1, 0),
    SOC_SINGLE("Fi", SND_SOC_NOPM, 1, 1, 0),
    SOC_SINGLE("Fo", SND_SOC_NOPM, 2, 1, 0),
    SOC_SINGLE("Fum", SND_SOC_NOPM, 3, 1, 0),
    SOC_SINGLE("Left Fee", SND_SOC_NOPM, 4, 1, 0),
    SOC_SINGLE("Right Fee", SND_SOC_NOPM, 5, 1, 0),
    SOC_SINGLE("Left Fi", SND_SOC_NOPM, 6, 1, 0),
    SOC_SINGLE("Right Fi", SND_SOC_NOPM, 7, 1, 0),
    SOC_SINGLE("Left Fo", SND_SOC_NOPM, 8, 1, 0),
    SOC_SINGLE("Right Fo", SND_SOC_NOPM, 9, 1, 0),
    SOC_SINGLE("Left Fum", SND_SOC_NOPM, 10, 1, 0),
    SOC_SINGLE("Right Fum", SND_SOC_NOPM, 11, 1, 0),
    };
#[no_mangle]
unsafe extern "C" fn test_snd_soc_card_get_kcontrol(test: *mut kunit) {
    static void test_snd_soc_card_get_kcontrol(struct kunit *test)
    {
    struct soc_card_test_priv *priv = test.priv;
    struct snd_soc_card *card = priv.card;
    struct snd_kcontrol *kc;
    struct soc_mixer_control *mc;
    int i, ret;
    ret = snd_soc_add_card_controls(card, test_card_controls, ARRAY_SIZE(test_card_controls));
    KUNIT_ASSERT_EQ(test, ret, 0);
// Look up every control
    for (i = 0; i < ARRAY_SIZE(test_card_controls); ++i) {
    kc = snd_soc_card_get_kcontrol(card, test_card_controls[i].name);
    KUNIT_EXPECT_NOT_ERR_OR_NULL_MSG(test, kc, "Failed to find '%s'\n",
    test_card_controls[i].name);
    if (!kc)
    continue;
// Test that it is the correct control
    mc = (struct soc_mixer_control *)kc.private_value;
    KUNIT_EXPECT_EQ_MSG(test, mc.shift, i, "For '%s'\n", test_card_controls[i].name);
    }
// Test some names that should not be found
    kc = snd_soc_card_get_kcontrol(card, "None");
    KUNIT_EXPECT_NULL(test, kc);
    kc = snd_soc_card_get_kcontrol(card, "Left None");
    KUNIT_EXPECT_NULL(test, kc);
    kc = snd_soc_card_get_kcontrol(card, "Left");
    KUNIT_EXPECT_NULL(test, kc);
    kc = snd_soc_card_get_kcontrol(card, core::ptr::null_mut());
    KUNIT_EXPECT_NULL(test, kc);
    }
#[no_mangle]
unsafe extern "C" fn soc_card_test_case_init(test: *mut kunit) -> c_int {
    static int soc_card_test_case_init(struct kunit *test)
    {
    struct soc_card_test_priv *priv;
    int ret;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    test.priv = priv;
    priv.card = kunit_kzalloc(test, sizeof(*priv.card), GFP_KERNEL);
    if (!priv.card)
    return -ENOMEM;
    priv.card_dev = kunit_device_register(test, "sound-soc-card-test");
    priv.card_dev = get_device(priv.card_dev);
    if (!priv.card_dev)
    return -ENODEV;
    priv.card.name = "soc-card-test";
    priv.card.dev = priv.card_dev;
    priv.card.owner = THIS_MODULE;
    ret = snd_soc_register_card(priv.card);
    if (ret) {
    put_device(priv.card_dev);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn soc_card_test_case_exit(test: *mut kunit) {
    static void soc_card_test_case_exit(struct kunit *test)
    {
    struct soc_card_test_priv *priv = test.priv;
    if (priv.card)
    snd_soc_unregister_card(priv.card);
    if (priv.card_dev)
    put_device(priv.card_dev);
    }
    static struct kunit_case soc_card_test_cases[] = {
    KUNIT_CASE(test_snd_soc_card_get_kcontrol),
    {}
    };
    static struct kunit_suite soc_card_test_suite = {
    .name = "soc-card",
    .test_cases = soc_card_test_cases,
    .init = soc_card_test_case_init,
    .exit = soc_card_test_case_exit,
    };
    kunit_test_suites(&soc_card_test_suite);
    MODULE_DESCRIPTION("ASoC soc-card KUnit test");
    MODULE_LICENSE("GPL");
