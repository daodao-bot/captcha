# captcha

---

## about

这是一个使用 rust 语言开发的命令行程序，用于生成图片验证码。它将返回一个随机的验证码文本和图像的 base64 格式字符串。

This is a command-line program developed using the rust language to generate image verification codes. It will return a random verification code text and a base64 format string of the image.

当前提供以下几种获取程序的方式：

The following ways are currently provided to obtain the program:

- release 下载
- docker 镜像
- compile 编译

---

## release

[Releases · daodao-bot/captcha](https://github.com/daodao-bot/captcha/releases)

---

## docker

[daodaobot/captcha - Docker Image | Docker Hub](https://hub.docker.com/r/daodaobot/captcha)

---

## compile

```shell
cargo build --release
./target/release/captcha -h
```
