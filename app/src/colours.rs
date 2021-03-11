use chargrid::render::Rgb24;

pub const WALL_TOP: Rgb24 = Rgb24::new(0x49, 0x2E, 0x00);
pub const WALL_FRONT: Rgb24 = Rgb24::new(0xD0, 0x8C, 0x15);
pub const FLOOR_BACKGROUND: Rgb24 = Rgb24::new(0xD4, 0xB8, 0x88);
pub const FLOOR_FOREGROUND: Rgb24 = Rgb24::new(0xB0, 0x8C, 0x4C);
pub const SPACE_BACKGROUND: Rgb24 = Rgb24::new(0x00, 0x00, 0x38);
pub const SPACE_FOREGROUND: Rgb24 = Rgb24::new_grey(0xAA);
pub const SPACE_FOREGROUND_DIM: Rgb24 = Rgb24::new_grey(0x66);
pub const WINDOWS: Rgb24 = Rgb24::new(0xBE, 0xED, 0xFF);
pub const STRIPE: Rgb24 = Rgb24::new(0xFF, 0xBE, 0x4C);
pub const DOOR: Rgb24 = Rgb24::new(0x88, 0x88, 0x88);
pub const DOOR_BORDER: Rgb24 = Rgb24::new_grey(0x33);
pub const STAIRS_BACKGROUND: Rgb24 = Rgb24::new_grey(0x33);
pub const STAIRS_0: Rgb24 = Rgb24::new_grey(0xAA);
pub const STAIRS_1: Rgb24 = Rgb24::new_grey(0x88);
pub const STAIRS_2: Rgb24 = Rgb24::new_grey(0x66);
pub const PLAYER: Rgb24 = Rgb24::new_grey(0x00);
pub const ZOMBIE: Rgb24 = Rgb24::new(0x3F, 0x3E, 0x0B);
pub const BLOAT: Rgb24 = Rgb24::new(0x4F, 0x09, 0x55);
pub const TANK: Rgb24 = Rgb24::new(0x51, 0x0C, 0x03);
pub const SKELETON: Rgb24 = Rgb24::new(0x03, 0x51, 0x45);
pub const BOOMER: Rgb24 = Rgb24::new(0x17, 0x80, 0x14);
pub const BLOOD: Rgb24 = Rgb24::new(0xFF, 0x00, 0x22);
pub const FUEL_BAY_BACKGROUND: Rgb24 = Rgb24::new_grey(0x44);
pub const FUEL_BAY_FOREGROUND: Rgb24 = Rgb24::new(0, 0, 255);
pub const BULLET: Rgb24 = Rgb24::new_grey(0);
pub const CREDIT_FOREGROUND: Rgb24 = Rgb24::new(0, 127, 127);
pub const UPGRADE_FOREGROUND: Rgb24 = Rgb24::new(0, 187, 0);
pub const UPGRADE_BACKGROUND: Rgb24 = Rgb24::new(0, 0, 0);
pub const GUN_METAL: Rgb24 = Rgb24::new_grey(0x11);
pub const WOOD: Rgb24 = Rgb24::new(0xab, 0x40, 0x0a);
pub const PLASMA: Rgb24 = Rgb24::new(0x00, 0xFF, 0xFF);
pub const CHAINSAW: Rgb24 = Rgb24::new(0x7a, 0x6a, 0x00);
pub const LASER: Rgb24 = Rgb24::new(0, 255, 0);
pub const GAUS: Rgb24 = Rgb24::new(127, 0, 255);
pub const OXYGEN: Rgb24 = Rgb24::new(127, 127, 255);
pub const HEALTH: Rgb24 = Rgb24::new(255, 0, 0);
