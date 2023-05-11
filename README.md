# Simple QR generator

A simple tool to generate QR code from your terminal.

## Usage

```bash
$ qr --help
CLI tool to generate QR codes

Supported output format: Unicode, PNG, SVG

Usage: qr.exe [OPTIONS] [TEXT]

Arguments:
  [TEXT]
          The text you want to encode in the QR

Options:
  -t, --output-type <OUTPUT_TYPE>
          Set the output type of QR code

          [default: unicode]

          Possible values:
          - unicode: Output QR code to the terminal
          - png:     Output QR code to a PNG image
          - svg:     Output QR code to an SVG image

  -o, --output <FILE>
          File to write QR code to, if using PNG or SVG format

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```

## License

MIT


