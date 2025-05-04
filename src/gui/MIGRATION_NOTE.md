# GUI Migration Note

## Status: Migration to ECS Complete

The GUI system has been migrated to follow ECS principles. All functionality has been moved to:

1. ECS Resources: `src/ecs/resources/gui_state.rs` and `src/ecs/resources/window_manager/*.rs`
2. ECS Systems: `src/ecs/systems/render/gui.rs`
3. Asset loading in ECS startup system: `src/ecs/systems/startup/assets.rs`

## Final Cleanup Steps

All of the code in the `gui` module can now be safely deleted as it has been migrated to ECS.

### Files/Directories to Delete
```
rm -rf src/gui
```

### Update Cargo.toml
Remove any gui-specific dependencies that are no longer used after the cleanup.

### Check for References
Before deletion, ensure there are no remaining references to the old `gui` module:
```
grep -r "use crate::gui" src/
```

### Update Documentation
Update any documentation or comments that reference the old GUI system to point to the new ECS-based approach.

## References to Update

When switching from the old GUI system to new ECS-based system:

1. Replace `gui::GuiState` → `ecs::resources::gui_state::GuiStateRes`
2. Replace `gui::windows::window_state::WindowState` → Direct functions on ECS resources
3. Replace `gui::windows::city_info::portraits::CivPortraits` → `ecs::resources::portraits::CivPortraits`
4. Replace individual window states with their ECS resource equivalents

## Benefits of the Migration

1. **Proper ECS Architecture**: All game state now lives in ECS resources
2. **Easier to Reason About**: State and behavior are clear and follow a consistent pattern
3. **Improved Performance**: No more wrapping/unwrapping when accessing GUI state
4. **Better Maintainability**: Systems access resources directly with consistent patterns 