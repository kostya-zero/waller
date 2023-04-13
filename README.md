# Waller

Waller is a small application that will help you apply pictures as wallpaper with tools like `swaybg` or `feh`.
It allows you to easily apply them with a single command and a simple setup.

## Usage

Show help message:

```shell
waller help
```

Add image to your collection:

```shell
waller add <path_to_image>
```

List all wallpapers in your collection:

```shell
waller list
```

Remove wallpaper from collection by index:

```shell
waller rm <index>
```

Apply picture from collection by index:

```shell
waller apply <index>
```

Apply picture by path from argument:

```shell
waller set <path_to_picture>
```

## Configuration

Waller uses a configuration file located in your home directory in `.config/waller`.
File have a TOML structure.

```toml
 method = "swaybg"
 mode = "fill"
 walls = []
 recent = ""
```

| Option          | Descritpion                                                            | Value                              |
| :-------------: | :--------------------------------------------------------------------- | :--------------------------------- |
| `method`        | Application which will be used to apply picture.                       | `gnome`, `swaybg` or `feh`         |
| `mode`          | Image display mode. Can work diffrent in each wallpaper application.   | `fit`, `fill`, `center`, `stretch` |
| `walls`         | Collection of images that you can add with `add` argument.             | Array of `string.`                 |
| `recent`        | Image that you set recently.                                           | `string`                           |

## Build

Nothing special. Just run `cargo` with `--release` flag, or without them if you want to build debug binary.

```shell
cargo build --release
```

## Note from author

This is my first project in written in Rust.
I will be glad if you support my project or help in its improvement and bug fixes.
By the way sorry for my bad English. Sometimes I use translator to check if I have any errors.
Have a good day!
