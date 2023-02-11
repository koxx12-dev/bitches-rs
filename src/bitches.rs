use fake::{faker::name::en::FirstName, Fake};
use rand::{distributions::Standard, prelude::Distribution, random, thread_rng, Rng};
use std::fmt;

#[derive(Debug)]
pub(crate) enum Gender {
    MALE,
    FEMALE,
    OTHER,
}

#[derive(Debug)]
pub(crate) enum Race {
    AFRICAN,
    ASIAN,
    CAUCASIAN,
    HISPANIC,
    INDIAN,
}

#[derive(Debug)]
pub(crate) enum PCType {
    PREBUILT,
    CUSTOM,
}

#[derive(Debug)]
pub(crate) enum CodeQuality {
    PERFECTION,
    MID,
    SHIT,
}

#[derive(Debug)]
pub(crate) enum MentalDoctor {
    EugenBleuler,
    AaronBeck,
    NathanSKline,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Gender::MALE => "Male",
                Gender::FEMALE => "Female",
                Gender::OTHER => "Other",
            }
        )
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Race::ASIAN => "Asian",
                Race::INDIAN => "Indian",
                Race::AFRICAN => "African",
                Race::HISPANIC => "Hispanic",
                Race::CAUCASIAN => "Caucasian",
            }
        )
    }
}

impl fmt::Display for PCType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PCType::PREBUILT => "Prebuilt",
                PCType::CUSTOM => "Custom",
            }
        )
    }
}

impl fmt::Display for CodeQuality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CodeQuality::PERFECTION => "Perfection",
                CodeQuality::MID => "Mid",
                CodeQuality::SHIT => "Shit",
            }
        )
    }
}

impl fmt::Display for MentalDoctor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MentalDoctor::AaronBeck => "Aaron Beck",
                MentalDoctor::EugenBleuler => "Eugen Bleuler",
                MentalDoctor::NathanSKline => "Nathan S. Kline",
            }
        )
    }
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..=2) {
            0 => Gender::FEMALE,
            1 => Gender::MALE,
            _ => Gender::OTHER,
        }
    }
}

impl Distribution<Race> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        match rng.gen_range(0..=4) {
            0 => Race::AFRICAN,
            1 => Race::ASIAN,
            2 => Race::CAUCASIAN,
            3 => Race::HISPANIC,
            _ => Race::INDIAN,
        }
    }
}

impl Distribution<PCType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PCType {
        match rng.gen_range(0..=1) {
            0 => PCType::PREBUILT,
            _ => PCType::CUSTOM,
        }
    }
}

