# xmllite

This is a boilerplate to create a spoofed xmllite library.

It is designed to override the original library by reexporting the original functions.
It also creates a new thread on the load which runs the desired functionality in the async tokio runtime.

### Usage
Compile the library as usual DLL, but make sure to include the original xmllite DLLs in the `/bin` directory as they are
embedded to the binary at compile time based on a target arch.

- `xmllite_32.dll` - for 32-bit system
- `xmllite_64.dll` - for 64-bit system

In order to run your code modify the `main` function to your liking. It is already prepared for async code, but you can 
remove the tokio dependency if you have to.

### WARNING
Do not replace the original xmllite library in the System32 directory as it will cause the system to be unable to boot up.

Good idea is to put the compiled binary into the folder of an application that depends on xmllite.dll
