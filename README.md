# Sol Language

Sol is a language made for study purpose, it is inspired by gdscript language, and its designed for systems programming.

## Sample code

This code is from the [hello_world.sol](tests/hello_world.sol) file.

```gdscript
import std;

func main() {
    print("Hello, World!");
}
```

## Branchs contents

- `main` - All the heavy developments it's here.
- `nightly` - Unstable release.
- `stable` - Stable edition of the language.

## Compiling from source

Use the rust package manager [cargo](https://doc.rust-lang.org/cargo/) to compile the language from source.

### In debug mode
This mode of compiling, builds the language without optimizations, so the compile timer is faster than the release mode, it's recommended to use this mode during the development.

In the root folder of the project, type `cargo build` in your console.

It will build all the members of the language project, and they can be encountered in the `/target/debug/` folder.

### In release folder
This mode of compiling, builds the language with optimizations, so the compile timer is slower than the debug mode, it's recommended to use this mode on building to release for the nightly or stable builds.

In the root folder of the project, type `cargo build --release` in your console.

It will build all the members of the language project, and they can be encountered in the `/target/release/` folder.

## License

This project is using the MIT license, the license can be encountered in [LICENSE](LICENSE) file.
