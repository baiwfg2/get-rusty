https://github.com/PacktPublishing/Asynchronous-Programming-in-Rust

## strange behavivor

```
received:Event { events: 1, epoll_data: 4 }
HTTP/1.1 200 OK
content-length: 9
connection: close
content-type: text/plain; charset=utf-8
date: Thu, 04 Dec 2025 14:18:40 GMT

request-4
-------------

connection closed by peer: Event { events: 1, epoll_data: 4 }
received:Event { events: 1, epoll_data: 3 }
HTTP/1.1 200 OK
content-length: 9
connection: close
content-type: text/plain; charset=utf-8
date: Thu, 04 Dec 2025 14:18:41 GMT

request-3
-------------

connection closed by peer: Event { events: 1, epoll_data: 3 }
received:Event { events: 1, epoll_data: 2 }
HTTP/1.1 200 OK
content-length: 9
connection: close
content-type: text/plain; charset=utf-8
date: Thu, 04 Dec 2025 14:18:42 GMT

request-2
-------------

connection closed by peer: Event { events: 1, epoll_data: 2 }
connection closed by peer: Event { events: 1, epoll_data: 2 }
received:Event { events: 1, epoll_data: 1 }
HTTP/1.1 200 OK
content-length: 9
connection: close
content-type: text/plain; charset=utf-8
date: Thu, 04 Dec 2025 14:18:43 GMT

request-1
-------------

connection closed by peer: Event { events: 1, epoll_data: 1 }
finnished
```

