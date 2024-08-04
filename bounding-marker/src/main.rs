/* main.rs
 *
 * Copyright 2024 Diego Iván M.E <diegoivan.mae@gmail.com>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

mod format_registry;

use std::{ffi::OsString, path::{PathBuf, Path}};

use clap::{Args, Parser, Subcommand, ValueEnum};
use format_registry::FormatRegistry;
use labelswap_data::{conversion_pipeline::ConversionPipeline, models::Format, transforms::RequiredTransformations};

#[derive(Debug, Parser)]
#[command(name="bounding-marker")]
#[command(about="Convert computer vision annotation formats to others")]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    print_formats: Option<bool>,

    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Convert {
        #[arg(long)]
        source_format: String,
        #[arg(long)]
        target_format: String,

        #[arg(long)]
        image_directory: Option<PathBuf>,
        #[arg(long)]
        class_mappings: Option<PathBuf>,

        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        target: PathBuf,
    }
}

fn main() -> Result<(), String> {
    let registry = FormatRegistry::new();
    let args = Cli::parse();

    if let Some(print_formats) = args.print_formats {
        if print_formats {
            print_formats_in_registry(&registry);
            return Ok(())
        }
    }

    match args.commands {
        Commands::Convert { 
            source_format, 
            target_format, 
            image_directory, 
            class_mappings, 
            source, 
            target 
        } => {
            handle_convert(
                source_format, 
                target_format, 
                image_directory, 
                class_mappings, 
                source, 
                target,
                &registry
            )?;
        }
    }
    Ok(())
}

fn handle_convert(
    source_format: String,
    target_format: String,
    image_directory: Option<PathBuf>,
    class_mappings: Option<PathBuf>,
    source: PathBuf,
    target: PathBuf,
    registry: &FormatRegistry,
) -> Result<(), String> {
    let source_format = match registry.lookup_format(&source_format) {
        Some(format) => format,
        None => return Err(format!("{source_format} is not a valid format")),
    };
    let target_format = match registry.lookup_format(&target_format) {
        Some(format) => format,
        None => return Err(format!("{target_format} is not a valid format")),
    };

    let mut pipeline = ConversionPipeline::new(source_format, target_format);
    pipeline.set_image_directory(image_directory);
    Ok(())
}

fn print_formats_in_registry(registry: &FormatRegistry) {
}