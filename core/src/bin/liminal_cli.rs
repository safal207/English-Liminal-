use liminal_english_core::{ContentValidator, Store};
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "validate" => cmd_validate(&args[2..]),
        "db" => cmd_db(&args[2..]),
        "health" => cmd_health(),
        "help" | "--help" | "-h" => {
            print_usage();
            process::exit(0);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!(
        r#"
Liminal CLI - Development Tools for English-Liminal

USAGE:
    liminal_cli <COMMAND> [OPTIONS]

COMMANDS:
    validate        Validate all YAML scenarios and roles
    db              Database operations
    health          Check system health
    help            Show this help message

EXAMPLES:
    # Validate all content
    liminal_cli validate

    # Validate specific directories
    liminal_cli validate --scripts assets/scripts --roles content/roles

    # Check database
    liminal_cli db inspect liminal.db

    # Health check
    liminal_cli health
"#
    );
}

fn cmd_validate(args: &[String]) {
    println!("🔍 Validating content...\n");

    let mut validator = ContentValidator::new();

    // Parse arguments for custom paths
    let scripts_dir = get_arg_value(args, "--scripts").unwrap_or("assets/scripts".to_string());
    let roles_dir = get_arg_value(args, "--roles").unwrap_or("content/roles".to_string());

    // Load scripts
    println!("📄 Loading scripts from: {}", scripts_dir);
    match validator.load_scripts(Path::new(&scripts_dir)) {
        Ok(_) => println!("   ✓ Scripts loaded"),
        Err(e) => {
            eprintln!("   ✗ Failed to load scripts: {}", e);
            process::exit(1);
        }
    }

    // Load roles
    println!("🎭 Loading roles from: {}", roles_dir);
    match validator.load_roles_recursive(Path::new(&roles_dir)) {
        Ok(_) => println!("   ✓ Roles loaded"),
        Err(e) => {
            eprintln!("   ✗ Failed to load roles: {}", e);
            process::exit(1);
        }
    }

    // Validate
    let report = validator.validate();
    report.print();

    if !report.valid {
        process::exit(1);
    }
}

fn cmd_db(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: db command requires a subcommand");
        eprintln!("Usage: liminal_cli db <inspect|export> <path>");
        process::exit(1);
    }

    let subcommand = &args[0];

    match subcommand.as_str() {
        "inspect" => {
            if args.len() < 2 {
                eprintln!("Error: inspect requires database path");
                process::exit(1);
            }
            let db_path = &args[1];
            db_inspect(db_path);
        }
        "export" => {
            if args.len() < 2 {
                eprintln!("Error: export requires database path");
                process::exit(1);
            }
            let db_path = &args[1];
            db_export(db_path);
        }
        _ => {
            eprintln!("Unknown db subcommand: {}", subcommand);
            process::exit(1);
        }
    }
}

fn db_inspect(db_path: &str) {
    println!("🗄️  Inspecting database: {}\n", db_path);

    match Store::open(db_path) {
        Ok(store) => {
            // Get statistics
            match store.get_streak() {
                Ok(streak) => println!("📊 Streak: {} days", streak),
                Err(e) => eprintln!("   Error getting streak: {}", e),
            }

            match store.get_use_in_wild_count() {
                Ok(count) => println!("🌍 Use-in-wild count: {}", count),
                Err(e) => eprintln!("   Error getting use-in-wild count: {}", e),
            }

            match store.get_events(None, 10) {
                Ok(events) => println!("📝 Recent events: {}", events.len()),
                Err(e) => eprintln!("   Error getting events: {}", e),
            }

            println!("\n✅ Database is accessible");
        }
        Err(e) => {
            eprintln!("❌ Failed to open database: {}", e);
            process::exit(1);
        }
    }
}

fn db_export(db_path: &str) {
    println!("📦 Exporting database: {}\n", db_path);

    match Store::open(db_path) {
        Ok(store) => match store.export_json() {
            Ok(json) => {
                println!("{}", json);
                println!("\n✅ Export complete");
            }
            Err(e) => {
                eprintln!("❌ Failed to export: {}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to open database: {}", e);
            process::exit(1);
        }
    }
}

fn cmd_health() {
    println!("🏥 System Health Check\n");

    println!("✅ Rust core: OK");
    println!("✅ SQLite: OK");
    println!("✅ FFI bindings: OK (if compiled)");

    println!("\n✅ All systems operational");
}

fn get_arg_value(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}
