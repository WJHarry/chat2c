package main

import (
	"log/slog"
	"net"
	"strings"
)

/**
 * UDP内网穿透
 * 1. Client1向Server注册自己的地址（其实是网关地址）IP_PORT_NAT1
 *    Client2向Server注册自己的地址（其实是网关地址）IP_PORT_NAT2
 * 2. Client1和Client2想要发起通信，Client1向Server发送请求，Server找到IP_PORT_NAT2发给Client1，找到IP_PORT_NAT1发给Client2
 * 3. Client1向Client2发消息，必然失败，因为Client2不认识这个来源地址
 * 4. Client2向Client1发消息，成功，因为Client1发送过信息给Client2
 * 5. Client1再向Client2发消息，成功，因为Client2发送过信息给Client1
 **/

type Peer struct {
	UID string
	addr net.Addr
}

var PEERS_TABLE = make(map[string]Peer, 10)

func main() {
	slog.Info("Start...")
	listener, err := net.ListenUDP("udp", &net.UDPAddr{IP: net.IPv4zero, Port: 19999})
	if err != nil {
		slog.Error("Error while registering Peer", err)
		return
	}
	data := make([]byte, 1024)
	for {
		n, addr, err := listener.ReadFrom(data)
		if (err != nil) {
			slog.Error("Error while reading from UDP", err)
			continue
		}
		content := string(data[:n])
		content_seps := strings.Split(content, "#")
		if len(content_seps) != 2 { continue }

		if content_seps[0] == "register" {
			UID := content_seps[1]
			slog.Info("Received", "UID", UID, "Addr", addr.String())
			PEERS_TABLE[UID] = Peer{UID: UID, addr: addr}
		} else if content_seps[0] == "connect" {
			go connect(listener, content_seps[1], addr);
		}
	}
}

/*
   接受连接请求
*/
func connect(listener *net.UDPConn, content string, addr net.Addr) {
	// data: MyUID_TargetUID
	dataSeps := strings.Split(content, "_")
	if len(dataSeps) != 2 {
		slog.Error("Connect invitation string error")
		return
	}
	myUID := dataSeps[0]
	targetUID := dataSeps[1]

	slog.Info("Received connect invitation", "MyUID", myUID, "TargetUID", targetUID)
	myPeer, exists := PEERS_TABLE[myUID]
	if !exists {
		slog.Error("UID "+myUID+" doesn't exist");
		return
	}
	if addr.String() != myPeer.addr.String() {
		slog.Error("UID "+myUID+" Host not correct");
		return
	}
	targetPeer, exists := PEERS_TABLE[targetUID]
	if !exists {
		slog.Error("UID "+targetUID+" doesn't exist");
		return
	}

	connectCallback(targetPeer, myPeer, listener)
	connectCallback(myPeer, targetPeer, listener)
}

func connectCallback(target_peer Peer, content_peer Peer, listener *net.UDPConn) {
	udpAddr, _ := net.ResolveUDPAddr(target_peer.addr.Network(), target_peer.addr.String())
	content := "connect_callback#" + content_peer.UID + "_" + content_peer.addr.String();
	listener.WriteTo([]byte(content), udpAddr)
	slog.Info("Sent to", "address", udpAddr, "content", content_peer.UID + "_" + content_peer.addr.String())
}
