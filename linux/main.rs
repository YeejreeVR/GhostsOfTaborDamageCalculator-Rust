use std::collections::HashMap;

fn main() {
    damagecal();
}
fn input(msg:&str) -> String {
    use std::io::{stdin,stdout,Write};
    print!("{}",msg);
    stdout().flush().unwrap();
    let mut res:String = String::new();
    stdin().read_line(&mut res).expect("Invalid Thingy!!! ^_^").to_string();
    return res.trim().to_string();
}

fn damagecal() {
    let ats:Vec<i32> = vec![624,706,624,624,624,1000,706,990,706,545,545,800,1000,180,949,596,857,180,923,706,1000,747,1000,1091,1091,1200,1500,1600,600,524,747,1040,947,1111,674,1046,747,600,600,857,180,180,180,180,180,180,180,180,180,180,60,60,60,60,180,180,500,666];
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
    let bodypart_damage:Vec<f32> = vec![1.0,1.0,1.0,0.5,0.5,0.35,0.35,0.25,0.25];
    //0 = None/A22
    //1 = LabV
    //2 = JPC2
    //3 = VestV
    //4 = 6B102
    //5 = 6B43
    //6 = R61
    let vest_covering_body:Vec<Vec<i32>> = vec![vec![],vec![2],vec![1,2],vec![1,2],vec![1,2],vec![1,2,3,4],vec![1,2,3,4,5]];
    //0 = FMJ
    //1 = AP
    //2 = Tracer
    let mut ammo_damage:HashMap<i32,Vec<f32>> = HashMap::new();
    ammo_damage.insert(0, vec![50.5, 41.0, 47.3, 46.5, 42.9, 48.6, 47.6, 43.6, 39.8, 74.6, 46.2, 44.9, 46.9, 69.0, 43.6, 41.0, 41.9, 49.0, 49.0, 49.0, 48.6, 31.9, 36.5, 41.1, 29.5, 33.9, 34.6, 28.0, 34.5, 37.4, 37.4, 42.3, 37.4, 39.7, 31.3, 38.8, 45.3, 45.3, 47.0, 36.5, 41.8, 41.8, 87.0, 34.2, 35.3, 35.3, 42.4, 31.0, 15.0, 35.9, 121.3, 81.4, 62.6, 94.9, 215.7, 81.4, 67.3, 78.3]);
    ammo_damage.insert(1, vec![60.1, 44.4, 49.6, 55.1, 44.9, 51.8, 51.7, 47.2, 43.0, 80.8, 50.2, 48.7, 50.9, 74.5, 47.2, 44.4, 45.3, 58.2, 58.2, 53.4, 51.8 ,33.5, 38.7, 42.0, 30.7, 35.8, 36.5, 29.5, 36.4, 39.7, 39.7, 42.3, 39.7, 44.2, 32.9, 40.1, 53.5, 53.5, 57.0, 38.7, 48.3, 48.3, 114.5, 37.4, 37.3, 37.3, 49.2, 32.5, 15.4, 37.0, 143.1, 86.2, 65.3, 103.0, 0.0, 86.2, 70.3, 78.3]);
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
    let mut ammocal:HashMap<i32,Vec<i32>> = HashMap::new();
    ammocal.insert(1,vec![1,1,1,1,1,1,1,1,1,0,1,1,1,0,1,1,1,1,1,1,1     ,4,4,3,4,4,4,4,4,4,4,3,4,3,4,3,4,4,4,4
    ,4,4,3,4,4,4,4,3,4,4     ,0,0,0,0     ,9,0     ,9,9,9     ,0,0]);
    ammocal.insert(0,vec![5,5,5,5,5,5,5,5,5,2,5,5,5,2,5,5,5,5,5,5,5    ,8,8,7,8,8,8,8,8,8,8,6,8,6,8,6,8,8,8,8     
    ,8,7,6,8,8,8,8,6,8,8     ,0,2,2,2     ,0,2     ,9,9,9     ,2,2]);
    //0 = None/A22
    //1 = Labv/SPH-5
    //2 = JPC2/SSH-68
    //3 = VestB/[6B47, ATE, C1300]
    //4 = 6B102/[Krtek Mask, Fish Cultist Maks, Ronin]
    //5 = 6B43/[ALTYN, Mich Helmet]
    //6 = R61
    let mut armor_damage_reduction:HashMap<i32,Vec<f32>> = HashMap::new();
    armor_damage_reduction.insert(0, vec![0.0 , 0.0 , 0.0 , 0.0 , 0.0 , 0.0 , 0.0 , 0.0 , 0.0]);
    armor_damage_reduction.insert(1, vec![0.15 , 0.15 , 0.15 , 0.15 , 0.15 , 0.15 , 0.33 , 0.46 , 0.55]);
    armor_damage_reduction.insert(2, vec![0.15 , 0.15 , 0.15 , 0.15 , 0.15 , 0.35 , 0.52 , 0.65, 0.65]);
    armor_damage_reduction.insert(3, vec![0.15 , 0.15 , 0.15 , 0.15 , 0.38 , 0.57 , 0.75 , 0.75 , 0.75]);
    armor_damage_reduction.insert(4, vec![0.15 , 0.15 , 0.40 , 0.40 , 0.63 , 0.81 , 0.85 , 0.85 , 0.85]);
    armor_damage_reduction.insert(5, vec![0.15 , 0.41 , 0.65 , 0.65 , 0.90 , 0.90 , 0.90 , 0.90 , 0.90]);
    armor_damage_reduction.insert(6, vec![0.15 , 0.67 , 0.88 , 0.93 , 0.93 , 0.93 , 0.93 , 0.93 , 0.93]);
    std::process::Command::new("clear").status().unwrap();
    let ph:&str = &input("Gun Type\nRifle: 0     SMG: 1     Handgun: 2     Bolt-Action Rifle: 3     Sniper Rifle: 4     LMG: 5     ALL: 6\n>>>     ");
    std::process::Command::new("clear").status().unwrap();
    let mut gun:usize = 0;
    if ph == "0" {
        gun = input("Gun\nAK Alpha: 0             AK5C: 1             AK-74: 2\nAKM: 3                  AK74-U: 4           AS VAL: 5\nAug: 6                  Famas: 7            G36K: 8\nG3A3: 9                 Galil: 10           L85A2: 11\nM16: 12                 M1 SASS: 13         M4: 14\nSCAR: 15                SG552: 16           SKS: 17\nSKS (auto): 18          Stoner: 19          VSS: 20\n>>>     ").parse().unwrap();
    }
    else if ph == "1" {
        gun = input("Gun\nAgram: 0               Cx8 Storm: 1       Vector10mm: 2\nVector9mm: 3            Luty: 4            Mac 10: 5\nMac 11: 6               MAT-49: 7          MP40: 8\nMP5: 9                  MP7: 10            MP9: 11\nP90: 12                 PP Bizon: 13       PPSH: 14\nThompson: 15            UMP45: 16          The Fin Reaper: 17\nJW MPX: 18\n>>>     ").parse::<usize>().unwrap() + 21;
    }
    else if ph == "2" {
        gun = input("Gun\nColt 1911: 0            JW 2011: 1         Deagle: 2\nFN57: 3                 Glock: 4           JW G34: 5\nUSP 45: 6               Makarov: 7         Ruger: 8\nTokarev: 9\n>>>     ").parse::<usize>().unwrap() +40;
    }
    else if ph == "3" {
        gun = input("Gun\nAWM: 0                  Mosin: 1           M1903: 2\nHunter 85: 3\n>>>     ").parse::<usize>().unwrap() + 50;
    }
    else if ph == "4" {
        gun = input("Gun\nBarret: 0             Dragunov: 1\n>>>     ").parse::<usize>().unwrap() + 54;
    }
    else if ph == "5" {
        gun = input("Gun\nBAR: 0                  XM250: 1\n>>>     ").parse::<usize>().unwrap() + 56;
    }
    std::process::Command::new("clear").status().unwrap();
    let mut bodypart:i32 = input("Body Part\nHead: 0     Neck:1     Torso: 2     Pelvis: 3     Upper Arms: 4     Upper Legs: 5     Lower Arms: 6     Lower Legs: 7     Hands: 8     Feet: 9\n>>>     ").parse().unwrap();
    std::process::Command::new("clear").status().unwrap();
    let mut vest:i32 = input("Vest\n0: None/A22     1: LabB     2: JPC2     3: VestB     4: 6B102     5: 6B43     6: R61\n>>>     ").parse().unwrap();
    std::process::Command::new("clear").status().unwrap();
    let mut helmet:i32 = input("Select A Helmet\nNone: 0     SPH-5: 1     SSh-68: 2    6B47: 3     ATE: 3     C1300: 3     Ronin: 4     Cultist Mask: 4     Krtek Mask: 4     Altyn: 5     Collector Helmet: 5\n>>>     ").parse().unwrap();
    std::process::Command::new("clear").status().unwrap();
    let mut ammo_type:i32 = input("Ammo Type\n0: FMJ     1: AP\n>>>     ").parse().unwrap();
    std::process::Command::new("clear").status().unwrap();
    
    let mut armor = 0;
    println!("{}",armor);
    if bodypart == 0 {
        armor = helmet;
    }
    else {
        if vest_covering_body[vest as usize].contains(&bodypart) {
            armor = vest
        }
        else {
            armor = 0;
        }
    }
    let damage_per_shot:f32 = ammo_damage.get(&ammo_type).unwrap()[gun] * (1.0 - armor_damage_reduction.get(&armor).unwrap()[ammocal.get(&ammo_type).unwrap()[gun] as usize]) * bodypart_damage[bodypart as usize];
    let mut shots_to_kill = 0.0;
    let mut time_to_kill = 0.0;
    println!("{}{}",shots_to_kill,time_to_kill);
    std::process::Command::new("clear").status().unwrap();
    if damage_per_shot >=0.0 {
        shots_to_kill = 100.0/damage_per_shot.ceil();
        time_to_kill = shots_to_kill / ats[gun] as f32;
    }
    else {
        shots_to_kill = 0.0;
        time_to_kill = 0.0;
    }
    let damage_per_second = damage_per_shot * (ats[gun] as f32 / 60.0);
    println!("Damage Per Shot: {}\nShots to Kill: {}\nTime to Kill; {}\nDamage Per Second: {}\nShots Per Minute: {}",damage_per_shot,shots_to_kill,time_to_kill,damage_per_second,ats[gun]);
    let mut done:bool = false;
    while !done {
        if input("Change Any Variables? (Y/n)\n>>>     ").to_lowercase() == "y" {
            std::process::Command::new("clear").status().unwrap();
            let pin = &input("What Variable To Change?\n0: Gun     1: Body Part     2: Vest     3: Helmet     4: Ammo Type     5: All");
            std::process::Command::new("clear").status().unwrap();
            if pin == "0" {
                let ph:&str = &input("Gun Type\nRifle: 0     SMG: 1     Handgun: 2     Bolt-Action Rifle: 3     Sniper Rifle: 4     LMG: 5     ALL: 6\n>>>     ");
                std::process::Command::new("clear").status().unwrap();
                if ph == "0" {
                    gun = input("Gun\nAK Alpha: 0             AK5C: 1             AK-74: 2\nAKM: 3                  AK74-U: 4           AS VAL: 5\nAug: 6                  Famas: 7            G36K: 8\nG3A3: 9                 Galil: 10           L85A2: 11\nM16: 12                 M1 SASS: 13         M4: 14\nSCAR: 15                SG552: 16           SKS: 17\nSKS (auto): 18          Stoner: 19          VSS: 20\n>>>     ").parse().unwrap();
                }
                else if ph == "1" {
                    gun = input("Gun\nAgram: 0               Cx8 Storm: 1       Vector10mm: 2\nVector9mm: 3            Luty: 4            Mac 10: 5\nMac 11: 6               MAT-49: 7          MP40: 8\nMP5: 9                  MP7: 10            MP9: 11\nP90: 12                 PP Bizon: 13       PPSH: 14\nThompson: 15            UMP45: 16          The Fin Reaper: 17\nJW MPX: 18\n>>>     ").parse::<usize>().unwrap() + 21;
                }
                else if ph == "2" {
                    gun = input("Gun\nColt 1911: 0            JW 2011: 1         Deagle: 2\nFN57: 3                 Glock: 4           JW G34: 5\nUSP 45: 6               Makarov: 7         Ruger: 8\nTokarev: 9\n>>>     ").parse::<usize>().unwrap() +40;
                }
                else if ph == "3" {
                    gun = input("Gun\nAWM: 0                  Mosin: 1           M1903: 2\nHunter 85: 3\n>>>     ").parse::<usize>().unwrap() + 50;
                }
                else if ph == "4" {
                    gun = input("Gun\nBarret: 0             Dragunov: 1\n>>>     ").parse::<usize>().unwrap() + 54;
                }
                else if ph == "5" {
                    gun = input("Gun\nBAR: 0                  XM250: 1\n>>>     ").parse::<usize>().unwrap() + 56;
                }
            }
            else if pin == "1" {
                bodypart = input("Body Part\nHead: 0     Neck:1     Torso: 2     Pelvis: 3     Upper Arms: 4     Upper Legs: 5     Lower Arms: 6     Lower Legs: 7     Hands: 8     Feet: 9\n>>>     ").parse().unwrap();
            }
            else if pin == "2" {
                vest = input("Vest\n0: None/A22     1: LabB     2: JPC2     3: VestB     4: 6B102     5: 6B43     6: R61\n>>>     ").parse().unwrap(); 
            }
            else if pin == "3" {
                helmet = input("Select A Helmet\nNone: 0     SPH-5: 1     SSh-68: 2    6B47: 3     ATE: 3     C1300: 3     Ronin: 4     Cultist Mask: 4     Krtek Mask: 4     Altyn: 5     Collector Helmet: 5\n>>>     ").parse().unwrap();
            }
            else if pin == "4" {
                ammo_type = input("Ammo Type\n0: FMJ     1: AP\n>>>     ").parse().unwrap();
            }
            else if pin == "5" {
                done = true;
                damagecal();
            }

            if bodypart == 0 {
                armor = helmet;
            }
            else {
                if vest_covering_body[vest as usize].contains(&bodypart) {
                    armor = vest
                }
                else {
                    armor = 0;
                }
            }
            let damage_per_shot:f32 = ammo_damage.get(&ammo_type).unwrap()[gun] * (1.0 - armor_damage_reduction.get(&armor).unwrap()[ammocal.get(&ammo_type).unwrap()[gun] as usize]) * bodypart_damage[bodypart as usize];
            let mut shots_to_kill = 0.0;
            let mut time_to_kill = 0.0;
            println!("{}{}",shots_to_kill,time_to_kill);
            std::process::Command::new("clear").status().unwrap();
            if damage_per_shot >=0.0 {
                shots_to_kill = 100 as f32/damage_per_shot.ceil();
                time_to_kill = shots_to_kill / ats[gun] as f32;
            }
            else {
                shots_to_kill = 0.0;
                time_to_kill = 0.0;
            }
            let damage_per_second = damage_per_shot * (ats[gun] as f32 / 60.0);
            println!("Damage Per Shot: {}\nShots to Kill: {}\nTime to Kill; {}\nDamage Per Second: {}\nShots Per Minute: {}",damage_per_shot,shots_to_kill,time_to_kill,damage_per_second,ats[gun])
        }
        else {
            done = true;
        }
    }
}
