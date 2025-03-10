- Login Server: Sign in attempt interval violated
- Getting this error sometimes
```rust
Failure(MidHandshakeSslStream {
    stream: SslStream {
        stream: TcpStream {
            addr: "127.0.0.1:1234",
            peer: "127.0.0.1:56692",
            socket: 196,
        },
        ssl: Ssl {
            state: "SSLv3/TLS write server done",
            verify_result: X509VerifyResult {
                code: 0,
                error: "ok",
            },
        },
    },
    error: Error {
        code: ErrorCode(5),
        cause: None,
    },
})
```
