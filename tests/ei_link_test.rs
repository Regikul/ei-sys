extern crate ei_sys;

#[allow(unused_assignments, unused_variables)]
#[test]
fn ei_decode_ok() {
    let mut x = 0 as *const ();
    x = ei_sys::ei_decode_atom          as *const ();
    x = ei_sys::ei_decode_atom_as       as *const ();
    x = ei_sys::ei_decode_binary        as *const ();
    x = ei_sys::ei_decode_boolean       as *const ();
    x = ei_sys::ei_decode_char          as *const ();
    x = ei_sys::ei_decode_double        as *const ();
    x = ei_sys::ei_decode_ei_term       as *const ();
    x = ei_sys::ei_decode_fun           as *const ();
    x = ei_sys::ei_decode_list_header   as *const ();
    x = ei_sys::ei_decode_long          as *const ();
    x = ei_sys::ei_decode_longlong      as *const ();
    x = ei_sys::ei_decode_map_header    as *const ();
    x = ei_sys::ei_decode_pid           as *const ();
    x = ei_sys::ei_decode_port          as *const ();
    x = ei_sys::ei_decode_ref           as *const ();
    x = ei_sys::ei_decode_string        as *const ();
    // x = ei_sys::ei_decode_term       as *const (); // b/c of undefined reference
    x = ei_sys::ei_decode_trace         as *const ();
    x = ei_sys::ei_decode_tuple_header  as *const ();
    x = ei_sys::ei_decode_ulong         as *const ();
    x = ei_sys::ei_decode_ulonglong     as *const ();
    x = ei_sys::ei_decode_version       as *const ();
    x = ei_sys::ei_decode_ei_term       as *const ();
}

#[allow(unused_assignments, unused_variables)]
#[test]
fn ei_encode_ok() {
    let mut x = 0 as *const ();
    x = ei_sys::ei_encode_atom          as *const ();
    x = ei_sys::ei_encode_atom_as       as *const ();
    x = ei_sys::ei_encode_atom_len      as *const ();
    x = ei_sys::ei_encode_atom_len_as   as *const ();
    x = ei_sys::ei_encode_binary        as *const ();
    x = ei_sys::ei_encode_boolean       as *const ();
    x = ei_sys::ei_encode_char          as *const ();
    x = ei_sys::ei_encode_double        as *const ();
    x = ei_sys::ei_encode_empty_list    as *const ();
    x = ei_sys::ei_encode_fun           as *const ();
    x = ei_sys::ei_encode_list_header   as *const ();
    x = ei_sys::ei_encode_long          as *const ();
    x = ei_sys::ei_encode_longlong      as *const ();
    x = ei_sys::ei_encode_map_header    as *const ();
    x = ei_sys::ei_encode_pid           as *const ();
    x = ei_sys::ei_encode_port          as *const ();
    x = ei_sys::ei_encode_ref           as *const ();
    x = ei_sys::ei_encode_string        as *const ();
    x = ei_sys::ei_encode_string_len    as *const ();
    // x = ei_sys::ei_encode_term       as *const (); // b/c of undefined reference
    x = ei_sys::ei_encode_trace         as *const ();
    x = ei_sys::ei_encode_tuple_header  as *const ();
    x = ei_sys::ei_encode_ulong         as *const ();
    x = ei_sys::ei_encode_ulonglong     as *const ();
    x = ei_sys::ei_encode_version       as *const ();

    x = ei_sys::ei_x_encode_atom          as *const ();
    x = ei_sys::ei_x_encode_atom_as       as *const ();
    x = ei_sys::ei_x_encode_atom_len      as *const ();
    x = ei_sys::ei_x_encode_atom_len_as   as *const ();
    x = ei_sys::ei_x_encode_binary        as *const ();
    x = ei_sys::ei_x_encode_boolean       as *const ();
    x = ei_sys::ei_x_encode_char          as *const ();
    x = ei_sys::ei_x_encode_double        as *const ();
    x = ei_sys::ei_x_encode_empty_list    as *const ();
    x = ei_sys::ei_x_encode_fun           as *const ();
    x = ei_sys::ei_x_encode_list_header   as *const ();
    x = ei_sys::ei_x_encode_long          as *const ();
    x = ei_sys::ei_x_encode_longlong      as *const ();
    x = ei_sys::ei_x_encode_map_header    as *const ();
    x = ei_sys::ei_x_encode_pid           as *const ();
    x = ei_sys::ei_x_encode_port          as *const ();
    x = ei_sys::ei_x_encode_ref           as *const ();
    x = ei_sys::ei_x_encode_string        as *const ();
    x = ei_sys::ei_x_encode_string_len    as *const ();
    // x = ei_sys::ei_x_encode_term       as *const (); // b/c of undefined reference
    x = ei_sys::ei_x_encode_trace         as *const ();
    x = ei_sys::ei_x_encode_tuple_header  as *const ();
    x = ei_sys::ei_x_encode_ulong         as *const ();
    x = ei_sys::ei_x_encode_ulonglong     as *const ();
    x = ei_sys::ei_x_encode_version       as *const ();
}

