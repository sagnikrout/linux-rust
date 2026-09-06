//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/ath/ath9k/gpio.c
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
// Copyright (c) 2008-2011 Atheros Communications Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

//
// LED functions
//

#[no_mangle]
unsafe extern "C" fn ath_fill_led_pin(sc: *mut ath_softc) {
    static void ath_fill_led_pin(struct ath_softc *sc)
    {
    struct ath_hw *ah = sc.sc_ah;
// Set default led pin if invalid
    if (ah.led_pin < 0) {
    if (AR_SREV_9287(ah))
    ah.led_pin = ATH_LED_PIN_9287;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: AR_SREV_9485(ah)) -> else {
    else if (AR_SREV_9485(ah))
    ah.led_pin = ATH_LED_PIN_9485;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: AR_SREV_9300(ah)) -> else {
    else if (AR_SREV_9300(ah))
    ah.led_pin = ATH_LED_PIN_9300;
#[no_mangle]
pub unsafe extern "C" fn if(AR_SREV_9565(ah): AR_SREV_9462(ah) ||) -> else {
    else if (AR_SREV_9462(ah) || AR_SREV_9565(ah))
    ah.led_pin = ATH_LED_PIN_9462;
    else
    ah.led_pin = ATH_LED_PIN_DEF;
    }
// Configure gpio for output
    ath9k_hw_gpio_request_out(ah, ah.led_pin, "ath9k-led",
    AR_GPIO_OUTPUT_MUX_AS_OUTPUT);
// LED off, active low
    ath9k_hw_set_gpio(ah, ah.led_pin, ah.config.led_active_high ? 0 : 1);
    }
    static void ath_led_brightness(struct led_classdev *led_cdev,
    enum led_brightness brightness)
    {
    struct ath_softc *sc = container_of(led_cdev, struct ath_softc, led_cdev);
    let mut val: u32 = (brightness == LED_OFF);
    if (sc.sc_ah.config.led_active_high)
    val = !val;
    ath9k_hw_set_gpio(sc.sc_ah, sc.sc_ah.led_pin, val);
    }
#[no_mangle]
pub unsafe extern "C" fn ath_deinit_leds(sc: *mut ath_softc) {
    void ath_deinit_leds(struct ath_softc *sc)
    {
    if (!sc.led_registered)
    return;
    ath_led_brightness(&sc.led_cdev, LED_OFF);
    led_classdev_unregister(&sc.led_cdev);
    ath9k_hw_gpio_free(sc.sc_ah, sc.sc_ah.led_pin);
    }
#[no_mangle]
pub unsafe extern "C" fn ath_init_leds(sc: *mut ath_softc) {
    void ath_init_leds(struct ath_softc *sc)
    {
    int ret;
    if (AR_SREV_9100(sc.sc_ah))
    return;
    ath_fill_led_pin(sc);
    if (!ath9k_led_blink)
    sc.led_cdev.default_trigger =
    ieee80211_get_radio_led_name(sc.hw);
    snprintf(sc.led_name, sizeof(sc.led_name),
    "ath9k-%s", wiphy_name(sc.hw.wiphy));
    sc.led_cdev.name = sc.led_name;
    sc.led_cdev.brightness_set = ath_led_brightness;
    ret = led_classdev_register(wiphy_dev(sc.hw.wiphy), &sc.led_cdev);
    if (ret < 0)
    return;
    sc.led_registered = true;
    }

//
// Rfkill
//
#[no_mangle]
unsafe extern "C" fn ath_is_rfkill_set(sc: *mut ath_softc) -> bool {
    static bool ath_is_rfkill_set(struct ath_softc *sc)
    {
    struct ath_hw *ah = sc.sc_ah;
    bool is_blocked;
    ath9k_ps_wakeup(sc);
    is_blocked = ath9k_hw_gpio_get(ah, ah.rfkill_gpio) ==
    ah.rfkill_polarity;
    ath9k_ps_restore(sc);
    return is_blocked;
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_rfkill_poll_state(hw: *mut ieee80211_hw) {
    void ath9k_rfkill_poll_state(struct ieee80211_hw *hw)
    {
    struct ath_softc *sc = hw.priv;
    let mut blocked: bool = !!ath_is_rfkill_set(sc);
    wiphy_rfkill_set_hw_state(hw.wiphy, blocked);
    }
#[no_mangle]
pub unsafe extern "C" fn ath_start_rfkill_poll(sc: *mut ath_softc) {
    void ath_start_rfkill_poll(struct ath_softc *sc)
    {
    struct ath_hw *ah = sc.sc_ah;
    if (ah.caps.hw_caps & ATH9K_HW_CAP_RFSILENT)
    wiphy_rfkill_start_polling(sc.hw.wiphy);
    }

//
// BTCOEX
//
// Detects if there is any priority bt traffic
//
#[no_mangle]
unsafe extern "C" fn ath_detect_bt_priority(sc: *mut ath_softc) {
    static void ath_detect_bt_priority(struct ath_softc *sc)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    struct ath_hw *ah = sc.sc_ah;
    if (ath9k_hw_gpio_get(sc.sc_ah, ah.btcoex_hw.btpriority_gpio))
    btcoex.bt_priority_cnt++;
    if (time_after(jiffies, btcoex.bt_priority_time +
    msecs_to_jiffies(ATH_BT_PRIORITY_TIME_THRESHOLD))) {
    clear_bit(BT_OP_PRIORITY_DETECTED, &btcoex.op_flags);
    clear_bit(BT_OP_SCAN, &btcoex.op_flags);
// Detect if colocated bt started scanning
    if (btcoex.bt_priority_cnt >= ATH_BT_CNT_SCAN_THRESHOLD) {
    ath_dbg(ath9k_hw_common(sc.sc_ah), BTCOEX,
    "BT scan detected\n");
    set_bit(BT_OP_PRIORITY_DETECTED, &btcoex.op_flags);
    set_bit(BT_OP_SCAN, &btcoex.op_flags);
    } else if (btcoex.bt_priority_cnt >= ATH_BT_CNT_THRESHOLD) {
    ath_dbg(ath9k_hw_common(sc.sc_ah), BTCOEX,
    "BT priority traffic detected\n");
    set_bit(BT_OP_PRIORITY_DETECTED, &btcoex.op_flags);
    }
    btcoex.bt_priority_cnt = 0;
    btcoex.bt_priority_time = jiffies;
    }
    }
#[no_mangle]
unsafe extern "C" fn ath_mci_ftp_adjust(sc: *mut ath_softc) {
    static void ath_mci_ftp_adjust(struct ath_softc *sc)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    struct ath_mci_profile *mci = &btcoex.mci;
    struct ath_hw *ah = sc.sc_ah;
    if (btcoex.bt_wait_time > ATH_BTCOEX_RX_WAIT_TIME) {
    if (ar9003_mci_state(ah, MCI_STATE_NEED_FTP_STOMP) &&
    (mci.num_pan || mci.num_other_acl))
    ah.btcoex_hw.mci.stomp_ftp =
    (sc.rx.num_pkts < ATH_BTCOEX_STOMP_FTP_THRESH);
    else
    ah.btcoex_hw.mci.stomp_ftp = false;
    btcoex.bt_wait_time = 0;
    sc.rx.num_pkts = 0;
    }
    }
//
// This is the master bt coex timer which runs for every
// 45ms, bt traffic will be given priority during 55% of this
// period while wlan gets remaining 45%
//
#[no_mangle]
unsafe extern "C" fn ath_btcoex_period_timer(t: *mut timer_list) {
    static void ath_btcoex_period_timer(struct timer_list *t)
    {
    struct ath_softc *sc = timer_container_of(sc, t, btcoex.period_timer);
    struct ath_hw *ah = sc.sc_ah;
    struct ath_btcoex *btcoex = &sc.btcoex;
    enum ath_stomp_type stomp_type;
    u32 timer_period;
    unsigned long flags;
    spin_lock_irqsave(&sc.sc_pm_lock, flags);
    if (sc.sc_ah.power_mode == ATH9K_PM_NETWORK_SLEEP) {
    btcoex.bt_wait_time += btcoex.btcoex_period;
    spin_unlock_irqrestore(&sc.sc_pm_lock, flags);
    goto skip_hw_wakeup;
    }
    spin_unlock_irqrestore(&sc.sc_pm_lock, flags);
    ath9k_ps_wakeup(sc);
    spin_lock_bh(&btcoex.btcoex_lock);
    if (ah.caps.hw_caps & ATH9K_HW_CAP_MCI) {
    ath9k_mci_update_rssi(sc);
    ath_mci_ftp_adjust(sc);
    }
    if (!(ah.caps.hw_caps & ATH9K_HW_CAP_MCI))
    ath_detect_bt_priority(sc);
    stomp_type = btcoex.bt_stomp_type;
    timer_period = btcoex.btcoex_no_stomp;
    if (!(ah.caps.hw_caps & ATH9K_HW_CAP_MCI)) {
    if (test_bit(BT_OP_SCAN, &btcoex.op_flags)) {
    stomp_type = ATH_BTCOEX_STOMP_ALL;
    timer_period = btcoex.btscan_no_stomp;
    }
    } else if (btcoex.stomp_audio >= 5) {
    stomp_type = ATH_BTCOEX_STOMP_AUDIO;
    btcoex.stomp_audio = 0;
    }
    ath9k_hw_btcoex_bt_stomp(ah, stomp_type);
    ath9k_hw_btcoex_enable(ah);
    spin_unlock_bh(&btcoex.btcoex_lock);
    if (btcoex.btcoex_period != btcoex.btcoex_no_stomp)
    mod_timer(&btcoex.no_stomp_timer,
    jiffies + msecs_to_jiffies(timer_period));
    ath9k_ps_restore(sc);
    skip_hw_wakeup:
    mod_timer(&btcoex.period_timer,
    jiffies + msecs_to_jiffies(btcoex.btcoex_period));
    }
//
// Generic tsf based hw timer which configures weight
// registers to time slice between wlan and bt traffic
//
#[no_mangle]
unsafe extern "C" fn ath_btcoex_no_stomp_timer(t: *mut timer_list) {
    static void ath_btcoex_no_stomp_timer(struct timer_list *t)
    {
    struct ath_softc *sc = timer_container_of(sc, t,
    btcoex.no_stomp_timer);
    struct ath_hw *ah = sc.sc_ah;
    struct ath_btcoex *btcoex = &sc.btcoex;
    ath9k_ps_wakeup(sc);
    spin_lock_bh(&btcoex.btcoex_lock);
    if (btcoex.bt_stomp_type == ATH_BTCOEX_STOMP_LOW ||
    (!(ah.caps.hw_caps & ATH9K_HW_CAP_MCI) &&
    test_bit(BT_OP_SCAN, &btcoex.op_flags)))
    ath9k_hw_btcoex_bt_stomp(ah, ATH_BTCOEX_STOMP_NONE);
#[no_mangle]
pub unsafe extern "C" fn if(ATH_BTCOEX_STOMP_ALL: btcoex->bt_stomp_type ==) -> else {
    else if (btcoex.bt_stomp_type == ATH_BTCOEX_STOMP_ALL)
    ath9k_hw_btcoex_bt_stomp(ah, ATH_BTCOEX_STOMP_LOW);
    ath9k_hw_btcoex_enable(ah);
    spin_unlock_bh(&btcoex.btcoex_lock);
    ath9k_ps_restore(sc);
    }
#[no_mangle]
unsafe extern "C" fn ath_init_btcoex_timer(sc: *mut ath_softc) {
    static void ath_init_btcoex_timer(struct ath_softc *sc)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    btcoex.btcoex_period = ATH_BTCOEX_DEF_BT_PERIOD;
    btcoex.btcoex_no_stomp = (100 - ATH_BTCOEX_DEF_DUTY_CYCLE) *
    btcoex.btcoex_period / 100;
    btcoex.btscan_no_stomp = (100 - ATH_BTCOEX_BTSCAN_DUTY_CYCLE) *
    btcoex.btcoex_period / 100;
    btcoex.bt_stomp_type = ATH_BTCOEX_STOMP_LOW;
    timer_setup(&btcoex.period_timer, ath_btcoex_period_timer, 0);
    timer_setup(&btcoex.no_stomp_timer, ath_btcoex_no_stomp_timer, 0);
    spin_lock_init(&btcoex.btcoex_lock);
    }
//
// (Re)start btcoex timers
//
#[no_mangle]
pub unsafe extern "C" fn ath9k_btcoex_timer_resume(sc: *mut ath_softc) {
    void ath9k_btcoex_timer_resume(struct ath_softc *sc)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    struct ath_hw *ah = sc.sc_ah;
    if (ath9k_hw_get_btcoex_scheme(ah) != ATH_BTCOEX_CFG_3WIRE &&
    ath9k_hw_get_btcoex_scheme(ah) != ATH_BTCOEX_CFG_MCI)
    return;
    ath_dbg(ath9k_hw_common(ah), BTCOEX, "Starting btcoex timers\n");
// make sure duty cycle timer is also stopped when resuming
    timer_delete_sync(&btcoex.no_stomp_timer);
    btcoex.bt_priority_cnt = 0;
    btcoex.bt_priority_time = jiffies;
    clear_bit(BT_OP_PRIORITY_DETECTED, &btcoex.op_flags);
    clear_bit(BT_OP_SCAN, &btcoex.op_flags);
    mod_timer(&btcoex.period_timer, jiffies);
    }
//
// Pause btcoex timer and bt duty cycle timer
//
#[no_mangle]
pub unsafe extern "C" fn ath9k_btcoex_timer_pause(sc: *mut ath_softc) {
    void ath9k_btcoex_timer_pause(struct ath_softc *sc)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    struct ath_hw *ah = sc.sc_ah;
    if (ath9k_hw_get_btcoex_scheme(ah) != ATH_BTCOEX_CFG_3WIRE &&
    ath9k_hw_get_btcoex_scheme(ah) != ATH_BTCOEX_CFG_MCI)
    return;
    ath_dbg(ath9k_hw_common(ah), BTCOEX, "Stopping btcoex timers\n");
    timer_delete_sync(&btcoex.period_timer);
    timer_delete_sync(&btcoex.no_stomp_timer);
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_btcoex_stop_gen_timer(sc: *mut ath_softc) {
    void ath9k_btcoex_stop_gen_timer(struct ath_softc *sc)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    timer_delete_sync(&btcoex.no_stomp_timer);
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_btcoex_aggr_limit(sc: *mut ath_softc, max_4ms_framelen: u32) -> u16 {
    u16 ath9k_btcoex_aggr_limit(struct ath_softc *sc, u32 max_4ms_framelen)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    struct ath_mci_profile *mci = &sc.btcoex.mci;
    let mut aggr_limit: u16 = 0;
    if ((sc.sc_ah.caps.hw_caps & ATH9K_HW_CAP_MCI) && mci.aggr_limit)
    aggr_limit = (max_4ms_framelen * mci.aggr_limit) >> 4;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: test_bit(BT_OP_PRIORITY_DETECTED, _arg: &btcoex->op_flags)) -> else {
    else if (test_bit(BT_OP_PRIORITY_DETECTED, &btcoex.op_flags))
    aggr_limit = min((max_4ms_framelen * 3) / 8,
    (u32)ATH_AMPDU_LIMIT_MAX);
    return aggr_limit;
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_btcoex_handle_interrupt(sc: *mut ath_softc, status: u32) {
    void ath9k_btcoex_handle_interrupt(struct ath_softc *sc, u32 status)
    {
    if (status & ATH9K_INT_MCI)
    ath_mci_intr(sc);
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_start_btcoex(sc: *mut ath_softc) {
    void ath9k_start_btcoex(struct ath_softc *sc)
    {
    struct ath_hw *ah = sc.sc_ah;
    if (ah.btcoex_hw.enabled ||
    ath9k_hw_get_btcoex_scheme(ah) == ATH_BTCOEX_CFG_NONE)
    return;
    if (!(ah.caps.hw_caps & ATH9K_HW_CAP_MCI))
    ath9k_hw_btcoex_set_weight(ah, AR_BT_COEX_WGHT,
    AR_STOMP_LOW_WLAN_WGHT, 0);
    else
    ath9k_hw_btcoex_set_weight(ah, 0, 0,
    ATH_BTCOEX_STOMP_NONE);
    ath9k_hw_btcoex_enable(ah);
    ath9k_btcoex_timer_resume(sc);
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_stop_btcoex(sc: *mut ath_softc) {
    void ath9k_stop_btcoex(struct ath_softc *sc)
    {
    struct ath_hw *ah = sc.sc_ah;
    if (!ah.btcoex_hw.enabled ||
    ath9k_hw_get_btcoex_scheme(ah) == ATH_BTCOEX_CFG_NONE)
    return;
    ath9k_btcoex_timer_pause(sc);
    ath9k_hw_btcoex_disable(ah);
    if (ah.caps.hw_caps & ATH9K_HW_CAP_MCI)
    ath_mci_flush_profile(&sc.btcoex.mci);
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_deinit_btcoex(sc: *mut ath_softc) {
    void ath9k_deinit_btcoex(struct ath_softc *sc)
    {
    struct ath_hw *ah = sc.sc_ah;
    if (ath9k_hw_mci_is_enabled(ah))
    ath_mci_cleanup(sc);
    else {
    let mut scheme: enum ath_btcoex_scheme = ath9k_hw_get_btcoex_scheme(ah);
    if (scheme == ATH_BTCOEX_CFG_2WIRE ||
    scheme == ATH_BTCOEX_CFG_3WIRE)
    ath9k_hw_btcoex_deinit(sc.sc_ah);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_init_btcoex(sc: *mut ath_softc) -> c_int {
    int ath9k_init_btcoex(struct ath_softc *sc)
    {
    struct ath_txq *txq;
    struct ath_hw *ah = sc.sc_ah;
    int r;
    ath9k_hw_btcoex_init_scheme(ah);
    switch (ath9k_hw_get_btcoex_scheme(sc.sc_ah)) {
    case ATH_BTCOEX_CFG_NONE:
    break;
    case ATH_BTCOEX_CFG_2WIRE:
    ath9k_hw_btcoex_init_2wire(sc.sc_ah);
    break;
    case ATH_BTCOEX_CFG_3WIRE:
    ath9k_hw_btcoex_init_3wire(sc.sc_ah);
    ath_init_btcoex_timer(sc);
    txq = sc.tx.txq_map[IEEE80211_AC_BE];
    ath9k_hw_init_btcoex_hw(sc.sc_ah, txq.axq_qnum);
    break;
    case ATH_BTCOEX_CFG_MCI:
    ath_init_btcoex_timer(sc);
    sc.btcoex.duty_cycle = ATH_BTCOEX_DEF_DUTY_CYCLE;
    INIT_LIST_HEAD(&sc.btcoex.mci.info);
    ath9k_hw_btcoex_init_mci(ah);
    r = ath_mci_setup(sc);
    if (r)
    return r;
    break;
    default:
    WARN_ON(1);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ath9k_dump_mci_btcoex(sc: *mut ath_softc, buf: *mut u8, size: u32) -> c_int {
    static int ath9k_dump_mci_btcoex(struct ath_softc *sc, u8 *buf, u32 size)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    struct ath_mci_profile *mci = &btcoex.mci;
    struct ath_hw *ah = sc.sc_ah;
    struct ath_btcoex_hw *btcoex_hw = &ah.btcoex_hw;
    let mut len: u32 = 0;
    int i;
    ATH_DUMP_BTCOEX("Total BT profiles", NUM_PROF(mci));
    ATH_DUMP_BTCOEX("MGMT", mci.num_mgmt);
    ATH_DUMP_BTCOEX("SCO", mci.num_sco);
    ATH_DUMP_BTCOEX("A2DP", mci.num_a2dp);
    ATH_DUMP_BTCOEX("HID", mci.num_hid);
    ATH_DUMP_BTCOEX("PAN", mci.num_pan);
    ATH_DUMP_BTCOEX("ACL", mci.num_other_acl);
    ATH_DUMP_BTCOEX("BDR", mci.num_bdr);
    ATH_DUMP_BTCOEX("Aggr. Limit", mci.aggr_limit);
    ATH_DUMP_BTCOEX("Stomp Type", btcoex.bt_stomp_type);
    ATH_DUMP_BTCOEX("BTCoex Period (msec)", btcoex.btcoex_period);
    ATH_DUMP_BTCOEX("Duty Cycle", btcoex.duty_cycle);
    ATH_DUMP_BTCOEX("BT Wait time", btcoex.bt_wait_time);
    ATH_DUMP_BTCOEX("Concurrent Tx", btcoex_hw.mci.concur_tx);
    ATH_DUMP_BTCOEX("Concurrent RSSI cnt", btcoex.rssi_count);
    len += scnprintf(buf + len, size - len, "BT Weights: ");
    for (i = 0; i < AR9300_NUM_BT_WEIGHTS; i++)
    len += scnprintf(buf + len, size - len, "%08x ",
    btcoex_hw.bt_weight[i]);
    len += scnprintf(buf + len, size - len, "\n");
    len += scnprintf(buf + len, size - len, "WLAN Weights: ");
    for (i = 0; i < AR9300_NUM_BT_WEIGHTS; i++)
    len += scnprintf(buf + len, size - len, "%08x ",
    btcoex_hw.wlan_weight[i]);
    len += scnprintf(buf + len, size - len, "\n");
    len += scnprintf(buf + len, size - len, "Tx Priorities: ");
    for (i = 0; i < ATH_BTCOEX_STOMP_MAX; i++)
    len += scnprintf(buf + len, size - len, "%08x ",
    btcoex_hw.tx_prio[i]);
    len += scnprintf(buf + len, size - len, "\n");
    return len;
    }
#[no_mangle]
unsafe extern "C" fn ath9k_dump_legacy_btcoex(sc: *mut ath_softc, buf: *mut u8, size: u32) -> c_int {
    static int ath9k_dump_legacy_btcoex(struct ath_softc *sc, u8 *buf, u32 size)
    {
    struct ath_btcoex *btcoex = &sc.btcoex;
    let mut len: u32 = 0;
    ATH_DUMP_BTCOEX("Stomp Type", btcoex.bt_stomp_type);
    ATH_DUMP_BTCOEX("BTCoex Period (msec)", btcoex.btcoex_period);
    ATH_DUMP_BTCOEX("Duty Cycle", btcoex.duty_cycle);
    ATH_DUMP_BTCOEX("BT Wait time", btcoex.bt_wait_time);
    return len;
    }
#[no_mangle]
pub unsafe extern "C" fn ath9k_dump_btcoex(sc: *mut ath_softc, buf: *mut u8, size: u32) -> c_int {
    int ath9k_dump_btcoex(struct ath_softc *sc, u8 *buf, u32 size)
    {
    if (ath9k_hw_mci_is_enabled(sc.sc_ah))
    return ath9k_dump_mci_btcoex(sc, buf, size);
    else
    return ath9k_dump_legacy_btcoex(sc, buf, size);
    }
