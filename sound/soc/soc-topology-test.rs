//! Automatically rewritten from C to Rust
//! Source: sound/soc/soc-topology-test.c
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
// soc-topology-test.c  --  ALSA SoC Topology Kernel Unit Tests
//
// Copyright(c) 2021 Intel Corporation.
//

// ===== HELPER FUNCTIONS ===================================================
//
// snd_soc_component needs device to operate on (primarily for prints), create
// fake one, as we don't register with PCI or anything else
// device_driver name is used in some of the prints (fmt_single_name) so
// we also mock up minimal one
//
    static struct device *test_dev;
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_init(test: *mut kunit) -> c_int {
    static int snd_soc_tplg_test_init(struct kunit *test)
    {
    test_dev = kunit_device_register(test, "sound-soc-topology-test");
    test_dev = get_device(test_dev);
    if (!test_dev)
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_exit(test: *mut kunit) {
    static void snd_soc_tplg_test_exit(struct kunit *test)
    {
    put_device(test_dev);
    }
//
// helper struct we use when registering component, as we load topology during
// component probe, we need to pass struct kunit somehow to probe function, so
// we can report test result
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kunit_soc_component {
    pub kunit: *mut kunit,
    pub /: *mut *mut int expect; / what result we expect when loading topology,
    pub card: snd_soc_card,
    pub fw: firmware,
}

#[no_mangle]
unsafe extern "C" fn d_probe(component: *mut snd_soc_component) -> c_int {
    static int d_probe(struct snd_soc_component *component)
    {
    struct kunit_soc_component *kunit_comp = snd_soc_component_to_priv(component);
    int ret;
    ret = snd_soc_tplg_component_load(component, core::ptr::null_mut(), &kunit_comp.fw);
    KUNIT_EXPECT_EQ_MSG(kunit_comp.kunit, kunit_comp.expect, ret,
    "Failed topology load");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn d_remove(component: *mut snd_soc_component) {
    static void d_remove(struct snd_soc_component *component)
    {
    struct kunit_soc_component *kunit_comp = snd_soc_component_to_priv(component);
    int ret;
    ret = snd_soc_tplg_component_remove(component);
    KUNIT_EXPECT_EQ(kunit_comp.kunit, 0, ret);
    }
//
// ASoC minimal boiler plate
//
    SND_SOC_DAILINK_DEF(dummy, DAILINK_COMP_ARRAY(COMP_DUMMY()));
    SND_SOC_DAILINK_DEF(platform, DAILINK_COMP_ARRAY(COMP_PLATFORM("sound-soc-topology-test")));
    static struct snd_soc_dai_link kunit_dai_links[] = {
    {
    .name = "KUNIT Audio Port",
    .id = 0,
    .stream_name = "Audio Playback/Capture",
    .nonatomic = 1,
    .dynamic = 1,
    .trigger = {SND_SOC_DPCM_TRIGGER_POST, SND_SOC_DPCM_TRIGGER_POST},
    SND_SOC_DAILINK_REG(dummy, dummy, platform),
    },
    };
    static const struct snd_soc_component_driver test_component = {
    .name = "sound-soc-topology-test",
    .probe = d_probe,
    .remove = d_remove,
    };
// ===== TOPOLOGY TEMPLATES =================================================
// Structural representation of topology which can be generated with:
// $ touch empty
// $ alsatplg -c empty -o empty.tplg
// $ xxd -i empty.tplg
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tplg_tmpl_001 {
    pub header: snd_soc_tplg_hdr,
    pub manifest: snd_soc_tplg_manifest,
    pub __packed: },
    static struct tplg_tmpl_001 tplg_tmpl_empty = {
    .header = {
    .magic = cpu_to_le32(SND_SOC_TPLG_MAGIC),
    .abi = cpu_to_le32(5),
    .version = 0,
    .type = cpu_to_le32(SND_SOC_TPLG_TYPE_MANIFEST),
    .size = cpu_to_le32(sizeof(struct snd_soc_tplg_hdr)),
    .vendor_type = 0,
    .payload_size = cpu_to_le32(sizeof(struct snd_soc_tplg_manifest)),
    .index = 0,
    .count = cpu_to_le32(1),
    },
    .manifest = {
    .size = cpu_to_le32(sizeof(struct snd_soc_tplg_manifest)),
// rest of fields is 0
    },
}

// Structural representation of topology containing SectionPCM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tplg_tmpl_002 {
    pub header: snd_soc_tplg_hdr,
    pub manifest: snd_soc_tplg_manifest,
    pub pcm_header: snd_soc_tplg_hdr,
    pub pcm: snd_soc_tplg_pcm,
    pub __packed: },
    static struct tplg_tmpl_002 tplg_tmpl_with_pcm = {
    .header = {
    .magic = cpu_to_le32(SND_SOC_TPLG_MAGIC),
    .abi = cpu_to_le32(5),
    .version = 0,
    .type = cpu_to_le32(SND_SOC_TPLG_TYPE_MANIFEST),
    .size = cpu_to_le32(sizeof(struct snd_soc_tplg_hdr)),
    .vendor_type = 0,
    .payload_size = cpu_to_le32(sizeof(struct snd_soc_tplg_manifest)),
    .index = 0,
    .count = cpu_to_le32(1),
    },
    .manifest = {
    .size = cpu_to_le32(sizeof(struct snd_soc_tplg_manifest)),
    .pcm_elems = cpu_to_le32(1),
// rest of fields is 0
    },
    .pcm_header = {
    .magic = cpu_to_le32(SND_SOC_TPLG_MAGIC),
    .abi = cpu_to_le32(5),
    .version = 0,
    .type = cpu_to_le32(SND_SOC_TPLG_TYPE_PCM),
    .size = cpu_to_le32(sizeof(struct snd_soc_tplg_hdr)),
    .vendor_type = 0,
    .payload_size = cpu_to_le32(sizeof(struct snd_soc_tplg_pcm)),
    .index = 0,
    .count = cpu_to_le32(1),
    },
    .pcm = {
    .size = cpu_to_le32(sizeof(struct snd_soc_tplg_pcm)),
    .pcm_name = "KUNIT Audio",
    .dai_name = "kunit-audio-dai",
    .pcm_id = 0,
    .dai_id = 0,
    .playback = cpu_to_le32(1),
    .capture = cpu_to_le32(1),
    .compress = 0,
    .stream = {
    [0] = {
    .channels = cpu_to_le32(2),
    },
    [1] = {
    .channels = cpu_to_le32(2),
    },
    },
    .num_streams = 0,
    .caps = {
    [0] = {
    .name = "kunit-audio-playback",
    .channels_min = cpu_to_le32(2),
    .channels_max = cpu_to_le32(2),
    },
    [1] = {
    .name = "kunit-audio-capture",
    .channels_min = cpu_to_le32(2),
    .channels_max = cpu_to_le32(2),
    },
    },
    .flag_mask = 0,
    .flags = 0,
    .priv = { 0 },
    },
}

// ===== TEST CASES =========================================================
// TEST CASE
// Test passing NULL component as parameter to snd_soc_tplg_component_load
//
// need to override generic probe function with one using NULL when calling
// topology load during component initialization, we don't need .remove
// handler as load should fail
//
#[no_mangle]
unsafe extern "C" fn d_probe_null_comp(component: *mut snd_soc_component) -> c_int {
    static int d_probe_null_comp(struct snd_soc_component *component)
    {
    struct kunit_soc_component *kunit_comp = snd_soc_component_to_priv(component);
    int ret;
// instead of passing component pointer as first argument, pass NULL here
    ret = snd_soc_tplg_component_load(core::ptr::null_mut(), core::ptr::null_mut(), &kunit_comp.fw);
    KUNIT_EXPECT_EQ_MSG(kunit_comp.kunit, kunit_comp.expect, ret,
    "Failed topology load");
    return 0;
    }
    static const struct snd_soc_component_driver test_component_null_comp = {
    .name = "sound-soc-topology-test",
    .probe = d_probe_null_comp,
    };
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_with_null_comp(test: *mut kunit) {
    static void snd_soc_tplg_test_load_with_null_comp(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = -EINVAL; /* expect failure */
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component_null_comp, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    snd_soc_unregister_component(test_dev);
    }
// TEST CASE
// Test passing NULL ops as parameter to snd_soc_tplg_component_load
//
// NULL ops is default case, we pass empty topology (fw), so we don't have
// anything to parse and just do nothing, which results in return 0; from
// calling soc_tplg_dapm_complete in soc_tplg_process_headers
//
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_with_null_ops(test: *mut kunit) {
    static void snd_soc_tplg_test_load_with_null_ops(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = 0; /* expect success */
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    snd_soc_unregister_component(test_dev);
    }
// TEST CASE
// Test passing NULL fw as parameter to snd_soc_tplg_component_load
//
// need to override generic probe function with one using NULL pointer to fw
// when calling topology load during component initialization, we don't need
// .remove handler as load should fail
//
#[no_mangle]
unsafe extern "C" fn d_probe_null_fw(component: *mut snd_soc_component) -> c_int {
    static int d_probe_null_fw(struct snd_soc_component *component)
    {
    struct kunit_soc_component *kunit_comp = snd_soc_component_to_priv(component);
    int ret;
// instead of passing fw pointer as third argument, pass NULL here
    ret = snd_soc_tplg_component_load(component, core::ptr::null_mut(), core::ptr::null_mut());
    KUNIT_EXPECT_EQ_MSG(kunit_comp.kunit, kunit_comp.expect, ret,
    "Failed topology load");
    return 0;
    }
    static const struct snd_soc_component_driver test_component_null_fw = {
    .name = "sound-soc-topology-test",
    .probe = d_probe_null_fw,
    };
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_with_null_fw(test: *mut kunit) {
    static void snd_soc_tplg_test_load_with_null_fw(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = -EINVAL; /* expect failure */
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component_null_fw, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    snd_soc_unregister_component(test_dev);
    }
// TEST CASE
// Test passing "empty" topology file
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg(test: *mut kunit) {
    static void snd_soc_tplg_test_load_empty_tplg(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    struct tplg_tmpl_001 *data;
    int size;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = 0; /* expect success */
    size = sizeof(tplg_tmpl_empty);
    data = kunit_kzalloc(kunit_comp.kunit, size, GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(kunit_comp.kunit, data);
    memcpy(data, &tplg_tmpl_empty, sizeof(tplg_tmpl_empty));
    kunit_comp.fw.data = (u8 *)data;
    kunit_comp.fw.size = size;
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    snd_soc_unregister_component(test_dev);
    }
// TEST CASE
// Test "empty" topology file, but with bad "magic"
// In theory we could loop through all possible bad values, but it takes too
// long, so just use SND_SOC_TPLG_MAGIC + 1
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg_bad_magic(test: *mut kunit) {
    static void snd_soc_tplg_test_load_empty_tplg_bad_magic(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    struct tplg_tmpl_001 *data;
    int size;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = -EINVAL; /* expect failure */
    size = sizeof(tplg_tmpl_empty);
    data = kunit_kzalloc(kunit_comp.kunit, size, GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(kunit_comp.kunit, data);
    memcpy(data, &tplg_tmpl_empty, sizeof(tplg_tmpl_empty));
//
// override abi
// any value != magic number is wrong
//
    data.header.magic = cpu_to_le32(SND_SOC_TPLG_MAGIC + 1);
    kunit_comp.fw.data = (u8 *)data;
    kunit_comp.fw.size = size;
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    snd_soc_unregister_component(test_dev);
    }
// TEST CASE
// Test "empty" topology file, but with bad "abi"
// In theory we could loop through all possible bad values, but it takes too
// long, so just use SND_SOC_TPLG_ABI_VERSION + 1
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg_bad_abi(test: *mut kunit) {
    static void snd_soc_tplg_test_load_empty_tplg_bad_abi(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    struct tplg_tmpl_001 *data;
    int size;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = -EINVAL; /* expect failure */
    size = sizeof(tplg_tmpl_empty);
    data = kunit_kzalloc(kunit_comp.kunit, size, GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(kunit_comp.kunit, data);
    memcpy(data, &tplg_tmpl_empty, sizeof(tplg_tmpl_empty));
//
// override abi
// any value != accepted range is wrong
//
    data.header.abi = cpu_to_le32(SND_SOC_TPLG_ABI_VERSION + 1);
    kunit_comp.fw.data = (u8 *)data;
    kunit_comp.fw.size = size;
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    snd_soc_unregister_component(test_dev);
    }
// TEST CASE
// Test "empty" topology file, but with bad "size"
// In theory we could loop through all possible bad values, but it takes too
// long, so just use sizeof(struct snd_soc_tplg_hdr) + 1
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg_bad_size(test: *mut kunit) {
    static void snd_soc_tplg_test_load_empty_tplg_bad_size(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    struct tplg_tmpl_001 *data;
    int size;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = -EINVAL; /* expect failure */
    size = sizeof(tplg_tmpl_empty);
    data = kunit_kzalloc(kunit_comp.kunit, size, GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(kunit_comp.kunit, data);
    memcpy(data, &tplg_tmpl_empty, sizeof(tplg_tmpl_empty));
//
// override size
// any value != struct size is wrong
//
    data.header.size = cpu_to_le32(sizeof(struct snd_soc_tplg_hdr) + 1);
    kunit_comp.fw.data = (u8 *)data;
    kunit_comp.fw.size = size;
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    snd_soc_unregister_component(test_dev);
    }
// TEST CASE
// Test "empty" topology file, but with bad "payload_size"
// In theory we could loop through all possible bad values, but it takes too
// long, so just use the known wrong one
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_empty_tplg_bad_payload_size(test: *mut kunit) {
    static void snd_soc_tplg_test_load_empty_tplg_bad_payload_size(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    struct tplg_tmpl_001 *data;
    int size;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = -EINVAL; /* expect failure */
    size = sizeof(tplg_tmpl_empty);
    data = kunit_kzalloc(kunit_comp.kunit, size, GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(kunit_comp.kunit, data);
    memcpy(data, &tplg_tmpl_empty, sizeof(tplg_tmpl_empty));
//
// override payload size
// there is only explicit check for 0, so check with it, other values
// are handled by just not reading behind EOF
//
    data.header.payload_size = 0;
    kunit_comp.fw.data = (u8 *)data;
    kunit_comp.fw.size = size;
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
// cleanup
    snd_soc_unregister_component(test_dev);
    snd_soc_unregister_card(&kunit_comp.card);
    }
// TEST CASE
// Test passing topology file with PCM definition
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_pcm_tplg(test: *mut kunit) {
    static void snd_soc_tplg_test_load_pcm_tplg(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    u8 *data;
    int size;
    int ret;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = 0; /* expect success */
    size = sizeof(tplg_tmpl_with_pcm);
    data = kunit_kzalloc(kunit_comp.kunit, size, GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(kunit_comp.kunit, data);
    memcpy(data, &tplg_tmpl_with_pcm, sizeof(tplg_tmpl_with_pcm));
    kunit_comp.fw.data = data;
    kunit_comp.fw.size = size;
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
    snd_soc_unregister_component(test_dev);
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    }
// TEST CASE
// Test passing topology file with PCM definition
// with component reload
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_pcm_tplg_reload_comp(test: *mut kunit) {
    static void snd_soc_tplg_test_load_pcm_tplg_reload_comp(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    u8 *data;
    int size;
    int ret;
    int i;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = 0; /* expect success */
    size = sizeof(tplg_tmpl_with_pcm);
    data = kunit_kzalloc(kunit_comp.kunit, size, GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(kunit_comp.kunit, data);
    memcpy(data, &tplg_tmpl_with_pcm, sizeof(tplg_tmpl_with_pcm));
    kunit_comp.fw.data = data;
    kunit_comp.fw.size = size;
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    for (i = 0; i < 100; i++) {
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
    snd_soc_unregister_component(test_dev);
    }
// cleanup
    snd_soc_unregister_card(&kunit_comp.card);
    }
// TEST CASE
// Test passing topology file with PCM definition
// with card reload
#[no_mangle]
unsafe extern "C" fn snd_soc_tplg_test_load_pcm_tplg_reload_card(test: *mut kunit) {
    static void snd_soc_tplg_test_load_pcm_tplg_reload_card(struct kunit *test)
    {
    struct kunit_soc_component *kunit_comp;
    struct snd_soc_component *component;
    u8 *data;
    int size;
    int ret;
    int i;
// prepare
    kunit_comp = kunit_kzalloc(test, sizeof(*kunit_comp), GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, kunit_comp);
    kunit_comp.kunit = test;
    kunit_comp.expect = 0; /* expect success */
    size = sizeof(tplg_tmpl_with_pcm);
    data = kunit_kzalloc(kunit_comp.kunit, size, GFP_KERNEL);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(kunit_comp.kunit, data);
    memcpy(data, &tplg_tmpl_with_pcm, sizeof(tplg_tmpl_with_pcm));
    kunit_comp.fw.data = data;
    kunit_comp.fw.size = size;
    kunit_comp.card.dev = test_dev;
    kunit_comp.card.name = "kunit-card";
    kunit_comp.card.owner = THIS_MODULE;
    kunit_comp.card.dai_link = kunit_dai_links;
    kunit_comp.card.num_links = ARRAY_SIZE(kunit_dai_links);
    kunit_comp.card.fully_routed = true;
    component = snd_soc_component_alloc(test_dev);
    KUNIT_ASSERT_NOT_NULL(test, component);
    snd_soc_component_set_priv(component, kunit_comp);
// run test
    ret = snd_soc_register_component(component, &test_component, core::ptr::null_mut(), 0);
    KUNIT_EXPECT_EQ(test, 0, ret);
    for (i = 0; i < 100; i++) {
    ret = snd_soc_register_card(&kunit_comp.card);
    if (ret != 0 && ret != -EPROBE_DEFER)
    KUNIT_FAIL(test, "Failed to register card");
    snd_soc_unregister_card(&kunit_comp.card);
    }
// cleanup
    snd_soc_unregister_component(test_dev);
    }
// ===== KUNIT MODULE DEFINITIONS ===========================================
    static struct kunit_case snd_soc_tplg_test_cases[] = {
    KUNIT_CASE(snd_soc_tplg_test_load_with_null_comp),
    KUNIT_CASE(snd_soc_tplg_test_load_with_null_ops),
    KUNIT_CASE(snd_soc_tplg_test_load_with_null_fw),
    KUNIT_CASE(snd_soc_tplg_test_load_empty_tplg),
    KUNIT_CASE(snd_soc_tplg_test_load_empty_tplg_bad_magic),
    KUNIT_CASE(snd_soc_tplg_test_load_empty_tplg_bad_abi),
    KUNIT_CASE(snd_soc_tplg_test_load_empty_tplg_bad_size),
    KUNIT_CASE(snd_soc_tplg_test_load_empty_tplg_bad_payload_size),
    KUNIT_CASE(snd_soc_tplg_test_load_pcm_tplg),
    KUNIT_CASE(snd_soc_tplg_test_load_pcm_tplg_reload_comp),
    KUNIT_CASE(snd_soc_tplg_test_load_pcm_tplg_reload_card),
    {}
    };
    static struct kunit_suite snd_soc_tplg_test_suite = {
    .name = "snd_soc_tplg_test",
    .init = snd_soc_tplg_test_init,
    .exit = snd_soc_tplg_test_exit,
    .test_cases = snd_soc_tplg_test_cases,
    };
    kunit_test_suites(&snd_soc_tplg_test_suite);
    MODULE_DESCRIPTION("ASoC Topology Kernel Unit Tests");
    MODULE_LICENSE("GPL");