#[allow(unused_assignments, unused_variables)]
#[test]
fn ei_other_ok() {
    let mut x = 0 as *const ();
    x = ei_sys::free_fun                as *const ();
    x = ei_sys::ei_get_type             as *const ();
    x = ei_sys::ei_print_term           as *const ();
    x = ei_sys::ei_s_print_term         as *const ();
    x = ei_sys::ei_set_compat_rel       as *const ();
    x = ei_sys::ei_skip_term            as *const ();
    x = ei_sys::ei_x_append             as *const ();
    x = ei_sys::ei_x_append_buf         as *const ();
    x = ei_sys::ei_x_format             as *const ();
    x = ei_sys::ei_x_format_wo_ver      as *const ();
    x = ei_sys::ei_x_free               as *const ();
    x = ei_sys::ei_x_new                as *const ();
    x = ei_sys::ei_x_new_with_version   as *const ();
}

#[allow(unused_assignments, unused_variables)]
#[test]
fn ei_connect_ok() {
    let mut x = 0 as *const ();
    x = ei_sys::ei_accept               as *const ();
    x = ei_sys::ei_accept_tmo           as *const ();
    x = ei_sys::ei_connect              as *const ();
    x = ei_sys::ei_xconnect             as *const ();
    x = ei_sys::ei_xconnect_tmo         as *const ();
    x = ei_sys::ei_connect_init         as *const ();
    x = ei_sys::ei_connect_xinit        as *const ();
    x = ei_sys::ei_get_tracelevel       as *const ();
    x = ei_sys::ei_publish              as *const ();
    x = ei_sys::ei_publish_tmo          as *const ();
    x = ei_sys::ei_receive              as *const ();
    x = ei_sys::ei_receive_encoded      as *const ();
    x = ei_sys::ei_receive_encoded_tmo  as *const ();
    x = ei_sys::ei_receive_msg          as *const ();
    x = ei_sys::ei_receive_msg_tmo      as *const ();
    x = ei_sys::ei_receive_tmo          as *const ();
    x = ei_sys::ei_reg_send             as *const ();
    x = ei_sys::ei_reg_send_tmo         as *const ();
    x = ei_sys::ei_rpc                  as *const ();
    x = ei_sys::ei_rpc_from             as *const ();
    x = ei_sys::ei_rpc_to               as *const ();
    x = ei_sys::ei_self                 as *const ();
    x = ei_sys::ei_send                 as *const ();
    x = ei_sys::ei_send_encoded         as *const ();
    x = ei_sys::ei_send_encoded_tmo     as *const ();
    x = ei_sys::ei_send_reg_encoded     as *const ();
    x = ei_sys::ei_send_reg_encoded_tmo as *const ();
    x = ei_sys::ei_send_tmo             as *const ();
    x = ei_sys::ei_set_tracelevel       as *const ();
    x = ei_sys::ei_thisalivename        as *const ();
    x = ei_sys::ei_thishostname         as *const ();
    x = ei_sys::ei_thisnodename         as *const ();
    x = ei_sys::ei_unpublish            as *const ();
    x = ei_sys::ei_unpublish_tmo        as *const ();
    x = ei_sys::ei_xreceive_msg         as *const ();
    x = ei_sys::ei_xreceive_msg_tmo     as *const ();
}
