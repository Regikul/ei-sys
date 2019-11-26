//! Low level bindings for ei, a library to handle the Erlang external term format and to
//! communicate with distributed Erlang nodes.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::{ffi::c_void, ops};
use in_addr;
use libc::{
  c_char,
  c_uchar,
  c_int,
  c_uint,
  c_long,
  c_ulong,
  c_longlong,
  c_short,
  c_ulonglong,
  c_double,
};

pub const ERL_ERROR: c_int = -1;
pub const ERL_NO_DAEMON: c_int = -2;
pub const ERL_NO_PORT: c_int = -3;
pub const ERL_CONNECT_FAIL: c_int = -4;
pub const ERL_TIMEOUT: c_int = -5;
pub const ERL_NO_REMOTE: c_int = -6;

pub const ERL_TICK: c_int = 0;
pub const ERL_MSG: c_int = 1;

pub const ERL_NO_TIMEOUT: c_int = -1;

pub const ERL_LINK: u8 = 1;
pub const ERL_SEND: u8 = 2;
pub const ERL_EXIT: u8 = 3;
pub const ERL_UNLINK: u8 = 4;
pub const ERL_NODE_LINK: u8 = 5;
pub const ERL_REG_SEND: u8 = 6;
pub const ERL_GROUP_LEADER: u8 = 7;
pub const ERL_EXIT2: u8 = 8;
pub const ERL_PASS_THROUGH: u8 = b'p';

pub const ERL_SEND_TT: u8 = 12;
pub const ERL_EXIT_TT: u8 = 13;
pub const ERL_REG_SEND_TT: u8 = 16;
pub const ERL_EXIT2_TT: u8 = 18;
pub const ERL_MONITOR_P: u8 = 19;
pub const ERL_DEMONITOR_P: u8 = 20;
pub const ERL_MONITOR_P_EXIT: u8 = 21;

pub const ERL_SMALL_INTEGER_EXT: u8 = b'a';
pub const ERL_INTEGER_EXT: u8 = b'b';
pub const ERL_FLOAT_EXT: u8 = b'c';
pub const NEW_FLOAT_EXT: u8 = b'F';
pub const ERL_ATOM_EXT: u8 = b'd';
pub const ERL_SMALL_ATOM_EXT: u8 = b's';
pub const ERL_ATOM_UTF8_EXT: u8 = b'v';
pub const ERL_SMALL_ATOM_UTF8_EXT: u8 = b'w';
pub const ERL_REFERENCE_EXT: u8 = b'e';
pub const ERL_NEW_REFERENCE_EXT: u8 = b'r';
pub const ERL_PORT_EXT: u8 = b'f';
pub const ERL_PID_EXT: u8 = b'g';
pub const ERL_SMALL_TUPLE_EXT: u8 = b'h';
pub const ERL_LARGE_TUPLE_EXT: u8 = b'i';
pub const ERL_NIL_EXT: u8 = b'j';
pub const ERL_STRING_EXT: u8 = b'k';
pub const ERL_LIST_EXT: u8 = b'l';
pub const ERL_BINARY_EXT: u8 = b'm';
pub const ERL_SMALL_BIG_EXT: u8 = b'n';
pub const ERL_LARGE_BIG_EXT: u8 = b'o';
pub const ERL_NEW_FUN_EXT: u8 = b'p';
pub const ERL_MAP_EXT: u8 = b't';
pub const ERL_FUN_EXT: u8 = b'u';

pub const ERL_NEW_CACHE: u8 = b'N';
pub const ERL_CACHED_ATOM: u8 = b'C';

pub const EI_MAXHOSTNAMELEN: usize = 64;
pub const EI_MAXALIVELEN: usize = 63;
pub const EI_MAX_COOKIE_SIZE: usize = 512;
pub const MAXATOMLEN: usize = 255 + 1;
pub const MAXATOMLEN_UTF8: usize = 255 * 4 + 1;
pub const MAXNODELEN: usize = EI_MAXALIVELEN + 1 + EI_MAXHOSTNAMELEN;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct erlang_char_encoding(pub u32);

pub const ERLANG_ASCII: erlang_char_encoding = erlang_char_encoding(1);
pub const ERLANG_LATIN1: erlang_char_encoding = erlang_char_encoding(2);
pub const ERLANG_UTF8: erlang_char_encoding = erlang_char_encoding(4);

