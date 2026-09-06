//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/hwtstamp_config.c
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
// Test program for SIOC{G,S}HWTSTAMP
// Copyright 2013 Solarflare Communications
// Author: Ben Hutchings
//

    static int
    lookup_value(const char **names, int size, const char *name)
    {
    int value;
    for (value = 0; value < size; value++)
    if (names[value] && strcasecmp(names[value], name) == 0)
    return value;
    return -1;
    }
    static const char *
    lookup_name(const char **names, int size, int value)
    {
    return (value >= 0 && value < size) ? names[value] : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn list_names(f: *mut FILE, names: *const c_char, size: c_int) {
    static void list_names(FILE *f, const char **names, int size)
    {
    int value;
    for (value = 0; value < size; value++)
    if (names[value])
    fprintf(f, "    %s\n", names[value]);
    }
    static const char *tx_types[] = {

    TX_TYPE(OFF),
    TX_TYPE(ON),
    TX_TYPE(ONESTEP_SYNC)

    };

    static const char *rx_filters[] = {

    RX_FILTER(NONE),
    RX_FILTER(ALL),
    RX_FILTER(SOME),
    RX_FILTER(PTP_V1_L4_EVENT),
    RX_FILTER(PTP_V1_L4_SYNC),
    RX_FILTER(PTP_V1_L4_DELAY_REQ),
    RX_FILTER(PTP_V2_L4_EVENT),
    RX_FILTER(PTP_V2_L4_SYNC),
    RX_FILTER(PTP_V2_L4_DELAY_REQ),
    RX_FILTER(PTP_V2_L2_EVENT),
    RX_FILTER(PTP_V2_L2_SYNC),
    RX_FILTER(PTP_V2_L2_DELAY_REQ),
    RX_FILTER(PTP_V2_EVENT),
    RX_FILTER(PTP_V2_SYNC),
    RX_FILTER(PTP_V2_DELAY_REQ),

    };

#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    fputs("Usage: hwtstamp_config if_name [tx_type rx_filter]\n"
    "tx_type is any of (case-insensitive):\n",
    stderr);
    list_names(stderr, tx_types, N_TX_TYPES);
    fputs("rx_filter is any of (case-insensitive):\n", stderr);
    list_names(stderr, rx_filters, N_RX_FILTERS);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct ifreq ifr;
    struct hwtstamp_config config;
    const char *name;
    int sock;
    if ((argc != 2 && argc != 4) || (strlen(argv[1]) >= IFNAMSIZ)) {
    usage();
    return 2;
    }
    if (argc == 4) {
    config.flags = 0;
    config.tx_type = lookup_value(tx_types, N_TX_TYPES, argv[2]);
    config.rx_filter = lookup_value(rx_filters, N_RX_FILTERS, argv[3]);
    if (config.tx_type < 0 || config.rx_filter < 0) {
    usage();
    return 2;
    }
    }
    sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) {
    perror("socket");
    return 1;
    }
    strcpy(ifr.ifr_name, argv[1]);
    ifr.ifr_data = (caddr_t)&config;
    if (ioctl(sock, (argc == 2) ? SIOCGHWTSTAMP : SIOCSHWTSTAMP, &ifr)) {
    perror("ioctl");
    return 1;
    }
    printf("flags = %#x\n", config.flags);
    name = lookup_name(tx_types, N_TX_TYPES, config.tx_type);
    if (name)
    printf("tx_type = %s\n", name);
    else
    printf("tx_type = %d\n", config.tx_type);
    name = lookup_name(rx_filters, N_RX_FILTERS, config.rx_filter);
    if (name)
    printf("rx_filter = %s\n", name);
    else
    printf("rx_filter = %d\n", config.rx_filter);
    return 0;
    }
