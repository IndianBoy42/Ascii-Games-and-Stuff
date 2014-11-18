extern crate tcod;

use self::tcod::{Console, background_flag, Color, TextAlignment, Key};

use data::{Object, Entity, Bounds, Point, GameWorld};

trait GameLoop<'a> {
	fn get_player(&mut self) -> Entity;
	fn get_entities(&mut self) -> Vec<&'a Entity>;

	fn get_world(&mut self)	-> GameWorld;

	fn get_window(&mut self) -> ConsoleWindow;

	fn get_last_keypress(&mut self) -> Key;
	fn update(&mut self);
	fn render(&'a mut self) {
		self.get_window().clear();

		let window = &mut self.get_window();
		let renderer = &mut window.get_renderer();

		renderer.render_object(self.get_player().get_object());
		
		for e in self.get_entities().iter() {
			renderer.render_object(e.get_object());
		}
	}
}

trait ConsoleWindow {
	fn new(bounds: Bounds) -> Self;

	fn get_bounds(&self) -> Bounds;
	fn get_bg_color(&self) -> Color;
	fn get_renderer(&mut self) -> GameRenderer;

	fn clear(&mut self) {
		self.get_renderer().get_console().set_default_background(self.get_bg_color());
		self.get_renderer().get_console().clear();
	}
}

trait GameRenderer {
	fn get_console(&mut self) -> Console;

	fn render_object(&mut self, c: Object) {
		self.get_console().put_char(c.pos.x as int, c.pos.y as int, c.char, background_flag::Set)
	}

	fn print_msg(&mut self, xy: Point, alignment: TextAlignment, text: &str) {
		self.get_console().print_ex(xy.x as int, xy.y as int, background_flag::Set, alignment, text);
	}
}