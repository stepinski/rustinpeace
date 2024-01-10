use bracket_lib::prelude::*;

// #https://www.youtube.com/watch?v=79GyLlXAk-0&list=PL6NyXXOC9Mzl54Au66dPnKO4C7Trd6B_W&index=28


struct Player{
    x:i32,
    y:i32,
    velocity:f32,
}

impl Player{
    fn new(x:i32,y:i32) -> Self{
        Player {
            x,
            y,
            velocity: 0.0

        }


    }

}


enum GameMode {
    Menu,
    Playing,
    End
}

struct State {
    mode: GameMode,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.main_menu(ctx), //self.dead(ctx),
            GameMode::Playing => self.main_menu(ctx),//self.play(ctx),
        }
    }
}

impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "welcome to floppy dragon");
        ctx.print_centered(8, "P play game");
        ctx.print_centered(9, "Q quit game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
      .with_title("Flappy Dragon")
      .build()?;
  
    main_loop(context, State::new())
  }