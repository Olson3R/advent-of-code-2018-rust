#[macro_use]
extern crate lazy_static;
extern crate advent_of_code_2018_rust;
extern crate regex;

use regex::Regex;
use std::collections::HashMap;

use advent_of_code_2018_rust::util::Util;

lazy_static! {
  static ref ROW_REGEX: Regex = Regex::new(r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?:Guard \#(?P<guard_id>\d+) )?(?P<action>.*)").unwrap();
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Action {
    Begin,
    Asleep,
    Awake,
}

impl Action {
    fn parse_str_to_action(action_str: &str) -> Result<Action, ()> {
        match action_str {
            "begins shift" => Ok(Action::Begin),
            "falls asleep" => Ok(Action::Asleep),
            "wakes up" => Ok(Action::Awake),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Row {
    minute: i32,
    guard_id: Option<i32>,
    action: Action,
}

impl Row {
    pub fn get_action(&self) -> Action {
        self.action.clone()
    }

    pub fn get_guard_id(&self) -> Option<i32> {
        self.guard_id
    }
    pub fn get_minute(&self) -> i32 {
        self.minute
    }
}

fn main() {
    let mut lines = Util::load_input("input/d4-input.txt").expect("Failed to read the input file.");
    lines.sort();
    let rows: Vec<Row> = lines
        .into_iter()
        .filter_map(|line| {
            if let Some(captures) = ROW_REGEX.captures(&line) {
                let minute = captures["minute"]
                    .parse::<i32>()
                    .expect("Couldn't parse minute.");
                let guard_id =
                    captures
                        .name("guard_id")
                        .and_then(|id| match id.as_str().parse::<i32>() {
                            Ok(id) => Some(id),
                            Err(_) => None,
                        });
                let action = Action::parse_str_to_action(&captures["action"])
                    .expect("Couldn't parse action.");
                return Some(Row {
                    minute: minute,
                    guard_id: guard_id,
                    action: action,
                });
            }
            None
        })
        .collect();

    let guard_sleep_map: HashMap<i32, HashMap<i32, i32>> = rows
        .into_iter()
        .fold((0, (Action::Begin, 0), HashMap::new()), |mut acc, row| {
            if let Some(guard_id) = row.get_guard_id() {
                acc.0 = guard_id;
            }

            {
                let cache = acc.2.entry(acc.0).or_insert_with(HashMap::new);
                let previous_action = acc.1;
                if let (Action::Asleep, Action::Awake) = (previous_action.0, row.get_action()) {
                    for minute in previous_action.1..row.get_minute() {
                        let counter = cache.entry(minute).or_insert(0);
                        *counter += 1;
                    }
                }
            }

            acc.1 = (row.get_action(), row.get_minute());
            acc
        })
        .2;
    let sleepy_guard: (i32, HashMap<i32, i32>) = guard_sleep_map
        .into_iter()
        .max_by(|(_, a), (_, b)| a.values().max().cmp(&b.values().max()))
        .expect("Couldn't get sleepy guard.");

    let sleepy_guard_minute: (i32, i32) = sleepy_guard
        .1
        .clone()
        .into_iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .expect("Couldn't get sleepy guard minute.");
    println!("{:?}", sleepy_guard.0 * sleepy_guard_minute.0);
}
