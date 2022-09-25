use std::collections::HashMap;
use std::io::Write;

use std::fs::read_to_string;

use crate::prelude::*;

pub const MAXROW: u8 = 24;
pub const MAXCOL: u8 = 80;

pub const QUADRANT_WIDTH: u32 = 8;
pub const QUADRANT_HEIGHT: u32 = 8;

const SRS_1: &str = "------------------------";

const TILESTR: [&str; 5] = ["   ", " * ", ">!<", "+K+", "<*>"];

const LRS_1: &str = "-------------------";

const DCR_1: &str = "Damage Control report:";

const GR_1: &str = "   ----- ----- ----- ----- ----- ----- ----- -----";

const GM_1: &str = "  ----- ----- ----- ----- ----- ----- ----- -----";
const DIST_1: &str = "  DISTANCE = %s\n\n";
const STR_S: &str = "s";

const DEVICE_NAME: [&str; 8] = [
    "Warp engines",
    "Short range sensors",
    "Long range sensors",
    "Phaser control",
    "Photon tubes",
    "Damage control",
    "Shield control",
    "Library computer",
];

const QUADRANT_NAME: [&str; 16] = [
    "Antares",
    "Rigel",
    "Procyon",
    "Vega",
    "Canopus",
    "Altair",
    "Sagittarius",
    "Pollux",
    "Sirius",
    "Deneb",
    "Capella",
    "Betelgeuse",
    "Aldebaran",
    "Regulus",
    "Arcturus",
    "Spica",
];

const SECTOR_NAME: [&str; 8] = [" I", " II", " III", " IV", " V", " VI", " VII", " VIII"];

pub const STARTING_ENERGY: u32 = 3000; /* Starting Energy */
// energy0
pub const DEFAULT_PHOTON_TORPEDO_CAPACITY: u32 = 10; /* Photon Torpedo capacity */
//torps0

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

pub trait HasPosition {
    fn get_position(&self) -> &Vec2;

    fn get_mut_position(&mut self) -> &mut Vec2;
}

impl HasPosition for Vec2 {
    fn get_position(&self) -> &Vec2 {
        self
    }

    fn get_mut_position(&mut self) -> &mut Vec2 {
        self
    }
}

impl SpaceCoordinates for Vec2 {}

