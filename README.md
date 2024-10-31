# dcui

> A minimal TUI application (under 1 MB compiled) for controlling external displays.

<table>
<tbody>
  <tr>
    <td>
    <img width="450" src="https://github.com/user-attachments/assets/e3a95ec9-2e32-4f94-83b7-750524e44fc5" />
    </td>
    <td>
    <img width="450" src="https://github.com/user-attachments/assets/6d4cbfeb-2aa0-43d0-ab4d-bab85c5377d9" />
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

## About

### TL;DR (Disclaimer)

- So far, only tested on Linux
- Feature-incomplete

### Objectives

There were two personal objectives in creating this application:

1. To test-drive the Ratatui library, assessing its suitability for a more extensive TUI project.

2. To create an accessible tool for adjusting display brightness with instant visual feedback, directly from the environment that is always open and in use on my machine — the terminal.

Both are fulfilled, and others may benefit from the latter as well. Still, the application is lacking in features, proper project structure, and testing across devices and operating systems to serve a general user base. So far, exploratory testing has only been done on Linux, so release binaries for other OSes are not yet provided.

Feasible optimizations, enhancements, and fixes of personal and public interest will be added as time permits.

## Credits

Thank you to all the contributors of the projects listed in the `Cargo.toml` file.
