// Generated by gir (https://github.com/gtk-rs/gir @ f2739a9)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 6088bb6)
// DO NOT EDIT

use libadwaita_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["libadwaita-1"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "AdwActionRow",
        Layout {
            size: size_of::<AdwActionRow>(),
            alignment: align_of::<AdwActionRow>(),
        },
    ),
    (
        "AdwActionRowClass",
        Layout {
            size: size_of::<AdwActionRowClass>(),
            alignment: align_of::<AdwActionRowClass>(),
        },
    ),
    (
        "AdwApplicationWindow",
        Layout {
            size: size_of::<AdwApplicationWindow>(),
            alignment: align_of::<AdwApplicationWindow>(),
        },
    ),
    (
        "AdwApplicationWindowClass",
        Layout {
            size: size_of::<AdwApplicationWindowClass>(),
            alignment: align_of::<AdwApplicationWindowClass>(),
        },
    ),
    (
        "AdwAvatarClass",
        Layout {
            size: size_of::<AdwAvatarClass>(),
            alignment: align_of::<AdwAvatarClass>(),
        },
    ),
    (
        "AdwBin",
        Layout {
            size: size_of::<AdwBin>(),
            alignment: align_of::<AdwBin>(),
        },
    ),
    (
        "AdwBinClass",
        Layout {
            size: size_of::<AdwBinClass>(),
            alignment: align_of::<AdwBinClass>(),
        },
    ),
    (
        "AdwCarouselClass",
        Layout {
            size: size_of::<AdwCarouselClass>(),
            alignment: align_of::<AdwCarouselClass>(),
        },
    ),
    (
        "AdwCarouselIndicatorDotsClass",
        Layout {
            size: size_of::<AdwCarouselIndicatorDotsClass>(),
            alignment: align_of::<AdwCarouselIndicatorDotsClass>(),
        },
    ),
    (
        "AdwCarouselIndicatorLinesClass",
        Layout {
            size: size_of::<AdwCarouselIndicatorLinesClass>(),
            alignment: align_of::<AdwCarouselIndicatorLinesClass>(),
        },
    ),
    (
        "AdwCenteringPolicy",
        Layout {
            size: size_of::<AdwCenteringPolicy>(),
            alignment: align_of::<AdwCenteringPolicy>(),
        },
    ),
    (
        "AdwClampClass",
        Layout {
            size: size_of::<AdwClampClass>(),
            alignment: align_of::<AdwClampClass>(),
        },
    ),
    (
        "AdwClampLayoutClass",
        Layout {
            size: size_of::<AdwClampLayoutClass>(),
            alignment: align_of::<AdwClampLayoutClass>(),
        },
    ),
    (
        "AdwClampScrollableClass",
        Layout {
            size: size_of::<AdwClampScrollableClass>(),
            alignment: align_of::<AdwClampScrollableClass>(),
        },
    ),
    (
        "AdwComboRow",
        Layout {
            size: size_of::<AdwComboRow>(),
            alignment: align_of::<AdwComboRow>(),
        },
    ),
    (
        "AdwComboRowClass",
        Layout {
            size: size_of::<AdwComboRowClass>(),
            alignment: align_of::<AdwComboRowClass>(),
        },
    ),
    (
        "AdwEnumListModelClass",
        Layout {
            size: size_of::<AdwEnumListModelClass>(),
            alignment: align_of::<AdwEnumListModelClass>(),
        },
    ),
    (
        "AdwEnumValueObjectClass",
        Layout {
            size: size_of::<AdwEnumValueObjectClass>(),
            alignment: align_of::<AdwEnumValueObjectClass>(),
        },
    ),
    (
        "AdwExpanderRow",
        Layout {
            size: size_of::<AdwExpanderRow>(),
            alignment: align_of::<AdwExpanderRow>(),
        },
    ),
    (
        "AdwExpanderRowClass",
        Layout {
            size: size_of::<AdwExpanderRowClass>(),
            alignment: align_of::<AdwExpanderRowClass>(),
        },
    ),
    (
        "AdwFlapClass",
        Layout {
            size: size_of::<AdwFlapClass>(),
            alignment: align_of::<AdwFlapClass>(),
        },
    ),
    (
        "AdwFlapFoldPolicy",
        Layout {
            size: size_of::<AdwFlapFoldPolicy>(),
            alignment: align_of::<AdwFlapFoldPolicy>(),
        },
    ),
    (
        "AdwFlapTransitionType",
        Layout {
            size: size_of::<AdwFlapTransitionType>(),
            alignment: align_of::<AdwFlapTransitionType>(),
        },
    ),
    (
        "AdwHeaderBarClass",
        Layout {
            size: size_of::<AdwHeaderBarClass>(),
            alignment: align_of::<AdwHeaderBarClass>(),
        },
    ),
    (
        "AdwLeafletClass",
        Layout {
            size: size_of::<AdwLeafletClass>(),
            alignment: align_of::<AdwLeafletClass>(),
        },
    ),
    (
        "AdwLeafletPageClass",
        Layout {
            size: size_of::<AdwLeafletPageClass>(),
            alignment: align_of::<AdwLeafletPageClass>(),
        },
    ),
    (
        "AdwLeafletTransitionType",
        Layout {
            size: size_of::<AdwLeafletTransitionType>(),
            alignment: align_of::<AdwLeafletTransitionType>(),
        },
    ),
    (
        "AdwNavigationDirection",
        Layout {
            size: size_of::<AdwNavigationDirection>(),
            alignment: align_of::<AdwNavigationDirection>(),
        },
    ),
    (
        "AdwPreferencesGroup",
        Layout {
            size: size_of::<AdwPreferencesGroup>(),
            alignment: align_of::<AdwPreferencesGroup>(),
        },
    ),
    (
        "AdwPreferencesGroupClass",
        Layout {
            size: size_of::<AdwPreferencesGroupClass>(),
            alignment: align_of::<AdwPreferencesGroupClass>(),
        },
    ),
    (
        "AdwPreferencesPage",
        Layout {
            size: size_of::<AdwPreferencesPage>(),
            alignment: align_of::<AdwPreferencesPage>(),
        },
    ),
    (
        "AdwPreferencesPageClass",
        Layout {
            size: size_of::<AdwPreferencesPageClass>(),
            alignment: align_of::<AdwPreferencesPageClass>(),
        },
    ),
    (
        "AdwPreferencesRow",
        Layout {
            size: size_of::<AdwPreferencesRow>(),
            alignment: align_of::<AdwPreferencesRow>(),
        },
    ),
    (
        "AdwPreferencesRowClass",
        Layout {
            size: size_of::<AdwPreferencesRowClass>(),
            alignment: align_of::<AdwPreferencesRowClass>(),
        },
    ),
    (
        "AdwPreferencesWindow",
        Layout {
            size: size_of::<AdwPreferencesWindow>(),
            alignment: align_of::<AdwPreferencesWindow>(),
        },
    ),
    (
        "AdwPreferencesWindowClass",
        Layout {
            size: size_of::<AdwPreferencesWindowClass>(),
            alignment: align_of::<AdwPreferencesWindowClass>(),
        },
    ),
    (
        "AdwSqueezerClass",
        Layout {
            size: size_of::<AdwSqueezerClass>(),
            alignment: align_of::<AdwSqueezerClass>(),
        },
    ),
    (
        "AdwSqueezerPageClass",
        Layout {
            size: size_of::<AdwSqueezerPageClass>(),
            alignment: align_of::<AdwSqueezerPageClass>(),
        },
    ),
    (
        "AdwSqueezerTransitionType",
        Layout {
            size: size_of::<AdwSqueezerTransitionType>(),
            alignment: align_of::<AdwSqueezerTransitionType>(),
        },
    ),
    (
        "AdwStatusPageClass",
        Layout {
            size: size_of::<AdwStatusPageClass>(),
            alignment: align_of::<AdwStatusPageClass>(),
        },
    ),
    (
        "AdwSwipeGroupClass",
        Layout {
            size: size_of::<AdwSwipeGroupClass>(),
            alignment: align_of::<AdwSwipeGroupClass>(),
        },
    ),
    (
        "AdwSwipeTrackerClass",
        Layout {
            size: size_of::<AdwSwipeTrackerClass>(),
            alignment: align_of::<AdwSwipeTrackerClass>(),
        },
    ),
    (
        "AdwSwipeableInterface",
        Layout {
            size: size_of::<AdwSwipeableInterface>(),
            alignment: align_of::<AdwSwipeableInterface>(),
        },
    ),
    (
        "AdwValueObjectClass",
        Layout {
            size: size_of::<AdwValueObjectClass>(),
            alignment: align_of::<AdwValueObjectClass>(),
        },
    ),
    (
        "AdwViewSwitcherBarClass",
        Layout {
            size: size_of::<AdwViewSwitcherBarClass>(),
            alignment: align_of::<AdwViewSwitcherBarClass>(),
        },
    ),
    (
        "AdwViewSwitcherClass",
        Layout {
            size: size_of::<AdwViewSwitcherClass>(),
            alignment: align_of::<AdwViewSwitcherClass>(),
        },
    ),
    (
        "AdwViewSwitcherPolicy",
        Layout {
            size: size_of::<AdwViewSwitcherPolicy>(),
            alignment: align_of::<AdwViewSwitcherPolicy>(),
        },
    ),
    (
        "AdwViewSwitcherTitleClass",
        Layout {
            size: size_of::<AdwViewSwitcherTitleClass>(),
            alignment: align_of::<AdwViewSwitcherTitleClass>(),
        },
    ),
    (
        "AdwWindow",
        Layout {
            size: size_of::<AdwWindow>(),
            alignment: align_of::<AdwWindow>(),
        },
    ),
    (
        "AdwWindowClass",
        Layout {
            size: size_of::<AdwWindowClass>(),
            alignment: align_of::<AdwWindowClass>(),
        },
    ),
    (
        "AdwWindowTitleClass",
        Layout {
            size: size_of::<AdwWindowTitleClass>(),
            alignment: align_of::<AdwWindowTitleClass>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) ADW_CENTERING_POLICY_LOOSE", "0"),
    ("(gint) ADW_CENTERING_POLICY_STRICT", "1"),
    ("(gint) ADW_FLAP_FOLD_POLICY_ALWAYS", "1"),
    ("(gint) ADW_FLAP_FOLD_POLICY_AUTO", "2"),
    ("(gint) ADW_FLAP_FOLD_POLICY_NEVER", "0"),
    ("(gint) ADW_FLAP_TRANSITION_TYPE_OVER", "0"),
    ("(gint) ADW_FLAP_TRANSITION_TYPE_SLIDE", "2"),
    ("(gint) ADW_FLAP_TRANSITION_TYPE_UNDER", "1"),
    ("(gint) ADW_LEAFLET_TRANSITION_TYPE_OVER", "0"),
    ("(gint) ADW_LEAFLET_TRANSITION_TYPE_SLIDE", "2"),
    ("(gint) ADW_LEAFLET_TRANSITION_TYPE_UNDER", "1"),
    ("(gint) ADW_NAVIGATION_DIRECTION_BACK", "0"),
    ("(gint) ADW_NAVIGATION_DIRECTION_FORWARD", "1"),
    ("(gint) ADW_SQUEEZER_TRANSITION_TYPE_CROSSFADE", "1"),
    ("(gint) ADW_SQUEEZER_TRANSITION_TYPE_NONE", "0"),
    ("(gint) ADW_VIEW_SWITCHER_POLICY_AUTO", "0"),
    ("(gint) ADW_VIEW_SWITCHER_POLICY_NARROW", "1"),
    ("(gint) ADW_VIEW_SWITCHER_POLICY_WIDE", "2"),
];