impl Distribution<CodeQuality> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CodeQuality {
        match rng.gen_range(0..=2) {
            0 => CodeQuality::SHIT,
            1 => CodeQuality::MID,
            _ => CodeQuality::PERFECTION,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Bitches {
    Bitch(Bitch),
    AnnoyingBitch(AnnoyingBitch),
    CTTBitch(CTTBitch),
    CyberBitch(CyberBitch),
    EmoBitch(EmoBitch),
    ExGirlfriendBitch(ExGirlfriendBitch),
    FemboyBitch(FemboyBitch),
    KittenBitch(KittenBitch),
    SkidderBitch(SkidderBitch),
    WaifuBitch(WaifuBitch),
    Emily(Emily),
    Gami(Gami),
    GrowlyX(GrowlyX),
    Viet(Viet),
}

#[derive(Debug)]
pub(crate) struct Bitch {
    pub name: String,
    pub age: u8,
    pub race: Race,
    pub gender: Gender,
}

#[derive(Debug)]
pub(crate) struct AnnoyingBitch {
    pub bitch: Bitch,
    pub annoying_level: i32,
    pub pregnant: bool,
    pub fatherless: bool,
    pub transgender: bool,
}

#[derive(Debug)]
pub(crate) struct CTTBitch {
    pub bitch: Bitch,
    pub uses_fog_in_videos: bool,
    pub gets_pegged_by_woofina: bool,
    pub has_a_shitty_mic: bool,
}

#[derive(Debug)]
pub(crate) struct CyberBitch {
    pub bitch: Bitch,
    pub pc_type: PCType,
    pub gay: bool,
}

#[derive(Debug)]
pub(crate) struct EmoBitch {
    pub bitch: Bitch,
    pub has_barcode_wrists: bool,
    pub wears_black_clothes: bool,
}

#[derive(Debug)]
pub(crate) struct ExGirlfriendBitch {
    pub bitch: Bitch,
    pub cheated: bool,
    pub only_reason_i_would_take_them_back: String,
    pub body_count_after_me: u8,
}

#[derive(Debug)]
pub(crate) struct FemboyBitch {
    pub bitch: Bitch,
    pub hotness: i32,
    pub fatherless: bool,
    pub degenerate: bool,
}

#[derive(Debug)]
pub(crate) struct KittenBitch {
    pub bitch: FemboyBitch,
    pub does_as_they_are_told: bool,
    pub twitter_user: bool,
    pub twitter_username: String,
}

#[derive(Debug)]
pub(crate) struct SkidderBitch {
    pub bitch: EmoBitch,
    pub amount_of_code_stolen: i32,
    pub skidded_repositories: i32,
    pub disgusting_brackets: bool,
    pub poor_naming_conventions: bool,
}

#[derive(Debug)]
pub(crate) struct WaifuBitch {
    pub bitch: Bitch,
    pub wife_material: bool,
    pub sexual_nicknames: Vec<String>,
    pub favorite_anime: String,
}

#[derive(Debug)]
pub(crate) struct Emily {
    pub bitch: KittenBitch,
    pub email: String,
}

#[derive(Debug)]
pub(crate) struct Gami {
    pub bitch: AnnoyingBitch,
    pub owns_a_shit_server: bool,
}

#[derive(Debug)]
pub(crate) struct GrowlyX {
    pub bitch: SkidderBitch,
    pub code_quality: CodeQuality,
}

#[derive(Debug)]
pub(crate) struct Viet {
    pub bitch: EmoBitch,
    pub discord_tag: String,
    pub shit_pc_that_probably_will_blow_up_soon: PCType,
    pub irl_bitches: i32,
    pub ego_level: i32,
    pub doctors: Vec<MentalDoctor>,
}

impl Bitch {
    pub fn new() -> Self {
        Bitch {
            name: FirstName().fake(),
            age: thread_rng().gen_range(18..=48),
            race: random(),
            gender: random(),
        }
    }
}

impl AnnoyingBitch {
    pub fn new(annoying_level: i32, pregnant: bool, fatherless: bool, transgender: bool) -> Self {
        AnnoyingBitch {
            bitch: Bitch::new(),
            annoying_level,
            pregnant,
            fatherless,
            transgender,
        }
    }
}

impl CTTBitch {
    pub fn new(
        uses_fog_in_videos: bool,
        gets_pegged_by_woofina: bool,
        has_a_shitty_mic: bool,
    ) -> Self {
        CTTBitch {
            bitch: Bitch::new(),
            uses_fog_in_videos,
            gets_pegged_by_woofina,
            has_a_shitty_mic,
        }
    }
}

impl CyberBitch {
    pub fn new(pc_type: PCType, gay: bool) -> Self {
        CyberBitch {
            bitch: Bitch::new(),
            pc_type,
            gay,
        }
    }
}

impl EmoBitch {
    pub fn new(has_barcode_wrists: bool, wears_black_clothes: bool) -> Self {
        EmoBitch {
            bitch: Bitch::new(),
            has_barcode_wrists,
            wears_black_clothes,
        }
    }
}

impl ExGirlfriendBitch {
    pub fn new(cheated: bool, only_reason_i_would_take_them_back: String) -> Self {
        ExGirlfriendBitch {
            bitch: Bitch::new(),
            cheated,
            only_reason_i_would_take_them_back,
            body_count_after_me: 0,
        }
    }
}

impl FemboyBitch {
    pub fn new(fatherless: bool, degenerate: bool) -> Self {
        FemboyBitch {
            bitch: Bitch::new(),
            hotness: -i32::MAX,
            fatherless,
            degenerate,
        }
    }
}

impl KittenBitch {
    pub fn new(does_as_they_are_told: bool, twitter_user: bool) -> Self {
        KittenBitch {
            bitch: FemboyBitch::new(true, false),
            does_as_they_are_told,
            twitter_user,
            twitter_username: (if twitter_user { "Noxiuam" } else { "" }).to_string(),
        }
    }
}

impl SkidderBitch {
    pub fn new(
        amount_of_code_stolen: i32,
        skidded_repositories: i32,
        disgusting_brackets: bool,
        poor_naming_conventions: bool,
    ) -> Self {
        SkidderBitch {
            bitch: EmoBitch::new(true, true),
            amount_of_code_stolen,
            skidded_repositories,
            disgusting_brackets,
            poor_naming_conventions,
        }
    }
}

impl WaifuBitch {
    pub fn new(wife_material: bool, sexual_nicknames: Vec<String>, favorite_anime: String) -> Self {
        WaifuBitch {
            bitch: Bitch::new(),
            wife_material,
            sexual_nicknames,
            favorite_anime,
        }
    }
}

impl Emily {
    pub fn new(does_as_they_are_told: bool, twitter_user: bool) -> Self {
        Emily {
            bitch: KittenBitch::new(does_as_they_are_told, twitter_user),
            email: "cum@cum.com".to_string(),
        }
    }
}

impl Gami {
    pub fn new(owns_a_shit_server: bool) -> Self {
        Gami {
            bitch: AnnoyingBitch::new(1000, false, true, true),
            owns_a_shit_server,
        }
    }
}

impl GrowlyX {
    pub fn new(code_quality: CodeQuality) -> Self {
        GrowlyX {
            bitch: SkidderBitch::new(i32::MAX, 563, true, true),
            code_quality,
        }
    }
}

impl Viet {
    pub fn new(
        discord_tag: String,
        shit_pc_that_probably_will_blow_up_soon: PCType,
        irl_bitches: i32,
        ego_level: i32,
        doctors: Vec<MentalDoctor>,
        faggot: bool,
        has_barcode_wrists: bool,
    ) -> Self {
        Viet {
            bitch: EmoBitch::new(has_barcode_wrists, faggot),
            discord_tag,
            shit_pc_that_probably_will_blow_up_soon,
            irl_bitches,
            ego_level,
            doctors,
        }
    }
}
