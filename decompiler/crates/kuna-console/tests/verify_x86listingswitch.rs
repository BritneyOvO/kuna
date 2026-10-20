//! End-to-end gate for `x86listingswitch`: the discovery Listing adds the case
//! bodies of an i386 `jmp [idx*4 + table]` switch only where the index bound is
//! proved, and the result does not depend on the lane schedule or on whether
//! the walk was resumed.
//!
//! Fixtures (sources beside them in `kuna-analysis/tests/fixtures`):
//! `x86listingswitch_shapes_i386` holds one function per dispatch shape, each
//! with its table as a `t_<shape>` data symbol, and `x86listingswitch_i386` is
//! the stripped image the stage test drives.

use std::collections::BTreeSet;
use std::path::PathBuf;

use kuna_analysis::listing::model::{DiscoveredFunction, FlowKind, FlowType, RefKind};
use kuna_analysis::listing::{Listing, ListingDetail, WalkPlan};
use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use object::{Object, ObjectSection, ObjectSymbol};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

struct Loaded {
    prog: ConsoleProgram,
    bytes: Vec<u8>,
    path: String,
}

fn load(name: &str) -> Loaded {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name);
    let path = bin.to_str().unwrap().to_string();
    let specs = vec![repo_root().join("specs").to_str().unwrap().to_string()];
    let prog = bootstrap_from_object(&path, "", &specs).expect("bootstrap fixture with built processor specs");
    let bytes = std::fs::read(&bin).expect("read fixture");
    Loaded { prog, bytes, path }
}

fn text_seeds(file: &object::File, bytes: &[u8]) -> Vec<u64> {
    let mut seeds = kuna_analysis::entry::collect_entries(file, bytes);
    seeds.extend(
        file.symbols().filter(|s| s.kind() == object::SymbolKind::Text && s.address() != 0).map(|s| s.address()),
    );
    seeds.sort_unstable();
    seeds.dedup();
    seeds
}

fn build(l: &mut Loaded, on: bool, ceiling: u32, seeds: &[u64], detail: ListingDetail, plan: Option<(usize, usize)>) -> Listing {
    l.prog.arch_mut().set_kuna_option("x86listingswitch", if on { "on" } else { "off" }).unwrap();
    l.prog.arch_mut().max_jumptable_size = ceiling;
    let file = object::File::parse(&*l.bytes).unwrap();
    let image = ObjectLoadImage::from_bytes(&l.path, &l.bytes).unwrap();
    let arch = l.prog.arch();
    let walk = match plan {
        Some((lanes, per_lane)) => WalkPlan::for_lanes(arch, lanes, per_lane, 0),
        None => WalkPlan::serial(),
    };
    let listing = Listing::build_with_meta_planned(&file, &image, arch, arch.translate(), seeds, seeds, &[], detail, &walk);
    if let Some((lanes, per_lane)) = plan {
        assert_eq!(walk.engaged(), 1, "{lanes} lanes x {per_lane} took the serial path");
    }
    listing
}

fn symbol(file: &object::File, name: &str) -> (u64, u64) {
    let s = file.symbols().find(|s| s.name() == Ok(name)).unwrap_or_else(|| panic!("no symbol {name}"));
    (s.address(), s.size())
}

/// The words of the `t_<shape>` table symbol.
fn table(file: &object::File, shape: &str) -> Vec<u64> {
    let (at, size) = symbol(file, &format!("t_{shape}"));
    let sec = file.sections().find(|s| s.address() <= at && at < s.address() + s.size()).unwrap();
    let data = sec.data().unwrap();
    let off = (at - sec.address()) as usize;
    data[off..off + size as usize].chunks(4).map(|w| u64::from(u32::from_le_bytes(w.try_into().unwrap()))).collect()
}

/// The computed jump inside `sh_<shape>`.
fn dispatch(listing: &Listing, file: &object::File, shape: &str) -> u64 {
    let (at, size) = symbol(file, &format!("sh_{shape}"));
    listing
        .instructions_in_range(at, Some(at + size))
        .find(|(_, i)| i.flow.kind == FlowKind::ComputedJump)
        .map(|(&a, _)| a)
        .unwrap_or_else(|| panic!("{shape}: no computed jump decoded"))
}

const PROVEN: [&str; 11] = [
    "ja", "jae", "prenorm", "sched", "repeat", "ceiling", "adjacent", "zextbyte", "zextpre", "zextmid", "zextpreal",
];