impl ops::BitOr<erlang_char_encoding> for erlang_char_encoding {
  type Output = Self;

  #[inline]
  fn bitor(self, other: Self) -> Self {
    erlang_char_encoding(self.0 | other.0)
  }
}

impl ops::BitOrAssign for erlang_char_encoding {
  #[inline]
  fn bitor_assign(&mut self, rhs: erlang_char_encoding) {
    self.0 |= rhs.0;
  }
}

impl ops::BitAnd<erlang_char_encoding> for erlang_char_encoding {
  type Output = Self;

  #[inline]
  fn bitand(self, other: Self) -> Self {
    erlang_char_encoding(self.0 & other.0)
  }
}

impl ops::BitAndAssign for erlang_char_encoding {
  #[inline]
  fn bitand_assign(&mut self, rhs: erlang_char_encoding) {
    self.0 &= rhs.0;
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erlang_pid {
  pub node: [c_char; MAXATOMLEN_UTF8],
  pub num: c_uint,
  pub serial: c_uint,
  pub creation: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erlang_port {
  pub node: [c_char; MAXATOMLEN_UTF8],
  pub id: c_uint,
  pub creation: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erlang_ref {
  pub node: [c_char; MAXATOMLEN_UTF8],
  pub len: c_int,
  pub n: [c_uint; 3],
  pub creation: c_uint,
}

#[repr(C)]
#[derive(Clone)]
pub struct erlang_trace {
  pub serial: c_long,
  pub prev: c_long,
  pub from: erlang_pid,
  pub label: c_long,
  pub flags: c_long,
}

#[repr(C)]
#[derive(Clone)]
pub struct erlang_msg {
  pub msgtype: c_long,
  pub from: erlang_pid,
  pub to: erlang_pid,
  pub toname: [c_char; MAXATOMLEN_UTF8],
  pub cookie: [c_char; MAXATOMLEN_UTF8],
  pub token: erlang_trace,
}

#[repr(C)]
#[derive(Clone)]
pub struct erlang_fun {
  pub arity: c_long,
  pub module: [c_char; MAXATOMLEN_UTF8],
  pub module_org_enc: erlang_char_encoding,
  pub md5: [c_char; 16],
  pub index: c_long,
  pub old_index: c_long,
  pub uniq: c_long,
  pub n_free_vars: c_long,
  pub pid: erlang_pid,
  pub free_var_len: c_long,
  pub free_vars: *mut c_char,
}

#[repr(C)]
#[derive(Clone)]
pub struct erlang_big {
  pub arity: c_uint,
  pub is_neg: c_int,
  pub digits: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union term_value {
  pub i_val: c_long,
  pub d_val: c_double,
  pub atom_name: [c_char; MAXATOMLEN_UTF8],
  pid: erlang_pid,
  port: erlang_port,
  ref_: erlang_ref,
}

#[repr(C)]
#[derive(Clone)]
pub struct ei_term {
  pub ie_type: c_char,
  pub arity: c_int,
  pub size: c_int,
  pub value: term_value
}

#[repr(C)]
#[derive(Clone)]
pub struct ErlConnect {
  pub ipadr: [c_char; 4],
  pub nodename: [c_char; MAXNODELEN + 1],
}

#[repr(C)]
#[derive(Clone)]
pub struct ei_cnode {
  pub thishostname: [c_char; EI_MAXHOSTNAMELEN + 1],
  pub thisnodename: [c_char; MAXNODELEN + 1],
  pub thisalivename: [c_char; EI_MAXALIVELEN + 1],
  pub ei_connect_cookie: [c_char; EI_MAX_COOKIE_SIZE + 1],
  pub creation: c_short,
  pub self_: erlang_pid,
}

type Erl_IpAddr = in_addr::in_addr;

#[repr(C)]
#[derive(Clone)]
pub struct ei_x_buff {
  pub buff: *mut c_char,
  pub buffsz: c_int,
  pub index: c_int,
}

pub unsafe fn ei_encode_empty_list(buf: *mut c_char, i: *mut c_int) -> c_int {
    ei_encode_list_header(buf, i, 0)
}

pub const EI_SMALLKEY: usize = 32;

#[repr(C)]
#[derive(Clone)]
pub struct ei_bucket {
  pub rawhash: c_int,
  pub key: *const c_char,
  pub keybuf: [c_char; EI_SMALLKEY],
  pub value: *const c_void,
  pub next: *mut ei_bucket,
}

#[repr(C)]
#[derive(Clone)]
pub struct ei_hash {
  pub tab: *mut *mut ei_bucket,
  pub hash: extern "C" fn (buf: *const c_char) -> c_int,
  pub size: c_int,
  pub nelem: c_int,
  pub npos: c_int,
  pub freelist: *mut ei_bucket,
}

pub const EI_DIRTY: u32 = 0x01;
pub const EI_DELET: u32 = 0x02;
pub const EI_INT: u32 = 0x10;
pub const EI_FLT: u32 = 0x20;
pub const EI_STR: u32 = 0x40;
pub const EI_BIN: u32 = 0x80;

#[repr(C)]
#[derive(Copy, Clone)]
pub union ei_val {
  pub i: c_long,
  pub f: c_double,
  pub s: *mut c_char,
  pub p: *mut c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct ei_reg_obj {
  pub attr: c_int,
  pub size: c_int,
  pub val: ei_val,
  pub next: *mut ei_reg_obj,
}

#[repr(C)]
#[derive(Clone)]
pub struct ei_reg {
  pub freelist: *mut ei_reg_obj,
  pub tab: *mut ei_hash,
}

#[repr(C)]
#[derive(Clone)]
pub struct ei_reg_stat {
  pub attr: c_int,
  pub size: c_int,
}

#[repr(C)]
#[derive(Clone)]
pub struct ei_reg_tabstat {
  pub size: c_int,
  pub nelem: c_int,
  pub npos: c_int,
  pub collisions: c_int,
}

pub const EI_FORCE: u32 = 0x1;
pub const EI_NOPURGE: u32 = 0x2;

extern "C" {
  /// The initial capacity of an [`ei_x_buff`] in bytes, when created with [`ei_x_new`].
  ///
  /// [`ei_x_buff`]: type.ei_x_buff.html
  /// [`ei_x_new`]: fn.ei_x_new.html
  pub static mut ei_x_extra: c_int;

  /// Returns a pointer to the thread-local storage where `erl_errno` is stored.
  pub fn __erl_errno_place() -> *mut c_int;

  pub fn ei_connect_init(
    ec: *mut ei_cnode,
    this_node_name: *const c_char,
    cookie: *const c_char,
    creation: c_short,
  ) -> c_int;

  pub fn ei_connect_xinit(
    ec: *mut ei_cnode,
    thishostname: *const c_char,
    thisalivename: *const c_char,
    thisnodename: *const c_char,
    thisipaddr: *mut Erl_IpAddr,
    cookie: *const c_char,
    creation: c_short,
  ) -> c_int;

  pub fn ei_connect(ec: *mut ei_cnode, nodename: *mut c_char) -> c_int;

  pub fn ei_connect_tmo(ec: *mut ei_cnode, nodename: *mut c_char, ms: c_uint) -> c_int;

  /// Connects to an Erlang node, without a timeout.
  ///
  /// Refer to the documentation of [`ei_xconnect_tmo`] for more information on safety and return
  /// values.
  ///
  /// [`ei_xconnect_tmo`]: fn.ei_xconnect_tmo.html
  ///
  /// # See Also
  ///
  /// [The official entry for this function in the Erlang documentation.](http://erlang.org/doc/man/ei_connect.html#ei_xconnect)
  pub fn ei_xconnect(
    node: *mut ei_cnode,
    remote_host: *mut Erl_IpAddr,
    remote_name: *mut c_char,
  ) -> c_int;

  /// Connects to an Erlang node.
  ///
  /// # Safety
  ///
  /// * The `node` must have been successfuly initialized by [`ei_connect_init`] or
  ///   [`ei_connect_xinit`].
  /// * The `remote_host` must be a non-null pointer to a `libc::in_addr`.
  /// * The `remote_name` must be a non-null pointer to a null-terminated string.
  ///
  /// [`ei_connect_init`]: fn.ei_connect_init.html
  /// [`ei_connect_xinit`]: fn.ei_connect_xinit.html
  ///
  /// # Returns
  ///
  /// * The file descriptor of the socket, a non-zero integer, on success.
  /// * [`ERL_NO_PORT`], if the port could not be resolved from the EPMD instance.
  ///
  /// [`ERL_NO_PORT`]: constant.ERL_NO_PORT.html
  ///
  /// # See Also
  ///
  /// [The official entry for this function in the Erlang documentation.](http://erlang.org/doc/man/ei_connect.html#ei_xconnect_tmo)
  pub fn ei_xconnect_tmo(
    node: *mut ei_cnode,
    remote_host: *mut Erl_IpAddr,
    remote_name: *mut c_char,
    ms: c_uint,
  ) -> c_int;

  pub fn ei_receive(fd: c_int, bufp: *mut c_uchar, bufsize: c_int) -> c_int;

  pub fn ei_receive_tmo(fd: c_int, bufp: *mut c_uchar, bufsize: c_int, ms: c_uint) -> c_int;

  pub fn ei_receive_msg(fd: c_int, msg: *mut erlang_msg, x: *mut ei_x_buff) -> c_int;

  pub fn ei_receive_msg_tmo(
    fd: c_int,
    msg: *mut erlang_msg,
    x: *mut ei_x_buff,
    ms: c_uint,
  ) -> c_int;

  /// Receives an Erlang message from the connection setup on the file descriptor `fd`.
  ///
  /// # Returns
  ///
  /// * [`ERL_ERROR`], if the function failed. The error code is put in [`erl_errno`].
  /// * [`ERL_TICK`], if the remote end is probing this node for liveness.
  /// * [`ERL_MSG`], if the message is available.
  ///
  /// Note that the [`erlang_msg`] will only be filled if the function returns [`ERL_MSG`].
  ///
  /// [`ERL_ERROR`]: constant.ERL_ERROR.html
  /// [`ERL_TICK`]: constant.ERL_TICK.html
  /// [`ERL_MSG`]: constant.ERL_MSG.html
  /// [`erl_errno`]: fn.__erl_errno_place.html
  /// [`erlang_msg`]: struct.erlang_msg.html
  ///
  /// # See Also
  ///
  /// [The official entry for this function in the Erlang documentation.](http://erlang.org/doc/man/ei_connect.html#ei_xreceive_msg)
  pub fn ei_xreceive_msg(fd: c_int, message: *mut erlang_msg, x: *mut ei_x_buff) -> c_int;

  pub fn ei_xreceive_msg_tmo(
    fd: c_int,
    message: *mut erlang_msg,
    buffer: *mut ei_x_buff,
    timeout_ms: c_uint,
  ) -> c_int;

  pub fn ei_send(fd: c_int, to: *mut erlang_pid, buf: *mut c_char, len: c_int) -> c_int;

  pub fn ei_send_tmo(
    fd: c_int,
    to: *mut erlang_pid,
    buf: *mut c_char,
    len: c_int,
    ms: c_uint,
  ) -> c_int;

  pub fn ei_reg_send(
    ec: *mut ei_cnode,
    fd: c_int,
    server_name: *mut c_char,
    buf: *mut c_char,
    len: c_int,
  ) -> c_int;

  pub fn ei_reg_send_tmo(
    ec: *mut ei_cnode,
    fd: c_int,
    server_name: *mut c_char,
    buf: *mut c_char,
    len: c_int,
    ms: c_uint,
  ) -> c_int;

  pub fn ei_rpc(
    ec: *mut ei_cnode,
    fd: c_int,
    mod_: *mut c_char,
    fun: *mut c_char,
    inbuf: *const c_char,
    inbuflen: c_int,
    x: *mut ei_x_buff,
  ) -> c_int;

  pub fn ei_rpc_to(
    ec: *mut ei_cnode,
    fd: c_int,
    mod_: *mut c_char,
    fun: *mut c_char,
    buf: *const c_char,
    len: c_int,
  ) -> c_int;

  pub fn ei_rpc_from(
    ec: *mut ei_cnode,
    fd: c_int,
    timeout: c_int,
    msg: *mut erlang_msg,
    x: *mut ei_x_buff,
  ) -> c_int;

  pub fn ei_publish(ec: *mut ei_cnode, port: c_int) -> c_int;

  pub fn ei_publish_tmo(ec: *mut ei_cnode, port: c_int, ms: c_uint) -> c_int;

  pub fn ei_accept(ec: *mut ei_cnode, lfd: c_int, conp: *mut ErlConnect) -> c_int;

  pub fn ei_accept_tmo(ec: *mut ei_cnode, lfd: c_int, conp: *mut ErlConnect, ms: c_uint) -> c_int;

  pub fn ei_unpublish(ec: *mut ei_cnode);

  pub fn ei_unpublish_tmo(alive: *const c_char, ms: c_uint);

  pub fn ei_thisnodename(ec: *const ei_cnode) -> *const c_char;

  pub fn ei_thishostname(ec: *const ei_cnode) -> *const c_char;

  pub fn ei_thisalivename(ec: *const ei_cnode) -> *const c_char;

  pub fn ei_self(ec: *mut ei_cnode) -> *mut erlang_pid;

  pub fn ei_set_compat_rel(rel: c_uint);

  pub fn ei_set_tracelevel(arg1: c_int);

  pub fn ei_get_tracelevel() -> c_int;

  pub fn ei_encode_version(buf: *mut c_char, index: *mut c_int) -> c_int;

  pub fn ei_x_encode_version(x: *mut ei_x_buff) -> c_int;

  pub fn ei_encode_long(buf: *mut c_char, index: *mut c_int, p: c_long) -> c_int;

  pub fn ei_x_encode_long(x: *mut ei_x_buff, n: c_long) -> c_int;

  pub fn ei_encode_ulong(buf: *mut c_char, index: *mut c_int, p: c_ulong) -> c_int;

  pub fn ei_x_encode_ulong(x: *mut ei_x_buff, n: c_ulong) -> c_int;

  pub fn ei_encode_double(buf: *mut c_char, index: *mut c_int, p: f64) -> c_int;

  pub fn ei_x_encode_double(x: *mut ei_x_buff, dbl: f64) -> c_int;

  pub fn ei_encode_boolean(buf: *mut c_char, index: *mut c_int, p: c_int) -> c_int;

  pub fn ei_x_encode_boolean(x: *mut ei_x_buff, p: c_int) -> c_int;

  pub fn ei_encode_char(buf: *mut c_char, index: *mut c_int, p: c_char) -> c_int;

  pub fn ei_x_encode_char(x: *mut ei_x_buff, p: c_char) -> c_int;

  pub fn ei_encode_string(buf: *mut c_char, index: *mut c_int, p: *const c_char) -> c_int;

  pub fn ei_encode_string_len(
    buf: *mut c_char,
    index: *mut c_int,
    p: *const c_char,
    len: c_int,
  ) -> c_int;

  pub fn ei_x_encode_string(x: *mut ei_x_buff, s: *const c_char) -> c_int;

  pub fn ei_x_encode_string_len(x: *mut ei_x_buff, s: *const c_char, len: c_int) -> c_int;

  pub fn ei_encode_atom(buf: *mut c_char, index: *mut c_int, p: *const c_char) -> c_int;

  pub fn ei_encode_atom_as(
    buf: *mut c_char,
    index: *mut c_int,
    p: *const c_char,
    from: erlang_char_encoding,
    to: erlang_char_encoding,
  ) -> c_int;

  pub fn ei_encode_atom_len(
    buf: *mut c_char,
    index: *mut c_int,
    p: *const c_char,
    len: c_int,
  ) -> c_int;

  pub fn ei_encode_atom_len_as(
    buf: *mut c_char,
    index: *mut c_int,
    p: *const c_char,
    len: c_int,
    from: erlang_char_encoding,
    to: erlang_char_encoding,
  ) -> c_int;

  pub fn ei_x_encode_atom(x: *mut ei_x_buff, s: *const c_char) -> c_int;

  pub fn ei_x_encode_atom_as(
    x: *mut ei_x_buff,
    s: *const c_char,
    from: erlang_char_encoding,
    to: erlang_char_encoding,
  ) -> c_int;

  pub fn ei_x_encode_atom_len(x: *mut ei_x_buff, s: *const c_char, len: c_int) -> c_int;

  pub fn ei_x_encode_atom_len_as(
    x: *mut ei_x_buff,
    s: *const c_char,
    len: c_int,
    from: erlang_char_encoding,
    to: erlang_char_encoding,
  ) -> c_int;

  pub fn ei_encode_binary(
    buf: *mut c_char,
    index: *mut c_int,
    p: *const c_void,
    len: c_long,
  ) -> c_int;

  pub fn ei_x_encode_binary(x: *mut ei_x_buff, s: *const c_void, len: c_int) -> c_int;

  pub fn ei_encode_pid(buf: *mut c_char, index: *mut c_int, p: *const erlang_pid) -> c_int;

  pub fn ei_x_encode_pid(x: *mut ei_x_buff, pid: *const erlang_pid) -> c_int;

  pub fn ei_encode_fun(buf: *mut c_char, index: *mut c_int, p: *const erlang_fun) -> c_int;

  pub fn ei_x_encode_fun(x: *mut ei_x_buff, fun: *const erlang_fun) -> c_int;

  pub fn ei_encode_port(buf: *mut c_char, index: *mut c_int, p: *const erlang_port) -> c_int;

  pub fn ei_x_encode_port(x: *mut ei_x_buff, p: *const erlang_port) -> c_int;

  pub fn ei_encode_ref(buf: *mut c_char, index: *mut c_int, p: *const erlang_ref) -> c_int;

  pub fn ei_x_encode_ref(x: *mut ei_x_buff, p: *const erlang_ref) -> c_int;

  pub fn ei_encode_term(buf: *mut c_char, index: *mut c_int, t: *mut c_void) -> c_int;

  pub fn ei_x_encode_term(x: *mut ei_x_buff, t: *mut c_void) -> c_int;

  pub fn ei_encode_trace(buf: *mut c_char, index: *mut c_int, p: *const erlang_trace) -> c_int;

  pub fn ei_x_encode_trace(x: *mut ei_x_buff, p: *const erlang_trace) -> c_int;

  pub fn ei_encode_tuple_header(buf: *mut c_char, index: *mut c_int, arity: c_int) -> c_int;

  pub fn ei_x_encode_tuple_header(x: *mut ei_x_buff, n: c_long) -> c_int;

  pub fn ei_encode_list_header(buf: *mut c_char, index: *mut c_int, arity: c_int) -> c_int;

  pub fn ei_x_encode_list_header(x: *mut ei_x_buff, n: c_long) -> c_int;

  pub fn ei_x_encode_empty_list(x: *mut ei_x_buff) -> c_int;

  pub fn ei_encode_map_header(buf: *mut c_char, index: *mut c_int, arity: c_int) -> c_int;

  pub fn ei_x_encode_map_header(x: *mut ei_x_buff, n: c_long) -> c_int;

  pub fn ei_get_type(
    buf: *const c_char,
    index: *const c_int,
    type_: *mut c_int,
    size: *mut c_int,
  ) -> c_int;

  pub fn ei_decode_version(buf: *const c_char, index: *mut c_int, version: *mut c_int) -> c_int;

  pub fn ei_decode_long(buf: *const c_char, index: *mut c_int, p: *mut c_long) -> c_int;

  pub fn ei_decode_ulong(buf: *const c_char, index: *mut c_int, p: *mut c_ulong) -> c_int;

  pub fn ei_decode_double(buf: *const c_char, index: *mut c_int, p: *mut f64) -> c_int;

  pub fn ei_decode_boolean(buf: *const c_char, index: *mut c_int, p: *mut c_int) -> c_int;

  pub fn ei_decode_char(buf: *const c_char, index: *mut c_int, p: *mut c_char) -> c_int;

  pub fn ei_decode_string(buf: *const c_char, index: *mut c_int, p: *mut c_char) -> c_int;

  pub fn ei_decode_atom(buf: *const c_char, index: *mut c_int, p: *mut c_char) -> c_int;

  pub fn ei_decode_atom_as(
    buf: *const c_char,
    index: *mut c_int,
    p: *mut c_char,
    destlen: c_int,
    want: erlang_char_encoding,
    was: *mut erlang_char_encoding,
    result: *mut erlang_char_encoding,
  ) -> c_int;

  pub fn ei_decode_binary(
    buf: *const c_char,
    index: *mut c_int,
    p: *mut c_void,
    len: *mut c_long,
  ) -> c_int;

  pub fn ei_decode_fun(buf: *const c_char, index: *mut c_int, p: *mut erlang_fun) -> c_int;

  pub fn free_fun(f: *mut erlang_fun);

  pub fn ei_decode_pid(buf: *const c_char, index: *mut c_int, p: *mut erlang_pid) -> c_int;

  pub fn ei_decode_port(buf: *const c_char, index: *mut c_int, p: *mut erlang_port) -> c_int;

  pub fn ei_decode_ref(buf: *const c_char, index: *mut c_int, p: *mut erlang_ref) -> c_int;

  pub fn ei_decode_term(buf: *const c_char, index: *mut c_int, t: *mut c_void) -> c_int;

  pub fn ei_decode_trace(buf: *const c_char, index: *mut c_int, p: *mut erlang_trace) -> c_int;

  pub fn ei_decode_tuple_header(buf: *const c_char, index: *mut c_int, arity: *mut c_int) -> c_int;

  pub fn ei_decode_list_header(buf: *const c_char, index: *mut c_int, arity: *mut c_int) -> c_int;

  pub fn ei_decode_map_header(buf: *const c_char, index: *mut c_int, arity: *mut c_int) -> c_int;

  pub fn ei_print_term(fp: *mut libc::FILE, buf: *const c_char, index: *mut c_int) -> c_int;

  pub fn ei_s_print_term(s: *mut *mut c_char, buf: *const c_char, index: *mut c_int) -> c_int;

  pub fn ei_x_format(x: *mut ei_x_buff, fmt: *const c_char, ...) -> c_int;

  pub fn ei_x_format_wo_ver(x: *mut ei_x_buff, fmt: *const c_char, ...) -> c_int;

  /// Initializes a new [`ei_x_buff`] by filling all fields and preallocating a buffer of size
  /// [`ei_x_extra`].
  ///
  /// [`ei_x_buff`]: type.ei_x_buff.html
  /// [`ei_x_extra`]: static.ei_x_extra.html
  ///
  /// # Returns
  ///
  /// If the buffer was successfuly allocated, this function returns `0`, otherwise it returns `-1`.
  ///
  /// # Safety
  ///
  /// If the fuction fails, the buffer will be initialized, but invalid. It must not be passed to
  /// other functions.
  ///
  /// # See Also
  ///
  /// [The official entry for this function in the Erlang documentation.](http://erlang.org/doc/man/ei.html#ei_x_new)
  pub fn ei_x_new(buffer: *mut ei_x_buff) -> c_int;

  pub fn ei_x_new_with_version(x: *mut ei_x_buff) -> c_int;

  pub fn ei_x_free(x: *mut ei_x_buff) -> c_int;

  pub fn ei_x_append(x: *mut ei_x_buff, x2: *const ei_x_buff) -> c_int;

  pub fn ei_x_append_buf(x: *mut ei_x_buff, buf: *const c_char, len: c_int) -> c_int;

  pub fn ei_skip_term(buf: *const c_char, index: *mut c_int) -> c_int;

  pub fn ei_reg_open(size: c_int) -> *mut ei_reg;

  pub fn ei_reg_resize(oldreg: *mut ei_reg, newsize: c_int) -> c_int;

  pub fn ei_reg_close(reg: *mut ei_reg) -> c_int;

  pub fn ei_reg_setival(reg: *mut ei_reg, key: *const c_char, i: c_long) -> c_int;

  pub fn ei_reg_setfval(reg: *mut ei_reg, key: *const c_char, f: c_double) -> c_int;

  pub fn ei_reg_setsval(reg: *mut ei_reg, key: *const c_char, s: *const c_char) -> c_int;

  pub fn ei_reg_setpval(reg: *mut ei_reg, key: *const c_char, p: *const c_void, size: c_int) -> c_int;

  pub fn ei_reg_setval(reg: *mut ei_reg, key: *const c_char, flags: c_int, ...) -> c_int;

  pub fn ei_reg_getival(reg: *mut ei_reg, key: *const c_char ) -> c_long;

  pub fn ei_reg_getfval(reg: *mut ei_reg, key: *const c_char ) -> c_double;

  pub fn ei_reg_getsval(reg: *mut ei_reg, key: *const c_char ) -> *const c_char;

  pub fn ei_reg_getpval(reg: *mut ei_reg, key: *const c_char, size: *mut c_int) -> *const c_void;

  pub fn ei_reg_getval(reg: *mut ei_reg, key: *const c_char, flags: c_int, ...) -> c_int;

  pub fn ei_reg_markdirty(reg: *mut ei_reg, key: *const c_char) -> c_int;

  pub fn ei_reg_delete(reg: *mut ei_reg, key: *const c_char) -> c_int;

  pub fn ei_reg_stat(reg: *mut ei_reg, key: *const c_char, obuf: *mut ei_reg_stat) -> c_int;

  pub fn ei_reg_tabstat(reg: *mut ei_reg, obuf: *mut ei_reg_stat) -> c_int;

  pub fn ei_reg_dump(fd: c_int, reg: *mut ei_reg, mntab: *const c_char, flags: c_int) -> c_int;

  pub fn ei_reg_restore(fd: c_int, reg: *mut ei_reg, mntab: *const c_char) -> c_int;

  pub fn ei_reg_purge(reg: *mut ei_reg) -> c_int;

  pub fn ei_decode_longlong(buf: *const c_char, index: *mut c_int, p: *mut c_longlong) -> c_int;

  pub fn ei_decode_ulonglong(buf: *const c_char, index: *mut c_int, p: *mut c_ulonglong) -> c_int;

  pub fn ei_encode_longlong(buf: *mut c_char, index: *mut c_int, p: c_longlong) -> c_int;

  pub fn ei_encode_ulonglong(buf: *mut c_char, index: *mut c_int, p: c_ulonglong) -> c_int;

  pub fn ei_x_encode_longlong(x: *mut ei_x_buff, n: c_longlong) -> c_int;

  pub fn ei_x_encode_ulonglong(x: *mut ei_x_buff, n: c_ulonglong) -> c_int;

  pub fn ei_receive_encoded(
    fd: c_int,
    bufp: *mut *mut c_char,
    bufsz: *mut c_int,
    to: *mut erlang_msg,
    msglen: *mut c_int,
  ) -> c_int;

  pub fn ei_receive_encoded_tmo(
    fd: c_int,
    bufp: *mut *mut c_char,
    bufsz: *mut c_int,
    to: *mut erlang_msg,
    msglen: *mut c_int,
    ms: c_uint,
  ) -> c_int;

  pub fn ei_send_encoded(
    fd: c_int,
    to: *const erlang_pid,
    msg: *mut c_char,
    msglen: c_int,
  ) -> c_int;

  pub fn ei_send_encoded_tmo(
    fd: c_int,
    to: *const erlang_pid,
    msg: *mut c_char,
    msglen: c_int,
    ms: c_uint,
  ) -> c_int;

  pub fn ei_send_reg_encoded(
    fd: c_int,
    from: *const erlang_pid,
    to: *const c_char,
    msg: *mut c_char,
    msglen: c_int,
  ) -> c_int;

  pub fn ei_send_reg_encoded_tmo(
    fd: c_int,
    from: *const erlang_pid,
    to: *const c_char,
    msg: *mut c_char,
    msglen: c_int,
    ms: c_uint,
  ) -> c_int;

  pub fn ei_encode_big(buf: *mut c_char, index: *mut c_int, big: *mut erlang_big) -> c_int;

  pub fn ei_x_encode_big(x: *mut ei_x_buff, big: *mut erlang_big) -> c_int;

  pub fn ei_decode_big(buf: *const c_char, index: *mut c_int, p: *mut erlang_big) -> c_int;

  pub fn ei_big_comp(x: *mut erlang_big, y: *mut erlang_big) -> c_int;

  pub fn ei_big_to_double(b: *mut erlang_big, resp: *mut c_double) -> c_int;

  pub fn ei_small_to_big(s: c_int, b: *mut erlang_big) -> c_int;

  pub fn ei_alloc_big(arity: c_uint) -> *mut erlang_big;

  pub fn ei_free_big(b: *mut erlang_big) -> c_void;

}
