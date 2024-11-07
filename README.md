# egui-fixed-scrollbar

A fixed-size scrollbar widget for egui that provides a simple way to add scrollbars to your egui applications which modify floats.

## Features
- Fixed-width scrollbar with configurable handle height
- Smooth scrolling support
- Configurable scroll sensitivity
- Multiple container options (side panel, floating, docked)

## Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
egui-fixed-scrollbar = "0.1.0"
```

Basic usage:
```rust
use egui_fixed_scrollbar::FixedScrollbar;

// As a side panel:
scrollbar.show_in_side_panel(ctx, "my_scrollbar");

// As a floating element:
scrollbar.show_floating(ui, egui::pos2(100.0, 100.0));

// Or docked to an area:
let area = ui.available_rect_before_wrap();
scrollbar.show_docked(ui, area);

// Or directly added to a UI:
ui.add(scrollbar);
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.