# Vaultify
A password manager built with Rust prioritizing security, speed, and reliability.

# Why Rust?
Rust is known for its memory safety and high performance. Its focus on safety without sacrificing performance makes it an excellent choice for developing a secure password manager. Additionally, Rust can be compiled to WebAssembly, allowing for cross-platform compatibility.

Some of the notable apps that use Rust
- Dropbox: Dropbox has developed a high-performance network storage system called "Magic Pocket" using Rust.
- Microsoft Azure: Microsoft has used Rust to develop a number of components of their cloud computing platform, including the networking stack and the Project Verona research project.
- Cloudflare: Cloudflare uses Rust to build components of their edge computing infrastructure, such as their L7 router and WebAssembly-based serverless platform.
- Figma: Figma, the collaborative design platform, uses Rust for its rendering engine.
- Discord: Discord uses Rust for a number of performance-critical components, including audio processing and video encoding.
- Red Hat: Red Hat has developed a number of tools in Rust, including the Stratis storage management system and the Podman container engine.
- Mozilla: Rust was originally developed by Mozilla and is used extensively in their Firefox browser, as well as other projects such as the Servo browser engine.

# How Does it Work?
Vaultify impolements a "vault" methodology.

1. Creates a ***Vault Key*** with `hash(email + password)`
2. Creates ***Authentication Key*** with `hash(vault key + password)`

* The ***Vault Key*** is used to both encrypt/decrypt the ***vault***
* The ***vault*** stores all the stored logins & passwords
* ***Authentication Key*** is used to request your vault from the server

    ### Server knowledge 
    The server shouldn't know the password or the Vault Key. This is a principle known as ***"zero-knowledge"*** and is a cornerstone of secure password managers.
    
    ***All the server knows is the user's email and authenticaiton key.***
    
    Therefore, even in the case of the server being hacked, only the user themselves will be able to get into the vault.