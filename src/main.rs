use rand::seq::SliceRandom;
use std::{thread, time};
use std::io::{self, Write};
use colored::Colorize;
use clap::Parser;

/// A colourful one-arm bandit that recommends a backend language.
#[derive(Parser)]
#[command(name = "backend-slot",    // binary name
          version,                  // clap will fill in the package version
          about  = "Spin the slot machine and pick your next backend language.",
          long_about = None)]
struct Cli { /* No options for now. Can add --fast etc. in the future */ }

fn main() {
    // ----- CLI parsing (--help/--version are handled here) -----
    let _args = Cli::parse();
    let languages = vec![
        ("Rust", "red"),
        ("Go", "cyan"),
        ("Python", "blue"),
        ("Node.js", "green"),
        ("Java", "yellow"),
        ("C#", "magenta"),
        ("PHP", "purple"),
        ("Ruby", "red"),
        ("Elixir", "magenta"),
        ("Kotlin", "orange"),
    ];

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
    let chosen_lang = languages.choose(&mut rng).unwrap().0;
    
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
    let mut top_fixed    = [None::<&str>; 3];
    let mut bottom_fixed = [None::<&str>; 3];

    // Spin the slot machine
    for i in 0..60 {
        // Check if each column has stopped
        let left_stopped  = i >= 40;
        let mid_stopped   = i >= 50;
        let right_stopped = i >= 59;

        // Middle row (row 2) always shows chosen_lang when column stops
        let l2 = if left_stopped  { chosen_lang } else { languages.choose(&mut rng).unwrap().0 };
        let m2 = if mid_stopped   { chosen_lang } else { languages.choose(&mut rng).unwrap().0 };
        let r2 = if right_stopped { chosen_lang } else { languages.choose(&mut rng).unwrap().0 };

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
        let l1 = if left_stopped  { top_fixed[0].unwrap() } else { languages.choose(&mut rng).unwrap().0 };
        let m1 = if mid_stopped   { top_fixed[1].unwrap() } else { languages.choose(&mut rng).unwrap().0 };
        let r1 = if right_stopped { top_fixed[2].unwrap() } else { languages.choose(&mut rng).unwrap().0 };

        // === Row 3 ===
        let l3 = if left_stopped  { bottom_fixed[0].unwrap() } else { languages.choose(&mut rng).unwrap().0 };
        let m3 = if mid_stopped   { bottom_fixed[1].unwrap() } else { languages.choose(&mut rng).unwrap().0 };
        let r3 = if right_stopped { bottom_fixed[2].unwrap() } else { languages.choose(&mut rng).unwrap().0 };

        // Draw all 3 rows together
        print!("\r{v}");                     // -- Row 1 --
        print_colored(l1, language_color(l1, &languages), 10);
        print!("{v}");
        print_colored(m1, language_color(m1, &languages), 10);
        print!("{v}");
        print_colored(r1, language_color(r1, &languages), 10);
        println!("{v}");

        print!("{v}");                       // -- Row 2 --
        print_colored(l2, language_color(l2, &languages), 10);
        print!("{v}");
        print_colored(m2, language_color(m2, &languages), 10);
        print!("{v}");
        print_colored(r2, language_color(r2, &languages), 10);
        println!("{v}");

        print!("{v}");                       // -- Row 3 --
        print_colored(l3, language_color(l3, &languages), 10);
        print!("{v}");
        print_colored(m3, language_color(m3, &languages), 10);
        print!("{v}");
        print_colored(r3, language_color(r3, &languages), 10);
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
    let color = language_color(chosen_lang, &languages);
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
    print_colored_line(chosen_lang, color);
    println!(" for your next backend project!");
}

// Return a random language other than the chosen_lang
fn random_other<'a>(
    rng: &mut rand::rngs::ThreadRng,
    exclude: &str,
    langs: &Vec<(&'a str, &'a str)>,
) -> &'a str {
    loop {
        let cand = langs.choose(rng).unwrap().0;
        if cand != exclude {
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

fn language_color<'a>(lang: &str, languages: &Vec<(&'a str, &'a str)>) -> &'a str {
    for (name, color) in languages {
        if *name == lang {
            return color;
        }
    }
    "white"
}
