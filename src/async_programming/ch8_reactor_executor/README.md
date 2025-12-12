

### http1 发生连续两次READABLE

```
Executor: creating a new waker for task 0
Program starting
first poll - start operation, register task:1 for readable
WouldBlock - not ready yet, set waker for task:1
main: 1 tasks remaining, sleep until notified
Reactor: event for task 1, getting waker  // 这里竟然发生了两次针对http1 请求的event，所以ready_queue被push了两个相同 id 的对象
waker: waking task 0, unpark thread
Reactor: event for task 1, getting waker
waker: waking task 0, unpark thread
Executor: creating a new waker for task 0
peer closed, reply len: 141
HTTP/1.1 200 OK
content-length: 6
connection: close
content-type: text/plain; charset=utf-8
date: Fri, 12 Dec 2025 09:09:38 GMT

hello1
first poll - start operation, register task:2 for readable
WouldBlock - not ready yet, set waker for task:2
Executor: creating a new waker for task 0
WouldBlock - not ready yet, set waker for task:2
main: 1 tasks remaining, sleep until notified
Reactor: event for task 2, getting waker
waker: waking task 0, unpark thread
Executor: creating a new waker for task 0
peer closed, reply len: 141
HTTP/1.1 200 OK
content-length: 6
connection: close
content-type: text/plain; charset=utf-8
date: Fri, 12 Dec 2025 09:09:38 GMT

hello2
main: all tasks completed
```

### http2 请求发生连续两次READABLE(与http1 现象不太一样)

```
Executor: creating a new waker for task 0
Program starting
first poll - start operation, register task:1 for readable
WouldBlock - not ready yet, set waker for task:1
thread-main: 1 tasks remaining, sleep until notified
Reactor: event for task 1, getting waker ...
waker: waking task 0, unpark thread
Executor: creating a new waker for task 0
peer closed, reply len: 141
HTTP/1.1 200 OK
content-length: 6
connection: close
content-type: text/plain; charset=utf-8
date: Fri, 12 Dec 2025 11:31:03 GMT

hello1
first poll - start operation, register task:2 for readable
WouldBlock - not ready yet, set waker for task:2
thread-main: 1 tasks remaining, sleep until notified
Reactor: event for task 2, getting waker ...
waker: waking task 0, unpark thread
Reactor: event for task 2, getting waker ...
waker: waking task 0, unpark thread
Executor: creating a new waker for task 0
peer closed, reply len: 141
HTTP/1.1 200 OK
content-length: 6
connection: close
content-type: text/plain; charset=utf-8
date: Fri, 12 Dec 2025 11:31:03 GMT

hello2
Executor: task 0 not found, maybe already completed, skip  // 果然这里需要检测
thread-main: all tasks completed
```