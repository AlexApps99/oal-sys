#[cfg(feature = "generate")]
const HEADER: &str = "/* OpenAL */
#include <AL/al.h>
#include <AL/alc.h>
#include <AL/alext.h>
#include <AL/efx.h>
#include <AL/efx-creative.h>
#include <AL/efx-presets.h>";

#[cfg(feature = "generate")]
fn generate(path: &std::path::Path) {
    let filename = path.join("openal.rs");
    let bindings = bindgen::Builder::default()
        .header_contents("openal.h", HEADER)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function("^al.+$")
        .whitelist_type("^(?:(?:LP|PFN)?AL.+|EFXEAXREVERBPROPERTIES)$")
        .whitelist_var("^ALC?_.+$")
        .blacklist_type("^__u?int64_t$")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(&filename)
        .expect("Couldn't write bindings!");

    // This code is so hacky it's scaring me
    use std::io::{BufRead, Write};
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("Could not open file");
    let mut presets =
        std::io::BufReader::new(std::fs::File::open("presets.h").expect("Could not find presets"))
            .lines();
    file.write_all(b"\n/* automatically generated by build.rs */\n\n")
        .unwrap();
    file.write_all(b"impl EFXEAXREVERBPROPERTIES {").unwrap();
    loop {
        let name = presets.next();
        let name = if name.is_none() {
            break;
        } else {
            name.unwrap().unwrap()
        };
        let name = &name["#define ".len()..name.len() - 2];
        let lname = name["EFX_REVERB_PRESET_".len()..].to_lowercase();
        let mut d = presets
            .next()
            .unwrap()
            .unwrap()
            .replace("{", "")
            .replace("}", "")
            .replace("f", "");
        d.retain(|c| !c.is_whitespace());
        let d: Vec<_> = d.split(",").collect();
        let _ = presets.next();
        assert_eq!(d.len(), 27);
        #[rustfmt_skip]
        file.write_all(
            format!(
                "
    #[doc = \"{}\"]
    pub fn {}() -> Self {{
        Self {{
            flDensity: {},
            flDiffusion: {},
            flGain: {},
            flGainHF: {},
            flGainLF: {},
            flDecayTime: {},
            flDecayHFRatio: {},
            flDecayLFRatio: {},
            flReflectionsGain: {},
            flReflectionsDelay: {},
            flReflectionsPan: [{}, {}, {}],
            flLateReverbGain: {},
            flLateReverbDelay: {},
            flLateReverbPan: [{}, {}, {}],
            flEchoTime: {},
            flEchoDepth: {},
            flModulationTime: {},
            flModulationDepth: {},
            flAirAbsorptionGainHF: {},
            flHFReference: {},
            flLFReference: {},
            flRoomRolloffFactor: {},
            iDecayHFLimit: {},
        }}
    }}\n\n",
            name, lname,
            d[00], d[01], d[02], d[03], d[04],
            d[05], d[06], d[07], d[08], d[09],
            d[10], d[11], d[12], d[13], d[14],
            d[15], d[16], d[17], d[18], d[19],
            d[20], d[21], d[22], d[23], d[24],
            d[25], d[26]
        ).as_bytes()).unwrap();
    }
    file.write_all(b"}\n").unwrap();
}

fn main() {
    println!("cargo:rustc-link-lib=openal");
    #[cfg(feature = "generate")]
    generate(&std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()));
}