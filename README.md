# rcon-shell

This crate provides an executable that provides a command line interface to some server (Valve Source Server, Minecraft) that takes commands with the rcon protocol.


## Features

  - Connect with IP address or hostname (172.0.0.1, myserver.com).

  - You can provide the rcon server password as a command line argument or interactively.

  - Readline-like support provided by the [rustyline crate](https://github.com/kkawakam/rustyline).


## Bugs

  - When sending commands to a Minecraft server you may get an "Unexpected EOF" or "Connection reset" error that crashes the program. Blame the crap rcon impl.
  
## Changelog
  - 0.1.1
    - Command line history is saved on crash.
