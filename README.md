# Dispatch

Dispatch is a sub-command dispatcher.

You've probably seen CLI programs that have many subcommands.
For example with containers you can do `crictl push/pull/run/etc`.
For openssl you can do `openssl enc/base64/rsa/rsautl/etc`.
With these programs you have a primary command, and a bunch of subcommands.

Dispatch is a library that lets you easily add subcommands like this in your programs.