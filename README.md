# daytime
> A useful debugging and measurement tool is a daytime service.  A daytime service simply sends a the current date and time as a character string without regard to the input.
> [- Daytime Protocol](https://tools.ietf.org/html/rfc867)

1. Install xinetd on Ubuntu 16.04

```
$  sudo apt-get install xinetd
```

2. Enable daytime server

```
$ diff -u /etc/xinetd.d/daytime.origin /etc/xinetd.d/daytime
--- /etc/xinetd.d/daytime.origin        2018-07-16 12:04:13.401053882 +0900
+++ /etc/xinetd.d/daytime       2018-07-16 11:55:12.526850335 +0900
@@ -4,7 +4,7 @@
 # This is the tcp version.
 service daytime
 {
-       disable         = yes
+       disable         = no
        type            = INTERNAL
        id              = daytime-stream
        socket_type     = stream
```

3. Restart xinetd service

```
$ /etc/init.d/xinetd restart
```

4. Run daytime client

```
$ cargo run localhost
```
