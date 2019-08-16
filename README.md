# qr-worker

A simple worker that generates QR codes as SVG images.

## Query format

```
https://worker.example.com/fg=000000/bg=ffffff/min=128/max=256/ec=m/qz=1?data
```

Options are position-insensitive.

Option | Description | Values | Default
---|---|---|---
`data` (everything after `?`) | Data to encode in the QR code | Any | Whole URL of the request
`fg` | Foreground color | Hex colors (without `#`) | `000000`
`bg` | Background color | Hex colors (without `#`) | `ffffff`
`min` | Minimum size of the resulting code, including the quiet zone | [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) | n/a
`max` | Maximum size of the resulting code, including the quiet zone; overrides effect of `min` | [`u32`](https://doc.rust-lang.org/std/primitive.u32.html) | n/a
`ec` | Error correction | `l`, `m`, `q`, `h` - see [qrcode crate docs](https://docs.rs/qrcode/0.11.0/qrcode/types/enum.EcLevel.html#variants) | `m`
`qz` | Quiet zone | `1` = `true`, `0` = `false` | `1`