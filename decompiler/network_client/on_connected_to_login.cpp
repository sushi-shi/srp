// This function is called when the server send the first response.
// The response needs to be a single byte representing the state of the user account (see login_server_message_types_enum for an example).
//
// TODO: Document who calls this function
// TODO: Find code which reads the first byte from the message
//
// Calls `switch_to_login` which would set up an error message and allow you to try again.
// Calls `switch_to_lobby` on sucessful authentification (without going through ssh though)
//
// It seems like this function is called after doing SSH authentification, since there in no switch case for `valid_user_name_message_type`
// TODO: Find the function doing this and confirm the theory.
void __thiscall survarium::network_client::on_connected_to_login(
        survarium::network_client *this,
        vostok::connection_error_types_enum connection_error,
        vostok::handshaking_error_types_enum handshaking_error,
        vostok::socket_error_types_enum socket_error,
        vostok::login_server_message_types_enum message_type)
{
  __int16 v5; // bx
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v6; // ecx
  void (__cdecl *v7)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  void (__cdecl *v8)(boost::detail::function::function_buffer *, boost::detail::function::function_buffer *, int); // eax
  void (__cdecl *v9)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  void (__cdecl *v10)(boost::detail::function::function_buffer *, boost::detail::function::function_buffer *, int); // eax
  void (__cdecl *v11)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  void (__cdecl *v12)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  void (__cdecl *v13)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  void (__cdecl *v14)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  void (__cdecl *v15)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v16; // ecx
  void (__cdecl *v17)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v18; // ecx
  void (__cdecl *v19)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v20; // ecx
  void (__cdecl *v21)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v22; // ecx
  void (__cdecl *v23)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v24; // ecx
  void (__cdecl *v25)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v26; // ecx
  void (__cdecl *v27)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v28; // ecx
  void (__cdecl *v29)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  void (__cdecl *v30)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> log_callback; // [esp+10h] [ebp-108h] BYREF
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> v33; // [esp+30h] [ebp-E8h] BYREF
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> v34; // [esp+50h] [ebp-C8h] BYREF
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> v35; // [esp+70h] [ebp-A8h] BYREF
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> v36; // [esp+90h] [ebp-88h] BYREF
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> v37; // [esp+B0h] [ebp-68h] BYREF
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> v38; // [esp+D0h] [ebp-48h] BYREF
  int v39; // [esp+F4h] [ebp-24h]
  boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> v40; // [esp+F8h] [ebp-20h] BYREF

  v5 = 0;
  v39 = 0;
  if ( connection_error )
  {
    if ( connection_error == cannot_connect )
    {
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v9 = vostok::core::g_log_callback;
        v33.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v33.functor,
            &v33.functor,
            destroy_functor_tag);
        if ( v9 )
        {
          v33.functor.obj_ptr = v9;
          v33.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v33.vtable = 0;
        }
        LOBYTE(v5) = 1;
        vostok::logging::append(
          &v33,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0xD8u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "game: cannot connect to login server");
      }
      if ( (v5 & 1) != 0 && v33.vtable )
      {
        if ( ((int)v33.vtable & 1) == 0 )
        {
          v10 = *(void (__cdecl **)(boost::detail::function::function_buffer *, boost::detail::function::function_buffer *, int))((int)v33.vtable & 0xFFFFFFFE);
          if ( v10 )
            v10(&v33.functor, &v33.functor, 2);
        }
        v33.vtable = 0;
      }
    }
    else
    {
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v7 = vostok::core::g_log_callback;
        log_callback.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &log_callback.functor,
            &log_callback.functor,
            destroy_functor_tag);
        if ( v7 )
        {
          log_callback.functor.obj_ptr = v7;
          log_callback.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                                       + 1);
        }
        else
        {
          log_callback.vtable = 0;
        }
        LOBYTE(v5) = 2;
        vostok::logging::append(
          &log_callback,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0xDFu,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "game: unexpected socket error type");
      }
      if ( (v5 & 2) != 0 && log_callback.vtable )
      {
        if ( ((int)log_callback.vtable & 1) == 0 )
        {
          v8 = *(void (__cdecl **)(boost::detail::function::function_buffer *, boost::detail::function::function_buffer *, int))((int)log_callback.vtable & 0xFFFFFFFE);
          if ( v8 )
            v8(&log_callback.functor, &log_callback.functor, 2);
        }
        log_callback.vtable = 0;
      }
    }
    goto LABEL_175;
  }
  if ( handshaking_error )
  {
    if ( handshaking_error == cannot_handshake )
    {
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v12 = vostok::core::g_log_callback;
        log_callback.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &log_callback.functor,
            &log_callback.functor,
            destroy_functor_tag);
        if ( v12 )
        {
          log_callback.functor.obj_ptr = v12;
          log_callback.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                                       + 1);
        }
        else
        {
          log_callback.vtable = 0;
        }
        LOBYTE(v5) = 4;
        vostok::logging::append(
          &log_callback,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0xECu,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "game: SSL certificate verification failed");
      }
      if ( (v5 & 4) == 0 )
        goto LABEL_175;