const REFUSED: [&str; 20] = [
    "zextpreclobber", "zextother", "zextsigned", "noguard", "wrongidx", "overwritten", "partial", "signed", "lowerbound", "bytecmp", "postnorm",
    "flagclobber", "bypass", "bypassmid", "writable", "invalid", "truncated", "scale2", "wrap", "conflict",
];

/// Every proven shape decodes exactly the cases its bound admits, each as a
/// code edge of the dispatch, and nothing past the bound: the callback word
/// after the adjacent table and the entries past a lowered `jumptablemax`.
#[test]
fn a_proven_bound_adds_exactly_its_cases() {
    let mut l = load("x86listingswitch_shapes_i386");
    let bytes = l.bytes.clone();
    let file = object::File::parse(&*bytes).unwrap();
    let seeds = text_seeds(&file, &bytes);
    let off = build(&mut l, false, 0, &seeds, ListingDetail::FULL, None);
    let on = build(&mut l, true, 0, &seeds, ListingDetail::FULL, None);
    for shape in PROVEN {
        let words = table(&file, shape);
        let cases: BTreeSet<u64> = match shape {
            "adjacent" => words[..4].iter().copied().collect(),
            _ => words.iter().copied().collect(),
        };
        let d = dispatch(&on, &file, shape);
        for &case in &cases {
            assert!(!off.is_instruction_start(case), "{shape}: off, case {case:#x} must stay undecoded");
            assert!(on.is_instruction_start(case), "{shape}: on, case {case:#x} must be decoded");
            assert!(on.function_at(case).is_none(), "{shape}: a case is not a function entry");
        }
        let edges: BTreeSet<u64> = on.refs_from(d).iter().filter(|r| r.kind == RefKind::Code).map(|r| r.to).collect();
        assert_eq!(edges, cases, "{shape}: the dispatch's code edges are its cases");
    }
    assert_eq!(table(&file, "repeat").iter().collect::<BTreeSet<_>>().len(), 3);
    let callback = table(&file, "adjacent")[4];
    assert!(!on.is_instruction_start(callback), "the word after the adjacent table is not a fifth case");

    let capped = build(&mut l, true, 4, &seeds, ListingDetail::FULL, None);
    let words = table(&file, "ceiling");
    assert_eq!(words.len(), 8);
    assert!(words[..4].iter().all(|&c| capped.is_instruction_start(c)), "the ceiling keeps the first entries");
    assert!(words[4..].iter().all(|&c| !capped.is_instruction_start(c)), "nothing past the ceiling is read");
}

/// Every unproven shape reads nothing, with no fallback to an unbounded scan:
/// none of its table's words is decoded, and its extent is decoded exactly as
/// with the option off.
#[test]
fn an_unproven_dispatch_reads_nothing() {
    let mut l = load("x86listingswitch_shapes_i386");
    let bytes = l.bytes.clone();
    let file = object::File::parse(&*bytes).unwrap();
    let seeds = text_seeds(&file, &bytes);
    let off = build(&mut l, false, 0, &seeds, ListingDetail::FULL, None);
    let on = build(&mut l, true, 0, &seeds, ListingDetail::FULL, None);
    let (conflict_fn, _) = symbol(&file, "sh_conflict_fn");
    for shape in REFUSED {
        for case in table(&file, shape) {
            if case == 0 || case == conflict_fn {
                continue;
            }
            assert!(!on.is_instruction_start(case), "{shape}: case {case:#x} must not be read");
        }
        let (at, size) = symbol(&file, &format!("sh_{shape}"));
        let span = |l: &Listing| l.instructions_in_range(at, Some(at + size)).map(|(&a, _)| a).collect::<Vec<_>>();
        assert_eq!(span(&on), span(&off), "{shape}: the extent decodes as with the option off");
    }
    let (slot, _) = symbol(&file, "veneer_slot");
    let target = {
        let sec = file.sections().find(|s| s.address() <= slot && slot < s.address() + s.size()).unwrap();
        let off = (slot - sec.address()) as usize;
        u64::from(u32::from_le_bytes(sec.data().unwrap()[off..off + 4].try_into().unwrap()))
    };
    assert!(!on.is_instruction_start(target), "a jump through one read-only slot is not a table");
}

