# Current handoff — 2026-11-25

## 0.106.1 published

[ic-backup 0.106.1](https://crates.io/crates/ic-backup/0.106.1) shipped from
tag `v0.106.1` at `7faaf01`. Both workspaces adopted Toko Tooling 0.46.5.
The release gate passed 247 native tests, five doctests and all 11 Linux
PocketIC scenarios. Registry readback verified the archive checksum, embedded
commit, 182 tracked package files and licence. The independent consumer built
against the registry release. Evidence is under `target/evidence/release-0.106.1/`.

Local root admission, bounded inventory traversal and retained-file publication now
share one private implementation behind the existing store API. The planned Canic
fleet catalog consumes completion reports. Its subnet placement and optional fleet
configuration belong to the integration. See the [storage boundary](../architecture/local-inventory.md#storage-and-fleet-reporting).

The isolated PocketIC consumer recovered the exact first receipt through
`inspect()?.completed` after a second capture and replica shutdown. Evidence is
under `qualification/target/fixtures/capture-Isolated-1994372-1795627037392543521/`.
The release-profile native suite passed 247 tests and five doctests. Fleet
deployment, catalog acknowledgement and Blob-backed restore remain open in
[Canic #111](https://github.com/dragginzgame/canic/issues/111),
[#112](https://github.com/dragginzgame/canic/issues/112) and
[#113](https://github.com/dragginzgame/canic/issues/113).

Backup 0.106.1 exposes authenticated snapshot metadata to local readers via
`CaptureSource::metadata()`. The direct-controller PocketIC fixture also
exercises an exclusive handoff of completed history to fresh files, including
restore of the older of two captures after changing live stable memory. Evidence
is under `qualification/target/fixtures/capture-Isolated-2070040-1795627404513500576/`. The
[local-store contract](../design/local-store.md#moving-completed-capture-data-between-hosts)
records the files required and the outstanding host-loss and Blob requirements.

## 0.106.0 published

[ic-backup 0.106.0](https://crates.io/crates/ic-backup/0.106.0) shipped from
tag `v0.106.0` at `043e449`. The release gate passed 247 native tests, five
doctests and 11 Linux PocketIC scenarios. Registry readback verified the archive
checksum, embedded commit, canonical licence and 181 tracked package files.
The independent consumer built against the published version. Evidence is under
`target/evidence/release-0.106.0/` and `target/publication/`.

Both workspaces pin Toko Tooling 0.46.4. The registry exporter verified all
62 managed files. The publication caller fixture exercises a relative evidence
root with `CDPATH` set. Rust cleanup groups imports and decoder constants.
The minor release adopts Platform 0.35.1. Backup exposes Platform certificate
error payloads through `StatusCertificateError`. Consumers that name those
payloads need Platform 0.35. The archive comparison showed that Platform removed
its optional stable-structures export, which Backup does not select. Approval,
journal and bundle formats carry forward from 0.105.14.

Both locks select Platform 0.35.1, Types 0.21.5, Protocol 0.3.10,
Host FS/Artifacts 0.80.8 and HTTP 0.80.3. Backup owns its Reqwest 0.13.5
client. Qualification selects Host Process 0.81.4, Testkit 0.110.4 and
PocketIC 16.1. The first-party registry review confirmed these versions and
Toko Tooling 0.46.4 were current at release preparation.
[Setup](../development/setup.md) owns tool preparation.
The [transfer contract](../architecture/capture-transfer.md#full-platform-port--2026-11-17)
records Platform decoder and regression provenance.

## 0.105.14 published

[ic-backup 0.105.14](https://crates.io/crates/ic-backup/0.105.14) shipped from
tag `v0.105.14` at `8532c64`. All 247 native tests, five doctests and 11 Linux
PocketIC journeys passed. Registry readback verified all 181 tracked package
files, the license, the embedded commit and the archive checksum. The independent
consumer built against the published version. Evidence is under
`target/evidence/release-0.105.14/`.

## Implemented scope

The experimental library implements capture, history/latest, held sources and
payloads, same-ID/network/release restore, original-call recovery and
completed-target succession. Capture and restore support direct-controller and
Canic Root routes and leave the application stopped.
One recorder preserves finite spending and exact original evidence; pending calls
never repeat and changed/unknown files refuse intact. Completed replay requires
no live authority or writes. [Capture](../architecture/capture-intent.md),
[transfer](../architecture/capture-transfer.md) and
[restore](../design/local-store.md#immutable-restore-approval-and-original-custody)
own these guarantees. Full-history replay remains quadratic; reduced payload IO
does not establish end-to-end speedup. Measurements belong to the
[restart design](../design/restart.md#size-maintained-surface-and-repeated-io).

## Published Canic integration — selected persistent counter

Canic v0.224.0 qualifies the published host adapter with Backup 0.105.6 and IC Host Sign 0.1.0. The installed direct route passed with a disposable keyring-backed secp256k1 identity. The Root-managed route recovered the original take and load after lost replies and host restart. With Root as sole controller, the selected counter was restored stopped, accepted under a fresh authorization and explicitly opened. The consumer scope is one same-release persistent counter without timers, external work or additional participants. These are installed test results; deployment of a live application with operator credentials is a separate operational step. See [external signer qualification](https://github.com/dragginzgame/canic/issues/5), [Root executor delivery](https://github.com/dragginzgame/canic/issues/6) and [application fencing qualification](https://github.com/dragginzgame/canic/issues/7).

The earlier local Root/child restart witness retained the original take and
load replies, both lookup certificates and journals under
`target/evidence/root-host-consumer/run-1794996922952207507/`.
The 20 MiB profile is qualified on the direct route.
See the [Root witness and supplier hashes at release](https://github.com/dragginzgame/ic-backup/blob/6ae096f/docs/status/current.md#root-execute-and-original-result-recovery--local-qualification).

## Preservation and remaining integration

Original wires, roots, journals, child outputs and server logs remain under
`qualification/target/fixtures/`, including failed runs. Never discard uncertain
publication attempts or surviving locks; reconcile the registry before retrying.
The original reader remains at `b7c2c6ae754a5ae2452e65b9be537b517614ebd2`,
`preserve/backup-0.101.13-before-replacement`, with evidence under
`.git/recovery/backup-0.101.13-before-replacement/`. The status-only restore reader
remains under `.git/recovery/host-store-1-before-restore-20261116/` (185 source files,
both locks, six artifacts; archive SHA-256
`6658a8963fe3d56cb5fe13ec210c9170f8d6b7b50f0de33583e627287a371b06`).
[Setup](../development/setup.md#host-store1-reader-before-complete-restore) owns details.
Old operations keep their matching readers, IDs, authority and evidence;
this release migrates no records and changes no legacy/sibling data.

Production HTTPS, actual-IC qualification on macOS and physical power loss
remain unqualified. The [sibling review](../design/sibling-integration.md)
owns the boundary for other applications and external obligations.
