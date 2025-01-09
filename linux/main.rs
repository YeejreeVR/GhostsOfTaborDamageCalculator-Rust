use std::{collections::HashMap, vec};



fn input(msg:&str) -> String {
    use std::io::{stdin,stdout,Write};
    print!("{}",msg);
    stdout().flush().unwrap();
    let mut res:String = String::new();
    stdin().read_line(&mut res).expect("Invalid Thingy!!! ^_^").to_string();
    return res.trim().to_string();
}
fn main() {
    damagecal();
}
#[allow(unused_assignments)]
fn damagecal() {
    
    let fmjdam0:Vec<f32> = vec![50.5, 41.0, 47.3, 46.5, 42.9, 48.6, 47.6, 43.6, 39.8, 74.6, 46.2, 44.9, 46.9, 69.0, 43.6, 41.0, 41.9, 49.0, 49.0, 49.0, 48.6, 31.9, 36.5, 41.1, 29.5, 33.9, 34.6, 28.0, 34.5, 37.4, 37.4, 42.3, 37.4, 39.7, 31.3, 38.8, 45.3, 45.3, 47.0, 36.5, 41.8, 41.8, 87.0, 34.2, 35.3, 35.3, 42.4, 31.0, 15.0, 35.9, 121.3, 81.4, 62.6, 94.9, 215.7, 81.4, 67.3, 78.3];
    let fmjdam1:Vec<f32> = vec![42.9, 34.9, 40.2, 39.5, 36.5, 41.3, 40.5, 37.1, 33.8, 63.4, 39.3, 38.2, 39.9, 58.7, 37.1, 34.9, 35.6, 41.7, 41.7, 41.7, 41.3, 14.4, 16.4, 22.2, 13.3, 15.3, 15.6, 12.6, 15.5, 16.8, 16.8, 28.6, 16.8, 26.8, 14.1, 26.2, 20.4, 20.4, 21.2, 16.4, 18.8, 18.8, 58.7, 23.1, 15.9, 15.9, 19.1, 14.0, 6.8, 24.2, 103.1, 69.2, 53.2, 80.7, 183.3, 69.2, 57.2, 66.6];
    let fmjdam2:Vec<f32> = vec![32.8, 26.7, 30.7, 30.2, 27.9, 31.6, 30.9, 28.3, 25.9, 63.4, 30.0, 29.2, 30.5, 58.7, 28.3, 26.7, 27.2, 31.9, 31.9, 31.9, 31.6, 11.2, 12.8, 14.4, 10.3, 11.9, 12.1, 9.8, 12.1, 13.1, 13.1, 20.5, 13.1, 19.3, 11.0, 18.8, 15.9, 15.9, 16.5, 12.8, 14.6, 14.6, 30.5, 16.6, 12.4, 12.4, 14.8, 10.9, 5.3, 17.4, 103.1, 69.2, 53.2, 80.7, 183.3, 69.2, 57.2, 66.6];
    let fmjdam3:Vec<f32> = vec![21.7, 17.6, 20.3, 20.0, 18.4, 20.9, 20.5, 18.7, 17.1, 63.4, 19.9, 19.3, 20.2, 58.7, 18.7, 17.6, 18.0, 21.1, 21.1, 21.1, 20.9, 8.0, 9.1, 10.3, 7.4, 8.5, 8.7, 7.0, 8.6, 9.4, 9.4, 10.6, 9.4, 9.9, 7.8, 9.7, 11.3, 11.3, 11.8, 9.1, 10.5, 10.5, 21.8, 8.6, 8.8, 8.8, 10.6, 7.8, 3.8, 9.0, 103.1, 69.2, 53.2, 80.7, 183.3, 69.2, 57.2, 66.6];
    let fmjdam4:Vec<f32> = vec![9.6, 7.8, 9.0, 8.8, 8.2, 9.2, 9.0, 8.3, 7.6, 44.8, 8.8, 8.5, 8.9, 41.4, 8.3, 7.8, 8.0, 9.3, 9.3, 9.3, 9.2, 4.8, 5.5, 6.2, 4.4, 5.1, 5.2, 4.2, 5.2, 5.6, 5.6, 6.3, 5.6, 6.0, 4.7, 5.8, 6.8, 6.8, 7.1, 5.5, 6.3, 6.3, 13.1, 5.1, 5.3, 5.3, 6.4, 4.7, 2.3, 5.4, 103.1, 48.8, 37.6, 56.9, 183.3, 48.8, 40.4, 47.0];
    let fmjdam5:Vec<f32> = vec![5.1, 4.1, 4.7, 4.7, 4.3, 4.9, 4.8, 4.4, 4.0, 25.9, 4.6, 4.5, 4.7, 24.0, 4.4, 4.1, 4.2, 4.9, 4.9, 4.9, 4.9, 3.2, 3.7, 4.1, 3.0, 3.4, 3.5, 2.8, 3.5, 3.7, 3.7, 4.2, 3.7, 4.0, 3.1, 3.9, 4.5, 4.5, 4.7, 3.7, 4.2, 4.2, 8.7, 3.4, 3.5, 3.5, 4.2, 3.1, 1.5, 3.6, 103.1, 28.3, 21.8, 33.0, 183.3, 28.3, 23.4, 27.2];
    let fmjdam6:Vec<f32> = vec![3.5, 2.9, 3.3, 3.3, 3.0, 3.4, 3.3, 3.1, 2.8, 8.8, 3.2, 3.1, 3.3, 4.8, 3.1, 2.9, 2.9, 3.4, 3.4, 3.4, 3.4, 2.2, 2.6, 2.9, 2.1, 2.4, 2.4, 2.0, 2.4, 2.6, 2.6, 3.0, 2.6, 2.8, 2.2, 2.7, 3.2, 3.2, 3.3, 2.6, 2.6, 2.6, 6.1, 2.4, 2.5, 2.5, 3.0, 2.2, 1.1, 2.5, 103.1, 9.6, 7.4, 11.2, 183.3, 9.6, 7.9, 9.2];

    let apdam0:Vec<f32> = vec![60.1, 44.4, 49.6, 55.1, 44.9, 51.8, 51.7, 47.2, 43.0, 80.8, 50.2, 48.7, 50.9, 74.5, 47.2, 44.4, 45.3, 58.2, 58.2, 53.4, 51.8 ,33.5, 38.7, 42.0, 30.7, 35.8, 36.5, 29.5, 36.4, 39.7, 39.7, 42.3, 39.7, 44.2, 32.9, 40.1, 53.5, 53.5, 57.0, 38.7, 48.3, 48.3, 114.5, 37.4, 37.3, 37.3, 49.2, 32.5, 15.4, 37.0, 143.1, 86.2, 65.3, 103.0, 0.0, 86.2, 70.3, 78.3];
    let apdam1:Vec<f32> = vec![51.1, 37.7, 42.2, 46.8, 38.2, 44.0, 43.9, 40.1, 36.6, 68.7, 42.7, 41.4, 43.3, 63.3, 40.1, 37.7, 38.5, 49.5, 49.5, 45.4, 44.0, 28.5, 32.9, 35.7, 26.1, 30.4, 31.0, 25.1, 30.9, 33.7, 33.7, 36.0, 33.7, 37.6, 28.0, 34.1, 45.5, 45.5, 48.5, 32.9, 41.1, 41.1, 97.3, 31.8, 31.7, 31.7, 41.8, 27.6, 13.1, 31.5, 121.6, 73.3, 55.5, 87.6, 0.0, 73.3, 59.8, 66.6];
    let apdam2:Vec<f32> = vec![51.1, 37.7, 42.2, 46.8, 38.2, 44.0, 43.9, 40.1, 36.6, 68.7, 42.7, 41.4, 43.3, 63.3, 40.1, 37.7, 38.5, 49.5, 49.5, 45.4, 44.0, 28.5, 32.9, 35.7, 26.1, 30.4, 31.0, 25.1, 30.9, 33.7, 33.7, 36.0, 33.7, 37.6, 28.0, 34.1, 45.5, 45.5, 48.5, 32.9, 41.1, 41.1, 97.3, 31.8, 31.7, 31.7, 41.8, 27.6, 13.1, 31.5, 121.6, 73.3, 55.5, 87.6, 0.0, 73.3, 59.8, 66.6];
    let apdam3:Vec<f32> = vec![51.1, 37.7, 42.2, 46.8, 38.2, 44.0, 43.9, 40.1, 36.6, 68.7, 42.7, 41.4, 43.3, 63.3, 40.1, 37.7, 38.5, 49.5, 49.5, 45.4, 44.0, 20.9, 24.2, 26.3, 19.2, 22.4, 22.8, 18.4, 22.8, 24.8, 24.8, 36.0, 24.8, 37.6, 20.6, 34.1, 33.4, 33.4, 48.5, 24.2, 30.2, 30.2, 97.3, 31.8, 23.3, 23.3, 30.8, 20.3, 9.6, 31.5, 121.6, 73.3, 55.5, 87.6, 0.0, 73.3, 59.8, 66.6];
    let apdam4:Vec<f32> = vec![51.1, 37.7, 42.2, 46.8, 38.2, 44.0, 43.9, 40.1, 36.6, 68.7, 42.7, 41.4, 43.3, 63.3, 40.1, 37.7, 38.5, 49.5, 49.5, 45.4, 44.0, 12.6, 14.5, 15.8, 11.5, 13.4, 13.7, 11.1, 13.7, 14.9, 14.9, 25.4, 14.9, 26.5, 12.3, 24.1, 20.1, 20.1, 21.4, 14.5, 14.9, 14.9, 42.9, 22.4, 14.0, 14.0, 18.5, 12.2, 5.8, 22.2, 121.6, 73.3, 55.5, 87.6, 0.0, 73.3, 59.8, 66.6];
    let apdam5:Vec<f32> = vec![35.3, 26.1, 29.1, 32.4, 26.4, 30.4, 30.4, 27.7, 25.3, 68.7, 29.5, 28.6, 29.9, 63.3, 27.7, 26.1, 26.6, 34.2, 34.2, 31.4, 30.4, 3.4, 3.9, 4.2, 3.1, 3.6, 3.7, 3.0, 3.6, 4.0, 4.0, 14.7, 4.0, 15.4, 3.3, 13.9, 5.4, 5.4, 5.7, 3.9, 4.8, 4.8, 11.5, 13.0, 3.7, 3.7, 4.9, 3.3, 1.5, 12.9, 121.6, 73.3, 55.5, 87.6, 0.0, 73.3, 59.8, 66.6];
    let apdam6:Vec<f32> = vec![19.9, 14.7, 16.4, 18.2, 14.9, 17.1, 17.1, 15.6, 14.2, 68.7, 16.6, 16.1, 16.8, 63.3, 15.6, 14.7, 15., 19.3, 19.3, 17.7, 17.1, 2.3, 2.7, 2.9, 2.1, 2.5, 2.6, 2.1, 2.5, 2.8, 2.8, 3.0, 2.8, 3.1, 2.2, 2.8, 3.7, 3.7, 4.0, 2.7, 3.4, 3.4, 8.0, 2.6, 2.6, 2.6, 3.4, 2.3, 1.1, 2.6, 121.6, 73.3, 55.5, 87.6, 0.0, 73.3, 59.8, 66.6];

    let ats:Vec<i32> = vec![624,706,624,624,624,1000,706,990,706,545,545,800,1000,180,949,596,857,180,923,706,1000,747,1000,1091,1091,1200,1500,1600,600,524,747,1040,947,1111,674,1046,747,600,600,857,180,180,180,180,180,180,180,180,180,180,60,60,60,60,180,180,500,666];

    let mut amsw:HashMap<i32,Vec<Vec<f32>>> = HashMap::new();
    amsw.insert(0, vec![fmjdam0,fmjdam1,fmjdam2,fmjdam3,fmjdam4,fmjdam5,fmjdam6]);
    amsw.insert(1, vec![apdam0,apdam1,apdam2,apdam3,apdam4,apdam5,apdam6]);
    let mut bpr:HashMap<i32,f32> = HashMap::new();
    bpr.insert(0, 1.0);
    bpr.insert(1, 1.0);
    bpr.insert(2, 1.0);
    bpr.insert(3, 0.50);
    bpr.insert(4, 0.5);
    bpr.insert(5, 0.5);
    bpr.insert(6, 0.35);
    bpr.insert(7, 0.35);
    bpr.insert(8, 0.25);
    bpr.insert(9, 0.25);
    let mut arcobp:HashMap<i32,Vec<i32>> = HashMap::new();
    arcobp.insert(0, vec![]);
    arcobp.insert(1, vec![2]);
    arcobp.insert(2, vec![1,2]);
    arcobp.insert(3, vec![1,2]);
    arcobp.insert(4, vec![1,2]);
    arcobp.insert(5, vec![1,2,3,4]);
    arcobp.insert(6, vec![1,2,3,4,5]);
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
    let bodypart:i32 = input("Body Part\nHead: 0     Neck:1     Torso: 2     Pelvis: 3     Upper Arms: 4     Upper Legs: 5     Lower Arms: 6     Lower Legs: 7     Hands: 8     Feet: 9\n>>>     ").parse().unwrap();
    std::process::Command::new("clear").status().unwrap();
    let ammotype:i32 =  input("Ammo Type\nFMJ: 0     AP: 1\n>>>  ").parse().unwrap();
    std::process::Command::new("clear").status().unwrap();
    let presets:&str = &input("Presets\nFish Cultist: 0     Krtek: 1     The Collector: 2     Player: 3     Custom: 4\n>>> ");
    let mut hp = 0;
    let mut vest = 0;
    let mut helmet = 0;
    println!("{}{}{}", hp,vest,helmet);
    std::process::Command::new("clear").status().unwrap();
    if presets =="0" {
        hp = 320;
        vest = 2;
        helmet = 4;
    }
    else if presets == "1" {
        hp = 720;
        vest = 5;
        helmet = 4;
    }
    else if presets == "2" {
        hp = 750;
        vest = 6;
        helmet = 5;
    }
    else if presets == "3" {
        hp = 100;
        vest = input("Select A Vest\nNone/A22: 0     LABV: 1     JPC2: 2     VestB: 3     6B102: 4     6B43: 5     R61: 6\n>>>     ").parse().unwrap();
        std::process::Command::new("clear").status().unwrap();
        helmet = input("Select A Helmet\nNone: 0     SPH-5: 1     SSh-68: 2    6B47: 3     ATE: 3     C1300: 3     Ronin: 4     Cultist Mask: 4     Krtek Mask: 4     Altyn: 5     Collector Helmet: 5\n>>>     ").parse().unwrap();
    }
    else if presets == "4" {
        hp = input("How Much HP?\n>>>     ").parse().unwrap();
        std::process::Command::new("clear").status().unwrap();
        vest = input("Select A Vest\nNone/A22: 0     LABV: 1     JPC2: 2     VestB: 3     6B102: 4     6B43: 5     R61: 6\n>>>     ").parse().unwrap();
        std::process::Command::new("clear").status().unwrap();
        helmet = input("Select A Helmet\nNone: 0     SPH-5: 1     SSh-68: 2    6B47: 3     ATE: 3     C1300: 3     Ronin: 4     Cultist Mask: 4     Krtek Mask: 4     Altyn: 5     Collector Helmet: 5\n>>>     ").parse().unwrap();
    }
    std::process::Command::new("clear").status().unwrap();
    let mut armor = 0;
    if bodypart == 0 {
        armor = helmet;
    }
    else {
        if arcobp.get(&vest).clone().unwrap().contains(&bodypart) {
            armor = vest as usize
        }
        else {
            armor = 0;
        }
    }
    let damage_per_shot = amsw.get(&ammotype).unwrap()[armor][gun] * bpr[&bodypart];
    let mut shots_to_kill = 0.0;
    let mut time_to_kill = 0.0;
    if damage_per_shot >=0.0 {
        shots_to_kill = hp as f32/damage_per_shot.ceil();
        time_to_kill = shots_to_kill / ats[gun] as f32;
    }
    else {
        shots_to_kill = 0.0;
        time_to_kill = 0.0;
    }
    let damage_per_second = damage_per_shot * (ats[gun] as f32 / 60.0);
    println!("Damage Per Shot: {}\nShots to Kill: {}\nTime to Kill; {}\nDamage Per Second: {}\nShots Per Minute: {}",damage_per_shot,shots_to_kill,time_to_kill,damage_per_second,ats[gun])
}
