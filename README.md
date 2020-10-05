# kawakudari-nannou

This project implements part of the [std15.h](https://github.com/IchigoJam/c4ij/blob/master/src/std15.h) API (from [c4ij](https://github.com/IchigoJam/c4ij)) with [nannou](https://nannou.cc), and [Kawakudari Game](https://ichigojam.github.io/print/en/KAWAKUDARI.html) on top of it.

It will allow programming for [IchigoJam](https://ichigojam.net/index-en.html)-like targets that display [IchigoJam FONT](https://mitsuji.github.io/ichigojam-font.json/) on screen using a Rust programming language.
```
fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.running {
        if model.frame % 3 == 0 {
            let std15 = &mut model.std15;
            std15.locate(model.x,5);
            std15.putc('0');
            std15.locate(random_range::<i32>(0,32),23);
            std15.putc('*');
            std15.scroll(Direction::Up);
            if std15.scr(model.x,5) != '\0' {
              std15.locate(0,23);
              std15.putstr("Game Over...");
              std15.putnum(model.frame as i32);
              model.running = false;
            }
        }
    }
    model.frame +=1;
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
  match event {
    KeyPressed(Key::Left)  => model.x -= 1,
    KeyPressed(Key::Right) => model.x += 1,
    _ => ()
  }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    _model.std15.draw_screen(app,&frame);
}
```

## Prerequisite

* Video card and driver that supports [Vulkan](https://www.khronos.org/vulkan/) installed.
* This project using programming language Rust, so you need [rustup](https://rustup.rs) build tool properly installd to run example code.


## How to use

To just run example
```
$ cargo run
```

To build executeble and run example
```
$ cargo build
$ target/debug/kawakudari_nannou
```


## License
[![Creative Commons License](https://i.creativecommons.org/l/by/4.0/88x31.png)](http://creativecommons.org/licenses/by/4.0/)
[CC BY](https://creativecommons.org/licenses/by/4.0/) [mitsuji.org](https://mitsuji.org)

This work is licensed under a [Creative Commons Attribution 4.0 International License](http://creativecommons.org/licenses/by/4.0/).
