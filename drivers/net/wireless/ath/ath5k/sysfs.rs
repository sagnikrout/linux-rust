//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/ath/ath5k/sysfs.c
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


// SPDX-License-Identifier: GPL-2.0

    static ssize_t ath5k_attr_show_##name(struct device *dev,		\
    struct device_attribute *attr,			\
    char *buf)					\
    {									\
    struct ieee80211_hw *hw = dev_get_drvdata(dev);			\
    struct ath5k_hw *ah = hw.priv;				\
    return sysfs_emit(buf, "%d\n", get);			\
    }									\
    \
    static ssize_t ath5k_attr_store_##name(struct device *dev,		\
    struct device_attribute *attr,			\
    const char *buf, size_t count)			\
    {									\
    struct ieee80211_hw *hw = dev_get_drvdata(dev);			\
    struct ath5k_hw *ah = hw.priv;				\
    int val, ret;							\
    \
    ret = kstrtoint(buf, 10, &val);					\
    if (ret < 0)							\
    return ret;						\
    set(ah, val);						\
    return count;							\
    }									\
    static DEVICE_ATTR(name, 0644,						\
    ath5k_attr_show_##name, ath5k_attr_store_##name)

    static ssize_t ath5k_attr_show_##name(struct device *dev,		\
    struct device_attribute *attr,			\
    char *buf)					\
    {									\
    struct ieee80211_hw *hw = dev_get_drvdata(dev);			\
    struct ath5k_hw *ah = hw.priv;				\
    return sysfs_emit(buf, "%d\n", get);			\
    }									\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR(_arg: name, _arg: 0444, _arg: ath5k_attr_show_##name, _arg: NULL) -> static {
    static DEVICE_ATTR(name, 0444, ath5k_attr_show_##name, core::ptr::null_mut())
// ANI
    SIMPLE_SHOW_STORE(ani_mode, ah.ani_state.ani_mode, ath5k_ani_init);
    SIMPLE_SHOW_STORE(noise_immunity_level, ah.ani_state.noise_imm_level,
    ath5k_ani_set_noise_immunity_level);
    SIMPLE_SHOW_STORE(spur_level, ah.ani_state.spur_level,
    ath5k_ani_set_spur_immunity_level);
    SIMPLE_SHOW_STORE(firstep_level, ah.ani_state.firstep_level,
    ath5k_ani_set_firstep_level);
    SIMPLE_SHOW_STORE(ofdm_weak_signal_detection, ah.ani_state.ofdm_weak_sig,
    ath5k_ani_set_ofdm_weak_signal_detection);
    SIMPLE_SHOW_STORE(cck_weak_signal_detection, ah.ani_state.cck_weak_sig,
    ath5k_ani_set_cck_weak_signal_detection);
    SIMPLE_SHOW(spur_level_max, ah.ani_state.max_spur_level);
    static ssize_t ath5k_attr_show_noise_immunity_level_max(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", ATH5K_ANI_MAX_NOISE_IMM_LVL);
    }
    static DEVICE_ATTR(noise_immunity_level_max, 0444,
    ath5k_attr_show_noise_immunity_level_max, core::ptr::null_mut());
    static ssize_t ath5k_attr_show_firstep_level_max(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    return sysfs_emit(buf, "%d\n", ATH5K_ANI_MAX_FIRSTEP_LVL);
    }
    static DEVICE_ATTR(firstep_level_max, 0444,
    ath5k_attr_show_firstep_level_max, core::ptr::null_mut());
    static struct attribute *ath5k_sysfs_entries_ani[] = {
    &dev_attr_ani_mode.attr,
    &dev_attr_noise_immunity_level.attr,
    &dev_attr_spur_level.attr,
    &dev_attr_firstep_level.attr,
    &dev_attr_ofdm_weak_signal_detection.attr,
    &dev_attr_cck_weak_signal_detection.attr,
    &dev_attr_noise_immunity_level_max.attr,
    &dev_attr_spur_level_max.attr,
    &dev_attr_firstep_level_max.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group ath5k_attribute_group_ani = {
    .name = "ani",
    .attrs = ath5k_sysfs_entries_ani,
    };
// register / unregister
    int
    ath5k_sysfs_register(struct ath5k_hw *ah)
    {
    struct device *dev = ah.dev;
    int err;
    err = sysfs_create_group(&dev.kobj, &ath5k_attribute_group_ani);
    if (err) {
    ATH5K_ERR(ah, "failed to create sysfs group\n");
    return err;
    }
    return 0;
    }
    void
    ath5k_sysfs_unregister(struct ath5k_hw *ah)
    {
    struct device *dev = ah.dev;
    sysfs_remove_group(&dev.kobj, &ath5k_attribute_group_ani);
    }
