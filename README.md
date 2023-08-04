# Tonic gRPC-Web attempt

## Setup

Add `127.0.0.1 tonic-web-play.localhost` in your /etc/hosts

## Run

**Server**:

```
$ cargo run --bin server
```

**Client**:

```
$ cargo run --bin client
```


Will run on `http://tonic-web-play.localhost:8301`

<!-- ## SSL generated with -->

<!-- ``` -->
<!-- openssl req -x509 -noenc -subj '/CN=tonic-web-play.localhost' -newkey rsa -keyout root.key -out root.crt -->
<!-- openssl req -noenc -newkey rsa -keyout client.key -out client.csr -subj '/CN=tonic-web-play.localhost' -addext subjectAltName=DNS:tonic-web-play.localhost -->
<!-- openssl x509 -req -in client.csr -CA root.crt -CAkey root.key -days 365 -out client.crt -copy_extensions copy -->
<!-- mv client.crt tonic-web-play.localhost.crt -->
<!-- mv client.key tonic-web-play.localhost.key -->
<!-- mv root.key ca.key -->
<!-- mv root.crt ca.pem -->
<!-- ``` -->

## The error

```
➜  tonic-web-play git:(main) ✗ curl --http1.1 -XPOST 'http://tonic-web-play.localhost:8301/ping.Ping/Ping' \
  -H 'content-type: application/grpc-web' \
  -H 'accept: application/grpc-web+proto' \
  --data-raw $'\u0000\u0000\u0000\u0000\u0002\n\u0000' -i
HTTP/1.1 200 OK
content-type: application/grpc-web+proto
grpc-status: 13
grpc-message: Missing%20request%20message.
content-length: 0
date: Fri, 04 Aug 2023 06:31:43 GMT
```

The client output:

```
➜  tonic-web-play git:(main) ✗ cargo run --bin client
   Compiling tonic-web-play v0.1.0 (/Users/kon/workspace/tonic-web-play)
    Finished dev [unoptimized + debuginfo] target(s) in 1.29s
     Running `target/debug/client`
BODY=Body(Full(b"\0\0\0\0\x05\n\x03Bob"))
REQUEST=Request { method: POST, uri: http://tonic-web-play.localhost:8301/ping.Ping/Ping, version: HTTP/1.1, headers: {"content-type": "application/grpc-web", "accept": "application/grpc-web"}, body: Body(Full(b"\0\0\0\0\x05\n\x03Bob")) }
REPLY=PingRsp { message: "Hello Bob!" }
```

