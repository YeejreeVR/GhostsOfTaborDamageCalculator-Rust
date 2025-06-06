use wasm_bindgen::prelude::*;


#[wasm_bindgen]
    pub fn calculatedamage(gun:&str, bodypart:&str, vest:&str, helmet:&str, ammotype:&str) -> String {
        use std::collections::HashMap;
        let newgun:usize = gun.parse().unwrap();
        let newbodypart = bodypart.parse().unwrap();
        let newvest = vest.parse().unwrap();
        let newhelmet = helmet.parse().unwrap();
        let newammotype = ammotype.parse().unwrap();
    //Bodyparts
    //0 = Head
    //1 = Neck
    //2 = Torso
    //3 = Pelvis
    //4 = Upper Arms
    //5 = Upper Legs
    //6 = Lower Arms
    //7 = Lower Legs
    //8 = Hands
    //9 = Feet

    //Armor
    //0 = None/A22
    //1 = Labv/SPH-5
    //2 = JPC2/SSH-68
    //3 = VestB/[6B47, ATE, C1300]
    //4 = 6B102/[Krtek Mask, Fish Cultist Maks, Ronin]
    //5 = 6B43/[ALTYN, Mich Helmet]
    //6 = R61

    //Ammo Type
    //0 = FMJ
    //1 = AP
    //2 = Tracer

    let ats: Vec<i32> = vec![624, 706, 624, 624, 624, 1000, 706, 990, 706, 545, 545,
                             800, 1000, 180, 949, 596, 857, 180, 923, 706, 1000,
                             747, 1000, 1091, 1091, 1200, 1500, 1600, 600, 524, 747,
                             1040, 947, 1111, 674, 1046, 747, 600, 600, 857, 180,
                             180, 180, 180, 180, 180, 180, 180, 180, 180, 60,
                             60, 60, 60, 180, 180, 500, 666, 1000, 700, 970,
                             970, 1200, 666];
    let bodypart_damage: Vec<f32> = vec![2.5, 2.5, 1.0, 1.0, 1.35, 1.3, 1.1, 1.05, 0.9,0.85];
    let vest_covering_body: Vec<Vec<i32>> = vec![vec![], vec![2], vec![1, 2], vec![1, 2], vec![1, 2], vec![1, 2, 3, 4], vec![1, 2, 3, 4, 5]];

    let mut ammo_damage: HashMap<i32, Vec<f32>> = HashMap::new();
    ammo_damage.insert(0, vec![50.5, 41.0, 47.3, 46.5, 42.9, 48.6, 47.6,
                               43.6, 39.8, 74.6, 46.2,
                               44.9, 46.9, 69.0, 43.6, 41.0, 41.9, 49.0, 49.0, 49.0, 48.6,
                               //smgs
                               31.9, 36.5, 41.1, 29.5, 33.9, 34.6, 28.0, 34.5, 37.4, 37.4,
                               42.3, 37.4, 39.7, 31.3, 38.8, 45.3, 45.3, 47.0, 36.5,
                               //Pistols
                               41.8,
                               41.8, 87.0, 34.2, 35.3, 35.3, 42.4, 31.0, 15.0, 35.9,
                               //Bolt Action Rifles
                               121.3,
                               81.4, 62.6, 94.9,
                               //Snipers
                               215.7, 81.4,
                               //LMGs
                               67.3, 78.3,

                               //New Guys
                               43.4, 78.0,44.0,41.0, 35.3, 77.8]);
    ammo_damage.insert(1, vec![60.1, 44.4, 49.6, 55.1, 44.9, 51.8, 51.7, 47.2, 43.0, 80.8, 50.2,
                               48.7, 50.9, 74.5, 47.2, 44.4, 45.3, 58.2, 58.2, 53.4, 51.8,
                               33.5, 38.7, 42.0, 30.7, 35.8, 36.5, 29.5, 36.4, 39.7, 39.7,
                               42.3, 39.7, 44.2, 32.9, 40.1, 53.5, 53.5, 57.0, 38.7, 48.3,
                               48.3, 114.5, 37.4, 37.3, 37.3, 49.2, 32.5, 15.4, 37.0, 143.1,
                               86.2, 65.3, 103.0, 0.0, 86.2, 70.3, 78.3,46.2,84.0,47.0,
                               45.0, 37.3, 77.8]);
    //0 = Full Power AP
    //1 = Intermediate AP
    //2 = Full Power
    //3 = PDW AP
    //4 = Pistol AP
    //5 = Intermediate
    //6 = PDW
    //7 = Pistol+
    //8 = Pistol
    //9 = N / A
    let mut ammocal: HashMap<i32, Vec<i32>> = HashMap::new();
    ammocal.insert(1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1,
                           1, 1, 0, 1, 1, 1, 1, 1, 1, 1,
                           4, 4, 3, 4, 4, 4, 4, 4, 4, 4,
                           3, 4, 3, 4, 3, 4, 4, 4, 4, 4,
                           4, 3, 4, 4, 4, 4, 3, 4, 4, 0,
                           0, 0, 0, 9, 0, 0, 0, 1, 0, 1,
                           1, 4, 0]);
    ammocal.insert(0, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 2, 5,
                           5, 5, 2, 5, 5, 5, 5, 5, 5, 5,
                           8, 8, 7, 8, 8, 8, 8, 8, 8, 8,
                           6, 8, 6, 8, 6, 8, 8, 8, 8, 8,
                           7, 6, 6, 8, 8, 8, 6, 8, 8, 0,
                           2, 2, 2, 0, 2, 2, 2, 5, 2, 5,
                           5, 8, 2]);

    let mut armor_damage_reduction: HashMap<i32, Vec<f32>> = HashMap::new();
    //A22/None
    armor_damage_reduction.insert(0, vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    //LabV                                      FPAP   IAP   FP   PDWAP PAP   I     PDW   BP    P     Barret
    armor_damage_reduction.insert(1, vec![0.05, 0.05, 0.05, 0.05, 0.05, 0.05, 0.35, 0.05, 0.05, 1.0]);
    //JPC2
    armor_damage_reduction.insert(2, vec![0.10, 0.10, 0.10, 0.05, 0.05, 0.10, 0.50, 0.50, 0.50, 1.0]);
    //VestB
    armor_damage_reduction.insert(3, vec![0.15, 0.15, 0.15, 0.15, 0.15, 0.15, 0.55, 0.55, 0.55, 1.0]);
    //6B102
    armor_damage_reduction.insert(4, vec![0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.55, 0.65, 0.65, 1.0]);
    //6B43
    armor_damage_reduction.insert(5, vec![0.25, 0.25, 0.75, 0.25, 0.75, 0.75, 0.75, 0.75, 0.75, 1.0]);
    //R61
    armor_damage_reduction.insert(6, vec![0.30, 0.30, 0.80, 0.80, 0.80, 0.80, 0.80, 0.80, 0.80, 1.0]);
    let mut armor = 0;
    if newbodypart == 0 {
        armor = newhelmet;
    } else {
        if vest_covering_body[newvest as usize].contains(&newbodypart) {
            armor = newvest
        } else {
            armor = 0;
        }
    }
    println!("armor: {}\nAmmo Type: {}\nArmor Damage Reduction: {}\nGun Id: {}", armor,ammocal.get(&newammotype).unwrap()[newgun] as usize,armor_damage_reduction.get(&armor).unwrap()[ammocal.get(&newammotype).unwrap()[newgun] as usize],newgun);
    let damage_per_shot: f32 = ammo_damage.get(&newammotype).unwrap()[newgun] * (1.0 - armor_damage_reduction.get(&armor).unwrap()[ammocal.get(&newammotype).unwrap()[newgun] as usize]) * bodypart_damage[newbodypart as usize];
    let mut shots_to_kill = 0.0;
    let mut time_to_kill = 0.0;
    if damage_per_shot >= 0.0 {
        shots_to_kill = 100.0 / damage_per_shot;
        time_to_kill = (shots_to_kill.ceil() - 1.0) / (ats[newgun] as f32 / 60.0);
    } else {
        shots_to_kill = 0.0;
        time_to_kill = 0.0;
    }
    let damage_per_second = damage_per_shot * (ats[newgun] as f32 / 60.0);
    //Damage per shot = calculate_damage()["damage_per_shot"]
    //shots to kill = calculate_damage()["shots_to_kill"]
    //time to kill = calculate_damage()["time_to_kill"]
    return format!("{}///{}///{}///{}///{}",damage_per_shot,shots_to_kill.ceil(),time_to_kill,damage_per_second,ats[newgun]);
}
