# Waller

Waller is a small application to help you in applying pictures as wallpaper with tools like `swaybg` or `feh`.
It allow you to easily apply them by single command and a simple .

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

Apply random wallpaper from directory that you specify in `config.toml`:

```shell
waller random
```

## Configuration

Waller are using the configuration file placed in your home directory in `.config/waller`.
File have a TOML structure.

```toml
 method = "swaybg"
 mode = "fill"
 random_folder = "/path/to/folder/with/pictures"
```

| Option          | Descritpion                                                            | Value                              |
| :-------------: | :--------------------------------------------------------------------- | :--------------------------------- |
| `method`        | Application which will be used to apply picture.                       | `swaybg` or `feh`                  |
| `mode`          | Image display mode. Can work diffrent in each wallpaper application.   | `fit`, `fill`, `center`, `stretch` |
| `random_folder` | Path to folder with images to apply. Waller will select them randomly. | `string`                           |

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
