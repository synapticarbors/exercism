use std::collections::HashMap;

type Stats = HashMap<String, TeamStats>;
const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Debug, Default)]
struct TeamStats {
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

fn stats2tally(s: &Stats) -> String {
    let mut out = HEADER.to_string();

    let mut x: Vec<(String, &TeamStats)> = s.iter().map(|(k, v)| (k.to_owned(), v)).collect();
    x.sort_by(|(ka, va), (kb, vb)| vb.points.cmp(&va.points).then(ka.cmp(&kb)));

    for (team_name, ts) in x.iter() {
        out.push_str(&format!(
            "\n{:31}| {:^3}| {:^3}| {:^3}| {:^3}| {:>2}",
            team_name, ts.matches_played, ts.wins, ts.draws, ts.losses, ts.points
        ));
    }

    out
}

pub fn tally(match_results: &str) -> String {
    let stats: Stats = match_results.lines().fold(HashMap::new(), |mut acc, line| {
        let x: Vec<&str> = line.split(';').collect();
        let s = acc.entry(x[0].to_string()).or_default();
        s.matches_played += 1;
        match x[2] {
            "win" => {
                s.points += 3;
                s.wins += 1;
            }
            "draw" => {
                s.points += 1;
                s.draws += 1;
            }
            "loss" => s.losses += 1,
            _ => unreachable!(),
        }

        let s = acc.entry(x[1].to_string()).or_default();
        s.matches_played += 1;
        match x[2] {
            "win" => {
                s.losses += 1;
            }
            "draw" => {
                s.points += 1;
                s.draws += 1;
            }
            "loss" => {
                s.wins += 1;
                s.points += 3
            }
            _ => unreachable!(),
        }

        acc
    });

    stats2tally(&stats)
}
