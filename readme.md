# Story tablet driver
Custom tablet driver for CTL-472

## Build
`cargo build`

## Installation
### Windows
Run install.bat. Driver will auto restart on boot.
### Others
Auto start installer not provided. Manually add it (for now).

## Customizing
Open `configurator/index.html` (incomplete)

## Spec
| Name     | CTL-472               |
|----------|-----------------------|
| Width    | 152 mm (15200)        |
| Height   | 95 mm  (9500)         |
| Vendor   | Wacom (0x056a)        |
| Product  | CTL-472 (0x037a)      |
| Features | 0x02 0x02 (Digitizer) |

## License

```text
Copyright (c) 2020 storycraft

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```