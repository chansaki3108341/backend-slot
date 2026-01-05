use rand::seq::SliceRandom;
use std::{thread, time};
use std::io::{self, Write};
use std::path::PathBuf;
use colored::Colorize;
use clap::Parser;

mod config;
mod languages;

use languages::Language;

/// A colourful one-arm bandit that recommends a backend language.
#[derive(Parser)]
#[command(name = "backend-slot",    // binary name
          version,                  // clap will fill in the package version
          about  = "Spin the slot machine and pick your next backend language.",
          long_about = None)]
struct Cli {
    /// Load languages from a TOML config file.
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,
}

fn main() {
    // ----- CLI parsing (--help/--version are handled here) -----
    let args = Cli::parse();
    let languages: Vec<Language> = match args.config.as_deref() {
        Some(path) => match config::load_languages_from_file(path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("backend-slot: {}", e);
                std::process::exit(2);
            }
        },
        None => languages::builtin_languages(),
    };

    // ── Light-up Marquee ──────────────────────────────
    // match cabinet width (3×10 cells + 2 separators + 2 borders = 34)
    const CAB_WIDTH: usize = 34;

    let marquee_top =  format!("╔{}╗", "═".repeat(CAB_WIDTH - 2));
    let marquee_mid = format!("║{:^width$}║", "Backend Slot", width = CAB_WIDTH - 2);
    let marquee_bot = format!("╚{}╝", "═".repeat(CAB_WIDTH - 2));

    println!("{}", marquee_top.bright_yellow());
    println!("{}", marquee_mid.bright_yellow().bold());
    println!("{}", marquee_bot.bright_yellow());
    println!();
    
    let mut rng = rand::thread_rng();
    let short_duration = time::Duration::from_millis(50);
    let long_duration = time::Duration::from_millis(500);
    
    // Pre-determine the language that will be chosen
    let chosen_lang: &Language = languages
        .choose(&mut rng)
        .expect("builtin languages must be non-empty");
    
    // ── Slot Cabinet ─────────────────────────────────
    let v = "║".bright_yellow();            // yellow vertical bar (re-use everywhere)
    println!("{}", "╔══════════╦══════════╦══════════╗".bright_yellow());
    println!("{v}          {v}          {v}          {v}");
    println!("{v}          {v}          {v}          {v}");
    println!("{v}          {v}          {v}          {v}");
    println!("{}", "╚══════════╩══════════╩══════════╝".bright_yellow());

    // Display empty yellow frame
    let empty_frame_top =  format!("╔{}╗", "═".repeat(CAB_WIDTH - 2));
    let empty_frame_mid = format!("║{:^width$}║", "", width = CAB_WIDTH - 2);
    let empty_frame_bot =  format!("╚{}╝", "═".repeat(CAB_WIDTH - 2));
    println!("{}", empty_frame_top.bright_yellow());
    println!("{}", empty_frame_mid.bright_yellow());
    println!("{}", empty_frame_bot.bright_yellow());
    println!();
    
    // Move up 5 lines to the top reel position & hide cursor
    print!("\x1B[8A\x1B[?25l");
    io::stdout().flush().unwrap();
    
    // Languages to fix in top and bottom rows when each column stops
    let mut top_fixed: [Option<&Language>; 3] = [None, None, None];
    let mut bottom_fixed: [Option<&Language>; 3] = [None, None, None];

    // Spin the slot machine
    for i in 0..60 {
        // Check if each column has stopped
        let left_stopped  = i >= 40;
        let mid_stopped   = i >= 50;
        let right_stopped = i >= 59;

        // Middle row (row 2) always shows chosen_lang when column stops
        let l2: &Language = if left_stopped  { chosen_lang } else { languages.choose(&mut rng).unwrap() };
        let m2: &Language = if mid_stopped   { chosen_lang } else { languages.choose(&mut rng).unwrap() };
        let r2: &Language = if right_stopped { chosen_lang } else { languages.choose(&mut rng).unwrap() };

        // When a column stops, determine fixed values for top and bottom rows (different from chosen_lang)
        if left_stopped  && top_fixed[0].is_none() {
            top_fixed[0]    = Some(random_other(&mut rng, chosen_lang, &languages));
            bottom_fixed[0] = Some(random_other(&mut rng, chosen_lang, &languages));
        }
        if mid_stopped   && top_fixed[1].is_none() {
            top_fixed[1]    = Some(random_other(&mut rng, chosen_lang, &languages));
            bottom_fixed[1] = Some(random_other(&mut rng, chosen_lang, &languages));
        }
        if right_stopped && top_fixed[2].is_none() {
            top_fixed[2]    = Some(random_other(&mut rng, chosen_lang, &languages));
            bottom_fixed[2] = Some(random_other(&mut rng, chosen_lang, &languages));
        }

        // === Row 1 ===
        let l1: &Language = if left_stopped  { top_fixed[0].unwrap() } else { languages.choose(&mut rng).unwrap() };
        let m1: &Language = if mid_stopped   { top_fixed[1].unwrap() } else { languages.choose(&mut rng).unwrap() };
        let r1: &Language = if right_stopped { top_fixed[2].unwrap() } else { languages.choose(&mut rng).unwrap() };

        // === Row 3 ===
        let l3: &Language = if left_stopped  { bottom_fixed[0].unwrap() } else { languages.choose(&mut rng).unwrap() };
        let m3: &Language = if mid_stopped   { bottom_fixed[1].unwrap() } else { languages.choose(&mut rng).unwrap() };
        let r3: &Language = if right_stopped { bottom_fixed[2].unwrap() } else { languages.choose(&mut rng).unwrap() };

        // Draw all 3 rows together
        print!("\r{v}");                     // -- Row 1 --
        print_colored(l1.name.as_str(), l1.color.as_str(), 10);
        print!("{v}");
        print_colored(m1.name.as_str(), m1.color.as_str(), 10);
        print!("{v}");
        print_colored(r1.name.as_str(), r1.color.as_str(), 10);
        println!("{v}");

        print!("{v}");                       // -- Row 2 --
        print_colored(l2.name.as_str(), l2.color.as_str(), 10);
        print!("{v}");
        print_colored(m2.name.as_str(), m2.color.as_str(), 10);
        print!("{v}");
        print_colored(r2.name.as_str(), r2.color.as_str(), 10);
        println!("{v}");

        print!("{v}");                       // -- Row 3 --
        print_colored(l3.name.as_str(), l3.color.as_str(), 10);
        print!("{v}");
        print_colored(m3.name.as_str(), m3.color.as_str(), 10);
        print!("{v}");
        print_colored(r3.name.as_str(), r3.color.as_str(), 10);
        println!("{v}");

        io::stdout().flush().unwrap();

        if [39, 49].contains(&i) {
            thread::sleep(long_duration);
        } else {
            thread::sleep(short_duration);
        }

        // Move up 3 lines for the next frame
        print!("\x1B[3A");
    }
    
    // ── After loop, move below the bottom frame & show cursor ───
    print!("\x1B[8B\x1B[?25h");
    
    // ── JACKPOT Banner ──────────────────────────────
    let color = chosen_lang.color.as_str();
    let jp_top =  format!("╔{}╗", "═".repeat(CAB_WIDTH - 2));
    let jp_mid =  format!("║{:^width$}║", "JACKPOT!", width = CAB_WIDTH - 2);
    let jp_bot =  format!("╚{}╝", "═".repeat(CAB_WIDTH - 2));

    // Move up 4 lines to overwrite with JACKPOT display
    print!("\x1B[4A");
    println!("{}", jp_top.bright_magenta());
    println!("{}", jp_mid.bright_magenta().bold());
    println!("{}", jp_bot.bright_magenta());
    println!();

    print!("Use ");
    print_colored_line(chosen_lang.name.as_str(), color);
    println!(" for your next backend project!");
}

