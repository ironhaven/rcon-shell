# rcon-shell
This crate proviesa a executable that provies a command line interface to to some server (Valve Source Server, Minecraft) that takes commands with the rcon protocol.

## Features
  - Connect with ip address or hostname (172.0.0.1, myserver.com).
  - Provied the rcon server password as an command line argument or interactively.
  - Readline like support provied by the [rustyline crate](https://github.com/kkawakam/rustyline).

## Bugs
  - When sending commands to a minecraft server you may get a "Uneepected EOF" or "Connection reset" error that crashes the program. Blame the crap rcon support.
  - On windows use cmd.exe with this program because of sortcomings in the rustyline crate.
