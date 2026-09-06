//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/mptcp/pm_nl_ctl.c
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

pub const IPPROTO_MPTCP: c_int = 262;

#[no_mangle]
unsafe extern "C" fn syntax(argv[]: *mut c_char) {
    static void syntax(char *argv[])
    {
    fprintf(stderr, "%s add|ann|rem|csf|dsf|get|set|del|flush|dump|events|listen|accept [<args>]\n", argv[0]);
    fprintf(stderr, "\tadd [flags signal|subflow|backup|fullmesh] [id <nr>] [dev <name>] <ip>\n");
    fprintf(stderr, "\tann <local-ip> id <local-id> token <token> [port <local-port>] [dev <name>]\n");
    fprintf(stderr, "\trem id <local-id> token <token>\n");
    fprintf(stderr, "\tcsf lip <local-ip> lid <local-id> rip <remote-ip> rport <remote-port> token <token>\n");
    fprintf(stderr, "\tdsf lip <local-ip> lport <local-port> rip <remote-ip> rport <remote-port> token <token>\n");
    fprintf(stderr, "\tdel <id> [<ip>]\n");
    fprintf(stderr, "\tget <id>\n");
    fprintf(stderr, "\tset [<ip>] [id <nr>] flags [no]backup|[no]fullmesh [port <nr>] [token <token>] [rip <ip>] [rport <port>]\n");
    fprintf(stderr, "\tflush\n");
    fprintf(stderr, "\tdump\n");
    fprintf(stderr, "\tlimits [<rcv addr max> <subflow max>]\n");
    fprintf(stderr, "\tevents\n");
    fprintf(stderr, "\tlisten <local-ip> <local-port>\n");
    exit(0);
    }
#[no_mangle]
unsafe extern "C" fn init_genl_req(data: *mut c_char, family: c_int, cmd: c_int, version: c_int) -> c_int {
    static int init_genl_req(char *data, int family, int cmd, int version)
    {
    struct nlmsghdr *nh = (void *)data;
    struct genlmsghdr *gh;
    let mut off: c_int = 0;
    nh.nlmsg_type = family;
    nh.nlmsg_flags = NLM_F_REQUEST;
    nh.nlmsg_len = NLMSG_LENGTH(GENL_HDRLEN);
    off += NLMSG_ALIGN(sizeof(*nh));
    gh = (void *)(data + off);
    gh.cmd = cmd;
    gh.version = version;
    off += NLMSG_ALIGN(sizeof(*gh));
    return off;
    }
#[no_mangle]
unsafe extern "C" fn nl_error(nh: *mut nlmsghdr) -> c_int {
    static int nl_error(struct nlmsghdr *nh)
    {
    struct nlmsgerr *err = (struct nlmsgerr *)NLMSG_DATA(nh);
    let mut len: c_int = nh.nlmsg_len - sizeof(*nh);
    uint32_t off;
    if (len < sizeof(struct nlmsgerr)) {
    error(1, 0, "netlink error message truncated %d min %ld", len,
    sizeof(struct nlmsgerr));
    return -1;
    }
    if (err.error) {
// check messages from kernel
    struct rtattr *attrs = (struct rtattr *)NLMSG_DATA(nh);
    fprintf(stderr, "netlink error %d (%s)\n",
    err.error, strerror(-err.error));
    while (RTA_OK(attrs, len)) {
    if (attrs.rta_type == NLMSGERR_ATTR_MSG)
    fprintf(stderr, "netlink ext ack msg: %s\n",
    (char *)RTA_DATA(attrs));
    if (attrs.rta_type == NLMSGERR_ATTR_OFFS) {
    memcpy(&off, RTA_DATA(attrs), 4);
    fprintf(stderr, "netlink err off %d\n",
    (int)off);
    }
    attrs = RTA_NEXT(attrs, len);
    }
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn capture_events(fd: c_int, event_group: c_int) -> c_int {
    static int capture_events(int fd, int event_group)
    {
    u_int8_t buffer[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) + 1024];
    struct genlmsghdr *ghdr;
    struct rtattr *attrs;
    struct nlmsghdr *nh;
    let mut ret: c_int = 0;
    int res_len;
    int msg_len;
    fd_set rfds;
    if (setsockopt(fd, SOL_NETLINK, NETLINK_ADD_MEMBERSHIP,
    &event_group, sizeof(event_group)) < 0)
    error(1, errno, "could not join the " MPTCP_PM_EV_GRP_NAME " mcast group");
    do {
    let mut server_side: bool = false;
    FD_ZERO(&rfds);
    FD_SET(fd, &rfds);
    res_len = NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) + 1024;
    ret = select(FD_SETSIZE, &rfds, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (ret < 0)
    error(1, ret, "error in select() on NL socket");
    res_len = recv(fd, buffer, res_len, 0);
    if (res_len < 0)
    error(1, res_len, "error on recv() from NL socket");
    nh = (struct nlmsghdr *)buffer;
    for (; NLMSG_OK(nh, res_len); nh = NLMSG_NEXT(nh, res_len)) {
    if (nh.nlmsg_type == NLMSG_ERROR)
    error(1, NLMSG_ERROR, "received invalid NL message");
    ghdr = (struct genlmsghdr *)NLMSG_DATA(nh);
    if (ghdr.cmd == 0)
    continue;
    fprintf(stderr, "type:%d", ghdr.cmd);
    msg_len = nh.nlmsg_len - NLMSG_LENGTH(GENL_HDRLEN);
    attrs = (struct rtattr *) ((char *) ghdr + GENL_HDRLEN);
    while (RTA_OK(attrs, msg_len)) {
    if (attrs.rta_type == MPTCP_ATTR_TOKEN)
    fprintf(stderr, ",token:%u", *(__u32 *)RTA_DATA(attrs));
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_FAMILY: attrs->rta_type ==) -> else {
    else if (attrs.rta_type == MPTCP_ATTR_FAMILY)
    fprintf(stderr, ",family:%u", *(__u16 *)RTA_DATA(attrs));
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_LOC_ID: attrs->rta_type ==) -> else {
    else if (attrs.rta_type == MPTCP_ATTR_LOC_ID)
    fprintf(stderr, ",loc_id:%u", *(__u8 *)RTA_DATA(attrs));
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_REM_ID: attrs->rta_type ==) -> else {
    else if (attrs.rta_type == MPTCP_ATTR_REM_ID)
    fprintf(stderr, ",rem_id:%u", *(__u8 *)RTA_DATA(attrs));
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_SADDR4: attrs->rta_type ==) -> else {
    let mut saddr4: u_int32_t = ntohl(*(__u32 *)RTA_DATA(attrs));
    fprintf(stderr, ",saddr4:%u.%u.%u.%u", saddr4 >> 24,
    (saddr4 >> 16) & 0xFF, (saddr4 >> 8) & 0xFF,
    (saddr4 & 0xFF));
    } else if (attrs.rta_type == MPTCP_ATTR_SADDR6) {
    char buf[INET6_ADDRSTRLEN];
    if (inet_ntop(AF_INET6, RTA_DATA(attrs), buf,
    sizeof(buf)) != core::ptr::null_mut())
    fprintf(stderr, ",saddr6:%s", buf);
    } else if (attrs.rta_type == MPTCP_ATTR_DADDR4) {
    let mut daddr4: u_int32_t = ntohl(*(__u32 *)RTA_DATA(attrs));
    fprintf(stderr, ",daddr4:%u.%u.%u.%u", daddr4 >> 24,
    (daddr4 >> 16) & 0xFF, (daddr4 >> 8) & 0xFF,
    (daddr4 & 0xFF));
    } else if (attrs.rta_type == MPTCP_ATTR_DADDR6) {
    char buf[INET6_ADDRSTRLEN];
    if (inet_ntop(AF_INET6, RTA_DATA(attrs), buf,
    sizeof(buf)) != core::ptr::null_mut())
    fprintf(stderr, ",daddr6:%s", buf);
    } else if (attrs.rta_type == MPTCP_ATTR_SPORT)
    fprintf(stderr, ",sport:%u",
    ntohs(*(__u16 *)RTA_DATA(attrs)));
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_DPORT: attrs->rta_type ==) -> else {
    else if (attrs.rta_type == MPTCP_ATTR_DPORT)
    fprintf(stderr, ",dport:%u",
    ntohs(*(__u16 *)RTA_DATA(attrs)));
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_BACKUP: attrs->rta_type ==) -> else {
    else if (attrs.rta_type == MPTCP_ATTR_BACKUP)
    fprintf(stderr, ",backup:%u", *(__u8 *)RTA_DATA(attrs));
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_ERROR: attrs->rta_type ==) -> else {
    else if (attrs.rta_type == MPTCP_ATTR_ERROR)
    fprintf(stderr, ",error:%u", *(__u8 *)RTA_DATA(attrs));
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_SERVER_SIDE: attrs->rta_type ==) -> else {
    else if (attrs.rta_type == MPTCP_ATTR_SERVER_SIDE)
    server_side = !!*(__u8 *)RTA_DATA(attrs);
#[no_mangle]
pub unsafe extern "C" fn if(MPTCP_ATTR_FLAGS: attrs->rta_type ==) -> else {
    let mut flags: __u16 = *(__u16 *)RTA_DATA(attrs);
// only print when present, easier
    if (flags & MPTCP_PM_EV_FLAG_DENY_JOIN_ID0)
    fprintf(stderr, ",deny_join_id0:1");
    if (flags & MPTCP_PM_EV_FLAG_SERVER_SIDE)
    server_side = true;
    }
    attrs = RTA_NEXT(attrs, msg_len);
    }
    }
    if (server_side)
    fprintf(stderr, ",server_side:1");
    fprintf(stderr, "\n");
    } while (1);
    return 0;
    }
// do a netlink command and, if max > 0, fetch the reply ; nh's size >1024B
#[no_mangle]
unsafe extern "C" fn do_nl_req(fd: c_int, nh: *mut nlmsghdr, len: c_int, max: c_int) -> c_int {
    static int do_nl_req(int fd, struct nlmsghdr *nh, int len, int max)
    {
    void *data = nh;
    int rem, ret;
    let mut err: c_int = 0;
// If no expected answer, ask for an ACK to look for errors if any
    if (max == 0) {
    nh.nlmsg_flags |= NLM_F_ACK;
    max = 1024;
    }
    nh.nlmsg_len = len;
    ret = send(fd, data, len, 0);
    if (ret != len)
    error(1, errno, "send netlink: %uB != %uB\n", ret, len);
    ret = recv(fd, data, max, 0);
    if (ret < 0)
    error(1, errno, "recv netlink: %uB\n", ret);
    rem = ret;
// Beware: the NLMSG_NEXT macro updates the 'rem' argument
    for (; NLMSG_OK(nh, rem); nh = NLMSG_NEXT(nh, rem)) {
    if (nh.nlmsg_type == NLMSG_DONE)
    break;
    if (nh.nlmsg_type == NLMSG_ERROR && nl_error(nh))
    err = 1;
    }
    if (err)
    error(1, 0, "bailing out due to netlink error[s]");
    return ret;
    }
    static int genl_parse_getfamily(struct nlmsghdr *nlh, int *pm_family,
    int *events_mcast_grp)
    {
    struct genlmsghdr *ghdr = NLMSG_DATA(nlh);
    let mut len: c_int = nlh.nlmsg_len;
    struct rtattr *attrs;
    struct rtattr *grps;
    struct rtattr *grp;
    int got_events_grp;
    int got_family;
    int grps_len;
    int grp_len;
    if (nlh.nlmsg_type != GENL_ID_CTRL)
    error(1, errno, "Not a controller message, len=%d type=0x%x\n",
    nlh.nlmsg_len, nlh.nlmsg_type);
    len -= NLMSG_LENGTH(GENL_HDRLEN);
    if (len < 0)
    error(1, errno, "wrong controller message len %d\n", len);
    if (ghdr.cmd != CTRL_CMD_NEWFAMILY)
    error(1, errno, "Unknown controller command %d\n", ghdr.cmd);
    attrs = (struct rtattr *) ((char *) ghdr + GENL_HDRLEN);
    got_family = 0;
    got_events_grp = 0;
    while (RTA_OK(attrs, len)) {
    if (attrs.rta_type == CTRL_ATTR_FAMILY_ID) {
// pm_family = *(__u16 *)RTA_DATA(attrs);
    got_family = 1;
    } else if (attrs.rta_type == CTRL_ATTR_MCAST_GROUPS) {
    grps = RTA_DATA(attrs);
    grps_len = RTA_PAYLOAD(attrs);
    while (RTA_OK(grps, grps_len)) {
    grp = RTA_DATA(grps);
    grp_len = RTA_PAYLOAD(grps);
    got_events_grp = 0;
    while (RTA_OK(grp, grp_len)) {
    if (grp.rta_type == CTRL_ATTR_MCAST_GRP_ID)
// events_mcast_grp = *(__u32 *)RTA_DATA(grp);
    else if (grp.rta_type == CTRL_ATTR_MCAST_GRP_NAME &&
    !strcmp(RTA_DATA(grp), MPTCP_PM_EV_GRP_NAME))
    got_events_grp = 1;
    grp = RTA_NEXT(grp, grp_len);
    }
    if (got_events_grp)
    break;
    grps = RTA_NEXT(grps, grps_len);
    }
    }
    if (got_family && got_events_grp)
    return 0;
    attrs = RTA_NEXT(attrs, len);
    }
    error(1, errno, "can't find CTRL_ATTR_FAMILY_ID attr");
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn resolve_mptcp_pm_netlink(fd: c_int, pm_family: *mut c_int, events_mcast_grp: *mut c_int) -> c_int {
    static int resolve_mptcp_pm_netlink(int fd, int *pm_family, int *events_mcast_grp)
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    struct nlmsghdr *nh;
    struct rtattr *rta;
    int namelen;
    let mut off: c_int = 0;
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, GENL_ID_CTRL, CTRL_CMD_GETFAMILY, 0);
    rta = (void *)(data + off);
    namelen = strlen(MPTCP_PM_NAME) + 1;
    rta.rta_type = CTRL_ATTR_FAMILY_NAME;
    rta.rta_len = RTA_LENGTH(namelen);
    memcpy(RTA_DATA(rta), MPTCP_PM_NAME, namelen);
    off += NLMSG_ALIGN(rta.rta_len);
    do_nl_req(fd, nh, off, sizeof(data));
    return genl_parse_getfamily((void *)data, pm_family, events_mcast_grp);
    }
#[no_mangle]
pub unsafe extern "C" fn dsf(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int dsf(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    struct rtattr *rta, *addr;
    u_int16_t family, port;
    struct nlmsghdr *nh;
    u_int32_t token;
    int addr_start;
    let mut off: c_int = 0;
    int arg;
    const char *params[5];
    memset(params, 0, 5 * sizeof(const char *));
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_SUBFLOW_DESTROY,
    MPTCP_PM_VER);
    if (argc < 12)
    syntax(argv);
// Params recorded in this order:
// <local-ip>, <local-port>, <remote-ip>, <remote-port>, <token>
//
    for (arg = 2; arg < argc; arg++) {
    if (!strcmp(argv[arg], "lip")) {
    if (++arg >= argc)
    error(1, 0, " missing local IP");
    params[0] = argv[arg];
    } else if (!strcmp(argv[arg], "lport")) {
    if (++arg >= argc)
    error(1, 0, " missing local port");
    params[1] = argv[arg];
    } else if (!strcmp(argv[arg], "rip")) {
    if (++arg >= argc)
    error(1, 0, " missing remote IP");
    params[2] = argv[arg];
    } else if (!strcmp(argv[arg], "rport")) {
    if (++arg >= argc)
    error(1, 0, " missing remote port");
    params[3] = argv[arg];
    } else if (!strcmp(argv[arg], "token")) {
    if (++arg >= argc)
    error(1, 0, " missing token");
    params[4] = argv[arg];
    } else
    error(1, 0, "unknown keyword %s", argv[arg]);
    }
    for (arg = 0; arg < 4; arg = arg + 2) {
// addr header
    addr_start = off;
    addr = (void *)(data + off);
    addr.rta_type = NLA_F_NESTED |
    ((arg == 0) ? MPTCP_PM_ATTR_ADDR : MPTCP_PM_ATTR_ADDR_REMOTE);
    addr.rta_len = RTA_LENGTH(0);
    off += NLMSG_ALIGN(addr.rta_len);
// addr data
    rta = (void *)(data + off);
    if (inet_pton(AF_INET, params[arg], RTA_DATA(rta))) {
    family = AF_INET;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR4;
    rta.rta_len = RTA_LENGTH(4);
    } else if (inet_pton(AF_INET6, params[arg], RTA_DATA(rta))) {
    family = AF_INET6;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR6;
    rta.rta_len = RTA_LENGTH(16);
    } else
    error(1, errno, "can't parse ip %s", params[arg]);
    off += NLMSG_ALIGN(rta.rta_len);
// family
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FAMILY;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &family, 2);
    off += NLMSG_ALIGN(rta.rta_len);
// port
    port = atoi(params[arg + 1]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_PORT;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &port, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    addr.rta_len = off - addr_start;
    }
// token
    token = strtoul(params[4], core::ptr::null_mut(), 10);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_TOKEN;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &token, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    do_nl_req(fd, nh, off, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn csf(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int csf(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    let mut flags: u_int32_t = MPTCP_PM_ADDR_FLAG_SUBFLOW;
    const char *params[5];
    struct nlmsghdr *nh;
    struct rtattr *addr;
    struct rtattr *rta;
    u_int16_t family;
    u_int32_t token;
    u_int16_t port;
    int addr_start;
    u_int8_t id;
    let mut off: c_int = 0;
    int arg;
    memset(params, 0, 5 * sizeof(const char *));
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_SUBFLOW_CREATE,
    MPTCP_PM_VER);
    if (argc < 12)
    syntax(argv);
// Params recorded in this order:
// <local-ip>, <local-id>, <remote-ip>, <remote-port>, <token>
//
    for (arg = 2; arg < argc; arg++) {
    if (!strcmp(argv[arg], "lip")) {
    if (++arg >= argc)
    error(1, 0, " missing local IP");
    params[0] = argv[arg];
    } else if (!strcmp(argv[arg], "lid")) {
    if (++arg >= argc)
    error(1, 0, " missing local id");
    params[1] = argv[arg];
    } else if (!strcmp(argv[arg], "rip")) {
    if (++arg >= argc)
    error(1, 0, " missing remote ip");
    params[2] = argv[arg];
    } else if (!strcmp(argv[arg], "rport")) {
    if (++arg >= argc)
    error(1, 0, " missing remote port");
    params[3] = argv[arg];
    } else if (!strcmp(argv[arg], "token")) {
    if (++arg >= argc)
    error(1, 0, " missing token");
    params[4] = argv[arg];
    } else
    error(1, 0, "unknown param %s", argv[arg]);
    }
    for (arg = 0; arg < 4; arg = arg + 2) {
// addr header
    addr_start = off;
    addr = (void *)(data + off);
    addr.rta_type = NLA_F_NESTED |
    ((arg == 0) ? MPTCP_PM_ATTR_ADDR : MPTCP_PM_ATTR_ADDR_REMOTE);
    addr.rta_len = RTA_LENGTH(0);
    off += NLMSG_ALIGN(addr.rta_len);
// addr data
    rta = (void *)(data + off);
    if (inet_pton(AF_INET, params[arg], RTA_DATA(rta))) {
    family = AF_INET;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR4;
    rta.rta_len = RTA_LENGTH(4);
    } else if (inet_pton(AF_INET6, params[arg], RTA_DATA(rta))) {
    family = AF_INET6;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR6;
    rta.rta_len = RTA_LENGTH(16);
    } else
    error(1, errno, "can't parse ip %s", params[arg]);
    off += NLMSG_ALIGN(rta.rta_len);
// family
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FAMILY;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &family, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    if (arg == 2) {
// port
    port = atoi(params[arg + 1]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_PORT;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &port, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    }
    if (arg == 0) {
// id
    id = atoi(params[arg + 1]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ID;
    rta.rta_len = RTA_LENGTH(1);
    memcpy(RTA_DATA(rta), &id, 1);
    off += NLMSG_ALIGN(rta.rta_len);
    }
// addr flags
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FLAGS;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &flags, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    addr.rta_len = off - addr_start;
    }
// token
    token = strtoul(params[4], core::ptr::null_mut(), 10);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_TOKEN;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &token, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    do_nl_req(fd, nh, off, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn remove_addr(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int remove_addr(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    struct nlmsghdr *nh;
    struct rtattr *rta;
    u_int32_t token;
    u_int8_t id;
    let mut off: c_int = 0;
    int arg;
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_REMOVE,
    MPTCP_PM_VER);
    if (argc < 6)
    syntax(argv);
    for (arg = 2; arg < argc; arg++) {
    if (!strcmp(argv[arg], "id")) {
    if (++arg >= argc)
    error(1, 0, " missing id value");
    id = atoi(argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_LOC_ID;
    rta.rta_len = RTA_LENGTH(1);
    memcpy(RTA_DATA(rta), &id, 1);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "token")) {
    if (++arg >= argc)
    error(1, 0, " missing token value");
    token = strtoul(argv[arg], core::ptr::null_mut(), 10);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_TOKEN;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &token, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    } else
    error(1, 0, "unknown keyword %s", argv[arg]);
    }
    do_nl_req(fd, nh, off, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn announce_addr(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int announce_addr(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    let mut flags: u_int32_t = MPTCP_PM_ADDR_FLAG_SIGNAL;
    let mut token: u_int32_t = UINT_MAX;
    struct rtattr *rta, *addr;
    let mut id: u_int32_t = UINT_MAX;
    struct nlmsghdr *nh;
    u_int16_t family;
    int addr_start;
    let mut off: c_int = 0;
    int arg;
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_ANNOUNCE,
    MPTCP_PM_VER);
    if (argc < 7)
    syntax(argv);
// local-ip header
    addr_start = off;
    addr = (void *)(data + off);
    addr.rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    addr.rta_len = RTA_LENGTH(0);
    off += NLMSG_ALIGN(addr.rta_len);
// local-ip data
// record addr type
    rta = (void *)(data + off);
    if (inet_pton(AF_INET, argv[2], RTA_DATA(rta))) {
    family = AF_INET;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR4;
    rta.rta_len = RTA_LENGTH(4);
    } else if (inet_pton(AF_INET6, argv[2], RTA_DATA(rta))) {
    family = AF_INET6;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR6;
    rta.rta_len = RTA_LENGTH(16);
    } else
    error(1, errno, "can't parse ip %s", argv[2]);
    off += NLMSG_ALIGN(rta.rta_len);
// addr family
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FAMILY;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &family, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    for (arg = 3; arg < argc; arg++) {
    if (!strcmp(argv[arg], "id")) {
// local-id
    if (++arg >= argc)
    error(1, 0, " missing id value");
    id = atoi(argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ID;
    rta.rta_len = RTA_LENGTH(1);
    memcpy(RTA_DATA(rta), &id, 1);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "dev")) {
// for the if_index
    int32_t ifindex;
    if (++arg >= argc)
    error(1, 0, " missing dev name");
    ifindex = if_nametoindex(argv[arg]);
    if (!ifindex)
    error(1, errno, "unknown device %s", argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_IF_IDX;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &ifindex, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "port")) {
// local-port (optional)
    u_int16_t port;
    if (++arg >= argc)
    error(1, 0, " missing port value");
    port = atoi(argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_PORT;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &port, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "token")) {
// MPTCP connection token
    if (++arg >= argc)
    error(1, 0, " missing token value");
    token = strtoul(argv[arg], core::ptr::null_mut(), 10);
    } else
    error(1, 0, "unknown keyword %s", argv[arg]);
    }
// addr flags
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FLAGS;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &flags, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    addr.rta_len = off - addr_start;
    if (id == UINT_MAX || token == UINT_MAX)
    error(1, 0, " missing mandatory inputs");
// token
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_TOKEN;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &token, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    do_nl_req(fd, nh, off, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn add_addr(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int add_addr(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    struct rtattr *rta, *nest;
    struct nlmsghdr *nh;
    let mut flags: u_int32_t = 0;
    u_int16_t family;
    int nest_start;
    u_int8_t id;
    let mut off: c_int = 0;
    int arg;
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_ADD_ADDR,
    MPTCP_PM_VER);
    if (argc < 3)
    syntax(argv);
    nest_start = off;
    nest = (void *)(data + off);
    nest.rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    nest.rta_len = RTA_LENGTH(0);
    off += NLMSG_ALIGN(nest.rta_len);
// addr data
    rta = (void *)(data + off);
    if (inet_pton(AF_INET, argv[2], RTA_DATA(rta))) {
    family = AF_INET;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR4;
    rta.rta_len = RTA_LENGTH(4);
    } else if (inet_pton(AF_INET6, argv[2], RTA_DATA(rta))) {
    family = AF_INET6;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR6;
    rta.rta_len = RTA_LENGTH(16);
    } else
    error(1, errno, "can't parse ip %s", argv[2]);
    off += NLMSG_ALIGN(rta.rta_len);
// family
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FAMILY;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &family, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    for (arg = 3; arg < argc; arg++) {
    if (!strcmp(argv[arg], "flags")) {
    char *tok, *str;
// flags
    if (++arg >= argc)
    error(1, 0, " missing flags value");
// do not support flag list yet
    for (str = argv[arg]; (tok = strtok(str, ","));
    str = core::ptr::null_mut()) {
    if (!strcmp(tok, "subflow"))
    flags |= MPTCP_PM_ADDR_FLAG_SUBFLOW;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tok, _arg: "signal")) -> else {
    else if (!strcmp(tok, "signal"))
    flags |= MPTCP_PM_ADDR_FLAG_SIGNAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tok, _arg: "laminar")) -> else {
    else if (!strcmp(tok, "laminar"))
    flags |= MPTCP_PM_ADDR_FLAG_LAMINAR;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tok, _arg: "backup")) -> else {
    else if (!strcmp(tok, "backup"))
    flags |= MPTCP_PM_ADDR_FLAG_BACKUP;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tok, _arg: "fullmesh")) -> else {
    else if (!strcmp(tok, "fullmesh"))
    flags |= MPTCP_PM_ADDR_FLAG_FULLMESH;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tok, _arg: "unknown")) -> else {
    else if (!strcmp(tok, "unknown"))
    flags |= MPTCP_PM_ADDR_FLAG_UNKNOWN;
    else
    error(1, errno,
    "unknown flag %s", argv[arg]);
    }
    if (flags & MPTCP_PM_ADDR_FLAG_SIGNAL &&
    flags & MPTCP_PM_ADDR_FLAG_FULLMESH) {
    error(1, errno, "error flag fullmesh");
    }
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FLAGS;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &flags, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "id")) {
    if (++arg >= argc)
    error(1, 0, " missing id value");
    id = atoi(argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ID;
    rta.rta_len = RTA_LENGTH(1);
    memcpy(RTA_DATA(rta), &id, 1);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "dev")) {
    int32_t ifindex;
    if (++arg >= argc)
    error(1, 0, " missing dev name");
    ifindex = if_nametoindex(argv[arg]);
    if (!ifindex)
    error(1, errno, "unknown device %s", argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_IF_IDX;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &ifindex, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "port")) {
    u_int16_t port;
    if (++arg >= argc)
    error(1, 0, " missing port value");
    if (!(flags & MPTCP_PM_ADDR_FLAG_SIGNAL))
    error(1, 0, " flags must be signal when using port");
    port = atoi(argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_PORT;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &port, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    } else
    error(1, 0, "unknown keyword %s", argv[arg]);
    }
    nest.rta_len = off - nest_start;
    do_nl_req(fd, nh, off, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn del_addr(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int del_addr(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    struct rtattr *rta, *nest;
    struct nlmsghdr *nh;
    u_int16_t family;
    int nest_start;
    u_int8_t id;
    let mut off: c_int = 0;
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_DEL_ADDR,
    MPTCP_PM_VER);
// the only argument is the address id (nonzero)
    if (argc != 3 && argc != 4)
    syntax(argv);
    id = atoi(argv[2]);
// zero id with the IP address
    if (!id && argc != 4)
    syntax(argv);
    nest_start = off;
    nest = (void *)(data + off);
    nest.rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    nest.rta_len =  RTA_LENGTH(0);
    off += NLMSG_ALIGN(nest.rta_len);
// build a dummy addr with only the ID set
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ID;
    rta.rta_len = RTA_LENGTH(1);
    memcpy(RTA_DATA(rta), &id, 1);
    off += NLMSG_ALIGN(rta.rta_len);
    if (!id) {
// addr data
    rta = (void *)(data + off);
    if (inet_pton(AF_INET, argv[3], RTA_DATA(rta))) {
    family = AF_INET;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR4;
    rta.rta_len = RTA_LENGTH(4);
    } else if (inet_pton(AF_INET6, argv[3], RTA_DATA(rta))) {
    family = AF_INET6;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR6;
    rta.rta_len = RTA_LENGTH(16);
    } else {
    error(1, errno, "can't parse ip %s", argv[3]);
    }
    off += NLMSG_ALIGN(rta.rta_len);
// family
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FAMILY;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &family, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    }
    nest.rta_len = off - nest_start;
    do_nl_req(fd, nh, off, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn print_addr(attrs: *mut rtattr, len: c_int) {
    static void print_addr(struct rtattr *attrs, int len)
    {
    let mut family: u16 = 0;
    let mut port: u16 = 0;
    char str[1024];
    uint32_t flags;
    uint8_t id;
    while (RTA_OK(attrs, len)) {
    if (attrs.rta_type == MPTCP_PM_ADDR_ATTR_FAMILY)
    memcpy(&family, RTA_DATA(attrs), 2);
    if (attrs.rta_type == MPTCP_PM_ADDR_ATTR_PORT)
    memcpy(&port, RTA_DATA(attrs), 2);
    if (attrs.rta_type == MPTCP_PM_ADDR_ATTR_ADDR4) {
    if (family != AF_INET)
    error(1, errno, "wrong IP (v4) for family %d",
    family);
    inet_ntop(AF_INET, RTA_DATA(attrs), str, sizeof(str));
    printf("%s", str);
    if (port)
    printf(" %d", port);
    }
    if (attrs.rta_type == MPTCP_PM_ADDR_ATTR_ADDR6) {
    if (family != AF_INET6)
    error(1, errno, "wrong IP (v6) for family %d",
    family);
    inet_ntop(AF_INET6, RTA_DATA(attrs), str, sizeof(str));
    printf("%s", str);
    if (port)
    printf(" %d", port);
    }
    if (attrs.rta_type == MPTCP_PM_ADDR_ATTR_ID) {
    memcpy(&id, RTA_DATA(attrs), 1);
    printf("id %d ", id);
    }
    if (attrs.rta_type == MPTCP_PM_ADDR_ATTR_FLAGS) {
    memcpy(&flags, RTA_DATA(attrs), 4);
    printf("flags ");
    if (flags & MPTCP_PM_ADDR_FLAG_SIGNAL) {
    printf("signal");
    flags &= ~MPTCP_PM_ADDR_FLAG_SIGNAL;
    if (flags)
    printf(",");
    }
    if (flags & MPTCP_PM_ADDR_FLAG_SUBFLOW) {
    printf("subflow");
    flags &= ~MPTCP_PM_ADDR_FLAG_SUBFLOW;
    if (flags)
    printf(",");
    }
    if (flags & MPTCP_PM_ADDR_FLAG_LAMINAR) {
    printf("laminar");
    flags &= ~MPTCP_PM_ADDR_FLAG_LAMINAR;
    if (flags)
    printf(",");
    }
    if (flags & MPTCP_PM_ADDR_FLAG_BACKUP) {
    printf("backup");
    flags &= ~MPTCP_PM_ADDR_FLAG_BACKUP;
    if (flags)
    printf(",");
    }
    if (flags & MPTCP_PM_ADDR_FLAG_FULLMESH) {
    printf("fullmesh");
    flags &= ~MPTCP_PM_ADDR_FLAG_FULLMESH;
    if (flags)
    printf(",");
    }
    if (flags & MPTCP_PM_ADDR_FLAG_IMPLICIT) {
    printf("implicit");
    flags &= ~MPTCP_PM_ADDR_FLAG_IMPLICIT;
    if (flags)
    printf(",");
    }
    if (flags & MPTCP_PM_ADDR_FLAG_UNKNOWN) {
    printf("unknown");
    flags &= ~MPTCP_PM_ADDR_FLAG_UNKNOWN;
    if (flags)
    printf(",");
    }
// bump unknown flags, if any
    if (flags)
    printf("0x%x", flags);
    printf(" ");
    }
    if (attrs.rta_type == MPTCP_PM_ADDR_ATTR_IF_IDX) {
    char name[IF_NAMESIZE], *ret;
    int32_t ifindex;
    memcpy(&ifindex, RTA_DATA(attrs), 4);
    ret = if_indextoname(ifindex, name);
    if (ret)
    printf("dev %s ", ret);
    else
    printf("dev unknown/%d", ifindex);
    }
    attrs = RTA_NEXT(attrs, len);
    }
    printf("\n");
    }
#[no_mangle]
unsafe extern "C" fn print_addrs(nh: *mut nlmsghdr, pm_family: c_int, total_len: c_int) {
    static void print_addrs(struct nlmsghdr *nh, int pm_family, int total_len)
    {
    struct rtattr *attrs;
    for (; NLMSG_OK(nh, total_len); nh = NLMSG_NEXT(nh, total_len)) {
    let mut len: c_int = nh.nlmsg_len;
    if (nh.nlmsg_type == NLMSG_DONE)
    break;
    if (nh.nlmsg_type == NLMSG_ERROR)
    nl_error(nh);
    if (nh.nlmsg_type != pm_family)
    continue;
    len -= NLMSG_LENGTH(GENL_HDRLEN);
    attrs = (struct rtattr *) ((char *) NLMSG_DATA(nh) +
    GENL_HDRLEN);
    while (RTA_OK(attrs, len)) {
    if (attrs.rta_type ==
    (MPTCP_PM_ATTR_ADDR | NLA_F_NESTED))
    print_addr((void *)RTA_DATA(attrs),
    attrs.rta_len);
    attrs = RTA_NEXT(attrs, len);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_addr(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int get_addr(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    struct rtattr *rta, *nest;
    struct nlmsghdr *nh;
    let mut token: u_int32_t = 0;
    int nest_start;
    u_int8_t id;
    let mut off: c_int = 0;
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_GET_ADDR,
    MPTCP_PM_VER);
// the only argument is the address id
    if (argc != 3 && argc != 5)
    syntax(argv);
    id = atoi(argv[2]);
    if (argc == 5 && !strcmp(argv[3], "token"))
    token = strtoul(argv[4], core::ptr::null_mut(), 10);
    nest_start = off;
    nest = (void *)(data + off);
    nest.rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    nest.rta_len =  RTA_LENGTH(0);
    off += NLMSG_ALIGN(nest.rta_len);
// build a dummy addr with only the ID set
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ID;
    rta.rta_len = RTA_LENGTH(1);
    memcpy(RTA_DATA(rta), &id, 1);
    off += NLMSG_ALIGN(rta.rta_len);
    nest.rta_len = off - nest_start;
// token
    if (token) {
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_TOKEN;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &token, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    }
    print_addrs(nh, pm_family, do_nl_req(fd, nh, off, sizeof(data)));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dump_addrs(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int dump_addrs(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    let mut pid: pid_t = getpid();
    struct nlmsghdr *nh;
    let mut token: u_int32_t = 0;
    struct rtattr *rta;
    let mut off: c_int = 0;
    if (argc != 2 && argc != 4)
    syntax(argv);
    if (argc == 4 && !strcmp(argv[2], "token"))
    token = strtoul(argv[3], core::ptr::null_mut(), 10);
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_GET_ADDR,
    MPTCP_PM_VER);
    nh.nlmsg_flags |= NLM_F_DUMP;
    nh.nlmsg_seq = 1;
    nh.nlmsg_pid = pid;
    nh.nlmsg_len = off;
// token
    if (token) {
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_TOKEN;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &token, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    }
    print_addrs(nh, pm_family, do_nl_req(fd, nh, off, sizeof(data)));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn flush_addrs(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int flush_addrs(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    struct nlmsghdr *nh;
    let mut off: c_int = 0;
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_FLUSH_ADDRS,
    MPTCP_PM_VER);
    do_nl_req(fd, nh, off, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn print_limits(nh: *mut nlmsghdr, pm_family: c_int, total_len: c_int) {
    static void print_limits(struct nlmsghdr *nh, int pm_family, int total_len)
    {
    struct rtattr *attrs;
    uint32_t max;
    for (; NLMSG_OK(nh, total_len); nh = NLMSG_NEXT(nh, total_len)) {
    let mut len: c_int = nh.nlmsg_len;
    if (nh.nlmsg_type == NLMSG_DONE)
    break;
    if (nh.nlmsg_type == NLMSG_ERROR)
    nl_error(nh);
    if (nh.nlmsg_type != pm_family)
    continue;
    len -= NLMSG_LENGTH(GENL_HDRLEN);
    attrs = (struct rtattr *) ((char *) NLMSG_DATA(nh) +
    GENL_HDRLEN);
    while (RTA_OK(attrs, len)) {
    let mut type: c_int = attrs.rta_type;
    if (type != MPTCP_PM_ATTR_RCV_ADD_ADDRS &&
    type != MPTCP_PM_ATTR_SUBFLOWS)
    goto next;
    memcpy(&max, RTA_DATA(attrs), 4);
    printf("%s %u\n", type == MPTCP_PM_ATTR_SUBFLOWS ?
    "subflows" : "accept", max);
    next:
    attrs = RTA_NEXT(attrs, len);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_set_limits(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int get_set_limits(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    let mut rcv_addr: u32 = 0, subflows = 0;
    int cmd, len = sizeof(data);
    struct nlmsghdr *nh;
    let mut off: c_int = 0;
// limit
    if (argc == 4) {
    rcv_addr = atoi(argv[2]);
    subflows = atoi(argv[3]);
    cmd = MPTCP_PM_CMD_SET_LIMITS;
    } else {
    cmd = MPTCP_PM_CMD_GET_LIMITS;
    }
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, cmd, MPTCP_PM_VER);
// limit
    if (cmd == MPTCP_PM_CMD_SET_LIMITS) {
    struct rtattr *rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_RCV_ADD_ADDRS;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &rcv_addr, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_SUBFLOWS;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &subflows, 4);
    off += NLMSG_ALIGN(rta.rta_len);
// do not expect a reply
    len = 0;
    }
    len = do_nl_req(fd, nh, off, len);
    if (cmd == MPTCP_PM_CMD_GET_LIMITS)
    print_limits(nh, pm_family, len);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn add_listener(argc: c_int, argv[]: *mut c_char) -> c_int {
    int add_listener(int argc, char *argv[])
    {
    struct sockaddr_storage addr;
    struct sockaddr_in6 *a6;
    struct sockaddr_in *a4;
    let mut family: u_int16_t = AF_UNSPEC;
    let mut enable: c_int = 1;
    int sock;
    int err;
    if (argc < 4)
    syntax(argv);
    memset(&addr, 0, sizeof(struct sockaddr_storage));
    a4 = (struct sockaddr_in *)&addr;
    a6 = (struct sockaddr_in6 *)&addr;
    if (inet_pton(AF_INET, argv[2], &a4.sin_addr)) {
    family = AF_INET;
    a4.sin_family = family;
    a4.sin_port = htons(atoi(argv[3]));
    } else if (inet_pton(AF_INET6, argv[2], &a6.sin6_addr)) {
    family = AF_INET6;
    a6.sin6_family = family;
    a6.sin6_port = htons(atoi(argv[3]));
    } else
    error(1, errno, "can't parse ip %s", argv[2]);
    sock = socket(family, SOCK_STREAM, IPPROTO_MPTCP);
    if (sock < 0)
    error(1, errno, "can't create listener sock\n");
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &enable, sizeof(enable))) {
    close(sock);
    error(1, errno, "can't set SO_REUSEADDR on listener sock\n");
    }
    err = bind(sock, (struct sockaddr *)&addr,
    ((family == AF_INET) ? sizeof(struct sockaddr_in) :
    sizeof(struct sockaddr_in6)));
    if (err == 0 && listen(sock, 30) == 0)
    pause();
    close(sock);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn set_flags(fd: c_int, pm_family: c_int, argc: c_int, argv[]: *mut c_char) -> c_int {
    int set_flags(int fd, int pm_family, int argc, char *argv[])
    {
    char data[NLMSG_ALIGN(sizeof(struct nlmsghdr)) +
    NLMSG_ALIGN(sizeof(struct genlmsghdr)) +
    1024];
    struct rtattr *rta, *nest;
    struct nlmsghdr *nh;
    let mut flags: u_int32_t = 0;
    let mut token: u_int32_t = 0;
    let mut rport: u_int16_t = 0;
    u_int16_t family;
    void *rip = core::ptr::null_mut();
    int nest_start;
    let mut use_id: c_int = 0;
    u_int8_t id;
    let mut off: c_int = 0;
    let mut arg: c_int = 2;
    memset(data, 0, sizeof(data));
    nh = (void *)data;
    off = init_genl_req(data, pm_family, MPTCP_PM_CMD_SET_FLAGS,
    MPTCP_PM_VER);
    if (argc < 3)
    syntax(argv);
    nest_start = off;
    nest = (void *)(data + off);
    nest.rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR;
    nest.rta_len = RTA_LENGTH(0);
    off += NLMSG_ALIGN(nest.rta_len);
    if (!strcmp(argv[arg], "id")) {
    if (++arg >= argc)
    error(1, 0, " missing id value");
    use_id = 1;
    id = atoi(argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ID;
    rta.rta_len = RTA_LENGTH(1);
    memcpy(RTA_DATA(rta), &id, 1);
    off += NLMSG_ALIGN(rta.rta_len);
    } else {
// addr data
    rta = (void *)(data + off);
    if (inet_pton(AF_INET, argv[arg], RTA_DATA(rta))) {
    family = AF_INET;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR4;
    rta.rta_len = RTA_LENGTH(4);
    } else if (inet_pton(AF_INET6, argv[arg], RTA_DATA(rta))) {
    family = AF_INET6;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR6;
    rta.rta_len = RTA_LENGTH(16);
    } else {
    error(1, errno, "can't parse ip %s", argv[arg]);
    }
    off += NLMSG_ALIGN(rta.rta_len);
// family
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FAMILY;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &family, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    }
    if (++arg >= argc)
    error(1, 0, " missing flags keyword");
    for (; arg < argc; arg++) {
    if (!strcmp(argv[arg], "token")) {
    if (++arg >= argc)
    error(1, 0, " missing token value");
// token
    token = strtoul(argv[arg], core::ptr::null_mut(), 10);
    } else if (!strcmp(argv[arg], "flags")) {
    char *tok, *str;
// flags
    if (++arg >= argc)
    error(1, 0, " missing flags value");
    for (str = argv[arg]; (tok = strtok(str, ","));
    str = core::ptr::null_mut()) {
    if (!strcmp(tok, "backup"))
    flags |= MPTCP_PM_ADDR_FLAG_BACKUP;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(tok, _arg: "fullmesh")) -> else {
    else if (!strcmp(tok, "fullmesh"))
    flags |= MPTCP_PM_ADDR_FLAG_FULLMESH;
    else if (strcmp(tok, "nobackup") &&
    strcmp(tok, "nofullmesh"))
    error(1, errno,
    "unknown flag %s", argv[arg]);
    }
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FLAGS;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &flags, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "port")) {
    u_int16_t port;
    if (use_id)
    error(1, 0, " port can't be used with id");
    if (++arg >= argc)
    error(1, 0, " missing port value");
    port = atoi(argv[arg]);
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_PORT;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &port, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    } else if (!strcmp(argv[arg], "rport")) {
    if (++arg >= argc)
    error(1, 0, " missing remote port");
    rport = atoi(argv[arg]);
    } else if (!strcmp(argv[arg], "rip")) {
    if (++arg >= argc)
    error(1, 0, " missing remote ip");
    rip = argv[arg];
    } else {
    error(1, 0, "unknown keyword %s", argv[arg]);
    }
    }
    nest.rta_len = off - nest_start;
// token
    if (token) {
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ATTR_TOKEN;
    rta.rta_len = RTA_LENGTH(4);
    memcpy(RTA_DATA(rta), &token, 4);
    off += NLMSG_ALIGN(rta.rta_len);
    }
// remote addr/port
    if (rip) {
    nest_start = off;
    nest = (void *)(data + off);
    nest.rta_type = NLA_F_NESTED | MPTCP_PM_ATTR_ADDR_REMOTE;
    nest.rta_len = RTA_LENGTH(0);
    off += NLMSG_ALIGN(nest.rta_len);
// addr data
    rta = (void *)(data + off);
    if (inet_pton(AF_INET, rip, RTA_DATA(rta))) {
    family = AF_INET;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR4;
    rta.rta_len = RTA_LENGTH(4);
    } else if (inet_pton(AF_INET6, rip, RTA_DATA(rta))) {
    family = AF_INET6;
    rta.rta_type = MPTCP_PM_ADDR_ATTR_ADDR6;
    rta.rta_len = RTA_LENGTH(16);
    } else {
    error(1, errno, "can't parse ip %s", (char *)rip);
    }
    off += NLMSG_ALIGN(rta.rta_len);
// family
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_FAMILY;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &family, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    if (rport) {
    rta = (void *)(data + off);
    rta.rta_type = MPTCP_PM_ADDR_ATTR_PORT;
    rta.rta_len = RTA_LENGTH(2);
    memcpy(RTA_DATA(rta), &rport, 2);
    off += NLMSG_ALIGN(rta.rta_len);
    }
    nest.rta_len = off - nest_start;
    }
    do_nl_req(fd, nh, off, 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int events_mcast_grp;
    int pm_family;
    int fd;
    if (argc < 2)
    syntax(argv);
    fd = socket(AF_NETLINK, SOCK_RAW, NETLINK_GENERIC);
    if (fd == -1)
    error(1, errno, "socket netlink");
    resolve_mptcp_pm_netlink(fd, &pm_family, &events_mcast_grp);
    if (!strcmp(argv[1], "add"))
    return add_addr(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "ann")) -> else {
    else if (!strcmp(argv[1], "ann"))
    return announce_addr(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "rem")) -> else {
    else if (!strcmp(argv[1], "rem"))
    return remove_addr(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "csf")) -> else {
    else if (!strcmp(argv[1], "csf"))
    return csf(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "dsf")) -> else {
    else if (!strcmp(argv[1], "dsf"))
    return dsf(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "del")) -> else {
    else if (!strcmp(argv[1], "del"))
    return del_addr(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "flush")) -> else {
    else if (!strcmp(argv[1], "flush"))
    return flush_addrs(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "get")) -> else {
    else if (!strcmp(argv[1], "get"))
    return get_addr(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "dump")) -> else {
    else if (!strcmp(argv[1], "dump"))
    return dump_addrs(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "limits")) -> else {
    else if (!strcmp(argv[1], "limits"))
    return get_set_limits(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "set")) -> else {
    else if (!strcmp(argv[1], "set"))
    return set_flags(fd, pm_family, argc, argv);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "events")) -> else {
    else if (!strcmp(argv[1], "events"))
    return capture_events(fd, events_mcast_grp);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(argv[1], _arg: "listen")) -> else {
    else if (!strcmp(argv[1], "listen"))
    return add_listener(argc, argv);
    fprintf(stderr, "unknown sub-command: %s", argv[1]);
    syntax(argv);
    return 0;
    }
