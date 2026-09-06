//! Automatically rewritten from C to Rust
//! Source: sound/hda/codecs/side-codecs/cirrus_scodec_test.c
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
// KUnit test for the Cirrus side-codec library.
//
// Copyright (C) 2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.

    KUNIT_DEFINE_ACTION_WRAPPER(faux_device_destroy_wrapper, faux_device_destroy,
    struct faux_device *)
    KUNIT_DEFINE_ACTION_WRAPPER(device_remove_software_node_wrapper,
    device_remove_software_node,
    struct device *)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_scodec_test_gpio {
    pub pin_state: c_uint,
    pub chip: gpio_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_scodec_test_priv {
    pub amp_dev: *mut faux_device,
    pub gpio_dev: *mut faux_device,
    pub gpio_priv: *mut cirrus_scodec_test_gpio,
}

    static int cirrus_scodec_test_gpio_get_direction(struct gpio_chip *chip,
    unsigned int offset)
    {
    return GPIO_LINE_DIRECTION_IN;
    }
    static int cirrus_scodec_test_gpio_direction_in(struct gpio_chip *chip,
    unsigned int offset)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cirrus_scodec_test_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int cirrus_scodec_test_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    struct cirrus_scodec_test_gpio *gpio_priv = gpiochip_get_data(chip);
    return !!(gpio_priv.pin_state & BIT(offset));
    }
    static int cirrus_scodec_test_gpio_direction_out(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    return -EOPNOTSUPP;
    }
    static int cirrus_scodec_test_gpio_set(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    return -EOPNOTSUPP;
    }
    static int cirrus_scodec_test_gpio_set_config(struct gpio_chip *gc,
    unsigned int offset,
    unsigned long config)
    {
    switch (pinconf_to_config_param(config)) {
    case PIN_CONFIG_LEVEL:
    case PIN_CONFIG_OUTPUT_ENABLE:
    return -EOPNOTSUPP;
    default:
    return 0;
    }
    }
    static const struct gpio_chip cirrus_scodec_test_gpio_chip = {
    .label			= "cirrus_scodec_test_gpio",
    .owner			= THIS_MODULE,
    .request		= gpiochip_generic_request,
    .free			= gpiochip_generic_free,
    .get_direction		= cirrus_scodec_test_gpio_get_direction,
    .direction_input	= cirrus_scodec_test_gpio_direction_in,
    .get			= cirrus_scodec_test_gpio_get,
    .direction_output	= cirrus_scodec_test_gpio_direction_out,
    .set			= cirrus_scodec_test_gpio_set,
    .set_config		= cirrus_scodec_test_gpio_set_config,
    .base			= -1,
    .ngpio			= 32,
    };
// software_node referencing the gpio driver
    static const struct software_node cirrus_scodec_test_gpio_swnode = {
    .name = "cirrus_scodec_test_gpio",
    };
