// This is called once response from the server is parsed.
// The number of arguments seems to be incorrect, which possibly causes misdecompilation
// But it seems like from lobby_connection_info it gets server and port
// Which are 127.0.0.1 and 1234 
// @TODO: see where 38 message is comming from
// Maybe here the lobby client is created and this succeeeds, but later on something writes 38 to it.
// Also we kill this connection, which is possibly incorrect
 void __userpurge survarium::lobby_client::connect(
        survarium::lobby_client *this@<ecx>,
        int a2@<eax>,
        const vostok::server_connection_info *lobby_connection_info)
{
  int v3; // edx
  unsigned __int16 v4; // cx

  v3 = *(_DWORD *)(a2 + 156);
  qmemcpy((void *)(a2 + 36), lobby_connection_info, 0x80u);
  v4 = *(_WORD *)(a2 + 40);
  *(_DWORD *)(a2 + 156) = v3;
  vostok::network::tcp_packet_client::connect(
    (vostok::network::tcp_packet_client *)(a2 + 168),
    (const char *)(a2 + 42),
    v4);
}