pub trait Moveable<T: HasPosition = Self>: HasPosition {
    fn move_to(&mut self, position: &Vec2) {
        let p = self.get_mut_position();
        p.x = position.x;
        p.y = position.y;
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Stardate {
    pub year: i32,
    pub date: i32,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum SectorType {
    #[default]
    SPACE,
    STAR,
    BASE,
    KLINGON,
    SHIP = 4,
}

pub trait SpaceCoordinates<T: HasPosition = Self>: HasPosition {
    fn get_quadrant_position(&self) -> Vec2 {
        let p = self.get_position();
        Vec2 {
            x: p.x / 8,
            y: p.y / 8,
        }
    }

    fn get_sector_position(&self) -> Vec2 {
        let p = self.get_position();
        Vec2 {
            x: p.x % 8,
            y: p.y % 8,
        }
    }

    fn is_outside(&self) -> bool {
        let p = self.get_position();
        p.y < 0 || p.y > 63 || p.x < 0 || p.x > 63
    }

    /* Return the distance to an object in x.xx fixed point */
    fn distance_to(&self, hp: &Box<dyn HasPosition>) -> f32 {
        let p1 = self.get_position();
        let p2 = hp.get_position();
        /* We do the squares in fixed point maths */

        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        let dsx = dx * dx;
        let dsy = dy * dy;

        let d = ((dsx + dsy) as f32).sqrt();

        return d;
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Klingon {
    pub position: Vec2,
    pub energy: u32,
    pub destroyed: bool,
}

impl HasPosition for Klingon {
    fn get_position(&self) -> &Vec2 {
        &self.position
    }

    fn get_mut_position(&mut self) -> &mut Vec2 {
        &mut self.position
    }
}

impl Moveable for Klingon {}

impl SpaceCoordinates for Klingon {}

#[derive(Debug, Default, Copy, Clone)]
pub struct Starbase {
    pub position: Vec2,
    pub destroyed: bool,
}

impl HasPosition for Starbase {
    fn get_position(&self) -> &Vec2 {
        &self.position
    }

    fn get_mut_position(&mut self) -> &mut Vec2 {
        &mut self.position
    }
}

impl SpaceCoordinates for Starbase {}

#[derive(Debug, Default, Clone)]
pub struct Ship {
    pub position: Vec2,
    pub docked: bool,                         /* Docked flag */
    pub torps: u32,                           /* Photon Torpedoes left */
    pub shield: u32,                          /* Current shield value */
    pub energy: u32,                          /* Current Energy */
    pub devices: HashMap<DeviceType, Device>, //[Device; 8], /* Damage Array */
    pub destroyed: bool,
}

impl Ship {
    pub fn get_total_energy(&self) -> u32 {
        self.shield + self.energy
    }

    pub fn get_device(&mut self, device_type: DeviceType) -> &Device {
        self.devices.get(&device_type).unwrap()
    }

    pub fn get_mut_device(&mut self, device_type: DeviceType) -> &mut Device {
        self.devices.get_mut(&device_type).unwrap()
    }

    pub fn is_unable_to_navigate(&self) -> bool {
        return self.get_total_energy() <= 10
            && (self.energy < 10
                || self
                    .devices
                    .get(&DeviceType::ShieldControl)
                    .unwrap()
                    .is_damaged());
    }
}

impl HasPosition for Ship {
    fn get_position(&self) -> &Vec2 {
        &self.position
    }

    fn get_mut_position(&mut self) -> &mut Vec2 {
        &mut self.position
    }
}

impl Moveable for Ship {}
impl SpaceCoordinates for Ship {}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DeviceType {
    #[default]
    WarpEngines,
    ShortRangeSensors,
    LongRangeSensors,
    PhaserControl,
    PhotonTubes,
    DamageControl,
    ShieldControl,
    LibraryComputer,
    DeviceNum,
}

impl DeviceType {
    pub fn from_u8(v: u8) -> DeviceType {
        match v {
            0 => DeviceType::WarpEngines,
            1 => DeviceType::ShortRangeSensors,
            2 => DeviceType::LongRangeSensors,
            3 => DeviceType::PhaserControl,
            4 => DeviceType::PhotonTubes,
            5 => DeviceType::DamageControl,
            6 => DeviceType::ShieldControl,
            7 => DeviceType::LibraryComputer,
            _ => panic!("Unknown value: {}", v),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Device {
    pub damage: u32,
    pub name: &'static str,
    pub device_type: DeviceType,
}

impl Device {
    pub fn repair_all_damage(&mut self) {
        self.damage = 0;
    }

    pub fn set_damage(&mut self, damage: u32) {
        self.damage = damage;
    }

    pub fn repair_damage(&mut self, damage: u32) {
        self.damage -= damage;
    }

    pub fn add_damage(&mut self, damage: u32) {
        self.damage += damage;
    }

    pub fn is_damaged(&self) -> bool {
        self.damage > 0
    }

    pub fn is_inoperable(&self) -> bool {
        if self.is_damaged() {
            println!(
                "{} {} inoperable.",
                self.name,
                if self.device_type == DeviceType::PhotonTubes {
                    "are"
                } else {
                    "is"
                }
            );

            return true;
        }

        false
    }
}

#[derive(Debug)]
pub struct Quadrant {
    pub position: Vec2,
    pub stars: u32,
    pub klingons: u32,
    pub starbases: u32,
    pub visited: bool,
    pub name: String,
}

impl Default for Quadrant {
    fn default() -> Self {
        Self {
            position: Vec2::default(),
            stars: 0,
            klingons: 0,
            starbases: 0,
            name: "".to_string(),
            visited: false,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Sector {
    pub position: Vec2,
    pub sector_type: SectorType,
}

pub struct Game {
    pub exit_flag: bool,
    pub starbases: Vec<Starbase>,
    pub klingons: Vec<Klingon>,
    pub ship: Ship,

    /* Starbase Location in sector */
    pub starbases_left: u32,  /* Total Starbases left */
    pub total_starbases: u32, /* Total Starbases at start */

    pub klingons_left: u32,  /* Total Klingons left */
    pub total_klingons: u32, /* Klingons at start */

    pub time_days: i32,  /* Starting Stardate */
    pub time_start: f32, /* Starting Stardate */
    pub time_up: f32,    /* End of time */
    pub stardate: f32,   /* Current Stardate */
    pub d4: i32,
    pub quadrant_map: [[Quadrant; 8]; 8], /* Galaxy. BCD of k b s plus flag */
    pub sector_map: [[Sector; 64]; 64],
}

impl Game {
    pub fn get_klingons_idxs_in_current_quadrant(&mut self) -> Vec<usize> {
        let ship_quadrant_position = self.ship.get_quadrant_position();

        self.klingons
            .iter()
            .enumerate()
            .filter(|(_, k)| !k.destroyed)
            .filter(|(_, k)| k.get_quadrant_position() == ship_quadrant_position)
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>()
    }

    pub fn no_klingons_in_current_quadrant(&self) -> bool {
        let current_quadrant = self.get_current_quadrant();

        if current_quadrant.klingons == 0 {
            println!(
                "Science Officer Spock reports:
            'Sensors show no enemy ships in this quadrant'"
            );

            return true;
        }

        false
    }

    pub fn get_sector(&self, position: &Vec2) -> &Sector {
        &self.sector_map[position.x as usize][position.y as usize]
    }

    pub fn get_mut_sector(&mut self, position: &Vec2) -> &mut Sector {
        &mut self.sector_map[position.x as usize][position.y as usize]
    }

    pub fn get_current_quadrant(&self) -> &Quadrant {
        let ship_quadrant_position = self.ship.get_quadrant_position();

        &self.quadrant_map[ship_quadrant_position.x as usize][ship_quadrant_position.y as usize]
    }

    pub fn get_mut_current_quadrant(&mut self) -> &mut Quadrant {
        let current_quadrant_position = self.ship.get_quadrant_position();

        &mut self.quadrant_map[current_quadrant_position.x as usize]
            [current_quadrant_position.y as usize]
    }

    pub fn initialize() -> Game {
        /* Seed the randomizer with the timer */
        // srand((unsigned) time(NULL));

        /* Max of 4000, which works nicely with our 0.1 fixed point giving
        us a 16bit unsigned range of time */
        let stardate = (get_randf32() * 2000.0) + 2000.0;

        let mut sector_map = [[Sector::default(); 64]; 64];
        let mut quadrant_map: [[Quadrant; 8]; 8] = [
            [
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
            ],
            [
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
            ],
            [
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
            ],
            [
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
            ],
            [
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
            ],
            [
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
            ],
            [
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
            ],
            [
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
                Quadrant::default(),
            ],
        ];

        /* Initialize time */
        let mut time_days = 25i32 + (get_randf32() * 10.0) as i32;

        /* Initialize Enterprise */
        let ship_position = find_empty_place_in_quadrant(
            &sector_map,
            &Vec2 {
                x: get_rand8(),
                y: get_rand8(),
            },
        );

        sector_map[ship_position.x as usize][ship_position.y as usize].sector_type =
            SectorType::SHIP;

        let energy = STARTING_ENERGY;
        let torps = DEFAULT_PHOTON_TORPEDO_CAPACITY;
        let shield = 0;

        let mut devices = HashMap::new(); //[Device::default(); DeviceType::DeviceNum as usize]; // (1..=8)

        for i in 0..DeviceType::DeviceNum as usize {
            let device_type = DeviceType::from_u8(i as u8);
            let device = Device {
                damage: 0,
                device_type: DeviceType::from_u8(i as u8),
                name: &DEVICE_NAME[i as usize],
            };

            devices.insert(device_type, device);
        }

        /* Setup What Exists in Galaxy */
        let mut klingons_in_quadrant: u32;
        let mut klingons_left: u32 = 0;
        let mut total_klingons: u32 = 0;

        let mut starbases_in_quadrant: u32;
        let mut starbases_left: u32 = 0;
        let mut total_starbases: u32 = 0;

        let mut klingons: Vec<Klingon> = Vec::new();
        let mut starbases: Vec<Starbase> = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                let quadrant_position = Vec2 { x: i, y: j };
                let r: u8 = get_rand(100i32) as u8;

                klingons_in_quadrant = 0;

                if r > 98 {
                    klingons_in_quadrant = 3;
                } else if r > 95 {
                    klingons_in_quadrant = 2;
                } else if r > 80 {
                    klingons_in_quadrant = 1
                }

                total_klingons = total_klingons + klingons_in_quadrant;

                for _ in 0..klingons_in_quadrant {
                    let klingon_position =
                        find_empty_place_in_quadrant(&sector_map, &quadrant_position);

                    sector_map[klingon_position.x as usize][klingon_position.y as usize]
                        .sector_type = SectorType::KLINGON;

                    let klingon = Klingon {
                        position: klingon_position,
                        energy: STARTING_ENERGY,
                        destroyed: false,
                    };

                    klingons.push(klingon);
                }

                starbases_in_quadrant = 0;

                let r: u8 = get_rand(100i32) as u8;

                if r > 96 {
                    starbases_in_quadrant = 1;
                }

                starbases_left = starbases_left + starbases_in_quadrant;

                for _ in 0..starbases_in_quadrant {
                    let starbase_position =
                        find_empty_place_in_quadrant(&sector_map, &quadrant_position);

                    sector_map[starbase_position.x as usize][starbase_position.y as usize]
                        .sector_type = SectorType::BASE;

                    let starbase = Starbase {
                        position: starbase_position,
                        destroyed: false,
                    };

                    starbases.push(starbase);
                }

                let stars_in_quadrant: u32 = rand8() as u32;

                for _k in 0..stars_in_quadrant {
                    let star_position =
                        find_empty_place_in_quadrant(&sector_map, &quadrant_position);

                    sector_map[star_position.x as usize][star_position.y as usize].sector_type =
                        SectorType::STAR;
                }

                let quadrant_name = get_quadrant_name(&quadrant_position, true);
                quadrant_map[i as usize][j as usize] = Quadrant {
                    visited: false,
                    klingons: klingons_in_quadrant,
                    starbases: starbases_in_quadrant,
                    stars: stars_in_quadrant,
                    position: quadrant_position,
                    name: quadrant_name,
                }
            }
        }

        /* Give more time for more Klingons */
        if klingons_left as i32 > time_days {
            time_days = klingons_left as i32 + 1;
        }

        if total_starbases == 0 {
            let starbase_quadrant_position = Vec2 {
                x: get_rand8(),
                y: get_rand8(),
            };

            let starbase_sector_position =
                find_empty_place_in_quadrant(&sector_map, &starbase_quadrant_position);

            sector_map[starbase_sector_position.x as usize][starbase_sector_position.y as usize]
                .sector_type = SectorType::BASE;

            total_starbases = 1;
        }

        starbases_left = total_starbases;
        klingons_left = total_klingons;

        let ship = Ship {
            position: ship_position,

            torps,
            energy,
            devices,
            shield,

            docked: false,
            destroyed: false,
        };

        let time_start = stardate;
        let time_up = time_start + time_days as f32;

        Game {
            d4: 0,
            exit_flag: false,

            starbases,       /* Starbase Location in sector */
            starbases_left,  /* Total Starbases left */
            total_starbases, /* Total Starbases at start */

            quadrant_map, /* Galaxy. BCD of k b s plus flag */

            klingons,       /* Klingons at start */
            klingons_left,  /* Total Klingons left */
            total_klingons, /* Klingons at start */
            ship,

            time_days,  /* days Stardate */
            time_start, /* Starting Stardate */
            time_up,    /* End of time */
            stardate,   /* Current Stardate */

            sector_map,
        }
    }
}

pub fn get_course(officer: &str) -> Option<f32> {
    //print!("Course (0-9): "); // 0? -> does not move
    //let mut course = input_f32();

    let mut course = input_f32("Course (0-9): ", 0.0, 1000.0);

    if course == 9.0 {
        course = 1.0;
    }

    if course < 1.0 || course >= 9.0 {
        println!("{officer} {INC_1}");
        return None;
    }

    Some(course)
}

pub fn get_new_position(c1: f32) -> Vec2 {
    let x = match c1 as i32 {
        1 => 1,
        2 => 1,
        3 => 0,
        4 => -1,
        5 => -1,
        6 => -1,
        7 => 0,
        8 => 1,
        _ => 0,
    };

    let y = match c1 as i32 {
        1 => 0,
        2 => 1,
        3 => 1,
        4 => 1,
        5 => 0,
        6 => -1,
        7 => -1,
        8 => -1,
        _ => 0,
    };

    Vec2 {
        x: x as i32,
        y: y as i32,
    }
}

pub fn klingons_move(game: &mut Game) {
    //println!("klingons_move");
    let ship_quadrant_position = game.ship.get_quadrant_position();

    let alive_local_klingons = game.get_klingons_idxs_in_current_quadrant();

    for idx in alive_local_klingons {
        let k = game.klingons.get_mut(idx).unwrap();

        let old_klingon_position = k.get_position().clone();

        game.sector_map[old_klingon_position.x as usize][old_klingon_position.y as usize]
            .sector_type = SectorType::SPACE;

        let new_klingon_position =
            find_empty_place_in_quadrant(&game.sector_map, &ship_quadrant_position);

        k.move_to(&new_klingon_position);

        game.sector_map[k.position.x as usize][k.position.y as usize].sector_type =
            SectorType::KLINGON;
    }

    klingons_shoot(game);
}

pub fn klingons_shoot(game: &mut Game) {
    println!("klingons_shoot");
    let ship_quadrant_position = game.ship.get_quadrant_position();

    if game.quadrant_map[ship_quadrant_position.x as usize][ship_quadrant_position.y as usize]
        .klingons
        <= 0
    {
        return;
    }

    if game.ship.docked {
        println!("Starbase shields protect the Enterprise");
        return;
    }

    let mut ship_is_destroyed = false;

    let alive_local_klingons = game.get_klingons_idxs_in_current_quadrant();

    for idx in alive_local_klingons {
        let k = game.klingons.get_mut(idx).unwrap();

        let klingon_sector_position = k.get_sector_position();

        // energy  + 200-300
        let mut h = k.energy * (200 + get_rand(100)) as u32;

        let hp: Box<dyn HasPosition> = Box::new(*k);
        let d = game.ship.distance_to(&hp);

        println!("distance: {}", d);
        h = (h as f32 / d) as u32;

        h /= 1000;

        if h > game.ship.shield {
            ship_is_destroyed = true;
        }

        game.ship.shield = game.ship.shield - h;

        k.energy = (k.energy * 100) / (300 + get_rand(100)) as u32;

        println!(
            "{} unit hit on Enterprise from sector {}, {}",
            h, klingon_sector_position.x, klingon_sector_position.y
        );

        if ship_is_destroyed {
            break;
        }

        println!("    <Shields down to {} units>\n", game.ship.shield);

        if h >= 20 {
            /* The check in basic is float and is h/s >.02. We
            have to use 32bit values here to avoid an overflow
            FIXME: use a better algorithm perhaps ? */

            let ratio = h / game.ship.shield;

            if get_rand(10) <= 6 || ratio > 2 {
                let r = rand8() as u8;

                let device = game.ship.get_mut_device(DeviceType::from_u8(r));
                /* The original basic code computed h/s in
                float form the C conversion broke this. We correct it in the fixed
                point change */
                device.add_damage((ratio + get_rand(50) as u32) as u32);

                /* FIXME: can we use dcr_1 here ?? */
                println!(
                    "Damage Control reports\n   '{}' damaged by hit\n",
                    device.name
                );
            }
        }
    }

    if ship_is_destroyed {
        println!("");
        ship_destroyed(game);
    }
}

pub fn repair_damage(game: &mut Game, warp: f32) {
    let mut d1: i32 = 0;

    let mut repair_factor: u32 = (warp * 100.0) as u32;

    if warp >= 100.0 {
        repair_factor = 10;
    }

    for i in 0..DeviceType::DeviceNum as u8 {
        let device = game.ship.get_mut_device(DeviceType::from_u8(i));

        if device.is_damaged() {
            device.repair_damage(repair_factor);

            let damage = device.damage;

            if damage < 10 && damage > 0 {
                /* -0.1 */
                device.set_damage(10);
            } else if !device.is_damaged() {
                if d1 != 1 {
                    d1 = 1;
                    print!("{}", DCR_1);
                }
                println!("    {} repair completed\n", device.name);
            }
        }
    }

    if get_rand(10) <= 2 {
        let r = rand8() as u8;
        let device = game.ship.get_mut_device(DeviceType::from_u8(r));

        if get_rand(10) < 6 {
            /* Working in 1/100ths */
            device.add_damage((get_rand(500) + 100) as u32);
            print!("{}", DCR_1);
            println!("    {} damaged\n", device.name);
        } else {
            /* Working in 1/100ths */
            device.repair_damage((get_rand(300) + 100) as u32);
            print!("{}", DCR_1);
            println!("    {} state of repair improved\n", device.name);
        }
    }
}

pub fn find_empty_place_in_quadrant(
    sector_map: &[[Sector; 64]; 64],
    quadrant_position: &Vec2,
) -> Vec2 {
    let mut x: i32;
    let mut y: i32;

    loop {
        x = (quadrant_position.x * 8 + get_rand8()) as i32;
        y = (quadrant_position.y * 8 + get_rand8()) as i32;

        if !(sector_map[x as usize][y as usize].sector_type != SectorType::SPACE) {
            break;
        }
    }

    Vec2 { x, y }
}

pub fn get_quadrant_name(p: &Vec2, small: bool) -> String {
    let x = p.x as usize;
    let y = p.y as usize;

    if p.is_outside() {
        return "Unknown".to_string();
    }

    let quadname = if x <= 4 {
        QUADRANT_NAME[y]
    } else {
        QUADRANT_NAME[y + 8]
    };

    let sectorname = if small { SECTOR_NAME[x] } else { "" };

    format!("{}{}", quadname, sectorname)
}

pub fn ship_destroyed(game: &mut Game) {
    println!(
        "The Enterprise has been destroyed.
    The Federation will be conquered.\n"
    );

    end_of_time(game);
}

pub fn end_of_time(game: &mut Game) {
    println!("It is stardate {:.2}.\n", game.stardate);

    resign_commision(game);
}

pub fn resign_commision(game: &mut Game) {
    println!(
        "There were {} Klingon Battlecruisers left at the
 end of your mission.\n",
        game.klingons_left
    );

    end_of_game(game);
}

pub fn won_game(game: &mut Game) {
    println!(
        "Congratulations, Captain!  The last Klingon Battle Cruiser
 menacing the Federation has been destroyed."
    );
    /*
    if game.stardate - game.time_start > 0.0 {
        let n = game.total_klingons as f32;
        let d = game.stardate - game.time_start;
        let v = square00(n as i32 / d as i32);

        println!("Your efficiency rating is {}\n", print100(v));
        // 1000 * pow(total_klingons / (float)(FROM_FIXED(t) - time_start), 2));
    }
    */
    end_of_game(game);
}

pub fn end_of_game(game: &mut Game) {
    game.exit_flag = true;

    if game.starbases_left > 0 {
        /* FIXME: showfile ? */
        println!(
            "The Federation is in need of a new starship commander
 for a similar mission. "
        );

        let x = get_command("If there is a volunteer, let him step forward and enter 'aye'");

        game.exit_flag = x != "aye";
    }
}

const INC_1: &str = "reports:\n  Incorrect course data, sir!";

pub fn course_control(game: &mut Game) {
    let mut warpmax: f32 = 8.0;

    let c1 = get_course("Lt. Sulu");

    if c1 == None {
        return;
    }

    let c1 = c1.unwrap();

    if game.ship.get_device(DeviceType::WarpEngines).is_damaged() {
        warpmax = 0.2;
    }

    //println!("Warp Factor (0-{}): ", warpmax);

    let warp = input_f32(
        format!("Warp Factor (0-{}): ", warpmax).as_str(),
        0.0,
        warpmax,
    );
    //println!("warp: {:.2}", warp);

    if game.ship.get_device(DeviceType::WarpEngines).is_damaged() && warp > 0.2 {
        println!(
            "Warp Engines are damaged.
Maximum speed = Warp 0.2\n"
        );
        return;
    }

    if warp <= 0.0 {
        return;
    }

    if warp > 8.0 {
        println!(
            "Chief Engineer Scott reports:\n
  The engines won't take warp {:.2}!",
            warp
        );
        return;
    }

    //println!("Warp: {warp}");
    //println!("Warp: {warp:.2}");

    //println!("Energy needed: {:.2}", warp * 8.0);
    //n = (warp * 8.0) as i32;
    //println!("Energy needed {n}");
    //n = (n + 50) / 100; // energy needed
    //println!("Energy needed {n}");

    let n = (((warp * 800.0) + 50.0) / 100.0) as u32; // rounded up energy needed

    println!("Energy needed {n}");

    /* FIXME: should be  s + e - n > 0 iff shield control undamaged */
    if game.ship.energy < n {
        println!(
            "Engineering reports:
  Insufficient energy available for maneuvering
 at warp {:.2}!\n",
            warp
        );

        if game.ship.shield >= n && !game.ship.get_device(DeviceType::ShieldControl).is_damaged() {
            println!(
                "Deflector Control Room acknowledges:
  {} units of energy presently deployed to shields.",
                game.ship.shield
            );
        }

        return;
    }

    klingons_move(game);

    repair_damage(game, warp);

    let c1_position = get_new_position(c1);

    // depends on c1

    println!(
        "c1 {c1}, c1_position_: {}, {}",
        c1_position.x, c1_position.y
    );

    // we advance n times: 0-64
    for i in 0..n {
        println!("i: {}", i);
        let mut can_move = true;

        let mut ship_new_position = game.ship.position.clone();

        ship_new_position.x += c1_position.x;
        ship_new_position.y += c1_position.y;

        if ship_new_position.is_outside() {
            let ship_sector_position = ship_new_position.get_sector_position();
            let ship_quadrant_position = ship_new_position.get_quadrant_position();

            /* Mostly showfile ? FIXME */
            println!(
                "LT. Uhura reports:
  Message from Starfleet Command:

  Permission to attempt crossing of galactic perimeter
  is hereby *denied*. Shut down your engines.

Chief Engineer Scott reports:
  Warp Engines shut down at sector {}, {} of quadrant {}, {}.\n",
                ship_sector_position.x,
                ship_sector_position.y,
                ship_quadrant_position.x,
                ship_quadrant_position.y
            );

            can_move = false;
        } else if game.sector_map[ship_new_position.x as usize][ship_new_position.x as usize]
            .sector_type
            != SectorType::SPACE
        {
            let ship_sector_position = ship_new_position.get_sector_position();

            println!(
                "Warp Engines shut down at sector
{}, {} due to bad navigation.\n",
                ship_sector_position.x, ship_sector_position.y
            );

            can_move = false;
        }

        maneuver_energy(game, n);

        if game.stardate > game.time_up {
            end_of_time(game);
        }

        if can_move {
            let ship_old_quadrant_position = game.ship.get_quadrant_position();
            let ship_new_quadrant_position = ship_new_position.get_quadrant_position();
            game.sector_map[game.ship.position.x as usize][game.ship.position.y as usize]
                .sector_type = SectorType::SPACE;
            game.ship.position = ship_new_position;
            game.sector_map[game.ship.position.x as usize][game.ship.position.y as usize]
                .sector_type = SectorType::SHIP;

            game.stardate += 0.1;

            if ship_new_quadrant_position != ship_old_quadrant_position {
                new_quadrant(game);
            }
        } else {
            break;
        }
    }

    complete_maneuver(game, warp as i32, n as u32);
}

pub fn complete_maneuver(game: &mut Game, warp: i32, n: u32) {
    let mut time_used: i32;

    maneuver_energy(game, n);

    time_used = 10;

    if warp < 100 {
        time_used = (warp / 100) * 10;
    }

    game.stardate += time_used as f32;

    if game.stardate > game.time_up {
        return end_of_time(game);
    }

    short_range_scan(game);
}

pub fn maneuver_energy(game: &mut Game, n: u32) {
    let energy_needed = n + 10;
    let enough_energy = game.ship.energy >= energy_needed;

    if enough_energy {
        game.ship.energy -= energy_needed;
        return;
    }

    /*
    /* FIXME:
    This never occurs with the nav code as is - ancient trek versions
    included shield power in movement allowance if shield control
    was undamaged */
    println!("Shield Control supplies energy to complete maneuver.");

    game.ship.shield += game.ship.energy as u32;
    game.ship.energy = 0;

    if game.ship.shield <= 0 {
        game.ship.shield = 0;
    }

    */
}

pub fn short_range_scan(game: &mut Game) {
    let mut s_c;

    let ship_sector_position = game.ship.get_sector_position();
    let ship_quadrant_position = game.ship.get_quadrant_position();
    println!("SHIP {:?}", &game.ship.position);
    println!("QUAD {:?}", &ship_quadrant_position);
    println!("SECT {:?}", &ship_sector_position);

    s_c = "GREEN";

    if game.ship.energy < STARTING_ENERGY / 10 {
        s_c = "YELLOW";
    }

    let current_quadrant = game.get_current_quadrant();

    if current_quadrant.klingons > 0 {
        s_c = "*RED*";
    }

    drop(current_quadrant);

    for i in (ship_sector_position.y - 1)..=(ship_sector_position.y + 1) {
        for j in (ship_sector_position.x - 1)..=(ship_sector_position.x + 1) {
            if i >= 1 && i <= 8 && j >= 1 && j <= 8 {
                if game.sector_map[i as usize][j as usize].sector_type == SectorType::BASE {
                    game.ship.docked = true;
                    s_c = "DOCKED";
                    game.ship.energy = STARTING_ENERGY;
                    game.ship.torps = DEFAULT_PHOTON_TORPEDO_CAPACITY;
                    print!("Shields dropped for docking purposes.");
                    game.ship.shield = 0;
                }
            }
        }
    }

    if game
        .ship
        .get_device(DeviceType::ShortRangeSensors)
        .is_damaged()
    {
        print!("\n*** Short Range Sensors are out ***");
        return;
    }

    println!("{SRS_1}");

    for i in 0..8 {
        for j in 0..8 {
            let x = (ship_quadrant_position.x * 8) + i;
            let y = (ship_quadrant_position.y * 8) + j;
            let v = game.sector_map[x as usize][y as usize].sector_type;
            print!("{}", TILESTR[v as usize]);
        }

        match i {
            // Match a single value
            0 => println!("    Stardate            {:.2}", game.stardate),
            // Match several values
            1 => println!("    Condition           {s_c}"),
            2 => println!(
                "    Quadrant            {}, {}",
                ship_quadrant_position.x + 1,
                ship_quadrant_position.y + 1
            ),
            3 => println!(
                "    Sector              {}, {}",
                ship_sector_position.x + 1,
                ship_sector_position.y + 1
            ),
            4 => println!("    Photon Torpedoes    {}", game.ship.torps),
            5 => println!("    Total Energy        {}", game.ship.get_total_energy()),
            6 => println!("    Shields             {}", game.ship.shield),
            7 => println!("    Klingons Remaining  {}", game.klingons_left),
            _ => println!("Ain't special"),
        }
    }
    println!("{SRS_1}");
}

pub fn put1bcd(v: u32) {
    print!("{}", v & 0x0F);
}

pub fn putbcd(q: &Quadrant) {
    put1bcd(q.klingons);
    put1bcd(q.starbases);
    put1bcd(q.stars);
}

pub fn long_range_scan(game: &mut Game) {
    if game
        .ship
        .get_device(DeviceType::LongRangeSensors)
        .is_inoperable()
    {
        return;
    }

    let ship_quadrant_position = game.ship.get_quadrant_position();

    println!(
        "Long Range Scan for Quadrant {}, {}\n",
        ship_quadrant_position.y, ship_quadrant_position.x
    );

    for i in (ship_quadrant_position.x - 1)..=(ship_quadrant_position.x + 1) {
        print!("{}\n:", LRS_1);
        for j in (ship_quadrant_position.y - 1)..=(ship_quadrant_position.y + 1) {
            print!(" ");
            if i >= 0 && i < 8 && j >= 0 && j < 8 {
                game.quadrant_map[i as usize][j as usize].visited = true;
                putbcd(&game.quadrant_map[i as usize][j as usize]);
            } else {
                print!("***");
            }
            print!(" :");
        }
        println!("");
    }

    println!("{}", LRS_1);
}

pub fn phaser_control(game: &mut Game) {
    if game
        .ship
        .get_device(DeviceType::PhaserControl)
        .is_inoperable()
    {
        return;
    }

    if game.no_klingons_in_current_quadrant() {
        return;
    }

    /* There's Klingons on the starboard bow... */
    if game
        .ship
        .get_device(DeviceType::LibraryComputer)
        .is_damaged()
    {
        println!("Computer failure hampers accuracy.");
    }

    println!(
        "Phasers locked on target;\n
    Energy available = {} units",
        game.ship.energy
    );

    let mut phaser_energy = input_i32("Number of units to fire", 0, 10000) as u32;

    if phaser_energy == 0 {
        return;
    }

    if game.ship.energy < phaser_energy {
        println!("Not enough energy available.");
        return;
    }

    game.ship.energy -= phaser_energy;

    /* We can fire up to nearly 3000 points of energy so we do this
    bit in 32bit math */

    if game
        .ship
        .get_device(DeviceType::LibraryComputer)
        .is_damaged()
    {
        phaser_energy *= get_rand(100) as u32;
    } else {
        phaser_energy *= 100;
    }

    let ship_quadrant_position = game.ship.get_quadrant_position();
    let alive_local_klingons = game
        .klingons
        .iter_mut()
        .filter(|k| !k.destroyed)
        .filter(|k| k.get_quadrant_position() == ship_quadrant_position)
        .collect::<Vec<_>>();

    let h1 = phaser_energy / alive_local_klingons.len() as u32;

    let mut destroyed_klingon_ships = 0;

    for k in alive_local_klingons {
        /* We are now 32bit with four digits accuracy */
        let mut h = h1 * (get_rand(100) as u32 + 200);
        /* Takes us down to 2 digit accuracy */

        let hp: Box<dyn HasPosition> = Box::new(*k);
        let d = game.ship.distance_to(&hp);
        h = (h as f32 / d) as u32;

        if h <= 15 * k.energy {
            /* was 0.15 */
            println!(
                "Sensors show no damage to enemy at
{}, {}\n",
                k.get_position().x,
                k.get_position().y
            );
        } else {
            println!(
                "{} unit hit on Klingon at sector
{}, {}",
                h,
                k.get_position().x,
                k.get_position().y
            );

            if k.energy < h {
                println!("*** Klingon Destroyed ***");
                k.energy = 0;
                k.destroyed = true;

                game.sector_map[k.get_position().x as usize][k.get_position().y as usize]
                    .sector_type = SectorType::SPACE;

                destroyed_klingon_ships += 1;

                game.klingons_left -= 1;
                if game.klingons_left == 0 {
                    break;
                }
            } else {
                k.energy -= h;
                println!("   (Sensors show {} units remaining.)\n", k.energy);
            }
        }
    }

    if destroyed_klingon_ships > 0 {
        /* Minus a Klingon.. */
        let current_quadrant = game.get_mut_current_quadrant();
        current_quadrant.klingons -= destroyed_klingon_ships;
        drop(current_quadrant);

        game.klingons_left -= destroyed_klingon_ships;

        if game.klingons_left == 0 {
            won_game(game);
        }
    }

    klingons_shoot(game);
}

pub fn photon_torpedoes(game: &mut Game) {
    if game.ship.torps == 0 {
        print!("All photon torpedoes expended");
        return;
    }

    if game
        .ship
        .get_device(DeviceType::PhotonTubes)
        .is_inoperable()
    {
        return;
    }

    let c1 = get_course("Ensign Chekov");

    if c1 == None {
        return;
    }

    /* FIXME: energy underflow check ? */
    game.ship.energy -= 2;
    game.ship.torps -= 1;

    let c1 = c1.unwrap();

    let c1_position = get_new_position(c1);

    // depends on c1
    println!(
        "c1 {c1}, c1_position_: {}, {}",
        c1_position.x, c1_position.y
    );

    print!("Torpedo Track:");

    let ship_quadrant_position = game.ship.get_quadrant_position();

    let mut torpedo_position = game.ship.position.clone();

    loop {
        torpedo_position.x += c1_position.x;
        torpedo_position.y += c1_position.y;

        if torpedo_position.get_quadrant_position() != ship_quadrant_position {
            break;
        }

        let torpedo_sector_position = torpedo_position.get_sector_position();

        println!(
            "    {}, {}",
            torpedo_sector_position.x, torpedo_sector_position.y
        );

        let sector_type = game.get_sector(&torpedo_position).sector_type;
        /* In certain corner cases the first trace we'll step is
        ourself. If so treat it as space */
        if sector_type != SectorType::SPACE && sector_type != SectorType::SHIP {
            torpedo_hit(game, &torpedo_position);
            klingons_shoot(game);
            return;
        }
    }

    println!("Torpedo Missed");

    klingons_shoot(game);
}

pub fn torpedo_hit(game: &mut Game, torpedo_position: &Vec2) {
    let sector_type = game.get_sector(&torpedo_position).sector_type;
    let torpedo_sector_position = torpedo_position.get_sector_position();

    match sector_type {
        SectorType::STAR => println!(
            "Star at {}, {} absorbed torpedo energy.\n",
            torpedo_sector_position.x, torpedo_sector_position.y
        ),
        SectorType::KLINGON => {
            println!("*** Klingon Destroyed ***");

            game.get_mut_current_quadrant().starbases -= 1;
            game.klingons_left -= 1;

            if game.klingons_left == 0 {
                won_game(game);
            }

            game.klingons
                .iter_mut()
                .filter(|k| !k.destroyed)
                .filter(|k| k.get_position() == torpedo_position)
                .collect::<Vec<_>>()
                .first_mut()
                .unwrap()
                .energy = 0;
            game.klingons
                .iter_mut()
                .filter(|k| !k.destroyed)
                .filter(|k| k.get_position() == torpedo_position)
                .collect::<Vec<_>>()
                .first_mut()
                .unwrap()
                .destroyed = true;
        }
        SectorType::BASE => {
            println!("*** Starbase Destroyed ***");
            game.starbases_left -= 1;

            if game.starbases_left == 0
            /*&&
            game.klingons_left <= game.time_up */
            {
                /* showfile ? FIXME */
                println!(
                    "That does it, Captain!!
You are hereby relieved of command
and sentenced to 99 stardates of hard
labor on Cygnus 12!!"
                );
                resign_commision(game);
            }

            println!(
                "Starfleet Command reviewing your record to consider
    court martial!"
            );

            game.ship.docked = false; /* Undock */

            game.get_mut_current_quadrant().starbases -= 1;

            game.starbases
                .iter_mut()
                .filter(|s| s.get_position() == torpedo_position)
                .collect::<Vec<_>>()
                .first_mut()
                .unwrap()
                .destroyed = true;
        }
        _ => println!("!!!!"),
    }

    game.get_mut_sector(torpedo_position).sector_type = SectorType::SPACE;
}

pub fn damage_control(game: &mut Game) {
    let mut repair_cost;

    if game.ship.get_device(DeviceType::DamageControl).is_damaged() {
        print!("Damage Control report not available.");
    }

    /* Offer repair if docked */
    if game.ship.docked {
        /* repair_cost is x.xx fixed point */
        repair_cost = 0;

        for i in 0..DeviceType::DeviceNum as u8 {
            let device = game.ship.get_device(DeviceType::from_u8(i));

            if device.is_damaged() {
                repair_cost = repair_cost + 10;
            }
        }

        if repair_cost > 0 {
            repair_cost = repair_cost + game.d4;

            if repair_cost >= 100 {
                repair_cost = 90; /* 0.9 */
            }

            println!(
                "Technicians standing by to effect repairs to your
ship;\nEstimated time to repair: {} stardates.",
                repair_cost
            );

            if yesno("Will you authorize the repair order?", false) {
                for i in 0..DeviceType::DeviceNum as u8 {
                    let device = game.ship.get_mut_device(DeviceType::from_u8(i));

                    if device.is_damaged() {
                        device.repair_all_damage();
                    }
                }
                /* Work from two digit to one digit. We might actually
                have to give in and make t a two digt offset from
                a saved constant base only used in printing to
                avoid that round below FIXME */
                game.stardate += ((repair_cost + 5) / 10 + 1) as f32;
            }

            return;
        }
    }

    if game.ship.get_device(DeviceType::DamageControl).is_damaged() {
        return;
    }

    print!("Device            State of Repair");

    for i in 1..8 as u8 {
        let device = game.ship.get_device(DeviceType::from_u8(i));
        println!("{:25}{:6}\n", device.name, device.damage);

        println!("");
    }
}

pub fn shield_control(game: &mut Game) {
    if game
        .ship
        .get_device(DeviceType::ShieldControl)
        .is_inoperable()
    {
        return;
    }

    println!("Energy available = {}", game.ship.get_total_energy());

    //let i = input_i32() as u32;

    let i = input_i32("Input number of units to shields", 0, 10000) as u32;

    if game.ship.shield == i {
        if i >= game.ship.get_total_energy() {
            println!(
                "Shield Control Reports:\n
      'This is not the Federation Treasury.'"
            );
        }
        println!("<Shields Unchanged>");
        return;
    }

    game.ship.energy = game.ship.energy + game.ship.shield - i;
    game.ship.shield = i;

    println!(
        "Deflector Control Room report:\n
  'Shields now at {} units per your command.'\n",
        game.ship.shield
    );
}

pub fn library_computer(game: &mut Game) {
    if game
        .ship
        .get_device(DeviceType::LibraryComputer)
        .is_inoperable()
    {
        return;
    }

    let i = input_i32("Computer active and awaiting command", 0, 9);

    //println!("selection: {i}");

    match i {
        0 => galactic_record(&game),
        1 => status_report(&game),

        2 => torpedo_data(&game),

        3 => nav_data(&game),
        4 => dirdist_calc(&game),
        5 => galaxy_map(&game),

        _ =>
        /* FIXME: showfile */
        {
            println!(
                "Functions available from Library-Computer:\n\n
  0 = Cumulative Galactic Record\n
  1 = Status Report\n
  2 = Photon Torpedo Data\n
  3 = Starbase Nav Data\n
  4 = Direction/Distance Calculator\n
  5 = Galaxy 'Region Name' Map"
            )
        }
    }
}

pub fn galactic_record(game: &Game) {
    let game_ship_quadrant = game.ship.get_quadrant_position();
    println!(
        "\n     Computer Record of Galaxy for Quadrant {},{}\n",
        game_ship_quadrant.x, game_ship_quadrant.y
    );
    println!("     1     2     3     4     5     6     7     8");

    for i in 0..8 {
        print!("{}\n{}", GR_1, i);

        for j in 0..8 {
            print!("   ");

            if game.quadrant_map[i][j].visited {
                putbcd(&game.quadrant_map[i][j]);
            } else {
                print!("***");
            }
        }
        println!("");
    }

    println!("{}", GR_1);
}

pub fn status_report(game: &Game) {
    let mut plural = "";

    print!("   Status Report:\n");

    if game.klingons_left > 1 {
        plural = STR_S;
    }

    /* Assumes fixed point is single digit fixed */
    println!(
        "Klingon{} Left: {}\n
Mission must be completed in {} stardates",
        plural,
        game.klingons_left,
        game.time_up - game.stardate
    );

    if game.starbases_left < 1 {
        println!(
            "Your stupidity has left you on your own in the galaxy\n
 -- you have no starbases left!"
        );
    } else {
        plural = "";

        if game.starbases_left > 1 {
            plural = STR_S;
        }

        println!(
            "The Federation is maintaining {} starbase{} in the galaxy\n",
            game.starbases_left, plural
        );
    }
}

pub fn torpedo_data(game: &Game) {
    let mut plural = "";

    let ship_quadrant_position = game.ship.get_quadrant_position();

    let klingons_in_quadrant = game.quadrant_map[ship_quadrant_position.x as usize]
        [ship_quadrant_position.y as usize]
        .klingons;

    let mut klingons: Vec<&Klingon> = Vec::new();

    for k in &game.klingons {
        let klingon_quadrant_position = k.get_quadrant_position();

        if klingon_quadrant_position == ship_quadrant_position {
            klingons.push(&k);
        }
    }

    /* Need to also check sensors here ?? */
    if klingons_in_quadrant == 0 {
        return;
    }

    if klingons_in_quadrant > 1 {
        plural = STR_S;
    }

    println!("From Enterprise to Klingon battlecruiser{}:\n\n", plural);

    for k in klingons {
        if k.energy > 0 {
            compute_vector(&k.position, &game.ship.position);
        }
    }
}

pub fn nav_data(game: &Game) {
    let ship_quadrant_position = game.ship.get_quadrant_position();

    if game.quadrant_map[ship_quadrant_position.x as usize][ship_quadrant_position.y as usize]
        .starbases
        <= 0
    {
        println!(
            "Mr. Spock reports,\n
          'Sensors show no starbases in this quadrant.'"
        );
        return;
    }

    for starbase in &game.starbases {
        let starbase_quadrant_position = starbase.get_quadrant_position();

        if ship_quadrant_position == starbase_quadrant_position {
            compute_vector(&starbase.position, &game.ship.position);
        }
    }
}

/* Q: do we want to support fractional co-ords ? */

pub fn dirdist_calc(game: &Game) {
    /*
    int16_t c1, a, w1, x;
    print!("Direction/Distance Calculator\n"
           "You are at quadrant %d,%d sector %d,%d\n\n"
           "Please enter initial X coordinate: ",
           quad_y, quad_x,
           FROM_FIXED00(ship_y), FROM_FIXED00(ship_x));

    c1 = TO_FIXED00(input_int());
    if (c1 < 0 || c1 > 900 )
        return;

    fprint!("Please enter initial Y coordinate: ", stdout);
    a = TO_FIXED00(input_int());
    if (a < 0 || a > 900)
        return;

    fprint!("Please enter final X coordinate: ", stdout);
    w1 = TO_FIXED00(input_int());
    if (w1 < 0 || w1 > 900)
        return;

    fprint!("Please enter final Y coordinate: ", stdout);
    x = TO_FIXED00(input_int());
    if (x < 0 || x > 900)
        return;
    compute_vector(w1, x, c1, a);
    */
}

pub fn galaxy_map(game: &Game) {
    return;
    /*
    println!("\n                   The Galaxy\n");
    println!("    1     2     3     4     5     6     7     8");

    for i in 0..8 {
        print!("%s%d ", gm_1, i);

        quadrant_name(1, i, 1);

        let j0 = (int) (11 - (strlen(quadname) / 2));

        for j in 0..j0 {
            print!(" ");
        }

        fprint!(quadname, stdout);

        for j in 0..j0 {
            print!(" ");
        }

        if (!(strlen(quadname) % 2))
            putchar(' ');

        quadrant_name(1, i, 5);

        j0 = (int) (12 - (strlen(quadname) / 2));

        for j in 0..j0 {
            print!(" ");
        }

        print!(quadname);
    }

    print!("{GM_1}");
     */
}

pub fn compute_vector(p1: &Vec2, p2: &Vec2) {
    print!("  DIRECTION = ");
}

pub fn show_file(filename: &str) {
    let contents = read_to_string(filename).expect("Should have been able to read the file");

    println!("{contents}");
}

pub fn intro() {
    show_file("assets/startrek.intro");

    if yesno("Do you need instructions?", false) {
        show_file("assets/startrek.doc");
    }

    show_file("assets/startrek.logo");
}

pub fn show_orders(game: &Game) {
    let (plural, plural_2) = if game.starbases_left != 1 {
        ("are", "s")
    } else {
        ("is", "")
    };

    println!(
        "Your orders are as follows:
 Destroy the {} Klingon warships which have invaded
 the galaxy before they can attack Federation Headquarters
 on stardate {:.2}. This gives you {} days. There {}
 {} starbase{} in the galaxy for resupplying your ship.
 ",
        game.klingons_left, game.stardate, game.time_days, plural, game.starbases_left, plural_2
    );

    any_key("Hit any key to accept command.");
}

pub fn new_quadrant(game: &mut Game) {
    /* Random factor for damage repair. We compute it on each new
    quadrant to stop the user just retrying until they get a number
    they like. The conversion here was wrong and now copies the BASIC
    code generate 0.00 to 0.49 */
    game.d4 = (get_rand(50) - 1) as i32; /* Used for computing damage repair time */

    //println!("{:?}", &game.ship);

    let mut tmp_current_quadrant = game.get_mut_current_quadrant();
    tmp_current_quadrant.visited = true;
    drop(tmp_current_quadrant);

    let current_quadrant = game.get_current_quadrant();
    //println!("{:?}", &current_quadrant);

    if !game.ship.is_outside() {
        let quadname = &current_quadrant.name;

        if game.time_start != game.stardate {
            println!("Now entering {quadname} quadrant...\n");
        } else {
            println!(
                "\nYour mission begins with your starship located
in the galactic quadrant {quadname}\n"
            );
        }
    }

    if current_quadrant.klingons > 0 {
        println!("Combat Area  Condition Red");

        if game.ship.shield < 200 {
            println!("Shields Dangerously Low");
        }
    }
}

pub fn run_game() -> bool {
    let mut game = Game::initialize();

    show_orders(&game);

    new_quadrant(&mut game);

    short_range_scan(&mut game);

    loop {
        if game.ship.is_unable_to_navigate() {
            show_file("assets/startrek.fatal");

            end_of_time(&mut game);

            break;
        }

        let cmd = get_command("Command?");

        match cmd.as_str() {
            "nav" => course_control(&mut game),
            "srs" => short_range_scan(&mut game),
            "lrs" => long_range_scan(&mut game),

            "pha" => phaser_control(&mut game),
            "tor" => photon_torpedoes(&mut game),

            "shi" => shield_control(&mut game),

            "dam" => damage_control(&mut game),
            "com" => library_computer(&mut game),

            "xxx" => resign_commision(&mut game),

            _ => {
                /* FIXME: showfile ?*/
                show_file("assets/commands.txt");
            }
        }

        if game.exit_flag {
            break;
        }
    }

    game.exit_flag
}
