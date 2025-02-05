extern crate reqwest;

fn main() {
    println!("Hello, world!");
    get_leaderboard_entry(
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
}

fn get_leaderboard_entry(
    area: Area,
    stage: usize, // should be 1,2,3,4,5,6
    direction: Direction,
    weather: Weather,
    group: Group,
    filter: Filter,
    platform: Platform,
    user: u64,
    friends: Vec<u64>,
) -> () {
    let area_name = area.value();
    let direction_name = direction.value();
    let weather_name = weather.value();
    let group_name = group.value();
    let filter_index = filter as isize;
    let platform_index = platform as isize;
    println!("{}", filter_index);
    println!("{}", platform_index);
    println!("https://www.funselektorfun.com/artofrally/leaderboard/{area_name}_Stage_{stage}_{direction_name}_{weather_name}_{group_name}/{filter_index}/{platform_index}/{user}");

    let body = reqwest::blocking::get(format!("https://www.funselektorfun.com/artofrally/leaderboard/{area_name}_Stage_{stage}_{direction_name}_{weather_name}_{group_name}/{filter_index}/{platform_index}/{user}"));

    println!("{:#?}", body.unwrap().text());
}

struct LeaderboardEntry {
    unique_id: u64, // unsure
    user_name: String,
    rank: usize,
    score: usize,
    country: usize,
    car_id: usize,
    replay_data: String,
    platform_id: Platform,
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
