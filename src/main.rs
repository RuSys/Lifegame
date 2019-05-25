mod lifegame;

extern crate piston_window;

use piston_window::*;
use rand::prelude::*;
use lifegame::*;

const X: usize = 640;
const Y: usize = 400;

const LGX: usize = X;
const LGY: usize = Y;

fn main() {
    let mut n = Lifegame::new(LGX, LGY);
    /*
    n.set(5,10);
    n.set(5,11);
    n.set(5,12);
    */


    for x in 0..LGX {
        for y in 0..LGY {
            if random() {
                n.set(x, y);
            }
        }
    }

    let mut window: PistonWindow =
        WindowSettings::new("Lifegame using Piston", [X as u32, Y as u32])
        .exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {

            clear([0.0; 4], graphics);

            for y in 0..LGY {
                for x in 0..LGX {
                    if n.get(x, y) {
                        rectangle([1.0, 1.0, 1.0, 1.0], // white
                                [x as f64, y as f64, 1.0, 1.0],
                                context.transform,
                                graphics);
                    }
                }
            }
            n.nextgen();
        });
    }
}
