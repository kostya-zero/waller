# Waller

Waller is a small application to help you in applying pictures as wallpaper with tools like `swaybg` or `feh`.
It allow you to easily apply them by one command and easy configuration.

## Usage

Show help message:

```shell
waller help
```

Apply picture that you specify in `config.toml`:

```shell
waller apply
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
 default_wall = "/path/to/wallpaper.png"
 random_folder = "/path/to/folder/with/pictures"
```

| Option          | Descritpion                                                            | Value                              |
| :-------------- | :--------------------------------------------------------------------- | :--------------------------------- |
| `method`        | Application which will be used to apply picture.                       | `swaybg` or `feh`                  |
| `mode`          | Image display mode. Can work diffrent in each wallpaper application.   | `fit`, `fill`, `center`, `stretch` |
| `default_wall`  | Path to default image that will be applied with `apply` argument.      | `string`                           |
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
