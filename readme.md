# P2P Gossip Protocall Demonstration

This is just a small demostration of how peer to peer gossip protocall works. In this project we'll be checking it using multiple terminals.

---

## Prerequisites

- **Rust** (stable, 1.75+): https://rustup.rs
- No PostgreSQL needed for local testing (mock DB activates automatically)

```powershell
rustup update stable
```

---

## Installation & Build

```powershell
git clone https://github.com/codeArray-go/P2P.rs.git
cd "P2P.rs"
cargo build
```

## Testing:-

- In First terminal
```powershell
cargo run --quiet <Your prefered port>
```

- In another terminal 
```powershell
cargo run --quiet <Your prefered port> 127.0.0.0.1:<previuos terminal port>
```

*Change <> as instructed*

Now you could type your message and then press **Enter key** to send it