LABEL_174:
      boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v6);
LABEL_175:
      survarium::game::switch_to_login((survarium::game *)v6, (int)this->m_game, login_menu_status_error_connection);
      return;
    }
    if ( handshaking_error != no_handshake )
    {
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v11 = vostok::core::g_log_callback;
        v33.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v33.functor,
            &v33.functor,
            destroy_functor_tag);
        if ( v11 )
        {
          v33.functor.obj_ptr = v11;
          v33.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v33.vtable = 0;
        }
        LOBYTE(v5) = 8;
        vostok::logging::append(
          &v33,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0xF6u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "game: unexpected SSL error");
      }
      if ( (v5 & 8) == 0 )
        goto LABEL_175;
      goto LABEL_174;
    }
  }
  if ( socket_error )
  {
    if ( socket_error == unable_to_write_to_socket )
    {
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v15 = vostok::core::g_log_callback;
        v34.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v34.functor,
            &v34.functor,
            destroy_functor_tag);
        if ( v15 )
        {
          v34.functor.obj_ptr = v15;
          v34.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v34.vtable = 0;
        }
        LOBYTE(v5) = 16;
        vostok::logging::append(
          &v34,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x103u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "game: unable to write to socket");
      }
      if ( (v5 & 0x10) == 0 )
        goto LABEL_175;
    }
    else if ( socket_error == unable_to_read_from_socket )
    {
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v14 = vostok::core::g_log_callback;
        log_callback.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &log_callback.functor,
            &log_callback.functor,
            destroy_functor_tag);
        if ( v14 )
        {
          log_callback.functor.obj_ptr = v14;
          log_callback.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                                       + 1);
        }
        else
        {
          log_callback.vtable = 0;
        }
        LOBYTE(v5) = 32;
        vostok::logging::append(
          &log_callback,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x10Au,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "game: unable to read from socket");
      }
      if ( (v5 & 0x20) == 0 )
        goto LABEL_175;
    }
    else
    {
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v13 = vostok::core::g_log_callback;
        v33.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v33.functor,
            &v33.functor,
            destroy_functor_tag);
        if ( v13 )
        {
          v33.functor.obj_ptr = v13;
          v33.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v33.vtable = 0;
        }
        LOBYTE(v5) = 64;
        vostok::logging::append(
          &v33,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x111u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "game: unexpected socket error type");
      }
      if ( (v5 & 0x40) == 0 )
        goto LABEL_175;
    }
    goto LABEL_174;
  }
  switch ( message_type )
  {
    case servers_connection_info_message_type:
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", info) )
      {
        v17 = vostok::core::g_log_callback;
        v34.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v34.functor,
            &v34.functor,
            destroy_functor_tag);
        if ( v17 )
        {
          v34.functor.obj_ptr = v17;
          v34.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v34.vtable = 0;
        }
        LOBYTE(v5) = 0x80;
        vostok::logging::append(
          &v34,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x11Bu,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          info,
          "on_connected_to_login.");
      }
      if ( (v5 & 0x80u) != 0 )
        boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v16);
      survarium::game::switch_to_lobby((survarium::game *)v16);
      break;
    case invalid_user_name_or_password_message_type:
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v23 = vostok::core::g_log_callback;
        v37.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v37.functor,
            &v37.functor,
            destroy_functor_tag);
        if ( v23 )
        {
          v37.functor.obj_ptr = v23;
          v37.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v37.vtable = 0;
        }
        v5 = 1024;
        vostok::logging::append(
          &v37,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x12Eu,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "sign in: invalid user name or password");
      }
      if ( (v5 & 0x400) != 0 )
        boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v22);
      survarium::game::switch_to_login(
        (survarium::game *)v22,
        (int)this->m_game,
        login_menu_status_invalid_user_or_password);
      break;
    case sign_in_attempt_interval_violated_message_type:
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v29 = vostok::core::g_log_callback;
        v38.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v38.functor,
            &v38.functor,
            destroy_functor_tag);
        if ( v29 )
        {
          v38.functor.obj_ptr = v29;
          v38.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v38.vtable = 0;
        }
        v5 = 0x2000;
        vostok::logging::append(
          &v38,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x140u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "sign in: attempt interval is violated");
      }
      if ( (v5 & 0x2000) != 0 )
        boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v28);
      survarium::game::switch_to_login(
        (survarium::game *)v28,
        (int)this->m_game,
        login_menu_status_sign_in_attempt_interval_violated);
      break;
    case user_banned_message_type:
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v25 = vostok::core::g_log_callback;
        v35.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v35.functor,
            &v35.functor,
            destroy_functor_tag);
        if ( v25 )
        {
          v35.functor.obj_ptr = v25;
          v35.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v35.vtable = 0;
        }
        v5 = 2048;
        vostok::logging::append(
          &v35,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x134u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "sign in: user banned");
      }
      if ( (v5 & 0x800) != 0 )
        boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v24);
      survarium::game::switch_to_login((survarium::game *)v24, (int)this->m_game, login_menu_status_user_banned);
      break;
    case user_restricted_by_access_level_message_type:
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v27 = vostok::core::g_log_callback;
        v36.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v36.functor,
            &v36.functor,
            destroy_functor_tag);
        if ( v27 )
        {
          v36.functor.obj_ptr = v27;
          v36.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v36.vtable = 0;
        }
        v5 = 4096;
        vostok::logging::append(
          &v36,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x13Au,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "sign in: user access level restriction");
      }
      if ( (v5 & 0x1000) != 0 )
        boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v26);
      survarium::game::switch_to_login(
        (survarium::game *)v26,
        (int)this->m_game,
        login_menu_status_access_level_restriction);
      break;
    case sign_in_user_already_signed_in:
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v19 = vostok::core::g_log_callback;
        v33.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v33.functor,
            &v33.functor,
            destroy_functor_tag);
        if ( v19 )
        {
          v33.functor.obj_ptr = v19;
          v33.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v33.vtable = 0;
        }
        v5 = 256;
        vostok::logging::append(
          &v33,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x122u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "sign in: user already signed in");
      }
      if ( (v5 & 0x100) != 0 )
        boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v18);
      survarium::game::switch_to_login(
        (survarium::game *)v18,
        (int)this->m_game,
        login_menu_status_sign_in_already_online);
      break;
    case sign_in_invalid_version:
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v21 = vostok::core::g_log_callback;
        log_callback.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &log_callback.functor,
            &log_callback.functor,
            destroy_functor_tag);
        if ( v21 )
        {
          log_callback.functor.obj_ptr = v21;
          log_callback.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                                       + 1);
        }
        else
        {
          log_callback.vtable = 0;
        }
        v5 = 512;
        vostok::logging::append(
          &log_callback,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x128u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "sign in: invalid version");
      }
      if ( (v5 & 0x200) != 0 )
        boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v20);
      survarium::game::switch_to_login((survarium::game *)v20, (int)this->m_game, login_menu_status_invalid_version);
      break;
    default:
      if ( !vostok::core::g_log_filter_tree
        || vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", error) )
      {
        v30 = vostok::core::g_log_callback;
        v40.vtable = 0;
        if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
          `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
            &v40.functor,
            &v40.functor,
            destroy_functor_tag);
        if ( v30 )
        {
          v40.functor.obj_ptr = v30;
          v40.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                              + 1);
        }
        else
        {
          v40.vtable = 0;
        }
        v5 = 0x4000;
        vostok::logging::append(
          &v40,
          (void *const)vostok::core::g_log_flags,
          &vostok::core::g_log_format,
          ".\\network_client.cpp",
          0x147u,
          "void __thiscall survarium::network_client::on_connected_to_login(const enum vostok::connection_error_types_enu"
          "m,const enum vostok::handshaking_error_types_enum,const enum vostok::socket_error_types_enum,const enum vostok"
          "::login_server_message_types_enum)",
          "game:",
          error,
          "sign in: unexpected message type");
      }
      if ( (v5 & 0x4000) != 0 )
        goto LABEL_174;
      goto LABEL_175;
  }
}
