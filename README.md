# dcui

[badge__build-status]: https://img.shields.io/github/actions/workflow/status/ttytm/dcui/ci.yml?branch=main&logo=github&logoColor=C0CAF5&labelColor=333
[badge__version]: https://img.shields.io/github/v/tag/ttytm/dcui?logo=task&logoColor=C0CAF5&labelColor=333&color=DEA584
[badge__msrv]: https://img.shields.io/badge/MSRV-1.74.0-DEA584?logo=rust&labelColor=333

[![][badge__build-status]](https://github.com/ttytm/dcui/actions?query=branch%3Amain)
[![][badge__version]](https://github.com/ttytm/dcui/releases/latest)
![][badge__msrv]

> A minimal TUI application (under 1 MB compiled) for controlling external displays.

<table>
<tbody>
  <tr>
    <td>
    <img width="450" src="https://github.com/user-attachments/assets/edf4565f-5f75-47f5-8138-0c9b14bdc3dc" />
    </td>
    <td>
    <img width="450" src="https://github.com/user-attachments/assets/e6ccb24c-1b45-45d0-9133-530a2e4fd590" />
    </td>
  </tr>
</tbody>
</table>

## Keymaps

| Description                                | Key                                                                                                                |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| Toggle Info                                | <kbd>?</kbd>                                                                                                       |
| Quit                                       | <kbd>q </kbd>                                                                                                      |
| Next Pane                                  | <kbd>tab</kbd>, <kbd>l</kbd>                                                                                       |
| Previous Pane                              | <kbd>shift</kbd><kbd>tab</kbd>, <kbd>h</kbd>                                                                       |
| Focus Neighboring Pane                     | <kbd>ctrl</kbd><kbd>h</kbd>\|<kbd>ctrl</kbd><kbd>j</kbd>\|<kbd>ctrl</kbd><kbd>k</kbd>\|<kbd>ctrl</kbd><kbd>l</kbd> |
| Down (Next Item in Current Pane)           | <kbd>down</kbd>, <kbd>j</kbd>                                                                                      |
| Up (Previous Item in Current Pane)         | <kbd>up</kbd>, <kbd>k</kbd>                                                                                        |
| Increase Value of Selected Display Setting | <kbd>left</kbd>, <kbd>shift</kbd><kbd>h</kbd>                                                                      |
| Decrease Value of Selected Display Setting | <kbd>right</kbd>, <kbd>shift</kbd><kbd>l</kbd>                                                                     |
| Set Display Setting Value to `<number>`    | <kbd><1-or-2-digit-input></kbd>                                                                                    |
| Toggle Grayscale                           | <kbd>shift</kbd><kbd>s</kbd>                                                                                       |

Keymaps are currently hard-coded.

## Installation

Release binaries can be downloaded in bare executable, AppImage and Deb formats from the [releases page](https://github.com/ttytm/dcui/releases).
Using `cargo` for installation from the Git source or self-compilation are methods for installation on all (including untested) platforms.

```sh
cargo install --git https://github.com/ttytm/dcui
```

## Disclaimer

This is early beta software.
So far, exploratory testing has only been done on GNU/Linux, so it's the only OS prebuilt binaries are provided for.

## Credits

Thank you to all the contributors of the projects listed in the `Cargo.toml` file.