// Return a random language other than the chosen_lang
fn random_other<'a>(
    rng: &mut rand::rngs::ThreadRng,
    exclude: &'a Language,
    langs: &'a [Language],
) -> &'a Language {
    // Degenerate case guard: if there's no alternative, return exclude to avoid infinite loop.
    if langs.iter().all(|l| l.name == exclude.name) {
        return exclude;
    }

    loop {
        let cand = langs.choose(rng).expect("language list must be non-empty");
        if cand.name != exclude.name {
            return cand;
        }
    }
}

fn print_colored(text: &str, color: &str, width: usize) {
    // Format with consistent width
    let padded = format!("{:^width$}", text, width = width);
    match color {
        "red"     => print!("{}", padded.red()),
        "blue"    => print!("{}", padded.blue()),
        "green"   => print!("{}", padded.green()),
        "yellow"  => print!("{}", padded.yellow()),
        "magenta" => print!("{}", padded.magenta()),
        "cyan"    => print!("{}", padded.cyan()),
        "purple"  => print!("{}", padded.purple()),
        "orange"  => print!("{}", padded.truecolor(255, 165, 0)),
        _         => print!("{}", padded),
    }
    // Flush immediately to ensure stable rendering
    io::stdout().flush().unwrap();
}

fn print_colored_line(text: &str, color: &str) {
    match color {
        "red" => print!("{}", text.red()),
        "blue" => print!("{}", text.blue()),
        "green" => print!("{}", text.green()),
        "yellow" => print!("{}", text.yellow()),
        "magenta" => print!("{}", text.magenta()),
        "cyan" => print!("{}", text.cyan()),
        "purple" => print!("{}", text.purple()),
        "orange" => print!("{}", text.truecolor(255, 165, 0)),
        _ => print!("{}", text),
    }
}
