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
