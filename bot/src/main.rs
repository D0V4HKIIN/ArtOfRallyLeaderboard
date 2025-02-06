extern crate reqwest;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
fn main() {
    println!("Hello, world!");
    let area = Area::Finland;
    let stage = 1;
    let direction = Direction::Forward;
    let weather = Weather::Dry;
    let group = Group::Sixties;
    let filter = Filter::Friends;
    let platform = Platform::Steam;
    let entries = get_leaderboard_entries(
        &area,
        stage,
        &direction,
        &weather,
        &group,
        &filter,
        &platform,
        76561198230518420,
        &vec![76561198087789780, 76561198062269100],
    );

    println!("{:#?}", entries);

    // save_entries(&entries, &area, stage, &direction, &weather, &group);
    // println!(
    //     "{:#?}",
    //     read_entries(&area, stage, &direction, &weather, &group).unwrap()
    // );
}

fn stage_string(
    area: &Area,
    stage: usize, // should be 1,2,3,4,5,6
    direction: &Direction,
    weather: &Weather,
    group: &Group,
) -> String {
    let area_name = area.value();
    let direction_name = direction.value();
    let weather_name = weather.value();
    let group_name = group.value();

    return format!("{area_name}_Stage_{stage}_{direction_name}_{weather_name}_{group_name}");
}

fn get_leaderboard_entries(
    area: &Area,
    stage: usize, // should be 1,2,3,4,5,6
    direction: &Direction,
    weather: &Weather,
    group: &Group,
    filter: &Filter,
    platform: &Platform,
    user: u64,
    friends: &Vec<u64>,
) -> Vec<LeaderboardEntry> {
    let stage_string = stage_string(area, stage, direction, weather, group);
    let filter_index = filter.value();
    let platform_index = platform.value();

    let friend_list: String = format!("{:?}", friends).split_whitespace().collect();

    let body = reqwest::blocking::get(format!("https://www.funselektorfun.com/artofrally/leaderboard/{stage_string}/{filter_index}/{platform_index}/{user}/{friend_list}")).unwrap().text().unwrap();

    let result: Response = serde_json::from_str(&body).expect("response is not well-formated");

    return result.leaderboard;
}

fn save_entries(
    entries: &Vec<LeaderboardEntry>,
    area: &Area,
    stage: usize, // should be 1,2,3,4,5,6
    direction: &Direction,
    weather: &Weather,
    group: &Group,
) -> () {
    let json_entries = serde_json::to_string_pretty(&entries)
        .expect(&format!("failed to serialize entries {:?}", entries));

    let mut file = File::create(stage_string(area, stage, direction, weather, group))
        .expect("failed to create stage file");
    file.write_all(json_entries.as_bytes())
        .expect("failed to write to stage file");
}

fn read_entries(
    area: &Area,
    stage: usize, // should be 1,2,3,4,5,6
    direction: &Direction,
    weather: &Weather,
    group: &Group,
) -> Result<Vec<LeaderboardEntry>, serde_json::Error> {
    let file = File::open(stage_string(area, stage, direction, weather, group))
        .expect("can't open stage file");

    serde_json::from_reader(file)
}

// result will never be used but is needed for json parsing
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Response {
    leaderboard: Vec<LeaderboardEntry>,
    result: isize, // idk what this value does
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct LeaderboardEntry {
    uniqueID: u64, // unsure
    userName: String,
    rank: usize,
    score: usize,
    country: usize,
    carID: usize,
    replayData: String,
    platformID: u8,
}

enum Area {
    Finland,
    Sardinia,
    Japan,
    Norway,
    Germany,
    Kenya,
}

impl Area {
    fn value(&self) -> &str {
        match *self {
            Area::Finland => "Finland",
            Area::Sardinia => "Sardinia",
            Area::Japan => "Japan",
            Area::Norway => "Norway",
            Area::Germany => "Germany",
            Area::Kenya => "Kenya",
        }
    }
}

enum Direction {
    Forward,
    Backward,
}

impl Direction {
    fn value(&self) -> &str {
        match *self {
            Direction::Forward => "Forward",
            Direction::Backward => "Backward",
        }
    }
}

enum Weather {
    Dry,
    Wet,
}

impl Weather {
    fn value(&self) -> &str {
        match *self {
            Weather::Dry => "Dry",
            Weather::Wet => "Wet",
        }
    }
}

enum Group {
    Sixties,
    Seventies,
    Eighties,
    GroupB,
    GroupS,
    GroupA,
    BonusVans,
    BonusPiaggio,
    BonusDakar,
    BonusLogging,
}

impl Group {
    fn value(&self) -> &str {
        match *self {
            Group::Sixties => "60s",
            Group::Seventies => "70s",
            Group::Eighties => "80s",
            Group::GroupB => "GroupB",
            Group::GroupS => "GroupS",
            Group::GroupA => "GroupA",
            Group::BonusVans => "Bonus_Vans",
            Group::BonusPiaggio => "Bonus_Monkey",
            Group::BonusDakar => "Bonus_Dakar",
            Group::BonusLogging => "Bonus_Logging",
        }
    }
}

// idk what i am doing with all theses numbers :))
enum Filter {
    Top,
    AroundMe,
    Number,
    Count,
    PlayerRank,
    NumberOne,
    Friends,
    OnlyMe,
}

impl Filter {
    fn value(&self) -> usize {
        match *self {
            Filter::Top => 0,
            Filter::AroundMe => 1,
            Filter::Number => 2,
            Filter::Count => 3,
            Filter::PlayerRank => 4,
            Filter::NumberOne => 5,
            Filter::Friends => 6,
            Filter::OnlyMe => 7,
        }
    }
}

// realistically we only care about steam
#[allow(dead_code)]
enum Platform {
    Epic = 0,
    Gog = 1,
    Steam = 2,
    Xbox = 3,
    Playstation = 4,
    Nintendo = 5,
    None = 6,
}

impl Platform {
    fn value(&self) -> usize {
        match *self {
            Platform::Epic => 0,
            Platform::Gog => 1,
            Platform::Steam => 2,
            Platform::Xbox => 3,
            Platform::Playstation => 4,
            Platform::Nintendo => 5,
            Platform::None => 6,
        }
    }
}
