use crate::weapons::models::{ shoot, Weapon };

pub fn make() {
  let roto = Weapon {
    accuracy: 310,
    ammo: 100,
    category: "machinegun",
    damage: 595,
    fire_rate: 660,
    movement: 73,
    range: 18.2,
  };

  shoot();
  print!("{:?}", roto)
}