// use embedded_graphics::{
//     coord::Coord,
//     fonts::{Font, Font12x16},
//     prelude::*,
//     primitives::{Circle, Line},
//     Drawing,
// };
// use embedded_hal::prelude::*;
// use epd_waveshare::{
//     epd7in5_v2::{Display7in5, EPD7in5},
//     graphics::{Display, DisplayRotation},
//     prelude::*,
// };
// use linux_embedded_hal::{
//     spidev::{self, SpidevOptions},
//     sysfs_gpio::Direction,
//     Delay, Pin, Spidev,
// };
// use profont::ProFont24Point;




// fn run_display_test() -> Result<(), io::Error> {
//     let mut spi = Spidev::open("/dev/spidev0.0")?; //.expect("spidev directory");
//     let options = SpidevOptions::new()
//         .bits_per_word(8)
//         .max_speed_hz(4_000_000)
//         .mode(spidev::SpiModeFlags::SPI_MODE_0)
//         .build();
//     spi.configure(&options)?; //.expect("spi configuration");

//     // Configure Digital I/O Pin to be used as Chip Select for SPI
//     let cs = Pin::new(8);
//     cs.export().expect("cs export");
//     while !cs.is_exported() {}
//     cs.set_direction(Direction::Out).expect("CS Direction");
//     cs.set_value(1).expect("CS Value set to 1");

//     let busy = Pin::new(24);
//     busy.export().expect("busy export");
//     while !busy.is_exported() {}
//     busy.set_direction(Direction::In).expect("busy Direction");

//     let dc = Pin::new(25);
//     dc.export().expect("dc export");
//     while !dc.is_exported() {}
//     dc.set_direction(Direction::Out).expect("dc Direction");
//     dc.set_value(1).expect("dc Value set to 1");

//     let rst = Pin::new(17);
//     rst.export().expect("rst export");
//     while !rst.is_exported() {}
//     rst.set_direction(Direction::Out).expect("rst Direction");
//     rst.set_value(1).expect("rst Value set to 1");

//     let mut delay = Delay {};

//     println!("Init");
//     let mut epd7in5 =
//         EPD7in5::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("eink initalize error");
//     let mut display = Display7in5::default();

//     println!("Test all the rotations");
//     display.clear_buffer(Color::White);

//     display.set_rotation(DisplayRotation::Rotate0);
//     display.draw(
//         Font12x16::render_str("Rotate 0!")
//             .stroke(Some(Color::Black))
//             .fill(Some(Color::White))
//             .translate(Coord::new(5, 50))
//             .into_iter(),
//     );

//     display.set_rotation(DisplayRotation::Rotate90);
//     display.draw(
//         Font12x16::render_str("Rotate 90!")
//             .stroke(Some(Color::Black))
//             .fill(Some(Color::White))
//             .translate(Coord::new(5, 50))
//             .into_iter(),
//     );

//     display.set_rotation(DisplayRotation::Rotate180);
//     display.draw(
//         Font12x16::render_str("Rotate 180!")
//             .stroke(Some(Color::Black))
//             .fill(Some(Color::White))
//             .translate(Coord::new(5, 50))
//             .into_iter(),
//     );

//     display.set_rotation(DisplayRotation::Rotate270);
//     display.draw(
//         Font12x16::render_str("Rotate 270!")
//             .stroke(Some(Color::Black))
//             .fill(Some(Color::White))
//             .translate(Coord::new(5, 50))
//             .into_iter(),
//     );

//     epd7in5
//         .update_and_display_frame(&mut spi, &display.buffer())
//         .expect("display frame new graphics");
//     delay.delay_ms(2000u16);

//     println!("Now test new graphics with default rotation and some special stuff:");
//     display.set_rotation(DisplayRotation::Rotate0);
//     display.clear_buffer(Color::White);

//     // draw a analog clock
//     display.draw(
//         Circle::new(Coord::new(64, 64), 64)
//             .stroke(Some(Color::Black))
//             .into_iter(),
//     );
//     display.draw(
//         Line::new(Coord::new(64, 64), Coord::new(0, 64))
//             .stroke(Some(Color::Black))
//             .into_iter(),
//     );
//     display.draw(
//         Line::new(Coord::new(64, 64), Coord::new(80, 80))
//             .stroke(Some(Color::Black))
//             .into_iter(),
//     );

//     // draw white on black background
//     display.draw(
//         Font12x16::render_str("It's working-WoB!")
//             // Using Style here
//             .style(Style {
//                 fill_color: Some(Color::Black),
//                 stroke_color: Some(Color::White),
//                 stroke_width: 0u8, // Has no effect on fonts
//             })
//             .translate(Coord::new(175, 250))
//             .into_iter(),
//     );

//     // use bigger/different font
//     display.draw(
//         Font12x16::render_str("It's working-BoW!")
//             // Using Style here
//             .style(Style {
//                 fill_color: Some(Color::White),
//                 stroke_color: Some(Color::Black),
//                 stroke_width: 0u8, // Has no effect on fonts
//             })
//             .translate(Coord::new(50, 200))
//             .into_iter(),
//     );

//     epd7in5
//         .update_and_display_frame(&mut spi, &display.buffer())
//         .expect("display frame new graphics");
//     delay.delay_ms(2000u16);

//     display.set_rotation(DisplayRotation::Rotate0);
//     display.clear_buffer(Color::White);

//     // // a moving `Hello World!`
//     // let limit = 10;
//     // for i in 0..limit {
//     //     println!("Moving Hello World. Loop {} from {}", (i + 1), limit);

//     //     display.draw(
//     //         Font12x16::render_str("  Hello World! ")
//     //             .style(Style {
//     //                 fill_color: Some(Color::White),
//     //                 stroke_color: Some(Color::Black),
//     //                 stroke_width: 0u8, // Has no effect on fonts
//     //             })
//     //             .translate(Coord::new(5 + i * 12, 50))
//     //             .into_iter(),
//     //     );

//     //     epd7in5
//     //         .update_and_display_frame(&mut spi, &display.buffer())
//     //         .expect("display frame new graphics");
//     //     delay.delay_ms(500u16);
//     // }

//     println!("Single test");
//     display.set_rotation(DisplayRotation::Rotate0);
//     display.clear_buffer(Color::White);
//     display.draw(
//         ProFont24Point::render_str("What a blast!")
//             .style(Style {
//                 fill_color: Some(Color::White),
//                 stroke_color: Some(Color::Black),
//                 stroke_width: 0u8, // Has no effect on fonts
//             })
//             .translate(Coord::new(300, 200))
//             .into_iter(),
//     );
//     display.draw(
//         Font12x16::render_str("Yep, that's right!")
//             .style(Style {
//                 fill_color: Some(Color::White),
//                 stroke_color: Some(Color::Black),
//                 stroke_width: 0u8, // Has no effect on fonts
//             })
//             .translate(Coord::new(250, 250))
//             .into_iter(),
//     );

//     epd7in5
//         .update_and_display_frame(&mut spi, &display.buffer())
//         .expect("display frame new graphics");

//     println!("Finished tests - going to sleep");
//     // epd7in5.clear_frame(&mut spi).expect("clear frame");
//     epd7in5.sleep(&mut spi)
// }
