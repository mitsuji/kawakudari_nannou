use nannou::prelude::*;
use std::time::Duration;
use nannou::rand::random_range;
use kawakudari_nannou::Std15;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    std15: Std15,
    tick: i32,
    x: i32,
    running: bool,
}

fn model(app: & App) -> Model {
    let interval = Duration::from_millis(16);
    app.set_loop_mode(nannou::app::LoopMode::Rate{update_interval:interval});
    app.new_window().size(512+10+10,384+10+10).event(event).view(view).build().unwrap();
    Model {
        std15 : Std15::new(512,384,32,24),
        tick : 0,
        x: 15,
        running: true,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.running {
        if model.tick % 3 == 0 {
            let std15 = &mut model.std15;
            std15.locate(model.x,5);
            std15.putc('0');
            std15.locate(random_range::<i32>(0,32),23);
            std15.putc('*');
            std15.scroll();
	    if std15.scr(model.x,5) != '\0' {
	      model.running = false;
	    }
        }
    }
    model.tick +=1;
}

fn event(_app: &App, model: &mut Model, event: WindowEvent) {
  match event {
    KeyPressed(Key::Left)  => model.x -= 1,
    KeyPressed(Key::Right) => model.x += 1,
    _ => ()
  }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw:Draw = app.draw();
    _model.std15.papplet_draw(app,&draw);
    draw.to_frame(app, &frame).unwrap();
}

