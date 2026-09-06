//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/eeprom.c
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2016 Felix Fietkau <nbd@nbd.name>
//

    enum mt76_sku_type {
    MT76_SKU_RATE,
    MT76_SKU_BACKOFF,
    MT76_SKU_BACKOFF_BF_OFFSET,
    };
#[no_mangle]
unsafe extern "C" fn mt76_get_of_eeprom_data(dev: *mut mt76_dev, eep: *mut c_void, len: c_int) -> c_int {
    static int mt76_get_of_eeprom_data(struct mt76_dev *dev, void *eep, int len)
    {
    struct device_node *np = dev.dev.of_node;
    const void *data;
    int size;
    data = of_get_property(np, "mediatek,eeprom-data", &size);
    if (!data)
    return -ENOENT;
    if (size > len)
    return -EINVAL;
    memcpy(eep, data, size);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt76_get_of_data_from_mtd(dev: *mut mt76_dev, eep: *mut c_void, offset: c_int, len: c_int) -> c_int {
    int mt76_get_of_data_from_mtd(struct mt76_dev *dev, void *eep, int offset, int len)
    {

    struct device_node *np = dev.dev.of_node;
    struct mtd_info *mtd;
    const __be32 *list;
    const char *part;
    phandle phandle;
    size_t retlen;
    int size;
    int ret;
    list = of_get_property(np, "mediatek,mtd-eeprom", &size);
    if (!list)
    return -ENOENT;
    phandle = be32_to_cpup(list++);
    if (!phandle)
    return -ENOENT;
    np = of_find_node_by_phandle(phandle);
    if (!np)
    return -EINVAL;
    part = of_get_property(np, "label", core::ptr::null_mut());
    if (!part)
    part = np.name;
    mtd = get_mtd_device_nm(part);
    if (IS_ERR(mtd)) {
    ret =  PTR_ERR(mtd);
    goto out_put_node;
    }
    if (size <= sizeof(*list)) {
    ret = -EINVAL;
    goto out_put_node;
    }
    offset += be32_to_cpup(list);
    ret = mtd_read(mtd, offset, len, &retlen, eep);
    put_mtd_device(mtd);
    if (mtd_is_bitflip(ret))
    ret = 0;
    if (ret) {
    dev_err(dev.dev, "reading EEPROM from mtd %s failed: %i\n",
    part, ret);
    goto out_put_node;
    }
    if (retlen < len) {
    ret = -EINVAL;
    goto out_put_node;
    }
    if (of_property_read_bool(dev.dev.of_node, "big-endian")) {
    u8 *data = (u8 *)eep;
    int i;
// convert eeprom data in Little Endian
    for (i = 0; i < round_down(len, 2); i += 2)
    put_unaligned_le16(get_unaligned_be16(&data[i]),
    &data[i]);
    }

    dev.test_mtd.name = devm_kstrdup(dev.dev, part, GFP_KERNEL);
    if (!dev.test_mtd.name) {
    ret = -ENOMEM;
    goto out_put_node;
    }
    dev.test_mtd.offset = offset;

    out_put_node:
    of_node_put(np);
    return ret;

    return -ENOENT;

    }
    EXPORT_SYMBOL_GPL(mt76_get_of_data_from_mtd);
    int mt76_get_of_data_from_nvmem(struct mt76_dev *dev, void *eep,
    const char *cell_name, int len)
    {
    struct device_node *np = dev.dev.of_node;
    struct nvmem_cell *cell;
    const void *data;
    size_t retlen;
    let mut ret: c_int = 0;
    cell = of_nvmem_cell_get(np, cell_name);
    if (IS_ERR(cell))
    return PTR_ERR(cell);
    data = nvmem_cell_read(cell, &retlen);
    nvmem_cell_put(cell);
    if (IS_ERR(data))
    return PTR_ERR(data);
    if (retlen < len) {
    ret = -EINVAL;
    goto exit;
    }
    memcpy(eep, data, len);
    exit:
    kfree(data);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mt76_get_of_data_from_nvmem);
#[no_mangle]
unsafe extern "C" fn mt76_get_of_eeprom(dev: *mut mt76_dev, eep: *mut c_void, len: c_int) -> c_int {
    static int mt76_get_of_eeprom(struct mt76_dev *dev, void *eep, int len)
    {
    struct device_node *np = dev.dev.of_node;
    int ret;
    if (!np)
    return -ENOENT;
    ret = mt76_get_of_eeprom_data(dev, eep, len);
    if (!ret)
    return 0;
    ret = mt76_get_of_data_from_mtd(dev, eep, 0, len);
    if (!ret)
    return 0;
    return mt76_get_of_data_from_nvmem(dev, eep, "eeprom", len);
    }
    int
    mt76_eeprom_override(struct mt76_phy *phy)
    {
    struct mt76_dev *dev = phy.dev;
    struct device_node *np = dev.dev.of_node;
    int err;
    err = of_get_mac_address(np, phy.macaddr);
    if (err == -EPROBE_DEFER)
    return err;
    if (!is_valid_ether_addr(phy.macaddr)) {
    eth_random_addr(phy.macaddr);
    dev_info(dev.dev,
    "Invalid MAC address, using random address %pM\n",
    phy.macaddr);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt76_eeprom_override);
#[no_mangle]
unsafe extern "C" fn mt76_string_prop_find(prop: *mut property, str: *const c_char) -> bool {
    static bool mt76_string_prop_find(struct property *prop, const char *str)
    {
    const char *cp = core::ptr::null_mut();
    if (!prop || !str || !str[0])
    return false;
    while ((cp = of_prop_next_string(prop, cp)) != core::ptr::null_mut())
    if (!strcasecmp(cp, str))
    return true;
    return false;
    }
    struct device_node *
    mt76_find_power_limits_node(struct mt76_dev *dev)
    {
    struct device_node *np = dev.dev.of_node;
    const char *const region_names[] = {
    [NL80211_DFS_UNSET] = "ww",
    [NL80211_DFS_ETSI] = "etsi",
    [NL80211_DFS_FCC] = "fcc",
    [NL80211_DFS_JP] = "jp",
    };
    struct device_node *cur, *fallback = core::ptr::null_mut();
    const char *region_name = core::ptr::null_mut();
    if (dev.region < ARRAY_SIZE(region_names))
    region_name = region_names[dev.region];
    np = of_get_child_by_name(np, "power-limits");
    if (!np)
    return core::ptr::null_mut();
    for_each_child_of_node(np, cur) {
    struct property *country = of_find_property(cur, "country", core::ptr::null_mut());
    struct property *regd = of_find_property(cur, "regdomain", core::ptr::null_mut());
    if (!country && !regd) {
    fallback = cur;
    continue;
    }
    if (mt76_string_prop_find(country, dev.alpha2) ||
    mt76_string_prop_find(regd, region_name)) {
    of_node_put(np);
    return cur;
    }
    }
    of_node_put(np);
    return fallback;
    }
    EXPORT_SYMBOL_GPL(mt76_find_power_limits_node);
    static const __be32 *
    mt76_get_of_array(struct device_node *np, char *name, size_t *len, int min)
    {
    struct property *prop = of_find_property(np, name, core::ptr::null_mut());
    if (!prop || !prop.value || prop.length < min * 4)
    return core::ptr::null_mut();
// len = prop->length;
    return prop.value;
    }
    static const s8 *
    mt76_get_of_array_s8(struct device_node *np, char *name, size_t *len, int min)
    {
    struct property *prop = of_find_property(np, name, core::ptr::null_mut());
    if (!prop || !prop.value || prop.length < min)
    return core::ptr::null_mut();
// len = prop->length;
    return prop.value;
    }
    struct device_node *
    mt76_find_channel_node(struct device_node *np, struct ieee80211_channel *chan)
    {
    struct device_node *cur;
    const __be32 *val;
    size_t len;
    for_each_child_of_node(np, cur) {
    val = mt76_get_of_array(cur, "channels", &len, 2);
    if (!val)
    continue;
    while (len >= 2 * sizeof(*val)) {
    if (chan.hw_value >= be32_to_cpu(val[0]) &&
    chan.hw_value <= be32_to_cpu(val[1]))
    return cur;
    val += 2;
    len -= 2 * sizeof(*val);
    }
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(mt76_find_channel_node);
    static s8
    mt76_get_txs_delta(struct device_node *np, u8 nss)
    {
    const __be32 *val;
    size_t len;
    val = mt76_get_of_array(np, "txs-delta", &len, nss);
    if (!val)
    return 0;
    return be32_to_cpu(val[nss - 1]);
    }
#[no_mangle]
pub unsafe extern "C" fn mt76_backoff_n_chains(dev: *mut mt76_dev, idx: u8) -> u8 {
    static inline u8 mt76_backoff_n_chains(struct mt76_dev *dev, u8 idx)
    {
// 0:1T1ss, 1:2T1ss, ..., 14:5T5ss
    static const u8 connac3_table[] = {
    1, 2, 3, 4, 5, 2, 3, 4, 5, 3, 4, 5, 4, 5, 5};
    static const u8 connac2_table[] = {
    1, 2, 3, 4, 2, 3, 4, 3, 4, 4, 0, 0, 0, 0, 0};
    if (idx >= ARRAY_SIZE(connac3_table))
    return 0;
    return is_mt799x(dev) ? connac3_table[idx] : connac2_table[idx];
    }
    static void
    mt76_apply_array_limit(struct mt76_dev *dev, s8 *pwr, size_t pwr_len,
    const s8 *data, s8 target_power, s8 nss_delta,
    s8 *max_power, int n_chains, enum mt76_sku_type type)
    {
    int i;
    if (!data)
    return;
    for (i = 0; i < pwr_len; i++) {
    let mut backoff_chain_idx: u8 = i;
    int backoff_n_chains;
    s8 backoff_delta;
    s8 delta;
    switch (type) {
    case MT76_SKU_RATE:
    delta = 0;
    backoff_delta = 0;
    backoff_n_chains = 0;
    break;
    case MT76_SKU_BACKOFF_BF_OFFSET:
    backoff_chain_idx += 1;
    fallthrough;
    case MT76_SKU_BACKOFF:
    delta = mt76_tx_power_path_delta(n_chains);
    backoff_n_chains = mt76_backoff_n_chains(dev, backoff_chain_idx);
    backoff_delta = mt76_tx_power_path_delta(backoff_n_chains);
    break;
    default:
    return;
    }
    pwr[i] = min_t(s8, target_power + delta - backoff_delta, data[i] + nss_delta);
// used for padding, doesn't need to be considered
    if (data[i] >= S8_MAX - 1)
    continue;
// only consider backoff value for the configured chain number
    if (type != MT76_SKU_RATE && n_chains != backoff_n_chains)
    continue;
// max_power = max(*max_power, pwr[i]);
    }
    }
    static void
    mt76_apply_multi_array_limit(struct mt76_dev *dev, s8 *pwr, size_t pwr_len,
    s8 pwr_num, const s8 *data, size_t len,
    s8 target_power, s8 nss_delta, s8 *max_power,
    int n_chains, enum mt76_sku_type type)
    {
    let mut connac2_backoff_ru_idx: static int = 2;
    int i, cur;
    if (!data)
    return;
    cur = data[0];
    for (i = 0; i < pwr_num; i++) {
    if (len < pwr_len + 1)
    break;
// Each RU entry (RU26, RU52, RU106, BW20, ...) in the DTS
// corresponds to 10 stream combinations (1T1ss, 2T1ss, 3T1ss,
// 4T1ss, 2T2ss, 3T2ss, 4T2ss, 3T3ss, 4T3ss, 4T4ss).
//
// For beamforming tables:
// - In connac2, beamforming entries for BW20~BW160 and OFDM
// do not include 1T1ss.
// - In connac3, beamforming entries for BW20~BW160 and RU
// include 1T1ss, but OFDM beamforming does not include 1T1ss.
//
// Non-beamforming and RU entries for both connac2 and connac3
// include 1T1ss.
//
    if (!is_mt799x(dev) && type == MT76_SKU_BACKOFF &&
    i > connac2_backoff_ru_idx)
    type = MT76_SKU_BACKOFF_BF_OFFSET;
    mt76_apply_array_limit(dev, pwr + pwr_len * i, pwr_len, data + 1,
    target_power, nss_delta, max_power,
    n_chains, type);
    if (--cur > 0)
    continue;
    data += pwr_len + 1;
    len -= pwr_len + 1;
    if (!len)
    break;
    cur = data[0];
    }
    }
    s8 mt76_get_rate_power_limits(struct mt76_phy *phy,
    struct ieee80211_channel *chan,
    struct mt76_power_limits *dest,
    s8 target_power)
    {
    struct mt76_dev *dev = phy.dev;
    struct device_node *np;
    const s8 *val;
    char name[16];
    char band;
    size_t len;
    let mut max_power: i8 = -127;
    s8 txs_delta;
    let mut n_chains: c_int = hweight16(phy.chainmask);
    memset(dest, target_power, sizeof(*dest) - sizeof(dest.path));
    memset(&dest.path, 0, sizeof(dest.path));
    if (!IS_ENABLED(CONFIG_OF))
    return target_power;
    np = mt76_find_power_limits_node(dev);
    if (!np)
    return target_power;
    switch (chan.band) {
    case NL80211_BAND_2GHZ:
    band = '2';
    break;
    case NL80211_BAND_5GHZ:
    band = '5';
    break;
    case NL80211_BAND_6GHZ:
    band = '6';
    break;
    default:
    return target_power;
    }
    snprintf(name, sizeof(name), "txpower-%cg", band);
    np = of_get_child_by_name(np, name);
    if (!np)
    return target_power;
    np = mt76_find_channel_node(np, chan);
    if (!np)
    return target_power;
    txs_delta = mt76_get_txs_delta(np, hweight16(phy.chainmask));
    val = mt76_get_of_array_s8(np, "rates-cck", &len, ARRAY_SIZE(dest.cck));
    mt76_apply_array_limit(dev, dest.cck, ARRAY_SIZE(dest.cck), val,
    target_power, txs_delta, &max_power, n_chains, MT76_SKU_RATE);
    val = mt76_get_of_array_s8(np, "rates-ofdm", &len, ARRAY_SIZE(dest.ofdm));
    mt76_apply_array_limit(dev, dest.ofdm, ARRAY_SIZE(dest.ofdm), val,
    target_power, txs_delta, &max_power, n_chains, MT76_SKU_RATE);
    val = mt76_get_of_array_s8(np, "rates-mcs", &len, ARRAY_SIZE(dest.mcs[0]) + 1);
    mt76_apply_multi_array_limit(dev, dest.mcs[0], ARRAY_SIZE(dest.mcs[0]),
    ARRAY_SIZE(dest.mcs), val, len, target_power,
    txs_delta, &max_power, n_chains, MT76_SKU_RATE);
    val = mt76_get_of_array_s8(np, "rates-ru", &len, ARRAY_SIZE(dest.ru[0]) + 1);
    mt76_apply_multi_array_limit(dev, dest.ru[0], ARRAY_SIZE(dest.ru[0]),
    ARRAY_SIZE(dest.ru), val, len, target_power,
    txs_delta, &max_power, n_chains, MT76_SKU_RATE);
    val = mt76_get_of_array_s8(np, "paths-cck", &len, ARRAY_SIZE(dest.path.cck));
    mt76_apply_array_limit(dev, dest.path.cck, ARRAY_SIZE(dest.path.cck), val,
    target_power, txs_delta, &max_power, n_chains, MT76_SKU_BACKOFF);
    val = mt76_get_of_array_s8(np, "paths-ofdm", &len, ARRAY_SIZE(dest.path.ofdm));
    mt76_apply_array_limit(dev, dest.path.ofdm, ARRAY_SIZE(dest.path.ofdm), val,
    target_power, txs_delta, &max_power, n_chains, MT76_SKU_BACKOFF);
    val = mt76_get_of_array_s8(np, "paths-ofdm-bf", &len, ARRAY_SIZE(dest.path.ofdm_bf));
    mt76_apply_array_limit(dev, dest.path.ofdm_bf, ARRAY_SIZE(dest.path.ofdm_bf), val,
    target_power, txs_delta, &max_power, n_chains,
    MT76_SKU_BACKOFF_BF_OFFSET);
    val = mt76_get_of_array_s8(np, "paths-ru", &len, ARRAY_SIZE(dest.path.ru[0]) + 1);
    mt76_apply_multi_array_limit(dev, dest.path.ru[0], ARRAY_SIZE(dest.path.ru[0]),
    ARRAY_SIZE(dest.path.ru), val, len, target_power,
    txs_delta, &max_power, n_chains, MT76_SKU_BACKOFF);
    val = mt76_get_of_array_s8(np, "paths-ru-bf", &len, ARRAY_SIZE(dest.path.ru_bf[0]) + 1);
    mt76_apply_multi_array_limit(dev, dest.path.ru_bf[0], ARRAY_SIZE(dest.path.ru_bf[0]),
    ARRAY_SIZE(dest.path.ru_bf), val, len, target_power,
    txs_delta, &max_power, n_chains, MT76_SKU_BACKOFF);
    return max_power;
    }
    EXPORT_SYMBOL_GPL(mt76_get_rate_power_limits);
    int
    mt76_eeprom_init(struct mt76_dev *dev, int len)
    {
    dev.eeprom.size = len;
    dev.eeprom.data = devm_kzalloc(dev.dev, len, GFP_KERNEL);
    if (!dev.eeprom.data)
    return -ENOMEM;
    return !mt76_get_of_eeprom(dev, dev.eeprom.data, len);
    }
    EXPORT_SYMBOL_GPL(mt76_eeprom_init);
