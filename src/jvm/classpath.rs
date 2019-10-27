/*
 * FILE: XXXXX
 * DESCRIPTION:
 *
 * Copyright (c) 2019, Will Hawkins
 *
 * This file is part of Rust-JVM.
 *
 * Rust-JVM is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Rust-JVM is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Rust-JVM.  If not, see <https://www.gnu.org/licenses/>.
 */

use jvm::class::Class;
use jvm::debug::Debug;
use jvm::debug::DebugLevel;
use jvm::error::FatalError;
use jvm::error::FatalErrorType;
use rjar::Jar;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub enum ClassLocation {
	ClassFile(String),
	JarFile(String, String),
}

pub struct ClassPath {
	classes: HashMap<String, ClassLocation>,
	debug_level: DebugLevel,
}

fn class_name_from_class_file(class_file: &String) -> Option<String> {
	if let Some(class) = Class::load_from_file(class_file) {
		class.get_class_name()
	} else {
		None
	}
}

impl ClassPath {
	pub fn class_location_for_class(&self, class: &str) -> Option<ClassLocation> {
		if let Some(class) = self.classes.get(class) {
			Some(class.clone())
		} else {
			None
		}
	}
	pub fn new(classpath: &[&str], debug_level: DebugLevel) -> Self {
		let mut classes = HashMap::<String, ClassLocation>::new();
		for path in classpath {
			if let Ok(dir_list) = fs::read_dir(Path::new(&path)) {
				for dir in dir_list {
					if let Ok(path_entry) = dir {
						if let Some(path_entry) = path_entry.path().to_str() {
							let path_entry_file = path_entry.to_string();
							if path_entry_file.ends_with("class") {
								Debug(
									format!("Loading class file {}", path_entry_file),
									&debug_level,
									DebugLevel::Info,
								);
								/*
								 * TODO: This has to peak at the head of the file to get the
								 * classname
								 */
								if let Some(class_name) =
									class_name_from_class_file(&path_entry_file)
								{
									Debug(
										format!(
											"{} classpath entry contains object {}.",
											path_entry_file, class_name
										),
										&debug_level,
										DebugLevel::Info,
									);
									classes.insert(
										class_name,
										ClassLocation::ClassFile(path_entry_file),
									);
								} else {
									Debug(
										format!("Could not load a name for {}.", path_entry_file),
										&debug_level,
										DebugLevel::Info,
									);
								}
							} else if path_entry_file.ends_with("jar") {
								Debug(
									format!("Loading JAR file {}", path_entry_file),
									&debug_level,
									DebugLevel::Info,
								);

								match Jar::open(&path_entry_file.clone()) {
									Ok(mut jar) => {
										for file in jar.file_names() {
											/*
											 * Skip class files.
											 */
											if !file.ends_with("class") {
												continue;
											}
											if let Ok(file_bytes) = jar.file_contents_by_name(&file)
											{
												if let Some(class) =
													Class::load_from_bytes(file_bytes)
												{
													if let Some(class_name) = class.get_class_name()
													{
														classes.insert(
															class_name,
															ClassLocation::JarFile(
																path_entry_file.clone(),
																file.clone(),
															),
														);
													}
												} else {
													Debug(
														format!(
														"Could not read a class from bytes from {}:{}.",
														path_entry_file, file
													),
														&debug_level,
														DebugLevel::Warning,
													);
												}
											} else {
												Debug(
													format!(
														"Could not read a class from {}:{}.",
														path_entry_file, file
													),
													&debug_level,
													DebugLevel::Warning,
												);
											}
										}
									}
									Err(_e) => {}
								};

								/*
								 * TODO: This has to use *something* from the JAR file to figure
								 * out which classes are in this JAR file.
								for f in xxxx {
									classes.insert(f, path_entry_file);
								}
								 */
							}
						}
					}
				}
			}
		}
		ClassPath {
			classes,
			debug_level,
		}
	}
}
