# Zebratron Architecture: The Path from High-Level API to 6502

This document outlines the evolutionary path of the Zebratron Game System (ZGS). It defines how we maintain a "fail-safe," developer-friendly system today while leaving the door open for "true retro" 6502 hardware emulation in the future.

## Current State: The "Fantasy Console" (Phase 1)

Currently, the Zebratron is a **High-Level Fantasy Console**. It uses modern Rust logic to "drive" virtual 8-bit hardware (the PPU and APU).

### How it works:
*   **The Engine**: A pure Rust library (`zebratron-core`) that contains the "soul" of the system.
*   **The "Cartridges"**: High-level Rust structs (like `HambertCartridge`) that implement game logic using a modern API.
*   **The Hardware**: 
    *   **PPU (Graphics)**: Handles tiles, sprites, and palettes with strict 8-bit limitations (320x240 resolution).
    *   **APU (Sound)**: Uses digital oscillators (SID-style) to create authentic crunchy audio.
*   **The CPU**: Currently a **Ghost CPU**. It exists as a struct but is bypassed by the high-level cartridge logic.

### Why this is "Fail-Safe":
*   **No Memory Corruption**: High-level Rust prevents the system from crashing due to "illegal opcodes" or "stack overflows."
*   **Rapid Development**: Developers (currently just us!) can iterate on gameplay without fighting 1970s assembly syntax.
*   **Portable**: It runs identically in a Web Browser (WASM) and natively on a Raspberry Pi.

---

## The Transition: The "Virtual Machine" (Phase 2)

To allow other developers to make games, we will move toward a **Scripted Virtual Machine**.

### The Vision:
Instead of hardcoding games into the Rust core, we provide a scripting layer (like **Rhai** or **Lua**).

1.  **Developer writes a script**: `spr(1, 100, 150); sfx(5);`
2.  **Zebratron Host** (Browser or Pi) executes the script.
3.  **The Script calls the ZGS API**: These calls are mapped directly to our Rust PPU and APU.

**Result**: A "legit" feeling 8-bit experience that is as easy to program as a modern web app.

---

## The Future: The "True Retro" 6502 (Phase 3)

If the project demands absolute authenticity, we can "activate" the 6502 CPU without throwing away our work.

### How we get there:
1.  **Complete the Opcode Map**: Finish the `step()` function in `cpu.rs` to handle all 56 standard 6502 instructions.
2.  **Memory Mapping**: Map the PPU and APU registers to specific memory addresses (e.g., PPU Control at `$2000`).
3.  **The "Bridge"**: Our existing high-level API calls (like `ppu.add_sprite()`) become the internal implementation of those memory-mapped registers.

### Why this path makes sense:
*   **Layered Design**: By building the PPU and APU as high-level "services" first, we ensure they are robust and well-tested.
*   **Dual-Mode Support**: The ZGS could eventually support BOTH "High-Level Scripts" (for most devs) and "Raw 6502 ROMs" (for hardcore retro devs).

---

## Summary of the Path

| Feature | Phase 1 (Current) | Phase 2 (Mid-term) | Phase 3 (Future) |
| :--- | :--- | :--- | :--- |
| **Logic** | Hardcoded Rust | Scripted (Rhai/Lua) | 6502 Assembly |
| **PPU/APU** | High-level calls | High-level API | Memory-mapped I/O |
| **Safety** | Maximum (Rust-enforced) | High (VM-enforced) | Low (Raw Memory) |
| **Accessibility**| Developer Only | Everyone | Hardcore Only |

**Conclusion**: We will prioritize the **Phase 2 (Scripted API)** approach for the Raspberry Pi port, as it aligns perfectly with the "fail-safe" and "rugged" project mandates while ensuring we can build for two targets in parallel.
