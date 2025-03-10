 void __thiscall vostok::network::login_client_impl::on_sign_in_answer_received(
        vostok::network::login_client_impl *this,
        boost::function4<void,enum vostok::connection_error_types_enum,enum vostok::handshaking_error_types_enum,enum vostok::socket_error_types_enum,enum vostok::lobby_server_message_types_enum> *callback,
        boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> *error_code,
        unsigned int bytes_transferred)
{
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> *v4; // ecx
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> *v5; // ecx
  bool v6; // al
  bool has_passed_filters; // al
  const stlp_std::allocator<char> *v8; // eax
  boost::_bi::bind_t<void,boost::_mfi::mf1<void,vostok::network::login_client_impl,unsigned int>,boost::_bi::list2<boost::_bi::value<vostok::network::login_client_impl *>,boost::_bi::value<enum vostok::network::login_client_impl::<unnamed_tag> > > > v9; // [esp-Ch] [ebp-61Ch]
  void (__cdecl *v10)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // [esp+0h] [ebp-610h]
  int v11; // [esp+4h] [ebp-60Ch]
  boost::asio::ip::udp *v12; // [esp+8h] [ebp-608h]
  boost::asio::ip::basic_endpoint<boost::asio::ip::tcp> *v14; // [esp+2E0h] [ebp-330h]
  boost::asio::ip::basic_endpoint<boost::asio::ip::tcp> *v15; // [esp+30Ch] [ebp-304h]
  char v16; // [esp+454h] [ebp-1BCh]
  boost::_bi::bind_t<void,boost::_mfi::mf1<void,vostok::sound::sound_voice,void *>,boost::_bi::list2<boost::_bi::value<vostok::sound::sound_voice *>,boost::_bi::value<void *> > > v17; // [esp+458h] [ebp-1B8h] BYREF
  boost::asio::ip::basic_endpoint<boost::asio::ip::udp> peer_endpoint; // [esp+464h] [ebp-1ACh] BYREF
  int v19; // [esp+480h] [ebp-190h] BYREF
  int v20; // [esp+484h] [ebp-18Ch] BYREF
  boost::asio::ip::address v21; // [esp+488h] [ebp-188h] BYREF
  boost::asio::ip::basic_endpoint<boost::asio::ip::tcp> result; // [esp+4A4h] [ebp-16Ch] BYREF
  boost::asio::ip::udp *protocol; // [esp+4C0h] [ebp-150h]
  stlp_std::basic_string<char,stlp_std::char_traits<char>,stlp_std::allocator<char> > v24; // [esp+4C4h] [ebp-14Ch] BYREF
  boost::asio::ip::address v25; // [esp+4DCh] [ebp-134h] BYREF
  boost::asio::ip::basic_endpoint<boost::asio::ip::tcp> v26; // [esp+4F8h] [ebp-118h] BYREF
  survarium::game_camera v27; // [esp+517h] [ebp-F9h] BYREF
  stlp_std::basic_string<char,stlp_std::char_traits<char>,stlp_std::allocator<char> > v28; // [esp+574h] [ebp-9Ch] BYREF
  char v29; // [esp+58Fh] [ebp-81h]
  boost::asio::ip::basic_resolver_query<boost::asio::ip::udp> query; // [esp+590h] [ebp-80h] BYREF
  boost::asio::ip::basic_resolver_iterator<boost::asio::ip::udp> iterator; // [esp+5E4h] [ebp-2Ch] BYREF
  unsigned __int8 length1; // [esp+5F3h] [ebp-1Dh]
  boost::asio::ip::basic_resolver<boost::asio::ip::udp,boost::asio::ip::resolver_service<boost::asio::ip::udp> > resolver; // [esp+5F4h] [ebp-1Ch] BYREF
  unsigned __int8 length2; // [esp+603h] [ebp-Dh]
  char port[8]; // [esp+604h] [ebp-Ch] BYREF
  unsigned __int8 *buffer; // [esp+60Ch] [ebp-4h]

  v16 = 0;
  v29 = 0;
  survarium::weapon_user_dead_state::finalize((survarium::game_camera *)this);
  v4 = error_code;
  if ( (error_code->vtable != 0
      ? (unsigned int)boost::intrusive::detail::destructor_impl<boost::intrusive::detail::generic_hook<boost::intrusive::get_set_node_algo<void *,0>,boost::intrusive::member_tag,1,0>>
      : 0) == 0
    && bytes_transferred )
  {
    if ( !vostok::core::g_log_filter_tree
      || (has_passed_filters = vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "network:", info),
          (v4 = (boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> *)has_passed_filters) != 0) )
    {
      boost::function<void __cdecl (void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>::function<void __cdecl (void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>(
        v4,
        v10,
        v11);
      v16 = 4;
      vostok::logging::append(
        (const boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> *)((char *)&v27.m_inverted_view_matrix.lines[1].elements[1] + 1),
        (void *const)vostok::core::g_log_flags,
        &vostok::core::g_log_format,
        ".\\login_client_impl_sign_in.cpp",
        0x21u,
        "void __thiscall vostok::network::login_client_impl::on_sign_in_answer_received(const class boost::function<void "
        "__cdecl(enum vostok::connection_error_types_enum,enum vostok::handshaking_error_types_enum,enum vostok::socket_e"
        "rror_types_enum,enum vostok::login_server_message_types_enum)> &,const class boost::system::error_code &,const unsigned int)",
        "network:",
        info,
        "[LOGIN] answer has been received!\r\n");
    }
    if ( (v16 & 4) != 0 )
      boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>((boost::function<void __cdecl(unsigned int,float,float,char const *)> *)(v16 & 4));
    buffer = this->m_data;
    if ( this->m_data[0] == 8 )
    {
      length1 = *++buffer;
      ++buffer;
      this->m_server_browser_address[0] = 0;
      this->m_server_browser_initial_query[0] = 0;
      memcpy((unsigned __int8 *)this->m_server_browser_address, buffer, length1);
      buffer += length1;
      this->m_server_browser_address[length1] = 0;
      length2 = *buffer++;
      memcpy((unsigned __int8 *)this->m_server_browser_initial_query, buffer, length2);
      buffer += length2;
      this->m_server_browser_initial_query[length2] = 0;
      this->m_session_id = *(_DWORD *)buffer;
      buffer += 4;
      _itoa_s(25100, port, 6u, 10);
      boost::asio::basic_io_object<boost::asio::ip::resolver_service<boost::asio::ip::udp>>::basic_io_object<boost::asio::ip::resolver_service<boost::asio::ip::udp>>(
        &resolver,
        this->m_io_service);
      v8 = (const stlp_std::allocator<char> *)survarium::weapon_core::cast_weapon_core((survarium::game_options *)&v27);
      string_0(
        (stlp_std::basic_string<char,stlp_std::char_traits<char>,stlp_std::allocator<char> > *)((char *)&v27.__vftable
                                                                                              + 1),
        port,
        v8);
      v15 = boost::asio::basic_socket<boost::asio::ip::tcp,boost::asio::stream_socket_service<boost::asio::ip::tcp>>::remote_endpoint(
              &this->m_socket,
              &result);
      boost::asio::ip::detail::endpoint::address(&v15->impl_, &v21);
      if ( v21.type_ )
      {
        v19 = 23;
        v12 = (boost::asio::ip::udp *)&v19;
      }
      else
      {
        v20 = 2;
        v12 = (boost::asio::ip::udp *)&v20;
      }
      protocol = v12;
      v14 = boost::asio::basic_socket<boost::asio::ip::tcp,boost::asio::stream_socket_service<boost::asio::ip::tcp>>::remote_endpoint(
              &this->m_socket,
              &v26);
      boost::asio::ip::detail::endpoint::address(&v14->impl_, &v25);
      if ( v25.type_ == ipv6 )
        boost::asio::ip::address_v6::to_string(&v25.ipv6_address_, &v24);
      else
        boost::asio::ip::address_v4::to_string(&v25.ipv4_address_, &v24);
      boost::asio::ip::basic_resolver_query<boost::asio::ip::udp>::basic_resolver_query<boost::asio::ip::udp>(
        &query,
        protocol,
        &v24,
        (const stlp_std::basic_string<char,stlp_std::char_traits<char>,stlp_std::allocator<char> > *)((char *)&v27.__vftable + 1),
        address_configured);
      stlp_std::priv::_String_base<char,stlp_std::allocator<char>>::_M_deallocate_block(&v24);
      stlp_std::priv::_String_base<char,stlp_std::allocator<char>>::_M_deallocate_block((stlp_std::basic_string<char,stlp_std::char_traits<char>,stlp_std::allocator<char> > *)((char *)&v27.__vftable + 1));
      survarium::weapon_user_dead_state::finalize(&v27);
      boost::asio::ip::basic_resolver<boost::asio::ip::udp,boost::asio::ip::resolver_service<boost::asio::ip::udp>>::resolve(
        &resolver,
        &iterator,
        &query);
      qmemcpy(&peer_endpoint, &iterator.values_.px->_M_impl._M_start[iterator.index_], sizeof(peer_endpoint));
      boost::asio::basic_socket<boost::asio::ip::udp,boost::asio::datagram_socket_service<boost::asio::ip::udp>>::connect(
        &this->m_ping_socket,
        &peer_endpoint);
      this->m_client_state = 3;
      vostok::network::login_client_impl::close_connection(this, 0);
      if ( !this->m_in_destructor )
        boost::function4<void,enum vostok::connection_error_types_enum,enum vostok::handshaking_error_types_enum,enum vostok::socket_error_types_enum,enum vostok::login_server_message_types_enum>::operator()(
          callback,
          0,
          successfully_handshaked,
          host_cannot_be_resolved,
          (vostok::lobby_server_message_types_enum)8);
      v9 = *(boost::_bi::bind_t<void,boost::_mfi::mf1<void,vostok::network::login_client_impl,unsigned int>,boost::_bi::list2<boost::_bi::value<vostok::network::login_client_impl *>,boost::_bi::value<enum vostok::network::login_client_impl::<unnamed_tag> > > > *)boost::bind<void,vostok::network::match_client,vostok::network_core::udp_network_flow_emulator_options const *,vostok::network::match_client *,vostok::network_core::udp_network_flow_emulator_options const *>(&v17, (void (__thiscall *)(vostok::sound::sound_voice *, void *))vostok::network::login_client_impl::ping, (vostok::sound::sound_voice *)this, (void *)0xA);
      boost::asio::detail::deadline_timer_service<boost::asio::time_traits<boost::posix_time::ptime>>::async_wait<boost::_bi::bind_t<void,boost::_mfi::mf1<void,vostok::network::login_client_impl,unsigned int>,boost::_bi::list2<boost::_bi::value<vostok::network::login_client_impl *>,boost::_bi::value<enum vostok::network::login_client_impl::_unnamed_tag_>>>>(
        &this->m_ping_timer.service->service_impl_,
        &this->m_ping_timer.implementation,
        v9);
      if ( iterator.values_.pn.pi_ )
        boost::detail::sp_counted_base::release(iterator.values_.pn.pi_);
      stlp_std::priv::_String_base<char,stlp_std::allocator<char>>::_M_deallocate_block(&query.service_name_);
      stlp_std::priv::_String_base<char,stlp_std::allocator<char>>::_M_deallocate_block(&query.host_name_);
      boost::asio::basic_io_object<boost::asio::ip::resolver_service<boost::asio::ip::udp>>::~basic_io_object<boost::asio::ip::resolver_service<boost::asio::ip::udp>>(&resolver);
    }
    else
    {
      this->m_client_state = signed_in;
      if ( !this->m_in_destructor )
        boost::function4<void,enum vostok::connection_error_types_enum,enum vostok::handshaking_error_types_enum,enum vostok::socket_error_types_enum,enum vostok::login_server_message_types_enum>::operator()(
          callback,
          0,
          successfully_handshaked,
          host_cannot_be_resolved,
          (vostok::lobby_server_message_types_enum)*buffer);
    }
  }
  else
  {
    this->m_client_state = signed_in;
    vostok::network::login_client_impl::close_connection(this, 0);
    v5 = error_code;
    if ( (error_code->vtable != 0
        ? (unsigned int)boost::intrusive::detail::destructor_impl<boost::intrusive::detail::generic_hook<boost::intrusive::get_set_node_algo<void *,0>,boost::intrusive::member_tag,1,0>>
        : 0) != 0 )
    {
      if ( !vostok::core::g_log_filter_tree
        || (v6 = vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "network:", error),
            (v5 = (boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> *)v6) != 0) )
      {
        boost::function<void __cdecl (void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>::function<void __cdecl (void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>(
          v5,
          v10,
          v11);
        v16 = 3;
        (*((void (__thiscall **)(boost::detail::function::vtable_base *, stlp_std::basic_string<char,stlp_std::char_traits<char>,stlp_std::allocator<char> > *, boost::detail::function::vtable_base *))(&error_code->vtable)[1]->manager
         + 2))(
          (&error_code->vtable)[1],
          &v28,
          error_code->vtable);
        vostok::logging::append(
          (const boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> *)((char *)&v27.m_inverted_view_matrix.lines[3].elements[1] + 1),
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\login_client_impl_sign_in.cpp",
          0x1Au,
          "void __thiscall vostok::network::login_client_impl::on_sign_in_answer_received(const class boost::function<voi"
          "d __cdecl(enum vostok::connection_error_types_enum,enum vostok::handshaking_error_types_enum,enum vostok::sock"
          "et_error_types_enum,enum vostok::login_server_message_types_enum)> &,const class boost::system::error_code &,c"
          "onst unsigned int)",
          "network:",
          error,
          "[LOGIN] error during reading sign in answer: %s\r\n",
          v28._M_start_of_storage._M_data);
      }
      if ( (v16 & 2) != 0 )
      {
        v16 &= ~2u;
        stlp_std::priv::_String_base<char,stlp_std::allocator<char>>::_M_deallocate_block(&v28);
      }
      if ( (v16 & 1) != 0 )
        boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>((boost::function<void __cdecl(unsigned int,float,float,char const *)> *)v5);
    }
    if ( !this->m_in_destructor )
      boost::function4<void,enum vostok::connection_error_types_enum,enum vostok::handshaking_error_types_enum,enum vostok::socket_error_types_enum,enum vostok::login_server_message_types_enum>::operator()(
        callback,
        0,
        successfully_handshaked,
        host_cannot_be_resolved,
        (vostok::lobby_server_message_types_enum)12);
  }
}
