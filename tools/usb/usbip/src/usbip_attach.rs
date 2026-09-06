//! Automatically rewritten from C to Rust
//! Source: tools/usb/usbip/src/usbip_attach.c
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
// Copyright (C) 2011 matt mooney <mfm@muteddisk.com>
// 2005-2007 Takahiro Hirofuchi
// Copyright (C) 2015-2016 Samsung Electronics
// Igor Kotrasinski <i.kotrasinsk@samsung.com>
// Krzysztof Opasiak <k.opasiak@samsung.com>
//

    static const char usbip_attach_usage_string[] =
    "usbip attach <args>\n"
    "    -r, --remote=<host>      The machine with exported USB devices\n"
    "    -b, --busid=<busid>    Busid of the device on <host>\n"
    "    -d, --device=<devid>    Id of the virtual UDC on <host>\n";
#[no_mangle]
pub unsafe extern "C" fn usbip_attach_usage() {
    void usbip_attach_usage(void)
    {
    printf("usage: %s", usbip_attach_usage_string);
    }
pub const MAX_BUFF: c_int = 100;
#[no_mangle]
unsafe extern "C" fn record_connection(host: *mut c_char, port: *mut c_char, busid: *mut c_char, rhport: c_int) -> c_int {
    static int record_connection(char *host, char *port, char *busid, int rhport)
    {
    int fd;
    char path[PATH_MAX+1];
    char buff[MAX_BUFF+1];
    int ret;
    ret = mkdir(VHCI_STATE_PATH, 0700);
    if (ret < 0) {
// if VHCI_STATE_PATH exists, then it better be a directory
    if (errno == EEXIST) {
    struct stat s;
    ret = stat(VHCI_STATE_PATH, &s);
    if (ret < 0)
    return -1;
    if (!(s.st_mode & S_IFDIR))
    return -1;
    } else
    return -1;
    }
    snprintf(path, PATH_MAX, VHCI_STATE_PATH"/port%d", rhport);
    fd = open(path, O_WRONLY|O_CREAT|O_TRUNC, S_IRWXU);
    if (fd < 0)
    return -1;
    snprintf(buff, MAX_BUFF, "%s %s %s\n",
    host, port, busid);
    ret = write(fd, buff, strlen(buff));
    if (ret != (ssize_t) strlen(buff)) {
    close(fd);
    return -1;
    }
    close(fd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn import_device(sockfd: c_int, udev: *mut usbip_usb_device) -> c_int {
    static int import_device(int sockfd, struct usbip_usb_device *udev)
    {
    int rc;
    int port;
    let mut speed: u32 = udev.speed;
    rc = usbip_vhci_driver_open();
    if (rc < 0) {
    err("open vhci_driver (is vhci_hcd loaded?)");
    goto err_out;
    }
    do {
    port = usbip_vhci_get_free_port(speed);
    if (port < 0) {
    err("no free port");
    goto err_driver_close;
    }
    dbg("got free port %d", port);
    rc = usbip_vhci_attach_device(port, sockfd, udev.busnum,
    udev.devnum, udev.speed);
    if (rc < 0 && errno != EBUSY) {
    err("import device");
    goto err_driver_close;
    }
    } while (rc < 0);
    usbip_vhci_driver_close();
    return port;
    err_driver_close:
    usbip_vhci_driver_close();
    err_out:
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn query_import_device(sockfd: c_int, busid: *mut c_char) -> c_int {
    static int query_import_device(int sockfd, char *busid)
    {
    int rc;
    struct op_import_request request;
    struct op_import_reply   reply;
    let mut code: u16 = OP_REP_IMPORT;
    int status;
    memset(&request, 0, sizeof(request));
    memset(&reply, 0, sizeof(reply));
// send a request
    rc = usbip_net_send_op_common(sockfd, OP_REQ_IMPORT, 0);
    if (rc < 0) {
    err("send op_common");
    return -1;
    }
    strncpy(request.busid, busid, SYSFS_BUS_ID_SIZE-1);
    PACK_OP_IMPORT_REQUEST(0, &request);
    rc = usbip_net_send(sockfd, (void *) &request, sizeof(request));
    if (rc < 0) {
    err("send op_import_request");
    return -1;
    }
// receive a reply
    rc = usbip_net_recv_op_common(sockfd, &code, &status);
    if (rc < 0) {
    err("Attach Request for %s failed - %s\n",
    busid, usbip_op_common_status_string(status));
    return -1;
    }
    rc = usbip_net_recv(sockfd, (void *) &reply, sizeof(reply));
    if (rc < 0) {
    err("recv op_import_reply");
    return -1;
    }
    PACK_OP_IMPORT_REPLY(0, &reply);
// check the reply
    if (strncmp(reply.udev.busid, busid, SYSFS_BUS_ID_SIZE)) {
    err("recv different busid %s", reply.udev.busid);
    return -1;
    }
// import a device
    return import_device(sockfd, &reply.udev);
    }
#[no_mangle]
unsafe extern "C" fn attach_device(host: *mut c_char, busid: *mut c_char) -> c_int {
    static int attach_device(char *host, char *busid)
    {
    int sockfd;
    int rc;
    int rhport;
    sockfd = usbip_net_tcp_connect(host, usbip_port_string);
    if (sockfd < 0) {
    err("tcp connect");
    return -1;
    }
    rhport = query_import_device(sockfd, busid);
    if (rhport < 0)
    return -1;
    close(sockfd);
    rc = record_connection(host, usbip_port_string, busid, rhport);
    if (rc < 0) {
    err("record connection");
    return -1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn usbip_attach(argc: c_int, argv[]: *mut c_char) -> c_int {
    int usbip_attach(int argc, char *argv[])
    {
    static const struct option opts[] = {
    { "remote", required_argument, core::ptr::null_mut(), 'r' },
    { "busid",  required_argument, core::ptr::null_mut(), 'b' },
    { "device",  required_argument, core::ptr::null_mut(), 'd' },
    { core::ptr::null_mut(), 0,  core::ptr::null_mut(), 0 }
    };
    char *host = core::ptr::null_mut();
    char *busid = core::ptr::null_mut();
    int opt;
    let mut ret: c_int = -1;
    for (;;) {
    opt = getopt_long(argc, argv, "d:r:b:", opts, core::ptr::null_mut());
    if (opt == -1)
    break;
    switch (opt) {
    case 'r':
    host = optarg;
    break;
    case 'd':
    case 'b':
    busid = optarg;
    break;
    default:
    goto err_out;
    }
    }
    if (!host || !busid)
    goto err_out;
    ret = attach_device(host, busid);
    goto out;
    err_out:
    usbip_attach_usage();
    out:
    return ret;
    }