#[no_mangle]
unsafe extern "C" fn cirrus_scodec_test_gpio_probe(fdev: *mut faux_device) -> c_int {
    static int cirrus_scodec_test_gpio_probe(struct faux_device *fdev)
    {
    struct cirrus_scodec_test_gpio *gpio_priv;
    int ret;
    gpio_priv = devm_kzalloc(&fdev.dev, sizeof(*gpio_priv), GFP_KERNEL);
    if (!gpio_priv)
    return -ENOMEM;
    ret = device_add_software_node(&fdev.dev, &cirrus_scodec_test_gpio_swnode);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(&fdev.dev, device_remove_software_node_wrapper,
    &fdev.dev);
    if (ret)
    return ret;
// GPIO core modifies our struct gpio_chip so use a copy
    gpio_priv.chip = cirrus_scodec_test_gpio_chip;
    gpio_priv.chip.parent = &fdev.dev;
    ret = devm_gpiochip_add_data(&fdev.dev, &gpio_priv.chip, gpio_priv);
    if (ret)
    return dev_err_probe(&fdev.dev, ret, "Failed to add gpiochip\n");
    dev_set_drvdata(&fdev.dev, gpio_priv);
    return 0;
    }
    static const struct faux_device_ops cirrus_scodec_test_gpio_driver_ops = {
    .probe		= cirrus_scodec_test_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn cirrus_scodec_test_create_gpio(test: *mut kunit) {
    static void cirrus_scodec_test_create_gpio(struct kunit *test)
    {
    struct cirrus_scodec_test_priv *priv = test.priv;
    priv.gpio_dev = faux_device_create("cirrus_scodec_test_gpio_drv", core::ptr::null_mut(),
    &cirrus_scodec_test_gpio_driver_ops);
    KUNIT_ASSERT_NOT_NULL(test, priv.gpio_dev);
    KUNIT_ASSERT_EQ(test, 0, kunit_add_action_or_reset(test,
    faux_device_destroy_wrapper,
    priv.gpio_dev));
    priv.gpio_priv = dev_get_drvdata(&priv.gpio_dev.dev);
    KUNIT_ASSERT_NOT_NULL(test, priv.gpio_priv);
    }
    static void cirrus_scodec_test_set_gpio_ref_arg(struct software_node_ref_args *arg,
    int gpio_num)
    {
    struct software_node_ref_args template =
    SOFTWARE_NODE_REFERENCE(&cirrus_scodec_test_gpio_swnode, gpio_num, 0);
// arg = template;
    }
    static int cirrus_scodec_test_set_spkid_swnode(struct kunit *test,
    struct device *dev,
    struct software_node_ref_args *args,
    int num_args)
    {
    const struct property_entry props_template[] = {
    PROPERTY_ENTRY_REF_ARRAY_LEN("spk-id-gpios", args, num_args),
    { }
    };
    struct property_entry *props;
    struct software_node *node;
    node = kunit_kzalloc(test, sizeof(*node), GFP_KERNEL);
    if (!node)
    return -ENOMEM;
    props = kunit_kzalloc(test, sizeof(props_template), GFP_KERNEL);
    if (!props)
    return -ENOMEM;
    memcpy(props, props_template, sizeof(props_template));
    node.properties = props;
    return device_add_software_node(dev, node);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cirrus_scodec_test_spkid_param {
    pub num_amps: c_int,
    pub gpios_per_amp: c_int,
    pub num_amps_sharing: c_int,
}

#[no_mangle]
unsafe extern "C" fn cirrus_scodec_test_spkid_parse(test: *mut kunit) {
    static void cirrus_scodec_test_spkid_parse(struct kunit *test)
    {
    struct cirrus_scodec_test_priv *priv = test.priv;
    const struct cirrus_scodec_test_spkid_param *param = test.param_value;
    let mut num_spk_id_refs: c_int = param.num_amps * param.gpios_per_amp;
    struct software_node_ref_args *refs;
    struct device *dev = &priv.amp_dev.dev;
    unsigned int v;
    int i, ret;
    refs = kunit_kcalloc(test, num_spk_id_refs, sizeof(*refs), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, refs);
    for (i = 0, v = 0; i < num_spk_id_refs; ) {
    cirrus_scodec_test_set_gpio_ref_arg(&refs[i++], v++);
//
// If amps are sharing GPIOs repeat the last set of
// GPIOs until we've done that number of amps.
// We have done all GPIOs for an amp when i is a multiple
// of gpios_per_amp.
// We have done all amps sharing the same GPIOs when i is
// a multiple of (gpios_per_amp * num_amps_sharing).
//
    if (!(i % param.gpios_per_amp) &&
    (i % (param.gpios_per_amp * param.num_amps_sharing)))
    v -= param.gpios_per_amp;
    }
    ret = cirrus_scodec_test_set_spkid_swnode(test, dev, refs, num_spk_id_refs);
    KUNIT_EXPECT_EQ_MSG(test, ret, 0, "Failed to add swnode\n");
    for (i = 0; i < param.num_amps; ++i) {
    for (v = 0; v < (1 << param.gpios_per_amp); ++v) {
// Set only the GPIO bits used by this amp
    priv.gpio_priv.pin_state =
    v << (param.gpios_per_amp * (i / param.num_amps_sharing));
    ret = cirrus_scodec_get_speaker_id(dev, i, param.num_amps, -1);
    KUNIT_EXPECT_EQ_MSG(test, ret, v,
    "get_speaker_id failed amp:%d pin_state:%#x\n",
    i, priv.gpio_priv.pin_state);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn cirrus_scodec_test_no_spkid(test: *mut kunit) {
    static void cirrus_scodec_test_no_spkid(struct kunit *test)
    {
    struct cirrus_scodec_test_priv *priv = test.priv;
    struct device *dev = &priv.amp_dev.dev;
    int ret;
    ret = cirrus_scodec_get_speaker_id(dev, 0, 4, -1);
    KUNIT_EXPECT_EQ(test, ret, -ENOENT);
    }
#[no_mangle]
unsafe extern "C" fn cirrus_scodec_test_case_init(test: *mut kunit) -> c_int {
    static int cirrus_scodec_test_case_init(struct kunit *test)
    {
    struct cirrus_scodec_test_priv *priv;
    priv = kunit_kzalloc(test, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    test.priv = priv;
// Create dummy GPIO
    cirrus_scodec_test_create_gpio(test);
// Create dummy amp driver dev
    priv.amp_dev = faux_device_create("cirrus_scodec_test_amp_drv", core::ptr::null_mut(), core::ptr::null_mut());
    KUNIT_ASSERT_NOT_NULL(test, priv.amp_dev);
    KUNIT_ASSERT_EQ(test, 0, kunit_add_action_or_reset(test,
    faux_device_destroy_wrapper,
    priv.amp_dev));
    return 0;
    }
    static const struct cirrus_scodec_test_spkid_param cirrus_scodec_test_spkid_param_cases[] = {
    { .num_amps = 2, .gpios_per_amp = 1, .num_amps_sharing = 1 },
    { .num_amps = 2, .gpios_per_amp = 2, .num_amps_sharing = 1 },
    { .num_amps = 2, .gpios_per_amp = 3, .num_amps_sharing = 1 },
    { .num_amps = 2, .gpios_per_amp = 4, .num_amps_sharing = 1 },
    { .num_amps = 3, .gpios_per_amp = 1, .num_amps_sharing = 1 },
    { .num_amps = 3, .gpios_per_amp = 2, .num_amps_sharing = 1 },
    { .num_amps = 3, .gpios_per_amp = 3, .num_amps_sharing = 1 },
    { .num_amps = 3, .gpios_per_amp = 4, .num_amps_sharing = 1 },
    { .num_amps = 4, .gpios_per_amp = 1, .num_amps_sharing = 1 },
    { .num_amps = 4, .gpios_per_amp = 2, .num_amps_sharing = 1 },
    { .num_amps = 4, .gpios_per_amp = 3, .num_amps_sharing = 1 },
    { .num_amps = 4, .gpios_per_amp = 4, .num_amps_sharing = 1 },
// Same GPIO shared by all amps
    { .num_amps = 2, .gpios_per_amp = 1, .num_amps_sharing = 2 },
    { .num_amps = 2, .gpios_per_amp = 2, .num_amps_sharing = 2 },
    { .num_amps = 2, .gpios_per_amp = 3, .num_amps_sharing = 2 },
    { .num_amps = 2, .gpios_per_amp = 4, .num_amps_sharing = 2 },
    { .num_amps = 3, .gpios_per_amp = 1, .num_amps_sharing = 3 },
    { .num_amps = 3, .gpios_per_amp = 2, .num_amps_sharing = 3 },
    { .num_amps = 3, .gpios_per_amp = 3, .num_amps_sharing = 3 },
    { .num_amps = 3, .gpios_per_amp = 4, .num_amps_sharing = 3 },
    { .num_amps = 4, .gpios_per_amp = 1, .num_amps_sharing = 4 },
    { .num_amps = 4, .gpios_per_amp = 2, .num_amps_sharing = 4 },
    { .num_amps = 4, .gpios_per_amp = 3, .num_amps_sharing = 4 },
    { .num_amps = 4, .gpios_per_amp = 4, .num_amps_sharing = 4 },
// Two sets of shared GPIOs
    { .num_amps = 4, .gpios_per_amp = 1, .num_amps_sharing = 2 },
    { .num_amps = 4, .gpios_per_amp = 2, .num_amps_sharing = 2 },
    { .num_amps = 4, .gpios_per_amp = 3, .num_amps_sharing = 2 },
    { .num_amps = 4, .gpios_per_amp = 4, .num_amps_sharing = 2 },
    };
    static void cirrus_scodec_test_spkid_param_desc(const struct cirrus_scodec_test_spkid_param *param,
    char *desc)
    {
    snprintf(desc, KUNIT_PARAM_DESC_SIZE, "amps:%d gpios_per_amp:%d num_amps_sharing:%d",
    param.num_amps, param.gpios_per_amp, param.num_amps_sharing);
    }
    KUNIT_ARRAY_PARAM(cirrus_scodec_test_spkid, cirrus_scodec_test_spkid_param_cases,
    cirrus_scodec_test_spkid_param_desc);
    static struct kunit_case cirrus_scodec_test_cases[] = {
    KUNIT_CASE_PARAM(cirrus_scodec_test_spkid_parse, cirrus_scodec_test_spkid_gen_params),
    KUNIT_CASE(cirrus_scodec_test_no_spkid),
    { } /* terminator */
    };
    static struct kunit_suite cirrus_scodec_test_suite = {
    .name = "snd-hda-cirrus-scodec-test",
    .init = cirrus_scodec_test_case_init,
    .test_cases = cirrus_scodec_test_cases,
    };
    kunit_test_suite(cirrus_scodec_test_suite);
    MODULE_IMPORT_NS("SND_HDA_CIRRUS_SCODEC");
    MODULE_DESCRIPTION("KUnit test for the Cirrus side-codec library");
    MODULE_AUTHOR("Richard Fitzgerald <rf@opensource.cirrus.com>");
    MODULE_LICENSE("GPL");
