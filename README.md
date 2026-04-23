# unin - by notchapplez

### Now you may ask – what is unin?
#### → And I have the answer: 
##### unin is a simple, yet powerful, universal installer. I know you hate remembering all the commands for compiling projects in all the different languages. 
###### So I made this. I thought of this in my sleep and I'm glad I did. 

#### unin has also a feature to self-update without having to manually copy the files and compile the code.
#### unin also moves all the release files to /usr/local/bin so they don't conflict with other executables in /usr/bin.

## Syntax:
| Command                                                                        | Description                                                                                |
|--------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------|
| unin --setup <full(all the languages),rust,cmake,make,go,zig,swift,haskell,d,> | Setup languages system-wide                                                                |
| unin <"path"> (--noinstall)                                                    | Compile project at the given path, noinstall only compiles the binaries, doesn't move them |
| unin --clean  <"path">                                                         | Clean the artefacts built                                                                  |