/// The stripped stage image: the four-plus-one cases behind the `ja` become the
/// dispatcher's code and their callees functions, while the independent
/// function between case blocks, the gap before it, the signed dispatch's cases
/// and the function only a callback word names all stay undecoded.
#[test]
fn the_stage_image_decodes_cases_and_nothing_around_them() {
    let mut l = load("x86listingswitch_i386");
    let bytes = l.bytes.clone();
    let file = object::File::parse(&*bytes).unwrap();
    let seeds = kuna_analysis::entry::collect_entries(&file, &bytes);
    let off = build(&mut l, false, 0, &seeds, ListingDetail::FULL, None);
    let on = build(&mut l, true, 0, &seeds, ListingDetail::FULL, None);
    let cases = [0x10017c, 0x100186, 0x10019a, 0x1001ba, 0x1001c4];
    let callees = [0x1001d4, 0x1001de, 0x1001e8, 0x1001f2, 0x1001fc];
    for case in cases {
        assert!(!off.is_instruction_start(case) && on.is_instruction_start(case), "case {case:#x}");
        assert!(on.function_at(case).is_none(), "case {case:#x} is not an entry");
    }
    for callee in callees {
        assert!(off.function_at(callee).is_none() && on.function_at(callee).is_some(), "callee {callee:#x}");
    }
    assert!(on.function_at(0x100190).is_some(), "the called function between cases keeps its entry");
    for untouched in [0x1001a4, 0x1001b0, 0x100216, 0x100220, 0x10022a, 0x10023a] {
        assert!(!on.is_instruction_start(untouched), "{untouched:#x} stays undecoded");
    }
}

type Partition = (
    Vec<(u64, u32, Option<u64>, FlowType, Vec<u64>, String, String)>,
    Vec<(u64, DiscoveredFunction)>,
    Vec<(u64, Vec<(u64, u64, RefKind)>)>,
);

fn partition(listing: &Listing) -> Partition {
    let insns = listing
        .instructions()
        .map(|(&a, i)| (a, i.len, i.fall_through, i.flow, i.flows.clone(), i.mnemonic.to_string(), i.operands.clone()))
        .collect();
    let funcs = listing.functions().map(|(&a, f)| (a, f.clone())).collect();
    let refs = listing
        .ref_source_iter()
        .map(|a| (a, listing.refs_from(a).iter().map(|r| (r.from, r.to, r.kind)).collect()))
        .collect();
    (insns, funcs, refs)
}

/// Serial, every lane x interval cell, and a resumed walk all produce the same
/// extended Listing.
#[test]
fn serial_parallel_and_resumed_builds_agree() {
    for name in ["x86listingswitch_shapes_i386", "x86listingswitch_i386"] {
        let mut l = load(name);
        let bytes = l.bytes.clone();
        let file = object::File::parse(&*bytes).unwrap();
        let seeds = text_seeds(&file, &bytes);
        for detail in [ListingDetail::PARTITION_ONLY, ListingDetail::FULL] {
            let want = partition(&build(&mut l, true, 0, &seeds, detail, None));
            assert!(want.0.len() > 100, "{name}: a real program is walked");
            for lanes in [2usize, 3, 4, 8] {
                for per_lane in [1usize, 4, 32] {
                    let got = partition(&build(&mut l, true, 0, &seeds, detail, Some((lanes, per_lane))));
                    assert!(got == want, "{name}: {lanes} lanes x {per_lane} moved the extended walk");
                }
            }
        }
    }

    let mut l = load("x86listingswitch_shapes_i386");
    let bytes = l.bytes.clone();
    let file = object::File::parse(&*bytes).unwrap();
    let seeds = text_seeds(&file, &bytes);
    let cold = partition(&build(&mut l, true, 0, &seeds, ListingDetail::FULL, None));
    let (bypass, _) = symbol(&file, "sh_bypass");
    let (noguard, _) = symbol(&file, "sh_noguard");
    let extended: Vec<u64> = seeds.iter().copied().filter(|&s| s < bypass).collect();
    assert!(extended.len() > 2 && extended.len() < seeds.len());
    for (first, grew) in [(extended, true), (vec![noguard], false)] {
        let partial = build(&mut l, true, 0, &first, ListingDetail::FULL, None);
        let ja = table(&file, "ja");
        assert_eq!(ja.iter().all(|&c| partial.is_instruction_start(c)), grew, "the first walk's own extension");
        let arch = l.prog.arch();
        let resumed =
            partial.resume(&file, arch, arch.translate(), &seeds, &seeds, &[], ListingDetail::FULL, &WalkPlan::serial());
        assert!(partition(&resumed) == cold, "a resumed walk equals a cold one (first walk extended: {grew})");
    }
}
