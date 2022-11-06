#[derive(Debug)]
pub struct Weapon<'a> {
  pub accuracy: i32,
  pub ammo: i32,
  pub category: &'a str,
  pub damage: i32,
  pub fire_rate: i32,
  pub movement: i32,
  pub range: f32,
}

pub fn shoot() {
  println!("pew pew madafaka")
}