# x86listingswitch: the discovery walk leaves i386 switch cases undefined

Status: proposal; no engine change has been implemented. This is a partial
underlying improvement for [#299](https://github.com/Noelo-Lab/kuna/issues/299),
not a claim that its original acceptance bar is met.

## Fixed-byte evidence

The public fixture `decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386`
has a four-entry absolute-pointer table. `dispatch` starts at `0x100000`, checks
`eax <= 3`, and dispatches at `0x100009`. Its case starts are `0x100010`,
`0x100017`, `0x10001e`, and `0x100025`; the default starts at `0x10002c`.

The existing reference reader reaches each case:

```sh
decompiler/target/release/kuna xrefs \
  decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386 \
  --to 0x100010 --kind jump --json
```

A native API comparison, with both walks seeded only at `0x100000`, measured:

| Fact | `Listing::build` | `xrefs::build` |
|---|---:|---:|
| Decoded instructions | 11 | 19 |
| Four table case starts decoded / reached | 0 | 4 |
| Statically reached default decoded | yes | yes |

The Listing instruction query was `is_instruction_start(case)`. The xrefs query
was a `Jump` reference from `0x100009` to each case. This measures missing code
coverage, not an AIF count reduction: this tiny fixture does not supply AIF's
minimum twenty-function fingerprint corpus. Both this API check and the
negative witness below were re-run on base `5fe62be4`, using an own release
build of its exact source tree `d5fb1cb3`.

## Why copying the reference reader is insufficient

The direct-table arm in `listing/xrefs.rs::build_with_focus` calls
`kuna_switchtable::targets` with `cases=None`. That reader scans consecutive
read-only pointers until a word fails its section test or reaches `jumptablemax`.
A neighbouring callback array can contain perfectly valid code pointers.

This runnable negative witness adds such an array after the fixture's four
entries. It builds an isolated temporary ELF; repository files are unchanged:

```sh
python3 - <<'PY'
from pathlib import Path
import subprocess, tempfile
kuna = Path('decompiler/target/release/kuna').resolve()
source = Path('decompiler/crates/kuna-analysis/tests/fixtures/switchtable_i386.s').read_text()
source = source.replace('\t.section\t.rodata', '''\t.globl independent
\t.type independent, @function
independent:
\tpushl %ebp
\tmovl %esp,%ebp
\txorl %eax,%eax
\tleave
\tret
\t.size independent, .-independent
\t.section\t.rodata''')
source = source.replace('msg_alpha:\n', 'adjacent_callback_array:\n\t.long independent\nmsg_alpha:\n')
directory = Path(tempfile.mkdtemp(prefix='kuna-x86listingswitch-'))
assembly, binary = directory / 'adjacent.s', directory / 'adjacent'
assembly.write_text(source)
subprocess.run(['gcc', '-m32', '-nostdlib', '-no-pie', '-Wl,-Ttext=0x100000',
                '-Wl,-e,dispatch', str(assembly), '-o', str(binary)], check=True)
subprocess.run([str(kuna), 'xrefs', str(binary), '--from', 'dispatch',
                '--kind', 'jump', '--json'], check=True)
PY
```

Current output includes a fifth jump from `0x100009` to the independent function
at `0x100042`. The guard permits indices zero through three, so that edge is
false. If discovery copied this unbounded scan, it could decode a genuine
unreferenced function as an intra-function successor and stop entry discovery
from admitting it. Lower entry counts would then conceal lost recall.

The bounded register-table path already has `TableRead::stated`, guard decoding,
and pointer-table reads. Reuse those facilities where their proof applies;
their existence does not establish that an arbitrary nearest comparison checks
the actual table index. The new admission rule must establish that connection.

## Cause and ownership

`listing/walk.rs::step` follows static flow and has special table readers for
Thumb and A32. It has no x86 table reader. `StepCtx` and the constructors in
`listing/mod.rs::Listing::build_resuming` therefore cannot supply one.
`listing/kuna_pdecode.rs` builds its own `StepCtx` for each lane and must make
the same decision as the serial walk. A predecessor-dependent reader added only
to the serial loop would violate that invariant.

The xrefs walk has richer p-code capture, register provenance, predecessor
availability, and a retry list for missing predecessor decodes. The minimal
Listing capture retains only each op's first input. Consequently this is an
analysis feature requiring bounded proof and scheduling design, rather than
the omission of one universally safe successor call. P1 owns it; its normative
chapter is `docs/spec/01-program-prep.md`.

## What #299 still requires

The [latest measured issue update](https://github.com/Noelo-Lab/kuna/issues/299#issuecomment-6162124373)
reports AIF-marginal corpus mid-body entries reduced from 11,010 to 1,504 after
already-merged improvements. The original bar included a sixty-percent corpus
reduction and at most one hundred mid-body entries on the original crackme.
The latter remains unmet; reported PE totals remain 1,335 for Wine and 442 for
MinGW crackmes. These are historical measurements, not new measurements of this
proposal.

The preserved earlier classification associates 466 Wine and 49 MinGW mid-body
entries with unresolved computed jumps in their enclosing symbol extents.
Association is not reachability proof or a predicted 515-entry improvement.
Every recovered dispatch and changed function must be checked afresh.

The original 3.4 MB private i386 PE, its IDA 9.2 entry/extent inventory, and the
exact 590 KB i386 crackme plus unstripped twin are not attached or identified by
retrievable path, public URL, or hash in the issue. They are required to verify
those original witnesses. Public fixtures and cached Wine/MinGW twins permit
implementation and corpus validation, but cannot certify the original crackme
bar or close #299. ARM conditional returns, typed-data Listing infrastructure,
and x86-64 delta-table discovery are separate features.
