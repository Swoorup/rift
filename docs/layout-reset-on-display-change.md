# Layout Reset on Display Change / Wake from Sleep

## Symptom

Occasionally when displays change or the computer wakes from sleep, all window
layouts reset to defaults — windows are rearranged as if they were freshly
launched with no prior tiling state.

## Root Cause

After sleep/wake or certain display reconfigurations, macOS assigns **new space
IDs** to displays without changing the display topology (same display UUIDs,
same order). The Rift layout engine stores per-space layout trees keyed by
`(SpaceId, VirtualWorkspaceId)` pairs in `WorkspaceLayouts`.

Prior to the fix, the `allow_space_remap` flag in
`handle_screen_parameters_changed` (`src/actor/reactor/events/space.rs:312`)
was only `true` when the display UUID set or display order changed. When only
space IDs change but topology remains the same, `allow_space_remap` was
`false`, so the old layout state (keyed to now-stale space IDs) was never
migrated to the new space IDs. The subsequent `expose_all_spaces()` call
emitted `SpaceExposed` events, which created fresh default layouts for the
new space IDs — effectively resetting everything.

## Fix

Added `has_space_id_changes_for_known_display` detection in
`handle_screen_parameters_changed`. This detects when a previously-known
display has a new space ID even when topology hasn't changed, enabling
`allow_space_remap` to trigger `remap_space()` which migrates all layout
state from old space IDs to new ones.

**File:** `src/actor/reactor/events/space.rs:306-322`