### tcpdump
```
22:18:43.729409 IP 127.0.0.1.8080 > 127.0.0.1.55550: Flags [P.], seq 1:145, ack 69, win 512, options [nop,nop,TS val 1949402366 ecr 1949398364], length 144: HTTP: HTTP/1.1 200 OK
E...e.@.@..a.............e.9...t...........
t1..t1u\HTTP/1.1 200 OK
content-length: 9
connection: close
content-type: text/plain; charset=utf-8
date: Thu, 04 Dec 2025 14:18:43 GMT

request-1
22:18:43.729434 IP 127.0.0.1.55550 > 127.0.0.1.8080: Flags [.], ack 145, win 511, options [nop,nop,TS val 1949402366 ecr 1949402366], length 0
E..4l.@.@..................t.e.......(.....
t1..t1..
22:18:43.729468 IP 127.0.0.1.8080 > 127.0.0.1.55550: Flags [F.], seq 145, ack 69, win 512, options [nop,nop,TS val 1949402366 ecr 1949402366], length 0  (SERVER SEND FIN against request-1 stream)
E..4e.@.@................e.....t.....(.....
t1..t1..
22:18:43.729609 IP 127.0.0.1.55544 > 127.0.0.1.8080: Flags [F.], seq 69, ack 1, win 512, options [nop,nop,TS val 1949402366 ecr 1949398364], length 0
E..4Aa@.@..`.............Y.:.........(.....
t1..t1u\
22:18:43.729655 IP 127.0.0.1.55550 > 127.0.0.1.8080: Flags [F.], seq 69, ack 146, win 512, options [nop,nop,TS val 1949402366 ecr 1949402366], length 0  (CLIENT 已经走到了streams[] drop 流程，开始发FIN)
E..4l.@.@..................t.e.......(.....
t1..t1..
22:18:43.729681 IP 127.0.0.1.8080 > 127.0.0.1.55550: Flags [.], ack 70, win 512, options [nop,nop,TS val 1949402366 ecr 1949402366], length 0
E..4e.@.@................e.....u.....(.....
t1..t1..
22:18:43.729709 IP 127.0.0.1.55558 > 127.0.0.1.8080: Flags [F.], seq 69, ack 146, win 512, options [nop,nop,TS val 1949402366 ecr 1949401367], length 0
E..4W.@.@....................4.;.....(.....
t1..t1..
22:18:43.729731 IP 127.0.0.1.8080 > 127.0.0.1.55558: Flags [.], ack 70, win 512, options [nop,nop,TS val 1949402366 ecr 1949402366], length 0
E..4..@.@.<..............4.;........IC.....
t1..t1..
22:18:43.729758 IP 127.0.0.1.55564 > 127.0.0.1.8080: Flags [F.], seq 69, ack 146, win 512, options [nop,nop,TS val 1949402366 ecr 1949400367], length 0
22:18:43.729765 IP 127.0.0.1.8080 > 127.0.0.1.55564: Flags [.], ack 70, win 512, options [nop,nop,TS val 1949402366 ecr 1949402366], length 0
E..4..@.@.<...............X..._............
t1..t1..
22:18:43.729791 IP 127.0.0.1.55570 > 127.0.0.1.8080: Flags [F.], seq 69, ack 146, win 512, options [nop,nop,TS val 1949402366 ecr 1949399367], length 0
E..4..@.@.."............K`..K........(.....
t1..t1yG
22:18:43.729798 IP 127.0.0.1.8080 > 127.0.0.1.55570: Flags [.], ack 70, win 512, options [nop,nop,TS val 1949402366 ecr 1949402366], length 0
E..4..@.@.<.............K...K`......0......
t1..t1..
22:18:43.770097 IP 127.0.0.1.8080 > 127.0.0.1.55544: Flags [.], ack 70, win 512, options [nop,nop,TS val 1949402407 ecr 1949402366], length 0
E..4..@.@.;..................Y.;.....(.....
t1.'t1..
22:18:44.729717 IP 127.0.0.1.8080 > 127.0.0.1.55544: Flags [P.], seq 1:145, ack 70, win 512, options [nop,nop,TS val 1949403366 ecr 1949402366], length 144: HTTP: HTTP/1.1 200 OK
E.....@.@.;^.................Y.;...........
t1..t1..HTTP/1.1 200 OK
content-length: 9
connection: close
content-type: text/plain; charset=utf-8
date: Thu, 04 Dec 2025 14:18:44 GMT

request-0 （服务器试图发request-0 的响应，但是client已经关闭了，所以才给服务端来了 RST）
22:18:44.729735 IP 127.0.0.1.55544 > 127.0.0.1.8080: Flags [R], seq 425322811, win 0, length 0

```

You've made some excellent observations about the program's behavior and the `tcpdump` logs. This is a classic and subtle issue when working with low-level I/O multiplexing like `epoll`. Let's break down what's happening.

### 1. Why does a stream receive two EOF notifications?

This is a known, though not always common, behavior with edge-triggered `epoll` (`EPOLLET`). Here's the sequence of events:

1.  **First EOF:** The server finishes sending data and closes its end of the connection by sending a `FIN` packet. Your client's `epoll` detects this as a readable event (`EPOLLIN`). Your `handle_events` function calls `read()`, which returns `0`, correctly indicating End-Of-File (EOF). You print "connection closed by peer" and increment `handled_eve`.

2.  **Socket State:** At this point, your client-side socket is in a "half-closed" state. It can no longer receive data, but it can still send it (though it's not doing so). You have processed the `EPOLLIN` event, but you haven't removed the file descriptor from `epoll`'s interest list.

3.  **Second Notification:** Because the socket is still open on your end but closed by the peer, `epoll` can send another notification. This is often a combination of events, including `EPOLLIN` and `EPOLLHUP` (Hang-up). Since your code only checks for readability, it sees another readable event. When you call `read()` on this already-closed socket, it immediately returns `0` again. This leads to the second "connection closed by peer" message.

This is essentially a "spurious" notification from `epoll`'s perspective regarding new data, but it's a valid notification about the socket's state. The original author of the code you're referencing also noted this as a surprising behavior.

A common way to prevent this is to explicitly close or de-register the socket from `epoll` after detecting the first EOF.

### 2. Why does the client send multiple `FIN` packets and receive a response *after*?

This is the most confusing part, and it stems from a race condition between your client application and the server, combined with how TCP connection termination works.

Looking at your `tcpdump` log:

1.  **Client sends `FIN`s:** Your client code initiates requests with very short delays between them (5s, 4s, 3s, 2s, 1s). The requests with shorter delays get responses first. As soon as a response is fully read and the connection is closed by the server, your client loop for that stream ends. However, your main `t4_main` function continues to run. The `TcpStream` objects are still in scope. When `t4_main` finally finishes, the `streams` vector is dropped. This is when Rust automatically closes all the `TcpStream` sockets that are still open, causing your client to send `FIN` packets for all of them at nearly the same time.

2.  **Server is still working:** Meanwhile, the server is still processing the long-delay request (e.g., `request-0` with a 5-second delay). It's completely unaware that your client is already trying to close the socket from its end.

3.  **Race Condition:**
    *   Your client sends a `FIN` for the `request-0` connection.
    *   The server, having finished its 5-second sleep, tries to send the `request-0` data on that same connection.
    *   The server's TCP stack sees the incoming `FIN` from your client. Depending on the exact timing, it might send the data and *then* process the `FIN`, or it might see the `FIN` and decide the connection is closing. In your log, it appears the server sends the data.
    *   The server's attempt to send data after receiving a `FIN` can lead to the kernel sending a `RST` (Reset) packet, as seen in the last line of your `tcpdump`: `Flags [R]`. This is the kernel's way of saying, "This conversation is over; I'm not listening anymore."

In short, your client application decided it was "done" and started closing all connections before the server had finished processing all the requests. The TCP protocol tried to handle this gracefully, but the out-of-order closing and sending of data resulted in the confusing log and the final connection being abruptly reset.

# ch5 fibers

```
top: 110142354820352, bottom: 110142354820400, align_bottom:110142354820400
Bytes of the `hello` function at 0x642c57e95b90:
72 131 236 56 72 141 124 36
mem: 110142354820400, val: 0
mem: 110142354820399, val: 0
mem: 110142354820398, val: 0
mem: 110142354820397, val: 0
mem: 110142354820396, val: 0
mem: 110142354820395, val: 0
mem: 110142354820394, val: 0
mem: 110142354820393, val: 0
mem: 110142354820392, val: 0
mem: 110142354820391, val: 0 -- highest byte of hello address
mem: 110142354820390, val: 0
mem: 110142354820389, val: 100
mem: 110142354820388, val: 44
mem: 110142354820387, val: 87
mem: 110142354820386, val: 233
mem: 110142354820385, val: 91
mem: 110142354820384, val: 144 (这是 align bottom offset(-16) 的位置，即对应着hello地址的低字节 0x90) ; 在 `"mov rsp, [{0} + 0x00]",` 后，rsp 即指向这里
mem: 110142354820383, val: 0
mem: 110142354820382, val: 0
mem: 110142354820381, val: 0
mem: 110142354820380, val: 0

```