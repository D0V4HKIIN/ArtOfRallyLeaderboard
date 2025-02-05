extern crate reqwest;
use serde::Deserialize;
fn main() {
    println!("Hello, world!");
    let entries = get_leaderboard_entries(
        Area::Finland,
        1,
        Direction::Forward,
        Weather::Dry,
        Group::Sixties,
        Filter::OnlyMe,
        Platform::Steam,
        76561198230518420,
        vec![76561198087789780],
    );

    println!("{:#?}", entries);
}

fn get_leaderboard_entries(
    area: Area,
    stage: usize, // should be 1,2,3,4,5,6
    direction: Direction,
    weather: Weather,
    group: Group,
    filter: Filter,
    platform: Platform,
    user: u64,
    friends: Vec<u64>,
) -> Vec<LeaderboardEntry> {
    let area_name = area.value();
    let direction_name = direction.value();
    let weather_name = weather.value();
    let group_name = group.value();
    let filter_index = filter as isize;
    let platform_index = platform as isize;

    let body = reqwest::blocking::get(format!("https://www.funselektorfun.com/artofrally/leaderboard/{area_name}_Stage_{stage}_{direction_name}_{weather_name}_{group_name}/{filter_index}/{platform_index}/{user}")).unwrap().text().unwrap();

    let result: Response = serde_json::from_str(&body).expect("response is not well-formated");

    return result.leaderboard;
}

#[derive(Debug, Deserialize)]
struct Response {
    leaderboard: Vec<LeaderboardEntry>,
    result: isize, // idk what this value does
}

#[derive(Debug, Deserialize)]
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
    Top = 0,
    AroundMe = 1,
    Number = 2,
    Count = 3,
    PlayerRank = 4,
    NumberOne = 5,
    Friends = 6,
    OnlyMe = 7,
}

enum Platform {
    Epic = 0,
    Gog = 1,
    Steam = 2,
    Xbox = 3,
    Playstation = 4,
    Nintendo = 5,
    None = 6,
}
