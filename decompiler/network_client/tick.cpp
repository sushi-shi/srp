void __thiscall survarium::network_client::tick(
        survarium::network_client *this,
        unsigned int current_time_in_ms,
        bool is_game_paused)
{
  unsigned int v3; // ebx
  bool v5; // zf
  survarium::network_client_vtbl *v6; // eax
  vostok::network::login_client *(__thiscall *login_client)(struct survarium::network_client *); // eax
  vostok::network::login_client *v8; // eax
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v9; // ecx
  survarium::game *v10; // ecx
  survarium::lobby_client *v11; // ebx
  survarium::network_client *v12; // ecx
  boost::function<void __cdecl(unsigned int,float,float,char const *)> *v13; // ecx
  void (__cdecl *v14)(void *, const char *, unsigned int, const char *, const char *, vostok::logging::verbosity, const char *, unsigned int, vostok::logging::callback_flag); // esi
  char v15; // bl
  survarium::match_client *v16; // eax
  survarium::network_client *v17; // ecx
  survarium::match_client *v18; // eax
  survarium::match_client *(__thiscall *match_client)(struct survarium::network_client *); // edx
  int v20; // eax
  survarium::match_client *v21; // eax
  survarium::network_client *v22; // ecx
  survarium::network_client *v23; // ecx
  survarium::match_client *v24; // eax
  survarium::player *v25; // eax
  survarium::network_client *v26; // ecx
  survarium::player *m_object; // eax
  vostok::resources::unmanaged_resource *v28; // eax
  survarium::player *v29; // edi
  vostok::resources::resource_ptr<survarium::player,vostok::resources::unmanaged_intrusive_base> player; // [esp+10h] [ebp-28h] BYREF
  survarium::messaging_client *id; // [esp+14h] [ebp-24h]
  boost::function<void __cdecl(enum vostok::connection_error_types_enum,enum vostok::handshaking_error_types_enum,enum vostok::socket_error_types_enum,enum vostok::login_server_message_types_enum)> callback; // [esp+18h] [ebp-20h] BYREF

  v3 = current_time_in_ms;
  id = 0;
  this->m_last_tick_time_in_ms = current_time_in_ms;
  survarium::network_client::draw_stats(this, (unsigned int)this);
  if ( this->login_client(this)->m_client_state == signed_in )
  {
    v5 = !this->lobby_client(this)->m_connection_info.need_resolve;
    v6 = this->__vftable;
    if ( v5 )
    {
      if ( v6->messaging_client(this)->m_connection_info.need_resolve
        && current_time_in_ms - messaging_resolve_time > 0xBB8 )
      {
        id = this->messaging_client(this);
        id->m_connection_info.need_resolve = survarium::network_client::http_query_server_connection_info(v17, this, 4u) == 0;
        messaging_resolve_time = current_time_in_ms;
      }
    }
    else if ( v6->lobby_client(this)->m_connection_info.connection_error_count <= 3 )
    {
      if ( current_time_in_ms - lobby_resolve_time > 0x1388 )
      {
        v11 = this->lobby_client(this);
        v11->m_connection_info.need_resolve = survarium::network_client::http_query_server_connection_info(
                                                v12,
                                                this,
                                                2u) == 0;
        lobby_resolve_time = current_time_in_ms;
        if ( vostok::core::g_log_filter_tree
          && !vostok::logging::has_passed_filters(vostok::core::g_log_filter_tree, "game:", info) )
        {
          v15 = (char)id;
        }
        else
        {
          v14 = vostok::core::g_log_callback;
          callback.vtable = 0;
          if ( `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager )
            `boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager(
              &callback.functor,
              &callback.functor,
              destroy_functor_tag);
          if ( v14 )
          {
            callback.functor.obj_ptr = v14;
            callback.vtable = (boost::detail::function::vtable_base *)((char *)&`boost::function9<void,void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag>::assign_to<void (__cdecl *)(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)>'::`2'::stored_vtable.base.manager
                                                                     + 1);
          }
          else
          {
            callback.vtable = 0;
          }
          v15 = 1;
          vostok::logging::append(
            (const boost::function<void __cdecl(void *,char const *,unsigned int,char const *,char const *,enum vostok::logging::verbosity,char const *,unsigned int,enum vostok::logging::callback_flag)> *)&callback,
            (void *const)vostok::core::g_log_flags,
            &vostok::core::g_log_format,
            ".\\network_client_processing.cpp",
            0x268u,
            "void __thiscall survarium::network_client::tick(unsigned int,const bool)",
            "game:",
            info,
            "LOBBY: try reconnect");
        }
        if ( (v15 & 1) != 0 )
          boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v13);
        v3 = current_time_in_ms;
      }
    }
    else
    {
      this->lobby_client(this)->m_connection_info.connection_error_count = 0;
      login_client = this->login_client;
      callback.vtable = 0;
      v8 = login_client(this);
      vostok::network::login_client::sign_out(v8, &callback);
      boost::function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>::~function<enum vostok::animation::callback_return_type_enum __cdecl (vostok::animation::animation_callback_params &)>(v9);
      survarium::game::switch_to_login(v10, (int)this->m_game, login_menu_status_error_connection);
    }
  }
  v16 = this->match_client(this);
  if ( vostok::network::match_client::is_disconnected(&v16->m_client) )
  {
    this->m_player_inputs.m_end = this->m_player_inputs.m_begin;
  }
  else
  {
    v18 = this->match_client(this);
    if ( vostok::network::match_client::is_connected(&v18->m_client) )
    {
      v21 = this->match_client(this);
      if ( vostok::network::match_client::last_receive_time_in_ms(&v21->m_client) + 3000 <= v3 )
      {
        this->m_player_inputs.m_end = this->m_player_inputs.m_begin;
        survarium::game_world_ui::set_broken_connection_message(
          &this->m_game->m_game_world.game_ui,
          (const char *)&this->m_game->m_game_world.game_ui);
      }
      if ( this->m_last_player_input_send_time_in_ms + 33 <= v3 )
      {
        this->m_last_player_input_send_time_in_ms = 33 * (v3 / 0x21);
        survarium::network_client::send_player_inputs(v22);
      }
      if ( this->m_is_time_synchronized_first_time )
      {
        v23 = (survarium::network_client *)(v3 - this->m_last_sync_request_time);
        if ( (unsigned int)v23 > 0xFA0 )
          survarium::network_client::send_sync_request(v23);
      }
      if ( this->m_match_client.m_are_there_any_packets_to_send
        || this->m_match_client.m_last_send_queed_packets_time_in_ms + 33 <= v3 )
      {
        v24 = this->match_client(this);
        v24->m_last_send_queed_packets_time_in_ms = v3;
        v24->m_are_there_any_packets_to_send = 0;
        vostok::network::match_client::send_queued_packets(&v24->m_client, v3);
      }
      if ( is_game_paused
        && (v25 = this->m_current_player.m_object) != 0
        && vostok::intrusive_ptr<vostok::animation::mixing::binary_tree_weight_node,vostok::animation::mixing::binary_tree_base_node,vostok::threading::single_threading_policy>::c_ptr
        && v25->m_has_been_inserted )
      {
        survarium::player::update_camera((survarium::player *)vostok::intrusive_ptr<vostok::animation::mixing::binary_tree_weight_node,vostok::animation::mixing::binary_tree_base_node,vostok::threading::single_threading_policy>::c_ptr);
      }
      else
      {
        LOBYTE(id) = 0;
        do
        {
          this->get_player(this, &player, (const unsigned __int8)id);
          m_object = player.m_object;
          if ( player.m_object )
          {
            if ( player.m_object->m_has_been_inserted )
            {
              if ( !this->m_is_player_ticked && player.m_object->is_local )
              {
                this->m_is_player_ticked = 1;
                survarium::network_client::send_sync_request(v26);
                m_object = player.m_object;
              }
              if ( this->m_is_time_synchronized_first_time || !m_object->is_local )
                survarium::player::tick((survarium::player *)v26, (const unsigned int)m_object);
              vostok::resources::resource_ptr<survarium::player,vostok::resources::unmanaged_intrusive_base>::~resource_ptr<survarium::player,vostok::resources::unmanaged_intrusive_base>(&player);
            }
            else
            {
              v26 = (survarium::network_client *)_InterlockedExchangeAdd(
                                                   &player.m_object->m_reference_count,
                                                   0xFFFFFFFF);
              if ( !v26 )
              {
                if ( player.m_object )
                  v28 = &player.m_object->vostok::resources::unmanaged_resource;
                else
                  v28 = 0;
                vostok::resources::unmanaged_intrusive_base::destroy(
                  &player.m_object->vostok::resources::unmanaged_intrusive_base,
                  v28);
              }
            }
          }
          LOBYTE(id) = (_BYTE)id + 1;
        }
        while ( (unsigned __int8)id < 0x14u );
        v29 = this->m_current_player.m_object;
        if ( v29
          && vostok::intrusive_ptr<vostok::animation::mixing::binary_tree_weight_node,vostok::animation::mixing::binary_tree_base_node,vostok::threading::single_threading_policy>::c_ptr
          && v29->m_has_been_inserted )
        {
          survarium::player::update_camera((survarium::player *)v26);
        }
      }
    }
    else
    {
      this->m_player_inputs.m_end = this->m_player_inputs.m_begin;
      if ( v3 >= this->m_last_send_queued_packets_time_in_ms + 33 )
      {
        match_client = this->match_client;
        this->m_last_send_queued_packets_time_in_ms = v3;
        v20 = (int)match_client(this);
        *(_DWORD *)(v20 + 9112) = v3;
        *(_BYTE *)(v20 + 9116) = 0;
        vostok::network::match_client::send_queued_packets((vostok::network::match_client *)v20, v3);
      }
    }
  }
}
